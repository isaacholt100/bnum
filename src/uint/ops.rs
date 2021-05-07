use super::BUint;
use crate::tryops::TryOps;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

impl<const N: usize> Add for BUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs).expect("attempt to add with overflow")
    }
}

impl<const N: usize> AddAssign for BUint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> BitAnd for BUint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        self.op(&rhs, |a, b| {
            a & b
        })
    }
}

impl<const N: usize> BitAndAssign for BUint<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl<const N: usize> BitOr for BUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.op(&rhs, |a, b| {
            a | b
        })
    }
}

impl<const N: usize> BitOrAssign for BUint<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl<const N: usize> BitXor for BUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        self.op(&rhs, |a, b| {
            a ^ b
        })
    }
}

impl<const N: usize> BitXorAssign for BUint<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl<const N: usize> Div for BUint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.try_div(rhs).unwrap()
    }
}

impl<const N: usize> DivAssign for BUint<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl<const N: usize> Mul for BUint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.try_mul(rhs).unwrap()
    }
}

impl<const N: usize> MulAssign for BUint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl<const N: usize> Not for BUint<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self::from_uninit(|i| {
            !self.digits[i]
        })
    }
}

impl<const N: usize> Rem for BUint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.try_rem(rhs).unwrap()
    }
}

impl<const N: usize> RemAssign for BUint<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl<const N: usize> Shl<u32> for BUint<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        self.try_shl(rhs).unwrap()
    }
}

impl<const N: usize> ShlAssign<u32> for BUint<N> {
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shl(rhs);
    }
}

impl<const N: usize> Shr<u32> for BUint<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        self.try_shr(rhs).unwrap()
    }
}

/*impl<const N: usize> Shr<u128> for BUint<N> {
    type Output = Self;

    fn shr(self, rhs: u128) -> Self {
        if rhs > (N << 6) as u128 {
            panic!("Underflow");
        }
        let shift_index = rhs >> 6;
        let small_shift = rhs & (u64::MAX as u128);
        
        self.try_shr(rhs).unwrap()
    }
}*/

impl<const N: usize> ShrAssign<u32> for BUint<N> {
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shr(rhs);
    }
}

impl<const N: usize> Sub for BUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs).expect("attempt to subtract with overflow")
    }
}

impl<const N: usize> SubAssign for BUint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}