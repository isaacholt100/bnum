use super::Float;
use crate::float::FloatExponent;
use crate::float::UnsignedFloatExponent;
use crate::BIntD8;
use crate::BUintD8;
use crate::ExpType;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(crate) fn mul_internal(self, rhs: Self, negative: bool) -> Self {
        let (a, b) = (self, rhs);
        let (_, exp_a, mant_a) = a.into_biased_parts();
        let (_, exp_b, mant_b) = b.into_biased_parts();

        // TODO: make so as_ can infer type so can switch trait definition if needed
        let mut mant_prod = mant_a.widening_mul(mant_b);

        let prod_bits = if mant_prod.1.bits() == 0 {
            mant_prod.0.bits()
        } else {
            mant_prod.1.bits() + Self::BITS
        };

        if prod_bits == 0 {
            return if negative { Self::NEG_ZERO } else { Self::ZERO };
        }

        let extra_bits = if prod_bits > (Self::MB + 1) {
            prod_bits - (Self::MB + 1)
        } else {
            0
        };

        let mut exp =
            (exp_a as FloatExponent) - Self::EXP_BIAS + (exp_b as FloatExponent) + (extra_bits as FloatExponent)
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

        let mp0tz = mant_prod.0.trailing_zeros();
        let tz = if mp0tz == Self::BITS {
            mant_prod.1.trailing_zeros() + Self::BITS
        } else {
            mp0tz
        };

        let sticky_bit = (tz as UnsignedFloatExponent + 1) < total_shift;
        let mut mant = match ExpType::try_from(total_shift - 1) {
            Ok(sub) => {
                if sub > Self::BITS * 2 {
                    (BUintD8::ZERO, BUintD8::ZERO)
                } else if sub >= Self::BITS {
                    (mant_prod.1 >> (sub - Self::BITS), BUintD8::ZERO)
                } else {
                    let mask = BUintD8::MAX >> (Self::BITS - sub);
                    let carry = mant_prod.1 & mask;
                    mant_prod.1 >>= sub;
                    mant_prod.0 = (mant_prod.0 >> sub) | (carry << (Self::BITS - sub));
                    mant_prod
                }
            }
            Err(_) => (BUintD8::ZERO, BUintD8::ZERO),
        };
        if mant.0.bit(0) {
            if sticky_bit || mant.0.bit(1) {
                // Round up
                let (sum, carry) = mant.0.overflowing_add(BUintD8::ONE);
                mant.0 = sum;
                if carry {
                    mant.1 += BUintD8::ONE;
                }
            }
        }
        {
            let carry = mant.1.bit(0);
            //mant.1 >>= 1 as ExpType;
            mant.0 >>= 1 as ExpType;
            if carry {
                mant.0 |= BIntD8::MIN.to_bits();
            }
        }

        let mut m1b = mant.1.bits();
        if m1b != 0 {
            m1b -= 1;
        }
        /*let bits = if m1b == 0 {
            mant.0.bits()
        } else {
            m1b + Self::BITS
        };*/
        let m0b = mant.0.bits();
        if m0b > Self::MB + 1 {
            // it's possible that the mantissa has too many bits, so shift it right and increase the exponent until it has the correct number of bits
            let inc = m0b - (Self::MB + 1);
            mant.0 = mant.0 >> inc;
            exp += inc as FloatExponent;
        }

        if exp > Self::MAX_EXP + Self::EXP_BIAS - 1 {
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        if exp == 1 && m1b < Self::MB + 1 {
            return Self::from_raw_parts(negative, 0, mant.0);
        }
        //if mant >> Self::MB != BUintD8::ZERO {
        mant.0 ^= BUintD8::ONE << Self::MB;
        //}
        Self::from_raw_parts(negative, exp as UnsignedFloatExponent, mant.0)
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
