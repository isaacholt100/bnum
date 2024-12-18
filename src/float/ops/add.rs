use core::num::FpCategory;
use crate::{float::UnsignedFloatExponent, BUintD8, ExpType};
use super::Float;

// TODO: quickcheck tests are very occasionally failing, need to fix this function
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(super) fn add_internal(self, rhs: Self, negative: bool) -> Self {
        let (a, b) = if rhs.abs().gt(&self.abs()) {
            // If b has a larger exponent than a, swap a and b so that a has the larger exponent
            (rhs, self)
        } else {
            (self, rhs)
        };
        let (_, mut a_exp, mut a_mant) = a.into_biased_parts();
        let (_, b_exp, mut b_mant) = b.into_biased_parts();

        let exp_diff = a_exp - b_exp;

        let sticky_bit = b_mant.trailing_zeros() as UnsignedFloatExponent + 1 < exp_diff;

        // Append extra bits to the mantissas to ensure correct rounding
        a_mant <<= 2 as ExpType;
        b_mant <<= 2 as ExpType;

        // If the shift causes an overflow, the b_mant is too small so is set to 0
        b_mant = if UnsignedFloatExponent::BITS - exp_diff.leading_zeros() <= ExpType::BITS { // number of bits needed to store exp_diff is less than bit width of ExpType, so can cast
            b_mant.checked_shr(exp_diff as ExpType).unwrap_or(BUintD8::ZERO)
        } else {
            BUintD8::ZERO
        };

        if sticky_bit {
            b_mant |= BUintD8::ONE; // round up
        }

        let mut mant = a_mant + b_mant;

        let overflow = !(mant >> (MB + 3)).is_zero();
        if !overflow {
            if mant & BUintD8::from_digit(0b11) == BUintD8::from_digit(0b11)
                || mant & BUintD8::from_digit(0b110) == BUintD8::from_digit(0b110)
            {
                mant += BUintD8::FOUR;
                if !(mant >> (MB + 3)).is_zero() {
                    mant >>= 1 as ExpType;
                    a_exp += 1;
                }
            }
        } else {
            match (mant & BUintD8::from_digit(0b111)).digits()[0] {
                0b111 | 0b110 | 0b101 => {
                    mant += BUintD8::EIGHT;
                }
                0b100 => {
                    if mant & BUintD8::from_digit(0b1000) == BUintD8::from_digit(0b1000) {
                        mant += BUintD8::EIGHT; // 0b1000
                    }
                }
                _ => {}
            }

            mant >>= 1 as ExpType;
            a_exp += 1;
        }
        if a_exp > Self::MAX_UNBIASED_EXP {
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        mant >>= 2 as ExpType;

        if (mant >> Self::MB).is_zero() {
            a_exp = 0;
        } else {
            mant ^= BUintD8::ONE << Self::MB;
        }
        let a = Self::from_raw_parts(negative, a_exp, mant);
        a
    }

    #[inline]
    pub(crate) fn add(self, rhs: Self) -> Self {
        let self_negative = self.is_sign_negative();
        let rhs_negative = rhs.is_sign_negative();
        
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) => self,
            (_, FpCategory::Nan) => rhs,
            (FpCategory::Infinite, FpCategory::Infinite) => {
                if self_negative ^ rhs_negative {
                    Self::NAN
                } else {
                    self
                }
            }
            (FpCategory::Infinite, _) => self,
            (_, FpCategory::Infinite) => rhs,
            (FpCategory::Zero, FpCategory::Zero) => {
                if self_negative && rhs_negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (_, _) => {
                if self_negative ^ rhs_negative {
                    self.sub_internal(rhs, self_negative)
                } else {
                    let r = self.add_internal(rhs, self_negative);
                    r
                }
            }
        }
    }
}