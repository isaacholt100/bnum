use super::Float;
use crate::float::RoundingMode;
use crate::{Exponent, Int, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

#[inline]
const fn uint_low_u64<const W: usize>(u: &Uint<W>) -> u64 {
    let mut v = 0u64;
    let mut i = 0;
    while i < W && i < 8 {
        v |= (u.bytes[i] as u64) << (i * 8);
        i += 1;
    }
    v
}

/// Approximates `floor((num / den) * 2^64)` for `0 <= num < den` (used only by `Stochastic`).
#[inline]
const fn ratio_frac64<const W: usize>(num: &Uint<W>, den: &Uint<W>) -> u64 {
    if num.is_zero() {
        return 0;
    }
    let den_bw = den.bit_width();
    let shift = if den_bw > 63 { den_bw - 63 } else { 0 };
    let d_top = uint_low_u64(&den.shr(shift)) as u128;
    let n_top = uint_low_u64(&num.shr(shift)) as u128;
    if d_top == 0 {
        return 0;
    }
    let q = (n_top << 64) / d_top;
    if q > u64::MAX as u128 {
        u64::MAX
    } else {
        q as u64
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Divides two finite non-zero floats using rounding mode `RM`.
    #[inline]
    pub(crate) const fn div_internal(self, rhs: Self, negative: bool) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }

        let (_, e1, s1) = self.into_biased_uint_parts();
        let (_, e2, s2) = rhs.into_biased_uint_parts();

        let b1 = s1.bit_width();
        let b2 = s2.bit_width();
        let sh1 = (Self::MB + 1) - b1;
        let sh2 = (Self::MB + 1) - b2;

        let mut na = if sh1 != 0 { s1.shl(sh1) } else { s1 };
        let nb = if sh2 != 0 { s2.shl(sh2) } else { s2 };

        let bias = Self::exp_bias_uint().cast_signed();
        let exp_max = Self::exp_max_uint().cast_signed();

        let mut e: Int<W> = e1
            .cast_signed()
            .sub(e2.cast_signed())
            .add(bias)
            .add(Uint::cast_from_unsigned_float_exponent(b1).cast_signed())
            .sub(Uint::cast_from_unsigned_float_exponent(b2).cast_signed());

        if na.lt(&nb) {
            na = na.shl(1);
            e = e.sub(Int::ONE);
        }

        // Now 1 <= na / nb < 2 and exact quotient is (na / nb) * 2^(e - bias).
        if e.ge(&exp_max) {
            return Self::overflow_value(negative);
        }

        let left_shift: Exponent = if e.le(&Int::ZERO) {
            let sub_drop = Int::<W>::ONE.sub(e);
            let mb_plus_1 =
                Uint::<W>::cast_from_unsigned_float_exponent(Self::MB + 1).cast_signed();
            match sub_drop.cmp(&mb_plus_1) {
                Ordering::Greater => {
                    return Self::underflow_nonzero(negative);
                }
                Ordering::Equal => {
                    // Exact value = na / (2 * nb) in [0.5, 1.0) ULPs of MIN_POSITIVE_SUBNORMAL.
                    let sticky = na.gt(&nb);
                    let frac64 = if RM == RoundingMode::Stochastic as u8 {
                        let extra = na.sub(nb);
                        (1u64 << 63) | (ratio_frac64(&extra, &nb) >> 1)
                    } else {
                        1u64 << 63
                    };
                    let seed = uint_low_u64(&na) ^ uint_low_u64(&nb);
                    return if RoundingMode::should_round_up::<RM>(
                        negative,
                        false,
                        true,
                        sticky,
                        frac64,
                        seed,
                    ) {
                        Self::from_biased_uint_parts(negative, Uint::ONE, Uint::ONE)
                    } else if negative {
                        Self::NEG_ZERO
                    } else {
                        Self::ZERO
                    };
                }
                Ordering::Less => {
                    let drop =
                        sub_drop.cast_unsigned().cast_to_unsigned_float_exponent() as Exponent;
                    e = Int::ONE;
                    Self::MB - drop
                }
            }
        } else {
            Self::MB
        };

        let widened_na = na.to_digits::<u64>().grow::<1, W>();
        let widened_nb = nb.to_digits::<u64>().grow::<1, W>();
        let num = if left_shift == 0 {
            widened_na
        } else {
            unsafe { widened_na.unchecked_shl(left_shift) }
        };

        let (mut q, r): (Uint<W>, Uint<W>) = match num.cmp(&widened_nb) {
            Ordering::Less => (Uint::ZERO, na),
            Ordering::Equal => (Uint::ONE, Uint::ZERO),
            Ordering::Greater => {
                let (dq, dr) = num.div_rem_unchecked(widened_nb);
                (dq.remove_tail().to_integer(), dr.remove_tail().to_integer())
            }
        };

        let twice_r = r.shl(1);
        let (round_bit, sticky) = match twice_r.cmp(&nb) {
            Ordering::Greater => (true, true),
            Ordering::Equal => (true, false),
            Ordering::Less => (false, !r.is_zero()),
        };
        let frac64 = if RM == RoundingMode::Stochastic as u8 {
            ratio_frac64(&r, &nb)
        } else {
            0
        };
        let seed = uint_low_u64(&e.cast_unsigned())
            .wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ uint_low_u64(&q);

        if RoundingMode::should_round_up::<RM>(
            negative,
            q.is_odd(),
            round_bit,
            sticky,
            frac64,
            seed,
        ) {
            q = q.add(Uint::ONE);
        }

        if q.bit(Self::MB + 1) {
            q = q.shr(1);
            e = e.add(Int::ONE);
        }

        if e.ge(&exp_max) {
            return Self::overflow_value(negative);
        }

        Self::from_biased_uint_parts(negative, e.cast_unsigned(), q)
    }

    #[inline]
    pub(crate) const fn div(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => Self::NAN,
            (FpCategory::Infinite, FpCategory::Infinite) => Self::NAN,
            (FpCategory::Zero, FpCategory::Zero) => Self::NAN,
            (FpCategory::Infinite, _) | (_, FpCategory::Zero) => {
                if negative {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                }
            }
            (FpCategory::Zero, _) | (_, FpCategory::Infinite) => {
                if negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (_, _) => self.div_internal(rhs, negative),
        }
    }
}
