use super::Int;
use crate::Uint;

#[doc = doc::bigint_helpers::impl_desc!()]
impl<const N: usize> Int<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
        let (s1, o1) = self.overflowing_add(rhs);
        if carry {
            let (s2, o2) = s1.overflowing_add(Self::ONE);
            (s2, o1 ^ o2)
        } else {
            (s1, o1)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
        let (s1, o1) = self.overflowing_sub(rhs);
        if borrow {
            let (s2, o2) = s1.overflowing_sub(Self::ONE);
            (s2, o1 ^ o2)
        } else {
            (s1, o1)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn widening_mul(self, rhs: Self) -> (Uint<N>, Self) {
        let (u_lo, u_hi) = self.unsigned_abs().widening_mul(rhs.unsigned_abs());
        if self.is_negative() == rhs.is_negative() {
            (u_lo, Self::from_bits(u_hi))
        } else {
            let (u_lo, overflow) = u_lo.not().overflowing_add(Uint::ONE);
            let hi = if overflow {
                u_hi.cast_signed().wrapping_neg()
            } else {
                u_hi.cast_signed().not()
            };
            (u_lo, hi)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Uint<N>, Self) {
        // we pretend that we have a "super bigint" - a big int with two "digits", where the digits are themselves big ints (Uints). then apply the same logic that an wrapping_add for signed ints is the same as wrapping_add for unsigned ints
        let (lo, hi) = self.widening_mul(rhs);
        let extension = if carry.is_negative() {
            Self::NEG_ONE // all ones
        } else {
            Self::ZERO // all zeros
        };
        let (lo, overflow) = lo.overflowing_add(carry.to_bits());
        let hi = hi.wrapping_add(extension);
        if overflow {
            (lo, hi.wrapping_add(Self::ONE))
        } else {
            (lo, hi)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn carrying_mul_add(self, rhs: Self, carry: Self, add: Self) -> (Uint<N>, Self) {
        // similarly to carrying_mul
        let (lo, hi) = self.carrying_mul(rhs, carry);
        let extension = if add.is_negative() {
            Self::NEG_ONE // all ones
        } else {
            Self::ZERO // all zeros
        };
        let (lo, overflow) = lo.overflowing_add(add.to_bits());
        let hi = hi.wrapping_add(extension);
        if overflow {
            (lo, hi.wrapping_add(Self::ONE))
        } else {
            (lo, hi)
        }
    }
}

#[cfg(all(test, feature = "nightly"))] // since bigint_helper_methods are not stable yet
crate::test::test_all_widths! {
    crate::ints::bigint_helpers::tests!(itest);
}

use crate::doc;
