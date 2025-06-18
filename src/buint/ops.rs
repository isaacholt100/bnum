use super::Uint;
use crate::Digit;
use crate::ExpType;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

// TODO: mark that this has been removed
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

impl<const N: usize> Div<Digit> for Uint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Digit) -> Self {
        self.div_rem_digit(rhs).0
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

impl<const N: usize> Rem<Digit> for Uint<N> {
    type Output = Digit;

    #[inline]
    fn rem(self, rhs: Digit) -> Digit {
        self.div_rem_digit(rhs).1
    }
}

#[cfg(feature = "signed")]
use crate::Int;

crate::int::ops::impls!(Uint);

#[cfg(test)]
mod tests {
    use crate::test::{test_bignum, types::*};

    crate::int::ops::tests!(utest);
}
