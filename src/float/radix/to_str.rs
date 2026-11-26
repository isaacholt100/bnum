use core::cmp::Ordering;
use core::fmt::{self, Alignment, Debug, Formatter};

use super::from_str::{
    EXACT_STACK_LIMBS, Fixed, POW10_U64, fp_bit_width, fp_norm_mul, fp_pow5, fp_shl, slice_cmp,
    slice_load_uint_shl, slice_mul_add_u64, slice_mul_pow5,
};
#[cfg(feature = "alloc")]
use super::from_str::EXACT_HEAP_MAX_LIMBS;
use crate::{Exponent, Float, Uint};

/// `floor(log10(2) * 2^128)`, used to compute `floor(x * log10(2))` to within `±1` for all `i128`.
const LOG10_2_Q128: u128 = 0x4d10_4d42_7de7_fbcc_47c4_acd6_05be_48bc;

/// Stack digit buffer size for exact fixed-precision formatting (64 limbs ≈ 4096 bits <= 1234
/// decimal digits, covering 100% of `F16`, `F32`, and `F64` without heap allocation).
const EXACT_STACK_DIGITS: usize = EXACT_STACK_LIMBS * 20;

/// Stack-allocated ASCII digit buffer of size `3 * W + 8` bytes.
///
/// Since `MB < 8 * W`, the maximum number of significant digits in any shortest decimal
/// representation is `ceil((MB + 1) * log10(2)) + 2 < 2.41 * W + 3 < 3 * W + 8` for all `W >= 1`.
#[repr(C)]
struct SigBuf<const W: usize>([[u8; W]; 3], [u8; 8]);

impl<const W: usize> SigBuf<W> {
    #[inline]
    const fn new() -> Self {
        Self([[0u8; W]; 3], [0u8; 8])
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        const {
            assert!(core::mem::size_of::<Self>() == 3 * W + 8);
            assert!(core::mem::align_of::<Self>() == 1);
        }
        // SAFETY: `Self` is `#[repr(C)]` with align 1 and no padding, consisting solely of `u8`s.
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, 3 * W + 8) }
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        const {
            assert!(core::mem::size_of::<Self>() == 3 * W + 8);
            assert!(core::mem::align_of::<Self>() == 1);
        }
        // SAFETY: `Self` is `#[repr(C)]` with align 1 and no padding, consisting solely of `u8`s.
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut u8, 3 * W + 8) }
    }
}

/// Computes the high 128 bits of `a * b` (`floor((a * b) / 2^128)`).
#[inline]
const fn mul_u128_hi(a: u128, b: u128) -> u128 {
    let a_lo = a as u64 as u128;
    let a_hi = (a >> 64) as u64 as u128;
    let b_lo = b as u64 as u128;
    let b_hi = (b >> 64) as u64 as u128;

    let ll = a_lo * b_lo;
    let lh = a_lo * b_hi;
    let hl = a_hi * b_lo;
    let hh = a_hi * b_hi;

    let mid = (ll >> 64) + (lh & 0xffff_ffff_ffff_ffff) + (hl & 0xffff_ffff_ffff_ffff);
    hh + (lh >> 64) + (hl >> 64) + (mid >> 64)
}

/// Computes `floor(x * log10(2))` accurate to within `±1` for every `x: i128`.
#[inline]
const fn mul_log10_2_floor(x: i128) -> i128 {
    if x == 0 {
        return 0;
    }
    let hi = mul_u128_hi(x.unsigned_abs(), LOG10_2_Q128);
    if x > 0 {
        hi as i128
    } else {
        -(hi as i128) - 1
    }
}

/// Converts `u` to `u128`, returning `None` if `u >= 2^128`.
#[inline]
const fn uint_to_u128<const W: usize>(u: &Uint<W>) -> Option<u128> {
    let mut i = 16;
    while i < W {
        if u.bytes[i] != 0 {
            return None;
        }
        i += 1;
    }
    let mut buf = [0u8; 16];
    let limit = if W < 16 { W } else { 16 };
    let mut j = 0;
    while j < limit {
        buf[j] = u.bytes[j];
        j += 1;
    }
    Some(u128::from_le_bytes(buf))
}

/// Writes the decimal digits of `v` into the tail of `buf` and returns the populated slice.
#[inline]
fn u128_to_dec_bytes(mut v: u128, buf: &mut [u8; 40]) -> &[u8] {
    let mut pos = buf.len();
    if v == 0 {
        pos -= 1;
        buf[pos] = b'0';
        return &buf[pos..];
    }
    while v > 0 {
        pos -= 1;
        buf[pos] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    &buf[pos..]
}

// -----------------------------------------------------------------------------
// Formatting & padding primitives (matching `core::fmt::Formatter::pad_formatted_parts`)
// -----------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Part<'a> {
    Bytes(&'a [u8]),
    Zeros(usize),
}

impl<'a> Part<'a> {
    #[inline]
    const fn len(&self) -> usize {
        match *self {
            Part::Bytes(b) => b.len(),
            Part::Zeros(n) => n,
        }
    }
}

fn write_zeros(f: &mut Formatter<'_>, mut count: usize) -> fmt::Result {
    const ZEROS: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    while count >= ZEROS.len() {
        f.write_str(ZEROS)?;
        count -= ZEROS.len();
    }
    if count > 0 {
        f.write_str(&ZEROS[..count])?;
    }
    Ok(())
}

fn write_fill(f: &mut Formatter<'_>, fill: char, count: usize) -> fmt::Result {
    if count == 0 {
        return Ok(());
    }
    if fill == ' ' {
        const SPACES: &str = "                                                                ";
        let mut rem = count;
        while rem >= SPACES.len() {
            f.write_str(SPACES)?;
            rem -= SPACES.len();
        }
        if rem > 0 {
            f.write_str(&SPACES[..rem])?;
        }
        return Ok(());
    }
    if fill == '0' {
        return write_zeros(f, count);
    }
    let mut buf = [0u8; 4];
    let s = fill.encode_utf8(&mut buf);
    for _ in 0..count {
        f.write_str(s)?;
    }
    Ok(())
}

fn write_parts(f: &mut Formatter<'_>, parts: &[Part<'_>]) -> fmt::Result {
    for part in parts {
        match *part {
            Part::Bytes(b) => {
                // SAFETY: all formatted float bytes are ASCII.
                let s = unsafe { core::str::from_utf8_unchecked(b) };
                f.write_str(s)?;
            }
            Part::Zeros(n) => write_zeros(f, n)?,
        }
    }
    Ok(())
}

/// Applies `Formatter` width, alignment, fill, and sign-aware zero padding to `(sign, parts)`,
/// matching Rust's `core::fmt::Formatter::pad_formatted_parts` used by `f32`/`f64`.
fn pad_and_write(f: &mut Formatter<'_>, sign: &'static str, parts: &[Part<'_>]) -> fmt::Result {
    let body_len = parts
        .iter()
        .fold(0usize, |acc, p| acc.saturating_add(p.len()));
    let total_len = sign.len().saturating_add(body_len);

    let Some(width) = f.width() else {
        if !sign.is_empty() {
            f.write_str(sign)?;
        }
        return write_parts(f, parts);
    };

    if width <= total_len {
        if !sign.is_empty() {
            f.write_str(sign)?;
        }
        return write_parts(f, parts);
    }

    let pad = width - total_len;
    if f.sign_aware_zero_pad() {
        if !sign.is_empty() {
            f.write_str(sign)?;
        }
        write_zeros(f, pad)?;
        write_parts(f, parts)
    } else {
        let fill = f.fill();
        let align = f.align().unwrap_or(Alignment::Right);
        let (pad_left, pad_right) = match align {
            Alignment::Left => (0, pad),
            Alignment::Right => (pad, 0),
            Alignment::Center => (pad / 2, pad - pad / 2),
        };
        write_fill(f, fill, pad_left)?;
        if !sign.is_empty() {
            f.write_str(sign)?;
        }
        write_parts(f, parts)?;
        write_fill(f, fill, pad_right)
    }
}

#[inline]
fn determine_sign(is_negative: bool, force_plus: bool) -> &'static str {
    if is_negative {
        "-"
    } else if force_plus {
        "+"
    } else {
        ""
    }
}

fn fmt_zero(f: &mut Formatter<'_>, sign: &'static str, frac_digits: usize) -> fmt::Result {
    if frac_digits == 0 {
        pad_and_write(f, sign, &[Part::Bytes(b"0")])
    } else {
        pad_and_write(f, sign, &[Part::Bytes(b"0."), Part::Zeros(frac_digits)])
    }
}

/// Formats shortest decimal digits `0.<digits> * 10^exp_k` according to Rust's `Debug` rules:
/// - Scientific notation (`1e16`, `1.5e-5`, no `.0` for single digit) when `vis_exp < -4` or
///   `vis_exp >= 16` (where `vis_exp = exp_k - 1`).
/// - Fixed decimal notation with at least 1 fractional digit (`1.0`, `0.0001`) otherwise.
fn fmt_shortest_digits(
    f: &mut Formatter<'_>,
    sign: &'static str,
    digits: &[u8],
    exp_k: i128,
) -> fmt::Result {
    debug_assert!(!digits.is_empty());
    debug_assert!(digits[0] > b'0');

    let vis_exp = exp_k - 1;
    if vis_exp < -4 || vis_exp >= 16 {
        let mut exp_buf = [0u8; 40];
        let exp_bytes = u128_to_dec_bytes(vis_exp.unsigned_abs(), &mut exp_buf);
        let exp_prefix: &[u8] = if vis_exp < 0 { b"e-" } else { b"e" };
        if digits.len() == 1 {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(digits),
                    Part::Bytes(exp_prefix),
                    Part::Bytes(exp_bytes),
                ],
            )
        } else {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(&digits[..1]),
                    Part::Bytes(b"."),
                    Part::Bytes(&digits[1..]),
                    Part::Bytes(exp_prefix),
                    Part::Bytes(exp_bytes),
                ],
            )
        }
    } else if exp_k <= 0 {
        let leading_zeros = (-exp_k) as usize;
        pad_and_write(
            f,
            sign,
            &[
                Part::Bytes(b"0."),
                Part::Zeros(leading_zeros),
                Part::Bytes(digits),
            ],
        )
    } else {
        let exp_u = exp_k as usize;
        if exp_u < digits.len() {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(&digits[..exp_u]),
                    Part::Bytes(b"."),
                    Part::Bytes(&digits[exp_u..]),
                ],
            )
        } else {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(digits),
                    Part::Zeros(exp_u - digits.len()),
                    Part::Bytes(b".0"),
                ],
            )
        }
    }
}

/// Formats decimal integer digits `digits` with the decimal point placed `scale_right` digits
/// from the right, followed by `trailing_zeros` additional fractional zero digits.
fn fmt_fixed_scaled_digits(
    f: &mut Formatter<'_>,
    sign: &'static str,
    digits: &[u8],
    scale_right: usize,
    trailing_zeros: usize,
) -> fmt::Result {
    debug_assert!(!digits.is_empty());
    let total_frac = scale_right + trailing_zeros;
    if total_frac == 0 {
        return pad_and_write(f, sign, &[Part::Bytes(digits)]);
    }
    if scale_right == 0 {
        return pad_and_write(
            f,
            sign,
            &[
                Part::Bytes(digits),
                Part::Bytes(b"."),
                Part::Zeros(trailing_zeros),
            ],
        );
    }
    if digits.len() > scale_right {
        let split = digits.len() - scale_right;
        if trailing_zeros == 0 {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(&digits[..split]),
                    Part::Bytes(b"."),
                    Part::Bytes(&digits[split..]),
                ],
            )
        } else {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(&digits[..split]),
                    Part::Bytes(b"."),
                    Part::Bytes(&digits[split..]),
                    Part::Zeros(trailing_zeros),
                ],
            )
        }
    } else {
        let leading_zeros = scale_right - digits.len();
        if trailing_zeros == 0 {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(b"0."),
                    Part::Zeros(leading_zeros),
                    Part::Bytes(digits),
                ],
            )
        } else {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(b"0."),
                    Part::Zeros(leading_zeros),
                    Part::Bytes(digits),
                    Part::Zeros(trailing_zeros),
                ],
            )
        }
    }
}

// -----------------------------------------------------------------------------
// Additional `Fixed<W>` primitives for Stage 1 / Stage 2 shortest scaling
// -----------------------------------------------------------------------------

#[inline]
fn fp_shr<const W: usize>(a: &Fixed<W>, shift: Exponent, limbs: usize) -> Fixed<W> {
    if shift == 0 {
        return *a;
    }
    let mut out = Fixed::<W>::ALL_ZEROS;
    let word_shift = (shift / 64) as usize;
    let bit_shift = shift % 64;
    if word_shift >= limbs {
        return out;
    }
    let limit = limbs - word_shift;
    for i in 0..limit {
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
    }
    out
}

#[inline]
fn fp_add<const W: usize>(a: &Fixed<W>, b: &Fixed<W>, limbs: usize) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut carry = 0u128;
    for i in 0..limbs {
        let sum = (a.get(i) as u128) + (b.get(i) as u128) + carry;
        out.set(i, sum as u64);
        carry = sum >> 64;
    }
    debug_assert!(carry == 0);
    out
}

#[inline]
fn fp_sub<const W: usize>(a: &Fixed<W>, b: &Fixed<W>, limbs: usize) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let mut borrow = 0u64;
    for i in 0..limbs {
        let ai = a.get(i);
        let bi = b.get(i);
        let (d1, b1) = ai.overflowing_sub(bi);
        let (d2, b2) = d1.overflowing_sub(borrow);
        out.set(i, d2);
        borrow = (b1 || b2) as u64;
    }
    debug_assert!(borrow == 0);
    out
}

#[inline]
fn fp_add_pow2<const W: usize>(a: &Fixed<W>, bit_idx: Exponent, limbs: usize) -> Fixed<W> {
    let mut out = *a;
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
    debug_assert!(!c);
    out
}

#[inline]
fn fp_sub_pow2<const W: usize>(a: &Fixed<W>, bit_idx: Exponent, limbs: usize) -> Fixed<W> {
    let mut out = *a;
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
    debug_assert!(!b);
    out
}

#[inline]
fn fp_add_u64<const W: usize>(a: &Fixed<W>, rhs: u64, limbs: usize) -> Fixed<W> {
    let mut out = *a;
    if rhs == 0 || limbs == 0 {
        return out;
    }
    let (s, mut c) = out.get(0).overflowing_add(rhs);
    out.set(0, s);
    let mut i = 1;
    while c && i < limbs {
        let (s2, c2) = out.get(i).overflowing_add(1);
        out.set(i, s2);
        c = c2;
        i += 1;
    }
    debug_assert!(!c);
    out
}

#[inline]
fn fp_bit<const W: usize>(a: &Fixed<W>, bit_idx: Exponent) -> bool {
    let word_idx = (bit_idx / 64) as usize;
    let bit_off = bit_idx % 64;
    ((a.get(word_idx) >> bit_off) & 1) != 0
}

#[inline]
fn fp_cmp<const W: usize>(a: &Fixed<W>, b: &Fixed<W>, limbs: usize) -> Ordering {
    for i in (0..limbs).rev() {
        let ai = a.get(i);
        let bi = b.get(i);
        if ai != bi {
            return ai.cmp(&bi);
        }
    }
    Ordering::Equal
}

#[inline]
fn fp_clamp<const W: usize>(
    val: Fixed<W>,
    lo: &Fixed<W>,
    hi: &Fixed<W>,
    limbs: usize,
) -> Fixed<W> {
    if fp_cmp(&val, lo, limbs) == Ordering::Less {
        *lo
    } else if fp_cmp(&val, hi, limbs) == Ordering::Greater {
        *hi
    } else {
        val
    }
}

/// Divides `a[0..limbs]` by `div` in-place, returning the remainder.
#[inline]
fn fp_div_rem_u64<const W: usize>(a: &mut Fixed<W>, div: u64, limbs: usize) -> u64 {
    let div128 = div as u128;
    let mut rem = 0u128;
    for i in (0..limbs).rev() {
        let d = a.get(i);
        if rem == 0 && d == 0 {
            continue;
        }
        let cur = (rem << 64) | (d as u128);
        a.set(i, (cur / div128) as u64);
        rem = cur % div128;
    }
    rem as u64
}

/// Loads `u << shift` (for `shift <= 2`) into `Fixed<W>`.
#[inline]
fn fp_load_uint_shl<const W: usize>(u: &Uint<W>, shift: u32, limbs: usize) -> Fixed<W> {
    let mut out = Fixed::<W>::ALL_ZEROS;
    let u_digits = u.as_digits::<u64>();
    let u_limbs = W.div_ceil(8).min(limbs);
    let mut carry = 0u64;
    for i in 0..u_limbs {
        let d = u_digits.get(i);
        if shift == 0 {
            out.set(i, d);
        } else {
            out.set(i, (d << shift) | carry);
            carry = d >> (64 - shift);
        }
    }
    if carry != 0 && u_limbs < limbs {
        out.set(u_limbs, carry);
    }
    out
}

/// Finds the largest `r >= 0` such that `[a, b]` contains a multiple of `10^r`, returning
/// `(r, ceil(a / 10^r), floor(b / 10^r))`.
fn reduce_interval_by_10<const W: usize>(
    mut a: Fixed<W>,
    mut b: Fixed<W>,
    int_limbs: usize,
) -> (u32, Fixed<W>, Fixed<W>) {
    let mut active = int_limbs;
    while active > 0 && b.get(active - 1) == 0 {
        active -= 1;
    }
    let mut r = 0u32;
    loop {
        let mut a_next = a;
        let rem_a = fp_div_rem_u64(&mut a_next, 10, active);
        if rem_a != 0 {
            a_next = fp_add_u64(&a_next, 1, active);
        }
        let mut b_next = b;
        let _ = fp_div_rem_u64(&mut b_next, 10, active);
        if fp_cmp(&a_next, &b_next, active) == Ordering::Greater {
            break;
        }
        a = a_next;
        b = b_next;
        r += 1;
        while active > 0 && b.get(active - 1) == 0 {
            active -= 1;
        }
    }
    (r, a, b)
}

/// Computes `floor(val / 10^r + 0.5)` for `r >= 1`, returning `floor(val / 10^r) + (last_digit >= 5)`.
fn fp_div_pow10_round_half_up<const W: usize>(
    mut val: Fixed<W>,
    r: u32,
    int_limbs: usize,
) -> Fixed<W> {
    debug_assert!(r >= 1);
    let mut rem_steps = r as usize;
    while rem_steps > 19 {
        let _ = fp_div_rem_u64(&mut val, POW10_U64[19], int_limbs);
        rem_steps -= 19;
    }
    if rem_steps > 1 {
        let _ = fp_div_rem_u64(&mut val, POW10_U64[rem_steps - 1], int_limbs);
    }
    let last_digit = fp_div_rem_u64(&mut val, 10, int_limbs);
    if last_digit >= 5 {
        fp_add_u64(&val, 1, int_limbs)
    } else {
        val
    }
}

/// Converts a positive integer in `val[0..int_limbs]` to ASCII decimal digits inside `out`,
/// returning the starting index in `out` (`&out[start..]`).
fn fp_int_to_dec_digits<'a, const W: usize>(
    mut val: Fixed<W>,
    mut int_limbs: usize,
    out: &'a mut [u8],
) -> &'a [u8] {
    while int_limbs > 0 && val.get(int_limbs - 1) == 0 {
        int_limbs -= 1;
    }
    let mut pos = out.len();
    if int_limbs == 0 {
        pos -= 1;
        out[pos] = b'0';
        return &out[pos..];
    }
    const BASE: u64 = POW10_U64[19];
    while int_limbs > 1 {
        let mut chunk = fp_div_rem_u64(&mut val, BASE, int_limbs);
        while int_limbs > 0 && val.get(int_limbs - 1) == 0 {
            int_limbs -= 1;
        }
        for _ in 0..19 {
            pos -= 1;
            out[pos] = b'0' + (chunk % 10) as u8;
            chunk /= 10;
        }
    }
    if int_limbs == 1 {
        let mut v = val.get(0);
        while v > 0 {
            pos -= 1;
            out[pos] = b'0' + (v % 10) as u8;
            v /= 10;
        }
    }
    &out[pos..]
}

// -----------------------------------------------------------------------------
// Stage 1 / Stage 2: Fast rigorous fixed-point shortest decimal finder
// -----------------------------------------------------------------------------

enum ShortestOutcome<const W: usize> {
    Resolved { q: Fixed<W>, int_limbs: usize, p: i128 },
    Ambiguous { approx_q: Fixed<W>, int_limbs: usize, p: i128 },
}

/// Attempts to find the provably exact shortest decimal representation `Q * 10^p` for
/// `v = mant * 2^exp2`, `low = (mant - 1) * 2^exp2`, `high = (mant + plus) * 2^exp2`
/// at `limbs` 64-bit limbs of fixed-point precision.
fn shortest_at_precision<const W: usize, const MB: usize>(
    m: &Uint<W>,
    lower_half_spacing: bool,
    exp2: i128,
    limbs: usize,
) -> ShortestOutcome<W> {
    let p_bits = (limbs as Exponent) * 64;
    let mant_shift = if lower_half_spacing { 2u32 } else { 1u32 };
    let mant_fp = fp_load_uint_shl(m, mant_shift, limbs);

    // Choose decimal scaling exponent `q` so that `(high - low) * 10^(-q) in [10, 15000)`
    // and `high * 10^(-q) < 2^(MB + 16)`.
    let k_est = mul_log10_2_floor(exp2 + 1);
    let q = k_est - 2;

    // Compute normalized `5^(-q)` and scale `v`, `delta_minus`, `delta_plus`.
    let (p5_norm, p5_exp2) = fp_pow5::<W>(-q, limbs);
    let unit_exp2 = p5_exp2 + exp2 - q;

    let m_bw = fp_bit_width(&mant_fp, limbs);
    let m_norm = fp_shl(&mant_fp, p_bits - m_bw, limbs);
    let (v_norm, v_mul_exp) = fp_norm_mul(&m_norm, m_bw as i128, &p5_norm, unit_exp2, limbs);

    // Align `V`, `delta_minus`, `delta_plus` at a common binary point with 2 extra top bits so
    // `H + err < 0.5 * 2^P` can never overflow `Fixed<W>`.
    let int_bits = (v_mul_exp + 2) as Exponent;
    let int_limbs = (int_bits as usize).div_ceil(64).min(limbs);

    let abs_q = (-q).unsigned_abs();
    let q_bits: Exponent = if abs_q == 0 {
        0
    } else {
        128 - abs_q.leading_zeros()
    };
    let limb_bits: Exponent = 64 - (limbs as u64).leading_zeros();
    let err_bits: Exponent = q_bits + limb_bits + 8;

    if int_bits >= p_bits || err_bits + 4 >= p_bits - int_bits {
        // Not enough fractional bits at this precision; compute a best-effort approx and retry.
        let frac_bits = p_bits.saturating_sub(int_bits);
        let v_fp = fp_shr(&v_norm, 2, limbs);
        let approx_q = fp_shr(&v_fp, frac_bits, limbs);
        return ShortestOutcome::Ambiguous {
            approx_q,
            int_limbs,
            p: q,
        };
    }

    let frac_bits = p_bits - int_bits;
    let v_fp = fp_shr(&v_norm, 2, limbs);

    let shift_d = (int_bits as i128 - unit_exp2) as Exponent;
    let delta_minus_fp = fp_shr(&p5_norm, shift_d, limbs);
    let delta_plus_fp = if lower_half_spacing {
        fp_shr(&p5_norm, shift_d - 1, limbs)
    } else {
        delta_minus_fp
    };

    let l_fp = fp_sub(&v_fp, &delta_minus_fp, limbs);
    let h_fp = fp_add(&v_fp, &delta_plus_fp, limbs);

    // Rigorous strict enclosures: L_lo < L < L_hi, V_lo < V < V_hi, H_lo < H < H_hi.
    let l_lo = fp_sub_pow2(&l_fp, err_bits, limbs);
    let l_hi = fp_add_pow2(&l_fp, err_bits, limbs);
    let v_lo = fp_sub_pow2(&v_fp, err_bits, limbs);
    let v_hi = fp_add_pow2(&v_fp, err_bits, limbs);
    let h_lo = fp_sub_pow2(&h_fp, err_bits, limbs);
    let h_hi = fp_add_pow2(&h_fp, err_bits, limbs);

    let a_in = fp_add_u64(&fp_shr(&l_hi, frac_bits, limbs), 1, int_limbs);
    let a_out = fp_add_u64(&fp_shr(&l_lo, frac_bits, limbs), 1, int_limbs);
    let b_in = fp_shr(&h_lo, frac_bits, limbs);
    let b_out = fp_shr(&h_hi, frac_bits, limbs);

    let (r_in, big_a_in, big_b_in) = reduce_interval_by_10(a_in, b_in, int_limbs);
    let (r_out, big_a_out, big_b_out) = reduce_interval_by_10(a_out, b_out, int_limbs);

    let v_lo_int = fp_shr(&v_lo, frac_bits, limbs);
    let v_hi_int = fp_shr(&v_hi, frac_bits, limbs);

    if r_in != r_out {
        let approx_q = if r_in == 0 {
            fp_clamp(v_hi_int, &big_a_in, &big_b_in, int_limbs)
        } else {
            let q_est = fp_div_pow10_round_half_up(v_hi_int, r_in, int_limbs);
            fp_clamp(q_est, &big_a_in, &big_b_in, int_limbs)
        };
        return ShortestOutcome::Ambiguous {
            approx_q,
            int_limbs,
            p: q + (r_in as i128),
        };
    }

    let r = r_in;
    let (q_unclamped_lo, q_unclamped_hi) = if r == 0 {
        let lo = if fp_bit(&v_lo, frac_bits - 1) {
            fp_add_u64(&v_lo_int, 1, int_limbs)
        } else {
            v_lo_int
        };
        let hi = if fp_bit(&v_hi, frac_bits - 1) {
            fp_add_u64(&v_hi_int, 1, int_limbs)
        } else {
            v_hi_int
        };
        (lo, hi)
    } else {
        (
            fp_div_pow10_round_half_up(v_lo_int, r, int_limbs),
            fp_div_pow10_round_half_up(v_hi_int, r, int_limbs),
        )
    };

    // By monotonicity of clamp(x, lo, hi) in all three arguments:
    //   clamp(q_unclamped_lo, A_out, B_in) <= Q_true <= clamp(q_unclamped_hi, A_in, B_out).
    let q_min = fp_clamp(q_unclamped_lo, &big_a_out, &big_b_in, int_limbs);
    let q_max = fp_clamp(q_unclamped_hi, &big_a_in, &big_b_out, int_limbs);

    if fp_cmp(&q_min, &q_max, int_limbs) == Ordering::Equal {
        ShortestOutcome::Resolved {
            q: q_min,
            int_limbs,
            p: q + (r as i128),
        }
    } else {
        ShortestOutcome::Ambiguous {
            approx_q: q_max,
            int_limbs,
            p: q + (r as i128),
        }
    }
}

// -----------------------------------------------------------------------------
// Stage 3: Exact Burger-Dybvig Dragon4 for shortest mode (ties away from zero)
// -----------------------------------------------------------------------------

#[inline]
fn lt_rounding(ord: Ordering, inclusive: bool) -> bool {
    if inclusive {
        ord != Ordering::Greater
    } else {
        ord == Ordering::Less
    }
}

/// Compares `a` against `b + (c << c_shift)` (where `c_shift` is 0 or 1) in O(n) time and O(1) space.
fn slice_cmp_sum(a: &[u64], b: &[u64], c: &[u64], c_shift: u32) -> Ordering {
    let max_len = a.len().max(b.len()).max(c.len() + (c_shift as usize));
    let mut carry: u128 = 0;
    let mut c_prev_hi: u64 = 0;
    let mut ord = Ordering::Equal;

    for i in 0..max_len {
        let ai = if i < a.len() { a[i] } else { 0 };
        let bi = if i < b.len() { b[i] } else { 0 };
        let ci_raw = if i < c.len() { c[i] } else { 0 };
        let ci = if c_shift == 0 {
            ci_raw
        } else {
            let shifted = (ci_raw << 1) | c_prev_hi;
            c_prev_hi = ci_raw >> 63;
            shifted
        };
        let sum = (bi as u128) + (ci as u128) + carry;
        let si = sum as u64;
        carry = sum >> 64;
        if ai != si {
            ord = ai.cmp(&si);
        }
    }

    if carry != 0 {
        Ordering::Less
    } else {
        ord
    }
}

/// Compares `a << 1` against `b` in O(n) time and O(1) space.
fn slice_cmp_shl1(a: &[u64], b: &[u64]) -> Ordering {
    let max_len = (a.len() + 1).max(b.len());
    let mut a_prev_hi = 0u64;
    let mut ord = Ordering::Equal;

    for i in 0..max_len {
        let ai_raw = if i < a.len() { a[i] } else { 0 };
        let ai = (ai_raw << 1) | a_prev_hi;
        a_prev_hi = ai_raw >> 63;
        let bi = if i < b.len() { b[i] } else { 0 };
        if ai != bi {
            ord = ai.cmp(&bi);
        }
    }
    ord
}

/// Subtracts `q * s[..s_len]` from `r[..*r_len]` in-place (caller guarantees `r >= q * s`).
fn slice_sub_mul_u64(r: &mut [u64], r_len: &mut usize, s: &[u64], q: u64) {
    if q == 0 {
        return;
    }
    let q128 = q as u128;
    let mut mul_carry = 0u128;
    let mut borrow = 0u64;
    let s_len = s.len();
    for i in 0..s_len {
        let prod = (s[i] as u128) * q128 + mul_carry;
        let p_lo = prod as u64;
        mul_carry = prod >> 64;

        let ri = r[i];
        let (d1, b1) = ri.overflowing_sub(p_lo);
        let (d2, b2) = d1.overflowing_sub(borrow);
        r[i] = d2;
        borrow = (b1 || b2) as u64;
    }
    let mut idx = s_len;
    let mut rem_sub = (mul_carry as u64) + borrow;
    while rem_sub != 0 && idx < *r_len {
        let (d, b) = r[idx].overflowing_sub(rem_sub);
        r[idx] = d;
        rem_sub = b as u64;
        idx += 1;
    }
    debug_assert!(rem_sub == 0);
    while *r_len > 0 && r[*r_len - 1] == 0 {
        *r_len -= 1;
    }
}

/// Computes `d = floor(r / s)` and replaces `r` with `r % s`, assuming `0 <= r < 10 * s` and
/// `s` is normalized (`!s.is_empty()` and `*s.last().unwrap() != 0`).
fn slice_div_rem_digit(r: &mut [u64], r_len: &mut usize, s: &[u64]) -> u8 {
    let s_len = s.len();
    debug_assert!(s_len > 0 && s[s_len - 1] != 0);
    if *r_len < s_len {
        return 0;
    }
    if s_len == 1 {
        let s0 = s[0] as u128;
        let r_val = (r[0] as u128) | if *r_len > 1 { (r[1] as u128) << 64 } else { 0 };
        let q = (r_val / s0) as u8;
        let rem = (r_val % s0) as u64;
        r[0] = rem;
        if *r_len > 1 {
            r[1] = 0;
        }
        *r_len = if rem == 0 { 0 } else { 1 };
        return q;
    }

    let s_top = s[s_len - 1];
    let lz = s_top.leading_zeros();
    let s_hi64 = if lz == 0 {
        s_top
    } else {
        (s_top << lz) | (s[s_len - 2] >> (64 - lz))
    };

    let r_top2 = if *r_len > s_len { r[s_len] } else { 0 };
    let r_top1 = r[s_len - 1];
    let r_top0 = r[s_len - 2];
    let (hi_word, lo_word) = if lz == 0 {
        (r_top2, r_top1)
    } else {
        (
            (r_top2 << lz) | (r_top1 >> (64 - lz)),
            (r_top1 << lz) | (r_top0 >> (64 - lz)),
        )
    };
    let r_hi128 = ((hi_word as u128) << 64) | (lo_word as u128);
    // `q <= floor(r / s)` is guaranteed (never overestimates), and underestimates by at most 1.
    let mut q = (r_hi128 / ((s_hi64 as u128) + 1)) as u64;
    if q > 0 {
        slice_sub_mul_u64(r, r_len, s, q);
    }
    while slice_cmp(r, *r_len, s, s_len) != Ordering::Less {
        slice_sub_mul_u64(r, r_len, s, 1);
        q += 1;
    }
    debug_assert!(q < 10);
    q as u8
}

/// Increments ASCII decimal digits `buf` by 1, returning `Some(extra_digit)` if length grew.
fn round_up_digits(buf: &mut [u8]) -> Option<u8> {
    match buf.iter().rposition(|&c| c != b'9') {
        Some(i) => {
            buf[i] += 1;
            buf[i + 1..].fill(b'0');
            None
        }
        None if buf.is_empty() => Some(b'1'),
        None => {
            buf[0] = b'1';
            buf[1..].fill(b'0');
            Some(b'0')
        }
    }
}

/// Exact Burger-Dybvig Dragon4 shortest digit generator using provided zero-initialized buffers.
/// Matches Rust's `core::num::imp::flt2dec::strategy::dragon::format_shortest` (ties round up).
fn dragon4_shortest_in_bufs<const W: usize>(
    m: &Uint<W>,
    lower_half_spacing: bool,
    exp2: i128,
    inclusive: bool,
    r_buf: &mut [u64],
    minus_buf: &mut [u64],
    s_buf: &mut [u64],
    out_digits: &mut [u8],
) -> (usize, i128) {
    let mant_shift = if lower_half_spacing { 2usize } else { 1usize };
    let plus_shift = if lower_half_spacing { 1u32 } else { 0u32 };
    let mant_bw = (m.bit_width() as i128) + (mant_shift as i128);
    let mut k = mul_log10_2_floor(mant_bw - 1 + exp2) + 1;

    let p2 = exp2 - k;
    let p5 = -k;

    let p2_pos = p2.max(0) as usize;
    let p2_neg = (-p2).max(0) as usize;

    let mut r_len = slice_load_uint_shl(m, mant_shift + p2_pos, r_buf);
    minus_buf[p2_pos / 64] = 1u64 << (p2_pos % 64);
    let mut minus_len = p2_pos / 64 + 1;
    s_buf[p2_neg / 64] = 1u64 << (p2_neg % 64);
    let mut s_len = p2_neg / 64 + 1;

    if p5 > 0 {
        slice_mul_pow5(r_buf, &mut r_len, p5 as u128);
        slice_mul_pow5(minus_buf, &mut minus_len, p5 as u128);
    } else if p5 < 0 {
        slice_mul_pow5(s_buf, &mut s_len, (-p5) as u128);
    }

    // Tighten `k` so `1 < (r_num + plus) / s_den <= 10` (with inclusivity matching `inclusive`).
    while lt_rounding(
        slice_cmp_sum(
            &s_buf[..s_len],
            &r_buf[..r_len],
            &minus_buf[..minus_len],
            plus_shift,
        ),
        inclusive,
    ) {
        let ovf = slice_mul_add_u64(s_buf, &mut s_len, 10, 0);
        debug_assert!(!ovf);
        k += 1;
    }
    loop {
        let ovf1 = slice_mul_add_u64(r_buf, &mut r_len, 10, 0);
        let ovf2 = slice_mul_add_u64(minus_buf, &mut minus_len, 10, 0);
        debug_assert!(!ovf1 && !ovf2);
        if lt_rounding(
            slice_cmp_sum(
                &s_buf[..s_len],
                &r_buf[..r_len],
                &minus_buf[..minus_len],
                plus_shift,
            ),
            inclusive,
        ) {
            break;
        }
        k -= 1;
    }

    let mut i = 0usize;
    let (down, up) = loop {
        let d = slice_div_rem_digit(r_buf, &mut r_len, &s_buf[..s_len]);
        debug_assert!(d < 10);
        out_digits[i] = b'0' + d;
        i += 1;

        let down = lt_rounding(slice_cmp(r_buf, r_len, minus_buf, minus_len), inclusive);
        let up = lt_rounding(
            slice_cmp_sum(
                &s_buf[..s_len],
                &r_buf[..r_len],
                &minus_buf[..minus_len],
                plus_shift,
            ),
            inclusive,
        );
        if down || up {
            break (down, up);
        }

        let ovf1 = slice_mul_add_u64(r_buf, &mut r_len, 10, 0);
        let ovf2 = slice_mul_add_u64(minus_buf, &mut minus_len, 10, 0);
        debug_assert!(!ovf1 && !ovf2);
    };

    if up && (!down || slice_cmp_shl1(&r_buf[..r_len], &s_buf[..s_len]) != Ordering::Less) {
        if let Some(c) = round_up_digits(&mut out_digits[..i]) {
            out_digits[i] = c;
            i += 1;
            k += 1;
        }
    }

    while i > 1 && out_digits[i - 1] == b'0' {
        i -= 1;
    }
    (i, k)
}

/// Runs exact Stage 3 Dragon4 if the required bigints fit in stack (`no_alloc`) or heap (`alloc`).
fn dragon4_shortest<const W: usize, const MB: usize>(
    m: &Uint<W>,
    lower_half_spacing: bool,
    exp2: i128,
    inclusive: bool,
    out_digits: &mut [u8],
) -> Option<(usize, i128)> {
    let abs_e = exp2.unsigned_abs();
    if abs_e > 1_000_000 {
        return None;
    }
    let mant_shift = if lower_half_spacing { 2i128 } else { 1i128 };
    let mant_bw = (m.bit_width() as i128) + mant_shift;
    let k = mul_log10_2_floor(mant_bw - 1 + exp2) + 1;
    let p2 = exp2 - k;
    let p5 = -k;

    let r_bits: usize = (MB + 4)
        + (p2.max(0) as usize)
        + (((p5.max(0) as u128) * 2322) / 1000) as usize;
    let s_bits: usize = 2
        + ((-p2).max(0) as usize)
        + ((((-p5).max(0) as u128) * 2322) / 1000) as usize;
    let needed_limbs = (r_bits.max(s_bits) + 64).div_ceil(64) + 2;

    if needed_limbs <= EXACT_STACK_LIMBS {
        let mut r_buf = [0u64; EXACT_STACK_LIMBS];
        let mut minus_buf = [0u64; EXACT_STACK_LIMBS];
        let mut s_buf = [0u64; EXACT_STACK_LIMBS];
        return Some(dragon4_shortest_in_bufs(
            m,
            lower_half_spacing,
            exp2,
            inclusive,
            &mut r_buf[..needed_limbs],
            &mut minus_buf[..needed_limbs],
            &mut s_buf[..needed_limbs],
            out_digits,
        ));
    }

    #[cfg(feature = "alloc")]
    if needed_limbs <= EXACT_HEAP_MAX_LIMBS {
        let mut r_buf = alloc::vec![0u64; needed_limbs];
        let mut minus_buf = alloc::vec![0u64; needed_limbs];
        let mut s_buf = alloc::vec![0u64; needed_limbs];
        return Some(dragon4_shortest_in_bufs(
            m,
            lower_half_spacing,
            exp2,
            inclusive,
            &mut r_buf,
            &mut minus_buf,
            &mut s_buf,
            out_digits,
        ));
    }

    None
}

/// Top-level shortest formatter for finite nonzero `|v| = m * 2^e`.
fn fmt_finite_shortest<const W: usize, const MB: usize>(
    f: &mut Formatter<'_>,
    sign: &'static str,
    m: &Uint<W>,
    e: i128,
    lower_half_spacing: bool,
    inclusive: bool,
) -> fmt::Result {
    let exp2 = if lower_half_spacing { e - 2 } else { e - 1 };

    let max_limbs = (2 * W + 24) / 8;
    let exp_bits = if exp2 == 0 {
        0
    } else {
        128 - exp2.unsigned_abs().leading_zeros()
    };
    let l1 = ((MB + 84 + exp_bits as usize).div_ceil(64)).min(max_limbs);

    let mut sig_buf = SigBuf::<W>::new();

    // Stage 1: Fast fixed-point scaling at `l1` limbs, followed by Stage 2 high-precision retry
    // at `max_limbs` when `l1 < max_limbs`.
    let (approx_q, approx_limbs, approx_p) =
        match shortest_at_precision::<W, MB>(m, lower_half_spacing, exp2, l1) {
            ShortestOutcome::Resolved { q, int_limbs, p } => {
                let digits = fp_int_to_dec_digits(q, int_limbs, sig_buf.as_mut_slice());
                let exp_k = p + (digits.len() as i128);
                return fmt_shortest_digits(f, sign, digits, exp_k);
            }
            ShortestOutcome::Ambiguous {
                approx_q,
                int_limbs,
                p,
            } => {
                if l1 < max_limbs {
                    match shortest_at_precision::<W, MB>(m, lower_half_spacing, exp2, max_limbs) {
                        ShortestOutcome::Resolved { q, int_limbs, p } => {
                            let digits = fp_int_to_dec_digits(q, int_limbs, sig_buf.as_mut_slice());
                            let exp_k = p + (digits.len() as i128);
                            return fmt_shortest_digits(f, sign, digits, exp_k);
                        }
                        ShortestOutcome::Ambiguous {
                            approx_q,
                            int_limbs,
                            p,
                        } => (approx_q, int_limbs, p),
                    }
                } else {
                    (approx_q, int_limbs, p)
                }
            }
        };

    // Stage 3: Exact Burger-Dybvig Dragon4.
    if let Some((len, exp_k)) =
        dragon4_shortest::<W, MB>(m, lower_half_spacing, exp2, inclusive, sig_buf.as_mut_slice())
    {
        return fmt_shortest_digits(f, sign, &sig_buf.as_slice()[..len], exp_k);
    }

    // Emergency fallback for extreme `|exp2| > 10^6` where exact bigints cannot allocate.
    let digits = fp_int_to_dec_digits(approx_q, approx_limbs, sig_buf.as_mut_slice());
    let exp_k = approx_p + (digits.len() as i128);
    fmt_shortest_digits(f, sign, digits, exp_k)
}

// -----------------------------------------------------------------------------
// Mode B: Exact fixed-precision formatting (`{:.PREC?}`, ties-to-even)
// -----------------------------------------------------------------------------

/// Converts `limbs[..len]` (modifying `limbs` in-place) into ASCII decimal digits stored at the
/// end of `out`, returning `&out[start..]`.
fn slice_to_dec_digits<'a>(limbs: &mut [u64], mut len: usize, out: &'a mut [u8]) -> &'a [u8] {
    while len > 0 && limbs[len - 1] == 0 {
        len -= 1;
    }
    if len == 0 {
        let pos = out.len() - 1;
        out[pos] = b'0';
        return &out[pos..];
    }

    let mut pos = out.len();
    const BASE: u128 = 10_000_000_000_000_000_000; // 10^19
    while len > 1 {
        let mut rem: u128 = 0;
        for d in limbs[..len].iter_mut().rev() {
            let cur = (rem << 64) | (*d as u128);
            *d = (cur / BASE) as u64;
            rem = cur % BASE;
        }
        while len > 0 && limbs[len - 1] == 0 {
            len -= 1;
        }
        let mut chunk = rem as u64;
        for _ in 0..19 {
            pos -= 1;
            out[pos] = b'0' + (chunk % 10) as u8;
            chunk /= 10;
        }
    }
    if len == 1 {
        let mut v = limbs[0];
        while v > 0 {
            pos -= 1;
            out[pos] = b'0' + (v % 10) as u8;
            v /= 10;
        }
    }
    &out[pos..]
}

/// Right-shifts `buf[..*len]` in-place by `shift >= 1` bits using IEEE 754 round-to-nearest-ties-to-even.
fn slice_shr_round_ties_even(buf: &mut [u64], len: &mut usize, shift: usize) {
    debug_assert!(shift >= 1);
    while *len > 0 && buf[*len - 1] == 0 {
        *len -= 1;
    }
    if *len == 0 {
        return;
    }

    let half_bit = shift - 1;
    let half_word = half_bit / 64;
    let half_off = half_bit % 64;

    if half_word >= *len {
        *len = 0;
        return;
    }

    let half = ((buf[half_word] >> half_off) & 1) != 0;
    let mut sticky = half_off > 0 && (buf[half_word] & ((1u64 << half_off) - 1)) != 0;
    if !sticky {
        for &w in &buf[..half_word] {
            if w != 0 {
                sticky = true;
                break;
            }
        }
    }

    let ws = shift / 64;
    let bs = shift % 64;
    let mut new_len = if ws >= *len {
        0
    } else {
        let nl = *len - ws;
        for i in 0..nl {
            let lo = buf[i + ws];
            let val = if bs == 0 {
                lo
            } else {
                let hi = if i + ws + 1 < *len {
                    buf[i + ws + 1]
                } else {
                    0
                };
                (lo >> bs) | (hi << (64 - bs))
            };
            buf[i] = val;
        }
        nl
    };
    while new_len > 0 && buf[new_len - 1] == 0 {
        new_len -= 1;
    }

    let q_is_odd = new_len > 0 && (buf[0] & 1) != 0;
    if half && (sticky || q_is_odd) {
        let mut carry = 1u64;
        for d in &mut buf[..new_len] {
            let (sum, c) = d.overflowing_add(carry);
            *d = sum;
            carry = c as u64;
            if carry == 0 {
                break;
            }
        }
        if carry != 0 {
            buf[new_len] = 1;
            new_len += 1;
        }
    }
    *len = new_len;
}

/// Formats `m_odd * 2^e_eff` in fixed-point notation with `prec` fractional digits using a
/// caller-provided zero-initialized limb scratch buffer and digit scratch buffer.
fn fmt_exact_fixed_in_bufs<const W: usize>(
    f: &mut Formatter<'_>,
    sign: &'static str,
    m_odd: &Uint<W>,
    e_eff: i128,
    prec: usize,
    limbs: &mut [u64],
    dec_buf: &mut [u8],
) -> fmt::Result {
    if e_eff >= 0 {
        let len = slice_load_uint_shl(m_odd, e_eff as usize, limbs);
        let digits = slice_to_dec_digits(limbs, len, dec_buf);
        fmt_fixed_scaled_digits(f, sign, digits, 0, prec)
    } else {
        let n = (-e_eff) as u128;
        if (prec as u128) >= n {
            let mut len = slice_load_uint_shl(m_odd, 0, limbs);
            slice_mul_pow5(limbs, &mut len, n);
            let digits = slice_to_dec_digits(limbs, len, dec_buf);
            let n_usize = n as usize;
            fmt_fixed_scaled_digits(f, sign, digits, n_usize, prec - n_usize)
        } else {
            let mut len = slice_load_uint_shl(m_odd, 0, limbs);
            slice_mul_pow5(limbs, &mut len, prec as u128);
            let shift = (n - (prec as u128)) as usize;
            slice_shr_round_ties_even(limbs, &mut len, shift);
            if len == 0 {
                fmt_zero(f, sign, prec)
            } else {
                let digits = slice_to_dec_digits(limbs, len, dec_buf);
                fmt_fixed_scaled_digits(f, sign, digits, prec, 0)
            }
        }
    }
}

/// Best-effort non-allocating fallback for `{:.PREC?}` when exact bigints exceed maximum buffer
/// limits (e.g. `no_alloc` on `F128::MAX` or formats with `|e_eff| > 10^6`).
fn fmt_fixed_approx_fallback<const W: usize, const MB: usize>(
    f: &mut Formatter<'_>,
    sign: &'static str,
    m_odd: &Uint<W>,
    e_eff: i128,
    prec: usize,
) -> fmt::Result {
    let max_limbs = (2 * W + 24) / 8;
    let p_bits = (max_limbs as Exponent) * 64;
    let m_fp = fp_load_uint_shl(m_odd, 0, max_limbs);
    let m_bw = fp_bit_width(&m_fp, max_limbs);
    let m_norm = fp_shl(&m_fp, p_bits - m_bw, max_limbs);

    // Compute `|v| * 10^prec` approximate scale:
    let (p5_norm, p5_exp2) = fp_pow5::<W>(prec as i128, max_limbs);
    let (prod_norm, prod_exp2) = fp_norm_mul(
        &m_norm,
        (m_bw as i128) + e_eff,
        &p5_norm,
        p5_exp2 + (prec as i128),
        max_limbs,
    );

    if prod_exp2 < 0 {
        return fmt_zero(f, sign, prec);
    }

    // Scale `|v| * 10^prec` down by `10^drop_dec` so the leading digits fit in `Fixed<W>`.
    let sig_target_bits = (MB as i128) + 32;
    let drop_dec: i128 = if prod_exp2 > sig_target_bits {
        mul_log10_2_floor(prod_exp2 - sig_target_bits).max(0)
    } else {
        0
    };

    let (scaled_norm, scaled_exp2) = if drop_dec > 0 {
        let (p5_d, p5_de) = fp_pow5::<W>(-drop_dec, max_limbs);
        fp_norm_mul(&prod_norm, prod_exp2, &p5_d, p5_de - drop_dec, max_limbs)
    } else {
        (prod_norm, prod_exp2)
    };

    if scaled_exp2 < 0 {
        return fmt_zero(f, sign, prec);
    }
    let shift = p_bits.saturating_sub(scaled_exp2 as Exponent);
    let mut q = fp_shr(&scaled_norm, shift, max_limbs);
    if shift > 0 && fp_bit(&scaled_norm, shift - 1) {
        q = fp_add_u64(&q, 1, max_limbs);
    }

    let mut sig_buf = SigBuf::<W>::new();
    let digits = fp_int_to_dec_digits(q, max_limbs, sig_buf.as_mut_slice());
    if digits == b"0" {
        return fmt_zero(f, sign, prec);
    }

    let drop_usize = usize::try_from(drop_dec).unwrap_or(usize::MAX);
    if drop_usize >= prec {
        let int_zeros = drop_usize - prec;
        if prec == 0 {
            pad_and_write(f, sign, &[Part::Bytes(digits), Part::Zeros(int_zeros)])
        } else {
            pad_and_write(
                f,
                sign,
                &[
                    Part::Bytes(digits),
                    Part::Zeros(int_zeros),
                    Part::Bytes(b"."),
                    Part::Zeros(prec),
                ],
            )
        }
    } else {
        let scale_right = prec - drop_usize;
        fmt_fixed_scaled_digits(f, sign, digits, scale_right, drop_usize)
    }
}

/// Top-level exact fixed-precision formatter (`{:.PREC?}`) for finite nonzero `|v| = m * 2^e`.
fn fmt_finite_exact_fixed<const W: usize, const MB: usize>(
    f: &mut Formatter<'_>,
    sign: &'static str,
    m: &Uint<W>,
    e: i128,
    prec: usize,
) -> fmt::Result {
    let tz = m.trailing_zeros();
    let m_odd = m.shr(tz);
    let bw_odd = m_odd.bit_width() as u128;
    let e_eff = e + (tz as i128);

    // Quick exact-zero filter when `|v| * 10^prec < 0.5`.
    if e_eff < 0 {
        let n = (-e_eff) as u128;
        if (prec as u128) < n {
            let shift = n - (prec as u128);
            let p5_max_bits = ((prec as u128) * 232_193).div_ceil(100_000);
            if bw_odd + p5_max_bits <= shift - 1 {
                return fmt_zero(f, sign, prec);
            }
        }
    }

    let needed_bits: Option<usize> = if e_eff >= 0 {
        let e_u = e_eff as u128;
        if e_u > 1_000_000 {
            None
        } else {
            Some((bw_odd + e_u) as usize)
        }
    } else {
        let n = (-e_eff) as u128;
        let pow5_exp = n.min(prec as u128);
        if pow5_exp > 1_000_000 {
            None
        } else {
            let p5_bits = (pow5_exp * 232_193).div_ceil(100_000);
            Some((bw_odd + p5_bits + 2) as usize)
        }
    };

    if let Some(bits) = needed_bits {
        let needed_limbs = bits.div_ceil(64) + 1;
        if needed_limbs <= EXACT_STACK_LIMBS {
            let mut limbs = [0u64; EXACT_STACK_LIMBS];
            let mut dec_buf = [0u8; EXACT_STACK_DIGITS];
            return fmt_exact_fixed_in_bufs(
                f,
                sign,
                &m_odd,
                e_eff,
                prec,
                &mut limbs[..needed_limbs],
                &mut dec_buf,
            );
        }

        #[cfg(feature = "alloc")]
        if needed_limbs <= EXACT_HEAP_MAX_LIMBS {
            let mut limbs = alloc::vec![0u64; needed_limbs];
            let mut dec_buf = alloc::vec![0u8; needed_limbs * 20 + 20];
            return fmt_exact_fixed_in_bufs(
                f,
                sign,
                &m_odd,
                e_eff,
                prec,
                &mut limbs,
                &mut dec_buf,
            );
        }
    }

    fmt_fixed_approx_fallback::<W, MB>(f, sign, &m_odd, e_eff, prec)
}

// -----------------------------------------------------------------------------
// `Debug` implementation for `Float<W, MB>`
// -----------------------------------------------------------------------------

impl<const W: usize, const MB: usize, const RM: u8> Debug for Float<W, MB, RM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const {
            assert!(
                MB >= 1 && MB < (Self::BITS as usize) - 1,
                "Float mantissa bits MB must satisfy 1 <= MB < Self::BITS - 1"
            );
        }

        let bits = self.to_bits();
        let is_negative = self.is_sign_negative();
        let raw_exp = bits.bitand(Self::SIGN_MASK).shr(Self::MB);
        let raw_mant = bits.bitand(Self::MANTISSA_MASK);
        let exp_max = Uint::<W>::MAX.wrapping_shr(Self::MB + 1);

        if raw_exp.eq(&exp_max) {
            return if !raw_mant.is_zero() {
                // NaN never prints a sign (`+` or `-`), even with `{:+?}` or negative sign bit.
                pad_and_write(f, "", &[Part::Bytes(b"NaN")])
            } else {
                let sign = determine_sign(is_negative, f.sign_plus());
                pad_and_write(f, sign, &[Part::Bytes(b"inf")])
            };
        }

        let sign = determine_sign(is_negative, f.sign_plus());

        if raw_exp.is_zero() && raw_mant.is_zero() {
            let frac_digits = f.precision().unwrap_or(1);
            return fmt_zero(f, sign, frac_digits);
        }

        let bias = Uint::<W>::power_of_two(Self::EXPONENT_BITS - 1).sub(Uint::ONE);
        let (m, e, lower_half_spacing) = if raw_exp.is_zero() {
            // Subnormal: |v| = raw_mant * 2^(1 - bias - MB)
            let bias_u128 = uint_to_u128(&bias)
                .expect("Float exponent magnitude exceeds i128 range for decimal formatting");
            let e = 1i128 - (bias_u128 as i128) - (MB as i128);
            (raw_mant, e, false)
        } else {
            // Normal: |v| = (raw_mant | 2^MB) * 2^(raw_exp - bias - MB)
            let delta: i128 = if raw_exp.ge(&bias) {
                let d = uint_to_u128(&raw_exp.sub(bias))
                    .expect("Float exponent magnitude exceeds i128 range for decimal formatting");
                d as i128
            } else {
                let d = uint_to_u128(&bias.sub(raw_exp))
                    .expect("Float exponent magnitude exceeds i128 range for decimal formatting");
                -(d as i128)
            };
            let m = raw_mant.bitor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
            let e = delta - (MB as i128);
            let lower_half_spacing = raw_mant.is_zero() && raw_exp.gt(&Uint::ONE);
            (m, e, lower_half_spacing)
        };

        if let Some(prec) = f.precision() {
            fmt_finite_exact_fixed::<W, MB>(f, sign, &m, e, prec)
        } else {
            let inclusive = m.is_even();
            fmt_finite_shortest::<W, MB>(f, sign, &m, e, lower_half_spacing, inclusive)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::{Debug, Write};

    use crate::Float;
    use crate::types::{F16, F32, F64, F80, F128, F256};

    /// Tiny fixed-capacity `fmt::Write` sink so core tests run even under `--no-default-features`.
    struct FixedStr<const N: usize> {
        buf: [u8; N],
        len: usize,
    }

    impl<const N: usize> FixedStr<N> {
        fn new() -> Self {
            Self {
                buf: [0u8; N],
                len: 0,
            }
        }

        fn as_str(&self) -> &str {
            core::str::from_utf8(&self.buf[..self.len]).unwrap()
        }
    }

    impl<const N: usize> Write for FixedStr<N> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let b = s.as_bytes();
            let end = self.len + b.len();
            assert!(end <= N, "FixedStr overflow: {end} > {N}");
            self.buf[self.len..end].copy_from_slice(b);
            self.len = end;
            Ok(())
        }
    }

    macro_rules! assert_debug_eq {
        ($fmt_str: literal, $prim: expr, $f: expr) => {{
            let mut a = FixedStr::<2048>::new();
            let mut b = FixedStr::<2048>::new();
            write!(a, $fmt_str, $f).unwrap();
            write!(b, $fmt_str, $prim).unwrap();
            assert_eq!(
                a.as_str(),
                b.as_str(),
                "Debug mismatch for fmt={} on bits={:#x}",
                $fmt_str,
                $prim.to_bits()
            );
        }};
    }

    fn check_all_format_flags_f64(x: f64) {
        let fx = F64::from(x);
        assert_debug_eq!("{:?}", x, fx);
        assert_debug_eq!("{:+?}", x, fx);
        assert_debug_eq!("{:-?}", x, fx);
        assert_debug_eq!("{:#?}", x, fx);
        assert_debug_eq!("{:10?}", x, fx);
        assert_debug_eq!("{:010?}", x, fx);
        assert_debug_eq!("{:+010?}", x, fx);
        assert_debug_eq!("{:<10?}", x, fx);
        assert_debug_eq!("{:^10?}", x, fx);
        assert_debug_eq!("{:>10?}", x, fx);
        assert_debug_eq!("{:<010?}", x, fx);
        assert_debug_eq!("{:^010?}", x, fx);
        assert_debug_eq!("{:>010?}", x, fx);
        assert_debug_eq!("{:*<10?}", x, fx);
        assert_debug_eq!("{:*^10?}", x, fx);
        assert_debug_eq!("{:*>10?}", x, fx);
        assert_debug_eq!("{:*<010?}", x, fx);
        assert_debug_eq!("{:.0?}", x, fx);
        assert_debug_eq!("{:.1?}", x, fx);
        assert_debug_eq!("{:.2?}", x, fx);
        assert_debug_eq!("{:.5?}", x, fx);
        assert_debug_eq!("{:.20?}", x, fx);
        assert_debug_eq!("{:+012.3?}", x, fx);
        assert_debug_eq!("{:*^15.4?}", x, fx);
        assert_debug_eq!("{:010.0?}", x, fx);
    }

    fn check_all_format_flags_f32(x: f32) {
        let fx = F32::from(x);
        assert_debug_eq!("{:?}", x, fx);
        assert_debug_eq!("{:+?}", x, fx);
        assert_debug_eq!("{:#?}", x, fx);
        assert_debug_eq!("{:010?}", x, fx);
        assert_debug_eq!("{:+010?}", x, fx);
        assert_debug_eq!("{:*<10?}", x, fx);
        assert_debug_eq!("{:*^10?}", x, fx);
        assert_debug_eq!("{:*>10?}", x, fx);
        assert_debug_eq!("{:.0?}", x, fx);
        assert_debug_eq!("{:.1?}", x, fx);
        assert_debug_eq!("{:.2?}", x, fx);
        assert_debug_eq!("{:.8?}", x, fx);
        assert_debug_eq!("{:+012.3?}", x, fx);
    }

    #[test]
    fn special_boundary_and_tie_cases_f32_f64() {
        let f64_cases = [
            0.0_f64,
            -0.0,
            1.0,
            -1.0,
            0.5,
            -0.5,
            1.5,
            2.5,
            3.5,
            0.25,
            0.1,
            0.2,
            0.3,
            0.05,
            0.15,
            0.005,
            9.999,
            99.5,
            99.9,
            f64::NAN,
            -f64::NAN,
            f64::from_bits(0x7ff8_0000_0000_0001),
            f64::from_bits(0xfff8_1234_5678_9abc),
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MAX,
            f64::MIN,
            f64::MIN_POSITIVE,
            f64::MIN_POSITIVE.next_down(),
            f64::MIN_POSITIVE.next_up(),
            f64::from_bits(1),
            f64::from_bits(2),
            f64::from_bits(3),
            f64::from_bits(0x8000_0000_0000_0001),
            f64::EPSILON,
            // Exact scientific-notation thresholds
            1e-4,
            (1e-4_f64).next_down(),
            (1e-4_f64).next_up(),
            -1e-4,
            (-1e-4_f64).next_down(),
            (-1e-4_f64).next_up(),
            1e16,
            (1e16_f64).next_down(),
            (1e16_f64).next_up(),
            -1e16,
            (-1e16_f64).next_down(),
            (-1e16_f64).next_up(),
            1e15,
            1e-5,
            1e-300,
            -1e-300,
            1e300,
            // Exact shortest-mode midpoint ties (must break away from zero, matching Rust's Dragon4)
            1125899906842624.25,
            1125899906842624.75,
            -1125899906842624.25,
            -1125899906842624.75,
            // Powers of 2 near powers of 10
            core::f64::consts::PI,
            core::f64::consts::E,
            1e23,
            1e-23,
            (1u64 << 53) as f64,
            ((1u64 << 53) - 1) as f64,
        ];
        for x in f64_cases {
            check_all_format_flags_f64(x);
        }

        // Also test deep exact fractional precision on f64::MIN_POSITIVE_SUBNORMAL
        let tiny = f64::from_bits(1);
        let ftiny = F64::from(tiny);
        assert_debug_eq!("{:.1073?}", tiny, ftiny);
        assert_debug_eq!("{:.1074?}", tiny, ftiny);
        assert_debug_eq!("{:.1080?}", tiny, ftiny);

        let f32_cases = [
            0.0_f32,
            -0.0,
            1.0,
            -1.0,
            0.5,
            1.5,
            2.5,
            0.1,
            0.2,
            0.3,
            0.05,
            0.15,
            9.999,
            f32::NAN,
            -f32::NAN,
            f32::from_bits(0x7fc0_0001),
            f32::from_bits(0xffc0_1234),
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::MAX,
            f32::MIN,
            f32::MIN_POSITIVE,
            f32::MIN_POSITIVE.next_down(),
            f32::MIN_POSITIVE.next_up(),
            f32::from_bits(1),
            f32::from_bits(2),
            f32::from_bits(0x8000_0001),
            f32::EPSILON,
            // Thresholds where 1e-4_f32 and 1e16_f32 are inexact binary floats
            1e-4,
            (1e-4_f32).next_down(),
            (1e-4_f32).next_up(),
            1e16,
            (1e16_f32).next_down(),
            (1e16_f32).next_up(),
            // Exact shortest-mode midpoint ties
            2097152.25,
            2097152.75,
            -2097152.25,
            -2097152.75,
            core::f32::consts::PI,
            core::f32::consts::E,
        ];
        for x in f32_cases {
            check_all_format_flags_f32(x);
        }

        let tiny32 = f32::from_bits(1);
        let ftiny32 = F32::from(tiny32);
        assert_debug_eq!("{:.148?}", tiny32, ftiny32);
        assert_debug_eq!("{:.149?}", tiny32, ftiny32);
        assert_debug_eq!("{:.160?}", tiny32, ftiny32);
    }

    #[test]
    fn random_f32_and_f64_parity() {
        let mut state = 0x1234_5678_9abc_def0_u64;
        let mut next_u64 = || -> u64 {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };

        for i in 0..20_000 {
            let r = next_u64();
            let u32_bits = match i % 5 {
                0 => r as u32,
                1 => (r as u32) & 0x807f_ffff,             // subnormals & zeros
                2 => ((r as u32) & 0x807f_ffff) | 0x0080_0000, // near MIN_POSITIVE
                3 => ((r as u32) & 0x8000_0007) | 0x4a00_0000, // near shortest-mode ties
                _ => (r as u32) | 0x7f00_0000,             // near MAX / inf / NaN
            };
            let f32_v = f32::from_bits(u32_bits);
            assert_debug_eq!("{:?}", f32_v, F32::from(f32_v));
            if i % 10 == 0 {
                assert_debug_eq!("{:.3?}", f32_v, F32::from(f32_v));
            }

            let u64_bits = match i % 5 {
                0 => r,
                1 => r & 0x800f_ffff_ffff_ffff,                 // subnormals & zeros
                2 => (r & 0x800f_ffff_ffff_ffff) | 0x0010_0000_0000_0000, // near MIN_POSITIVE
                3 => (r & 0x8000_0000_0000_0007) | 0x4310_0000_0000_0000, // near shortest-mode ties
                _ => r | 0x7fe0_0000_0000_0000,                 // near MAX / inf / NaN
            };
            let f64_v = f64::from_bits(u64_bits);
            assert_debug_eq!("{:?}", f64_v, F64::from(f64_v));
            if i % 10 == 0 {
                assert_debug_eq!("{:.4?}", f64_v, F64::from(f64_v));
            }
        }
    }

    #[test]
    fn exhaustive_f16_roundtrip_and_minimality() {
        // Test ALL 65,536 F16 bit patterns: formatting must round-trip via `F16::from_str`.
        for bits in 0u16..=u16::MAX {
            let f = F16::from_le_bytes(bits.to_le_bytes());
            let mut s = FixedStr::<64>::new();
            write!(s, "{f:?}").unwrap();
            let text = s.as_str();
            if f.is_nan() {
                assert_eq!(text, "NaN");
            } else {
                let parsed = F16::from_str(text).unwrap();
                assert_eq!(
                    parsed.to_bits(),
                    f.to_bits(),
                    "F16 round-trip failed for bits={bits:#06x}, formatted={text:?}"
                );
            }
        }
    }

    #[test]
    fn generic_and_large_float_formats() {
        fn fmt_debug<T: Debug>(v: T) -> FixedStr<256> {
            let mut s = FixedStr::<256>::new();
            write!(s, "{v:?}").unwrap();
            s
        }

        // Non-standard tiny / narrow formats
        type F1_1 = Float<1, 1>;
        type F1_2 = Float<1, 2>;
        type F1_3 = Float<1, 3>; // 8-bit E4M3 float
        type F1_6 = Float<1, 6>; // EXPONENT_BITS = 1 (subnormals only)
        assert_eq!(fmt_debug(F1_1::ZERO).as_str(), "0.0");
        assert_eq!(fmt_debug(F1_1::ONE).as_str(), "1.0");
        assert_eq!(fmt_debug(F1_1::HALF).as_str(), "0.5");
        assert_eq!(fmt_debug(F1_2::ONE).as_str(), "1.0");
        assert_eq!(fmt_debug(F1_2::NEG_ONE).as_str(), "-1.0");
        assert_eq!(fmt_debug(F1_3::ZERO).as_str(), "0.0");
        assert_eq!(fmt_debug(F1_3::NEG_ZERO).as_str(), "-0.0");
        assert_eq!(fmt_debug(F1_3::ONE).as_str(), "1.0");
        assert_eq!(fmt_debug(F1_3::INFINITY).as_str(), "inf");
        assert_eq!(fmt_debug(F1_3::NEG_INFINITY).as_str(), "-inf");
        assert_eq!(fmt_debug(F1_3::NAN).as_str(), "NaN");
        assert_eq!(fmt_debug(F1_6::from_le_bytes([0])).as_str(), "0.0");
        assert_eq!(fmt_debug(F1_6::from_le_bytes([32])).as_str(), "1.0");
        assert_eq!(fmt_debug(F1_6::from_le_bytes([16])).as_str(), "0.5");
        assert_eq!(fmt_debug(F1_6::from_le_bytes([0x40])).as_str(), "inf");
        assert_eq!(fmt_debug(F1_6::from_le_bytes([0x41])).as_str(), "NaN");

        type F2_13 = Float<2, 13>;
        for raw in [0u16, 1, 2, 0x1fff, 0x2000, 0x3fff, 0x4000, 0x5fff, 0x8000, 0xa000] {
            let v = F2_13::from_le_bytes(raw.to_le_bytes());
            let s = fmt_debug(v);
            if v.is_nan() {
                assert_eq!(s.as_str(), "NaN");
            } else {
                assert_eq!(F2_13::from_str(s.as_str()).unwrap().to_bits(), v.to_bits());
            }
        }

        // F80, F128, F256 round-trip and exact values
        assert_eq!(fmt_debug(F80::ONE).as_str(), "1.0");
        assert_eq!(fmt_debug(F80::HALF).as_str(), "0.5");
        assert_eq!(fmt_debug(F128::ONE).as_str(), "1.0");
        assert_eq!(fmt_debug(F128::NEG_ONE).as_str(), "-1.0");
        assert_eq!(fmt_debug(F256::QUARTER).as_str(), "0.25");

        for s in [
            "0.0",
            "-0.0",
            "1.0",
            "0.1",
            "3.14159265358979323846264338327950288",
            "1e-4",
            "9.999999999999999999999999999999999e-5",
            "1e16",
            "1e4932",
            "1e-4932",
            "1e-4950",
        ] {
            let v = F128::from_str(s).unwrap();
            let out = fmt_debug(v);
            assert_eq!(
                F128::from_str(out.as_str()).unwrap().to_bits(),
                v.to_bits(),
                "F128 round-trip failed for {s} -> {}",
                out.as_str()
            );
        }

        for s in [
            "0.1",
            "1.23456789012345678901234567890123456789012345678901234567890123456789",
            "1e78913",
            "1e-78913",
            "1e-78950",
        ] {
            let v = F256::from_str(s).unwrap();
            let out = fmt_debug(v);
            assert_eq!(
                F256::from_str(out.as_str()).unwrap().to_bits(),
                v.to_bits(),
                "F256 round-trip failed for {s} -> {}",
                out.as_str()
            );
        }

        // Huge exponent formats: EXPONENT_BITS = 60 and EXPONENT_BITS = 124
        type F8_3 = Float<8, 3>;
        for s in ["1.0", "1.5", "-2.25", "1e100000000000000000", "1e-100000000000000000"] {
            let v = F8_3::from_str(s).unwrap();
            let out = fmt_debug(v);
            assert_eq!(
                F8_3::from_str(out.as_str()).unwrap().to_bits(),
                v.to_bits(),
                "F8_3 round-trip failed for {s} -> {}",
                out.as_str()
            );
        }

        type F16_3 = Float<16, 3>;
        for s in [
            "1.0",
            "1e1000000000000000000000000000000000000",
            "1e-1000000000000000000000000000000000000",
        ] {
            let v = F16_3::from_str(s).unwrap();
            let out = fmt_debug(v);
            assert_eq!(
                F16_3::from_str(out.as_str()).unwrap().to_bits(),
                v.to_bits(),
                "F16_3 round-trip failed for {s} -> {}",
                out.as_str()
            );
        }
    }

    #[test]
    fn all_powers_of_two_neighbors_and_stage3_dragon4_parity() {
        use super::{SigBuf, dragon4_shortest, fmt_shortest_digits};

        // Helper that forces Stage 3 Dragon4 directly for any finite nonzero Float<W, MB>.
        struct Dragon4Only<const W: usize, const MB: usize>(Float<W, MB>);
        impl<const W: usize, const MB: usize> Debug for Dragon4Only<W, MB> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let bits = self.0.to_bits();
                let sign = if self.0.is_sign_negative() { "-" } else { "" };
                let raw_exp = bits.bitand(Float::<W, MB>::SIGN_MASK).shr(Float::<W, MB>::MB);
                let raw_mant = bits.bitand(Float::<W, MB>::MANTISSA_MASK);
                let bias = crate::Uint::<W>::power_of_two(Float::<W, MB>::EXPONENT_BITS - 1)
                    .sub(crate::Uint::ONE);
                let (m, e, lower_half_spacing) = if raw_exp.is_zero() {
                    let b = super::uint_to_u128(&bias).unwrap() as i128;
                    (raw_mant, 1 - b - (MB as i128), false)
                } else {
                    let delta = if raw_exp.ge(&bias) {
                        super::uint_to_u128(&raw_exp.sub(bias)).unwrap() as i128
                    } else {
                        -(super::uint_to_u128(&bias.sub(raw_exp)).unwrap() as i128)
                    };
                    let m = raw_mant.bitor(Float::<W, MB>::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
                    (m, delta - (MB as i128), raw_mant.is_zero() && raw_exp.gt(&crate::Uint::ONE))
                };
                let exp2 = if lower_half_spacing { e - 2 } else { e - 1 };
                let mut buf = SigBuf::<W>::new();
                let (len, exp_k) =
                    dragon4_shortest::<W, MB>(&m, lower_half_spacing, exp2, m.is_even(), buf.as_mut_slice())
                        .unwrap();
                fmt_shortest_digits(f, sign, &buf.as_slice()[..len], exp_k)
            }
        }

        // Every single f32 power of 2 (normal + subnormal) and its immediate neighbors
        for exp in 1u32..=254 {
            let p = f32::from_bits(exp << 23);
            for v in [p.next_down(), p, p.next_up()] {
                if v.is_finite() && v != 0.0 {
                    assert_debug_eq!("{:?}", v, F32::from(v));
                    assert_debug_eq!("{:?}", v, Dragon4Only(F32::from(v)));
                }
            }
        }
        for bit in 0u32..23 {
            let p = f32::from_bits(1u32 << bit);
            for v in [p.next_down(), p, p.next_up()] {
                if v != 0.0 {
                    assert_debug_eq!("{:?}", v, F32::from(v));
                    assert_debug_eq!("{:?}", v, Dragon4Only(F32::from(v)));
                }
            }
        }

        // Every single f64 power of 2 (normal + subnormal) and its immediate neighbors
        for exp in 1u64..=2046 {
            let p = f64::from_bits(exp << 52);
            for v in [p.next_down(), p, p.next_up()] {
                if v.is_finite() && v != 0.0 {
                    assert_debug_eq!("{:?}", v, F64::from(v));
                    if exp % 13 == 0 || exp <= 10 || exp >= 2036 || (exp >= 1000 && exp <= 1080) {
                        assert_debug_eq!("{:?}", v, Dragon4Only(F64::from(v)));
                    }
                }
            }
        }
        for bit in 0u64..52 {
            let p = f64::from_bits(1u64 << bit);
            for v in [p.next_down(), p, p.next_up()] {
                if v != 0.0 {
                    assert_debug_eq!("{:?}", v, F64::from(v));
                    assert_debug_eq!("{:?}", v, Dragon4Only(F64::from(v)));
                }
            }
        }
    }

    #[cfg(feature = "alloc")]
    mod alloc_tests {
        use super::*;
        use crate::test::test_bignum;
        use alloc::string::String;

        pub trait DebugFmt {
            fn debug_plain(&self) -> String;
            fn debug_formatted(&self, width: Option<u8>, prec: Option<u8>, flags: u8) -> String;
        }

        macro_rules! impl_debug_fmt {
            ($($ty: ty),*) => {
                $(
                    impl DebugFmt for $ty {
                        fn debug_plain(&self) -> String {
                            alloc::format!("{self:?}")
                        }

                        fn debug_formatted(&self, width: Option<u8>, prec: Option<u8>, flags: u8) -> String {
                            let w = width.map(|v| (v % 32) as usize);
                            let p = prec.map(|v| (v % 24) as usize);
                            match (w, p, flags % 8) {
                                (None, None, 0) => alloc::format!("{self:?}"),
                                (None, None, 1) => alloc::format!("{self:+?}"),
                                (None, None, 2) => alloc::format!("{self:#?}"),
                                (None, None, _) => alloc::format!("{self:-?}"),
                                (Some(w), None, 0) => alloc::format!("{self:w$?}"),
                                (Some(w), None, 1) => alloc::format!("{self:+0w$?}"),
                                (Some(w), None, 2) => alloc::format!("{self:0w$?}"),
                                (Some(w), None, 3) => alloc::format!("{self:<w$?}"),
                                (Some(w), None, 4) => alloc::format!("{self:^w$?}"),
                                (Some(w), None, 5) => alloc::format!("{self:*^w$?}"),
                                (Some(w), None, 6) => alloc::format!("{self:*<w$?}"),
                                (Some(w), None, _) => alloc::format!("{self:+>w$?}"),
                                (None, Some(p), 0) => alloc::format!("{self:.p$?}"),
                                (None, Some(p), 1) => alloc::format!("{self:+.p$?}"),
                                (None, Some(p), _) => alloc::format!("{self:#.p$?}"),
                                (Some(w), Some(p), 0) => alloc::format!("{self:w$.p$?}"),
                                (Some(w), Some(p), 1) => alloc::format!("{self:+0w$.p$?}"),
                                (Some(w), Some(p), 2) => alloc::format!("{self:0w$.p$?}"),
                                (Some(w), Some(p), 3) => alloc::format!("{self:*<w$.p$?}"),
                                (Some(w), Some(p), 4) => alloc::format!("{self:*^w$.p$?}"),
                                (Some(w), Some(p), _) => alloc::format!("{self:*>w$.p$?}"),
                            }
                        }
                    }
                )*
            };
        }

        crate::test::test_all! {
            testing floats;

            impl_debug_fmt!(FTest, FTestBase);

            test_bignum! {
                function: <FTest as DebugFmt>::debug_plain(a: ref &FTestBase)
            }
            test_bignum! {
                function: <FTest as DebugFmt>::debug_formatted(a: ref &FTestBase, width: Option<u8>, prec: Option<u8>, flags: u8)
            }
        }

        #[test]
        fn large_precision_and_f128_f256_exact_fixed() {
            // Huge precision on simple binary fraction (must stream zeros without large allocation)
            assert_eq!(
                alloc::format!("{:.5000?}", F64::from(0.5_f64)),
                alloc::format!("{:.5000?}", 0.5_f64)
            );
            assert_eq!(
                alloc::format!("{:.5000?}", F64::from(-0.0_f64)),
                alloc::format!("{:.5000?}", -0.0_f64)
            );
            assert_eq!(
                alloc::format!("{:.2?}", F64::MAX),
                alloc::format!("{:.2?}", f64::MAX)
            );
            assert_eq!(
                alloc::format!("{:.0?}", F128::ONE),
                "1"
            );
            assert_eq!(
                alloc::format!("{:.4?}", F128::HALF),
                "0.5000"
            );
            assert_eq!(
                alloc::format!("{:.2?}", F128::from_str("123.456").unwrap()),
                "123.46"
            );
            assert_eq!(
                alloc::format!("{:.2?}", F256::from_str("9.999").unwrap()),
                "10.00"
            );
            // F128::MAX exact fixed precision exercises heap-allocated bigint path
            let f128_max_fixed = alloc::format!("{:.2?}", F128::MAX);
            assert!(f128_max_fixed.ends_with(".00"));
            assert!(f128_max_fixed.starts_with("1189731495357231765085759326628007"));
            assert_eq!(f128_max_fixed.len(), 4933 + 3);

            // Wide padding (> 64 spaces, exact 64 spaces) and explicit '0' fill char
            assert_eq!(alloc::format!("{:100?}", F64::ONE), alloc::format!("{:100?}", 1.0_f64));
            assert_eq!(alloc::format!("{:67?}", F64::ONE), alloc::format!("{:67?}", 1.0_f64));
            assert_eq!(alloc::format!("{:0<40?}", F64::ONE), alloc::format!("{:0<40?}", 1.0_f64));
            assert_eq!(alloc::format!("{:040?}", F64::ONE), alloc::format!("{:040?}", 1.0_f64));

            // F256 shortest formatting (including values hitting Dragon4 heap path)
            let s_f256_min_sub = alloc::format!("{:?}", F256::MIN_POSITIVE_SUBNORMAL);
            assert_eq!(F256::from_str(&s_f256_min_sub).unwrap().to_bits(), F256::MIN_POSITIVE_SUBNORMAL.to_bits());
            let s_f256_max = alloc::format!("{:?}", F256::MAX);
            assert_eq!(F256::from_str(&s_f256_max).unwrap().to_bits(), F256::MAX.to_bits());
            let s_f256_min_pos = alloc::format!("{:?}", F256::MIN_POSITIVE);
            assert_eq!(F256::from_str(&s_f256_min_pos).unwrap().to_bits(), F256::MIN_POSITIVE.to_bits());

            // Direct coverage of dragon4_shortest heap tier (needed_limbs > 64) and > 1_000_000 None return,
            // plus round_up_digits carry-all-9s cases
            let mut d_out = [0u8; 256];
            let res_heap = super::super::dragon4_shortest::<16, 112>(
                &crate::Uint::<16>::ONE,
                false,
                -3000,
                true,
                &mut d_out,
            );
            assert!(res_heap.is_some());
            assert!(
                super::super::dragon4_shortest::<16, 112>(
                    &crate::Uint::<16>::ONE,
                    false,
                    -1_000_001,
                    true,
                    &mut d_out,
                )
                .is_none()
            );
            let mut empty_d: [u8; 0] = [];
            assert_eq!(super::super::round_up_digits(&mut empty_d), Some(b'1'));
            let mut nines = *b"999";
            assert_eq!(super::super::round_up_digits(&mut nines), Some(b'0'));
            assert_eq!(&nines, b"100");

            // Extreme exponent format (|e_eff| > 1_000_000) to exercise emergency shortest fallback
            // and fmt_fixed_approx_fallback (both prec == 0, prec > 0, drop_usize >= prec, < prec, prod_exp2 < 0)
            type HugeExpF = Float<8, 20>;
            let huge_pos = HugeExpF::from_str("1.23456e400000").unwrap();
            let s_huge_short = alloc::format!("{:?}", huge_pos);
            assert!(s_huge_short.contains('e'));
            let s_huge_fix0 = alloc::format!("{:.0?}", huge_pos);
            assert!(s_huge_fix0.starts_with("12345"));
            let s_huge_fix2 = alloc::format!("{:.2?}", huge_pos);
            assert!(s_huge_fix2.ends_with(".00"));

            let tiny_pos = HugeExpF::from_str("1.23456e-400000").unwrap();
            let s_tiny_short = alloc::format!("{:?}", tiny_pos);
            assert!(s_tiny_short.contains("e-"));
            let s_tiny_fix2 = alloc::format!("{:.2?}", tiny_pos);
            assert_eq!(s_tiny_fix2, "0.00");
        }
    }
}
