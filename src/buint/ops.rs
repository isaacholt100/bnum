use crate::digit;
use crate::ExpType;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

macro_rules! ops {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        impl<const N: usize> Add<$Digit> for $BUint<N> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: $Digit) -> Self {
                let mut out = self;
                let result = digit::$Digit::carrying_add(out.digits[0], rhs, false);
                out.digits[0] = result.0;
                let mut carry = result.1;
                let mut i = 1;
                while i < N && carry {
                    let result = out.digits[i].overflowing_add(1);
                    out.digits[i] = result.0;
                    carry = result.1;
                    i += 1;
                }
                out
            }
        }

        impl<const N: usize> BitAnd for $BUint<N> {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self::bitand(self, rhs)
            }
        }

        impl<const N: usize> BitOr for $BUint<N> {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self::bitor(self, rhs)
            }
        }

        impl<const N: usize> BitXor for $BUint<N> {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self::bitxor(self, rhs)
            }
        }

        impl<const N: usize> Div for $BUint<N> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self {
                Self::div(self, rhs)
            }
        }

        impl<const N: usize> Div<$Digit> for $BUint<N> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: $Digit) -> Self {
                self.div_rem_digit(rhs).0
            }
        }

        impl<const N: usize> Not for $BUint<N> {
            type Output = Self;

            #[inline]
            fn not(self) -> Self {
                Self::not(self)
            }
        }

        impl<const N: usize> Rem for $BUint<N> {
            type Output = Self;

            #[inline]
            fn rem(self, rhs: Self) -> Self {
                Self::rem(self, rhs)
            }
        }

        impl<const N: usize> Rem<$Digit> for $BUint<N> {
            type Output = $Digit;

            #[inline]
            fn rem(self, rhs: $Digit) -> $Digit {
                self.div_rem_digit(rhs).1
            }
        }

        crate::int::ops::impls!($BUint, $BUint, $BInt);

        #[cfg(all(test, test_int_bits = "64"))]
        paste::paste! {
            mod [<$Digit _add_digit_test>] {
                use super::*;
                use crate::test::{test_bignum, types::utest};
                use crate::test::types::big_types::$Digit::*;

                quickcheck::quickcheck! {
                    fn add_digit(a: utest, b: $Digit) -> quickcheck::TestResult {
                        use crate::cast::As;

                        let c: utest = b.as_();
                        match a.checked_add(c) {
                            None => quickcheck::TestResult::discard(),
                            Some(_d) => {
                                let e: UTEST = b.as_();
                                let f: UTEST = a.as_();
                                quickcheck::TestResult::from_bool(f + e == f + b)
                            }
                        }
                    }                    
                }
            }
        }
    };
}

#[cfg(test)]
crate::test::all_digit_tests! {
    use crate::test::{test_bignum, types::utest};

    crate::int::ops::tests!(utest);
}

crate::macro_impl!(ops);
