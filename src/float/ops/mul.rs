use super::Float;
use crate::Exponent;
use crate::Int;
use crate::Uint;
use crate::digits::Digits;
use crate::float::FloatExponent;
use crate::float::UnsignedFloatExponent;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(crate) fn mul_internal(self, rhs: Self, negative: bool) -> Self {
        let (a, b) = (self, rhs);
        let (_, exp_a, mant_a) = a.into_biased_parts();
        let (_, exp_b, mant_b) = b.into_biased_parts();
        
        let widened_mant_a = mant_a.to_digits::<u128>().grow::<1, W>(); // double the number of bits in mant_a, extending with zeros
        let widened_mant_b = mant_b.to_digits::<u128>().grow::<1, W>(); // double the number of bits in mant_b, extending with zeros
        let mant_prod = widened_mant_a.long_mul::<true>(widened_mant_b).0; // this will have twice as many bits as the original mantissas, so we can be sure that we won't lose any bits when multiplying

        let prod_bit_width = mant_prod.bit_width();

        if prod_bit_width == 0 {
            return if negative {
                Self::NEG_ZERO
            } else {
                Self::ZERO
            };
        }

        let extra_bits = if prod_bit_width > (Self::MB + 1) {
            prod_bit_width - (Self::MB + 1)
        } else {
            0
        };

        let mut exp = (exp_a as FloatExponent) - Self::EXP_BIAS
            + (exp_b as FloatExponent)
            + (extra_bits as FloatExponent)
            - (MB as FloatExponent);

        if exp > Self::MAX_EXP + Self::EXP_BIAS - 1 {
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        let mut extra_shift = 0;
        if !exp.is_positive() {
            extra_shift = (1 - exp) as UnsignedFloatExponent;
            exp = 1;
        }
        let total_shift = (extra_bits as UnsignedFloatExponent) + extra_shift;

        let tz = mant_prod.trailing_zeros();
        let sticky_bit = (tz as UnsignedFloatExponent + 1) < total_shift;
        let mut mant = match Exponent::try_from(total_shift - 1) {
            Ok(sub) => if sub >= 2 * Self::BITS {
                Digits::ALL_ZEROS
            } else {
                unsafe { mant_prod.unchecked_shr(sub, false) }
            }
            Err(_) => Digits::ALL_ZEROS,
        };
        if mant.bit(0) {
            if sticky_bit || mant.bit(1) {
                // Round up
                mant = mant.overflowing_add(Digits::ONE).0;
            }
        }
        mant = unsafe { mant.unchecked_shr(1, false) };

        let mut mant = mant.remove_tail().to_integer();
        if exp == 1 && mant.bit_width() < Self::MB + 1 {
            return Self::from_raw_parts(negative, 0, mant);
        }
        mant.set_bit(Self::MB, false); // ensure that the implicit bit is not set
        Self::from_raw_parts(negative, exp as UnsignedFloatExponent, mant)
    }

    #[inline]
    pub(super) fn mul(self, rhs: Self) -> Self {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => return Self::NAN,
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
            (_, _) => self.mul_internal(rhs, negative),
        }
    }
}
