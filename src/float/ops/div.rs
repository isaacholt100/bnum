use super::Float;
use crate::{Exponent, Int, Uint};
use core::cmp::Ordering;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    /// Divides two finite non-zero floats with IEEE 754 round-to-nearest-even.
    #[inline]
    pub(crate) const fn div_internal(self, rhs: Self, negative: bool) -> Self {
        const {
            assert!(W > 1, "Float requires W > 1");
            assert!(
                MB > 2 && (MB as Exponent) < Self::BITS - 2,
                "Float requires 2 < MB < BITS - 2"
            );
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
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
        }

        let left_shift: Exponent = if e.le(&Int::ZERO) {
            let sub_drop = Int::<W>::ONE.sub(e);
            let mb_plus_1 = Uint::<W>::cast_from_unsigned_float_exponent(Self::MB + 1).cast_signed();
            match sub_drop.cmp(&mb_plus_1) {
                Ordering::Greater => {
                    return if negative {
                        Self::NEG_ZERO
                    } else {
                        Self::ZERO
                    };
                }
                Ordering::Equal => {
                    // Exact value in [0.5, 1.0) ULPs of MIN_POSITIVE_SUBNORMAL.
                    // Ties-to-even rounds exact 0.5 (na == nb) to 0, strictly > 0.5 to 1.
                    return if na.gt(&nb) {
                        Self::from_biased_uint_parts(negative, Uint::ONE, Uint::ONE)
                    } else if negative {
                        Self::NEG_ZERO
                    } else {
                        Self::ZERO
                    };
                }
                Ordering::Less => {
                    let drop = sub_drop.cast_unsigned().cast_to_unsigned_float_exponent() as Exponent;
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
        match twice_r.cmp(&nb) {
            Ordering::Greater => q = q.add(Uint::ONE),
            Ordering::Equal => {
                if q.is_odd() {
                    q = q.add(Uint::ONE);
                }
            }
            Ordering::Less => {}
        }

        if q.bit(Self::MB + 1) {
            q = q.shr(1);
            e = e.add(Int::ONE);
        }

        if e.ge(&exp_max) {
            return if negative {
                Self::NEG_INFINITY
            } else {
                Self::INFINITY
            };
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
