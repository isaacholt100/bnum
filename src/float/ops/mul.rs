use super::Float;
use crate::digits::Digits;
use crate::float::RoundingMode;
use crate::{Exponent, Int, Uint};
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

#[inline]
const fn prod_frac64<const W: usize>(prod: &Digits<u128, W, 1, W>, shift: Exponent) -> u64 {
    if shift == 0 {
        0
    } else if shift <= 64 {
        let lo = prod.get(0) as u64;
        if shift == 64 {
            lo
        } else {
            (lo & ((1u64 << shift) - 1)) << (64 - shift)
        }
    } else {
        let shifted = unsafe { prod.unchecked_shr(shift - 64, false) };
        shifted.get(0) as u64
    }
}

impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Multiplies two finite non-zero floats using rounding mode `RM`.
    #[inline]
    pub(crate) const fn mul_internal(self, rhs: Self, negative: bool) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
            assert!(RM <= RoundingMode::MAX_U8, "invalid RoundingMode discriminant");
        }

        let (_, exp_a, mant_a) = self.into_biased_uint_parts();
        let (_, exp_b, mant_b) = rhs.into_biased_uint_parts();

        let widened_a = mant_a.to_digits::<u128>().grow::<1, W>();
        let widened_b = mant_b.to_digits::<u128>().grow::<1, W>();
        let mant_prod = widened_a.long_mul::<true>(widened_b).0;

        let prod_bw = mant_prod.bit_width();
        if prod_bw == 0 {
            return if negative {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            };
        }

        let extra_bits: Exponent = if prod_bw > Self::MB + 1 {
            prod_bw - (Self::MB + 1)
        } else {
            0
        };

        let bias = Self::exp_bias_uint().cast_signed();
        let exp_max = Self::exp_max_uint().cast_signed();

        let mut exp: Int<W> = exp_a
            .cast_signed()
            .sub(bias)
            .add(exp_b.cast_signed())
            .add(Uint::cast_from_unsigned_float_exponent(extra_bits).cast_signed())
            .sub(Uint::cast_from_unsigned_float_exponent(Self::MB).cast_signed());

        if exp.ge(&exp_max) {
            return Self::overflow_value(negative);
        }

        let extra_shift: Exponent = if exp.le(&Int::ZERO) {
            let sub_drop = Int::<W>::ONE.sub(exp);
            let max_useful =
                Uint::<W>::cast_from_unsigned_float_exponent(prod_bw - extra_bits).cast_signed();
            if sub_drop.gt(&max_useful) {
                return Self::underflow_nonzero(negative);
            }
            exp = Int::ONE;
            sub_drop.cast_unsigned().cast_to_unsigned_float_exponent() as Exponent
        } else {
            0
        };

        let total_shift = extra_bits + extra_shift;
        if total_shift == 0 {
            let mant = mant_prod.remove_tail().to_integer();
            return Self::from_biased_uint_parts(negative, exp.cast_unsigned(), mant);
        }

        // total_shift is guaranteed to satisfy 1 <= total_shift <= prod_bw < 2 * BITS.
        let round_bit = mant_prod.bit(total_shift - 1);
        let sticky = mant_prod.trailing_zeros() < total_shift - 1;
        let frac64 = if RM == RoundingMode::Stochastic as u8 {
            prod_frac64(&mant_prod, total_shift)
        } else {
            0
        };
        let mut mant: Uint<W> = if total_shift >= prod_bw {
            Uint::ZERO
        } else {
            unsafe { mant_prod.unchecked_shr(total_shift, false) }
                .remove_tail()
                .to_integer()
        };

        let seed = uint_low_u64(&exp.cast_unsigned())
            .wrapping_mul(0x9e37_79b9_7f4a_7c15)
            ^ uint_low_u64(&mant);
        if RoundingMode::should_round_up::<RM>(
            negative,
            mant.is_odd(),
            round_bit,
            sticky,
            frac64,
            seed,
        ) {
            mant = mant.add(Uint::ONE);
            if mant.bit(Self::MB + 1) {
                mant = mant.shr(1);
                exp = exp.add(Int::ONE);
            }
        }

        if exp.ge(&exp_max) {
            return Self::overflow_value(negative);
        }

        Self::from_biased_uint_parts(negative, exp.cast_unsigned(), mant)
    }

    #[inline]
    pub(crate) const fn mul(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => Self::NAN,
            (FpCategory::Infinite, FpCategory::Zero) | (FpCategory::Zero, FpCategory::Infinite) => {
                Self::NAN
            }
            (FpCategory::Infinite, _) | (_, FpCategory::Infinite) => {
                if negative {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                }
            }
            (FpCategory::Zero, _) | (_, FpCategory::Zero) => {
                if negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (_, _) => self.mul_internal(rhs, negative),
        }
    }
}
