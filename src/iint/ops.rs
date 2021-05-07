use super::BIint;
use crate::tryops::TryOps;
use crate::sign::Sign;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use num_traits::One;

impl<const N: usize> Add for BIint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.try_add(rhs).unwrap()
    }
}

impl<const N: usize> AddAssign for BIint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl<const N: usize> BitAnd for BIint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.bit_op(rhs, |a, b| {
            a & b
        }, |a, b| {
            match (a, b) {
                (Sign::Minus, Sign::Minus) => Sign::Minus,
                _ => Sign::Plus,
            }
        })
    }
}

impl<const N: usize> BitAndAssign for BIint<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<const N: usize> BitOr for BIint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.bit_op(rhs, |a, b| {
            a | b
        }, |a, b| {
            match (a, b) {
                (Sign::Plus, Sign::Plus) => Sign::Plus,
                _ => Sign::Minus,
            }
        })
    }
}

impl<const N: usize> BitOrAssign for BIint<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<const N: usize> BitXor for BIint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        self.bit_op(rhs, |a, b| {
            a ^ b
        }, |a, b| {
            if a == b {
                Sign::Plus
            } else {
                Sign::Minus
            }
        })
    }
}

impl<const N: usize> BitXorAssign for BIint<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<const N: usize> Div for BIint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.try_add(rhs).unwrap()
    }
}

impl<const N: usize> DivAssign for BIint<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl<const N: usize> Mul for BIint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.try_mul(rhs).unwrap()
    }
}

impl<const N: usize> MulAssign for BIint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl<const N: usize> Neg for BIint<N> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            uint: self.uint,
            sign: self.sign.negate(),
        }
    }
}

impl<const N: usize> Not for BIint<N> {
    type Output = Self;

    fn not(self) -> Self {
        self.neg() - BIint::<N>::one()
    }
}

impl<const N: usize> Rem for BIint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.try_rem(rhs).unwrap()
    }
}

impl<const N: usize> RemAssign for BIint<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl<const N: usize> Shl<u32> for BIint<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        self.try_shl(rhs).unwrap()
    }
}

impl<const N: usize> ShlAssign<u32> for BIint<N> {
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shl(rhs);
    }
}

impl<const N: usize> Shr<u32> for BIint<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        self.try_shr(rhs).unwrap()
    }
}

impl<const N: usize> ShrAssign<u32> for BIint<N> {
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shr(rhs);
    }
}

impl<const N: usize> Sub for BIint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.try_sub(rhs).unwrap()
    }
}

impl<const N: usize> SubAssign for BIint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calcs_bit_and() {
        let u1 = 882394723893242048395i128;
        let u2 = -905486943583959088340i128;
        let i1 = BIint::<10>::from(u1);
        let i2 = BIint::<10>::from(u2);
        assert_eq!(i1 & i2, BIint::<10>::from(u1 & u2));

        let u1 = -958495070839573345347i128;
        let u2 = -23497893475897234345i128;
        let i1 = BIint::<10>::from(u1);
        let i2 = BIint::<10>::from(u2);
        assert_eq!(i1 & i2, BIint::<10>::from(u1 & u2));
    }

    #[test]
    fn it_calcs_bit_or() {
        let u1 = 882394723893242048395i128;
        let u2 = -905486943583959088340i128;
        let i1 = BIint::<10>::from(u1);
        let i2 = BIint::<10>::from(u2);
        assert_eq!(i1 | i2, BIint::<10>::from(u1 | u2));

        let u1 = -958495070839573345347i128;
        let u2 = -23497893475897234345i128;
        let i1 = BIint::<10>::from(u1);
        let i2 = BIint::<10>::from(u2);
        assert_eq!(i1 | i2, BIint::<10>::from(u1 | u2));
    }

    #[test]
    fn it_calcs_bit_xor() {
        let u1 = 882394723893242048395i128;
        let u2 = -905486943583959088340i128;
        let i1 = BIint::<10>::from(u1);
        let i2 = BIint::<10>::from(u2);
        assert_eq!(i1 ^ i2, BIint::<10>::from(u1 ^ u2));

        let u1 = -958495070839573345347i128;
        let u2 = -23497893475897234345i128;
        let i1 = BIint::<10>::from(u1);
        let i2 = BIint::<10>::from(u2);
        assert_eq!(i1 ^ i2, BIint::<10>::from(u1 ^ u2));
    }
}