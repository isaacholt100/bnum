use super::Bint;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, DivAssign, Mul, MulAssign, Neg, Not, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::{expect, op_ref_impl, assign_ref_impl, all_shift_impls};
use crate::ExpType;

impl<const N: usize> const Add<Self> for Bint<N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_add(rhs), "attempt to add with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_add(rhs)
    }
}

op_ref_impl!(Add<Bint<N>> for Bint<N>, add);

impl<const N: usize> const AddAssign for Bint<N> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

assign_ref_impl!(AddAssign<Bint<N>> for Bint, add_assign);

impl<const N: usize> const BitAnd for Bint<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::from_bits(self.bits & rhs.bits)
    }
}

op_ref_impl!(BitAnd<Bint<N>> for Bint<N>, bitand);

impl<const N: usize> const BitAndAssign for Bint<N> {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

assign_ref_impl!(BitAndAssign<Bint<N>> for Bint, bitand_assign);

impl<const N: usize> const BitOr for Bint<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::from_bits(self.bits | rhs.bits)
    }
}

op_ref_impl!(BitOr<Bint<N>> for Bint<N>, bitor);

impl<const N: usize> const BitOrAssign for Bint<N> {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

assign_ref_impl!(BitOrAssign<Bint<N>> for Bint, bitor_assign);

impl<const N: usize> const BitXor for Bint<N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::from_bits(self.bits ^ rhs.bits)
    }
}

op_ref_impl!(BitXor<Bint<N>> for Bint<N>, bitxor);

impl<const N: usize> const BitXorAssign for Bint<N> {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

assign_ref_impl!(BitXorAssign<Bint<N>> for Bint, bitxor_assign);

impl<const N: usize> const DivAssign for Bint<N> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

assign_ref_impl!(DivAssign<Bint<N>> for Bint, div_assign);

impl<const N: usize> const Mul for Bint<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_mul(rhs), "attempt to multiply with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_mul(rhs)
    }
}

op_ref_impl!(Mul<Bint<N>> for Bint<N>, mul);

impl<const N: usize> const MulAssign for Bint<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

assign_ref_impl!(MulAssign<Bint<N>> for Bint, mul_assign);

impl<const N: usize> const Not for Bint<N> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::from_bits(!self.bits)
    }
}

impl<const N: usize> const Not for &Bint<N> {
    type Output = Bint<N>;

    #[inline]
    fn not(self) -> Bint<N> {
        !(*self)
    }
}

impl<const N: usize> const Neg for Bint<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_neg(), "attempt to negate with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_neg()
    }
}

impl<const N: usize> const Neg for &Bint<N> {
    type Output = Bint<N>;

    #[inline]
    fn neg(self) -> Bint<N> {
        (*self).neg()
    }
}

impl<const N: usize> const RemAssign for Bint<N> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

assign_ref_impl!(RemAssign<Bint<N>> for Bint, rem_assign);

impl<const N: usize> const Shl<ExpType> for Bint<N> {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: ExpType) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_shl(rhs), "attempt to shift left with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_shl(rhs)
    }
}

op_ref_impl!(Shl<ExpType> for Bint<N>, shl);

impl<const N: usize> const ShlAssign<ExpType> for Bint<N> {
    #[inline]
    fn shl_assign(&mut self, rhs: ExpType) {
        *self = self.shl(rhs);
    }
}

assign_ref_impl!(ShlAssign<ExpType> for Bint, shl_assign);

impl<const N: usize> const Shr<ExpType> for Bint<N> {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: ExpType) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_shr(rhs), "attempt to shift left with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_shr(rhs)
    }
}

op_ref_impl!(Shr<ExpType> for Bint<N>, shr);

impl<const N: usize> const ShrAssign<ExpType> for Bint<N> {
    #[inline]
    fn shr_assign(&mut self, rhs: ExpType) {
        *self = self.shr(rhs);
    }
}

assign_ref_impl!(ShrAssign<ExpType> for Bint, shr_assign);

use crate::uint::BUint;

all_shift_impls!(Bint);

impl<const N: usize> const Sub for Bint<N> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_sub(rhs), "attempt to subtract with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_sub(rhs)
    }
}

op_ref_impl!(Sub<Bint<N>> for Bint<N>, sub);

impl<const N: usize> const SubAssign for Bint<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

assign_ref_impl!(SubAssign<Bint<N>> for Bint, sub_assign);