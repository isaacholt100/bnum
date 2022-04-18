use super::Bint;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Rem, DivAssign, Mul, MulAssign, Neg, Not, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::{option_expect, impl_ops};
use crate::ExpType;
        
impl<const N: usize> const Neg for Bint<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_neg(), "attempt to negate with overflow");

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
        
impl<const N: usize> const BitAnd for Bint<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::from_bits(self.bits & rhs.bits)
    }
}

impl<const N: usize> const BitOr for Bint<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::from_bits(self.bits | rhs.bits)
    }
}

impl<const N: usize> const BitXor for Bint<N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::from_bits(self.bits ^ rhs.bits)
    }
}
        
impl<const N: usize> const Not for Bint<N> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::from_bits(!self.bits)
    }
}

impl_ops!(Bint);