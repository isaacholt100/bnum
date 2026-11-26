use core::cmp::Ordering;
use core::str::FromStr;

use crate::digits::Digits;
use crate::errors::ParseFloatError;
use crate::float::RoundingMode;
use crate::{Exponent, Float, Uint};

/// Fixed-point buffer with `2 * W + 24` bytes.
///
/// The number of full 64-bit limbs `MAX_LIMBS = (2 * W + 24) / 8` satisfies
/// `MAX_LIMBS * 64 >= 16 * W + 136 > 2 * MB + 128` for all `W > 1` and `MB < 8 * W - 2`,
/// and every limb in `0..MAX_LIMBS` is a full, untruncated `u64`.
pub(super) type Fixed<const W: usize> = Digits<u64, W, 2, 24>;

/// Saturating magnitude limit for parsed decimal exponents (`2^125 ≈ 4.25e37`), large enough
/// to represent all finite exponents for formats up to `EXPONENT_BITS = 127` while leaving
/// ample room to add/subtract string lengths without overflowing `i128`.
const EXP_SAT_LIMIT: i128 = 1i128 << 125;

/// Stack buffer capacity (in `u64` limbs) for exact midpoint comparisons in `no_alloc` builds.
/// 64 limbs = 4096 bits, which covers all exact midpoints with `|p| <= 1700` (including 100%
/// of `F16`, `F32`, `F64`, and every format with `EXPONENT_BITS <= 11`).
pub(super) const EXACT_STACK_LIMBS: usize = 64;

/// Maximum limb count for heap-allocated exact midpoint comparisons when `feature = "alloc"`
/// is enabled (32768 limbs ≈ 2,097,152 bits, covering 100% of `F80`, `F128`, and `F256`).
#[cfg(feature = "alloc")]
pub(super) const EXACT_HEAP_MAX_LIMBS: usize = 32_768;

/// Precomputed powers of 10 up to `10^19` (`10^19 < 2^64`).
pub(super) const POW10_U64: [u64; 20] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

/// Precomputed powers of 5 up to `5^27` (`5^27 < 2^64`).
const POW5_U64: [u64; 28] = [
    1,
    5,
    25,
    125,
    625,
    3_125,
    15_625,
    78_125,
    390_625,
    1_953_125,
    9_765_625,
    48_828_125,
    244_140_625,
    1_220_703_125,
    6_103_515_625,
    30_517_578_125,
    152_587_890_625,
    762_939_453_125,
    3_814_697_265_625,
    19_073_486_328_125,
    95_367_431_640_625,
    476_837_158_203_125,
    2_384_185_791_015_625,
    11_920_928_955_078_125,
    59_604_644_775_390_625,
    298_023_223_876_953_125,
    1_490_116_119_384_765_625,
    7_450_580_596_923_828_125,
];

#[inline]
const fn str_eq_case_insensitive(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut i = 0;
    while i < a.len() {
        let a_lower = a[i].to_ascii_lowercase();
        let b_lower = b[i].to_ascii_lowercase();
        if a_lower != b_lower {
            return false;
        }
        i += 1;
    }
    true
}

#[derive(Clone, Copy)]
struct ParsedDecimal<'a, const ALLOW_UNDERSCORES: bool> {
    is_negative: bool,
    int_digits: &'a [u8],
    frac_digits: &'a [u8],
    /// Decimal exponent of the first nonzero digit (`value = d_0.d_1... * 10^exp10`).
    exp10: i128,
    /// Byte index of the first nonzero digit in the concatenated `(int_digits, frac_digits)` stream.
    first_nz: usize,
    /// One past the byte index of the last nonzero digit in the concatenated stream.
    last_nz_end: usize,
    /// Number of ASCII digits (excluding underscores) in `first_nz..last_nz_end`.
    sig_digits: usize,
}

impl<'a, const ALLOW_UNDERSCORES: bool> ParsedDecimal<'a, ALLOW_UNDERSCORES> {
    #[inline(always)]
    const fn byte_at(&self, idx: usize) -> u8 {
        if idx < self.int_digits.len() {
            self.int_digits[idx]
        } else {
            self.frac_digits[idx - self.int_digits.len()]
        }
    }

    #[inline(always)]
    const fn digit_at(&self, idx: usize) -> u8 {
        self.byte_at(idx) - b'0'
    }

    #[inline(always)]
    const fn sig_len(&self) -> usize {
        self.sig_digits
    }

    /// Place value (power of 10) of the last nonzero digit.
    #[inline(always)]
    const fn last_nz_place(&self) -> i128 {
        self.exp10 - (self.sig_digits as i128 - 1)
    }
}

enum LexResult<'a, const ALLOW_UNDERSCORES: bool> {
    Zero { is_negative: bool },
    Infinity { is_negative: bool },
    Nan { is_negative: bool },
    Finite(ParsedDecimal<'a, ALLOW_UNDERSCORES>),
}

/// Lexes and validates `src` according to Rust's exact `f32`/`f64` `FromStr` grammar
/// (and optionally accepts `_` digit separators when `ALLOW_UNDERSCORES` is `true`):
///
/// ```text
/// Float  ::= Sign? ( 'inf' | 'infinity' | 'nan' | Number )
/// Number ::= ( Digit+ | Digit+ '.' Digit* | Digit* '.' Digit+ ) Exp?
/// Exp    ::= ('e' | 'E') ('+' | '-')? Digit+
/// ```
const fn lex_decimal<const ALLOW_UNDERSCORES: bool>(
    src: &str,
) -> Result<LexResult<'_, ALLOW_UNDERSCORES>, ParseFloatError> {
    if src.is_empty() {
        return Err(ParseFloatError::EMPTY);
    }

    let bytes = src.as_bytes();
    let (is_negative, rest) = match bytes[0] {
        b'+' => (false, bytes.split_at(1).1),
        b'-' => (true, bytes.split_at(1).1),
        _ => (false, bytes),
    };

    if rest.is_empty() {
        return Err(ParseFloatError::INVALID);
    }

    // Check special values (ASCII case-insensitive).
    // Fast-filter on first byte before full comparison.
    match rest[0] {
        b'i' | b'I' => {
            let s = unsafe { core::str::from_utf8_unchecked(rest) };
            if str_eq_case_insensitive(s, "inf") || str_eq_case_insensitive(s, "infinity") {
                return Ok(LexResult::Infinity { is_negative });
            }
            return Err(ParseFloatError::INVALID);
        }
        b'n' | b'N' => {
            let s = unsafe { core::str::from_utf8_unchecked(rest) };
            if str_eq_case_insensitive(s, "nan") {
                return Ok(LexResult::Nan { is_negative });
            }
            return Err(ParseFloatError::INVALID);
        }
        _ => {}
    }

    // Parse integer digits.
    let mut i = 0;
    let mut int_digit_count = 0usize;
    while i < rest.len() {
        let b = rest[i];
        if b.is_ascii_digit() {
            int_digit_count += 1;
            i += 1;
        } else if ALLOW_UNDERSCORES && b == b'_' {
            i += 1;
        } else {
            break;
        }
    }
    let (int_digits, after_int) = rest.split_at(i);

    // Parse optional fractional part.
    let mut frac_digit_count = 0usize;
    let (frac_digits, after_frac) = if !after_int.is_empty() && after_int[0] == b'.' {
        let after_dot = after_int.split_at(1).1;
        let mut j = 0;
        while j < after_dot.len() {
            let b = after_dot[j];
            if b.is_ascii_digit() {
                frac_digit_count += 1;
                j += 1;
            } else if ALLOW_UNDERSCORES && b == b'_' {
                j += 1;
            } else {
                break;
            }
        }
        after_dot.split_at(j)
    } else {
        (after_int.split_at(0).0, after_int)
    };

    // Must have at least one significand digit.
    if int_digit_count == 0 && frac_digit_count == 0 {
        return Err(ParseFloatError::INVALID);
    }

    // Parse optional exponent.
    let raw_exp: i128 = if !after_frac.is_empty() {
        if after_frac[0] != b'e' && after_frac[0] != b'E' {
            return Err(ParseFloatError::INVALID);
        }
        let exp_rest = after_frac.split_at(1).1;
        if exp_rest.is_empty() {
            return Err(ParseFloatError::INVALID);
        }
        let (exp_neg, exp_digits) = match exp_rest[0] {
            b'+' => (false, exp_rest.split_at(1).1),
            b'-' => (true, exp_rest.split_at(1).1),
            _ => (false, exp_rest),
        };

        let mut exp_digit_count = 0usize;
        let mut abs_exp: i128 = 0;
        let mut k = 0;
        while k < exp_digits.len() {
            let b = exp_digits[k];
            if ALLOW_UNDERSCORES && b == b'_' {
                k += 1;
                continue;
            }
            if !b.is_ascii_digit() {
                return Err(ParseFloatError::INVALID);
            }
            exp_digit_count += 1;
            let d = (b - b'0') as i128;
            if abs_exp < EXP_SAT_LIMIT {
                abs_exp = abs_exp.saturating_mul(10).saturating_add(d);
                if abs_exp > EXP_SAT_LIMIT {
                    abs_exp = EXP_SAT_LIMIT;
                }
            }
            k += 1;
        }
        if exp_digit_count == 0 {
            return Err(ParseFloatError::INVALID);
        }
        if exp_neg { -abs_exp } else { abs_exp }
    } else {
        0
    };

    // Locate first and last nonzero digits across (int_digits, frac_digits).
    let total_len = int_digits.len() + frac_digits.len();
    let mut first_nz = 0;
    let mut first_nz_digit_idx = 0usize;
    while first_nz < total_len {
        let b = if first_nz < int_digits.len() {
            int_digits[first_nz]
        } else {
            frac_digits[first_nz - int_digits.len()]
        };
        if ALLOW_UNDERSCORES && b == b'_' {
            first_nz += 1;
            continue;
        }
        if b != b'0' {
            break;
        }
        first_nz_digit_idx += 1;
        first_nz += 1;
    }

    if first_nz == total_len {
        return Ok(LexResult::Zero { is_negative });
    }

    let mut last_nz_end = total_len;
    while last_nz_end > first_nz {
        let idx = last_nz_end - 1;
        let b = if idx < int_digits.len() {
            int_digits[idx]
        } else {
            frac_digits[idx - int_digits.len()]
        };
        if b != b'0' && (!ALLOW_UNDERSCORES || b != b'_') {
            break;
        }
        last_nz_end -= 1;
    }

    let sig_digits = if ALLOW_UNDERSCORES {
        let mut count = 0usize;
        let mut p = first_nz;
        while p < last_nz_end {
            let b = if p < int_digits.len() {
                int_digits[p]
            } else {
                frac_digits[p - int_digits.len()]
            };
            if b != b'_' {
                count += 1;
            }
            p += 1;
        }
        count
    } else {
        last_nz_end - first_nz
    };

    let exp10 = raw_exp + (int_digit_count as i128) - 1 - (first_nz_digit_idx as i128);

    Ok(LexResult::Finite(ParsedDecimal {
        is_negative,
        int_digits,
        frac_digits,
        exp10,
        first_nz,
        last_nz_end,
        sig_digits,
    }))
}

// -----------------------------------------------------------------------------
// Fixed-point primitives on `Fixed<W>` (full 64-bit limbs in `0..limbs`)
// -----------------------------------------------------------------------------

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
pub(super) const fn fp_shl<const W: usize>(a: &Fixed<W>, shift: Exponent, limbs: usize) -> Fixed<W> {
    if shift == 0 {
        return *a;
    }
    let mut out = Fixed::<W>::ALL_ZEROS;
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

/// Multiplies `a` in place by `mul` and adds `add`, returning the overflow carry (if any).
#[inline]
const fn fp_mul_add_u64<const W: usize>(a: &mut Fixed<W>, mul: u64, add: u64, limbs: usize) -> u64 {
    let mut carry = add as u128;
    let mul128 = mul as u128;
    let mut i = 0;
    while i < limbs {
        let prod = (a.get(i) as u128) * mul128 + carry;
        a.set(i, prod as u64);
        carry = prod >> 64;
        i += 1;
    }
    carry as u64
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
        if ai == 0 {
            i += 1;
            continue;
        }
        let mut carry: u128 = 0;
        let mut j = if i + 1 < limbs { limbs - 1 - i } else { 0 };
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
        i += 1;
    }
    (out, guard)
}

/// Multiplies two normalized numbers `(a * 2^(ea - P))` and `(b * 2^(eb - P))` where both
/// have bit `P - 1` set (`in [0.5, 1.0)`), returning a normalized product `(m, em)`.
#[inline]
pub(super) const fn fp_norm_mul<const W: usize>(
    a: &Fixed<W>,
    ea: i128,
    b: &Fixed<W>,
    eb: i128,
    limbs: usize,
) -> (Fixed<W>, i128) {
    let (hi, guard) = fp_mul_hi_with_guard(a, b, limbs);
    if (hi.get(limbs - 1) >> 63) != 0 {
        (hi, ea + eb)
    } else {
        (fp_shl1_with_bit(&hi, guard >> 63, limbs), ea + eb - 1)
    }
}

/// Computes `1 / D` for `D = d * 2^(-P) in (0.5, 1.0)` via Newton-Raphson, returning
/// `r` such that `R = r * 2^(-(P - 1)) in (1.0, 2.0)`.
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

/// Computes normalized `5^q10` as `(mant, exp2)` such that `5^q10 ≈ mant * 2^(exp2 - 64*limbs)`
/// with bit `64*limbs - 1` of `mant` set.
pub(super) const fn fp_pow5<const W: usize>(q10: i128, limbs: usize) -> (Fixed<W>, i128) {
    let abs_q = q10.unsigned_abs();
    if abs_q == 0 {
        let mut one = Fixed::<W>::ALL_ZEROS;
        one.set(limbs - 1, 1u64 << 63);
        return (one, 1);
    }

    // base = 5 = 0.625 * 2^3 (bit P-1 and P-3 set: 0xA000_0000_0000_0000)
    let mut base = Fixed::<W>::ALL_ZEROS;
    base.set(limbs - 1, 5u64 << 61);
    let mut base_exp: i128 = 3;

    let mut acc = Fixed::<W>::ALL_ZEROS;
    let mut acc_exp: i128 = 0;
    let mut have_acc = false;

    let mut rem = abs_q;
    loop {
        if (rem & 1) != 0 {
            if have_acc {
                (acc, acc_exp) = fp_norm_mul(&acc, acc_exp, &base, base_exp, limbs);
            } else {
                acc = base;
                acc_exp = base_exp;
                have_acc = true;
            }
        }
        rem >>= 1;
        if rem == 0 {
            break;
        }
        (base, base_exp) = fp_norm_mul(&base, base_exp, &base, base_exp, limbs);
    }

    if q10 < 0 {
        // Since abs_q >= 1, 5^abs_q is odd (never a power of 2), so acc * 2^(-P) in (0.5, 1.0)
        // strictly, and 1 / (acc * 2^(-P)) = r * 2^(-(P - 1)) = (r * 2^(-P)) * 2^1.
        let r = fp_recip(&acc, limbs);
        (r, 1 - acc_exp)
    } else {
        (acc, acc_exp)
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

// -----------------------------------------------------------------------------
// Exact slice-based big-integer comparison for Stage 3 (midpoint resolution)
// -----------------------------------------------------------------------------

/// Multiplies `buf[..*len]` by `mul` and adds `add`, extending `*len` if needed.
/// Returns `true` if the result overflows `buf.len()`.
#[inline]
pub(super) const fn slice_mul_add_u64(buf: &mut [u64], len: &mut usize, mul: u64, add: u64) -> bool {
    let mut carry = add as u128;
    let mul128 = mul as u128;
    let cur_len = *len;
    let mut i = 0;
    while i < cur_len {
        let prod = (buf[i] as u128) * mul128 + carry;
        buf[i] = prod as u64;
        carry = prod >> 64;
        i += 1;
    }
    if carry != 0 {
        if cur_len >= buf.len() {
            return true;
        }
        buf[cur_len] = carry as u64;
        *len = cur_len + 1;
    }
    false
}

/// Multiplies `buf[..*len]` by `10^count`. Returns `true` on overflow of `buf`.
const fn slice_mul_pow10(buf: &mut [u64], len: &mut usize, mut count: u128) -> bool {
    if *len == 0 {
        return false;
    }
    while count >= 19 {
        if slice_mul_add_u64(buf, len, POW10_U64[19], 0) {
            return true;
        }
        count -= 19;
    }
    if count > 0 {
        if slice_mul_add_u64(buf, len, POW10_U64[count as usize], 0) {
            return true;
        }
    }
    false
}

/// Multiplies `buf[..*len]` by `5^count`. Caller guarantees `buf` is large enough.
pub(super) const fn slice_mul_pow5(buf: &mut [u64], len: &mut usize, mut count: u128) {
    while count >= 27 {
        let ovf = slice_mul_add_u64(buf, len, POW5_U64[27], 0);
        debug_assert!(!ovf);
        count -= 27;
    }
    if count > 0 {
        let ovf = slice_mul_add_u64(buf, len, POW5_U64[count as usize], 0);
        debug_assert!(!ovf);
    }
}

/// Compares `a[..len_a]` against `b[..len_b]` (normalized, no leading zero limbs assumed).
pub(super) const fn slice_cmp(a: &[u64], len_a: usize, b: &[u64], len_b: usize) -> Ordering {
    let mut la = len_a;
    while la > 0 && a[la - 1] == 0 {
        la -= 1;
    }
    let mut lb = len_b;
    while lb > 0 && b[lb - 1] == 0 {
        lb -= 1;
    }
    if la < lb {
        return Ordering::Less;
    }
    if la > lb {
        return Ordering::Greater;
    }
    let mut i = la;
    while i > 0 {
        i -= 1;
        if a[i] < b[i] {
            return Ordering::Less;
        }
        if a[i] > b[i] {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

/// Loads `m_odd << shift` into `buf_m` and returns its active limb length.
pub(super) const fn slice_load_uint_shl<const W: usize>(
    m_odd: &Uint<W>,
    shift: usize,
    buf: &mut [u64],
) -> usize {
    let u_digits = m_odd.as_digits::<u64>();
    let u_limbs = W.div_ceil(8);
    let ws = shift / 64;
    let bs = shift % 64;
    let mut top = 0;

    let mut i = 0;
    while i < u_limbs {
        let d = u_digits.get(i);
        if d == 0 {
            i += 1;
            continue;
        }
        let dst = i + ws;
        if bs == 0 {
            buf[dst] |= d;
            if dst + 1 > top {
                top = dst + 1;
            }
        } else {
            buf[dst] |= d << bs;
            if dst + 1 > top {
                top = dst + 1;
            }
            let hi = d >> (64 - bs);
            if hi != 0 {
                buf[dst + 1] |= hi;
                if dst + 2 > top {
                    top = dst + 2;
                }
            }
        }
        i += 1;
    }
    top
}

/// Parses `floor(dec * 10^scale_pow10)` into `buf_v` (where `scale_pow10 >= 0`), returning
/// `None` if the value overflows `buf_v` (meaning `V > M`), or `Some((len_v, has_tail_nonzero))`.
const fn slice_parse_scaled_decimal<const ALLOW_UNDERSCORES: bool>(
    dec: &ParsedDecimal<'_, ALLOW_UNDERSCORES>,
    scale_pow10: u128,
    buf_v: &mut [u64],
) -> Option<(usize, bool)> {
    // Digit index `first_nz` has place `exp10 + scale_pow10` after multiplying by 10^scale_pow10.
    let top_place = dec.exp10 + (scale_pow10 as i128);
    if top_place < 0 {
        // V * 10^scale_pow10 < 1, while M * 10^scale_pow10 >= 1 (strictly less).
        return Some((0, true));
    }

    let total_int_digits = (top_place as u128) + 1;
    let avail = dec.sig_len() as u128;
    let read_digits = if avail < total_int_digits {
        avail as usize
    } else {
        total_int_digits as usize
    };
    let trailing_zeros = total_int_digits - (read_digits as u128);
    let has_tail_nonzero = dec.last_nz_place() < -(scale_pow10 as i128);

    let mut len_v = 0usize;
    let mut pos = dec.first_nz;
    if ALLOW_UNDERSCORES {
        let mut done = 0usize;
        while done < read_digits {
            let rem = read_digits - done;
            let max_chunk = if rem < 19 { rem } else { 19 };
            let mut chunk: u64 = 0;
            let mut chunk_len = 0usize;
            while chunk_len < max_chunk {
                let b = dec.byte_at(pos);
                pos += 1;
                if b != b'_' {
                    chunk = chunk * 10 + ((b - b'0') as u64);
                    chunk_len += 1;
                }
            }
            if slice_mul_add_u64(buf_v, &mut len_v, POW10_U64[chunk_len], chunk) {
                return None;
            }
            done += chunk_len;
        }
    } else {
        let end = dec.first_nz + read_digits;
        while pos < end {
            let rem = end - pos;
            let chunk_len = if rem < 19 { rem } else { 19 };
            let mut chunk: u64 = 0;
            let mut k = 0;
            while k < chunk_len {
                chunk = chunk * 10 + (dec.digit_at(pos + k) as u64);
                k += 1;
            }
            if slice_mul_add_u64(buf_v, &mut len_v, POW10_U64[chunk_len], chunk) {
                return None;
            }
            pos += chunk_len;
        }
    }

    if trailing_zeros > 0 && slice_mul_pow10(buf_v, &mut len_v, trailing_zeros) {
        return None;
    }

    Some((len_v, has_tail_nonzero))
}

/// Exact comparison of `dec` against the dyadic midpoint `M = m_odd * 2^p` using the provided
/// zero-initialized scratch buffers `buf_m` and `buf_v`.
const fn exact_cmp_midpoint_in_bufs<const W: usize, const ALLOW_UNDERSCORES: bool>(
    dec: &ParsedDecimal<'_, ALLOW_UNDERSCORES>,
    m_odd: &Uint<W>,
    p: i128,
    buf_m: &mut [u64],
    buf_v: &mut [u64],
) -> Ordering {
    let (len_m, scale_pow10) = if p >= 0 {
        let lm = slice_load_uint_shl(m_odd, p as usize, buf_m);
        (lm, 0u128)
    } else {
        let n = (-p) as u128;
        let mut lm = slice_load_uint_shl(m_odd, 0, buf_m);
        slice_mul_pow5(buf_m, &mut lm, n);
        (lm, n)
    };

    match slice_parse_scaled_decimal(dec, scale_pow10, buf_v) {
        None => Ordering::Greater, // V * 10^scale overflowed M's buffer => V > M strictly
        Some((len_v, has_tail_nonzero)) => match slice_cmp(buf_v, len_v, buf_m, len_m) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                if has_tail_nonzero {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        },
    }
}

/// Attempts an exact comparison of `dec` against `M = m_odd * 2^p`.
/// Returns `Some(ordering)` whenever `M` fits in our exact stack buffer (always true for
/// `F16`, `F32`, `F64` and any `|p| <= 1700`) or heap buffer (when `feature = "alloc"` is
/// enabled, covering all `F80`, `F128`, `F256`).
fn exact_cmp_midpoint<const W: usize, const MB: usize, const ALLOW_UNDERSCORES: bool>(
    dec: &ParsedDecimal<'_, ALLOW_UNDERSCORES>,
    m_odd: &Uint<W>,
    p: i128,
) -> Option<Ordering> {
    let abs_p = p.unsigned_abs();
    // Guard against absurdly huge |p| (e.g., EXPONENT_BITS >= 64) before computing limb count.
    if abs_p > 1_000_000 {
        return None;
    }
    let needed_bits: usize = if p >= 0 {
        (MB + 3) + (abs_p as usize)
    } else {
        // log2(5) = 2.321928... < 2322 / 1000
        (MB + 4) + ((abs_p * 2322) / 1000) as usize
    };
    let needed_limbs = needed_bits.div_ceil(64) + 1;

    if needed_limbs <= EXACT_STACK_LIMBS {
        let mut buf_m = [0u64; EXACT_STACK_LIMBS];
        let mut buf_v = [0u64; EXACT_STACK_LIMBS];
        return Some(exact_cmp_midpoint_in_bufs(
            dec,
            m_odd,
            p,
            buf_m.split_at_mut(needed_limbs).0,
            buf_v.split_at_mut(needed_limbs).0,
        ));
    }

    #[cfg(feature = "alloc")]
    if needed_limbs <= EXACT_HEAP_MAX_LIMBS {
        let mut buf_m = alloc::vec![0u64; needed_limbs];
        let mut buf_v = alloc::vec![0u64; needed_limbs];
        return Some(exact_cmp_midpoint_in_bufs(
            dec,
            m_odd,
            p,
            &mut buf_m,
            &mut buf_v,
        ));
    }

    None
}

const fn exact_cmp_midpoint_with_const_buf<
    const LIMBS: usize,
    const W: usize,
    const ALLOW_UNDERSCORES: bool,
>(
    dec: &ParsedDecimal<'_, ALLOW_UNDERSCORES>,
    m_odd: &Uint<W>,
    p: i128,
    needed_limbs: usize,
) -> Ordering {
    let mut buf_m = [0u64; LIMBS];
    let mut buf_v = [0u64; LIMBS];
    exact_cmp_midpoint_in_bufs(
        dec,
        m_odd,
        p,
        buf_m.split_at_mut(needed_limbs).0,
        buf_v.split_at_mut(needed_limbs).0,
    )
}

/// Const-evaluation exact midpoint comparison using tiered stack buffers
/// (covering 100% of `F16`, `F32`, `F64`, `F80`, `F128`, and `F256`).
const fn exact_cmp_midpoint_const<
    const W: usize,
    const MB: usize,
    const ALLOW_UNDERSCORES: bool,
>(
    dec: &ParsedDecimal<'_, ALLOW_UNDERSCORES>,
    m_odd: &Uint<W>,
    p: i128,
) -> Option<Ordering> {
    let abs_p = p.unsigned_abs();
    if abs_p > 1_000_000 {
        return None;
    }
    let needed_bits: usize = if p >= 0 {
        (MB + 3) + (abs_p as usize)
    } else {
        (MB + 4) + ((abs_p * 2322) / 1000) as usize
    };
    let needed_limbs = needed_bits.div_ceil(64) + 1;

    if needed_limbs <= EXACT_STACK_LIMBS {
        return Some(exact_cmp_midpoint_with_const_buf::<
            EXACT_STACK_LIMBS,
            W,
            ALLOW_UNDERSCORES,
        >(dec, m_odd, p, needed_limbs));
    }
    if needed_limbs <= 1024 {
        return Some(exact_cmp_midpoint_with_const_buf::<
            1024,
            W,
            ALLOW_UNDERSCORES,
        >(dec, m_odd, p, needed_limbs));
    }
    if needed_limbs <= 10_000 {
        return Some(exact_cmp_midpoint_with_const_buf::<
            10_000,
            W,
            ALLOW_UNDERSCORES,
        >(dec, m_odd, p, needed_limbs));
    }

    None
}

// -----------------------------------------------------------------------------
// Stage 1 / Stage 2 scaling & rounding + Float::from_str entry point
// -----------------------------------------------------------------------------

#[inline]
const fn fp_extract_top64<const W: usize>(a: &Fixed<W>, shift: Exponent, limbs: usize) -> u64 {
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

#[inline]
const fn uint_low_u64_fs<const W: usize>(u: &Uint<W>) -> u64 {
    let mut v = 0u64;
    let mut i = 0;
    while i < W && i < 8 {
        v |= (u.bytes[i] as u64) << (i * 8);
        i += 1;
    }
    v
}

enum ScaleOutcome<const W: usize, const MB: usize, const RM: u8> {
    Resolved(Float<W, MB, RM>),
    /// Value lies within the error margin of either:
    /// - the halfway midpoint `M = (2 * q_lo + 1) * 2^p` (`is_grid_boundary == false`, nearest modes), or
    /// - the exact grid point `G = (2 * (q_lo + 1)) * 2^p` (`is_grid_boundary == true`, directed/odd modes).
    Ambiguous {
        is_subnormal: bool,
        q_lo: Uint<W>,
        p: i128,
        approx_q: Uint<W>,
        is_grid_boundary: bool,
    },
}

enum Stage12Outcome<'a, const W: usize, const MB: usize, const RM: u8, const ALLOW_UNDERSCORES: bool> {
    Resolved(Float<W, MB, RM>),
    NeedsStage3 {
        dec: ParsedDecimal<'a, ALLOW_UNDERSCORES>,
        bias: Uint<W>,
        bias_i128: Option<i128>,
        is_subnormal: bool,
        q_lo: Uint<W>,
        p: i128,
        approx_q: Uint<W>,
        is_grid_boundary: bool,
    },
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Packs a rounded significand `q` (normal or subnormal) into `Float<W, MB, RM>`,
    /// handling rounding carry into the next power of two, subnormal-to-normal carry,
    /// and mode-appropriate overflow.
    const fn pack_rounded(
        is_negative: bool,
        is_subnormal: bool,
        out_exp: i128,
        mut q: Uint<W>,
        bias: Uint<W>,
        bias_i128: Option<i128>,
    ) -> Self {
        let (biased_out_exp, mant_bits): (Uint<W>, Uint<W>) = if is_subnormal {
            if q.is_zero() {
                return if is_negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }
            if q.bit(Self::MB) {
                (Uint::ONE, Uint::ZERO)
            } else {
                (Uint::ZERO, q)
            }
        } else {
            let mut final_exp = out_exp;
            let carry_pow2 = q.bit(Self::MB + 1);
            if carry_pow2 {
                final_exp += 1;
            }
            if let Some(b_i128) = bias_i128 {
                if final_exp > b_i128 {
                    return Self::overflow_value(is_negative);
                }
            }
            let biased = add_i128_to_uint(bias, final_exp);
            if carry_pow2 {
                (biased, Uint::ZERO)
            } else {
                q.set_bit(Self::MB, false);
                (biased, q)
            }
        };

        let mut out_bits = biased_out_exp.shl(Self::MB).bitor(mant_bits);
        if is_negative {
            out_bits.set_bit(Self::BITS - 1, true);
        }
        Self::from_bits(out_bits)
    }

    /// Rounds `prod_mant` shifted right by `drop` under rounding mode `RM` at working
    /// precision `limbs`, returning `Resolved` if provably unambiguous or `Ambiguous` to trigger
    /// higher precision / exact Stage 3 comparison.
    const fn resolve_with_rm<const ALLOW_UNDERSCORES: bool>(
        dec: &ParsedDecimal<'_, ALLOW_UNDERSCORES>,
        prod_mant: &Fixed<W>,
        is_subnormal: bool,
        out_exp: i128,
        drop: Exponent,
        err_bits: Exponent,
        p_exact: i128,
        bias: Uint<W>,
        bias_i128: Option<i128>,
        limbs: usize,
    ) -> ScaleOutcome<W, MB, RM> {
        let p_bits = (limbs as Exponent) * 64;
        let (mid, carry_mid) = fp_add_pow2_with_carry(prod_mant, 0, drop - 1, limbs);
        let approx_q = fp_shr_with_carry_to_uint(&mid, carry_mid, drop, limbs);

        if RoundingMode::is_faithful(RM) {
            return ScaleOutcome::Resolved(Self::pack_rounded(
                dec.is_negative,
                is_subnormal,
                out_exp,
                approx_q,
                bias,
                bias_i128,
            ));
        }

        if RoundingMode::is_stochastic(RM) {
            let mut q = fp_shr_with_carry_to_uint(prod_mant, 0, drop, limbs);
            if err_bits + 2 < drop {
                let (hi, carry_hi) = fp_add_pow2_with_carry(prod_mant, 0, err_bits, limbs);
                let (lo, carry_lo) = fp_sub_pow2_with_carry(prod_mant, 0, err_bits, limbs);
                let q_hi = fp_shr_with_carry_to_uint(&hi, carry_hi, drop, limbs);
                let q_lo = fp_shr_with_carry_to_uint(&lo, carry_lo, drop, limbs);
                if !q_hi.eq(&q_lo) {
                    // Within `2^err_bits` of exact grid point `q_hi` (e.g. exact decimal `"1.5"`).
                    return ScaleOutcome::Resolved(Self::pack_rounded(
                        dec.is_negative,
                        is_subnormal,
                        out_exp,
                        q_hi,
                        bias,
                        bias_i128,
                    ));
                }
            }
            let frac64 = if drop <= p_bits {
                fp_extract_top64(prod_mant, drop, limbs)
            } else {
                // Subnormal shift > p_bits: drop == p_bits + 1 means [0.5, 1.0) of subnormal
                fp_extract_top64(prod_mant, p_bits, limbs) >> (drop - p_bits)
            };
            let round_bit = (frac64 >> 63) != 0;
            let sticky = (frac64 & !(1u64 << 63)) != 0 || drop > 64;
            let seed = (out_exp as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ uint_low_u64_fs(&q);
            if RoundingMode::should_round_up::<RM>(
                dec.is_negative,
                q.is_odd(),
                round_bit,
                sticky,
                frac64,
                seed,
            ) {
                q = q.add(Uint::ONE);
            }
            return ScaleOutcome::Resolved(Self::pack_rounded(
                dec.is_negative,
                is_subnormal,
                out_exp,
                q,
                bias,
                bias_i128,
            ));
        }

        if RoundingMode::is_directed_or_odd(RM) {
            if err_bits + 2 >= drop {
                let q_trunc = fp_shr_with_carry_to_uint(prod_mant, 0, drop, limbs);
                return ScaleOutcome::Ambiguous {
                    is_subnormal,
                    q_lo: q_trunc,
                    p: p_exact,
                    approx_q: q_trunc,
                    is_grid_boundary: true,
                };
            }
            let (hi, carry_hi) = fp_add_pow2_with_carry(prod_mant, 0, err_bits, limbs);
            let (lo, carry_lo) = fp_sub_pow2_with_carry(prod_mant, 0, err_bits, limbs);
            let q_hi = fp_shr_with_carry_to_uint(&hi, carry_hi, drop, limbs);
            let q_lo = fp_shr_with_carry_to_uint(&lo, carry_lo, drop, limbs);

            if q_hi.eq(&q_lo) {
                // Provably strictly inside (q_lo, q_lo + 1) — definitely inexact!
                let mut q = q_lo;
                if RoundingMode::should_round_up::<RM>(
                    dec.is_negative,
                    q.is_odd(),
                    false,
                    true,
                    0,
                    0,
                ) {
                    q = q.add(Uint::ONE);
                }
                return ScaleOutcome::Resolved(Self::pack_rounded(
                    dec.is_negative,
                    is_subnormal,
                    out_exp,
                    q,
                    bias,
                    bias_i128,
                ));
            }

            return ScaleOutcome::Ambiguous {
                is_subnormal,
                q_lo,
                p: p_exact,
                approx_q: q_hi,
                is_grid_boundary: true,
            };
        }

        // Round-to-nearest modes (TiesToEven, TiesToAway, TiesTowardPositive/Negative/Zero, TiesToOdd)
        if err_bits + 2 >= drop {
            return ScaleOutcome::Ambiguous {
                is_subnormal,
                q_lo: approx_q,
                p: p_exact,
                approx_q,
                is_grid_boundary: false,
            };
        }

        let (hi, carry_hi) = fp_add_pow2_with_carry(&mid, carry_mid, err_bits, limbs);
        let (lo, carry_lo) = fp_sub_pow2_with_carry(&mid, carry_mid, err_bits, limbs);
        let q_hi = fp_shr_with_carry_to_uint(&hi, carry_hi, drop, limbs);
        let q_lo = fp_shr_with_carry_to_uint(&lo, carry_lo, drop, limbs);

        if q_hi.eq(&q_lo) {
            return ScaleOutcome::Resolved(Self::pack_rounded(
                dec.is_negative,
                is_subnormal,
                out_exp,
                q_lo,
                bias,
                bias_i128,
            ));
        }

        ScaleOutcome::Ambiguous {
            is_subnormal,
            q_lo,
            p: p_exact,
            approx_q,
            is_grid_boundary: false,
        }
    }

    /// Evaluates `dec` at `limbs` 64-bit limbs of working precision and checks whether the
    /// rounding decision is provably unambiguous.
    const fn scale_at_precision<const ALLOW_UNDERSCORES: bool>(
        dec: &ParsedDecimal<'_, ALLOW_UNDERSCORES>,
        bias: Uint<W>,
        bias_i128: Option<i128>,
        limbs: usize,
    ) -> ScaleOutcome<W, MB, RM> {
        let p_bits = (limbs as Exponent) * 64;

        let mut d_val = Fixed::<W>::ALL_ZEROS;
        let mut pos = dec.first_nz;
        let end = dec.last_nz_end;
        let mut digits_read = 0usize;

        if ALLOW_UNDERSCORES {
            let fast_limit_digits = (limbs - 1) * 19;
            while pos < end && digits_read < fast_limit_digits {
                let rem = fast_limit_digits - digits_read;
                let max_chunk = if rem < 19 { rem } else { 19 };
                let mut chunk: u64 = 0;
                let mut chunk_len = 0usize;
                while pos < end && chunk_len < max_chunk {
                    let b = dec.byte_at(pos);
                    pos += 1;
                    if b != b'_' {
                        chunk = chunk * 10 + ((b - b'0') as u64);
                        chunk_len += 1;
                    }
                }
                if chunk_len > 0 {
                    let ovf = fp_mul_add_u64(&mut d_val, POW10_U64[chunk_len], chunk, limbs);
                    debug_assert!(ovf == 0);
                    digits_read += chunk_len;
                }
            }
            let top_thresh = u64::MAX / 10;
            while pos < end && d_val.get(limbs - 1) < top_thresh {
                let b = dec.byte_at(pos);
                pos += 1;
                if b != b'_' {
                    let d = (b - b'0') as u64;
                    let ovf = fp_mul_add_u64(&mut d_val, 10, d, limbs);
                    debug_assert!(ovf == 0);
                    digits_read += 1;
                }
            }
        } else {
            let fast_limit = dec.first_nz + (limbs - 1) * 19;
            let fast_end = if end < fast_limit { end } else { fast_limit };
            while pos < fast_end {
                let rem = fast_end - pos;
                let chunk_len = if rem < 19 { rem } else { 19 };
                let mut chunk: u64 = 0;
                let mut k = 0;
                while k < chunk_len {
                    chunk = chunk * 10 + (dec.digit_at(pos + k) as u64);
                    k += 1;
                }
                let ovf = fp_mul_add_u64(&mut d_val, POW10_U64[chunk_len], chunk, limbs);
                debug_assert!(ovf == 0);
                pos += chunk_len;
            }
            let top_thresh = u64::MAX / 10;
            while pos < end && d_val.get(limbs - 1) < top_thresh {
                let d = dec.digit_at(pos) as u64;
                let ovf = fp_mul_add_u64(&mut d_val, 10, d, limbs);
                debug_assert!(ovf == 0);
                pos += 1;
            }
            digits_read = pos - dec.first_nz;
        }

        let q10: i128 = dec.exp10 - (digits_read as i128 - 1);

        let d_bw = fp_bit_width(&d_val, limbs);
        let d_norm = fp_shl(&d_val, p_bits - d_bw, limbs);
        let d_exp2 = d_bw as i128;

        let (p5_norm, p5_exp2) = fp_pow5::<W>(q10, limbs);
        let (prod_mant, prod_exp) = fp_norm_mul(&d_norm, d_exp2, &p5_norm, p5_exp2, limbs);

        let out_exp = prod_exp + q10 - 1;

        let abs_q = q10.unsigned_abs();
        let q_bits: Exponent = if abs_q == 0 {
            0
        } else {
            128 - abs_q.leading_zeros()
        };
        let limb_bits: Exponent = 64 - (limbs as u64).leading_zeros();
        let err_bits: Exponent = q_bits + limb_bits + 8;

        if let Some(b_i128) = bias_i128 {
            let min_normal_exp = 1 - b_i128;
            let min_subnormal_exp = min_normal_exp - (MB as i128);

            if out_exp > b_i128 {
                return ScaleOutcome::Resolved(Self::overflow_value(dec.is_negative));
            }

            if out_exp < min_subnormal_exp - 2 {
                return ScaleOutcome::Resolved(Self::underflow_nonzero(dec.is_negative));
            }

            if out_exp == min_subnormal_exp - 2 {
                if RoundingMode::is_directed_or_odd(RM) {
                    return ScaleOutcome::Resolved(Self::underflow_nonzero(dec.is_negative));
                }
                if RoundingMode::is_faithful(RM) {
                    return ScaleOutcome::Resolved(if dec.is_negative {
                        Self::NEG_ZERO
                    } else {
                        Self::ZERO
                    });
                }
                if RoundingMode::is_stochastic(RM) {
                    let frac64 = fp_extract_top64(&prod_mant, p_bits, limbs) >> 2;
                    let seed = (out_exp as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
                    return ScaleOutcome::Resolved(
                        if RoundingMode::should_round_up::<RM>(
                            dec.is_negative,
                            false,
                            false,
                            true,
                            frac64,
                            seed,
                        ) {
                            Self::from_biased_uint_parts(dec.is_negative, Uint::ONE, Uint::ONE)
                        } else if dec.is_negative {
                            Self::NEG_ZERO
                        } else {
                            Self::ZERO
                        },
                    );
                }
                if err_bits < p_bits {
                    let (_, carry) = fp_add_pow2_with_carry(&prod_mant, 0, err_bits, limbs);
                    if carry == 0 {
                        return ScaleOutcome::Resolved(if dec.is_negative {
                            Self::NEG_ZERO
                        } else {
                            Self::ZERO
                        });
                    }
                }
                return ScaleOutcome::Ambiguous {
                    is_subnormal: true,
                    q_lo: Uint::ZERO,
                    p: min_subnormal_exp - 1,
                    approx_q: Uint::ZERO,
                    is_grid_boundary: false,
                };
            }

            if out_exp < min_normal_exp {
                let diff = (min_normal_exp - out_exp) as Exponent;
                let drop = p_bits + diff - (Self::MB + 1);
                return Self::resolve_with_rm(
                    dec,
                    &prod_mant,
                    true,
                    out_exp,
                    drop,
                    err_bits,
                    min_subnormal_exp - 1,
                    bias,
                    bias_i128,
                    limbs,
                );
            }
        }

        let drop = p_bits - (Self::MB + 1);
        Self::resolve_with_rm(
            dec,
            &prod_mant,
            false,
            out_exp,
            drop,
            err_bits,
            out_exp - (MB as i128) - 1,
            bias,
            bias_i128,
            limbs,
        )
    }

    const fn parse_stages_1_2<'a, const ALLOW_UNDERSCORES: bool>(
        src: &'a str,
    ) -> Result<Stage12Outcome<'a, W, MB, RM, ALLOW_UNDERSCORES>, ParseFloatError> {
        const {
            assert!(W > 1, "Float byte width W must be greater than 1");
            assert!(
                MB > 2 && MB < (Self::BITS as usize) - 2,
                "Float mantissa bits MB must satisfy 2 < MB < Self::BITS - 2"
            );
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }

        let dec = match lex_decimal::<ALLOW_UNDERSCORES>(src) {
            Ok(LexResult::Zero { is_negative }) => {
                return Ok(Stage12Outcome::Resolved(if is_negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }));
            }
            Ok(LexResult::Infinity { is_negative }) => {
                return Ok(Stage12Outcome::Resolved(if is_negative {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                }));
            }
            Ok(LexResult::Nan { is_negative }) => {
                return Ok(Stage12Outcome::Resolved(if is_negative {
                    Self::NEG_NAN
                } else {
                    Self::NAN
                }));
            }
            Ok(LexResult::Finite(dec)) => dec,
            Err(e) => return Err(e),
        };

        let bias: Uint<W> = Uint::power_of_two(Self::EXPONENT_BITS - 1).sub(Uint::ONE);
        let bias_i128: Option<i128> = if Self::EXPONENT_BITS <= 127 {
            Some(((1u128 << (Self::EXPONENT_BITS - 1)) - 1) as i128)
        } else {
            None
        };

        if let Some(b_i128) = bias_i128 {
            let max_exp10 = (b_i128 + 1) / 3 + 2;
            if dec.exp10 > max_exp10 {
                return Ok(Stage12Outcome::Resolved(Self::overflow_value(
                    dec.is_negative,
                )));
            }
            let min_exp10 = -((b_i128 + MB as i128) / 3 + 3);
            if dec.exp10 < min_exp10 {
                return Ok(Stage12Outcome::Resolved(Self::underflow_nonzero(
                    dec.is_negative,
                )));
            }
        }

        let max_limbs = (2 * W + 24) / 8;
        let exp10_bits = if dec.exp10 == 0 {
            0
        } else {
            128 - dec.exp10.unsigned_abs().leading_zeros()
        };
        let raw_l1 = (MB + 56 + exp10_bits as usize).div_ceil(64);
        let l1 = if raw_l1 < max_limbs { raw_l1 } else { max_limbs };

        if let ScaleOutcome::Resolved(f) = Self::scale_at_precision(&dec, bias, bias_i128, l1) {
            return Ok(Stage12Outcome::Resolved(f));
        }

        match Self::scale_at_precision(&dec, bias, bias_i128, max_limbs) {
            ScaleOutcome::Resolved(f) => Ok(Stage12Outcome::Resolved(f)),
            ScaleOutcome::Ambiguous {
                is_subnormal,
                q_lo,
                p,
                approx_q,
                is_grid_boundary,
            } => Ok(Stage12Outcome::NeedsStage3 {
                dec,
                bias,
                bias_i128,
                is_subnormal,
                q_lo,
                p,
                approx_q,
                is_grid_boundary,
            }),
        }
    }

    const fn finish_stage3(
        is_negative: bool,
        bias: Uint<W>,
        bias_i128: Option<i128>,
        is_subnormal: bool,
        q_lo: Uint<W>,
        p: i128,
        approx_q: Uint<W>,
        is_grid_boundary: bool,
        cmp_result: Option<Ordering>,
    ) -> Self {
        if is_grid_boundary {
            let q_hi = q_lo.add(Uint::ONE);
            let g_overflows = if !is_subnormal && q_hi.bit(Self::MB + 1) {
                matches!(bias_i128, Some(b_i128) if p + (Self::MB as i128) + 2 > b_i128)
            } else {
                false
            };
            let out_exp = p + (Self::MB as i128) + 1;
            return match cmp_result {
                Some(Ordering::Equal) => {
                    if g_overflows {
                        Self::overflow_value(is_negative)
                    } else {
                        Self::pack_rounded(
                            is_negative,
                            is_subnormal,
                            out_exp,
                            q_hi,
                            bias,
                            bias_i128,
                        )
                    }
                }
                Some(Ordering::Greater) => {
                    if g_overflows {
                        Self::overflow_value(is_negative)
                    } else {
                        let g = Self::pack_rounded(
                            is_negative,
                            is_subnormal,
                            out_exp,
                            q_hi,
                            bias,
                            bias_i128,
                        );
                        Self::round_with_tiny_delta(g, !is_negative)
                    }
                }
                Some(Ordering::Less) => {
                    if g_overflows {
                        let max_signed = if is_negative { Self::MIN } else { Self::MAX };
                        Self::round_with_tiny_delta(max_signed, !is_negative)
                    } else {
                        let g = Self::pack_rounded(
                            is_negative,
                            is_subnormal,
                            out_exp,
                            q_hi,
                            bias,
                            bias_i128,
                        );
                        Self::round_with_tiny_delta(g, is_negative)
                    }
                }
                None => Self::pack_rounded(
                    is_negative,
                    is_subnormal,
                    out_exp,
                    approx_q,
                    bias,
                    bias_i128,
                ),
            };
        }

        let final_q = match cmp_result {
            Some(Ordering::Less) => q_lo,
            Some(Ordering::Greater) => q_lo.add(Uint::ONE),
            Some(Ordering::Equal) => {
                let seed =
                    (p as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ uint_low_u64_fs(&q_lo);
                if RoundingMode::should_round_up::<RM>(
                    is_negative,
                    q_lo.is_odd(),
                    true,
                    false,
                    1u64 << 63,
                    seed,
                ) {
                    q_lo.add(Uint::ONE)
                } else {
                    q_lo
                }
            }
            None => approx_q,
        };

        let out_exp = p + (MB as i128) + 1;
        Self::pack_rounded(is_negative, is_subnormal, out_exp, final_q, bias, bias_i128)
    }

    /// Parses a base-10 floating-point string into `Float<W, MB, RM>`, rounding according to `RM`.
    ///
    /// Valid for all `W > 1` and `2 < MB < Self::BITS - 2`.
    pub fn from_str(src: &str) -> Result<Self, ParseFloatError> {
        let (dec, bias, bias_i128, is_subnormal, q_lo, p, approx_q, is_grid_boundary) =
            match Self::parse_stages_1_2::<false>(src)? {
                Stage12Outcome::Resolved(f) => return Ok(f),
                Stage12Outcome::NeedsStage3 {
                    dec,
                    bias,
                    bias_i128,
                    is_subnormal,
                    q_lo,
                    p,
                    approx_q,
                    is_grid_boundary,
                } => (
                    dec,
                    bias,
                    bias_i128,
                    is_subnormal,
                    q_lo,
                    p,
                    approx_q,
                    is_grid_boundary,
                ),
            };

        let cmp_target = if is_grid_boundary {
            q_lo.add(Uint::ONE).shl(1)
        } else {
            q_lo.shl(1).bitor(Uint::ONE)
        };
        let cmp_result = exact_cmp_midpoint::<W, MB, false>(&dec, &cmp_target, p);
        Ok(Self::finish_stage3(
            dec.is_negative,
            bias,
            bias_i128,
            is_subnormal,
            q_lo,
            p,
            approx_q,
            is_grid_boundary,
            cmp_result,
        ))
    }

    /// Const-compatible decimal float parser used by the [`f!`](crate::f) macro.
    pub(crate) const fn from_str_const<const ALLOW_UNDERSCORES: bool>(
        src: &str,
    ) -> Result<Self, ParseFloatError> {
        let (dec, bias, bias_i128, is_subnormal, q_lo, p, approx_q, is_grid_boundary) =
            match Self::parse_stages_1_2::<ALLOW_UNDERSCORES>(src) {
                Ok(Stage12Outcome::Resolved(f)) => return Ok(f),
                Ok(Stage12Outcome::NeedsStage3 {
                    dec,
                    bias,
                    bias_i128,
                    is_subnormal,
                    q_lo,
                    p,
                    approx_q,
                    is_grid_boundary,
                }) => (
                    dec,
                    bias,
                    bias_i128,
                    is_subnormal,
                    q_lo,
                    p,
                    approx_q,
                    is_grid_boundary,
                ),
                Err(e) => return Err(e),
            };

        let cmp_target = if is_grid_boundary {
            q_lo.add(Uint::ONE).shl(1)
        } else {
            q_lo.shl(1).bitor(Uint::ONE)
        };
        let cmp_result = exact_cmp_midpoint_const::<W, MB, ALLOW_UNDERSCORES>(&dec, &cmp_target, p);
        Ok(Self::finish_stage3(
            dec.is_negative,
            bias,
            bias_i128,
            is_subnormal,
            q_lo,
            p,
            approx_q,
            is_grid_boundary,
            cmp_result,
        ))
    }

    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ()> {
        if radix < 2 || radix > 36 {
            return Err(());
        }
        if !radix.is_power_of_two() {
            return Err(());
        }
        if src.is_empty() {
            return Err(());
        }
        let (is_negative, src) = match src.as_bytes()[0] {
            b'+' => (false, src.split_at(1).1),
            b'-' => (true, src.split_at(1).1),
            _ => (false, src),
        };
        if str_eq_case_insensitive(src, "inf") || str_eq_case_insensitive(src, "infinity") {
            return Ok(if is_negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            });
        }
        if str_eq_case_insensitive(src, "nan") {
            return Ok(if is_negative {
                Self::NAN
            } else {
                -Self::NAN
            });
        }

        todo!()
    }
}

impl<const W: usize, const MB: usize, const RM: u8> FromStr for Float<W, MB, RM> {
    type Err = ParseFloatError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Float::from_str(src)
    }
}

/// Computes `bias + delta` as `Uint<W>`, where `delta` is an unbiased exponent guaranteed to
/// produce a non-negative biased exponent `< 2^EXPONENT_BITS`.
#[inline]
const fn add_i128_to_uint<const W: usize>(bias: Uint<W>, delta: i128) -> Uint<W> {
    let abs_bytes = delta.unsigned_abs().to_le_bytes();
    let mut mag = Uint::<W>::ZERO;
    let mut i = 0;
    while i < W && i < 16 {
        mag.bytes[i] = abs_bytes[i];
        i += 1;
    }
    if delta >= 0 {
        bias.add(mag)
    } else {
        bias.sub(mag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_bignum;
    use crate::types::{F16, F80, F128, F256};
    #[cfg(feature = "alloc")]
    use crate::types::{F32, F64};

    #[test]
    fn test_str_eq_case_insensitive() {
        assert!(str_eq_case_insensitive("inf", "INF"));
        assert!(str_eq_case_insensitive("infinity", "INFINITY"));
        assert!(str_eq_case_insensitive("nan", "NAN"));
        assert!(str_eq_case_insensitive("NaN", "NAN"));
        assert!(str_eq_case_insensitive("Infinity", "INFINITy"));
        assert!(!str_eq_case_insensitive("inf", "infinity"));
        assert!(!str_eq_case_insensitive("inf", "nan"));
        assert!(!str_eq_case_insensitive("infinity", "nan"));
    }

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest as FromStr>::from_str,
            cases: [
                // Empty and malformed strings
                (""),
                ("+"),
                ("-"),
                ("."),
                ("+."),
                ("-."),
                (".e0"),
                (".e+1"),
                ("1e"),
                ("1e+"),
                ("1e-"),
                ("1E"),
                ("e1"),
                ("E-1"),
                (" 1"),
                ("1 "),
                ("1_0"),
                ("1.0_0"),
                ("1..0"),
                ("1.2.3"),
                ("++1"),
                ("+-1"),
                ("--1"),
                ("infin"),
                ("infinit"),
                ("nana"),
                ("nan(1)"),
                ("0e9999999999999999999999999999999999999999a"),
                ("1e9999999999999999999999999999999999999999a"),
                ("1e1.5"),
                ("1e1e1"),
                // Special values (note: NaN sign bits are verified below too)
                ("inf"),
                ("+inf"),
                ("-inf"),
                ("INF"),
                ("InF"),
                ("infinity"),
                ("+infinity"),
                ("-infinity"),
                ("INFINITY"),
                ("-InFiNiTy"),
                ("nan"),
                ("+nan"),
                ("-nan"),
                ("NaN"),
                ("-NaN"),
                ("NAN"),
                // Zeros & sign preservation
                ("0"),
                ("+0"),
                ("-0"),
                ("0.0"),
                ("-0.0"),
                ("+0.0"),
                ("0."),
                (".0"),
                ("-0."),
                ("-.0"),
                ("0e0"),
                ("-0e0"),
                ("0e999999999999999999999999999999999999999999"),
                ("-0e999999999999999999999999999999999999999999"),
                ("0e-999999999999999999999999999999999999999999"),
                ("-0e-999999999999999999999999999999999999999999"),
                ("000000000000000000000.0000000000000000000"),
                // Huge overflow / underflow
                ("1e999999999999999999999999999999999999999999"),
                ("-1e999999999999999999999999999999999999999999"),
                ("1e-999999999999999999999999999999999999999999"),
                ("-1e-999999999999999999999999999999999999999999"),
                // Standard & edge decimal formats
                ("1"),
                ("-1"),
                ("+1"),
                ("1."),
                (".5"),
                ("+.5"),
                ("-.5"),
                ("5.e2"),
                (".5e2"),
                ("1e+0"),
                ("1e-0"),
                ("1e0000000000000000000000000000000000000002"),
                ("0000000000000000000000000000000000000000.000000000000000000000000000000000000000001e42"),
                ("3.14159265358979323846264338327950288419716939937510"),
                ("-2.71828182845904523536028747135266249775724709369995"),
                // Subnormal and extreme f32/f64 values
                ("1e-45"),
                ("7.006492321624085e-46"),
                ("7.006492321624086e-46"),
                ("1.401298464324817e-45"),
                ("1.1754942106924411e-38"),
                ("1.1754943508222875e-38"),
                ("3.4028234663852886e38"),
                ("3.4028235677973366e38"),
                ("3.4028235677973367e38"),
                ("5e-324"),
                ("2.4703282292062327e-324"),
                ("2.4703282292062328e-324"),
                ("2.225073858507201e-308"),
                ("2.2250738585072014e-308"),
                ("1.7976931348623157e308"),
                ("1.7976931348623158e308"),
                ("1.7976931348623159e308"),
                // Midpoint ties-to-even near 1.0 and 0.5
                ("1.00000000000000011102230246251565404236316680908203125"),
                ("1.000000000000000111022302462515654042363166809082031250000000000000001"),
                ("1.00000000000000033306690738754696212708950042724609375"),
                ("1.000000000000000333066907387546962127089500427246093749999999999999999"),
                ("1.000000059604644775390625"),
                ("1.00000005960464477539062500000000000001"),
                ("1.000000178813934326171875"),
                ("1.00000017881393432617187499999999999999"),
                // Classic tricky dec2flt hard rounding inputs (from Rust core dec2flt tests)
                ("72057594037927936.0"),
                ("72057594037927944.0"),
                ("72057594037927944.000000000000000000001"),
                ("72057594037927952.0"),
                ("72057594037927960.0"),
                ("18014398509481984.0"),
                ("18014398509481986.0"),
                ("18014398509481986.000000000000000000001"),
                ("9007199254740992.5"),
                ("9007199254740993.5"),
                ("16777216.5"),
                ("16777217.5"),
                ("2.2250738585072011e-308"),
                ("2.2250738585072012e-308"),
                ("1e23"),
                ("1e24"),
                ("1e25"),
                ("1e-23"),
                ("9.999999999999999e22"),
                ("1.0000000000000001e23"),
                ("89884656743115795386465259539451236680898848947115328636715040578866337902750481566354238661203768010560056939935696678829394884407208311246423715319737062188883946712432742638151109800623047059726541476042502884419075341171231440736956555270413618581675255342293149119973622969239858152417678164812112068608.0")
            ]
        }

        #[cfg(feature = "alloc")]
        quickcheck::quickcheck! {
            fn quickcheck_roundtrip_to_string(f: FTestBase) -> bool {
                use alloc::string::ToString;
                use crate::test::TestConvert;

                let s = f.to_string();
                let actual = FTest::from_str(&s).unwrap();
                let expected = FTestBase::from_str(&s).unwrap();
                if f.is_nan() {
                    actual.is_nan() && actual.is_sign_negative() == expected.is_sign_negative()
                } else {
                    TestConvert::into(actual) == TestConvert::into(expected)
                }
            }
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn exact_f64_751_digit_subnormal_and_overflow_midpoints() {
        use alloc::string::String;
        use core::fmt::Write;

        // Helper: compute exact decimal string of `m_odd * 5^n` (for 2^-n * m_odd) or `m_odd * 2^p`.
        fn bigint_to_dec(limbs: &[u64]) -> String {
            let mut tmp = limbs.to_vec();
            while tmp.last() == Some(&0) {
                tmp.pop();
            }
            if tmp.is_empty() {
                return String::from("0");
            }
            let mut chunks = alloc::vec::Vec::new();
            while !tmp.is_empty() {
                let mut rem: u128 = 0;
                for d in tmp.iter_mut().rev() {
                    let cur = (rem << 64) | (*d as u128);
                    *d = (cur / 1_000_000_000) as u64;
                    rem = cur % 1_000_000_000;
                }
                chunks.push(rem as u32);
                while tmp.last() == Some(&0) {
                    tmp.pop();
                }
            }
            let mut out = String::new();
            let last = chunks.pop().unwrap();
            let _ = write!(out, "{last}");
            for c in chunks.into_iter().rev() {
                let _ = write!(out, "{c:09}");
            }
            out
        }

        // 1. Exact midpoint 2^-1075 = 5^1075 * 10^-1075 (between 0.0 and MIN_POSITIVE_SUBNORMAL).
        // Exact tie rounds to even (0.0); adding a tiny nonzero tail rounds UP to from_bits(1)!
        let mut buf = [0u64; 64];
        let mut len = 1;
        buf[0] = 1;
        slice_mul_pow5(&mut buf, &mut len, 1075);
        let digits = bigint_to_dec(&buf[..len]);
        let exact_str = alloc::format!("{digits}e-1075");
        let above_str = alloc::format!("{digits}0000000000000000000001e-1097");

        assert_eq!(
            F64::from_str(&exact_str).unwrap().to_bits(),
            F64::from(exact_str.parse::<f64>().unwrap()).to_bits()
        );
        assert_eq!(F64::from_str(&exact_str).unwrap().to_bits(), F64::ZERO.to_bits());
        assert_eq!(
            F64::from_str(&above_str).unwrap().to_bits(),
            F64::from(above_str.parse::<f64>().unwrap()).to_bits()
        );
        assert_eq!(
            F64::from_str(&above_str).unwrap().to_bits(),
            F64::MIN_POSITIVE_SUBNORMAL.to_bits()
        );

        // Also test odd lower neighbor: 3 * 2^-1075 (midpoint between 1 and 2 subnormal ULPs):
        // Exact tie rounds to even (2 = from_bits(2)); slightly below rounds DOWN to 1!
        let mut buf3 = [0u64; 64];
        let mut len3 = 1;
        buf3[0] = 3;
        slice_mul_pow5(&mut buf3, &mut len3, 1075);
        let d3 = bigint_to_dec(&buf3[..len3]);
        let exact3 = alloc::format!("{d3}e-1075");
        // Subtract 1 from the last digit (which is '5' -> '4') and append 30 nines
        let mut below3 = d3.clone();
        assert_eq!(below3.pop(), Some('5'));
        below3.push('4');
        for _ in 0..30 {
            below3.push('9');
        }
        below3.push_str("e-1105");
        assert_eq!(
            F64::from_str(&exact3).unwrap().to_bits(),
            F64::from(exact3.parse::<f64>().unwrap()).to_bits()
        );
        assert_eq!(
            F64::from_str(&exact3).unwrap().to_bits(),
            Uint::<8>::from_le_bytes(2u64.to_le_bytes())
        );
        assert_eq!(
            F64::from_str(&below3).unwrap().to_bits(),
            F64::from(below3.parse::<f64>().unwrap()).to_bits()
        );
        assert_eq!(
            F64::from_str(&below3).unwrap().to_bits(),
            Uint::<8>::from_le_bytes(1u64.to_le_bytes())
        );

        // 2. Exact overflow midpoint (2^54 - 1) * 2^970 (between f64::MAX and infinity).
        // Exact tie rounds to even (INFINITY); 1 less rounds to f64::MAX!
        let m_odd = Uint::<8>::from_le_bytes(((1u64 << 54) - 1).to_le_bytes());
        let mut buf_ovf = [0u64; 64];
        let len_ovf = slice_load_uint_shl(&m_odd, 970, &mut buf_ovf);
        let ovf_digits = bigint_to_dec(&buf_ovf[..len_ovf]);
        assert_eq!(
            F64::from_str(&ovf_digits).unwrap().to_bits(),
            F64::from(ovf_digits.parse::<f64>().unwrap()).to_bits()
        );
        assert_eq!(
            F64::from_str(&ovf_digits).unwrap().to_bits(),
            F64::INFINITY.to_bits()
        );
        // Subtract 1 from buf_ovf (even integer -> last bit 0, so borrow works easily)
        let mut buf_below = buf_ovf;
        let mut i = 0;
        while buf_below[i] == 0 {
            buf_below[i] = u64::MAX;
            i += 1;
        }
        buf_below[i] -= 1;
        let below_ovf_digits = bigint_to_dec(&buf_below[..len_ovf]);
        assert_eq!(
            F64::from_str(&below_ovf_digits).unwrap().to_bits(),
            F64::from(below_ovf_digits.parse::<f64>().unwrap()).to_bits()
        );
        assert_eq!(
            F64::from_str(&below_ovf_digits).unwrap().to_bits(),
            F64::MAX.to_bits()
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn exhaustive_f32_and_f64_random_bit_strings() {
        use alloc::string::ToString;

        let mut state = 0x9876_5432_10fe_dcba_u64;
        let mut next_u64 = || -> u64 {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };

        // Test thousands of random f32 and f64 values (including subnormals, extreme exponents,
        // and both Display and scientific {:e} formatting) for exact bit-for-bit parity.
        for i in 0..5_000 {
            let r = next_u64();

            let u32_bits = match i % 3 {
                0 => r as u32,
                1 => (r as u32) & 0x807f_ffff, // subnormals & zeros
                _ => (r as u32) | 0x7f00_0000, // near MAX / inf / NaN
            };
            let f32_v = f32::from_bits(u32_bits);
            if !f32_v.is_nan() {
                for s in [f32_v.to_string(), alloc::format!("{f32_v:e}")] {
                    let ours = F32::from_str(&s).unwrap();
                    let std_v = s.parse::<f32>().unwrap();
                    assert_eq!(
                        ours.to_bits(),
                        F32::from(std_v).to_bits(),
                        "F32 mismatch on {s:?}"
                    );
                }
            }

            let u64_bits = match i % 3 {
                0 => r,
                1 => r & 0x800f_ffff_ffff_ffff, // subnormals & zeros
                _ => r | 0x7fe0_0000_0000_0000, // near MAX / inf / NaN
            };
            let f64_v = f64::from_bits(u64_bits);
            if !f64_v.is_nan() {
                for s in [f64_v.to_string(), alloc::format!("{f64_v:e}")] {
                    let ours = F64::from_str(&s).unwrap();
                    let std_v = s.parse::<f64>().unwrap();
                    assert_eq!(
                        ours.to_bits(),
                        F64::from(std_v).to_bits(),
                        "F64 mismatch on {s:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn non_standard_and_large_float_formats() {
        // 1. Float<2, 13> (EXPONENT_BITS = 2, bias = 1, MIN_POSITIVE = 1.0, MAX = 4 - 2^-12)
        // Exact midpoint between MAX_SUBNORMAL (1 - 2^-13) and MIN_POSITIVE (1.0) is 1 - 2^-14 = 0.99993896484375
        type F2_13 = Float<2, 13>;
        assert_eq!(F2_13::from_str("1.0").unwrap().to_bits(), F2_13::MIN_POSITIVE.to_bits());
        assert_eq!(
            F2_13::from_str("0.99993896484375").unwrap().to_bits(),
            F2_13::MIN_POSITIVE.to_bits()
        ); // tie rounds to even (MIN_POSITIVE)
        assert_eq!(
            F2_13::from_str("0.999938964843749999999999").unwrap().to_bits(),
            F2_13::MAX_SUBNORMAL.to_bits()
        );
        // Half of MIN_POSITIVE_SUBNORMAL (2^-14 = 0.00006103515625)
        assert_eq!(
            F2_13::from_str("0.00006103515625").unwrap().to_bits(),
            F2_13::ZERO.to_bits()
        );
        assert_eq!(
            F2_13::from_str("0.00006103515625000000001").unwrap().to_bits(),
            F2_13::MIN_POSITIVE_SUBNORMAL.to_bits()
        );
        // Overflow at 4.0
        assert_eq!(F2_13::from_str("4.0").unwrap().to_bits(), F2_13::INFINITY.to_bits());
        assert_eq!(
            F2_13::from_str("-4.0").unwrap().to_bits(),
            F2_13::NEG_INFINITY.to_bits()
        );

        // 2. F16 (Float<2, 10>): exact subnormal midpoint 2^-25 = 0.0000000298023223876953125
        assert_eq!(F16::from_str("0.0000000298023223876953125").unwrap().to_bits(), F16::ZERO.to_bits());
        assert_eq!(
            F16::from_str("0.00000002980232238769531250001").unwrap().to_bits(),
            F16::MIN_POSITIVE_SUBNORMAL.to_bits()
        );
        assert_eq!(F16::from_str("65504").unwrap().to_bits(), F16::MAX.to_bits());
        assert_eq!(F16::from_str("65520").unwrap().to_bits(), F16::INFINITY.to_bits());
        assert_eq!(F16::from_str("65519.999999").unwrap().to_bits(), F16::MAX.to_bits());

        // 3. F80, F128, F256 basic exact values and extreme exponents
        assert_eq!(F80::from_str("0.5").unwrap().to_bits(), F80::HALF.to_bits());
        assert_eq!(F128::from_str("1.0").unwrap().to_bits(), F128::ONE.to_bits());
        assert_eq!(F128::from_str("-2.0").unwrap().to_bits(), F128::TWO.neg().to_bits());
        assert_eq!(F128::from_str("1e5000").unwrap().to_bits(), F128::INFINITY.to_bits());
        assert_eq!(F128::from_str("1e-5000").unwrap().to_bits(), F128::ZERO.to_bits());
        // F128 exact power of 2: 2^-10000 = 5^10000 * 10^-10000? Let's test 1e-4932 (normal) and 1e4932 (finite!)
        assert!(F128::from_str("1.189731495357231765085759326628007e4932").unwrap().is_finite());
        assert_eq!(
            F128::from_str("1.189731495357231765085759326628008e4932").unwrap().to_bits(),
            F128::INFINITY.to_bits()
        );
        assert_eq!(F256::from_str("0.25").unwrap().to_bits(), F256::QUARTER.to_bits());
        assert!(F256::from_str("1e78913").unwrap().is_finite());
        assert_eq!(F256::from_str("1e78914").unwrap().to_bits(), F256::INFINITY.to_bits());
        // F256::MIN_POSITIVE = 2^-262142 ≈ 2.482e-78913
        assert!(F256::from_str("3e-78913").unwrap().is_normal());
        assert!(F256::from_str("1e-78913").unwrap().is_subnormal());
        assert_eq!(F256::from_str("1e-79000").unwrap().to_bits(), F256::ZERO.to_bits());

        // 4. Huge EXPONENT_BITS >= 32: Float<8, 3> (EXPONENT_BITS = 60) and Float<16, 3> (EXPONENT_BITS = 124)
        type F8_3 = Float<8, 3>;
        let bias8 = Uint::<8>::power_of_two(59).sub(Uint::ONE);
        assert_eq!(F8_3::from_str("1.0").unwrap().to_bits(), bias8.shl(3));
        assert_eq!(
            F8_3::from_str("1.5").unwrap().to_bits(),
            bias8.shl(3).bitor(Uint::from_le_bytes(4u64.to_le_bytes()))
        );
        // 1e100000000000000000 (10^(10^17) < 2^(2^59)) is FINITE in Float<8, 3>!
        assert!(F8_3::from_str("1e100000000000000000").unwrap().is_finite());
        assert!(F8_3::from_str("1e100000000000000000").unwrap().is_normal());
        // 1e1000000000000000000 (10^(10^18) > 2^(2^59)) overflows to +inf!
        assert_eq!(
            F8_3::from_str("1e1000000000000000000").unwrap().to_bits(),
            F8_3::INFINITY.to_bits()
        );

        type F16_3 = Float<16, 3>;
        let bias16 = Uint::<16>::power_of_two(123).sub(Uint::ONE);
        assert_eq!(F16_3::from_str("1.0").unwrap().to_bits(), bias16.shl(3));
        assert!(F16_3::from_str("1e1000000000000000000000000000000000000").unwrap().is_finite());
        assert_eq!(
            F16_3::from_str("1e10000000000000000000000000000000000000").unwrap().to_bits(),
            F16_3::INFINITY.to_bits()
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn exact_f128_alloc_stage3_midpoint() {
        use alloc::string::String;
        use core::fmt::Write;

        // Test exact F128 subnormal midpoint 2^-16495 (requires heap-allocated Stage 3,
        // ~11,530 significant decimal digits!).
        let mut buf = alloc::vec![0u64; 700];
        let mut len = 1;
        buf[0] = 1;
        slice_mul_pow5(&mut buf, &mut len, 16495);

        let mut tmp = buf[..len].to_vec();
        let mut chunks = alloc::vec::Vec::new();
        while !tmp.is_empty() {
            let mut rem: u128 = 0;
            for d in tmp.iter_mut().rev() {
                let cur = (rem << 64) | (*d as u128);
                *d = (cur / 1_000_000_000) as u64;
                rem = cur % 1_000_000_000;
            }
            chunks.push(rem as u32);
            while tmp.last() == Some(&0) {
                tmp.pop();
            }
        }
        let mut digits = String::new();
        let last = chunks.pop().unwrap();
        let _ = write!(digits, "{last}");
        for c in chunks.into_iter().rev() {
            let _ = write!(digits, "{c:09}");
        }

        let exact_tie = alloc::format!("{digits}e-16495");
        let strictly_above = alloc::format!("{digits}00000001e-16503");

        assert_eq!(
            F128::from_str(&exact_tie).unwrap().to_bits(),
            F128::ZERO.to_bits()
        );
        assert_eq!(
            F128::from_str(&strictly_above).unwrap().to_bits(),
            F128::MIN_POSITIVE_SUBNORMAL.to_bits()
        );

        // Also exercise const Stage 3 buffer tier <= 1024 with from_str_const
        assert_eq!(
            F128::from_str_const::<false>(&exact_tie).unwrap().to_bits(),
            F128::ZERO.to_bits()
        );
        assert_eq!(
            F128::from_str_const::<false>(&strictly_above).unwrap().to_bits(),
            F128::MIN_POSITIVE_SUBNORMAL.to_bits()
        );
    }

    #[test]
    fn from_str_const_underscores_rounding_modes_and_radix() {
        const R_POS: u8 = RoundingMode::TowardPositive as u8;
        const R_NEG: u8 = RoundingMode::TowardNegative as u8;
        const R_ZERO: u8 = RoundingMode::TowardZero as u8;
        const R_FAITH: u8 = RoundingMode::Faithful as u8;
        const R_STOCH: u8 = RoundingMode::Stochastic as u8;

        // from_str_const with underscores (int, frac, exp, leading zeros, >19 digits, midpoint, errors)
        let a = Float::<8, 52>::from_str_const::<true>("0_0_1_234.5_6_7e+1_0").unwrap();
        assert_eq!(a.to_bits(), Float::<8, 52>::from_str("1234.567e10").unwrap().to_bits());
        let b = Float::<8, 52>::from_str_const::<true>("0.00_00_1_5").unwrap();
        assert_eq!(b.to_bits(), Float::<8, 52>::from_str("0.000015").unwrap().to_bits());
        let long_u = Float::<8, 52>::from_str_const::<true>(
            "1_234567890_1234567890_1234567890.123456789_0123456789_123456789"
        ).unwrap();
        assert_eq!(
            long_u.to_bits(),
            Float::<8, 52>::from_str("123456789012345678901234567890.1234567890123456789123456789").unwrap().to_bits()
        );
        assert!(Float::<8, 52>::from_str_const::<true>("").is_err());
        assert!(Float::<8, 52>::from_str_const::<true>("___").is_err());

        // Stage 3 midpoint with underscores (exact, less, greater)
        let tie_u = Float::<8, 52>::from_str_const::<true>(
            "1.000000000000000_11102230246251565404236316680908203125"
        ).unwrap();
        assert_eq!(tie_u.to_bits(), Float::<8, 52>::ONE.to_bits());
        let lt_u = Float::<8, 52>::from_str_const::<true>(
            "1.000000000000000_11102230246251565404236316680908203124999999999999"
        ).unwrap();
        assert_eq!(lt_u.to_bits(), Float::<8, 52>::ONE.to_bits());
        let gt_u = Float::<8, 52>::from_str_const::<true>(
            "1.000000000000000_11102230246251565404236316680908203125000000000001"
        ).unwrap();
        assert_eq!(gt_u.to_bits(), Float::<8, 52>::ONE.to_bits().add(Uint::ONE));

        // Grid boundaries in directed rounding modes (Equal, Less, Greater, overflow boundary)
        type F16p = Float<2, 10, R_POS>;
        type F16n = Float<2, 10, R_NEG>;
        type F16z = Float<2, 10, R_ZERO>;
        type F16f = Float<2, 10, R_FAITH>;
        type F16s = Float<2, 10, R_STOCH>;
        type F64s = Float<8, 52, R_STOCH>;
        type F128s = Float<16, 112, R_STOCH>;

        // Grid boundaries requiring Stage 3 (more than 100 trailing 0s/9s so Stage 2 is ambiguous)
        for s in [
            "1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
            "0.9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
            "-1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
            "-0.9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
            "65536.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "65536.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
            "65535.9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
            "-65535.9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999",
            "1e-100",
            "-1e-100",
            "2.98023223876953125e-8",  // 0.5 * F16::MIN_POSITIVE_SUBNORMAL
            "-2.98023223876953125e-8",
            "2.0e-8",                  // < 0.5 * F16::MIN_POSITIVE_SUBNORMAL
            "-2.0e-8",
            "4.0e-8",                  // in (0.5, 1.0) * F16::MIN_POSITIVE_SUBNORMAL
            "-4.0e-8",
        ] {
            let _ = F16p::from_str(s).unwrap();
            let _ = F16p::from_str_const::<false>(s).unwrap();
            let _ = F16n::from_str(s).unwrap();
            let _ = F16z::from_str(s).unwrap();
            let _ = F16f::from_str(s).unwrap();
            let _ = F16s::from_str(s).unwrap();
            let _ = F16::from_str(s).unwrap();
        }
        // Negative subnormal stochastic underflow (ensure both round-up and NEG_ZERO are exercised)
        for k in 10u32..40 {
            let _ = F16s::from_str(&alloc::format!("-{k}.1e-9")).unwrap();
        }
        // Formats with drop == 64 and drop < 64 in Stochastic to exercise fp_extract_top64 shift <= 64
        type F72m63s = Float<9, 63, R_STOCH>;
        type F80m70s = Float<10, 70, R_STOCH>;
        let _ = F72m63s::from_str("1.2345678901234567890123456789").unwrap();
        let _ = F80m70s::from_str("1.2345678901234567890123456789").unwrap();

        // exact_cmp_midpoint / exact_cmp_midpoint_const: p >= 0, tier <= 10_000, and abs_p > 1_000_000
        let dec_one = ParsedDecimal::<false> {
            is_negative: false,
            int_digits: b"1",
            frac_digits: b"",
            first_nz: 0,
            last_nz_end: 1,
            sig_digits: 1,
            exp10: 0,
        };
        let m_one = Uint::<8>::ONE;
        assert!(exact_cmp_midpoint::<8, 52, false>(&dec_one, &m_one, 1_000_001).is_none());
        assert!(exact_cmp_midpoint_const::<8, 52, false>(&dec_one, &m_one, 1_000_001).is_none());
        assert!(exact_cmp_midpoint_const::<8, 52, false>(&dec_one, &m_one, 80_000).is_some());
        assert!(exact_cmp_midpoint_const::<8, 52, false>(&dec_one, &m_one, 700_000).is_none());
        // Direct finish_stage3 None fallbacks (grid boundary and non-grid)
        let _ = F16::finish_stage3(false, Uint::from_le_bytes([15, 0]), Some(15), false, Uint::ONE, 0, Uint::ONE, true, None);
        let _ = F16::finish_stage3(false, Uint::from_le_bytes([15, 0]), Some(15), false, Uint::ONE, 0, Uint::ONE, false, None);

        // Stochastic with F64s and F128s (exercises fp_extract_top64 with shift <= 64, == 64, > 64)
        for s in ["1.234567890123456789", "3.1415926535897932384626433832795028841971", "1e-323", "-1e-323", "2.4703282292062327e-324"] {
            let _ = F64s::from_str(s).unwrap();
            let _ = F128s::from_str(s).unwrap();
        }

        // Exact huge-exponent format hitting abs_p > 1_000_000 fallback in Stage 3
        type HugeExpF = Float<8, 20>;
        let huge_s = "1.0000000000000000000000000000000000000000001e400000";
        assert!(HugeExpF::from_str(huge_s).unwrap().is_finite());
        assert!(HugeExpF::from_str_const::<false>(huge_s).unwrap().is_finite());

        // Const Stage 3 tier <= 10_000 (F256 subnormal-scale exponent ~ -70000 -> ~2500 limbs)
        let _ = F256::from_str_const::<false>("1.234567890123456789012345678901234567890123456789012345678901234567890123456e-70000").unwrap();

        // from_str_radix error and special value paths
        assert!(F16::from_str_radix("1", 1).is_err());
        assert!(F16::from_str_radix("1", 37).is_err());
        assert!(F16::from_str_radix("1", 10).is_err());
        assert!(F16::from_str_radix("", 16).is_err());
        assert_eq!(F16::from_str_radix("+inf", 16).unwrap().to_bits(), F16::INFINITY.to_bits());
        assert_eq!(F16::from_str_radix("-infinity", 16).unwrap().to_bits(), F16::NEG_INFINITY.to_bits());
        assert_eq!(F16::from_str_radix("inf", 2).unwrap().to_bits(), F16::INFINITY.to_bits());
        assert!(F16::from_str_radix("nan", 16).unwrap().is_nan());
        assert!(F16::from_str_radix("-NaN", 16).unwrap().is_nan());
    }
}
