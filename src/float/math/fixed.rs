//! Shared extended-precision fixed-point / normalized floating-point math engine
//! used by the transcendental and algebraic functions in `crate::float::math`.
//!
//! Design invariants:
//! - Completely `#![no_std]` and `no_alloc` compatible.
//! - Pure `const fn` throughout.
//! - Valid for all `W > 1` and `2 < MB < 8 * W - 2`, including `EXPONENT_BITS == 2`
//!   and `EXPONENT_BITS >= 32` (never touches `Float::EXP_BIAS` or `Float::ONE`).

use super::Float;
use crate::digits::Digits;
use crate::float::RoundingMode;
use crate::{Exponent, Int, Uint};
use core::cmp::Ordering;

/// Fixed-point buffer with `2 * W + 24` bytes.
///
/// The number of full 64-bit limbs `MAX_LIMBS = (2 * W + 24) / 8` satisfies
/// `MAX_LIMBS * 64 >= 16 * W + 136 > 2 * MB + 128` for all `W > 1` and `MB < 8 * W - 2`.
pub(in crate::float) type Fixed<const W: usize> = Digits<u64, W, 2, 24>;

/// Wider buffer (`4 * W + 48` bytes) used for Payne-Hanek trigonometric range reduction.
type WideFixed<const W: usize> = Digits<u64, W, 4, 48>;

/// Conservative Ziv error margin in ULPs of `Fixed<W>`: `2^24`.
pub(in crate::float) const ZIV_ERR_BITS: Exponent = 24;

/// Exact top 512 bits of `ln(2)` (`floor(ln(2) * 2^512)`), big-endian `u64` limbs.
const LN2_512_BE: [u64; 8] = [
    0xb172_17f7_d1cf_79ab,
    0xc9e3_b398_03f2_f6af,
    0x40f3_4326_7298_b62d,
    0x8a0d_175b_8baa_fa2b,
    0xe7b8_7620_6deb_ac98,
    0x5595_52fb_4afa_1b10,
    0xed2e_ae35_c138_2144,
    0x2757_3b29_1169_b825,
];

/// Exact top 512 bits of `ln(10) / 4` (`floor((ln(10) / 4) * 2^512)`), big-endian `u64` limbs.
const LN10_DIV4_512_BE: [u64; 8] = [
    0x935d_8ddd_aaa8_ac16,
    0xea56_d62b_82d3_0a28,
    0xe28f_ecf9_da5d_f90e,
    0x83c6_1e82_01f0_2d72,
    0x962f_02d7_b1a8_105c,
    0xcc70_cbc0_2c5f_0d68,
    0x2c62_2418_410b_e2da,
    0xfb8f_7884_02e5_16d6,
];

/// Exact top 512 bits of `pi / 4` (`floor((pi / 4) * 2^512)`), big-endian `u64` limbs.
const PI_DIV4_512_BE: [u64; 8] = [
    0xc90f_daa2_2168_c234,
    0xc4c6_628b_80dc_1cd1,
    0x2902_4e08_8a67_cc74,
    0x020b_bea6_3b13_9b22,
    0x514a_0879_8e34_04dd,
    0xef95_19b3_cd3a_431b,
    0x302b_0a6d_f25f_1437,
    0x4fe1_356d_6d51_c245,
];

/// Exact top 1408 bits of `2 / pi` (`floor((2 / pi) * 2^1408)`), big-endian `u64` limbs.
/// Covers Payne-Hanek range reduction for every `F16`, `F32`, `F64`, and all `|x| < 2^1200`.
pub(in crate::float) const TWO_OVER_PI_1408_BE: [u64; 22] = [
    0xa2f9_836e_4e44_1529,
    0xfc27_57d1_f534_ddc0,
    0xdb62_9599_3c43_9041,
    0xfe51_63ab_debb_c561,
    0xb724_6e3a_424d_d2e0,
    0x0649_2eea_09d1_921c,
    0xfe1d_eb1c_b129_a73e,
    0xe882_35f5_2ebb_4484,
    0xe99c_7026_b45f_7e41,
    0x3991_d639_8353_39f4,
    0x9c84_5f8b_bdf9_283b,
    0x1ff8_97ff_de05_980f,
    0xef2f_118b_5a0a_6d1f,
    0x6d36_7ecf_27cb_09b7,
    0x4f46_3f66_9e5f_ea2d,
    0x7527_bac7_ebe5_f17b,
    0x3d07_39f7_8a52_92ea,
    0x6bfb_5fb1_1f8d_5d08,
    0x5603_3046_fc7b_6bab,
    0xf0cf_bc20_9af4_361d,
    0xa9e3_9161_5ee6_1b08,
    0x6599_855f_14a0_6840,
];

/// Normalized extended-precision floating-point value at `P = 64 * limbs` bits of precision.
///
/// When `!zero`, `sig.get(limbs - 1) >= (1u64 << 63)` (bit `P - 1` is set), and the numerical
/// value represented is:
///
/// `(-1)^sign * (sig / 2^(P - 1)) * 2^exp`
///
/// where `sig / 2^(P - 1)` lies in `[1.0, 2.0)`.
#[derive(Clone, Copy)]
pub(in crate::float) struct Ext<const W: usize> {
    pub sign: bool,
    pub zero: bool,
    pub exp: Int<W>,
    pub sig: Fixed<W>,
}

#[inline(always)]
pub(in crate::float) const fn max_limbs<const W: usize>() -> usize {
    (2 * W + 24) / 8
}

#[inline(always)]
pub(in crate::float) const fn stage1_limbs<const W: usize, const MB: usize>() -> usize {
    let raw = (MB + 97).div_ceil(64);
    let m = max_limbs::<W>();
    let l = if raw < 2 { 2 } else { raw };
    if l < m { l } else { m }
}

// ---------------------------------------------------------------------------
// Low-level `Fixed<W>` primitives
// ---------------------------------------------------------------------------

#[inline]
pub(in crate::float) const fn fp_is_zero<const W: usize>(a: &Fixed<W>, limbs: usize) -> bool {
    let mut i = 0;
    while i < limbs {
        if a.get(i) != 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[inline]
pub(super) const fn fp_bit_width<const W: usize>(a: &Fixed<W>, limbs: usize) -> Exponent {
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let d = a.get(i);
        if d != 0 {
            return (i as Exponent) * 64 + (64 - d.leading_zeros());
        }
    }
    0
}

#[inline]
pub(super) const fn fp_bit<const W: usize>(a: &Fixed<W>, bit_idx: Exponent) -> bool {
    let w = (bit_idx / 64) as usize;
    let b = bit_idx % 64;
    (a.get(w) >> b) & 1 != 0
}

#[inline]
pub(super) const fn fp_has_low_bits<const W: usize>(a: &Fixed<W>, count: Exponent) -> bool {
    let full = (count / 64) as usize;
    let rem = count % 64;
    let mut i = 0;
    while i < full {
        if a.get(i) != 0 {
            return true;
        }
        i += 1;
    }
    if rem != 0 && (a.get(full) & ((1u64 << rem) - 1)) != 0 {
        return true;
    }
    false
}

#[inline]
pub(super) const fn fp_cmp<const W: usize>(
    a: &Fixed<W>,
    b: &Fixed<W>,
    limbs: usize,
) -> Ordering {
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let av = a.get(i);
        let bv = b.get(i);
        if av > bv {
            return Ordering::Greater;
        } else if av < bv {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

#[inline]
pub(in crate::float) const fn fp_add<const W: usize>(
    a: &Fixed<W>,
    b: &Fixed<W>,
    limbs: usize,
) -> (Fixed<W>, bool) {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut carry = false;
    let mut i = 0;
    while i < limbs {
        let (s1, c1) = a.get(i).overflowing_add(b.get(i));
        let (s2, c2) = s1.overflowing_add(carry as u64);
        out.set(i, s2);
        carry = c1 || c2;
        i += 1;
    }
    (out, carry)
}

#[inline]
pub(super) const fn fp_sub<const W: usize>(
    a: &Fixed<W>,
    b: &Fixed<W>,
    limbs: usize,
) -> (Fixed<W>, bool) {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut borrow = false;
    let mut i = 0;
    while i < limbs {
        let (d1, b1) = a.get(i).overflowing_sub(b.get(i));
        let (d2, b2) = d1.overflowing_sub(borrow as u64);
        out.set(i, d2);
        borrow = b1 || b2;
        i += 1;
    }
    (out, borrow)
}

#[inline]
pub(super) const fn fp_wrapping_neg<const W: usize>(a: &Fixed<W>, limbs: usize) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut carry = 1u128;
    let mut i = 0;
    while i < limbs {
        let sum = (!a.get(i) as u128) + carry;
        out.set(i, sum as u64);
        carry = sum >> 64;
        i += 1;
    }
    out
}

#[inline]
pub(super) const fn fp_shl<const W: usize>(
    a: &Fixed<W>,
    shift: Exponent,
    limbs: usize,
) -> Fixed<W> {
    if shift == 0 {
        return *a;
    }
    let mut out = Fixed::<W>::ALL_ZEROS;
    let p_bits = (limbs as Exponent) * 64;
    if shift >= p_bits {
        return out;
    }
    let word_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    let mut i = word_shift;
    while i < limbs {
        let hi = a.get(i - word_shift);
        let val = if bit_shift == 0 {
            hi
        } else {
            let lo = if i > word_shift {
                a.get(i - word_shift - 1)
            } else {
                0
            };
            (hi << bit_shift) | (lo >> (64 - bit_shift))
        };
        out.set(i, val);
        i += 1;
    }
    out
}

#[inline]
pub(super) const fn fp_shl1_with_bit<const W: usize>(
    a: &Fixed<W>,
    low_bit: u64,
    limbs: usize,
) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut carry = low_bit & 1;
    let mut i = 0;
    while i < limbs {
        let d = a.get(i);
        out.set(i, (d << 1) | carry);
        carry = d >> 63;
        i += 1;
    }
    out
}

#[inline]
pub(super) const fn fp_shr<const W: usize>(
    a: &Fixed<W>,
    shift: Exponent,
    limbs: usize,
) -> Fixed<W> {
    if shift == 0 {
        return *a;
    }
    let mut out = Fixed::<W>::ALL_ZEROS;
    let p_bits = (limbs as Exponent) * 64;
    if shift >= p_bits {
        return out;
    }
    let word_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    let mut i = 0;
    while i + word_shift < limbs {
        let lo = a.get(i + word_shift);
        let val = if bit_shift == 0 {
            lo
        } else {
            let hi = if i + word_shift + 1 < limbs {
                a.get(i + word_shift + 1)
            } else {
                0
            };
            (lo >> bit_shift) | (hi << (64 - bit_shift))
        };
        out.set(i, val);
        i += 1;
    }
    out
}

#[inline]
pub(super) const fn fp_shr1_with_carry<const W: usize>(a: &Fixed<W>, limbs: usize) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut carry = 1u64 << 63;
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let d = a.get(i);
        out.set(i, (d >> 1) | carry);
        carry = d << 63;
    }
    out
}

#[inline]
pub(super) const fn fp_from_uint_shl<const W: usize>(
    u: &Uint<W>,
    shift: Exponent,
    limbs: usize,
) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let u_digits = u.as_digits::<u64>();
    let u_limbs = W.div_ceil(8);
    let word_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;

    let mut i = 0;
    while i < u_limbs {
        let d = u_digits.get(i);
        if d != 0 {
            let dst = i + word_shift;
            if bit_shift == 0 {
                if dst < limbs {
                    out.set(dst, d);
                }
            } else {
                if dst < limbs {
                    out.set(dst, out.get(dst) | (d << bit_shift));
                }
                if dst + 1 < limbs {
                    out.set(dst + 1, out.get(dst + 1) | (d >> (64 - bit_shift)));
                }
            }
        }
        i += 1;
    }
    out
}

#[inline]
pub(super) const fn fp_shr_to_uint<const W: usize>(
    a: &Fixed<W>,
    shift: Exponent,
    limbs: usize,
) -> Uint<W> {
    let mut bytes = [0u8; W];
    let p_bits = (limbs as Exponent) * 64;
    if shift >= p_bits {
        return Uint::from_le_bytes(bytes);
    }
    let word_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    let u_limbs = W.div_ceil(8);
    let mut i = 0;
    while i < u_limbs {
        let src = i + word_shift;
        if src >= limbs {
            break;
        }
        let lo = a.get(src);
        let word = if bit_shift == 0 {
            lo
        } else {
            let hi = if src + 1 < limbs { a.get(src + 1) } else { 0 };
            (lo >> bit_shift) | (hi << (64 - bit_shift))
        };
        let wb = word.to_le_bytes();
        let mut b = 0;
        while b < 8 && i * 8 + b < W {
            bytes[i * 8 + b] = wb[b];
            b += 1;
        }
        i += 1;
    }
    Uint::from_le_bytes(bytes)
}

/// Computes the high `limbs` words of `a * b` together with one guard limb immediately below.
#[inline]
pub(super) const fn fp_mul_hi_with_guard<const W: usize>(
    a: &Fixed<W>,
    b: &Fixed<W>,
    limbs: usize,
) -> (Fixed<W>, u64) {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut guard: u64 = 0;

    let mut i = 0;
    while i < limbs {
        let ai = a.get(i) as u128;
        if ai != 0 {
            let mut carry: u128 = 0;
            let j_start = if i + 1 < limbs { limbs - 1 - i } else { 0 };
            let mut j = j_start;
            while j < limbs {
                let prod = ai * (b.get(j) as u128) + carry;
                let idx = i + j;
                if idx == limbs - 1 {
                    let sum = prod + (guard as u128);
                    guard = sum as u64;
                    carry = sum >> 64;
                } else {
                    let o_idx = idx - limbs;
                    let sum = prod + (out.get(o_idx) as u128);
                    out.set(o_idx, sum as u64);
                    carry = sum >> 64;
                }
                j += 1;
            }
            let mut k = i;
            while carry != 0 && k < limbs {
                let sum = (out.get(k) as u128) + carry;
                out.set(k, sum as u64);
                carry = sum >> 64;
                k += 1;
            }
        }
        i += 1;
    }
    (out, guard)
}

#[inline]
pub(super) const fn fp_mul_hi<const W: usize>(
    a: &Fixed<W>,
    b: &Fixed<W>,
    limbs: usize,
) -> Fixed<W> {
    fp_mul_hi_with_guard(a, b, limbs).0
}

#[inline]
pub(super) const fn fp_mul_u64<const W: usize>(
    a: &Fixed<W>,
    rhs: u64,
    limbs: usize,
) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut carry: u128 = 0;
    let rhs128 = rhs as u128;
    let mut i = 0;
    while i < limbs {
        let prod = (a.get(i) as u128) * rhs128 + carry;
        out.set(i, prod as u64);
        carry = prod >> 64;
        i += 1;
    }
    out
}

#[inline]
pub(in crate::float) const fn fp_div_u64<const W: usize>(
    a: &Fixed<W>,
    d: u64,
    limbs: usize,
) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut rem: u128 = 0;
    let d128 = d as u128;
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let cur = (rem << 64) | (a.get(i) as u128);
        out.set(i, (cur / d128) as u64);
        rem = cur % d128;
    }
    out
}

/// Computes `1 / D` for `D = d * 2^(-64*limbs) in (0.5, 1.0)` (strictly `> 0.5`) via
/// Newton-Raphson, returning `r` such that `R = r * 2^(-(64*limbs - 1)) in (1.0, 2.0)`.
pub(super) const fn fp_recip<const W: usize>(d: &Fixed<W>, limbs: usize) -> Fixed<W> {
    let d_hi = d.get(limbs - 1);
    debug_assert!(d_hi >= (1u64 << 63));
    let q = if d_hi == u64::MAX {
        1u64 << 63
    } else {
        (((1u128 << 127) - 1) / ((d_hi as u128) + 1)) as u64
    };
    let mut r = Fixed::<W>::ALL_ZEROS;
    r.set(limbs - 1, q);

    let target_bits = (limbs as Exponent) * 64;
    let mut acc_bits: Exponent = 58;
    while acc_bits < target_bits {
        let (dr_half, _) = fp_mul_hi_with_guard(d, &r, limbs);
        let two_minus_dr = fp_wrapping_neg(&dr_half, limbs);
        let (hi, guard) = fp_mul_hi_with_guard(&r, &two_minus_dr, limbs);
        r = fp_shl1_with_bit(&hi, guard >> 63, limbs);
        acc_bits *= 2;
    }
    r
}

/// Returns `true` iff `a == 1 << (64 * limbs - 1)` (an exact normalized `1.0`).
#[inline]
const fn fp_is_exact_half<const W: usize>(a: &Fixed<W>, limbs: usize) -> bool {
    if a.get(limbs - 1) != (1u64 << 63) {
        return false;
    }
    let mut i = 0;
    while i + 1 < limbs {
        if a.get(i) != 0 {
            return false;
        }
        i += 1;
    }
    true
}

/// Integer square root of a `u128`.
#[inline]
const fn isqrt_u128(n: u128) -> u64 {
    if n == 0 {
        return 0;
    }
    let shift = (128 - n.leading_zeros()).div_ceil(2);
    let mut x: u128 = 1u128 << shift;
    loop {
        let next = (x + n / x) >> 1;
        if next >= x {
            return x as u64;
        }
        x = next;
    }
}

/// Converts a `u64` into an `Int<W>` (valid for any `W > 1`).
#[inline]
pub(super) const fn int_from_u64<const W: usize>(v: u64) -> Int<W> {
    let mut bytes = [0u8; W];
    let vb = v.to_le_bytes();
    let mut i = 0;
    while i < W && i < 8 {
        bytes[i] = vb[i];
        i += 1;
    }
    Uint::from_le_bytes(bytes).cast_signed()
}

/// Converts an `i64` into an `Int<W>` (valid for any `W > 1`, assuming `|v| < 2^(8*W - 1)`).
#[inline]
pub(in crate::float) const fn int_from_i64<const W: usize>(v: i64) -> Int<W> {
    if v >= 0 {
        int_from_u64(v as u64)
    } else {
        int_from_u64(v.unsigned_abs()).wrapping_neg()
    }
}

/// Extracts a `u64` from a non-negative `Int<W>` known to be `<= u64::MAX`.
#[inline]
pub(super) const fn uint_low_u64<const W: usize>(u: &Uint<W>) -> u64 {
    let mut v = 0u64;
    let mut i = 0;
    while i < W && i < 8 {
        v |= (u.bytes[i] as u64) << (i * 8);
        i += 1;
    }
    v
}

// ---------------------------------------------------------------------------
// `Ext<W>` arithmetic primitives
// ---------------------------------------------------------------------------

#[inline]
pub(super) const fn ext_zero<const W: usize>(sign: bool) -> Ext<W> {
    Ext {
        sign,
        zero: true,
        exp: Int::ZERO,
        sig: Fixed::ALL_ZEROS,
    }
}

#[inline]
pub(super) const fn ext_one<const W: usize>(limbs: usize) -> Ext<W> {
    let mut sig = Fixed::<W>::ALL_ZEROS;
    sig.set(limbs - 1, 1u64 << 63);
    Ext {
        sign: false,
        zero: false,
        exp: Int::ZERO,
        sig,
    }
}

#[inline]
pub(in crate::float) const fn ext_from_u64<const W: usize>(v: u64, limbs: usize) -> Ext<W> {
    if v == 0 {
        return ext_zero(false);
    }
    let lz = v.leading_zeros();
    let mut sig = Fixed::<W>::ALL_ZEROS;
    sig.set(limbs - 1, v << lz);
    Ext {
        sign: false,
        zero: false,
        exp: int_from_u64((63 - lz) as u64),
        sig,
    }
}

#[inline]
pub(super) const fn ext_from_int<const W: usize>(v: Int<W>, limbs: usize) -> Ext<W> {
    if v.is_zero() {
        return ext_zero(false);
    }
    let sign = v.is_negative();
    let mag: Uint<W> = if sign {
        v.wrapping_neg().cast_unsigned()
    } else {
        v.cast_unsigned()
    };
    let bw = mag.bit_width();
    let p_bits = (limbs as Exponent) * 64;
    let sig = fp_from_uint_shl(&mag, p_bits - bw, limbs);
    Ext {
        sign,
        zero: false,
        exp: int_from_u64((bw - 1) as u64),
        sig,
    }
}

/// Converts any finite `Float<W, MB, RM>` to an exact `Ext<W>`.
#[inline]
pub(super) const fn ext_from_float<const W: usize, const MB: usize, const RM: u8>(
    x: Float<W, MB, RM>,
    limbs: usize,
) -> Ext<W> {
    if x.is_zero() {
        return ext_zero(x.is_sign_negative());
    }
    let (sign, biased_exp, mant) = x.into_biased_uint_parts();
    let bias = Float::<W, MB, RM>::exp_bias_uint().cast_signed();
    let bw = mant.bit_width();
    let sub_shift = (MB as Exponent + 1) - bw;
    let exp = biased_exp
        .cast_signed()
        .sub(bias)
        .sub(int_from_u64(sub_shift as u64));
    let p_bits = (limbs as Exponent) * 64;
    let sig = fp_from_uint_shl(&mant, p_bits - bw, limbs);
    Ext {
        sign,
        zero: false,
        exp,
        sig,
    }
}

#[inline]
pub(super) const fn ext_neg<const W: usize>(mut a: Ext<W>) -> Ext<W> {
    a.sign = !a.sign;
    a
}

#[inline]
pub(super) const fn ext_abs<const W: usize>(mut a: Ext<W>) -> Ext<W> {
    a.sign = false;
    a
}

#[inline]
pub(in crate::float) const fn ext_scale_pow2<const W: usize>(mut a: Ext<W>, k: i64) -> Ext<W> {
    if !a.zero {
        a.exp = a.exp.add(int_from_i64(k));
    }
    a
}

#[inline]
pub(super) const fn ext_scale_pow2_int<const W: usize>(mut a: Ext<W>, k: Int<W>) -> Ext<W> {
    if !a.zero {
        a.exp = a.exp.add(k);
    }
    a
}

#[inline]
pub(super) const fn ext_cmp_abs<const W: usize>(
    a: &Ext<W>,
    b: &Ext<W>,
    limbs: usize,
) -> Ordering {
    if a.zero && b.zero {
        return Ordering::Equal;
    }
    if a.zero {
        return Ordering::Less;
    }
    if b.zero {
        return Ordering::Greater;
    }
    match a.exp.cmp(&b.exp) {
        Ordering::Equal => fp_cmp(&a.sig, &b.sig, limbs),
        ord => ord,
    }
}

pub(super) const fn ext_add<const W: usize>(
    a: &Ext<W>,
    b: &Ext<W>,
    limbs: usize,
) -> Ext<W> {
    if a.zero {
        return *b;
    }
    if b.zero {
        return *a;
    }
    let (hi, lo) = match ext_cmp_abs(a, b, limbs) {
        Ordering::Less => (*b, *a),
        _ => (*a, *b),
    };
    let p_bits = (limbs as Exponent) * 64;
    let diff_int = hi.exp.sub(lo.exp);
    let p_int = int_from_u64(p_bits as u64);
    if diff_int.ge(&p_int) {
        return hi;
    }
    let shift = uint_low_u64(&diff_int.cast_unsigned()) as Exponent;
    let lo_shifted = fp_shr(&lo.sig, shift, limbs);

    if hi.sign == lo.sign {
        let (sum, carry) = fp_add(&hi.sig, &lo_shifted, limbs);
        if carry {
            Ext {
                sign: hi.sign,
                zero: false,
                exp: hi.exp.add(Int::ONE),
                sig: fp_shr1_with_carry(&sum, limbs),
            }
        } else {
            Ext {
                sign: hi.sign,
                zero: false,
                exp: hi.exp,
                sig: sum,
            }
        }
    } else {
        let (diff, _) = fp_sub(&hi.sig, &lo_shifted, limbs);
        let bw = fp_bit_width(&diff, limbs);
        if bw == 0 {
            return ext_zero(false);
        }
        let lshift = p_bits - bw;
        let sig = fp_shl(&diff, lshift, limbs);
        Ext {
            sign: hi.sign,
            zero: false,
            exp: hi.exp.sub(int_from_u64(lshift as u64)),
            sig,
        }
    }
}

#[inline]
pub(super) const fn ext_sub<const W: usize>(
    a: &Ext<W>,
    b: &Ext<W>,
    limbs: usize,
) -> Ext<W> {
    ext_add(a, &ext_neg(*b), limbs)
}

#[inline]
pub(super) const fn ext_mul<const W: usize>(
    a: &Ext<W>,
    b: &Ext<W>,
    limbs: usize,
) -> Ext<W> {
    let sign = a.sign ^ b.sign;
    if a.zero || b.zero {
        return ext_zero(sign);
    }
    let (hi, guard) = fp_mul_hi_with_guard(&a.sig, &b.sig, limbs);
    let exp_sum = a.exp.add(b.exp);
    if hi.get(limbs - 1) >= (1u64 << 63) {
        Ext {
            sign,
            zero: false,
            exp: exp_sum.add(Int::ONE),
            sig: hi,
        }
    } else {
        Ext {
            sign,
            zero: false,
            exp: exp_sum,
            sig: fp_shl1_with_bit(&hi, guard >> 63, limbs),
        }
    }
}

#[inline]
pub(in crate::float) const fn ext_recip<const W: usize>(a: &Ext<W>, limbs: usize) -> Ext<W> {
    debug_assert!(!a.zero);
    if fp_is_exact_half(&a.sig, limbs) {
        return Ext {
            sign: a.sign,
            zero: false,
            exp: a.exp.wrapping_neg(),
            sig: a.sig,
        };
    }
    let r = fp_recip(&a.sig, limbs);
    Ext {
        sign: a.sign,
        zero: false,
        exp: a.exp.wrapping_neg().sub(Int::ONE),
        sig: r,
    }
}

#[inline]
pub(in crate::float) const fn ext_div<const W: usize>(
    a: &Ext<W>,
    b: &Ext<W>,
    limbs: usize,
) -> Ext<W> {
    let sign = a.sign ^ b.sign;
    if a.zero {
        return ext_zero(sign);
    }
    let inv = ext_recip(b, limbs);
    ext_mul(a, &inv, limbs)
}

/// Computes `sqrt(a)` for non-negative `Ext<W>` via Newton-Raphson inverse square root.
pub(in crate::float) const fn ext_sqrt<const W: usize>(a: &Ext<W>, limbs: usize) -> Ext<W> {
    if a.zero {
        return ext_zero(a.sign);
    }
    debug_assert!(!a.sign);

    let e_odd = (a.exp.bytes[0] & 1) != 0;
    let half_exp: Int<W> = if e_odd {
        a.exp.sub(Int::ONE).shr(1)
    } else {
        a.exp.shr(1)
    };
    let v_ext = Ext {
        sign: false,
        zero: false,
        exp: if e_odd { Int::ONE } else { Int::ZERO },
        sig: a.sig,
    };

    // Initial 61-bit seed for 1 / sqrt(v_ext):
    // v_ext = top64 / 2^63 * 2^(e_odd).
    let top64 = a.sig.get(limbs - 1) as u128;
    // Scale to ~2^125 or ~2^126 so isqrt has 63 bits.
    let scaled = if e_odd { top64 << 63 } else { top64 << 62 };
    let s0 = isqrt_u128(scaled) as u128; // ~sqrt(v_ext) * 2^62.5? Wait:
    // If !e_odd: scaled = (top64/2^63) * 2^125 => s0 = sqrt(v) * 2^62.5. Better use even shifts!
    let _ = s0;
    let (v_scaled, shift_bits): (u128, u32) = if e_odd {
        // top64 * 2^62 = (top64 / 2^63 * 2) * 2^124 = v * 2^124 => isqrt = sqrt(v) * 2^62
        (top64 << 62, 62)
    } else {
        // top64 * 2^61 = (top64 / 2^63) * 2^124 = v * 2^124 => isqrt = sqrt(v) * 2^62
        (top64 << 61, 62)
    };
    let sqrt0 = (isqrt_u128(v_scaled) as u128) + 1; // >= sqrt(v) * 2^62
    // r = 1 / sqrt(v) in (0.5, 1.0]. We want r_sig = (2 / sqrt(v)) * 2^63 = 2^(64 + 62) / sqrt0 = 2^126 / sqrt0.
    //Wait: if v == 1.0, 1/sqrt(v) = 1.0 -> r_sig = 2^63, exp = 0; if v > 1.0, 1/sqrt(v) in (0.5, 1.0) -> exp = -1.
    let q = (1u128 << (shift_bits + 63)) / sqrt0; // ~(1 / sqrt(v)) * 2^63 in [2^62, 2^63]
    let mut r = if q >= (1u128 << 63) {
        ext_one(limbs)
    } else {
        let mut sig = Fixed::<W>::ALL_ZEROS;
        sig.set(limbs - 1, (q as u64) << 1);
        Ext {
            sign: false,
            zero: false,
            exp: int_from_i64(-1),
            sig,
        }
    };

    let three = ext_from_u64(3, limbs);
    let target_bits = (limbs as Exponent) * 64;
    let mut acc_bits: Exponent = 56;
    while acc_bits < target_bits {
        let r2 = ext_mul(&r, &r, limbs);
        let vr2 = ext_mul(&v_ext, &r2, limbs);
        let diff = ext_sub(&three, &vr2, limbs);
        r = ext_scale_pow2(ext_mul(&r, &diff, limbs), -1);
        acc_bits *= 2;
    }

    let sqrt_v = ext_mul(&v_ext, &r, limbs);
    ext_scale_pow2_int(sqrt_v, half_exp)
}

/// Rounds an `Ext<W>` to the nearest integer `Int<W>` (ties away from zero).
/// Caller must guarantee `|a| < 2^(BITS - 2)`.
pub(super) const fn ext_round_to_int<const W: usize>(a: &Ext<W>, limbs: usize) -> Int<W> {
    if a.zero || a.exp.lt(&int_from_i64(-1)) {
        return Int::ZERO;
    }
    let p_bits = (limbs as Exponent) * 64;
    if a.exp.eq(&int_from_i64(-1)) {
        // |a| in [0.5, 1.0): round to ±1.
        return if a.sign { int_from_i64(-1) } else { Int::ONE };
    }
    let e = uint_low_u64(&a.exp.cast_unsigned()) as Exponent;
    let shift = (p_bits - 1) - e;
    let mut mag = fp_shr_to_uint(&a.sig, shift, limbs);
    if shift > 0 && fp_bit(&a.sig, shift - 1) {
        mag = mag.add(Uint::ONE);
    }
    if a.sign {
        mag.cast_signed().wrapping_neg()
    } else {
        mag.cast_signed()
    }
}

// ---------------------------------------------------------------------------
// High-precision constants (512-bit tables + arbitrary-precision Machin/Størmer)
// ---------------------------------------------------------------------------

#[inline]
pub(in crate::float) const fn fp_from_be_table<const W: usize>(table: &[u64], limbs: usize) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut i = 0;
    while i < limbs && i < table.len() {
        out.set(limbs - 1 - i, table[i]);
        i += 1;
    }
    out
}

/// Computes `floor(atanh(1 / d) * 2^(64 * limbs))` for integer `d >= 26`.
const fn fp_atanh_inv_u64<const W: usize>(d: u64, limbs: usize) -> Fixed<W> {
    let mut p = Fixed::<W>::ALL_ZEROS;
    let d128 = d as u128;
    let mut rem: u128 = 1;
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let cur = rem << 64;
        p.set(i, (cur / d128) as u64);
        rem = cur % d128;
    }
    let mut sum = p;
    let d2 = d * d;
    let mut k: u64 = 3;
    loop {
        p = fp_div_u64(&p, d2, limbs);
        if fp_is_zero(&p, limbs) {
            break;
        }
        let term = fp_div_u64(&p, k, limbs);
        if fp_is_zero(&term, limbs) {
            break;
        }
        let (s, _) = fp_add(&sum, &term, limbs);
        sum = s;
        k += 2;
    }
    sum
}

/// Computes `floor(atan(1 / d) * 2^(64 * limbs))` for integer `d >= 5`.
const fn fp_atan_inv_u64<const W: usize>(d: u64, limbs: usize) -> Fixed<W> {
    let mut p = Fixed::<W>::ALL_ZEROS;
    let d128 = d as u128;
    let mut rem: u128 = 1;
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let cur = rem << 64;
        p.set(i, (cur / d128) as u64);
        rem = cur % d128;
    }
    let mut sum = p;
    let d2 = d * d;
    let mut k: u64 = 3;
    let mut sub = true;
    loop {
        p = fp_div_u64(&p, d2, limbs);
        if fp_is_zero(&p, limbs) {
            break;
        }
        let term = fp_div_u64(&p, k, limbs);
        if fp_is_zero(&term, limbs) {
            break;
        }
        sum = if sub {
            fp_sub(&sum, &term, limbs).0
        } else {
            fp_add(&sum, &term, limbs).0
        };
        sub = !sub;
        k += 2;
    }
    sum
}

/// Returns `ln(2)` as an `Ext<W>` accurate to `limbs` 64-bit limbs.
pub(in crate::float) const fn const_ln2<const W: usize>(limbs: usize) -> Ext<W> {
    let sig = if limbs <= LN2_512_BE.len() {
        fp_from_be_table(&LN2_512_BE, limbs)
    } else {
        // Machin-like identity: ln(2) = 18*atanh(1/26) - 2*atanh(1/4801) + 8*atanh(1/8749)
        let a = fp_mul_u64(&fp_atanh_inv_u64(26, limbs), 18, limbs);
        let b = fp_mul_u64(&fp_atanh_inv_u64(4801, limbs), 2, limbs);
        let c = fp_mul_u64(&fp_atanh_inv_u64(8749, limbs), 8, limbs);
        let (ab, _) = fp_sub(&a, &b, limbs);
        fp_add(&ab, &c, limbs).0
    };
    Ext {
        sign: false,
        zero: false,
        exp: int_from_i64(-1),
        sig,
    }
}

/// Returns `ln(10)` as an `Ext<W>` accurate to `limbs` 64-bit limbs.
pub(in crate::float) const fn const_ln10<const W: usize>(limbs: usize) -> Ext<W> {
    let sig = if limbs <= LN10_DIV4_512_BE.len() {
        fp_from_be_table(&LN10_DIV4_512_BE, limbs)
    } else {
        // ln(10) = 46*atanh(1/31) + 34*atanh(1/49) + 20*atanh(1/161)
        // Compute ln(10)/4 = (23*atanh(1/31) + 17*atanh(1/49) + 10*atanh(1/161)) / 2
        // to stay strictly in [0.5, 1.0) without overflow!
        let a = fp_mul_u64(&fp_atanh_inv_u64(31, limbs), 23, limbs);
        let b = fp_mul_u64(&fp_atanh_inv_u64(49, limbs), 17, limbs);
        let c = fp_mul_u64(&fp_atanh_inv_u64(161, limbs), 10, limbs);
        let (s1, c1) = fp_add(&a, &b, limbs);
        let (s2, c2) = fp_add(&s1, &c, limbs);
        let carry_top = (c1 as u64) + (c2 as u64);
        // s2 + carry_top * 2^P = ln(10)/2 in [1.15, 1.16) => carry_top == 1.
        debug_assert!(carry_top == 1);
        fp_shr1_with_carry(&s2, limbs)
    };
    Ext {
        sign: false,
        zero: false,
        exp: Int::ONE,
        sig,
    }
}

/// Returns `pi / 4` normalized in `[0.5, 1.0)` as `sig` (`exp = -1` for `pi/4`, `0` for `pi/2`, `1` for `pi`).
pub(in crate::float) const fn pi_div4_sig<const W: usize>(limbs: usize) -> Fixed<W> {
    if limbs <= PI_DIV4_512_BE.len() {
        fp_from_be_table(&PI_DIV4_512_BE, limbs)
    } else {
        // Størmer: pi/4 = 44*atan(1/57) + 7*atan(1/239) - 12*atan(1/682) + 24*atan(1/12943)
        let t1 = fp_mul_u64(&fp_atan_inv_u64(57, limbs), 44, limbs);
        let t2 = fp_mul_u64(&fp_atan_inv_u64(239, limbs), 7, limbs);
        let t3 = fp_mul_u64(&fp_atan_inv_u64(682, limbs), 12, limbs);
        let t4 = fp_mul_u64(&fp_atan_inv_u64(12943, limbs), 24, limbs);
        let (s1, _) = fp_add(&t1, &t2, limbs);
        let (s2, _) = fp_sub(&s1, &t3, limbs);
        fp_add(&s2, &t4, limbs).0
    }
}

#[inline]
pub(in crate::float) const fn const_pi<const W: usize>(limbs: usize) -> Ext<W> {
    Ext {
        sign: false,
        zero: false,
        exp: Int::ONE,
        sig: pi_div4_sig(limbs),
    }
}

#[inline]
pub(in crate::float) const fn const_pi_over_2<const W: usize>(limbs: usize) -> Ext<W> {
    Ext {
        sign: false,
        zero: false,
        exp: Int::ZERO,
        sig: pi_div4_sig(limbs),
    }
}

#[inline]
pub(in crate::float) const fn const_pi_over_4<const W: usize>(limbs: usize) -> Ext<W> {
    Ext {
        sign: false,
        zero: false,
        exp: int_from_i64(-1),
        sig: pi_div4_sig(limbs),
    }
}

// ---------------------------------------------------------------------------
// Core transcendental kernels on `Ext<W>`
// ---------------------------------------------------------------------------

/// Computes `atanh(z) = z + z^3/3 + z^5/5 + ...` for `|z| <= 0.5`.
pub(super) const fn ext_atanh_small<const W: usize>(z: &Ext<W>, limbs: usize) -> Ext<W> {
    if z.zero {
        return *z;
    }
    let u = ext_mul(z, z, limbs); // u = z^2 <= 0.25 (exp <= -2)
    let p_bits = (limbs as Exponent) * 64;
    // u_fp = floor(u * 2^P) = u.sig >> (-u.exp - 1)
    let shift_int = u.exp.wrapping_neg().sub(Int::ONE);
    if shift_int.ge(&int_from_u64(p_bits as u64)) {
        return *z;
    }
    let shift = uint_low_u64(&shift_int.cast_unsigned()) as Exponent;
    let u_fp = fp_shr(&u.sig, shift, limbs);

    // sum = u/3 + u^2/5 + u^3/7 + ... at scale 2^P (always < 0.1).
    let mut pow = u_fp;
    let mut sum = fp_div_u64(&pow, 3, limbs);
    let mut k: u64 = 5;
    loop {
        pow = fp_mul_hi(&pow, &u_fp, limbs);
        if fp_is_zero(&pow, limbs) {
            break;
        }
        let term = fp_div_u64(&pow, k, limbs);
        if fp_is_zero(&term, limbs) {
            break;
        }
        sum = fp_add(&sum, &term, limbs).0;
        k += 2;
    }

    // 1 + sum in Ext: value = (1 + sum/2^P) = ((2^(P-1) + sum/2) / 2^(P-1)) * 2^0.
    let mut one_plus = fp_shr(&sum, 1, limbs);
    one_plus.set(limbs - 1, one_plus.get(limbs - 1) | (1u64 << 63));
    let factor = Ext {
        sign: false,
        zero: false,
        exp: Int::ZERO,
        sig: one_plus,
    };
    ext_mul(z, &factor, limbs)
}

/// Computes `ln(x)` for positive `x` (`!x.sign && !x.zero`).
pub(super) const fn ext_ln<const W: usize>(x: &Ext<W>, limbs: usize) -> Ext<W> {
    debug_assert!(!x.zero && !x.sign);
    // Center M in (1/sqrt(2), sqrt(2)] so |z| <= 0.1716.
    const SQRT2_TOP64: u64 = 0xb504_f333_f9de_6484;
    let (m_exp, k) = if x.sig.get(limbs - 1) > SQRT2_TOP64 {
        (int_from_i64(-1), x.exp.add(Int::ONE))
    } else {
        (Int::ZERO, x.exp)
    };
    let mut m = Ext {
        sign: false,
        zero: false,
        exp: m_exp,
        sig: x.sig,
    };

    // For very high precision (limbs > 6), do a couple of square roots so atanh converges fast.
    let mut sqrt_steps: i64 = 0;
    if limbs > 6 {
        let mut s = 0;
        while s < 3 {
            m = ext_sqrt(&m, limbs);
            s += 1;
        }
        sqrt_steps = 3;
    }

    let one = ext_one(limbs);
    let num = ext_sub(&m, &one, limbs);
    let ln_m = if num.zero {
        ext_zero(false)
    } else {
        let den = ext_add(&m, &one, limbs);
        let z = ext_div(&num, &den, limbs);
        ext_scale_pow2(ext_atanh_small(&z, limbs), 1 + sqrt_steps)
    };

    if k.is_zero() {
        return ln_m;
    }
    let k_ln2 = ext_mul(&ext_from_int(k, limbs), &const_ln2(limbs), limbs);
    ext_add(&k_ln2, &ln_m, limbs)
}

/// Computes `ln(1 + x)` for `x > -1`, cancellation-free near `x = 0`.
pub(super) const fn ext_ln1p<const W: usize>(x: &Ext<W>, limbs: usize) -> Ext<W> {
    if x.zero {
        return *x;
    }
    // For |x| < 0.5, ln(1 + x) = 2 * atanh(x / (2 + x)) with |z| < 1/3 and zero cancellation.
    if x.exp.lt(&int_from_i64(-1)) {
        let two = ext_from_u64(2, limbs);
        let den = ext_add(&two, x, limbs);
        let z = ext_div(x, &den, limbs);
        return ext_scale_pow2(ext_atanh_small(&z, limbs), 1);
    }
    let one_plus_x = ext_add(&ext_one(limbs), x, limbs);
    ext_ln(&one_plus_x, limbs)
}

/// Computes `e^r - 1` for `|r| <= 1.0`, cancellation-free near `r = 0`.
pub(super) const fn ext_expm1_small<const W: usize>(r: &Ext<W>, limbs: usize) -> Ext<W> {
    if r.zero {
        return *r;
    }
    let neg = r.sign;
    let a = ext_abs(*r);

    // Halve a until u < 2^-9 so Taylor series converges in ~15-25 terms.
    let target_exp = int_from_i64(-10);
    let s: u32 = if a.exp.gt(&target_exp) {
        uint_low_u64(&a.exp.sub(target_exp).cast_unsigned()) as u32
    } else {
        0
    };
    let u = ext_scale_pow2(a, -(s as i64));

    let p_bits = (limbs as Exponent) * 64;
    let shift_int = u.exp.wrapping_neg().sub(Int::ONE);
    let mut d = if shift_int.ge(&int_from_u64(p_bits as u64)) {
        u
    } else {
        let shift = uint_low_u64(&shift_int.cast_unsigned()) as Exponent;
        let u_fp = fp_shr(&u.sig, shift, limbs);

        // S = u/2! + u^2/3! + u^3/4! + ... at scale 2^P (always < 0.001).
        let mut term = fp_shr(&u_fp, 1, limbs); // u / 2!
        let mut sum = term;
        let mut k: u64 = 3;
        loop {
            term = fp_mul_hi(&term, &u_fp, limbs);
            if fp_is_zero(&term, limbs) {
                break;
            }
            term = fp_div_u64(&term, k, limbs);
            if fp_is_zero(&term, limbs) {
                break;
            }
            sum = fp_add(&sum, &term, limbs).0;
            k += 1;
        }

        let mut one_plus = fp_shr(&sum, 1, limbs);
        one_plus.set(limbs - 1, one_plus.get(limbs - 1) | (1u64 << 63));
        let factor = Ext {
            sign: false,
            zero: false,
            exp: Int::ZERO,
            sig: one_plus,
        };
        ext_mul(&u, &factor, limbs)
    };

    // Undo halving: e^(2t) - 1 = 2*(e^t - 1) + (e^t - 1)^2 (all positive additions).
    let mut i = 0;
    while i < s {
        let d2 = ext_mul(&d, &d, limbs);
        d = ext_add(&ext_scale_pow2(d, 1), &d2, limbs);
        i += 1;
    }

    if !neg {
        d
    } else {
        // e^(-a) - 1 = -d / (1 + d)
        let one_plus_d = ext_add(&ext_one(limbs), &d, limbs);
        let mut res = ext_div(&d, &one_plus_d, limbs);
        res.sign = true;
        res
    }
}

/// Computes `e^x` for moderate `x` where `k = round(x / ln(2))` fits safely in `Int<W>`.
pub(super) const fn ext_exp<const W: usize>(x: &Ext<W>, limbs: usize) -> Ext<W> {
    if x.zero {
        return ext_one(limbs);
    }
    let ln2 = const_ln2(limbs);
    let q = ext_div(x, &ln2, limbs);
    let k = ext_round_to_int(&q, limbs);
    let r = if k.is_zero() {
        *x
    } else {
        let k_ln2 = ext_mul(&ext_from_int(k, limbs), &ln2, limbs);
        ext_sub(x, &k_ln2, limbs)
    };
    let d = ext_expm1_small(&r, limbs);
    let m = ext_add(&ext_one(limbs), &d, limbs);
    ext_scale_pow2_int(m, k)
}

/// Computes `(sin(r), cos(r))` for `|r| <= pi/4`, cancellation-free near `r = 0`.
pub(super) const fn ext_sin_cos_reduced<const W: usize>(
    r: &Ext<W>,
    limbs: usize,
) -> (Ext<W>, Ext<W>) {
    if r.zero {
        return (*r, ext_one(limbs));
    }
    let neg = r.sign;
    let a = ext_abs(*r);

    // Halve a until u < 2^-9 so u^2 < 2^-18 (~18+ bits per Taylor term).
    let target_exp = int_from_i64(-10);
    let s: u32 = if a.exp.gt(&target_exp) {
        uint_low_u64(&a.exp.sub(target_exp).cast_unsigned()) as u32
    } else {
        0
    };
    let u = ext_scale_pow2(a, -(s as i64));
    let u2 = ext_mul(&u, &u, limbs);

    let p_bits = (limbs as Exponent) * 64;
    let shift_int = u2.exp.wrapping_neg().sub(Int::ONE);

    // Compute sin(u) and c1(u) = 1 - cos(u).
    let (mut sin_t, mut c1_t) = if shift_int.ge(&int_from_u64(p_bits as u64)) {
        (u, ext_scale_pow2(u2, -1))
    } else {
        let shift = uint_low_u64(&shift_int.cast_unsigned()) as Exponent;
        let u2_fp = fp_shr(&u2.sig, shift, limbs);

        // sin(u) / u = 1 - u^2/3! + u^4/5! - ...
        // Represent 1.0 at bit P - 2 (value = fp / 2^(P-2)), so sum is in (0.999, 1.0].
        let mut s_sum = Fixed::<W>::ALL_ZEROS;
        s_sum.set(limbs - 1, 1u64 << 62);
        let mut s_term = fp_div_u64(&fp_shr(&u2_fp, 2, limbs), 6, limbs); // (u^2 / 6) * 2^(P-2)
        let mut s_sub = true;
        let mut k: u64 = 4;
        while !fp_is_zero(&s_term, limbs) {
            s_sum = if s_sub {
                fp_sub(&s_sum, &s_term, limbs).0
            } else {
                fp_add(&s_sum, &s_term, limbs).0
            };
            s_term = fp_mul_hi(&s_term, &u2_fp, limbs);
            s_term = fp_div_u64(&s_term, k * (k + 1), limbs);
            s_sub = !s_sub;
            k += 2;
        }
        // Normalize s_sum (value = s_sum / 2^(P-2) in (0.5, 1.0]).
        let s_bw = fp_bit_width(&s_sum, limbs);
        let s_lsh = p_bits - s_bw;
        let s_factor = Ext {
            sign: false,
            zero: false,
            exp: int_from_i64((s_bw as i64) - (p_bits as i64) + 1),
            sig: fp_shl(&s_sum, s_lsh, limbs),
        };
        let sin_u = ext_mul(&u, &s_factor, limbs);

        // (1 - cos(u)) / u^2 = 1/2 - u^2/4! + u^4/6! - ...
        // Scale by 2^(P-1): 1/2 sits at bit P - 2.
        let mut c_sum = Fixed::<W>::ALL_ZEROS;
        c_sum.set(limbs - 1, 1u64 << 62);
        let mut c_term = fp_div_u64(&fp_shr(&u2_fp, 1, limbs), 24, limbs); // (u^2 / 24) * 2^(P-1)
        let mut c_sub = true;
        let mut m: u64 = 5;
        while !fp_is_zero(&c_term, limbs) {
            c_sum = if c_sub {
                fp_sub(&c_sum, &c_term, limbs).0
            } else {
                fp_add(&c_sum, &c_term, limbs).0
            };
            c_term = fp_mul_hi(&c_term, &u2_fp, limbs);
            c_term = fp_div_u64(&c_term, m * (m + 1), limbs);
            c_sub = !c_sub;
            m += 2;
        }
        // value = c_sum / 2^(P-1) in (0.49, 0.5]
        let c_bw = fp_bit_width(&c_sum, limbs);
        let c_lsh = p_bits - c_bw;
        let c_factor = Ext {
            sign: false,
            zero: false,
            exp: int_from_i64((c_bw as i64) - (p_bits as i64)),
            sig: fp_shl(&c_sum, c_lsh, limbs),
        };
        let c1_u = ext_mul(&u2, &c_factor, limbs);

        (sin_u, c1_u)
    };

    // Double s times using cancellation-free identities:
    // sin(2t) = 2 * sin(t) * (1 - c1(t))
    // c1(2t)  = 2 * sin(t)^2
    let one = ext_one(limbs);
    let mut i = 0;
    while i < s {
        let cos_t = ext_sub(&one, &c1_t, limbs);
        let new_sin = ext_scale_pow2(ext_mul(&sin_t, &cos_t, limbs), 1);
        let new_c1 = ext_scale_pow2(ext_mul(&sin_t, &sin_t, limbs), 1);
        sin_t = new_sin;
        c1_t = new_c1;
        i += 1;
    }

    let cos_r = ext_sub(&one, &c1_t, limbs);
    let sin_r = if neg { ext_neg(sin_t) } else { sin_t };
    (sin_r, cos_r)
}

// ---------------------------------------------------------------------------
// Payne-Hanek trigonometric range reduction (`x = q * (pi/2) + r`, `|r| <= pi/4`)
// ---------------------------------------------------------------------------

/// Dynamically computes bits `[drop_bits .. drop_bits + c_limbs * 64)` of `2 / pi`
/// (with `zero_pad` leading zero bits prepended when `p < 2`) up to 18000 bits,
/// covering every `F80` and `F128` input up to `MAX`.
#[inline(never)]
const fn dynamic_two_over_pi_slice<const W: usize>(
    drop_bits: usize,
    zero_pad: usize,
    c_limbs: usize,
) -> WideFixed<W> {
    // 288 limbs = 18432 bits (2304 bytes on stack, only entered when |x| > 2^1200).
    const BUF_LIMBS: usize = 288;

    #[inline]
    const fn buf_div_u64(a: &[u64; BUF_LIMBS], d: u64, n: usize) -> [u64; BUF_LIMBS] {
        let mut out = [0u64; BUF_LIMBS];
        let mut rem: u128 = 0;
        let d128 = d as u128;
        let mut i = n;
        while i > 0 {
            i -= 1;
            let cur = (rem << 64) | (a[i] as u128);
            out[i] = (cur / d128) as u64;
            rem = cur % d128;
        }
        out
    }

    #[inline]
    const fn buf_is_zero(a: &[u64; BUF_LIMBS], n: usize) -> bool {
        let mut i = 0;
        while i < n {
            if a[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    #[inline]
    const fn buf_add_in_place(dst: &mut [u64; BUF_LIMBS], src: &[u64; BUF_LIMBS], n: usize) {
        let mut carry = false;
        let mut i = 0;
        while i < n {
            let (s1, c1) = dst[i].overflowing_add(src[i]);
            let (s2, c2) = s1.overflowing_add(carry as u64);
            dst[i] = s2;
            carry = c1 || c2;
            i += 1;
        }
    }

    #[inline]
    const fn buf_sub_in_place(dst: &mut [u64; BUF_LIMBS], src: &[u64; BUF_LIMBS], n: usize) {
        let mut borrow = false;
        let mut i = 0;
        while i < n {
            let (d1, b1) = dst[i].overflowing_sub(src[i]);
            let (d2, b2) = d1.overflowing_sub(borrow as u64);
            dst[i] = d2;
            borrow = b1 || b2;
            i += 1;
        }
    }

    #[inline]
    const fn buf_mul_u64(a: &[u64; BUF_LIMBS], rhs: u64, n: usize) -> [u64; BUF_LIMBS] {
        let mut out = [0u64; BUF_LIMBS];
        let mut carry: u128 = 0;
        let r128 = rhs as u128;
        let mut i = 0;
        while i < n {
            let prod = (a[i] as u128) * r128 + carry;
            out[i] = prod as u64;
            carry = prod >> 64;
            i += 1;
        }
        out
    }

    const fn buf_atan_inv(d: u64, n: usize) -> [u64; BUF_LIMBS] {
        let mut p = [0u64; BUF_LIMBS];
        let d128 = d as u128;
        let mut rem: u128 = 1;
        let mut i = n;
        while i > 0 {
            i -= 1;
            let cur = rem << 64;
            p[i] = (cur / d128) as u64;
            rem = cur % d128;
        }
        let mut sum = p;
        let d2 = d * d;
        let mut k: u64 = 3;
        let mut sub = true;
        loop {
            p = buf_div_u64(&p, d2, n);
            if buf_is_zero(&p, n) {
                break;
            }
            let term = buf_div_u64(&p, k, n);
            if buf_is_zero(&term, n) {
                break;
            }
            if sub {
                buf_sub_in_place(&mut sum, &term, n);
            } else {
                buf_add_in_place(&mut sum, &term, n);
            }
            sub = !sub;
            k += 2;
        }
        sum
    }

    const fn buf_mul_hi_guard(
        a: &[u64; BUF_LIMBS],
        b: &[u64; BUF_LIMBS],
        n: usize,
    ) -> ([u64; BUF_LIMBS], u64) {
        let mut out = [0u64; BUF_LIMBS];
        let mut guard: u64 = 0;
        let mut i = 0;
        while i < n {
            let ai = a[i] as u128;
            if ai != 0 {
                let mut carry: u128 = 0;
                let j_start = if i + 1 < n { n - 1 - i } else { 0 };
                let mut j = j_start;
                while j < n {
                    let prod = ai * (b[j] as u128) + carry;
                    let idx = i + j;
                    if idx == n - 1 {
                        let sum = prod + (guard as u128);
                        guard = sum as u64;
                        carry = sum >> 64;
                    } else {
                        let o_idx = idx - n;
                        let sum = prod + (out[o_idx] as u128);
                        out[o_idx] = sum as u64;
                        carry = sum >> 64;
                    }
                    j += 1;
                }
                let mut k = i;
                while carry != 0 && k < n {
                    let sum = (out[k] as u128) + carry;
                    out[k] = sum as u64;
                    carry = sum >> 64;
                    k += 1;
                }
            }
            i += 1;
        }
        (out, guard)
    }

    let needed_bits = drop_bits + c_limbs * 64 + 128;
    let n = {
        let raw = needed_bits.div_ceil(64);
        if raw > BUF_LIMBS { BUF_LIMBS } else { raw }
    };

    // Størmer: pi/4 = 44*atan(1/57) + 7*atan(1/239) - 12*atan(1/682) + 24*atan(1/12943)
    let mut pi_div4 = buf_mul_u64(&buf_atan_inv(57, n), 44, n);
    buf_add_in_place(&mut pi_div4, &buf_mul_u64(&buf_atan_inv(239, n), 7, n), n);
    buf_sub_in_place(&mut pi_div4, &buf_mul_u64(&buf_atan_inv(682, n), 12, n), n);
    buf_add_in_place(&mut pi_div4, &buf_mul_u64(&buf_atan_inv(12943, n), 24, n), n);

    // Invert D = pi/4 in (0.5, 1.0) -> R = 4/pi in (1.0, 2.0) => R/2 = 2/pi = r / 2^P!
    let d_hi = pi_div4[n - 1];
    let q0 = (((1u128 << 127) - 1) / ((d_hi as u128) + 1)) as u64;
    let mut r = [0u64; BUF_LIMBS];
    r[n - 1] = q0;
    let mut acc: usize = 58;
    let target = n * 64;
    while acc < target {
        let (dr_half, _) = buf_mul_hi_guard(&pi_div4, &r, n);
        let mut neg = [0u64; BUF_LIMBS];
        let mut carry = 1u128;
        let mut i = 0;
        while i < n {
            let sum = (!dr_half[i] as u128) + carry;
            neg[i] = sum as u64;
            carry = sum >> 64;
            i += 1;
        }
        let (hi, guard) = buf_mul_hi_guard(&r, &neg, n);
        let mut sh = [0u64; BUF_LIMBS];
        let mut c = guard >> 63;
        let mut i = 0;
        while i < n {
            let d = hi[i];
            sh[i] = (d << 1) | c;
            c = d >> 63;
            i += 1;
        }
        r = sh;
        acc *= 2;
    }

    // Now r (little-endian in r[0..n]) holds floor((2/pi) * 2^(64*n))!
    // Extract c_limbs words starting at MSB offset `drop_bits` with `zero_pad` leading zeros.
    let mut out = WideFixed::<W>::ALL_ZEROS;
    let total_out_bits = c_limbs * 64;
    let mut dst_word_from_msb = 0;
    while dst_word_from_msb < c_limbs {
        let out_msb_bit = dst_word_from_msb * 64;
        let mut word = 0u64;
        let mut b = 0;
        while b < 64 {
            let pos = out_msb_bit + b;
            if pos >= zero_pad {
                let src_msb_bit = drop_bits + (pos - zero_pad);
                let src_lsb_bit = (n * 64).saturating_sub(1 + src_msb_bit);
                let bit = (r[src_lsb_bit / 64] >> (src_lsb_bit % 64)) & 1;
                word |= bit << (63 - b);
            }
            b += 1;
        }
        out.set(c_limbs - 1 - dst_word_from_msb, word);
        let _ = total_out_bits;
        dst_word_from_msb += 1;
    }
    out
}

/// Extracts `c_limbs` words of `2 / pi` starting at MSB bit `drop_bits` with `zero_pad`
/// leading zeros from the static `TWO_OVER_PI_1408_BE` table.
const fn table_two_over_pi_slice<const W: usize>(
    drop_bits: usize,
    zero_pad: usize,
    c_limbs: usize,
) -> WideFixed<W> {
    let mut out = WideFixed::<W>::ALL_ZEROS;
    let mut dst_from_msb = 0;
    while dst_from_msb < c_limbs {
        let out_msb_bit = dst_from_msb * 64;
        let word = if out_msb_bit + 64 <= zero_pad {
            0u64
        } else if out_msb_bit >= zero_pad {
            let src_bit = drop_bits + (out_msb_bit - zero_pad);
            let idx = src_bit / 64;
            let off = src_bit % 64;
            let hi = if idx < TWO_OVER_PI_1408_BE.len() {
                TWO_OVER_PI_1408_BE[idx]
            } else {
                0
            };
            if off == 0 {
                hi
            } else {
                let lo = if idx + 1 < TWO_OVER_PI_1408_BE.len() {
                    TWO_OVER_PI_1408_BE[idx + 1]
                } else {
                    0
                };
                (hi << off) | (lo >> (64 - off))
            }
        } else {
            // Spans zero_pad boundary inside this 64-bit word.
            let pad_in_word = zero_pad - out_msb_bit; // 1..64
            let src_bit = drop_bits;
            let idx = src_bit / 64;
            let off = src_bit % 64;
            let hi = if idx < TWO_OVER_PI_1408_BE.len() {
                TWO_OVER_PI_1408_BE[idx]
            } else {
                0
            };
            let aligned = if off == 0 {
                hi
            } else {
                let lo = if idx + 1 < TWO_OVER_PI_1408_BE.len() {
                    TWO_OVER_PI_1408_BE[idx + 1]
                } else {
                    0
                };
                (hi << off) | (lo >> (64 - off))
            };
            aligned >> pad_in_word
        };
        out.set(c_limbs - 1 - dst_from_msb, word);
        dst_from_msb += 1;
    }
    out
}

/// Computes `x = (4k + quadrant) * (pi/2) + r` with `quadrant in 0..4` and `|r| <= pi/4`.
///
/// Uses exact Payne-Hanek range reduction for all exponents up to `16500` (100% of IEEE
/// `F16`, `F32`, `F64`, `F80`, `F128` inputs), and fast `O(log e)` modular reduction for
/// astronomical exponents (`e > 16500`, e.g., `Float<16, 3>::MAX`).
pub(super) const fn reduce_trig<const W: usize, const MB: usize, const RM: u8>(
    x: Float<W, MB, RM>,
    limbs: usize,
) -> (u8, Ext<W>) {
    if x.is_zero() {
        return (0, ext_zero(x.is_sign_negative()));
    }
    let neg = x.is_sign_negative();
    let abs_ext = ext_from_float(x.abs(), limbs);
    let pi_over_4 = const_pi_over_4(limbs);

    // Fast path: |x| <= pi/4 needs no range reduction.
    if !matches!(ext_cmp_abs(&abs_ext, &pi_over_4, limbs), Ordering::Greater) {
        let r = if neg { ext_neg(abs_ext) } else { abs_ext };
        return (0, r);
    }

    let (_, biased_exp, mant) = x.into_biased_uint_parts();
    let bias = Float::<W, MB, RM>::exp_bias_uint().cast_signed();
    // |x| = mant * 2^p where p = biased_exp - bias - MB.
    let p_int: Int<W> = biased_exp
        .cast_signed()
        .sub(bias)
        .sub(int_from_u64(MB as u64));

    let two_int = int_from_u64(2);
    let max_exact_drop = int_from_u64(16500);

    // Astronomical exponent fallback (p > 16502): reduce |x| mod (2*pi) in O(log p) steps.
    if p_int.gt(&max_exact_drop.add(two_int)) {
        let two_pi = ext_scale_pow2(const_pi(limbs), 1);
        let pi_2 = const_pi_over_2(limbs);
        // Compute (2^p mod 2*pi) by repeated squaring / doubling modulo 2*pi.
        let p_u = p_int.cast_unsigned();
        let mut pow2 = ext_one(limbs);
        let mut base = ext_from_u64(2, limbs);
        let bw = p_u.bit_width();
        let mut bit: Exponent = 0;
        while bit < bw {
            if p_u.bit(bit) {
                pow2 = ext_mul(&pow2, &base, limbs);
                // Reduce pow2 mod 2*pi
                let q = ext_div(&pow2, &two_pi, limbs);
                let qi = ext_round_to_int(&q, limbs);
                pow2 = ext_sub(&pow2, &ext_mul(&ext_from_int(qi, limbs), &two_pi, limbs), limbs);
            }
            base = ext_mul(&base, &base, limbs);
            let q = ext_div(&base, &two_pi, limbs);
            let qi = ext_round_to_int(&q, limbs);
            base = ext_sub(&base, &ext_mul(&ext_from_int(qi, limbs), &two_pi, limbs), limbs);
            bit += 1;
        }
        let m_ext = ext_from_int(mant.cast_signed(), limbs);
        let mut rem = ext_mul(&m_ext, &pow2, limbs);
        let q = ext_div(&rem, &pi_2, limbs);
        let qi = ext_round_to_int(&q, limbs);
        rem = ext_sub(&rem, &ext_mul(&ext_from_int(qi, limbs), &pi_2, limbs), limbs);
        let mut quad = (qi.bytes[0] & 3) as u8;
        if neg {
            quad = (4 - quad) & 3;
            rem = ext_neg(rem);
        }
        return (quad, rem);
    }

    let (drop_bits, zero_pad): (usize, usize) = if p_int.ge(&two_int) {
        (uint_low_u64(&p_int.sub(two_int).cast_unsigned()) as usize, 0)
    } else {
        (0, uint_low_u64(&two_int.sub(p_int).cast_unsigned()) as usize)
    };

    let m_limbs = W.div_ceil(8);
    // Need zero_pad + (MB + 1) + 64 * limbs + 128 bits in C so even heavy cancellation
    // near a multiple of pi/2 retains > P exact bits.
    let c_limbs = (zero_pad + MB + 1 + limbs * 64 + 128).div_ceil(64);
    let c_max_limbs = (4 * W + 48) / 8;
    let c_limbs = if c_limbs > c_max_limbs { c_max_limbs } else { c_limbs };

    let total_src_bits = drop_bits + (c_limbs * 64).saturating_sub(zero_pad);
    let c_slice = if total_src_bits <= TWO_OVER_PI_1408_BE.len() * 64 {
        table_two_over_pi_slice::<W>(drop_bits, zero_pad, c_limbs)
    } else {
        dynamic_two_over_pi_slice::<W>(drop_bits, zero_pad, c_limbs)
    };

    // Multiply mant (m_limbs) * c_slice (c_limbs) modulo 2^(64 * c_limbs).
    let m_digits = mant.as_digits::<u64>();
    let mut prod = WideFixed::<W>::ALL_ZEROS;
    let mut i = 0;
    while i < m_limbs {
        let mi = m_digits.get(i) as u128;
        if mi != 0 {
            let mut carry: u128 = 0;
            let mut j = 0;
            while i + j < c_limbs {
                let idx = i + j;
                let sum = mi * (c_slice.get(j) as u128) + (prod.get(idx) as u128) + carry;
                prod.set(idx, sum as u64);
                carry = sum >> 64;
                j += 1;
            }
        }
        i += 1;
    }

    // Top 2 bits of prod are q_int mod 4; remaining (64*c_limbs - 2) bits are frac in [0, 1).
    let top_word = prod.get(c_limbs - 1);
    let mut quad = (top_word >> 62) as u8;
    let frac_msb = (top_word >> 61) & 1 != 0;
    // Clear top 2 bits so prod holds frac * 2^(64 * c_limbs - 2).
    prod.set(c_limbs - 1, top_word & ((1u64 << 62) - 1));

    let f_neg = if frac_msb {
        // frac >= 0.5 -> bump quadrant and compute 1 - frac = 2^(64*c_limbs - 2) - prod.
        quad = (quad + 1) & 3;
        let mut borrow = false;
        let mut k = 0;
        while k < c_limbs {
            let (d1, b1) = 0u64.overflowing_sub(prod.get(k));
            let (d2, b2) = d1.overflowing_sub(borrow as u64);
            prod.set(k, d2);
            borrow = b1 || b2;
            k += 1;
        }
        prod.set(c_limbs - 1, prod.get(c_limbs - 1) & ((1u64 << 62) - 1));
        true
    } else {
        false
    };

    // Normalize prod (value = prod * 2^-(64*c_limbs - 2)) into Ext<W>.
    let mut highest_word = c_limbs;
    let mut bw: usize = 0;
    while highest_word > 0 {
        highest_word -= 1;
        let w = prod.get(highest_word);
        if w != 0 {
            bw = highest_word * 64 + (64 - w.leading_zeros() as usize);
            break;
        }
    }
    if bw == 0 {
        if neg {
            quad = (4 - quad) & 3;
        }
        return (quad, ext_zero(neg));
    }

    let frac_bits = c_limbs * 64 - 2;
    let f_exp = int_from_i64((bw as i64 - 1) - (frac_bits as i64));

    // Extract top `p_bits` starting at bit `bw - 1` down to `bw - p_bits` into Fixed<W>.
    let mut f_sig = Fixed::<W>::ALL_ZEROS;
    let mut k = 0;
    while k < limbs {
        // Destination limb `limbs - 1 - k` receives bits [bw - 64*k - 1 .. bw - 64*(k+1)].
        let hi_bit_plus1 = bw.saturating_sub(k * 64);
        if hi_bit_plus1 == 0 {
            break;
        }
        let word = if hi_bit_plus1 >= 64 {
            let lo_bit = hi_bit_plus1 - 64;
            let idx = lo_bit / 64;
            let off = lo_bit % 64;
            let lo = prod.get(idx);
            if off == 0 {
                lo
            } else {
                let hi = if idx + 1 < c_limbs { prod.get(idx + 1) } else { 0 };
                (lo >> off) | (hi << (64 - off))
            }
        } else {
            prod.get(0) << (64 - hi_bit_plus1)
        };
        f_sig.set(limbs - 1 - k, word);
        k += 1;
    }

    let f_ext = Ext {
        sign: f_neg,
        zero: false,
        exp: f_exp,
        sig: f_sig,
    };
    let mut r = ext_mul(&f_ext, &const_pi_over_2(limbs), limbs);
    if neg {
        quad = (4 - quad) & 3;
        r = ext_neg(r);
    }
    (quad, r)
}

/// Computes `(sin(x), cos(x))` for any finite `Float<W, MB, RM>` at precision `limbs`.
pub(super) const fn ext_sin_cos_float<const W: usize, const MB: usize, const RM: u8>(
    x: Float<W, MB, RM>,
    limbs: usize,
) -> (Ext<W>, Ext<W>) {
    let (quad, r) = reduce_trig(x, limbs);
    let (s, c) = ext_sin_cos_reduced(&r, limbs);
    match quad & 3 {
        0 => (s, c),
        1 => (c, ext_neg(s)),
        2 => (ext_neg(s), ext_neg(c)),
        _ => (ext_neg(c), s),
    }
}

/// Computes `atan(x)` for any finite `Ext<W>` using reciprocal + half-angle reduction.
pub(super) const fn ext_atan<const W: usize>(x: &Ext<W>, limbs: usize) -> Ext<W> {
    if x.zero {
        return *x;
    }
    let neg = x.sign;
    let a = ext_abs(*x);
    let one = ext_one(limbs);

    let (recip_reduced, mut t) = match ext_cmp_abs(&a, &one, limbs) {
        Ordering::Equal => {
            let res = const_pi_over_4(limbs);
            return if neg { ext_neg(res) } else { res };
        }
        Ordering::Greater => (true, ext_recip(&a, limbs)),
        Ordering::Less => (false, a),
    };

    // Halve t via atan(t) = 2 * atan(t / (1 + sqrt(1 + t^2))) until t < 2^-6.
    let target_exp = int_from_i64(-6);
    let mut halvings: i64 = 0;
    while !t.zero && t.exp.ge(&target_exp) {
        let t2 = ext_mul(&t, &t, limbs);
        let root = ext_sqrt(&ext_add(&one, &t2, limbs), limbs);
        let den = ext_add(&one, &root, limbs);
        t = ext_div(&t, &den, limbs);
        halvings += 1;
    }

    // Alternating Taylor series: atan(t) = t * (1 - t^2/3 + t^4/5 - t^6/7 + ...).
    let t2 = ext_mul(&t, &t, limbs);
    let p_bits = (limbs as Exponent) * 64;
    let shift_int = t2.exp.wrapping_neg().sub(Int::ONE);
    let atan_t = if shift_int.ge(&int_from_u64(p_bits as u64)) {
        t
    } else {
        let shift = uint_low_u64(&shift_int.cast_unsigned()) as Exponent;
        let u_fp = fp_shr(&t2.sig, shift, limbs);

        // Scale by 2^(P - 2) so 1.0 sits at bit P - 2.
        let mut sum = Fixed::<W>::ALL_ZEROS;
        sum.set(limbs - 1, 1u64 << 62);
        let mut pow = fp_shr(&u_fp, 2, limbs); // u * 2^(P - 2)
        let mut k: u64 = 3;
        let mut sub = true;
        while !fp_is_zero(&pow, limbs) {
            let term = fp_div_u64(&pow, k, limbs);
            if fp_is_zero(&term, limbs) {
                break;
            }
            sum = if sub {
                fp_sub(&sum, &term, limbs).0
            } else {
                fp_add(&sum, &term, limbs).0
            };
            pow = fp_mul_hi(&pow, &u_fp, limbs);
            sub = !sub;
            k += 2;
        }
        let bw = fp_bit_width(&sum, limbs);
        let lsh = p_bits - bw;
        let factor = Ext {
            sign: false,
            zero: false,
            exp: int_from_i64((bw as i64) - (p_bits as i64) + 1),
            sig: fp_shl(&sum, lsh, limbs),
        };
        ext_mul(&t, &factor, limbs)
    };

    let mut res = ext_scale_pow2(atan_t, halvings);
    if recip_reduced {
        res = ext_sub(&const_pi_over_2(limbs), &res, limbs);
    }
    if neg { ext_neg(res) } else { res }
}

// ---------------------------------------------------------------------------
// Two-stage Ziv rounding from `Ext<W>` to `Float<W, MB>`
// ---------------------------------------------------------------------------

/// Extracts the top 64 fractional bits `[shift - 64 .. shift)` of `a`, left-aligned into a `u64`.
#[inline]
pub(super) const fn fp_extract_top64<const W: usize>(
    a: &Fixed<W>,
    shift: Exponent,
    limbs: usize,
) -> u64 {
    if shift == 0 {
        return 0;
    }
    if shift <= 64 {
        let lo = a.get(0);
        if shift == 64 {
            lo
        } else {
            (lo & ((1u64 << shift) - 1)) << (64 - shift)
        }
    } else {
        let lo_bit = shift - 64;
        let idx = (lo_bit / 64) as usize;
        let off = lo_bit % 64;
        let lo = if idx < limbs { a.get(idx) } else { 0 };
        if off == 0 {
            lo
        } else {
            let hi = if idx + 1 < limbs { a.get(idx + 1) } else { 0 };
            (lo >> off) | (hi << (64 - off))
        }
    }
}

/// Checks whether the bits in `[low_incl .. high_excl)` of `a` all equal `target_bit`.
const fn fp_range_all_eq<const W: usize>(
    a: &Fixed<W>,
    low_incl: Exponent,
    high_excl: Exponent,
    target_bit: bool,
) -> bool {
    let mut b = low_incl;
    while b < high_excl {
        if fp_bit(a, b) != target_bit {
            return false;
        }
        b += 1;
    }
    true
}

/// Attempts to round `val` to `Float<W, MB, RM>` using rounding mode `RM`.
///
/// When `final_round == false`, returns `None` if `val` lies within `±2^(err_bits + 1)`
/// ULPs of `Fixed<W>` from the rounding-mode discontinuity (`0.5` ULP for round-to-nearest
/// modes, `0.0`/`1.0` ULP for directed and `ToOdd` modes), signaling the caller to retry at
/// `max_limbs::<W>()`. `Faithful` and `Stochastic` never need Stage 2 escalation.
pub(in crate::float) const fn ext_round_to_float<const W: usize, const MB: usize, const RM: u8>(
    val: &Ext<W>,
    limbs: usize,
    err_bits: Exponent,
    final_round: bool,
) -> Option<Float<W, MB, RM>> {
    if val.zero {
        return Some(if val.sign {
            Float::NEG_ZERO
        } else {
            Float::ZERO
        });
    }

    let p_bits = (limbs as Exponent) * 64;
    let bias = Float::<W, MB, RM>::exp_bias_uint().cast_signed();
    let exp_max = Float::<W, MB, RM>::exp_max_uint().cast_signed();
    let be = match val.exp.checked_add(bias) {
        Some(v) => v,
        None => {
            return Some(if val.exp.is_negative() {
                Float::<W, MB, RM>::underflow_nonzero(val.sign)
            } else {
                Float::<W, MB, RM>::overflow_value(val.sign)
            });
        }
    };

    // Overflow check
    if be.ge(&exp_max) {
        return Some(Float::<W, MB, RM>::overflow_value(val.sign));
    }

    let neg_mb = int_from_i64(-(MB as i64));
    let neg_mb_m1 = neg_mb.sub(Int::ONE);

    // Deep underflow (< 0.25 * MIN_POSITIVE_SUBNORMAL)
    if be.lt(&neg_mb_m1) {
        return Some(Float::<W, MB, RM>::underflow_nonzero(val.sign));
    }

    let seed_base = uint_low_u64(&val.exp.cast_unsigned()).wrapping_mul(0x9e37_79b9_7f4a_7c15);

    // Just below half of MIN_POSITIVE_SUBNORMAL: value in [0.25, 0.5) ULPs of subnormal.
    if be.eq(&neg_mb_m1) {
        if !final_round
            && RoundingMode::is_nearest(RM)
            && (err_bits + 2 >= p_bits || fp_range_all_eq(&val.sig, err_bits + 1, p_bits, true))
        {
            return None;
        }
        let frac64 = fp_extract_top64(&val.sig, p_bits, limbs) >> 2;
        return if RoundingMode::should_round_up::<RM>(
            val.sign,
            false,
            false,
            true,
            frac64,
            seed_base,
        ) {
            Some(Float::from_biased_uint_parts(val.sign, Uint::ONE, Uint::ONE))
        } else if val.sign {
            Some(Float::NEG_ZERO)
        } else {
            Some(Float::ZERO)
        };
    }

    // In [0.5, 1.0) ULPs of MIN_POSITIVE_SUBNORMAL.
    if be.eq(&neg_mb) {
        if !final_round && !RoundingMode::is_faithful(RM) && !RoundingMode::is_stochastic(RM) {
            if err_bits + 2 >= p_bits {
                return None;
            }
            if RoundingMode::is_nearest(RM)
                && fp_range_all_eq(&val.sig, err_bits + 1, p_bits - 1, false)
            {
                return None;
            }
            if RoundingMode::is_directed_or_odd(RM)
                && fp_range_all_eq(&val.sig, err_bits + 1, p_bits, true)
            {
                return None;
            }
        }
        let sticky = fp_has_low_bits(&val.sig, p_bits - 1);
        let frac64 = fp_extract_top64(&val.sig, p_bits, limbs) >> 1;
        return if RoundingMode::should_round_up::<RM>(
            val.sign,
            false,
            true,
            sticky,
            frac64,
            seed_base,
        ) {
            Some(Float::from_biased_uint_parts(val.sign, Uint::ONE, Uint::ONE))
        } else if val.sign {
            Some(Float::NEG_ZERO)
        } else {
            Some(Float::ZERO)
        };
    }

    // Normal or subnormal with 1 <= keep_bits <= MB + 1.
    let (mut out_be, sub_drop): (Int<W>, Exponent) = if be.ge(&Int::ONE) {
        (be, 0)
    } else {
        let drop = uint_low_u64(&Int::<W>::ONE.sub(be).cast_unsigned()) as Exponent;
        (Int::ONE, drop)
    };

    let keep_bits = (MB as Exponent + 1) - sub_drop;
    let shift = p_bits - keep_bits;

    let round_bit = fp_bit(&val.sig, shift - 1);
    if !final_round && !RoundingMode::is_faithful(RM) && !RoundingMode::is_stochastic(RM) {
        if err_bits + 2 >= shift {
            return None;
        }
        if RoundingMode::is_nearest(RM)
            && fp_range_all_eq(&val.sig, err_bits + 1, shift - 1, !round_bit)
        {
            return None;
        }
        if RoundingMode::is_directed_or_odd(RM)
            && (fp_range_all_eq(&val.sig, err_bits + 1, shift, false)
                || fp_range_all_eq(&val.sig, err_bits + 1, shift, true))
        {
            return None;
        }
    }

    let mut q = fp_shr_to_uint(&val.sig, shift, limbs);

    // At Stage 2 (high precision), if the fractional bits between the rounding position and
    // the approximation noise floor (`err_bits`) are all 0s or all 1s across at least MB + 64
    // consecutive bits, the true mathematical value is an exact grid point (e.g. `hypot(3, 4) = 5`,
    // `log10(1000) = 3`, `powf(9, 0.5) = 3`) perturbed only by Stage-2 ULP noise; snap it so
    // directed, round-to-odd, and stochastic modes return the exact value.
    if final_round && err_bits > 0 && shift >= err_bits + (MB as Exponent) + 65 {
        if fp_range_all_eq(&val.sig, err_bits + 1, shift, false) {
            return Some(Float::from_biased_uint_parts(
                val.sign,
                out_be.cast_unsigned(),
                q,
            ));
        }
        if fp_range_all_eq(&val.sig, err_bits + 1, shift, true) {
            q = q.add(Uint::ONE);
            if q.bit(MB as Exponent + 1) {
                q = q.shr(1);
                out_be = out_be.add(Int::ONE);
            }
            if out_be.ge(&exp_max) {
                return Some(Float::<W, MB, RM>::overflow_value(val.sign));
            }
            return Some(Float::from_biased_uint_parts(
                val.sign,
                out_be.cast_unsigned(),
                q,
            ));
        }
    }

    let sticky = fp_has_low_bits(&val.sig, shift - 1);
    let frac64 = if RM == RoundingMode::Stochastic as u8 {
        fp_extract_top64(&val.sig, shift, limbs)
    } else {
        0
    };
    let seed = seed_base ^ uint_low_u64(&q);

    if RoundingMode::should_round_up::<RM>(val.sign, q.is_odd(), round_bit, sticky, frac64, seed) {
        q = q.add(Uint::ONE);
        if q.bit(MB as Exponent + 1) {
            q = q.shr(1);
            out_be = out_be.add(Int::ONE);
        }
    }

    if out_be.ge(&exp_max) {
        return Some(Float::<W, MB, RM>::overflow_value(val.sign));
    }

    Some(Float::from_biased_uint_parts(
        val.sign,
        out_be.cast_unsigned(),
        q,
    ))
}
