use crate::ExpType;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

macro_rules! ops {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        impl<const N: usize> Neg for $BInt<N> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self::neg(self)
            }
        }

        impl<const N: usize> Neg for &$BInt<N> {
            type Output = $BInt<N>;

            #[inline]
            fn neg(self) -> $BInt<N> {
                $BInt::neg(*self)
            }
        }

        impl<const N: usize> BitAnd for $BInt<N> {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self::bitand(self, rhs)
            }
        }

        impl<const N: usize> BitOr for $BInt<N> {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self::bitor(self, rhs)
            }
        }

        impl<const N: usize> BitXor for $BInt<N> {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self::bitxor(self, rhs)
            }
        }

        impl<const N: usize> Div for $BInt<N> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self {
                Self::div(self, rhs)
            }
        }

        impl<const N: usize> Not for $BInt<N> {
            type Output = Self;

            fn not(self) -> Self {
                Self::not(self)
            }
        }

        impl<const N: usize> Rem for $BInt<N> {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self {
                Self::rem(self, rhs)
            }
        }

        crate::int::ops::impls!($BInt, $BUint, $BInt);
    };
}

#[cfg(test)]
crate::test::all_digit_tests! {
    use crate::test::{debug_skip, test_bignum, types::itest};
    use core::ops::Neg;

    test_bignum! {
        function: <itest>::neg(a: itest),
        skip: debug_skip!(a == itest::MIN)
    }
}

crate::macro_impl!(ops);
