use super::Uint;
use crate::Digit;
use crate::ExpType;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

// impl<const N: usize> Add<Digit> for Uint<N> {
//     type Output = Self;

//     #[inline]
//     fn add(self, rhs: Digit) -> Self {
//         let mut out = self;
//         let result = digit::carrying_add(out.digits[0], rhs, false);
//         out.digits[0] = result.0;
//         let mut carry = result.1;
//         let mut i = 1;
//         while i < N && carry {
//             let result = out.digits[i].overflowing_add(1);
//             out.digits[i] = result.0;
//             carry = result.1;
//             i += 1;
//         }
//         out
//     }
// }

impl<const N: usize> BitAnd for Uint<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::bitand(self, rhs)
    }
}

impl<const N: usize> BitOr for Uint<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::bitor(self, rhs)
    }
}

impl<const N: usize> BitXor for Uint<N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::bitxor(self, rhs)
    }
}

impl<const N: usize> Div for Uint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

impl<const N: usize> Div<u64> for Uint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u64) -> Self {
        self.div_rem_u64(rhs).0
    }
}

impl<const N: usize> Not for Uint<N> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const N: usize> Rem for Uint<N> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

impl<const N: usize> Rem<u64> for Uint<N> {
    type Output = u64;

    #[inline]
    fn rem(self, rhs: u64) -> u64 {
        self.div_rem_u64(rhs).1
    }
}

#[cfg(feature = "signed")]
use crate::Int;

crate::ints::ops::impls!(Uint);

#[cfg(test)]
crate::test::test_all_widths! {
    crate::ints::ops::tests!(utest);
}

#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;
    use core::ops::{BitAnd, BitOr, BitXor, Not};

    test_bignum! {
        function: <utest as BitAnd>::bitand(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest as BitOr>::bitor(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest as BitXor>::bitxor(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest as Not>::not(a: utest)
    }
}
