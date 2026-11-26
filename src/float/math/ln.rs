use super::Float;
use crate::digits::Digits;
use crate::{Exponent, Int, Uint};

/// Fixed-point buffer with `2 * W + 24` bytes.
///
/// The number of full 64-bit limbs `MAX_LIMBS = (2 * W + 24) / 8` satisfies
/// `MAX_LIMBS * 64 >= 16 * W + 136 > 2 * MB + 128` for all `W > 1` and `MB < 8 * W - 2`,
/// and every limb in `0..MAX_LIMBS` is a full, untruncated `u64`.
type Fixed<const W: usize> = Digits<u64, W, 2, 24>;

/// Exact top 512 bits of `ln(2)` (`floor(ln(2) * 2^512)`), most-significant `u64` limb first.
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

/// Conservative error bound exponent (2^24 ULPs of the working precision) used in Ziv's
/// rounding check. Stage 1 provides >= 68 guard bits, leaving >= 44 decisive bits before
/// falling back to Stage 2 at full `MAX_LIMBS` (>= 2 * MB + 136 bits).
const ZIV_ERR_BITS: Exponent = 24;

#[inline(always)]
const fn uint_low_u64<const W: usize>(x: &Uint<W>) -> u64 {
    let mut val = 0u64;
    let mut i = 0;
    while i < W && i < 8 {
        val |= (x.bytes[i] as u64) << (i * 8);
        i += 1;
    }
    val
}

#[inline(always)]
const fn extract_u16<const W: usize>(m: &Uint<W>, low_bit: Exponent) -> u16 {
    let byte_idx = (low_bit / 8) as usize;
    let bit_off = low_bit % 8;
    let b0 = m.bytes[byte_idx] as u32;
    let b1 = if byte_idx + 1 < W {
        m.bytes[byte_idx + 1] as u32
    } else {
        0
    };
    let b2 = if byte_idx + 2 < W {
        m.bytes[byte_idx + 2] as u32
    } else {
        0
    };
    (((b2 << 16) | (b1 << 8) | b0) >> bit_off) as u16
}

#[inline]
const fn fp_is_zero<const W: usize>(a: &Fixed<W>, limbs: usize) -> bool {
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
const fn fp_bit_width<const W: usize>(a: &Fixed<W>, limbs: usize) -> Exponent {
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
const fn fp_add<const W: usize>(a: &Fixed<W>, b: &Fixed<W>, limbs: usize) -> (Fixed<W>, bool) {
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
const fn fp_sub<const W: usize>(a: &Fixed<W>, b: &Fixed<W>, limbs: usize) -> (Fixed<W>, bool) {
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
const fn fp_wrapping_neg<const W: usize>(a: &Fixed<W>, limbs: usize) -> Fixed<W> {
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
const fn fp_shl1_with_bit<const W: usize>(a: &Fixed<W>, low_bit: u64, limbs: usize) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut carry = low_bit & 1;
    let mut i = 0;
    while i < limbs {
        let d = a.get(i);
        out.set(i, (d << 1) | carry);
        carry = d >> 63;
        i += 1;
    }
    debug_assert!(carry == 0);
    out
}

#[inline]
const fn fp_shr<const W: usize>(a: &Fixed<W>, shift: Exponent, limbs: usize) -> Fixed<W> {
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
const fn fp_from_uint_shl<const W: usize>(
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
const fn fp_from_uint_shr<const W: usize>(
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
    while i < limbs {
        let src = i + word_shift;
        if src >= u_limbs {
            break;
        }
        let lo = u_digits.get(src);
        let val = if bit_shift == 0 {
            lo
        } else {
            let hi = if src + 1 < u_limbs {
                u_digits.get(src + 1)
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

/// Computes the high `limbs` words of `a * b` together with one guard limb immediately below
/// (discarding pairs `i + j < limbs - 1`, which introduces at most `limbs` ULPs of truncation
/// error in the returned value).
#[inline]
const fn fp_mul_hi_with_guard<const W: usize>(
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
            debug_assert!(carry == 0);
        }
        i += 1;
    }
    (out, guard)
}

#[inline]
const fn fp_mul_hi<const W: usize>(a: &Fixed<W>, b: &Fixed<W>, limbs: usize) -> Fixed<W> {
    fp_mul_hi_with_guard(a, b, limbs).0
}

#[inline]
const fn fp_mul_u64<const W: usize>(a: &Fixed<W>, rhs: u64, limbs: usize) -> Fixed<W> {
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
    debug_assert!(carry == 0);
    out
}

#[inline]
const fn fp_div_u64<const W: usize>(a: &Fixed<W>, d: u64, limbs: usize) -> Fixed<W> {
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

/// Computes `1 / D` for `D = d * 2^(-64*limbs) in (0.5, 1.0)` via Newton-Raphson,
/// returning `r` such that `R = r * 2^(-(64*limbs - 1)) in (1.0, 2.0)`.
const fn fp_recip<const W: usize>(d: &Fixed<W>, limbs: usize) -> Fixed<W> {
    let d_hi = d.get(limbs - 1);
    debug_assert!(d_hi >= (1u64 << 63));
    let q = (((1u128 << 127) - 1) / ((d_hi as u128) + 1)) as u64;
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
    let mut n: u64 = 1;
    loop {
        p = fp_div_u64(&p, d2, limbs);
        if fp_is_zero(&p, limbs) {
            break;
        }
        let term = fp_div_u64(&p, 2 * n + 1, limbs);
        let (new_sum, carry) = fp_add(&sum, &term, limbs);
        debug_assert!(!carry);
        sum = new_sum;
        n += 1;
    }
    sum
}

/// Computes `ln(2)` at scale `2^(64 * limbs)` via the Machin-like identity
/// `ln(2) = 18 * atanh(1/26) - 2 * atanh(1/4801) + 8 * atanh(1/8749)`.
const fn compute_ln2_machin<const W: usize>(limbs: usize) -> Fixed<W> {
    let a = fp_atanh_inv_u64::<W>(26, limbs);
    let b = fp_atanh_inv_u64::<W>(4801, limbs);
    let c = fp_atanh_inv_u64::<W>(8749, limbs);

    let t1 = fp_mul_u64(&a, 18, limbs);
    let t2 = fp_mul_u64(&c, 8, limbs);
    let t3 = fp_mul_u64(&b, 2, limbs);

    let (pos, carry) = fp_add(&t1, &t2, limbs);
    debug_assert!(!carry);
    let (res, borrow) = fp_sub(&pos, &t3, limbs);
    debug_assert!(!borrow);
    res
}

/// Returns `ln(2)` at scale `2^(64 * limbs)`. Uses the precomputed 512-bit table for
/// `limbs <= 8` and falls back to the Machin-like formula for higher precisions.
#[inline]
const fn fp_ln2<const W: usize>(limbs: usize) -> Fixed<W> {
    if limbs <= LN2_512_BE.len() {
        let mut out = Fixed::<W>::ALL_ZEROS;
        let mut i = 0;
        while i < limbs {
            out.set(limbs - 1 - i, LN2_512_BE[i]);
            i += 1;
        }
        out
    } else {
        compute_ln2_machin(limbs)
    }
}

/// Adds `2^bit_idx` to `(val + carry * 2^(64*limbs))`.
#[inline]
const fn fp_add_pow2_with_carry<const W: usize>(
    val: &Fixed<W>,
    mut carry_hi: u64,
    bit_idx: Exponent,
    limbs: usize,
) -> (Fixed<W>, u64) {
    let mut out = *val;
    let word_idx = (bit_idx / 64) as usize;
    let bit_off = bit_idx % 64;
    debug_assert!(word_idx < limbs);

    let (s, mut c) = out.get(word_idx).overflowing_add(1u64 << bit_off);
    out.set(word_idx, s);
    let mut i = word_idx + 1;
    while c && i < limbs {
        let (s2, c2) = out.get(i).overflowing_add(1);
        out.set(i, s2);
        c = c2;
        i += 1;
    }
    if c {
        carry_hi += 1;
    }
    (out, carry_hi)
}

/// Subtracts `2^bit_idx` from `(val + carry * 2^(64*limbs))` (caller guarantees no underflow).
#[inline]
const fn fp_sub_pow2_with_carry<const W: usize>(
    val: &Fixed<W>,
    mut carry_hi: u64,
    bit_idx: Exponent,
    limbs: usize,
) -> (Fixed<W>, u64) {
    let mut out = *val;
    let word_idx = (bit_idx / 64) as usize;
    let bit_off = bit_idx % 64;
    debug_assert!(word_idx < limbs);

    let (d, mut b) = out.get(word_idx).overflowing_sub(1u64 << bit_off);
    out.set(word_idx, d);
    let mut i = word_idx + 1;
    while b && i < limbs {
        let (d2, b2) = out.get(i).overflowing_sub(1);
        out.set(i, d2);
        b = b2;
        i += 1;
    }
    if b {
        debug_assert!(carry_hi > 0);
        carry_hi -= 1;
    }
    (out, carry_hi)
}

/// Right-shifts `(val + carry * 2^(64 * limbs))` by `shift` bits and writes the result into
/// `Uint<W>` (caller guarantees the shifted result fits in `Uint<W>`).
#[inline]
const fn fp_shr_with_carry_to_uint<const W: usize>(
    val: &Fixed<W>,
    carry: u64,
    shift: Exponent,
    limbs: usize,
) -> Uint<W> {
    let mut out = Digits::<u64, W, 1, 0>::ALL_ZEROS;
    let out_limbs = W.div_ceil(8);
    let word_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;

    let mut i = 0;
    while i < out_limbs {
        let src = i + word_shift;
        let lo = if src < limbs {
            val.get(src)
        } else if src == limbs {
            carry
        } else {
            0
        };
        let word = if bit_shift == 0 {
            lo
        } else {
            let hi = if src + 1 < limbs {
                val.get(src + 1)
            } else if src + 1 == limbs {
                carry
            } else {
                0
            };
            (lo >> bit_shift) | (hi << (64 - bit_shift))
        };
        if word != 0 {
            out.set(i, word);
        }
        i += 1;
    }
    Uint::<W>::from_digits(out)
}

/// Computes `S = (2 * atanh(num / den)) * 2^(z_shift - 3)` at scale `2^(64 * limbs)`,
/// where `z_shift = den.bit_width() - num.bit_width() >= 2` and `num / den < 0.172`.
///
/// The returned value `sum` satisfies `0.125 < sum / 2^(64*limbs) < 0.51`, so it never
/// overflows and preserves full `64 * limbs`-bit relative precision even when `num / den` is tiny.
/// The true magnitude is `|ln(f)| = sum * 2^(3 - z_shift - 64 * limbs)`.
const fn fp_atanh_norm<const W: usize>(
    num: &Uint<W>,
    den: &Uint<W>,
    z_shift: Exponent,
    limbs: usize,
) -> Fixed<W> {
    let p_bits = (limbs as Exponent) * 64;
    let num_bw = num.bit_width();
    let den_bw = den.bit_width();

    // D in (0.5, 1.0) at scale 2^P; N in [0.125, 0.25) at scale 2^P.
    let d = fp_from_uint_shl(den, p_bits - den_bw, limbs);
    let n = fp_from_uint_shl(num, p_bits - 2 - num_bw, limbs);

    // r ≈ 1 / D in (1.0, 2.0) at scale 2^(P - 1).
    let r = fp_recip(&d, limbs);

    // z_norm = (N * r) >> (P - 1) = (num / den) * 2^(z_shift - 2) in (0.125, 0.5) at scale 2^P.
    let (z_hi, z_guard) = fp_mul_hi_with_guard(&n, &r, limbs);
    let z_norm = fp_shl1_with_bit(&z_hi, z_guard >> 63, limbs);

    // z^2 = z_norm^2 * 2^(-2 * (z_shift - 2)) at scale 2^P.
    let sq_shift = 2 * (z_shift - 2);
    if sq_shift >= p_bits {
        return z_norm;
    }
    let z_norm_sq = fp_mul_hi(&z_norm, &z_norm, limbs);
    let z2 = fp_shr(&z_norm_sq, sq_shift, limbs);
    if fp_is_zero(&z2, limbs) {
        return z_norm;
    }

    let mut sum = z_norm;
    let mut term = z_norm;
    let mut k: u64 = 1;
    loop {
        term = fp_mul_hi(&term, &z2, limbs);
        if fp_is_zero(&term, limbs) {
            break;
        }
        let div_term = fp_div_u64(&term, 2 * k + 1, limbs);
        if fp_is_zero(&div_term, limbs) {
            break;
        }
        let (new_sum, carry) = fp_add(&sum, &div_term, limbs);
        debug_assert!(!carry);
        sum = new_sum;
        k += 1;
    }

    sum
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Evaluates `|ln(x)|` at `limbs` 64-bit limbs of working precision and attempts to round
    /// to `Self`. Returns `None` if Ziv's error bound crosses a half-ULP midpoint (only checked
    /// when `check_ziv` is true, prompting a retry at higher precision).
    const fn ln_at_precision(
        k: Int<W>,
        num: Uint<W>,
        den: Uint<W>,
        ln_f_neg: bool,
        bias_signed: Int<W>,
        min_normal_exp: Int<W>,
        limbs: usize,
        check_ziv: bool,
    ) -> Option<Self> {
        let p_bits = (limbs as Exponent) * 64;

        let (out_sign, mag, out_exp): (bool, Fixed<W>, Int<W>) = if k.is_zero() {
            // x is close to 1 (f in [0.7071, 1.4143), f != 1): zero cancellation.
            let z_shift = den.bit_width() - num.bit_width();
            let sum = fp_atanh_norm(&num, &den, z_shift, limbs);
            let bw = fp_bit_width(&sum, limbs);
            let lz = p_bits - bw;
            // |ln(x)| = sum * 2^(3 - z_shift - P) = (sum / 2^(bw - 1)) * 2^(2 - z_shift - lz)
            let exp_neg = z_shift + lz - 2;
            let exp = Uint::cast_from_unsigned_float_exponent(exp_neg)
                .cast_signed()
                .neg();
            (ln_f_neg, sum, exp)
        } else {
            let k_neg = k.is_negative();
            let k_abs = k.unsigned_abs();
            let k_bw = k_abs.bit_width();

            let ln2 = fp_ln2::<W>(limbs);
            let k_norm = if k_bw <= p_bits - 1 {
                fp_from_uint_shl(&k_abs, (p_bits - 1) - k_bw, limbs)
            } else {
                fp_from_uint_shr(&k_abs, k_bw - (p_bits - 1), limbs)
            };
            let kln2 = fp_mul_hi(&k_norm, &ln2, limbs);

            let mag = if num.is_zero() {
                kln2
            } else {
                let z_shift = den.bit_width() - num.bit_width();
                let align_shift = k_bw + z_shift - 2;
                if align_shift >= p_bits {
                    kln2
                } else {
                    let sum = fp_atanh_norm(&num, &den, z_shift, limbs);
                    let lnf_aligned = fp_shr(&sum, align_shift, limbs);
                    if k_neg == ln_f_neg {
                        let (combined, carry) = fp_add(&kln2, &lnf_aligned, limbs);
                        debug_assert!(!carry);
                        combined
                    } else {
                        let (combined, borrow) = fp_sub(&kln2, &lnf_aligned, limbs);
                        debug_assert!(!borrow);
                        combined
                    }
                }
            };

            let bw = fp_bit_width(&mag, limbs);
            let lz = p_bits - bw;
            // |ln(x)| = mag * 2^(k_bw + 1 - P) = (mag / 2^(bw - 1)) * 2^(k_bw - lz)
            let exp = if k_bw >= lz {
                Uint::cast_from_unsigned_float_exponent(k_bw - lz).cast_signed()
            } else {
                Uint::cast_from_unsigned_float_exponent(lz - k_bw)
                    .cast_signed()
                    .neg()
            };
            (k_neg, mag, exp)
        };

        // Overflow check (can occur for negative ln(subnormal) when EXPONENT_BITS is small).
        if out_exp.gt(&bias_signed) {
            return Some(if out_sign {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            });
        }

        let bw = fp_bit_width(&mag, limbs);
        let is_subnormal = out_exp.lt(&min_normal_exp);
        let drop: Exponent = if is_subnormal {
            let diff = min_normal_exp
                .sub(out_exp)
                .cast_unsigned()
                .cast_to_unsigned_float_exponent();
            if diff > Self::MB + 1 {
                return Some(if out_sign {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                });
            }
            bw + diff - (Self::MB + 1)
        } else {
            bw - (Self::MB + 1)
        };

        // Round to nearest (exact half-ULP ties are impossible for transcendental ln(x), x != 1).
        let (mid, carry_mid) = fp_add_pow2_with_carry(&mag, 0, drop - 1, limbs);
        let mut rounded_mant = if check_ziv {
            debug_assert!(drop > ZIV_ERR_BITS + 1);
            let (hi, carry_hi) = fp_add_pow2_with_carry(&mid, carry_mid, ZIV_ERR_BITS, limbs);
            let (lo, carry_lo) = fp_sub_pow2_with_carry(&mid, carry_mid, ZIV_ERR_BITS, limbs);
            let q_hi = fp_shr_with_carry_to_uint(&hi, carry_hi, drop, limbs);
            let q_lo = fp_shr_with_carry_to_uint(&lo, carry_lo, drop, limbs);
            if q_hi.ne(&q_lo) {
                return None;
            }
            q_lo
        } else {
            fp_shr_with_carry_to_uint(&mid, carry_mid, drop, limbs)
        };

        let (biased_out_exp, mant_bits): (Uint<W>, Uint<W>) = if is_subnormal {
            if rounded_mant.bit(Self::MB) {
                // Rounded up across subnormal-to-normal boundary to MIN_POSITIVE.
                (Uint::ONE, Uint::ZERO)
            } else {
                (Uint::ZERO, rounded_mant)
            }
        } else {
            let mut final_exp = out_exp;
            if rounded_mant.bit(Self::MB + 1) {
                final_exp = final_exp.add(Int::ONE);
                if final_exp.gt(&bias_signed) {
                    return Some(if out_sign {
                        Self::NEG_INFINITY
                    } else {
                        Self::INFINITY
                    });
                }
                (final_exp.add(bias_signed).cast_unsigned(), Uint::ZERO)
            } else {
                rounded_mant.set_bit(Self::MB, false);
                (final_exp.add(bias_signed).cast_unsigned(), rounded_mant)
            }
        };

        let mut out_bits = biased_out_exp.shl(Self::MB).bitor(mant_bits);
        if out_sign {
            out_bits.set_bit(Self::BITS - 1, true);
        }
        Some(Self::from_bits(out_bits))
    }

    /// Computes the natural logarithm `ln(self)` for `Float<W, MB>`.
    ///
    /// Valid for all `W > 1` and `2 < MB < Self::BITS - 2`, including formats with small exponent
    /// widths (handling subnormal results and overflow of `ln(subnormal)` to `-∞`) and formats
    /// with arbitrarily large exponent widths (`EXPONENT_BITS >= 32`).
    pub(super) const fn ln_internal(self) -> Self {
        const {
            assert!(W > 1, "Float byte width W must be greater than 1");
            assert!(
                MB > 2 && MB < (Self::BITS as usize) - 2,
                "Float mantissa bits MB must satisfy 2 < MB < Self::BITS - 2"
            );
        }

        if self.is_nan() {
            return self;
        }
        if self.is_zero() {
            return Self::NEG_INFINITY;
        }
        if self.is_sign_negative() {
            return Self::NAN;
        }
        if !self.is_finite() {
            // +INFINITY
            return self;
        }

        let raw_exp = self.bits.bitand(Self::SIGN_MASK).shr(Self::MB);
        let raw_mant = self.bits.bitand(Self::MANTISSA_MASK);

        // Compute bias in `Uint<W>` directly so this works for any `EXPONENT_BITS` (even >= 32).
        let bias: Uint<W> = Uint::power_of_two(Self::EXPONENT_BITS - 1).sub(Uint::ONE);
        if raw_exp.eq(&bias) && raw_mant.is_zero() {
            // ln(1.0) == +0.0
            return Self::ZERO;
        }

        let bias_signed: Int<W> = bias.cast_signed();
        let min_normal_exp: Int<W> = Int::ONE.sub(bias_signed);

        // Normalise self = (m / 2^MB) * 2^e with 2^MB <= m < 2^(MB + 1).
        let (e, m): (Int<W>, Uint<W>) = if raw_exp.is_zero() {
            let norm_shift = Self::MB + 1 - raw_mant.bit_width();
            let norm_mant = raw_mant.shl(norm_shift);
            let shift_int = Uint::cast_from_unsigned_float_exponent(norm_shift).cast_signed();
            (min_normal_exp.sub(shift_int), norm_mant)
        } else {
            let norm_mant = raw_mant.bitor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
            (raw_exp.cast_signed().sub(bias_signed), norm_mant)
        };

        // Centre the significand around 1 using threshold sqrt(2) ≈ 0xB505 / 2^15:
        //   self = f * 2^k, with f in [0.7071, 1.4143), so |z| = |f - 1| / (f + 1) < 0.172.
        let top16: u16 = if Self::MB >= 15 {
            extract_u16(&m, Self::MB - 15)
        } else {
            (uint_low_u64(&m) as u16) << (15 - Self::MB)
        };

        let (k, num, den, ln_f_neg): (Int<W>, Uint<W>, Uint<W>, bool) = if top16 >= 0xB505 {
            let two_pow_mb1 = Uint::power_of_two(Self::MB + 1);
            (e.add(Int::ONE), two_pow_mb1.sub(m), two_pow_mb1.add(m), true)
        } else {
            let two_pow_mb = Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK;
            (e, m.sub(two_pow_mb), m.add(two_pow_mb), false)
        };

        let max_limbs = (2 * W + 24) / 8;
        let l1 = (MB + 72).div_ceil(64);
        debug_assert!(l1 < max_limbs);

        if let Some(res) =
            Self::ln_at_precision(k, num, den, ln_f_neg, bias_signed, min_normal_exp, l1, true)
        {
            return res;
        }

        // Stage 2 (Ziv fallback): re-evaluate at full precision (`>= 2 * MB + 136` bits).
        match Self::ln_at_precision(
            k,
            num,
            den,
            ln_f_neg,
            bias_signed,
            min_normal_exp,
            max_limbs,
            false,
        ) {
            Some(res) => res,
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{F16, F32, F64, F80, F128, F256};

    /// Independent high-precision fixed-point oracle (`BITS = 64 * NL`) using:
    /// - `ln(2) = sum_{n >= 1} 1 / (n * 2^n)` (independent of both `LN2_512_BE` and Machin), and
    /// - `exp(delta) = sum_{n >= 0} delta^n / n!` (independent of `atanh` and `fp_recip`)
    /// to verify `exp(m_lo - e * ln(2)) < m_real < exp(m_hi - e * ln(2))`, proving exact 0.5-ULP
    /// IEEE 754 round-to-nearest-ties-to-even for arbitrary `Float<W, MB>`.
    mod oracle {
        use super::*;

        // Fixed-point unsigned integer of `NL` limbs, representing `val * 2^(-64 * FRAC_LIMBS)`.
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub(super) struct BigFp<const NL: usize, const FRAC_LIMBS: usize>([u64; NL]);

        impl<const NL: usize, const FRAC_LIMBS: usize> BigFp<NL, FRAC_LIMBS> {
            const ZERO: Self = Self([0; NL]);
            const FRAC: usize = FRAC_LIMBS * 64;

            fn one() -> Self {
                let mut a = [0u64; NL];
                a[FRAC_LIMBS] = 1;
                Self(a)
            }

            fn is_zero(&self) -> bool {
                self.0.iter().all(|&x| x == 0)
            }

            fn cmp_fp(&self, other: &Self) -> core::cmp::Ordering {
                for i in (0..NL).rev() {
                    if self.0[i] != other.0[i] {
                        return self.0[i].cmp(&other.0[i]);
                    }
                }
                core::cmp::Ordering::Equal
            }

            fn add(&self, rhs: &Self) -> Self {
                let mut out = [0u64; NL];
                let mut carry = false;
                for i in 0..NL {
                    let (s1, c1) = self.0[i].overflowing_add(rhs.0[i]);
                    let (s2, c2) = s1.overflowing_add(carry as u64);
                    out[i] = s2;
                    carry = c1 || c2;
                }
                assert!(!carry, "oracle BigFp::add overflow");
                Self(out)
            }

            fn sub(&self, rhs: &Self) -> Self {
                let mut out = [0u64; NL];
                let mut borrow = false;
                for i in 0..NL {
                    let (d1, b1) = self.0[i].overflowing_sub(rhs.0[i]);
                    let (d2, b2) = d1.overflowing_sub(borrow as u64);
                    out[i] = d2;
                    borrow = b1 || b2;
                }
                assert!(!borrow, "oracle BigFp::sub underflow");
                Self(out)
            }

            fn shr(&self, shift: usize) -> Self {
                if shift >= NL * 64 {
                    return Self::ZERO;
                }
                let mut out = [0u64; NL];
                let ws = shift / 64;
                let bs = shift % 64;
                for i in 0..(NL - ws) {
                    let lo = self.0[i + ws];
                    out[i] = if bs == 0 {
                        lo
                    } else {
                        let hi = if i + ws + 1 < NL { self.0[i + ws + 1] } else { 0 };
                        (lo >> bs) | (hi << (64 - bs))
                    };
                }
                Self(out)
            }

            fn shl(&self, shift: usize) -> Self {
                assert!(shift < NL * 64);
                let mut out = [0u64; NL];
                let ws = shift / 64;
                let bs = shift % 64;
                for i in (ws..NL).rev() {
                    let hi = self.0[i - ws];
                    out[i] = if bs == 0 {
                        hi
                    } else {
                        let lo = if i > ws { self.0[i - ws - 1] } else { 0 };
                        (hi << bs) | (lo >> (64 - bs))
                    };
                }
                Self(out)
            }

            fn div_u64(&self, d: u64) -> Self {
                let mut out = [0u64; NL];
                let mut rem: u128 = 0;
                for i in (0..NL).rev() {
                    let cur = (rem << 64) | (self.0[i] as u128);
                    out[i] = (cur / (d as u128)) as u64;
                    rem = cur % (d as u128);
                }
                Self(out)
            }

            fn mul_fp(&self, rhs: &Self) -> Self {
                let mut out = [0u64; NL];
                let mut guard = [0u64; NL];
                for i in 0..NL {
                    let ai = self.0[i] as u128;
                    if ai == 0 {
                        continue;
                    }
                    let mut carry: u128 = 0;
                    for j in 0..NL {
                        let idx = i + j;
                        let prod = ai * (rhs.0[j] as u128) + carry;
                        if idx < FRAC_LIMBS {
                            let sum = prod + (guard[idx] as u128);
                            guard[idx] = sum as u64;
                            carry = sum >> 64;
                        } else if idx - FRAC_LIMBS < NL {
                            let o = idx - FRAC_LIMBS;
                            let sum = prod + (out[o] as u128);
                            out[o] = sum as u64;
                            carry = sum >> 64;
                        }
                    }
                    let mut idx = i + NL;
                    while carry != 0 && idx - FRAC_LIMBS < NL {
                        if idx < FRAC_LIMBS {
                            let sum = (guard[idx] as u128) + carry;
                            guard[idx] = sum as u64;
                            carry = sum >> 64;
                        } else {
                            let o = idx - FRAC_LIMBS;
                            let sum = (out[o] as u128) + carry;
                            out[o] = sum as u64;
                            carry = sum >> 64;
                        }
                        idx += 1;
                    }
                }
                Self(out)
            }

            /// Computes `ln(2) = sum_{n >= 1} 1 / (n * 2^n)` independently.
            pub(super) fn ln2() -> Self {
                let mut sum = Self::ZERO;
                let mut pow2 = Self::one();
                for n in 1u64..=(Self::FRAC as u64 + 64) {
                    pow2 = pow2.shr(1);
                    if pow2.is_zero() {
                        break;
                    }
                    sum = sum.add(&pow2.div_u64(n));
                }
                sum
            }

            /// Computes `exp(delta_abs)` where `0 <= delta_abs <= 2.0`.
            fn exp_pos(delta_abs: &Self) -> Self {
                let d = delta_abs.shr(4);
                let mut sum = Self::one();
                let mut term = Self::one();
                let mut n = 1u64;
                loop {
                    term = term.mul_fp(&d).div_u64(n);
                    if term.is_zero() {
                        break;
                    }
                    sum = sum.add(&term);
                    n += 1;
                }
                for _ in 0..4 {
                    sum = sum.mul_fp(&sum);
                }
                sum
            }

            /// Builds `u * 2^exp_shift` as a `BigFp` from `Uint<W>`.
            fn from_uint_scaled<const W: usize>(u: &Uint<W>, exp_shift: i64) -> Self {
                let mut raw = [0u64; NL];
                let u_limbs = W.div_ceil(8);
                let ud = u.as_digits::<u64>();
                for i in 0..u_limbs {
                    raw[i] = ud.get(i);
                }
                let total_shift = (Self::FRAC as i64) + exp_shift;
                if total_shift >= 0 {
                    Self(raw).shl(total_shift as usize)
                } else {
                    Self(raw).shr((-total_shift) as usize)
                }
            }
        }

        /// Signed big fixed-point number `sign * mag`.
        #[derive(Clone, Copy)]
        struct SignedFp<const NL: usize, const FRAC_LIMBS: usize> {
            neg: bool,
            mag: BigFp<NL, FRAC_LIMBS>,
        }

        impl<const NL: usize, const FRAC_LIMBS: usize> SignedFp<NL, FRAC_LIMBS> {
            fn sub_signed(self, other: Self) -> Self {
                let neg_other = Self {
                    neg: !other.neg,
                    mag: other.mag,
                };
                if self.neg == neg_other.neg {
                    Self {
                        neg: self.neg,
                        mag: self.mag.add(&neg_other.mag),
                    }
                } else {
                    match self.mag.cmp_fp(&neg_other.mag) {
                        core::cmp::Ordering::Greater => Self {
                            neg: self.neg,
                            mag: self.mag.sub(&neg_other.mag),
                        },
                        core::cmp::Ordering::Less => Self {
                            neg: neg_other.neg,
                            mag: neg_other.mag.sub(&self.mag),
                        },
                        core::cmp::Ordering::Equal => Self {
                            neg: false,
                            mag: BigFp::ZERO,
                        },
                    }
                }
            }
        }

        /// Verifies `y = x.ln_internal()` is the exact IEEE 754 round-to-nearest-ties-to-even
        /// natural logarithm of `x` for any `Float<W, MB>` whose exponent fits in `i64`.
        pub(super) fn assert_correctly_rounded<
            const W: usize,
            const MB: usize,
            const NL: usize,
            const FRAC_LIMBS: usize,
        >(
            x: Float<W, MB>,
            ln2_oracle: &BigFp<NL, FRAC_LIMBS>,
        ) {
            let y = x.ln_internal();

            if x.is_nan() {
                assert!(y.is_nan());
                assert_eq!(y.to_bits(), x.to_bits());
                return;
            }
            if x.is_zero() {
                assert_eq!(y.to_bits(), Float::<W, MB>::NEG_INFINITY.to_bits());
                return;
            }
            if x.is_sign_negative() {
                assert!(y.is_nan());
                return;
            }
            if !x.is_finite() {
                assert_eq!(y.to_bits(), Float::<W, MB>::INFINITY.to_bits());
                return;
            }

            let bias: Uint<W> =
                Uint::power_of_two(Float::<W, MB>::EXPONENT_BITS - 1).sub(Uint::ONE);
            let raw_exp = x.bits.bitand(Float::<W, MB>::SIGN_MASK).shr(MB as Exponent);
            let raw_mant = x.bits.bitand(Float::<W, MB>::MANTISSA_MASK);
            if raw_exp.eq(&bias) && raw_mant.is_zero() {
                assert_eq!(y.to_bits(), Float::<W, MB>::ZERO.to_bits());
                return;
            }

            let bias_s = bias.cast_signed();
            let min_normal_exp = Int::ONE.sub(bias_s);

            let (e_int, mx) = if raw_exp.is_zero() {
                let s = MB as Exponent + 1 - raw_mant.bit_width();
                (
                    min_normal_exp.sub(Uint::cast_from_unsigned_float_exponent(s).cast_signed()),
                    raw_mant.shl(s),
                )
            } else {
                (
                    raw_exp.cast_signed().sub(bias_s),
                    raw_mant.bitor(Float::<W, MB>::MANTISSA_IMPLICIT_LEADING_ONE_MASK),
                )
            };

            let e_i64 = int_to_i64(&e_int);
            let m_real = BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(&mx, -(MB as i64));

            let e_abs = e_i64.unsigned_abs();
            let e_fp = BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(
                &Uint::<8>::from_le_bytes(e_abs.to_le_bytes()),
                0,
            );
            let e_ln2 = SignedFp {
                neg: e_i64 < 0,
                mag: e_fp.mul_fp(ln2_oracle),
            };

            let check_val_vs_x = |v: SignedFp<NL, FRAC_LIMBS>| -> core::cmp::Ordering {
                let delta = v.sub_signed(e_ln2);
                let two = BigFp::<NL, FRAC_LIMBS>::one().shl(1);
                if delta.mag.cmp_fp(&two).is_ge() {
                    return if delta.neg {
                        core::cmp::Ordering::Less
                    } else {
                        core::cmp::Ordering::Greater
                    };
                }
                if delta.neg {
                    return core::cmp::Ordering::Less;
                }
                let exp_d = BigFp::exp_pos(&delta.mag);
                exp_d.cmp_fp(&m_real)
            };

            let float_to_midpoints =
                |f: Float<W, MB>| -> (SignedFp<NL, FRAC_LIMBS>, SignedFp<NL, FRAC_LIMBS>) {
                    let neg = f.is_sign_negative();
                    let re = f.bits.bitand(Float::<W, MB>::SIGN_MASK).shr(MB as Exponent);
                    let rm = f.bits.bitand(Float::<W, MB>::MANTISSA_MASK);
                    let (m_u, e_lsb, is_gt_min_normal_pow2) = if re.is_zero() {
                        let e_sub = int_to_i64(&min_normal_exp) - (MB as i64);
                        (rm, e_sub, false)
                    } else {
                        let e_n = int_to_i64(&re.cast_signed().sub(bias_s)) - (MB as i64);
                        let m_n = rm.bitor(Float::<W, MB>::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
                        (m_n, e_n, rm.is_zero() && re.gt(&Uint::ONE))
                    };

                    let m_hi_mag =
                        BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(&m_u.shl(1).add(Uint::ONE), e_lsb - 1);
                    let m_lo_mag = if is_gt_min_normal_pow2 {
                        BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(&m_u.shl(2).sub(Uint::ONE), e_lsb - 2)
                    } else {
                        BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(&m_u.shl(1).sub(Uint::ONE), e_lsb - 1)
                    };

                    if neg {
                        (
                            SignedFp {
                                neg: true,
                                mag: m_hi_mag,
                            },
                            SignedFp {
                                neg: true,
                                mag: m_lo_mag,
                            },
                        )
                    } else {
                        (
                            SignedFp {
                                neg: false,
                                mag: m_lo_mag,
                            },
                            SignedFp {
                                neg: false,
                                mag: m_hi_mag,
                            },
                        )
                    }
                };

            if y.is_infinite() {
                assert!(
                    y.is_sign_negative(),
                    "ln of finite float cannot overflow to +inf"
                );
                let max_u = Uint::<W>::power_of_two(MB as Exponent + 2).sub(Uint::ONE);
                let max_mid = SignedFp {
                    neg: true,
                    mag: BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(
                        &max_u,
                        int_to_i64(&bias_s) - (MB as i64) - 1,
                    ),
                };
                assert!(
                    check_val_vs_x(max_mid).is_ge(),
                    "ln(x) overflowed to -inf prematurely for x={:?}",
                    x.to_bits()
                );
                return;
            }

            assert!(y.is_finite() && !y.is_zero());
            let (lo, hi) = float_to_midpoints(y);
            assert!(
                check_val_vs_x(lo).is_lt(),
                "y={:?} is too large for x={:?}",
                y.to_bits(),
                x.to_bits()
            );
            assert!(
                check_val_vs_x(hi).is_gt(),
                "y={:?} is too small for x={:?}",
                y.to_bits(),
                x.to_bits()
            );
        }

        fn int_to_i64<const W: usize>(x: &Int<W>) -> i64 {
            let neg = x.is_negative();
            let abs = x.unsigned_abs();
            let mag = uint_low_u64(&abs) as i64;
            if neg { -mag } else { mag }
        }
    }

    #[test]
    fn machin_ln2_matches_table_and_independent_series() {
        let machin_8 = compute_ln2_machin::<32>(8);
        let table_8 = fp_ln2::<32>(8);
        for i in 1..8 {
            assert_eq!(machin_8.get(i), table_8.get(i), "mismatch at limb {i}");
        }
        // ~55 truncated terms multiplied by 18 can drift by ~1000 ULPs in the lowest limb
        assert!(
            machin_8.get(0).abs_diff(table_8.get(0)) <= 2048,
            "low limb drift too large: {}",
            machin_8.get(0).abs_diff(table_8.get(0))
        );

        let machin_12 = compute_ln2_machin::<64>(12);
        for i in 0..8 {
            assert_eq!(
                machin_12.get(11 - i),
                LN2_512_BE[i],
                "768-bit Machin mismatch at top limb {i}"
            );
        }
    }

    #[test]
    fn exhaustive_w2_formats() {
        // 192-bit oracle (128 fractional bits) is fast even in debug builds and vastly exceeds
        // worst-case TMD for all 16-bit formats.
        let ln2 = oracle::BigFp::<3, 2>::ln2();

        for bits in 0u16..=u16::MAX {
            let u = Uint::<2>::from_le_bytes(bits.to_le_bytes());
            oracle::assert_correctly_rounded::<2, 3, 3, 2>(Float::<2, 3>::from_bits(u), &ln2);
            oracle::assert_correctly_rounded::<2, 10, 3, 2>(F16::from_bits(u), &ln2);
            oracle::assert_correctly_rounded::<2, 12, 3, 2>(Float::<2, 12>::from_bits(u), &ln2);
            oracle::assert_correctly_rounded::<2, 13, 3, 2>(Float::<2, 13>::from_bits(u), &ln2);
        }
    }

    #[test]
    fn special_values_const_and_hard_f64_cases() {
        // 320-bit oracle (256 fractional bits) easily resolves 118-bit Lefèvre-Muller f64 cases.
        let ln2 = oracle::BigFp::<5, 4>::ln2();

        assert_eq!(F64::ZERO.ln_internal().to_bits(), F64::NEG_INFINITY.to_bits());
        assert_eq!(
            F64::NEG_ZERO.ln_internal().to_bits(),
            F64::NEG_INFINITY.to_bits()
        );
        assert!(F64::NEG_ONE.ln_internal().is_nan());
        assert!(F64::NEG_INFINITY.ln_internal().is_nan());
        assert!(F64::NAN.ln_internal().is_nan());
        assert_eq!(
            F64::INFINITY.ln_internal().to_bits(),
            F64::INFINITY.to_bits()
        );
        assert_eq!(F64::ONE.ln_internal().to_bits(), F64::ZERO.to_bits());

        let cases: &[f64] = &[
            f64::from_bits(1),                          // MIN_POSITIVE_SUBNORMAL
            f64::from_bits(0x000f_ffff_ffff_ffff),      // MAX_SUBNORMAL
            f64::MIN_POSITIVE,
            f64::MAX,
            0.5,
            2.0,
            4.0,
            8.0,
            core::f64::consts::E,
            core::f64::consts::SQRT_2,
            f64::from_bits(core::f64::consts::SQRT_2.to_bits() - 1),
            f64::from_bits(core::f64::consts::SQRT_2.to_bits() + 1),
            f64::from_bits(1.0_f64.to_bits() + 1),      // 1 + 2^-52
            f64::from_bits(1.0_f64.to_bits() + 2),      // 1 + 2^-51
            f64::from_bits(1.0_f64.to_bits() - 1),      // 1 - 2^-53
            f64::from_bits(1.0_f64.to_bits() - 2),      // 1 - 2^-52
            f64::from_bits(0x3ff0_0000_0000_0001),
            f64::from_bits(0x3fef_ffff_ffff_ffff),
            // Hard-to-round f64 values (including Lefèvre-Muller worst cases for ln)
            f64::from_bits(0x3fe6_2e42_fefa_39ef),
            f64::from_bits(0x3ff6_a09e_667f_3bcd),
            f64::from_bits(0x3ff1_62a6_e7d3_1107),
            f64::from_bits(0x3ff7_4847_9f1e_767e),
            f64::from_bits(0x3feb_a5c8_1234_5678),
            f64::from_bits(0x3ff1_e080_0544_08f5),
            f64::from_bits(0x3fec_7016_0d6b_c4dc),
        ];

        for &v in cases {
            let f = F64::from(v);
            oracle::assert_correctly_rounded::<8, 52, 5, 4>(f, &ln2);
            let ours = u64::from_le_bytes(f.ln_internal().to_bits().to_bytes());
            let theirs = v.ln().to_bits();
            assert!(
                ours.abs_diff(theirs) <= 1,
                "f64::ln diff > 1 ULP on {v:?}: ours={ours:#x}, std={theirs:#x}"
            );
        }

        const CONST_LN_2: F64 = {
            let bits = Uint::<8>::from_le_bytes(2.0_f64.to_bits().to_le_bytes());
            F64::from_bits(bits).ln_internal()
        };
        assert_eq!(
            CONST_LN_2.to_bits(),
            Uint::<8>::from_le_bytes(core::f64::consts::LN_2.to_bits().to_le_bytes())
        );
    }

    #[test]
    fn f32_and_f64_random_oracle() {
        let ln2 = oracle::BigFp::<5, 4>::ln2();
        let mut state = 0x2468_ace0_1357_9bdf_u64;
        let mut next_u64 = || -> u64 {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };

        for i in 0..2_000 {
            let r = next_u64();

            let bits32 = match i % 4 {
                0 => (r as u32) & 0x7fff_ffff,
                1 => (r as u32) & 0x007f_ffff,
                2 => 0x3f80_0000_u32.wrapping_add((r as u32) & 0x000f_ffff),
                _ => 0x3f80_0000_u32.wrapping_sub(((r as u32) & 0x000f_ffff) + 1),
            };
            let f32_val = F32::from_bits(Uint::<4>::from_le_bytes(bits32.to_le_bytes()));
            oracle::assert_correctly_rounded::<4, 23, 5, 4>(f32_val, &ln2);
            let std32 = f32::from_bits(bits32);
            if !std32.is_nan() {
                let ours = u32::from_le_bytes(f32_val.ln_internal().to_bits().to_bytes());
                let theirs = std32.ln().to_bits();
                assert!(ours.abs_diff(theirs) <= 1);
            }

            let bits64 = match i % 4 {
                0 => r & 0x7fff_ffff_ffff_ffff,
                1 => r & 0x000f_ffff_ffff_ffff,
                2 => 0x3ff0_0000_0000_0000_u64.wrapping_add(r & 0x000f_ffff_ffff_ffff),
                _ => 0x3ff0_0000_0000_0000_u64.wrapping_sub((r & 0x000f_ffff_ffff_ffff) + 1),
            };
            let f64_val = F64::from_bits(Uint::<8>::from_le_bytes(bits64.to_le_bytes()));
            oracle::assert_correctly_rounded::<8, 52, 5, 4>(f64_val, &ln2);
            let std64 = f64::from_bits(bits64);
            if !std64.is_nan() {
                let ours = u64::from_le_bytes(f64_val.ln_internal().to_bits().to_bytes());
                let theirs = std64.ln().to_bits();
                assert!(ours.abs_diff(theirs) <= 1);
            }
        }
    }

    #[test]
    fn diverse_widths_and_mantissa_bits() {
        let ln2_med = oracle::BigFp::<5, 4>::ln2();
        let ln2_big = oracle::BigFp::<16, 15>::ln2();
        let mut state = 0xdead_beef_cafe_babe_u64;
        let mut next_u64 = || -> u64 {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };

        macro_rules! test_format {
            ($W: literal, $MB: literal, $NL: literal, $FL: literal, $ln2: expr, $iters: literal) => {{
                let bias: Uint<$W> =
                    Uint::power_of_two(Float::<$W, $MB>::EXPONENT_BITS - 1).sub(Uint::ONE);
                let one_bits = bias.shl($MB as Exponent);
                let all_ones_mant = Float::<$W, $MB>::MANTISSA_MASK;
                let max_normal_exp = Float::<$W, $MB>::INFINITY
                    .to_bits()
                    .sub(Uint::power_of_two($MB as Exponent));

                let boundaries: [Float<$W, $MB>; 12] = [
                    Float::ZERO,
                    Float::NEG_ZERO,
                    Float::INFINITY,
                    Float::NEG_INFINITY,
                    Float::NAN,
                    Float::from_bits(one_bits).neg(),
                    Float::from_bits(Uint::ONE),
                    Float::from_bits(all_ones_mant),
                    Float::from_bits(Uint::power_of_two($MB as Exponent)),
                    Float::from_bits(one_bits),
                    Float::from_bits(one_bits.add(Uint::ONE)),
                    Float::from_bits(max_normal_exp.bitor(all_ones_mant)),
                ];
                for b in boundaries {
                    oracle::assert_correctly_rounded::<$W, $MB, $NL, $FL>(b, $ln2);
                }
                oracle::assert_correctly_rounded::<$W, $MB, $NL, $FL>(
                    Float::from_bits(one_bits.sub(Uint::ONE)),
                    $ln2,
                );

                for iter in 0..$iters {
                    let mut bytes = [0u8; $W];
                    let mut i = 0;
                    while i < $W {
                        let r = next_u64().to_le_bytes();
                        let mut j = 0;
                        while j < 8 && i + j < $W {
                            bytes[i + j] = r[j];
                            j += 1;
                        }
                        i += 8;
                    }
                    let mut u = Uint::<$W>::from_le_bytes(bytes);
                    u.set_bit(Float::<$W, $MB>::BITS - 1, false);
                    match iter % 4 {
                        0 => {}
                        1 => {
                            u = u.bitand(all_ones_mant);
                        }
                        2 => {
                            u = one_bits.bitor(u.bitand(all_ones_mant));
                        }
                        _ => {
                            if Float::<$W, $MB>::EXPONENT_BITS >= 3 {
                                u = bias
                                    .sub(Uint::ONE)
                                    .shl($MB as Exponent)
                                    .bitor(u.bitand(all_ones_mant));
                            } else {
                                u = u.bitand(all_ones_mant);
                            }
                        }
                    }
                    oracle::assert_correctly_rounded::<$W, $MB, $NL, $FL>(Float::from_bits(u), $ln2);
                }
            }};
        }

        // Odd byte widths & extreme MBs
        test_format!(3, 3, 5, 4, &ln2_med, 200);
        test_format!(3, 15, 5, 4, &ln2_med, 200);
        test_format!(3, 21, 5, 4, &ln2_med, 200); // EXPONENT_BITS = 2

        // W = 4
        test_format!(4, 3, 5, 4, &ln2_med, 200);
        test_format!(4, 28, 5, 4, &ln2_med, 200); // EXPONENT_BITS = 3
        test_format!(4, 29, 5, 4, &ln2_med, 200); // EXPONENT_BITS = 2

        // EXPONENT_BITS = 32 (exceeds i32 FloatExponent!) and EXPONENT_BITS = 31
        test_format!(5, 7, 5, 4, &ln2_med, 200);
        test_format!(5, 8, 5, 4, &ln2_med, 200);

        // W = 8: EXPONENT_BITS = 60, MB = 60, MB = 61 (EXPONENT_BITS = 2)
        test_format!(8, 3, 5, 4, &ln2_med, 200);
        test_format!(8, 60, 5, 4, &ln2_med, 200);
        test_format!(8, 61, 5, 4, &ln2_med, 200);

        // Extended, quad, octuple, and large custom widths (including limbs > 8 Machin path!)
        test_format!(10, 64, 5, 4, &ln2_med, 150);    // F80
        test_format!(10, 77, 5, 4, &ln2_med, 150);    // EXPONENT_BITS = 2
        test_format!(16, 112, 16, 15, &ln2_big, 120); // F128
        test_format!(16, 125, 16, 15, &ln2_big, 120); // EXPONENT_BITS = 2
        test_format!(32, 236, 16, 15, &ln2_big, 60);  // F256
        test_format!(32, 253, 16, 15, &ln2_big, 60);  // EXPONENT_BITS = 2

        // 512-bit float (MB = 500): `1 + 2^-500` needs > 2*MB = 1000 fractional bits in the oracle
        let ln2_512 = oracle::BigFp::<18, 17>::ln2();
        test_format!(64, 500, 18, 17, &ln2_512, 15);
    }

    #[test]
    fn ziv_stage2_hard_rounding_near_one() {
        // For any format with EXPONENT_BITS >= 3, x = 1 - 2^(-MB) satisfies:
        //   |ln(1 - 2^(-MB))| = 2^(-MB) + 2^(-2*MB - 1) + (1/3) * 2^(-3*MB) + ...
        // The exact half-ULP midpoint between 2^(-MB) and next_up(2^(-MB)) is 2^(-MB) + 2^(-2*MB - 1),
        // so the exact value has (MB + 1) consecutive zeros after the rounding bit!
        // For MB >= 50 this forces Ziv Stage 2 retry and must round up in magnitude to
        // -(2^(-MB) + 2^(-2*MB)) = -((1 + 2^(-MB)) * 2^(-MB)).
        macro_rules! check_one_minus_eps {
            ($W: literal, $MB: literal) => {{
                type F = Float<$W, $MB>;
                let bias: Uint<$W> = Uint::power_of_two(F::EXPONENT_BITS - 1).sub(Uint::ONE);
                let one_bits = bias.shl($MB as Exponent);
                // 1 - 2^(-MB) = one_bits - 2 (since ULP below 1.0 is 2^(-MB - 1))
                let x = F::from_bits(one_bits.sub(Uint::cast_from_unsigned_float_exponent(2)));
                let y = x.ln_internal();
                // Expected: -(2^(-MB) + 2^(-2*MB)), i.e. biased_exp = bias - MB, mantissa = 1, sign = 1
                let exp_bits = bias
                    .sub(Uint::cast_from_unsigned_float_exponent($MB as Exponent))
                    .shl($MB as Exponent);
                let expected = F::from_bits(exp_bits.bitor(Uint::ONE)).neg();
                assert_eq!(
                    y.to_bits(),
                    expected.to_bits(),
                    "failed Ziv Stage 2 test on Float<{}, {}> for 1 - 2^(-MB)",
                    $W,
                    $MB
                );
            }};
        }

        check_one_minus_eps!(2, 10);   // F16
        check_one_minus_eps!(4, 23);   // F32
        check_one_minus_eps!(8, 52);   // F64
        check_one_minus_eps!(10, 64);  // F80
        check_one_minus_eps!(16, 112); // F128
        check_one_minus_eps!(32, 236); // F256
        check_one_minus_eps!(64, 500); // 512-bit float (501 zeros after rounding bit!)
    }

    #[test]
    fn huge_exponent_bits_formats() {
        macro_rules! check_huge_exp {
            ($W: literal, $MB: literal) => {{
                type F = Float<$W, $MB>;
                let bias: Uint<$W> = Uint::power_of_two(F::EXPONENT_BITS - 1).sub(Uint::ONE);
                let one_bits = bias.shl($MB as Exponent);
                assert_eq!(F::from_bits(one_bits).ln_internal().to_bits(), F::ZERO.to_bits());
                assert_eq!(F::ZERO.ln_internal().to_bits(), F::NEG_INFINITY.to_bits());
                assert_eq!(F::INFINITY.ln_internal().to_bits(), F::INFINITY.to_bits());
                assert!(F::from_bits(one_bits).neg().ln_internal().is_nan());

                // x = 2^(2^100): ln(x) = 2^100 * ln(2) ≈ 1.38629... * 2^99
                // For MB = 3, 1.38629... rounds to 1.375 = 11/8 (mantissa bits = 0b011) at exp 99.
                let exp_100 = bias.add(Uint::power_of_two(100));
                let x = F::from_bits(exp_100.shl($MB as Exponent));
                let y = x.ln_internal();
                let expected_exp = bias.add(Uint::cast_from_unsigned_float_exponent(99));
                let expected_mant = Uint::cast_from_unsigned_float_exponent(0b011);
                let expected = expected_exp.shl($MB as Exponent).bitor(expected_mant);
                assert_eq!(y.to_bits(), expected);

                let exp_neg_100 = bias.sub(Uint::power_of_two(100));
                let x_neg = F::from_bits(exp_neg_100.shl($MB as Exponent));
                let y_neg = x_neg.ln_internal();
                assert_eq!(y_neg.to_bits(), F::from_bits(expected).neg().to_bits());

                // MAX of Float<W, 3>: ln(MAX) ≈ (2^(E-1) - 1) * ln(2) ≈ 1.38629... * 2^(E-2)
                let max_bits = F::INFINITY
                    .to_bits()
                    .sub(Uint::power_of_two($MB as Exponent))
                    .bitor(F::MANTISSA_MASK);
                let y_max = F::from_bits(max_bits).ln_internal();
                let exp_max_ln = bias.add(Uint::cast_from_unsigned_float_exponent(
                    F::EXPONENT_BITS - 2,
                ));
                let expected_max = exp_max_ln.shl($MB as Exponent).bitor(expected_mant);
                assert_eq!(y_max.to_bits(), expected_max);
            }};
        }

        check_huge_exp!(16, 3);
        check_huge_exp!(32, 3);
    }

    #[test]
    fn f80_f128_f256_ln_powers_of_two() {
        let ln2 = oracle::BigFp::<16, 15>::ln2();
        oracle::assert_correctly_rounded::<10, 64, 16, 15>(F80::TWO, &ln2);
        oracle::assert_correctly_rounded::<16, 112, 16, 15>(F128::TWO, &ln2);
        oracle::assert_correctly_rounded::<32, 236, 16, 15>(F256::TWO, &ln2);
        oracle::assert_correctly_rounded::<10, 64, 16, 15>(F80::HALF, &ln2);
        oracle::assert_correctly_rounded::<16, 112, 16, 15>(F128::HALF, &ln2);
        oracle::assert_correctly_rounded::<32, 236, 16, 15>(F256::HALF, &ln2);
    }
}
