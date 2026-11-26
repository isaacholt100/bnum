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
const fn fp_cmp<const W: usize>(a: &Fixed<W>, b: &Fixed<W>, limbs: usize) -> core::cmp::Ordering {
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let av = a.get(i);
        let bv = b.get(i);
        if av > bv {
            return core::cmp::Ordering::Greater;
        } else if av < bv {
            return core::cmp::Ordering::Less;
        }
    }
    core::cmp::Ordering::Equal
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
const fn fp_shl<const W: usize>(a: &Fixed<W>, shift: Exponent, limbs: usize) -> Fixed<W> {
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

/// Computes the high `limbs` words of `a * b` together with one guard limb immediately below.
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

/// Computes `delta = exp(r) - 1` at scale `2^(64 * limbs)` for `0 <= r < 0.5`.
///
/// Since `0 <= r < 0.5`, `delta < e^0.5 - 1 ≈ 0.6487 < 1.0`, so `delta` never overflows `2^P`
/// and every Taylor / squaring step is strictly non-negative (zero cancellation).
const fn fp_expm1_pos<const W: usize>(r: &Fixed<W>, limbs: usize) -> Fixed<W> {
    if fp_is_zero(r, limbs) {
        return Fixed::<W>::ALL_ZEROS;
    }

    let p_bits = (limbs as Exponent) * 64;
    let r_bw = fp_bit_width(r, limbs);

    // Halve `r` by `2^s` until `r_small < 2^(-target_lz)` so the Taylor series converges rapidly.
    let target_lz: Exponent = if p_bits <= 128 { 5 } else { 8 };
    let cur_lz = p_bits - r_bw;
    let s: Exponent = if cur_lz < target_lz {
        target_lz - cur_lz
    } else {
        0
    };

    let r_small = fp_shr(r, s, limbs);

    // delta = r_small + r_small^2 / 2! + r_small^3 / 3! + ...
    let mut sum = r_small;
    let mut term = r_small;
    let mut n: u64 = 2;
    loop {
        term = fp_mul_hi(&term, &r_small, limbs);
        term = fp_div_u64(&term, n, limbs);
        if fp_is_zero(&term, limbs) {
            break;
        }
        let (new_sum, carry) = fp_add(&sum, &term, limbs);
        debug_assert!(!carry);
        sum = new_sum;
        n += 1;
    }

    // Undo halving via `(1 + delta)^2 = 1 + (2 * delta + delta^2)`.
    let mut step = 0;
    while step < s {
        let sq = fp_mul_hi(&sum, &sum, limbs);
        let two_sum = fp_shl1_with_bit(&sum, 0, limbs);
        let (next, carry) = fp_add(&two_sum, &sq, limbs);
        debug_assert!(!carry);
        sum = next;
        step += 1;
    }

    sum
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Evaluates `exp(x)` at `limbs` 64-bit limbs of working precision and attempts to round to
    /// `Self`. Returns `None` if Ziv's error bound crosses a half-ULP midpoint (only checked when
    /// `check_ziv` is true, prompting a Stage 2 retry at `MAX_LIMBS`).
    const fn exp_at_precision(
        x_neg: bool,
        e: Int<W>,
        m: Uint<W>,
        k_bits_est: Exponent,
        bias: Uint<W>,
        bias_signed: Int<W>,
        min_normal_exp: Int<W>,
        limbs: usize,
        check_ziv: bool,
    ) -> Option<Self> {
        let p_bits = (limbs as Exponent) * 64;

        // Step 1: Range reduction x = k * ln(2) + r with |r| < 0.5.
        let (k_signed, r_neg, r_fp): (Int<W>, bool, Fixed<W>) = if k_bits_est == 0 {
            // |x| < 0.5 (e <= -2): no range reduction needed (k = 0, r = x).
            // r_fp = |x| * 2^P = m * 2^(P + e - MB).
            let e_abs = e.unsigned_abs().cast_to_unsigned_float_exponent();
            let neg_shift = e_abs + Self::MB;
            let r_val = if p_bits >= neg_shift {
                fp_from_uint_shl(&m, p_bits - neg_shift, limbs)
            } else {
                fp_from_uint_shr(&m, neg_shift - p_bits, limbs)
            };
            (Int::ZERO, x_neg, r_val)
        } else {
            // |x| >= 0.5 (e >= -1): compute k = round(|x| / ln(2)) and r = |x| - k * ln(2).
            let ln2 = fp_ln2::<W>(limbs);
            let inv_ln2 = fp_recip(&ln2, limbs); // (1 / ln(2)) * 2^(P - 1) in (1, 2)

            // Normalise m to [0.5, 1.0) at scale 2^P: m_norm = |x| * 2^(P - e - 1).
            let m_norm = fp_from_uint_shl(&m, p_bits - (Self::MB + 1), limbs);

            // q_val ≈ (|x| / ln(2)) * 2^(P - e - 2).
            let q_val = fp_mul_hi(&m_norm, &inv_ln2, limbs);

            // Round to nearest integer k_abs = floor((|x| / ln(2)) + 0.5).
            // Note: k_bits_est = e + 2 (when e >= 0) or 1 (when e == -1), so p_bits - e - 2 >= 64.
            let k_shift: Exponent = if e.is_negative() {
                // e == -1 => P - (-1) - 2 = P - 1
                p_bits - 1
            } else {
                let e_u = e.cast_unsigned().cast_to_unsigned_float_exponent();
                p_bits - e_u - 2
            };
            let (q_rounded, q_carry) = fp_add_pow2_with_carry(&q_val, 0, k_shift - 1, limbs);
            let k_abs = fp_shr_with_carry_to_uint(&q_rounded, q_carry, k_shift, limbs);

            // Compute both |x| and k_abs * ln(2) at fixed-point scale 2^frac_bits
            // where frac_bits = P - k_bits_est.
            let frac_bits = p_bits - k_bits_est;
            let x_shift: Exponent = if e.is_negative() {
                // e == -1 => frac_bits - 1 - MB
                frac_bits - 1 - Self::MB
            } else {
                let e_u = e.cast_unsigned().cast_to_unsigned_float_exponent();
                frac_bits + e_u - Self::MB
            };
            let x_scaled = fp_from_uint_shl(&m, x_shift, limbs);
            let k_aligned = fp_from_uint_shl(&k_abs, frac_bits, limbs);
            let kln2 = fp_mul_hi(&k_aligned, &ln2, limbs);

            let (r_neg_rel, r_abs) = match fp_cmp(&x_scaled, &kln2, limbs) {
                core::cmp::Ordering::Greater | core::cmp::Ordering::Equal => {
                    let (diff, borrow) = fp_sub(&x_scaled, &kln2, limbs);
                    debug_assert!(!borrow);
                    (false, diff)
                }
                core::cmp::Ordering::Less => {
                    let (diff, borrow) = fp_sub(&kln2, &x_scaled, limbs);
                    debug_assert!(!borrow);
                    (true, diff)
                }
            };

            let r_scaled = fp_shl(&r_abs, k_bits_est, limbs);
            let k_signed = if x_neg {
                k_abs.cast_signed().neg()
            } else {
                k_abs.cast_signed()
            };
            (k_signed, x_neg ^ r_neg_rel, r_scaled)
        };

        // Step 2: Compute normalised exp(r) = (exp_r / 2^(P - 1)) * 2^exp_r_shift,
        // with bit `P - 1` of `exp_r` set (`exp_r in [2^(P - 1), 2^P)`).
        let delta = fp_expm1_pos(&r_fp, limbs);
        let (exp_r, exp_r_shift): (Fixed<W>, Int<W>) = if fp_is_zero(&delta, limbs) {
            let mut one = Fixed::<W>::ALL_ZEROS;
            one.set(limbs - 1, 1u64 << 63);
            (one, Int::ZERO)
        } else {
            // d = ((1 + delta) / 2) * 2^P = 2^(P - 1) + (delta >> 1) in (0.5, 0.825) * 2^P.
            let mut d = fp_shr(&delta, 1, limbs);
            d.set(limbs - 1, d.get(limbs - 1) | (1u64 << 63));
            if !r_neg {
                // exp(r) = 1 + delta = (d / 2^(P - 1)) * 2^0.
                (d, Int::ZERO)
            } else {
                // exp(-|r|) = 1 / (1 + delta) = (1 / (2 * d)) = (r_inv / 2^(P - 1)) * 2^(-1),
                // where r_inv = fp_recip(&d) has bit P - 1 set since e^(-0.5) ≈ 0.6065 > 0.5.
                let r_inv = fp_recip(&d, limbs);
                (r_inv, Int::NEG_ONE)
            }
        };

        let out_exp = k_signed.add(exp_r_shift);

        // Conservative error bound in ULPs of bit 0 of `exp_r`.
        let limb_bits: Exponent = 64 - (limbs as u64).leading_zeros();
        let err_bits: Exponent = k_bits_est + limb_bits + 20;

        // Step 3: Overflow, underflow, subnormal, and normal rounding with Ziv check.
        if out_exp.gt(&bias_signed) {
            return Some(Self::INFINITY);
        }

        let mb_plus_1_int = Uint::cast_from_unsigned_float_exponent(Self::MB + 1).cast_signed();
        let min_subnormal_exp = min_normal_exp.sub(mb_plus_1_int).add(Int::ONE);
        let underflow_thresh = min_subnormal_exp.sub(Int::ONE).sub(Int::ONE); // min_subnormal_exp - 2

        if out_exp.lt(&underflow_thresh) {
            return Some(Self::ZERO);
        }

        if out_exp.eq(&underflow_thresh) {
            // Value lies in [0.25, 0.5) * MIN_POSITIVE_SUBNORMAL.
            if err_bits < p_bits {
                let (_, carry) = fp_add_pow2_with_carry(&exp_r, 0, err_bits, limbs);
                if carry == 0 {
                    return Some(Self::ZERO);
                }
            }
            return if check_ziv { None } else { Some(Self::ZERO) };
        }

        let is_subnormal = out_exp.lt(&min_normal_exp);
        let drop: Exponent = if is_subnormal {
            let diff = min_normal_exp
                .sub(out_exp)
                .cast_unsigned()
                .cast_to_unsigned_float_exponent();
            p_bits + diff - (Self::MB + 1)
        } else {
            p_bits - (Self::MB + 1)
        };

        // Round to nearest (exact half-ULP ties are impossible for transcendental e^x, x != 0).
        let (mid, carry_mid) = fp_add_pow2_with_carry(&exp_r, 0, drop - 1, limbs);
        let mut rounded_mant = if check_ziv {
            if err_bits + 2 >= drop {
                return None;
            }
            let (hi, carry_hi) = fp_add_pow2_with_carry(&mid, carry_mid, err_bits, limbs);
            let (lo, carry_lo) = fp_sub_pow2_with_carry(&mid, carry_mid, err_bits, limbs);
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
            if rounded_mant.is_zero() {
                return Some(Self::ZERO);
            }
            if rounded_mant.bit(Self::MB) {
                // Rounded up across the subnormal-to-normal boundary to MIN_POSITIVE.
                (Uint::ONE, Uint::ZERO)
            } else {
                (Uint::ZERO, rounded_mant)
            }
        } else {
            let mut final_exp = out_exp;
            if rounded_mant.bit(Self::MB + 1) {
                final_exp = final_exp.add(Int::ONE);
                if final_exp.gt(&bias_signed) {
                    return Some(Self::INFINITY);
                }
                (final_exp.add(bias_signed).cast_unsigned(), Uint::ZERO)
            } else {
                rounded_mant.set_bit(Self::MB, false);
                (final_exp.add(bias_signed).cast_unsigned(), rounded_mant)
            }
        };

        // exp(x) is always positive, so sign bit is 0.
        let _ = bias;
        let out_bits = biased_out_exp.shl(Self::MB).bitor(mant_bits);
        Some(Self::from_bits(out_bits))
    }

    /// Computes the exponential function `e^self` for `Float<W, MB>`.
    ///
    /// Valid for all `W > 1` and `2 < MB < Self::BITS - 2`, including formats with small exponent
    /// widths (e.g., `EXPONENT_BITS = 2`) and arbitrarily large exponent widths
    /// (`EXPONENT_BITS >= 32`).
    pub(super) const fn exp_internal(self) -> Self {
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

        // Compute bias in `Uint<W>` directly so this works for any `EXPONENT_BITS` (even >= 32).
        let bias: Uint<W> = Uint::power_of_two(Self::EXPONENT_BITS - 1).sub(Uint::ONE);
        let one = Self::from_bits(bias.shl(Self::MB));

        if self.is_zero() {
            return one;
        }

        let x_neg = self.is_sign_negative();
        if !self.is_finite() {
            return if x_neg { Self::ZERO } else { Self::INFINITY };
        }

        let raw_exp = self.bits.bitand(Self::SIGN_MASK).shr(Self::MB);
        let raw_mant = self.bits.bitand(Self::MANTISSA_MASK);
        let bias_signed: Int<W> = bias.cast_signed();
        let min_normal_exp: Int<W> = Int::ONE.sub(bias_signed);

        // Normalise |self| = (m / 2^MB) * 2^e with 2^MB <= m < 2^(MB + 1).
        let (e, m): (Int<W>, Uint<W>) = if raw_exp.is_zero() {
            let norm_shift = Self::MB + 1 - raw_mant.bit_width();
            let norm_mant = raw_mant.shl(norm_shift);
            let shift_int = Uint::cast_from_unsigned_float_exponent(norm_shift).cast_signed();
            (min_normal_exp.sub(shift_int), norm_mant)
        } else {
            let norm_mant = raw_mant.bitor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
            (raw_exp.cast_signed().sub(bias_signed), norm_mant)
        };

        // Tiny-argument fast path: for |x| < 2^(-MB - 2) (i.e. e <= -MB - 3), e^x is strictly
        // closer to 1.0 than to either adjacent float (`1 - 2^(-MB-1)` or `1 + 2^(-MB)`).
        let tiny_exp = Uint::cast_from_unsigned_float_exponent(Self::MB + 3)
            .cast_signed()
            .neg();
        if e.le(&tiny_exp) {
            return one;
        }

        // Large-argument overflow/underflow filter:
        // Overflow occurs when x > (bias + 1) * ln(2) < bias + 1;
        // underflow to 0 occurs when -x > (bias + MB) * ln(2) < bias + MB + 1.
        let max_k_bound = bias.add(Uint::cast_from_unsigned_float_exponent(Self::MB + 2));
        let max_e_int = Uint::cast_from_unsigned_float_exponent(max_k_bound.bit_width()).cast_signed();
        if e.ge(&max_e_int) {
            return if x_neg { Self::ZERO } else { Self::INFINITY };
        }

        // Estimate integer bit width needed for k = round(|x| / ln(2)):
        // - If e <= -2 (|x| < 0.5): k = 0 (no range reduction).
        // - If e == -1 (0.5 <= |x| < 1): k <= 1 (k_bits_est = 1).
        // - If e >= 0: |k| < 2^(e + 2) (k_bits_est = e + 2).
        let neg_two = Uint::cast_from_unsigned_float_exponent(2).cast_signed().neg();
        let k_bits_est: Exponent = if e.le(&neg_two) {
            0
        } else if e.is_negative() {
            1
        } else {
            e.cast_unsigned().cast_to_unsigned_float_exponent() + 2
        };

        let max_limbs = (2 * W + 24) / 8;
        let raw_l1 = (MB + (k_bits_est as usize) + 64).div_ceil(64);
        let l1 = if raw_l1 < max_limbs {
            raw_l1
        } else {
            max_limbs
        };

        if l1 < max_limbs {
            if let Some(res) = Self::exp_at_precision(
                x_neg,
                e,
                m,
                k_bits_est,
                bias,
                bias_signed,
                min_normal_exp,
                l1,
                true,
            ) {
                return res;
            }
        }

        // Stage 2 (Ziv fallback): re-evaluate at full precision (`>= 2 * MB + 136` bits).
        match Self::exp_at_precision(
            x_neg,
            e,
            m,
            k_bits_est,
            bias,
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

    /// Independent high-precision `ln` oracle verifying that `y = x.exp_internal()` is the
    /// unique IEEE 754 round-to-nearest-ties-to-even value of `e^x`:
    ///   `ln_oracle(m_lo) < x < ln_oracle(m_hi)`.
    ///
    /// Uses `ln(2) = sum_{n >= 1} 1 / (n * 2^n)` and `ln(m) = 2 * atanh((m - 1) / (m + 1))`
    /// computed via binary long division — completely independent of `exp_internal`'s
    /// Taylor series, squaring, Machin `ln(2)`, and `fp_recip`.
    mod oracle {
        use super::*;

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

            /// Exact binary long division `num / den` for `0 <= num < den`, returning `BigFp`
            /// at scale `2^FRAC`.
            fn div_ratio(num: &Self, den: &Self) -> Self {
                let mut rem = *num;
                let mut q = [0u64; NL];
                for bit in (0..Self::FRAC).rev() {
                    rem = rem.shl(1);
                    if rem.cmp_fp(den).is_ge() {
                        rem = rem.sub(den);
                        q[bit / 64] |= 1u64 << (bit % 64);
                    }
                }
                Self(q)
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

            /// Computes `ln(m)` for `m in [1.0, 2.0)` via `2 * atanh((m - 1) / (m + 1))`.
            fn ln_unit(m: &Self) -> Self {
                let one = Self::one();
                if m.cmp_fp(&one).is_eq() {
                    return Self::ZERO;
                }
                let num = m.sub(&one);
                let den = m.add(&one);
                let z = Self::div_ratio(&num, &den);
                let z2 = z.mul_fp(&z);

                let mut sum = z;
                let mut term = z;
                let mut n = 1u64;
                loop {
                    term = term.mul_fp(&z2);
                    if term.is_zero() {
                        break;
                    }
                    let dt = term.div_u64(2 * n + 1);
                    if dt.is_zero() {
                        break;
                    }
                    sum = sum.add(&dt);
                    n += 1;
                }
                sum.shl(1)
            }

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

        #[derive(Clone, Copy)]
        struct SignedFp<const NL: usize, const FRAC_LIMBS: usize> {
            neg: bool,
            mag: BigFp<NL, FRAC_LIMBS>,
        }

        impl<const NL: usize, const FRAC_LIMBS: usize> SignedFp<NL, FRAC_LIMBS> {
            fn add_signed(self, other: Self) -> Self {
                if self.neg == other.neg {
                    Self {
                        neg: self.neg,
                        mag: self.mag.add(&other.mag),
                    }
                } else {
                    match self.mag.cmp_fp(&other.mag) {
                        core::cmp::Ordering::Greater => Self {
                            neg: self.neg,
                            mag: self.mag.sub(&other.mag),
                        },
                        core::cmp::Ordering::Less => Self {
                            neg: other.neg,
                            mag: other.mag.sub(&self.mag),
                        },
                        core::cmp::Ordering::Equal => Self {
                            neg: false,
                            mag: BigFp::ZERO,
                        },
                    }
                }
            }

            fn cmp_signed(&self, other: &Self) -> core::cmp::Ordering {
                match (self.neg, other.neg) {
                    (true, false) => {
                        if self.mag.is_zero() && other.mag.is_zero() {
                            core::cmp::Ordering::Equal
                        } else {
                            core::cmp::Ordering::Less
                        }
                    }
                    (false, true) => {
                        if self.mag.is_zero() && other.mag.is_zero() {
                            core::cmp::Ordering::Equal
                        } else {
                            core::cmp::Ordering::Greater
                        }
                    }
                    (false, false) => self.mag.cmp_fp(&other.mag),
                    (true, true) => other.mag.cmp_fp(&self.mag),
                }
            }
        }

        fn int_to_i64<const W: usize>(x: &Int<W>) -> i64 {
            let neg = x.is_negative();
            let abs = x.unsigned_abs();
            let mut val = 0u64;
            for i in 0..W.min(8) {
                val |= (abs.bytes[i] as u64) << (i * 8);
            }
            if neg { -(val as i64) } else { val as i64 }
        }

        /// Computes `ln(u * 2^exp_lsb)` where `u > 0` as a `SignedFp`.
        fn ln_of_dyadic<const W: usize, const NL: usize, const FRAC_LIMBS: usize>(
            u: &Uint<W>,
            exp_lsb: i64,
            ln2: &BigFp<NL, FRAC_LIMBS>,
        ) -> SignedFp<NL, FRAC_LIMBS> {
            let bw = u.bit_width() as i64;
            let e = exp_lsb + (bw - 1);
            let m_unit = BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(u, -(bw - 1));
            let ln_m = SignedFp {
                neg: false,
                mag: BigFp::ln_unit(&m_unit),
            };
            let e_abs = e.unsigned_abs();
            let e_fp = BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(
                &Uint::<8>::from_le_bytes(e_abs.to_le_bytes()),
                0,
            );
            let e_ln2 = SignedFp {
                neg: e < 0,
                mag: e_fp.mul_fp(ln2),
            };
            e_ln2.add_signed(ln_m)
        }

        /// Verifies `y = x.exp_internal()` is the exact IEEE 754 round-to-nearest-ties-to-even
        /// value of `e^x` for any `Float<W, MB>` whose exponent fits in `i64`.
        pub(super) fn assert_correctly_rounded<
            const W: usize,
            const MB: usize,
            const NL: usize,
            const FRAC_LIMBS: usize,
        >(
            x: Float<W, MB>,
            ln2: &BigFp<NL, FRAC_LIMBS>,
        ) {
            let y = x.exp_internal();
            let bias: Uint<W> =
                Uint::power_of_two(Float::<W, MB>::EXPONENT_BITS - 1).sub(Uint::ONE);
            let one = Float::<W, MB>::from_bits(bias.shl(MB as Exponent));

            if x.is_nan() {
                assert!(y.is_nan());
                assert_eq!(y.to_bits(), x.to_bits());
                return;
            }
            if x.is_zero() {
                assert_eq!(y.to_bits(), one.to_bits());
                return;
            }
            if !x.is_finite() {
                if x.is_sign_negative() {
                    assert_eq!(y.to_bits(), Float::<W, MB>::ZERO.to_bits());
                } else {
                    assert_eq!(y.to_bits(), Float::<W, MB>::INFINITY.to_bits());
                }
                return;
            }

            assert!(!y.is_nan() && !y.is_sign_negative());

            let bias_s = bias.cast_signed();
            let bias_i64 = int_to_i64(&bias_s);
            let min_normal_exp_i64 = 1 - bias_i64;

            // Convert x to SignedFp (skipping giant |x| that trivially overflow/underflow BigFp integer bits).
            let x_neg = x.is_sign_negative();
            let re = x.bits.bitand(Float::<W, MB>::SIGN_MASK).shr(MB as Exponent);
            let rm = x.bits.bitand(Float::<W, MB>::MANTISSA_MASK);
            let (mx, ex_lsb, ex_unbiased) = if re.is_zero() {
                let s = MB as Exponent + 1 - rm.bit_width();
                let eu = min_normal_exp_i64 - (s as i64);
                (rm, min_normal_exp_i64 - (MB as i64), eu)
            } else {
                let eu = int_to_i64(&re.cast_signed().sub(bias_s));
                (
                    rm.bitor(Float::<W, MB>::MANTISSA_IMPLICIT_LEADING_ONE_MASK),
                    eu - (MB as i64),
                    eu,
                )
            };

            // If |x| >= 2^60, it trivially overflows (x > 0) or underflows (x < 0) for any format with E <= 60.
            if ex_unbiased >= 60 {
                if x_neg {
                    assert_eq!(y.to_bits(), Float::<W, MB>::ZERO.to_bits());
                } else {
                    assert_eq!(y.to_bits(), Float::<W, MB>::INFINITY.to_bits());
                }
                return;
            }
            // If |x| < 2^(-FRAC + 16), it is tiny and must round to 1.0.
            if ex_unbiased <= -((MB as i64) + 3) {
                assert_eq!(y.to_bits(), one.to_bits());
                return;
            }

            let x_fp = SignedFp {
                neg: x_neg,
                mag: BigFp::<NL, FRAC_LIMBS>::from_uint_scaled(&mx, ex_lsb),
            };

            // Handle y == +INFINITY
            if y.is_infinite() {
                let max_mid_u = Uint::<W>::power_of_two(MB as Exponent + 2).sub(Uint::ONE);
                let ln_max_mid = ln_of_dyadic(&max_mid_u, bias_i64 - (MB as i64) - 1, ln2);
                assert!(
                    x_fp.cmp_signed(&ln_max_mid).is_gt(),
                    "exp(x) overflowed prematurely for x={:?}",
                    x.to_bits()
                );
                return;
            }

            // Handle y == +0.0
            if y.is_zero() {
                let min_mid_u = Uint::<W>::ONE;
                let ln_min_mid = ln_of_dyadic(&min_mid_u, min_normal_exp_i64 - (MB as i64) - 1, ln2);
                assert!(
                    x_fp.cmp_signed(&ln_min_mid).is_lt(),
                    "exp(x) underflowed prematurely for x={:?}",
                    x.to_bits()
                );
                return;
            }

            // Finite nonzero positive y: extract lower and upper midpoints (m_lo, m_hi)
            let yre = y.bits.bitand(Float::<W, MB>::SIGN_MASK).shr(MB as Exponent);
            let yrm = y.bits.bitand(Float::<W, MB>::MANTISSA_MASK);
            let (my, ey_lsb, is_gt_min_normal_pow2) = if yre.is_zero() {
                (yrm, min_normal_exp_i64 - (MB as i64), false)
            } else {
                let eu = int_to_i64(&yre.cast_signed().sub(bias_s));
                (
                    yrm.bitor(Float::<W, MB>::MANTISSA_IMPLICIT_LEADING_ONE_MASK),
                    eu - (MB as i64),
                    yrm.is_zero() && yre.gt(&Uint::ONE),
                )
            };

            let hi_u = my.shl(1).add(Uint::ONE);
            let ln_hi = ln_of_dyadic(&hi_u, ey_lsb - 1, ln2);
            assert!(
                x_fp.cmp_signed(&ln_hi).is_lt(),
                "y={:?} is too small for x={:?}",
                y.to_bits(),
                x.to_bits()
            );

            let (lo_u, lo_exp) = if is_gt_min_normal_pow2 {
                (my.shl(2).sub(Uint::ONE), ey_lsb - 2)
            } else {
                (my.shl(1).sub(Uint::ONE), ey_lsb - 1)
            };
            let ln_lo = ln_of_dyadic(&lo_u, lo_exp, ln2);
            assert!(
                x_fp.cmp_signed(&ln_lo).is_gt(),
                "y={:?} is too large for x={:?}",
                y.to_bits(),
                x.to_bits()
            );
        }
    }

    #[test]
    fn exhaustive_w2_formats() {
        let ln2 = oracle::BigFp::<3, 2>::ln2();

        // Exhaustively test ALL 65,536 bit patterns across 4 16-bit formats:
        // - Float<2, 3>:  minimum MB (3), EXPONENT_BITS = 12
        // - Float<2, 10>: IEEE binary16 (F16), EXPONENT_BITS = 5
        // - Float<2, 12>: EXPONENT_BITS = 3
        // - Float<2, 13>: maximum MB (13), EXPONENT_BITS = 2 (MIN_POSITIVE = 1.0!)
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
        let ln2 = oracle::BigFp::<5, 4>::ln2();

        // Special cases on F64
        assert_eq!(F64::ZERO.exp_internal().to_bits(), F64::ONE.to_bits());
        assert_eq!(F64::NEG_ZERO.exp_internal().to_bits(), F64::ONE.to_bits());
        assert_eq!(
            F64::INFINITY.exp_internal().to_bits(),
            F64::INFINITY.to_bits()
        );
        assert_eq!(
            F64::NEG_INFINITY.exp_internal().to_bits(),
            F64::ZERO.to_bits()
        );
        assert!(F64::NAN.exp_internal().is_nan());
        assert!(F64::NEG_NAN.exp_internal().is_nan());

        // Critical boundary & hard-to-round F64 values
        let cases: &[f64] = &[
            1.0,
            -1.0,
            0.5,
            -0.5,
            core::f64::consts::LN_2,
            -core::f64::consts::LN_2,
            10.0 * core::f64::consts::LN_2,
            -10.0 * core::f64::consts::LN_2,
            // Near 0 (Ziv Stage 2 stress cases: +2^-53 and -2^-54 sit right above midpoints!)
            2.0_f64.powi(-52),
            2.0_f64.powi(-53),
            f64::from_bits(2.0_f64.powi(-53).to_bits() - 1),
            -2.0_f64.powi(-53),
            -2.0_f64.powi(-54),
            f64::from_bits((-2.0_f64.powi(-54)).to_bits() + 1),
            -2.0_f64.powi(-55),
            f64::from_bits(1),
            -f64::from_bits(1),
            // Near overflow (709.782712893384)
            709.7827128933839,
            709.782712893384,
            709.7827128933841,
            710.0,
            // Near subnormal boundary (-708.3964185322641)
            -708.396418532264,
            -708.3964185322641,
            -708.3964185322642,
            // Near underflow (-745.1332191019412)
            -745.1332191019411,
            -745.1332191019412,
            -745.1332191019413,
            -746.0,
            f64::MAX,
            f64::MIN,
            // Known Lefèvre-Muller hard-to-round f64 exp inputs (150+ bit runs!)
            f64::from_bits(0x3fd7_8b56_362c_ef38),
            f64::from_bits(0xbfd7_8b56_362c_ef38),
            f64::from_bits(0x4086_2e42_fefa_39ef),
            f64::from_bits(0xc087_4910_d52d_3051),
            f64::from_bits(0x3ff0_0000_0000_0001),
            f64::from_bits(0xbfe6_2e42_fefa_39ef),
        ];

        for &v in cases {
            let f = F64::from(v);
            oracle::assert_correctly_rounded::<8, 52, 5, 4>(f, &ln2);
            let ours = u64::from_le_bytes(f.exp_internal().to_bits().to_bytes());
            let theirs = v.exp().to_bits();
            assert!(
                ours.abs_diff(theirs) <= 1,
                "f64::exp diff > 1 ULP on {v:?}: ours={ours:#x}, std={theirs:#x}"
            );
        }

        // Const evaluation check: exp(1.0) == E
        const CONST_E: F64 = {
            let bits = Uint::<8>::from_le_bytes(1.0_f64.to_bits().to_le_bytes());
            F64::from_bits(bits).exp_internal()
        };
        assert_eq!(
            CONST_E.to_bits(),
            Uint::<8>::from_le_bytes(core::f64::consts::E.to_bits().to_le_bytes())
        );
    }

    #[test]
    fn f32_and_f64_random_oracle() {
        let ln2 = oracle::BigFp::<5, 4>::ln2();
        let mut state = 0x55aa_1234_9876_fedc_u64;
        let mut next_u64 = || -> u64 {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };

        for i in 0..2_000 {
            let r = next_u64();

            // Generate F32 values in the interesting finite range [-110, 95] plus arbitrary/tiny
            let f32_v = match i % 5 {
                0 => f32::from_bits(r as u32),
                1 => ((r as i32) as f32 / (i32::MAX as f32)) * 105.0,
                2 => -88.0 - ((r & 0xffff) as f32 / 65536.0) * 16.0, // subnormal outputs!
                3 => 88.0 + ((r & 0xffff) as f32 / 65536.0) * 1.0,   // near overflow!
                _ => ((r as i32) as f32 / (i32::MAX as f32)) * 2.0_f32.powi(-20), // near 0
            };
            let f32_b = F32::from(f32_v);
            oracle::assert_correctly_rounded::<4, 23, 5, 4>(f32_b, &ln2);
            if !f32_v.is_nan() {
                let ours = u32::from_le_bytes(f32_b.exp_internal().to_bits().to_bytes());
                let theirs = f32_v.exp().to_bits();
                assert!(ours.abs_diff(theirs) <= 1);
            }

            // Generate F64 values in [-760, 720], subnormal output range [-746, -708], near 0, etc.
            let f64_v = match i % 5 {
                0 => f64::from_bits(r),
                1 => ((r as i64) as f64 / (i64::MAX as f64)) * 750.0,
                2 => -708.0 - ((r & 0xffff_ffff) as f64 / 4294967296.0) * 38.0, // subnormal outputs!
                3 => 709.0 + ((r & 0xffff_ffff) as f64 / 4294967296.0) * 1.0,   // near overflow!
                _ => ((r as i64) as f64 / (i64::MAX as f64)) * 2.0_f64.powi(-48), // near 0
            };
            let f64_b = F64::from(f64_v);
            oracle::assert_correctly_rounded::<8, 52, 5, 4>(f64_b, &ln2);
            if !f64_v.is_nan() {
                let ours = u64::from_le_bytes(f64_b.exp_internal().to_bits().to_bytes());
                let theirs = f64_v.exp().to_bits();
                assert!(ours.abs_diff(theirs) <= 1);
            }
        }
    }

    #[test]
    fn ziv_stage2_hard_rounding_near_zero() {
        // For any format with EXPONENT_BITS >= 3:
        //   - x = +2^(-MB - 1): exp(x) = 1 + 2^(-MB - 1) + 2^(-2*MB - 3) + ... > 1 + 2^(-MB - 1)
        //     (strictly above midpoint => rounds UP to 1 + 2^(-MB) = next_up(1.0)!)
        //   - x = -2^(-MB - 2): exp(x) = 1 - 2^(-MB - 2) + 2^(-2*MB - 5) - ... > 1 - 2^(-MB - 2)
        //     (strictly above midpoint => rounds UP to 1.0!)
        //   - x = -(1 + 2^(-MB)) * 2^(-MB - 2): exp(x) < 1 - 2^(-MB - 2)
        //     (strictly below midpoint => rounds DOWN to 1 - 2^(-MB - 1) = next_down(1.0)!)
        macro_rules! check_near_zero {
            ($W: literal, $MB: literal) => {{
                type F = Float<$W, $MB>;
                let bias: Uint<$W> = Uint::power_of_two(F::EXPONENT_BITS - 1).sub(Uint::ONE);
                let one_bits = bias.shl($MB as Exponent);

                let pos_half = F::from_bits(
                    bias.sub(Uint::cast_from_unsigned_float_exponent($MB as Exponent + 1))
                        .shl($MB as Exponent),
                );
                assert_eq!(
                    pos_half.exp_internal().to_bits(),
                    one_bits.add(Uint::ONE),
                    "exp(+2^(-MB-1)) failed on Float<{}, {}>",
                    $W,
                    $MB
                );

                let neg_quarter = F::from_bits(
                    bias.sub(Uint::cast_from_unsigned_float_exponent($MB as Exponent + 2))
                        .shl($MB as Exponent),
                )
                .neg();
                assert_eq!(
                    neg_quarter.exp_internal().to_bits(),
                    one_bits,
                    "exp(-2^(-MB-2)) failed on Float<{}, {}>",
                    $W,
                    $MB
                );

                let neg_below = F::from_bits(
                    bias.sub(Uint::cast_from_unsigned_float_exponent($MB as Exponent + 2))
                        .shl($MB as Exponent)
                        .bitor(Uint::ONE),
                )
                .neg();
                assert_eq!(
                    neg_below.exp_internal().to_bits(),
                    one_bits.sub(Uint::ONE),
                    "exp(-(1+2^-MB)*2^(-MB-2)) failed on Float<{}, {}>",
                    $W,
                    $MB
                );
            }};
        }

        check_near_zero!(2, 10);   // F16
        check_near_zero!(4, 23);   // F32
        check_near_zero!(8, 52);   // F64
        check_near_zero!(10, 64);  // F80
        check_near_zero!(16, 112); // F128
        check_near_zero!(32, 236); // F256
        check_near_zero!(64, 500); // 512-bit float (503 zeros after rounding bit!)
    }

    #[test]
    fn diverse_widths_and_mantissa_bits() {
        let ln2_med = oracle::BigFp::<5, 4>::ln2();
        let ln2_big = oracle::BigFp::<16, 15>::ln2();
        let mut state = 0x1122_3344_5566_7788_u64;
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
                    Float::from_bits(one_bits),
                    Float::from_bits(one_bits).neg(),
                    Float::from_bits(Uint::ONE),
                    Float::from_bits(Uint::ONE).neg(),
                    Float::from_bits(all_ones_mant),
                    Float::from_bits(max_normal_exp.bitor(all_ones_mant)),
                    Float::from_bits(max_normal_exp.bitor(all_ones_mant)).neg(),
                ];
                for b in boundaries {
                    oracle::assert_correctly_rounded::<$W, $MB, $NL, $FL>(b, $ln2);
                }

                // Target exponents where exp(x) is non-trivial (neither trivially 1 nor trivially 0/inf)
                let max_interesting_e: i64 = (Float::<$W, $MB>::EXPONENT_BITS as i64).min(20);
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
                    let rand_mant = Uint::<$W>::from_le_bytes(bytes).bitand(all_ones_mant);
                    let sign = (next_u64() & 1) != 0;
                    let u = match iter % 3 {
                        0 => {
                            // Unbiased exponent in [-4, max_interesting_e]
                            let span = (max_interesting_e + 5) as u64;
                            let eu = ((next_u64() % span) as i64) - 4;
                            let min_e = 1 - (if Float::<$W, $MB>::EXPONENT_BITS < 62 {
                                (1i64 << (Float::<$W, $MB>::EXPONENT_BITS - 1)) - 1
                            } else {
                                i64::MAX / 2
                            });
                            let max_e = if Float::<$W, $MB>::EXPONENT_BITS < 62 {
                                (1i64 << (Float::<$W, $MB>::EXPONENT_BITS - 1)) - 1
                            } else {
                                i64::MAX / 2
                            };
                            let clamped = eu.clamp(min_e, max_e);
                            let biased = if clamped >= 0 {
                                bias.add(Uint::cast_from_unsigned_float_exponent(clamped as u32))
                            } else {
                                bias.sub(Uint::cast_from_unsigned_float_exponent((-clamped) as u32))
                            };
                            biased.shl($MB as Exponent).bitor(rand_mant)
                        }
                        1 => {
                            // Subnormal x (near 0)
                            rand_mant
                        }
                        _ => {
                            // Completely arbitrary bit pattern
                            Uint::<$W>::from_le_bytes(bytes)
                        }
                    };
                    let mut f = Float::<$W, $MB>::from_bits(u);
                    if sign && f.is_finite() {
                        f = f.neg();
                    }
                    oracle::assert_correctly_rounded::<$W, $MB, $NL, $FL>(f, $ln2);
                }
            }};
        }

        // Odd byte widths & extreme MBs
        test_format!(3, 3, 5, 4, &ln2_med, 150);
        test_format!(3, 15, 5, 4, &ln2_med, 150);
        test_format!(3, 21, 5, 4, &ln2_med, 150); // EXPONENT_BITS = 2

        // W = 4
        test_format!(4, 3, 5, 4, &ln2_med, 150);
        test_format!(4, 28, 5, 4, &ln2_med, 150); // EXPONENT_BITS = 3
        test_format!(4, 29, 5, 4, &ln2_med, 150); // EXPONENT_BITS = 2

        // EXPONENT_BITS = 32 (exceeds i32 FloatExponent!) and EXPONENT_BITS = 31
        test_format!(5, 7, 5, 4, &ln2_med, 150);
        test_format!(5, 8, 5, 4, &ln2_med, 150);

        // W = 8: EXPONENT_BITS = 60, MB = 60, MB = 61 (EXPONENT_BITS = 2)
        test_format!(8, 3, 5, 4, &ln2_med, 150);
        test_format!(8, 60, 5, 4, &ln2_med, 150);
        test_format!(8, 61, 5, 4, &ln2_med, 150);

        // Extended, quad, octuple, and 512-bit formats
        test_format!(10, 64, 5, 4, &ln2_med, 100);    // F80
        test_format!(10, 77, 5, 4, &ln2_med, 100);    // EXPONENT_BITS = 2
        test_format!(16, 112, 16, 15, &ln2_big, 80);  // F128
        test_format!(16, 125, 16, 15, &ln2_big, 80);  // EXPONENT_BITS = 2
        test_format!(32, 236, 16, 15, &ln2_big, 40);  // F256
        test_format!(32, 253, 16, 15, &ln2_big, 40);  // EXPONENT_BITS = 2

        // 512-bit float (exercises Machin ln(2) with limbs > 8!)
        let ln2_512 = oracle::BigFp::<18, 17>::ln2();
        test_format!(64, 500, 18, 17, &ln2_512, 10);
    }

    #[test]
    fn huge_exponent_bits_formats() {
        // Test EXPONENT_BITS > 64: Float<16, 3> (E = 124) and Float<32, 3> (E = 252)
        macro_rules! check_huge_exp {
            ($W: literal, $MB: literal) => {{
                type F = Float<$W, $MB>;
                let bias: Uint<$W> = Uint::power_of_two(F::EXPONENT_BITS - 1).sub(Uint::ONE);
                let one_bits = bias.shl($MB as Exponent);

                assert_eq!(F::ZERO.exp_internal().to_bits(), one_bits);
                assert_eq!(F::NEG_ZERO.exp_internal().to_bits(), one_bits);
                assert_eq!(F::INFINITY.exp_internal().to_bits(), F::INFINITY.to_bits());
                assert_eq!(F::NEG_INFINITY.exp_internal().to_bits(), F::ZERO.to_bits());

                // exp(1.0) = e ≈ 2.71828... = 1.35914... * 2^1 -> for MB = 3, rounds to 1.375 * 2^1 (mant 0b011)
                let e_expected = bias
                    .add(Uint::ONE)
                    .shl($MB as Exponent)
                    .bitor(Uint::cast_from_unsigned_float_exponent(0b011));
                assert_eq!(F::from_bits(one_bits).exp_internal().to_bits(), e_expected);

                // x = 2^100 is well below bias * ln(2) (~2^122.5), so exp(2^100) is finite and normal!
                let x_huge = F::from_bits(
                    bias.add(Uint::cast_from_unsigned_float_exponent(100))
                        .shl($MB as Exponent),
                );
                let y_huge = x_huge.exp_internal();
                assert!(y_huge.is_normal() && !y_huge.is_sign_negative());
                // Verify round-trip: ln(exp(2^100)) == 2^100!
                assert_eq!(y_huge.ln_internal().to_bits(), x_huge.to_bits());

                // -x_huge: exp(-2^100) is finite, normal, and ln(exp(-2^100)) == -2^100!
                let y_neg = x_huge.neg().exp_internal();
                assert!(y_neg.is_normal() && !y_neg.is_sign_negative());
                assert_eq!(y_neg.ln_internal().to_bits(), x_huge.neg().to_bits());

                // x = 2^(EXPONENT_BITS - 1) > bias * ln(2) => overflows to +inf / underflows to 0
                let x_ovf = F::from_bits(
                    bias.add(Uint::cast_from_unsigned_float_exponent(
                        F::EXPONENT_BITS - 1,
                    ))
                    .shl($MB as Exponent),
                );
                assert_eq!(x_ovf.exp_internal().to_bits(), F::INFINITY.to_bits());
                assert_eq!(x_ovf.neg().exp_internal().to_bits(), F::ZERO.to_bits());
            }};
        }

        check_huge_exp!(16, 3);
        check_huge_exp!(32, 3);
    }

    #[test]
    fn exp_ln_roundtrip_on_f80_f128_f256() {
        // Verify exp(ln(2)) == 2 and exp(ln(0.5)) == 0.5 on F80, F128, F256
        assert_eq!(F80::TWO.ln_internal().exp_internal().to_bits(), F80::TWO.to_bits());
        assert_eq!(F128::TWO.ln_internal().exp_internal().to_bits(), F128::TWO.to_bits());
        assert_eq!(F256::TWO.ln_internal().exp_internal().to_bits(), F256::TWO.to_bits());
        assert_eq!(F80::HALF.ln_internal().exp_internal().to_bits(), F80::HALF.to_bits());
        assert_eq!(F128::HALF.ln_internal().exp_internal().to_bits(), F128::HALF.to_bits());
        assert_eq!(F256::HALF.ln_internal().exp_internal().to_bits(), F256::HALF.to_bits());
    }
}
