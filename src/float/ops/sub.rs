use core::num::FpCategory;
use crate::cast::As;
use crate::float::FloatExponent;
use crate::float::UnsignedFloatExponent;
use crate::BUintD8;
use crate::ExpType;
use super::Float;

// TODO: this very occasionally fails quickcheck tests, need to fix
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(crate) fn sub_internal(mut self, mut rhs: Self, mut negative: bool) -> Self {
        if rhs.abs() > self.abs() {
            // If b has a larger exponent than a, swap a and b so that a has the larger exponent
            negative = !negative;
            core::mem::swap(&mut self, &mut rhs);
        }
        if self.abs() == rhs.abs() {
            return Self::ZERO;
        }

        let (_, a_exp, mut a_mant) = self.into_biased_parts();
        let (_, b_exp, mut b_mant) = rhs.into_biased_parts();
        let exp_diff = a_exp - b_exp;

        let mut a_exp = a_exp as FloatExponent;

        let sticky_bit2 = exp_diff != 0
            && exp_diff < BUintD8::<W>::BITS.into()
            && b_mant.bit(exp_diff.as_::<ExpType>() - 1);
        let all_zeros =
            exp_diff != 0 && b_mant.trailing_zeros() + 1 == exp_diff.as_::<ExpType>();

        // Append extra bits to the mantissas to ensure correct rounding
        a_mant = a_mant << 1 as ExpType;
        b_mant = b_mant << 1 as ExpType;

        let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();

        // If the shift causes an overflow, the b_mant is too small so is set to 0
        let shifted_b_mant = match exp_diff.try_into().ok() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUintD8::ZERO),
            None => BUintD8::ZERO,
        };

        // If the shift causes an overflow, the b_mant is too small so is set to 0

        if sticky_bit {
            //b_mant |= 1;
        }

        let mut mant = a_mant - shifted_b_mant;

        if mant.bits() == Self::MB + 2 {
            if mant & BUintD8::from(0b10u8) == BUintD8::from(0b10u8) && !sticky_bit {
                mant += BUintD8::ONE;
            }

            mant >>= 1 as ExpType;
        } else {
            a_exp -= 1;
            a_mant <<= 1 as ExpType;
            b_mant <<= 1 as ExpType;

            let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();

            // If the shift causes an overflow, the b_mant is too small so is set to 0
            let shifted_b_mant = match exp_diff.try_into().ok() {
                Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(BUintD8::ZERO),
                None => BUintD8::ZERO,
            };

            // If the shift causes an overflow, the b_mant is too small so is set to 0

            if sticky_bit {
                //b_mant |= 1;
            }

            mant = a_mant - shifted_b_mant;

            if mant.bits() == Self::MB + 2 {
                if mant & BUintD8::from(0b10u8) == BUintD8::from(0b10u8) && !sticky_bit {
                    mant += BUintD8::ONE;
                }

                mant >>= 1 as ExpType;
            } else {
                let _half_way = (); // TODO
                if sticky_bit2 && !all_zeros
                    || (sticky_bit2
                        && all_zeros
                        && b_mant & BUintD8::from(0b1u8) == BUintD8::from(0b1u8))
                {
                    mant -= BUintD8::ONE;
                }
                let bits = mant.bits();
                mant <<= Self::MB + 1 - bits;
                a_exp -= MB as FloatExponent + 2 - bits as FloatExponent;
                if !a_exp.is_positive() {
                    a_exp = 1;
                    mant >>= 1 - a_exp;
                }
            }
        }

        if (mant >> Self::MB).is_zero() {
            a_exp = 0;
        } else {
            mant ^= BUintD8::ONE << Self::MB;
        }

        Self::from_raw_parts(negative, a_exp as UnsignedFloatExponent, mant)
    }

    #[inline]
    pub(super) fn sub(self, rhs: Self) -> Self {
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => self,
            (_, FpCategory::Nan) => rhs,
            (FpCategory::Infinite, FpCategory::Infinite) => {
                match (self.is_sign_negative(), rhs.is_sign_negative()) {
                    (true, false) => Self::NEG_INFINITY,
                    (false, true) => Self::INFINITY,
                    _ => Self::NAN,
                }
            }
            (FpCategory::Infinite, _) => self,
            (_, FpCategory::Infinite) => rhs.neg(),
            (_, _) => {
                let self_negative = self.is_sign_negative();
                let rhs_negative = rhs.is_sign_negative();
                if self_negative ^ rhs_negative {
                    self.add_internal(rhs, self_negative)
                } else {
                    self.sub_internal(rhs, self_negative)
                }
            }
        }
    }
}
