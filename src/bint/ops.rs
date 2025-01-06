use super::BIntD8;
use crate::{BUintD8, Digit};

use crate::ExpType;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl<const N: usize> Neg for BIntD8<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const N: usize> Neg for &BIntD8<N> {
    type Output = BIntD8<N>;

    #[inline]
    fn neg(self) -> BIntD8<N> {
        BIntD8::neg(*self)
    }
}

impl<const N: usize> BitAnd for BIntD8<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::bitand(self, rhs)
    }
}

impl<const N: usize> BitOr for BIntD8<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::bitor(self, rhs)
    }
}

impl<const N: usize> BitXor for BIntD8<N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::bitxor(self, rhs)
    }
}

impl<const N: usize> Div for BIntD8<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

impl<const N: usize> Not for BIntD8<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const N: usize> Rem for BIntD8<N> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

crate::int::ops::impls!(BIntD8);

#[cfg(test)]
mod tests {
    use crate::test::{debug_skip, test_bignum, types::*};
    use core::ops::Neg;

    test_bignum! {
        function: <itest>::neg(a: itest),
        skip: debug_skip!(a == itest::MIN)
    }
}
