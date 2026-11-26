use super::Float;
use crate::digits::Digits;
use crate::float::RoundingMode;
use crate::{Exponent, Int, Uint};

#[inline(always)]
const fn extract_bits<const W: usize>(m: &Uint<W>, low_bit: Exponent, count: Exponent) -> u8 {
    debug_assert!(count >= 1 && count <= 3);
    let byte_idx = (low_bit / 8) as usize;
    let bit_off = low_bit % 8;
    let lo = m.bytes[byte_idx] as u16;
    let hi = if byte_idx + 1 < W {
        m.bytes[byte_idx + 1] as u16
    } else {
        0
    };
    let val = ((hi << 8) | lo) >> bit_off;
    (val as u8) & ((1u8 << count) - 1)
}

#[inline(always)]
const fn wide_ge<const W: usize>(
    a: &Digits<u64, W, 2, 1>,
    b: &Digits<u64, W, 2, 1>,
    limbs: usize,
) -> bool {
    let mut i = limbs;
    while i > 0 {
        i -= 1;
        let av = a.get(i);
        let bv = b.get(i);
        if av != bv {
            return av > bv;
        }
    }
    true
}

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
const fn uint_from_low_u64<const W: usize>(val: u64) -> Uint<W> {
    let mut out = Uint::<W>::ZERO;
    let bytes = val.to_le_bytes();
    let mut i = 0;
    while i < W && i < 8 {
        out.bytes[i] = bytes[i];
        i += 1;
    }
    out
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes the cube root of `self` rounded according to `RM`.
    ///
    /// Valid for all `W > 1` and `2 < MB < Self::BITS - 2`, including formats with small exponent
    /// widths where the cube root of a subnormal number is itself subnormal, and formats with
    /// arbitrarily large exponent widths.
    pub(super) const fn cbrt_internal(self) -> Self {
        const {
            assert!(W > 1, "Float byte width W must be greater than 1");
            assert!(
                MB > 2 && MB < (Self::BITS as usize) - 2,
                "Float mantissa bits MB must satisfy 2 < MB < Self::BITS - 2"
            );
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }

        // Preserve NaN, ±infinity, and ±0.0 verbatim (matching IEEE 754 cbrt / rootn(x, 3)).
        if self.is_zero() || !self.is_finite() {
            return self;
        }

        let sign = self.is_sign_negative();
        let raw_exp = self.bits.bitand(Self::SIGN_MASK).shr(Self::MB);
        let raw_mant = self.bits.bitand(Self::MANTISSA_MASK);

        // Compute the exponent bias in `Uint<W>` directly so this works for any `EXPONENT_BITS`,
        // including formats where `EXPONENT_BITS >= 32`.
        let bias: Uint<W> = Uint::power_of_two(Self::EXPONENT_BITS - 1).sub(Uint::ONE);
        let bias_signed: Int<W> = bias.cast_signed();
        let min_normal_exp: Int<W> = Int::ONE.sub(bias_signed);

        // Normalise |self| = (m / 2^MB) * 2^e, with 2^MB <= m < 2^(MB + 1).
        let (e, m): (Int<W>, Uint<W>) = if raw_exp.is_zero() {
            let norm_shift = Self::MB + 1 - raw_mant.bit_width();
            let norm_mant = raw_mant.shl(norm_shift);
            let shift_int = Uint::cast_from_unsigned_float_exponent(norm_shift).cast_signed();
            (min_normal_exp.sub(shift_int), norm_mant)
        } else {
            let norm_mant = raw_mant.bitor(Self::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
            (raw_exp.cast_signed().sub(bias_signed), norm_mant)
        };

        // Euclidean division e = 3 * q_e + r with r in {0, 1, 2}.
        // Then |self|^(1/3) = ((m * 2^r) / 2^MB)^(1/3) * 2^(q_e),
        // where 1 <= ((m * 2^r) / 2^MB)^(1/3) < 2.
        let (q_e, r): (Int<W>, Exponent) = if !e.is_negative() {
            let (q, rem) = e.cast_unsigned().div_rem_u64(3);
            (q.cast_signed(), rem as Exponent)
        } else {
            let (q, rem) = e.unsigned_abs().div_rem_u64(3);
            if rem == 0 {
                (q.cast_signed().neg(), 0)
            } else {
                (q.add(Uint::ONE).cast_signed().neg(), (3 - rem) as Exponent)
            }
        };

        // Determine target fractional precision `s`:
        // - Normal output (q_e >= 1 - bias): s = MB.
        // - Subnormal output (q_e < 1 - bias): s = MB - ((1 - bias) - q_e), which can occur
        //   when EXPONENT_BITS is small relative to MB (specifically 2^EXPONENT_BITS < MB + 4).
        // Because 1 - bias - MB <= e < 0 in the subnormal case, q_e = floor(e / 3) > e,
        // so 1 <= s < MB always holds.
        let (is_subnormal, s): (bool, Exponent) = if q_e.lt(&min_normal_exp) {
            let diff = min_normal_exp
                .sub(q_e)
                .cast_unsigned()
                .cast_to_unsigned_float_exponent();
            (true, Self::MB - diff)
        } else {
            (false, Self::MB)
        };

        // Compute `Z = floor((8 * N)^(1/3))` together with exact remainder status (`rem_nz`)
        // and fractional approximation (`frac64`, used only by `Stochastic`).
        // - `y_trunc = Z >> 1` is `floor(N^(1/3))`.
        // - `round_bit = (Z & 1) != 0`: when true, `Z^3` is odd != `8 * N` (even), so `rem > 0`
        //   strictly (`N^(1/3) > y_trunc + 0.5`).
        // - When `round_bit == false`, `N^(1/3) == y_trunc` exactly iff `!rem_nz`, else
        //   `y_trunc < N^(1/3) < y_trunc + 0.5`.
        let (mut rounded_mant, round_bit, rem_nz, frac64) = if Self::MB <= 28 {
            Self::cbrt_mantissa_u64(m, r, s)
        } else if Self::MB <= 60 {
            Self::cbrt_mantissa_u128(m, r, s)
        } else {
            Self::cbrt_mantissa_wide(m, r, s)
        };

        let sticky = round_bit || rem_nz;
        let seed = uint_low_u64(&q_e.cast_unsigned()).wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ uint_low_u64(&rounded_mant);
        if RoundingMode::should_round_up::<RM>(
            sign,
            rounded_mant.is_odd(),
            round_bit,
            sticky,
            frac64,
            seed,
        ) {
            rounded_mant = rounded_mant.add(Uint::ONE);
        }

        let (biased_out_exp, mant_bits): (Uint<W>, Uint<W>) = if is_subnormal {
            if rounded_mant.bit(Self::MB) {
                // Rounded up across the subnormal-to-normal boundary to MIN_POSITIVE.
                (Uint::ONE, Uint::ZERO)
            } else {
                (Uint::ZERO, rounded_mant)
            }
        } else {
            let biased = q_e.add(bias_signed).cast_unsigned();
            if rounded_mant.bit(Self::MB + 1) {
                // Rounded up to the next power of two (2^(MB + 1) -> 1.0 * 2^(q_e + 1)).
                (biased.add(Uint::ONE), Uint::ZERO)
            } else {
                rounded_mant.set_bit(Self::MB, false);
                (biased, rounded_mant)
            }
        };

        let mut out_bits = biased_out_exp.shl(Self::MB).bitor(mant_bits);
        if sign {
            out_bits.set_bit(Self::BITS - 1, true);
        }
        Self::from_bits(out_bits)
    }

    /// Fast path for `MB <= 28`: all intermediates fit in `u64` (`2 * MB + 7 <= 63`).
    #[inline]
    const fn cbrt_mantissa_u64(m: Uint<W>, r: Exponent, s: Exponent) -> (Uint<W>, bool, bool, u64) {
        let m_val = uint_low_u64(&m);
        let mut rem_m_bits = Self::MB - r;
        let top_chunk = m_val >> rem_m_bits;

        let mut rem: u64 = top_chunk - 1;
        let mut y: u64 = 1;
        let mut t: u64 = 3; // invariant: t == 3 * y^2

        let mut k: Exponent = 0;
        while k <= s {
            let chunk: u64 = if rem_m_bits >= 3 {
                rem_m_bits -= 3;
                (m_val >> rem_m_bits) & 0b111
            } else if rem_m_bits > 0 {
                let c = (m_val & ((1u64 << rem_m_bits) - 1)) << (3 - rem_m_bits);
                rem_m_bits = 0;
                c
            } else {
                0
            };

            rem = (rem << 3) | chunk;
            y <<= 1;
            t <<= 2;
            let u = y * 3;
            let b = t + u + 1;
            if rem >= b {
                rem -= b;
                t = b + u + 2;
                y |= 1;
            }

            k += 1;
        }

        let round_bit = (y & 1) != 0;
        let rem_nz = rem != 0;
        let frac64 = if RM == RoundingMode::Stochastic as u8 {
            // Sub-half fraction ≈ rem / (3*y^2 + 3*y + 1) in [0, 1).
            let denom = (t as u128) + (y as u128) * 3 + 1;
            let sub_half = if rem == 0 {
                0u64
            } else {
                (((rem as u128) << 63) / denom) as u64
            };
            ((round_bit as u64) << 63) | sub_half
        } else {
            0
        };
        (uint_from_low_u64(y >> 1), round_bit, rem_nz, frac64)
    }

    /// Fast path for `MB <= 60`: all intermediates fit in `u128` (`2 * MB + 7 <= 127`).
    #[inline]
    const fn cbrt_mantissa_u128(m: Uint<W>, r: Exponent, s: Exponent) -> (Uint<W>, bool, bool, u64) {
        let m_val = uint_low_u64(&m);
        let mut rem_m_bits = Self::MB - r;
        let top_chunk = m_val >> rem_m_bits;

        let mut rem: u128 = (top_chunk - 1) as u128;
        let mut y: u64 = 1;
        let mut t: u128 = 3; // invariant: t == 3 * y^2

        let mut k: Exponent = 0;
        while k <= s {
            let chunk: u128 = if rem_m_bits >= 3 {
                rem_m_bits -= 3;
                ((m_val >> rem_m_bits) & 0b111) as u128
            } else if rem_m_bits > 0 {
                let c = (m_val & ((1u64 << rem_m_bits) - 1)) << (3 - rem_m_bits);
                rem_m_bits = 0;
                c as u128
            } else {
                0
            };

            rem = (rem << 3) | chunk;
            y <<= 1;
            t <<= 2;
            let u = (y as u128) * 3;
            let b = t + u + 1;
            if rem >= b {
                rem -= b;
                t = b + u + 2;
                y |= 1;
            }

            k += 1;
        }

        let round_bit = (y & 1) != 0;
        let rem_nz = rem != 0;
        let frac64 = if RM == RoundingMode::Stochastic as u8 {
            let denom = t + (y as u128) * 3 + 1;
            let shift = (128 - denom.leading_zeros()).saturating_sub(63);
            let d_hi = denom >> shift;
            let r_hi = rem >> shift;
            let sub_half = if d_hi == 0 {
                0u64
            } else {
                let q = (r_hi << 63) / d_hi;
                if q >= (1u128 << 63) {
                    (1u64 << 63) - 1
                } else {
                    q as u64
                }
            };
            ((round_bit as u64) << 63) | sub_half
        } else {
            0
        };
        (uint_from_low_u64(y >> 1), round_bit, rem_nz, frac64)
    }

    /// General multi-precision bit-by-bit cube root for arbitrary `W` and `MB`.
    ///
    /// At step `k` (1..=s+1):
    /// - `y` has exact bit-width `k + 1 <= MB + 2 <= 8 * W - 1`, fitting in `Digits<u64, W, 1, 0>`.
    /// - `rem`, `t = 3 * y^2`, and `b = 3 * y^2 + 3 * y + 1` are all strictly less than
    ///   `2^(2 * k + 5) <= 2^(2 * MB + 7) <= 2^(16 * W + 1)`, fitting in `Digits<u64, W, 2, 1>`.
    const fn cbrt_mantissa_wide(m: Uint<W>, r: Exponent, s: Exponent) -> (Uint<W>, bool, bool, u64) {
        let mut rem_m_bits = Self::MB - r;
        let top_chunk = extract_bits(&m, rem_m_bits, r + 1);

        let mut rem = Digits::<u64, W, 2, 1>::ALL_ZEROS;
        let mut t = Digits::<u64, W, 2, 1>::ALL_ZEROS;
        let mut b = Digits::<u64, W, 2, 1>::ALL_ZEROS;
        let mut y = Digits::<u64, W, 1, 0>::ALL_ZEROS;

        rem.set(0, (top_chunk - 1) as u64);
        t.set(0, 3);
        y.set(0, 1);

        let mut k: Exponent = 1;
        while k <= s + 1 {
            let chunk: u64 = if rem_m_bits >= 3 {
                rem_m_bits -= 3;
                extract_bits(&m, rem_m_bits, 3) as u64
            } else if rem_m_bits > 0 {
                let c = (extract_bits(&m, 0, rem_m_bits) << (3 - rem_m_bits)) as u64;
                rem_m_bits = 0;
                c
            } else {
                0
            };

            let y_limbs = (k as usize + 1).div_ceil(64);
            let w_limbs = (2 * (k as usize) + 5).div_ceil(64);

            // rem = (rem << 3) | chunk
            let mut carry = chunk;
            let mut i = 0;
            while i < w_limbs {
                let d = rem.get(i);
                rem.set(i, (d << 3) | carry);
                carry = d >> 61;
                i += 1;
            }
            debug_assert!(carry == 0);

            // y <<= 1
            let mut y_carry = 0u64;
            let mut i = 0;
            while i < y_limbs {
                let d = y.get(i);
                y.set(i, (d << 1) | y_carry);
                y_carry = d >> 63;
                i += 1;
            }
            debug_assert!(y_carry == 0);

            // t <<= 2
            let mut t_carry = 0u64;
            let mut i = 0;
            while i < w_limbs {
                let d = t.get(i);
                t.set(i, (d << 2) | t_carry);
                t_carry = d >> 62;
                i += 1;
            }
            debug_assert!(t_carry == 0);

            // b = t + 3 * y + 1
            let mut add_carry: u128 = 1;
            let mut i = 0;
            while i < w_limbs {
                let yi = if i < y_limbs { y.get(i) as u128 } else { 0 };
                let sum = (t.get(i) as u128) + yi * 3 + add_carry;
                b.set(i, sum as u64);
                add_carry = sum >> 64;
                i += 1;
            }
            debug_assert!(add_carry == 0);

            if wide_ge(&rem, &b, w_limbs) {
                // rem -= b
                let mut borrow = false;
                let mut i = 0;
                while i < w_limbs {
                    let (d1, b1) = rem.get(i).overflowing_sub(b.get(i));
                    let (d2, b2) = d1.overflowing_sub(borrow as u64);
                    rem.set(i, d2);
                    borrow = b1 || b2;
                    i += 1;
                }
                debug_assert!(!borrow);

                // t = b + 3 * y + 2  (== 3 * (y + 1)^2)
                let mut upd_carry: u128 = 2;
                let mut i = 0;
                while i < w_limbs {
                    let yi = if i < y_limbs { y.get(i) as u128 } else { 0 };
                    let sum = (b.get(i) as u128) + yi * 3 + upd_carry;
                    t.set(i, sum as u64);
                    upd_carry = sum >> 64;
                    i += 1;
                }
                debug_assert!(upd_carry == 0);

                // y |= 1
                y.set(0, y.get(0) | 1);
            }

            k += 1;
        }

        let z = Uint::<W>::from_digits(y);
        let round_bit = z.bit(0);
        let final_w_limbs = (2 * (s as usize + 1) + 5).div_ceil(64);
        let mut rem_nz = false;
        let mut i = 0;
        while i < final_w_limbs {
            if rem.get(i) != 0 {
                rem_nz = true;
                break;
            }
            i += 1;
        }
        let frac64 = if RM == RoundingMode::Stochastic as u8 {
            // Approximate rem / (3*z^2 + 3*z + 1) using top limbs of t and rem.
            let mut top = final_w_limbs;
            let mut sub_half = 0u64;
            while top > 0 {
                top -= 1;
                let tv = t.get(top);
                if tv != 0 {
                    let shift = tv.leading_zeros();
                    let t_hi = if shift == 0 || top == 0 {
                        tv
                    } else {
                        (tv << shift) | (t.get(top - 1) >> (64 - shift))
                    };
                    let r_hi = if shift == 0 || top == 0 {
                        rem.get(top)
                    } else {
                        (rem.get(top) << shift) | (rem.get(top - 1) >> (64 - shift))
                    };
                    let d = (t_hi as u128) >> 1;
                    let n = (r_hi as u128) >> 1;
                    if d != 0 {
                        let q = (n << 63) / d;
                        sub_half = if q >= (1u128 << 63) {
                            (1u64 << 63) - 1
                        } else {
                            q as u64
                        };
                    }
                    break;
                }
            }
            ((round_bit as u64) << 63) | sub_half
        } else {
            0
        };
        (z.shr(1), round_bit, rem_nz, frac64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{F16, F32, F64, F80, F128, F256};

    /// Exact big-integer oracle verifying that `y = x.cbrt_internal()` is the unique
    /// IEEE 754 round-to-nearest-ties-to-even cube root of `x` for arbitrary `Float<W, MB>`.
    ///
    /// Uses `Uint<BIG>` (with enough bits to hold `(4 * 2^MB)^3` plus exponent alignment)
    /// to verify `m_lo^3 < |x| < m_hi^3` independently of the cube-root algorithm.
    fn assert_correctly_rounded<const W: usize, const MB: usize, const BIG: usize>(
        x: Float<W, MB>,
    ) {
        let y = x.cbrt_internal();

        if x.is_nan() {
            assert!(y.is_nan(), "cbrt(NaN) should be NaN");
            assert_eq!(y.to_bits(), x.to_bits());
            return;
        }
        if x.is_zero() || x.is_infinite() {
            assert_eq!(y.to_bits(), x.to_bits());
            return;
        }

        assert!(y.is_finite() && !y.is_zero());
        assert_eq!(
            y.is_sign_negative(),
            x.is_sign_negative(),
            "sign mismatch for x = {:?}",
            x.to_bits()
        );

        let bias: Uint<W> = Uint::power_of_two(Float::<W, MB>::EXPONENT_BITS - 1).sub(Uint::ONE);
        let bias_s: Int<W> = bias.cast_signed();
        let min_normal_exp: Int<W> = Int::ONE.sub(bias_s);
        let mb_int: Int<W> = Uint::cast_from_unsigned_float_exponent(MB as Exponent).cast_signed();

        // Extract exact integer representation v = M * 2^exp_lsb (where exp_lsb is Int<W>).
        let extract = |f: Float<W, MB>| -> (Uint<W>, Int<W>, bool) {
            let raw_exp = f.bits.bitand(Float::<W, MB>::SIGN_MASK).shr(MB as Exponent);
            let raw_mant = f.bits.bitand(Float::<W, MB>::MANTISSA_MASK);
            if raw_exp.is_zero() {
                (raw_mant, min_normal_exp.sub(mb_int), false)
            } else {
                let m = raw_mant.bitor(Float::<W, MB>::MANTISSA_IMPLICIT_LEADING_ONE_MASK);
                let is_gt_min_normal_pow2 = raw_mant.is_zero() && raw_exp.gt(&Uint::ONE);
                (
                    m,
                    raw_exp.cast_signed().sub(bias_s).sub(mb_int),
                    is_gt_min_normal_pow2,
                )
            }
        };

        let (mx_w, ex, _) = extract(x);
        let (my_w, ey, y_is_gt_min_normal_pow2) = extract(y);

        let widen = |u: Uint<W>| -> Uint<BIG> {
            let mut out = Uint::<BIG>::ZERO;
            let mut i = 0;
            while i < W {
                out.bytes[i] = u.bytes[i];
                i += 1;
            }
            out
        };

        let mx = widen(mx_w);
        let my = widen(my_w);

        // Compare exact rational `(num_a * 2^exp_a)` against `(num_b * 2^exp_b)`
        // where `|exp_a - exp_b|` is small (at most ~3*MB + 10).
        let cmp_scaled =
            |num_a: Uint<BIG>, exp_a: Int<W>, num_b: Uint<BIG>, exp_b: Int<W>| -> core::cmp::Ordering {
                if exp_a.ge(&exp_b) {
                    let shift = exp_a
                        .sub(exp_b)
                        .cast_unsigned()
                        .cast_to_unsigned_float_exponent();
                    num_a.shl(shift).cmp(&num_b)
                } else {
                    let shift = exp_b
                        .sub(exp_a)
                        .cast_unsigned()
                        .cast_to_unsigned_float_exponent();
                    num_a.cmp(&num_b.shl(shift))
                }
            };

        let three: Int<W> = Uint::cast_from_unsigned_float_exponent(3).cast_signed();
        let two: Int<W> = Uint::cast_from_unsigned_float_exponent(2).cast_signed();

        // Upper midpoint m_hi = (2 * my + 1) * 2^(ey - 1)
        // m_hi^3 = (2 * my + 1)^3 * 2^(3 * (ey - 1))
        let hi_num = my.shl(1).add(Uint::ONE);
        let hi_cube = hi_num.mul(hi_num).mul(hi_num);
        let hi_exp = ey.sub(Int::ONE).mul(three);

        assert!(
            cmp_scaled(mx, ex, hi_cube, hi_exp).is_lt(),
            "y = {:?} is too small for x = {:?}",
            y.to_bits(),
            x.to_bits()
        );

        // Lower midpoint m_lo:
        // - If y is a normal power of 2 strictly greater than MIN_POSITIVE:
        //   m_lo = (4 * my - 1) * 2^(ey - 2)
        // - Otherwise:
        //   m_lo = (2 * my - 1) * 2^(ey - 1)
        let (lo_num, lo_exp) = if y_is_gt_min_normal_pow2 {
            (my.shl(2).sub(Uint::ONE), ey.sub(two).mul(three))
        } else {
            (my.shl(1).sub(Uint::ONE), ey.sub(Int::ONE).mul(three))
        };
        let lo_cube = lo_num.mul(lo_num).mul(lo_num);

        assert!(
            cmp_scaled(lo_cube, lo_exp, mx, ex).is_lt(),
            "y = {:?} is too large for x = {:?}",
            y.to_bits(),
            x.to_bits()
        );
    }

    /// Helper verifying that `cbrt_mantissa_wide` agrees bit-for-bit with the fast paths.
    fn assert_kernels_agree<const W: usize, const MB: usize>(x: Float<W, MB>) {
        if x.is_zero() || !x.is_finite() {
            return;
        }
        let bias: Uint<W> = Uint::power_of_two(Float::<W, MB>::EXPONENT_BITS - 1).sub(Uint::ONE);
        let bias_signed = bias.cast_signed();
        let min_normal_exp = Int::ONE.sub(bias_signed);
        let raw_exp = x.bits.bitand(Float::<W, MB>::SIGN_MASK).shr(MB as Exponent);
        let raw_mant = x.bits.bitand(Float::<W, MB>::MANTISSA_MASK);
        let (e, m) = if raw_exp.is_zero() {
            let norm_shift = MB as Exponent + 1 - raw_mant.bit_width();
            (
                min_normal_exp.sub(Uint::cast_from_unsigned_float_exponent(norm_shift).cast_signed()),
                raw_mant.shl(norm_shift),
            )
        } else {
            (
                raw_exp.cast_signed().sub(bias_signed),
                raw_mant.bitor(Float::<W, MB>::MANTISSA_IMPLICIT_LEADING_ONE_MASK),
            )
        };
        let (q_e, r) = if !e.is_negative() {
            let (q, rem) = e.cast_unsigned().div_rem_u64(3);
            (q.cast_signed(), rem as Exponent)
        } else {
            let (q, rem) = e.unsigned_abs().div_rem_u64(3);
            if rem == 0 {
                (q.cast_signed().neg(), 0)
            } else {
                (q.add(Uint::ONE).cast_signed().neg(), (3 - rem) as Exponent)
            }
        };
        let s = if q_e.lt(&min_normal_exp) {
            let diff = min_normal_exp
                .sub(q_e)
                .cast_unsigned()
                .cast_to_unsigned_float_exponent();
            MB as Exponent - diff
        } else {
            MB as Exponent
        };

        let wide = Float::<W, MB>::cbrt_mantissa_wide(m, r, s);
        if MB <= 28 {
            assert_eq!(wide, Float::<W, MB>::cbrt_mantissa_u64(m, r, s));
        }
        if MB <= 60 {
            assert_eq!(wide, Float::<W, MB>::cbrt_mantissa_u128(m, r, s));
        }
    }

    #[test]
    fn exhaustive_w2_formats() {
        // Test ALL 65,536 bit patterns for extreme and standard 16-bit formats:
        // - Float<2, 3>:  minimum MB (3), EXPONENT_BITS = 12
        // - Float<2, 10>: IEEE binary16 (F16), EXPONENT_BITS = 5
        // - Float<2, 12>: EXPONENT_BITS = 3 (subnormal outputs occur!)
        // - Float<2, 13>: maximum MB (13), EXPONENT_BITS = 2 (many subnormal outputs + subnormal->normal rounding!)
        for bits in 0u16..=u16::MAX {
            let u = Uint::<2>::from_le_bytes(bits.to_le_bytes());

            let f_min = Float::<2, 3>::from_bits(u);
            assert_correctly_rounded::<2, 3, 16>(f_min);
            assert_kernels_agree(f_min);

            let f16 = F16::from_bits(u);
            assert_correctly_rounded::<2, 10, 16>(f16);
            assert_kernels_agree(f16);

            let f_e3 = Float::<2, 12>::from_bits(u);
            assert_correctly_rounded::<2, 12, 16>(f_e3);
            assert_kernels_agree(f_e3);

            let f_max = Float::<2, 13>::from_bits(u);
            assert_correctly_rounded::<2, 13, 16>(f_max);
            assert_kernels_agree(f_max);
        }
    }

    #[test]
    fn special_and_exact_values() {
        // F64 exact cubes and boundary values
        let exact_cases_f64: &[(f64, f64)] = &[
            (0.0, 0.0),
            (-0.0, -0.0),
            (1.0, 1.0),
            (-1.0, -1.0),
            (8.0, 2.0),
            (-8.0, -2.0),
            (27.0, 3.0),
            (-27.0, -3.0),
            (64.0, 4.0),
            (0.125, 0.5),
            (-0.125, -0.5),
            (1e300, 1e100),
            (-1e300, -1e100),
            (f64::INFINITY, f64::INFINITY),
            (f64::NEG_INFINITY, f64::NEG_INFINITY),
            (f64::from_bits(1), 2.0_f64.powi(-358)),
            // Value just below 8.0 whose cube root rounds up to 2.0
            (f64::from_bits(8.0_f64.to_bits() - 1), 2.0),
        ];

        for &(input, expected) in exact_cases_f64 {
            let f = F64::from(input);
            let res = f.cbrt_internal();
            assert_correctly_rounded::<8, 52, 64>(f);
            assert_kernels_agree(f);
            assert_eq!(
                res.to_bits(),
                F64::from(expected).to_bits(),
                "mismatch on f64 input {input:?}"
            );
        }

        // Inexact boundary values: `assert_correctly_rounded` proves exact 0.5-ULP rounding;
        // platform libm `cbrt` (e.g., glibc on x86_64) can drift by up to a few ULPs.
        for input in [
            f64::MIN_POSITIVE,
            f64::MAX,
            f64::MIN,
            f64::from_bits(0x000f_ffff_ffff_ffff),
        ] {
            let f = F64::from(input);
            let res = f.cbrt_internal();
            assert_correctly_rounded::<8, 52, 64>(f);
            assert_kernels_agree(f);
            let ours = u64::from_le_bytes(res.to_bits().to_bytes());
            let theirs = input.cbrt().to_bits();
            assert!(
                ours.abs_diff(theirs) <= 4,
                "cbrt ULP diff > 4 vs std on {input:?}: ours={ours:#x}, std={theirs:#x}"
            );
        }

        // NaN preserves NaN
        assert!(F64::NAN.cbrt_internal().is_nan());
        assert!(F64::NEG_NAN.cbrt_internal().is_nan());
        assert!(F32::NAN.cbrt_internal().is_nan());

        // Const evaluation check
        const CONST_CBRT_27: F64 = {
            let bits = Uint::<8>::from_le_bytes(27.0_f64.to_bits().to_le_bytes());
            F64::from_bits(bits).cbrt_internal()
        };
        assert_eq!(
            CONST_CBRT_27.to_bits(),
            Uint::<8>::from_le_bytes(3.0_f64.to_bits().to_le_bytes())
        );
    }

    #[test]
    fn f32_and_f64_boundary_and_random_oracle() {
        // Deterministic PRNG (splitmix64) so we get broad deterministic coverage
        let mut state = 0x1234_5678_9abc_def0_u64;
        let mut next_u64 = || -> u64 {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };

        for _ in 0..5_000 {
            let r = next_u64();

            let f32_val = F32::from_bits(Uint::<4>::from_le_bytes((r as u32).to_le_bytes()));
            assert_correctly_rounded::<4, 23, 32>(f32_val);
            assert_kernels_agree(f32_val);
            let std_f32 = f32::from_bits(r as u32);
            if !std_f32.is_nan() {
                let ours = u32::from_le_bytes(f32_val.cbrt_internal().to_bits().to_bytes());
                let theirs = std_f32.cbrt().to_bits();
                assert!(ours.abs_diff(theirs) <= 4);
            }

            // Also test subnormal F32
            let sub32 = F32::from_bits(Uint::<4>::from_le_bytes(((r as u32) & 0x807f_ffff).to_le_bytes()));
            assert_correctly_rounded::<4, 23, 32>(sub32);

            let f64_val = F64::from_bits(Uint::<8>::from_le_bytes(r.to_le_bytes()));
            assert_correctly_rounded::<8, 52, 64>(f64_val);
            assert_kernels_agree(f64_val);
            let std_f64 = f64::from_bits(r);
            if !std_f64.is_nan() {
                let ours = u64::from_le_bytes(f64_val.cbrt_internal().to_bits().to_bytes());
                let theirs = std_f64.cbrt().to_bits();
                assert!(ours.abs_diff(theirs) <= 4);
            }

            // Also test subnormal F64
            let sub64 = F64::from_bits(Uint::<8>::from_le_bytes(
                (r & 0x800f_ffff_ffff_ffff).to_le_bytes(),
            ));
            assert_correctly_rounded::<8, 52, 64>(sub64);
        }
    }

    #[test]
    fn diverse_widths_and_mantissa_bits() {
        let mut state = 0xfedc_ba98_7654_3210_u64;
        let mut next_u64 = || -> u64 {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };

        macro_rules! test_format {
            ($W: literal, $MB: literal, $BIG: literal, $iters: literal) => {
                // Key boundary values for this format
                let all_ones_mant = Float::<$W, $MB>::MANTISSA_MASK;
                let max_normal_exp = Float::<$W, $MB>::INFINITY
                    .to_bits()
                    .sub(Uint::power_of_two($MB as Exponent));
                // Also construct a value with r = 2 and all-ones mantissa (triggers power-of-2
                // carry AND achieves the maximum possible intermediate values of rem, t, b, y).
                let bias: Uint<$W> = Uint::power_of_two(Float::<$W, $MB>::EXPONENT_BITS - 1).sub(Uint::ONE);
                let exp_r2_val = if Float::<$W, $MB>::EXPONENT_BITS >= 3 {
                    // unbiased exponent +2 => r = 2, normal output
                    bias.add(Uint::cast_from_unsigned_float_exponent(2))
                } else {
                    // EXPONENT_BITS = 2: unbiased exponent -1 (subnormal, r = 2) is already MAX_SUBNORMAL
                    Uint::ONE
                };
                let max_r2_carry = Float::<$W, $MB>::from_bits(
                    exp_r2_val.shl($MB as Exponent).bitor(all_ones_mant),
                );
                let boundaries: [Float<$W, $MB>; 11] = [
                    Float::ZERO,
                    Float::NEG_ZERO,
                    Float::INFINITY,
                    Float::NEG_INFINITY,
                    Float::NAN,
                    Float::from_bits(Uint::ONE),                       // MIN_POSITIVE_SUBNORMAL
                    Float::from_bits(all_ones_mant),                   // MAX_SUBNORMAL
                    Float::from_bits(Uint::power_of_two($MB as Exponent)), // MIN_POSITIVE
                    Float::from_bits(max_normal_exp.bitor(all_ones_mant)), // MAX
                    Float::from_bits(max_normal_exp.bitor(all_ones_mant)).neg(), // MIN
                    max_r2_carry,
                ];
                for b in boundaries {
                    assert_correctly_rounded::<$W, $MB, $BIG>(b);
                    assert_kernels_agree(b);
                }

                // Random normal and subnormal values
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
                    // Every 3rd iteration, force subnormal to exercise subnormal paths thoroughly
                    if iter % 3 == 0 {
                        let sign_bit = u.bit(Float::<$W, $MB>::BITS - 1);
                        u = u.bitand(Float::<$W, $MB>::MANTISSA_MASK);
                        if sign_bit {
                            u.set_bit(Float::<$W, $MB>::BITS - 1, true);
                        }
                    }
                    let f = Float::<$W, $MB>::from_bits(u);
                    assert_correctly_rounded::<$W, $MB, $BIG>(f);
                    if $MB <= 60 {
                        assert_kernels_agree(f);
                    }
                }
            };
        }

        // Odd byte widths and boundary MB values
        test_format!(3, 3, 16, 400);
        test_format!(3, 15, 24, 400);
        test_format!(3, 21, 24, 400); // EXPONENT_BITS = 2

        // Fast path boundary MB = 28 (u64) and MB = 29 (u128, EXPONENT_BITS = 2)
        test_format!(4, 3, 24, 400);
        test_format!(4, 28, 32, 400);
        test_format!(4, 29, 32, 400);

        // EXPONENT_BITS = 32 (exceeds i32 FloatExponent!) and EXPONENT_BITS = 31
        test_format!(5, 7, 24, 400);
        test_format!(5, 8, 24, 400);

        // W = 8..=12: exercise all four odd byte-length residue classes of (2*W + 1) mod 8
        // at maximum MB = 8*W - 3 (EXPONENT_BITS = 2), plus EXPONENT_BITS = 60 and MB = 60/61 boundary
        test_format!(8, 3, 24, 400);
        test_format!(8, 60, 64, 400);
        test_format!(8, 61, 64, 400);
        test_format!(9, 69, 64, 300);
        test_format!(10, 64, 64, 400); // F80
        test_format!(10, 77, 72, 300);
        test_format!(11, 85, 80, 300);
        test_format!(12, 93, 88, 300);

        // Quad/octuple and large custom widths
        test_format!(16, 3, 32, 300);  // EXPONENT_BITS = 124!
        test_format!(16, 112, 96, 300); // F128
        test_format!(16, 125, 96, 300); // EXPONENT_BITS = 2!
        test_format!(32, 236, 160, 150); // F256
        test_format!(32, 253, 192, 150); // EXPONENT_BITS = 2!
        test_format!(64, 500, 320, 40);  // 512-bit float!
    }

    #[test]
    fn f80_f128_f256_exact_cubes() {
        // Verify exact cube roots of 1.0, 8.0, 27.0, -27.0 on F80, F128, F256
        macro_rules! check_pow8 {
            ($Ty: ty, $BIG: literal) => {
                // 8.0 -> 2.0, -8.0 -> -2.0, 1.0 -> 1.0, 0.125 -> 0.5
                assert_eq!(<$Ty>::ONE.cbrt_internal().to_bits(), <$Ty>::ONE.to_bits());
                assert_eq!(
                    <$Ty>::NEG_ONE.cbrt_internal().to_bits(),
                    <$Ty>::NEG_ONE.to_bits()
                );
                let eight = <$Ty>::normal_power_of_two(3);
                assert_eq!(eight.cbrt_internal().to_bits(), <$Ty>::TWO.to_bits());
                assert_eq!(
                    eight.neg().cbrt_internal().to_bits(),
                    <$Ty>::TWO.neg().to_bits()
                );
                let eighth = <$Ty>::normal_power_of_two(-3);
                assert_eq!(eighth.cbrt_internal().to_bits(), <$Ty>::HALF.to_bits());
            };
        }

        check_pow8!(F80, 64);
        check_pow8!(F128, 96);
        check_pow8!(F256, 160);
    }
}
