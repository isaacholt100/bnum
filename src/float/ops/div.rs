use super::Float;
use crate::Exponent;
use crate::Uint;
use crate::float::FloatExponent;
use crate::float::UnsignedFloatExponent;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(crate) fn div_internal(self, rhs: Self, negative: bool) -> Self {
        let (a, b) = (self, rhs);
        let (_, e1, s1) = a.into_biased_parts();
        let (_, e2, s2) = b.into_biased_parts();

        let b1 = s1.bit_width();
        let b2 = s2.bit_width();

        let mut e =
            (e1 as FloatExponent) - (e2 as FloatExponent) + Self::EXP_BIAS + (b1 as FloatExponent)
                - (b2 as FloatExponent)
                - 1;

        let mut extra_shift = 0;
        if !e.is_positive() {
            extra_shift = (1 - e) as UnsignedFloatExponent;
            e = 1;
        }

        let total_shift = (MB as FloatExponent + 1 + b2 as FloatExponent - b1 as FloatExponent)
            - (extra_shift as FloatExponent);

        let widened_s1 = s1.to_digits::<u64>().grow::<1, W>(); // double the number of bits in s1, extending with zeros
        let widened_s2 = s2.to_digits::<u64>().grow::<1, W>(); // double the number of bits in s2, extending with zeros

        let large = if !total_shift.is_negative() {
            unsafe { widened_s1.unchecked_shl(total_shift as Exponent) }
        } else {
            unsafe { widened_s1.unchecked_shr((-total_shift) as Exponent, false) }
        };
        let (q, r) = large.div_rem_unchecked(widened_s2);
        let mut division = Uint::<W>::from_digits(q.remove_tail()); // remove_tail means we resize to the desired width

        let rem = if division.bit_width() != Self::MB + 2 {
            let rem = Uint::<W>::from_digits(r.remove_tail()); // remove_tail means we resize to the desired width
            rem
        } else {
            e += 1;
            let (q, r) = unsafe { large.unchecked_shr(1, false).div_rem_unchecked(widened_s2) };
            division = Uint::<W>::from_digits(q.remove_tail()); // remove_tail means we resize to the desired width
            let rem = Uint::<W>::from_digits(r.remove_tail()); // remove_tail means we resize to the desired width
            rem
        };
        if rem * crate::n!(0b10) > s2 {
            division += crate::n!(0b1);
        } else if rem * crate::n!(0b10) == s2 {
            if division.is_odd() {
                division += crate::n!(0b1);
            }
        }
        if division.bit_width() == Self::MB + 2 {
            e += 1;
            division >>= 1 as Exponent;
        }

        if e > Self::MAX_EXP + Self::EXP_BIAS - 1 {
            return Self::INFINITY;
        }

        if e == 1 && division.bit_width() < Self::MB + 1 {
            return Self::from_raw_parts(negative, 0, division);
        }

        division.set_bit(Self::MB, false);
        
        Self::from_raw_parts(negative, e as UnsignedFloatExponent, division)
    }

    #[inline]
    pub(super) fn div(self, rhs: Self) -> Self {
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
