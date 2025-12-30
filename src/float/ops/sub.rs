use super::Float;
use crate::Exponent;
use crate::Uint;
use crate::cast::{As, CastFrom};
use crate::float::FloatExponent;
use crate::float::UnsignedFloatExponent;
use core::num::FpCategory;

// TODO: this very occasionally fails quickcheck tests, need to fix
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(crate) fn sub_internal_new(self, rhs: Self, negative: bool) -> Self {
        use core::cmp::Ordering;

        let (a, b, negative) = match rhs.abs().total_cmp(&self.abs()) {
            // If b has is larger than a, swap a and b so that a has the larger exponent
            Ordering::Greater => (rhs, self, !negative),
            Ordering::Less => (self, rhs, negative),
            Ordering::Equal => return Self::ZERO,
        };
        let (_, a_exp, a_mant) = a.into_normalised_signed_parts();
        let (_, b_exp, b_mant) = b.into_normalised_signed_parts();
        let exp_diff = (a_exp - b_exp) as UnsignedFloatExponent; // guaranteed to be non-negative since a >= b
        let (exp, mant) = if UnsignedFloatExponent::BITS - exp_diff.leading_zeros() <= Exponent::BITS
        {
            // number of bits needed to store exp_diff is less than bit width of Exponent, so can cast
            let exp_diff = exp_diff as Exponent;
            match b_mant.checked_shr(exp_diff) {
                // shift b_mant so it is aligned (in terms of exponents) with a_mant, so we can add them
                Some(mut shifted) => {
                    let half_way = b_mant.bit(exp_diff - 1) && b_mant.trailing_zeros() == exp_diff - 1; // when we obtained shifted from b_mant, we cut off exactly a power of two
                    let shifted_round_up = b_mant.bit(exp_diff - 1) && b_mant.trailing_zeros() < exp_diff - 1; // if we cut off more than half
                    if shifted_round_up {
                        // this can only happen if exp_diff >= 2
                        shifted += Uint::ONE; // round up since we cut off more than half
                    }
                    let mut mant = a_mant - shifted;
                    debug_assert!(!mant.is_zero()); // since a >= b and a != b, mant must be non-zero
                    // need to renormalise mant to pass into Self::from_normalised_parts
                    // shift back needed to renormalise is:
                    // - arbitrary, if exp_diff is 0. in this case, no shift of b_mant occurred, so we didn't lose any bits, so no need to check if we need to round
                    // - arbitrary, if exp_diff is 1. in this case, the lowest bit of b_mant was lost. 
                    // - 0 or 1, if exp_diff > 1.

                    // so in the exp_diff = 0 or 1 cases, just shift back by the required amount
                    let mut shift_back = Self::MANTISSA_DIGITS - mant.bits();
                    // dbg!(shift_back);
                    // dbg!(exp_diff);
                    if exp_diff == 1 {
                        if half_way && mant.is_odd() && shift_back == 0 {
                            mant.set_bit(0, false); // subtract one
                        }
                    }
                    let is_odd = mant.is_odd();
                    mant <<= shift_back as Exponent;
                    if exp_diff == 1 && half_way {
                        mant -= Uint::power_of_two(shift_back - 1);
                    }
                    // NOTE: code is correct for if exp_diff = 0, I think also for exp_diff = 1
                    // TODO: might need to use sticky bits
                    if exp_diff >= 2 && shift_back == 1 {
                        if !b_mant.bit(exp_diff - 2) {
                            
                        }
                        // dbg!(b_mant.bit(exp_diff - 1));
                        // dbg!(b_mant.trailing_zeros());
                        // let round_down = b_mant.bit(exp_diff - 1) && (b_mant.trailing_zeros() != exp_diff - 1 || is_odd);
                        // dbg!(round_down);
                        // println!("{:032b}", a_mant);
                        // println!("{:032b}", b_mant);
                        // println!("{:032b}", shifted);
                        // dbg!(is_odd);
                        // println!("{:032b}", mant);

                        // if round_down {
                        //     mant -= Uint::ONE;
                        //     dbg!(mant.is_even());
                        //     dbg!(mant.bit(Self::MANTISSA_DIGITS - 1));
                        //     if !mant.bit(Self::MANTISSA_DIGITS - 1) { // mantissa might have decreased in bit width by 1
                        //         shift_back += 1;
                        //         mant <<= 1 as Exponent;
                        //         let round_up = !b_mant.bit(exp_diff - 2); // then most significant lost bit of the true mantissa must be 0, or is 1 then all lower order bits are 0, so don't round
                        //         // if this bit is not set, then we know one of the lower order bits must be set as round_down is true
                        //         if round_up {
                        //             mant.set_bit(0, true); // add one
                        //         }
                        //     }
                        // }
                    }
                    (a_exp - shift_back as FloatExponent, mant)
                },
                None => (a_exp, a_mant), // no round down since the type holding b_mant has bit-width <= exp_diff, and b_mant has bit-width strictly smaller than the type's bit-width (since we enforce the float exponent to be at least one bit wide)
            }
        } else {
            // we can't even cast exp_diff to Exponent, so it would be right-shifted to 0, and no round down since b_mant can't ever be the full width of Exponent::MAX (since we enforce all bnum types to have bit width <= Exponent::MAX, and we enforce that the float exponent takes at least one bit)
            (a_exp, a_mant)
        };
        Self::from_normalised_signed_parts(negative, exp, mant)
    }

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
            && exp_diff < Uint::<W>::BITS.into()
            && b_mant.bit(exp_diff.as_::<Exponent>() - 1);
        let all_zeros = exp_diff != 0 && b_mant.trailing_zeros() + 1 == exp_diff.as_::<Exponent>();

        // Append extra bits to the mantissas to ensure correct rounding
        a_mant = a_mant << 1 as Exponent;
        b_mant = b_mant << 1 as Exponent;

        let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();

        // If the shift causes an overflow, the b_mant is too small so is set to 0
        let shifted_b_mant = match exp_diff.try_into().ok() {
            Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(Uint::ZERO),
            None => Uint::ZERO,
        };

        // If the shift causes an overflow, the b_mant is too small so is set to 0

        if sticky_bit {
            //b_mant |= 1;
        }

        let mut mant = a_mant - shifted_b_mant;

        if mant.bits() == Self::MB + 2 {
            if mant & Uint::cast_from(0b10u8) == Uint::cast_from(0b10u8) && !sticky_bit {
                mant += Uint::ONE;
            }

            mant >>= 1 as Exponent;
        } else {
            a_exp -= 1;
            a_mant <<= 1 as Exponent;
            b_mant <<= 1 as Exponent;

            let sticky_bit = b_mant.trailing_zeros() < exp_diff.as_();

            // If the shift causes an overflow, the b_mant is too small so is set to 0
            let shifted_b_mant = match exp_diff.try_into().ok() {
                Some(exp_diff) => b_mant.checked_shr(exp_diff).unwrap_or(Uint::ZERO),
                None => Uint::ZERO,
            };

            // If the shift causes an overflow, the b_mant is too small so is set to 0

            if sticky_bit {
                //b_mant |= 1;
            }

            mant = a_mant - shifted_b_mant;

            if mant.bits() == Self::MB + 2 {
                if mant & Uint::cast_from(0b10u8) == Uint::cast_from(0b10u8) && !sticky_bit {
                    mant += Uint::ONE;
                }

                mant >>= 1 as Exponent;
            } else {
                let _half_way = (); // TODO
                if sticky_bit2 && !all_zeros
                    || (sticky_bit2
                        && all_zeros
                        && b_mant & Uint::cast_from(0b1u8) == Uint::cast_from(0b1u8))
                {
                    mant -= Uint::ONE;
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
            mant ^= Uint::ONE << Self::MB;
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
            (FpCategory::Zero, FpCategory::Zero) => {
                if self.is_sign_negative() && rhs.is_sign_positive() {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (FpCategory::Zero, _) => rhs.neg(),
            (_, FpCategory::Zero) => self,
            (_, _) => {
                let self_negative = self.is_sign_negative();
                let rhs_negative = rhs.is_sign_negative();
                if self_negative != rhs_negative {
                    self.add_internal(rhs, self_negative)
                } else {
                    self.sub_internal_new(rhs, self_negative)
                }
            }
        }
    }
}
