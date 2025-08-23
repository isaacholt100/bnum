use super::Int;
use crate::Uint;

use crate::ExpType;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl<const N: usize> Neg for Int<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const N: usize> Neg for &Int<N> {
    type Output = Int<N>;

    #[inline]
    fn neg(self) -> Int<N> {
        Int::neg(*self)
    }
}

impl<const N: usize> BitAnd for Int<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::bitand(self, rhs)
    }
}

impl<const N: usize> BitOr for Int<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::bitor(self, rhs)
    }
}

impl<const N: usize> BitXor for Int<N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::bitxor(self, rhs)
    }
}

impl<const N: usize> Div for Int<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

impl<const N: usize> Not for Int<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const N: usize> Rem for Int<N> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

crate::ints::ops::impls!(Int);

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::debug_skip;
    use core::ops::Neg;
    use crate::test::test_bignum;

    test_bignum! {
        function: <itest>::neg(a: itest),
        skip: debug_skip!(a == itest::MIN)
    }
}
