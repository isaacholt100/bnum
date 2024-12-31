use core::num::FpCategory;
use crate::{float::UnsignedFloatExponent, BUintD8, ExpType};
use super::Float;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(super) fn add_internal(self, rhs: Self, negative: bool) -> Self {
        let (a, b) = if rhs.abs().total_cmp(&self.abs()) == core::cmp::Ordering::Greater {
            // If b has is larger than a, swap a and b so that a has the larger exponent
            (rhs, self)
        } else {
            (self, rhs)
        };
        let (_, a_exp, a_mant) = a.into_normalised_signed_parts();
        let (_, b_exp, b_mant) = b.into_normalised_signed_parts();
        let exp_diff = (a_exp - b_exp) as UnsignedFloatExponent; // guaranteed to be non-negative since a >= b
        
        // If the shift causes an overflow, the b_mant is too small so is set to 0
        let (exp, mant) = if UnsignedFloatExponent::BITS - exp_diff.leading_zeros() <= ExpType::BITS { // number of bits needed to store exp_diff is less than bit width of ExpType, so can cast
            let exp_diff = exp_diff as ExpType;
            match b_mant.checked_shr(exp_diff) { // shift b_mant so it is aligned (in terms of exponents) with a_mant, so we can add them
                Some(shifted) => {
                    if exp_diff == 0 {
                        let mut mant = a_mant + shifted; // result must have overflowed, since both shifted and a_mant have bit at index Self::MB set to 1
                        let round_up = mant.digits[0] & 0b11 == 0b11; // round by ties-to-even
                        mant = mant >> 1;
                        if round_up {
                            mant += BUintD8::ONE; // note this cannot overflow now, since if there was round up, then the last bit of the sum is one, meaning that a_mant and shifted can't both be their maximum value (which would be required for overflow here)
                        }
                        (a_exp + 1, mant)
                    } else {
                        let mut mant = a_mant + shifted;
                        let discarded_bits = b_mant & (BUintD8::MAX >> (BUintD8::<W>::BITS - exp_diff));
                        if mant.bit(Self::MB + 1) { // overflow occurred
                            let mut shifted_mant: BUintD8<W> = mant >> 1;
                            let gte_half = mant.is_odd(); // if discarded bits are at least a half
                            let round_up = gte_half && !(discarded_bits.is_zero() && shifted_mant.is_even()); // round by ties-to-even
                            if round_up {
                                shifted_mant += BUintD8::ONE;
                            }
                            (a_exp + 1, shifted_mant)
                        } else { // no overflow yet, but still need to check for overflow when performing ties-to-even rounding
                            let round_up = discarded_bits.bit(exp_diff - 1) && !(discarded_bits.is_power_of_two() && mant.is_even()); // round according to ties-to-even. exp_diff - 1 will be non-negative, since if exp_diff = 0, then we would have had the overflow condition earlier
                            if round_up {
                                mant += BUintD8::ONE;
                            }
                            if mant.bit(Self::MB + 1) { // overflow occurred
                                debug_assert!(mant.is_even()); // since overflow occurred and we added one, the result must be even
                                (a_exp + 1, mant >> 1) // don't need to worry about checking for round up here, as mantissa is even, so when right shifted by 1, the discarded bits will be less than half (i.e. no round up)
                            } else {
                                (a_exp, mant)
                            }
                        }
                    }
                },
                None => (a_exp, a_mant), // no round up since the type holding b_mant has bit-width <= exp_diff, and b_mant has bit-width strictly smaller than the type's bit-width (since we enforce the float exponent to be at least one bit wide)
            }
        } else {
            // we can't even cast exp_diff to ExpType, so it would be right-shifted to 0, and no round up since b_mant can't ever be the full width of ExpType::MAX (since we enforce all bnum types to have bit width <= ExpType::MAX, and we enforce that the float exponent takes at least one bit)
            (a_exp, a_mant)
        };
        if exp >= Self::MAX_EXP {
            debug_assert!(exp.eq(&Self::MAX_EXP)); // shouldn't be possible for exponent to have exceeded maximum exponent
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }
        Self::from_normalised_signed_parts(negative, exp, mant)
    }

    #[inline]
    pub(super) fn add_internal_old(self, rhs: Self, negative: bool) -> Self {
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
            if mant.digits[0] & 0b11 == 0b11
                || mant.digits[0] & 0b110 == 0b110
            {
                mant += BUintD8::FOUR; // += 0b100
                if !(mant >> (MB + 3)).is_zero() {
                    mant >>= 1 as ExpType;
                    a_exp += 1;
                }
            }
        } else {
            match mant.digits[0] & 0b111 {
                0b111 | 0b110 | 0b101 => {
                    mant += BUintD8::EIGHT; // 0b1000
                }
                0b100 => {
                    if mant.digits[0] & 0b1000 == 0b1000 {
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
                if self_negative != rhs_negative {
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
                if self_negative != rhs_negative {
                    self.sub_internal(rhs, self_negative)
                } else {
                    let r = self.add_internal(rhs, self_negative);
                    r
                }
            }
        }
    }
}