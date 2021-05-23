use super::BUint;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

impl<const N: usize> BUint<N> {
    pub const fn add(self, rhs: Self) -> Self {
        expect!(self.checked_add(rhs), "attempt to add with overflow")
    }
}

impl<const N: usize> Add<Self> for BUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

op_ref_impl!(Add<BUint<N>> for BUint, add);

impl<const N: usize> AddAssign for BUint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

assign_ref_impl!(AddAssign<BUint<N>> for BUint, add_assign);

impl<const N: usize> BitAnd for BUint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] & rhs.digits[i];
            i += 1;
        }
        out
    }
}

op_ref_impl!(BitAnd<BUint<N>> for BUint, bitand);

impl<const N: usize> BitAndAssign for BUint<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

assign_ref_impl!(BitAndAssign<BUint<N>> for BUint, bitand_assign);

impl<const N: usize> BitOr for BUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self::bitor(self, rhs)
    }
}

impl<const N: usize> BUint<N> {
    pub const fn bitor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] | rhs.digits[i];
            i += 1;
        }
        out
    }
}

op_ref_impl!(BitOr<BUint<N>> for BUint, bitor);

impl<const N: usize> BitOrAssign for BUint<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

assign_ref_impl!(BitOrAssign<BUint<N>> for BUint, bitor_assign);

impl<const N: usize> BitXor for BUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] ^ rhs.digits[i];
            i += 1;
        }
        out
    }
}

op_ref_impl!(BitXor<BUint<N>> for BUint, bitxor);

impl<const N: usize> BitXorAssign for BUint<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

assign_ref_impl!(BitXorAssign<BUint<N>> for BUint, bitxor_assign);

impl<const N: usize> Div for BUint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
}

op_ref_impl!(Div<BUint<N>> for BUint, div);

impl<const N: usize> DivAssign for BUint<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

assign_ref_impl!(DivAssign<BUint<N>> for BUint, div_assign);

impl<const N: usize> Mul for BUint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        expect!(self.checked_mul(rhs), "attempt to multiply with overflow")
    }
}

op_ref_impl!(Mul<BUint<N>> for BUint, mul);

impl<const N: usize> MulAssign for BUint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

assign_ref_impl!(MulAssign<BUint<N>> for BUint, mul_assign);

impl<const N: usize> BUint<N> {
    pub const fn not(self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = !self.digits[i];
            i += 1;
        }
        out
    }
}

impl<const N: usize> Not for BUint<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const N: usize> Not for &BUint<N> {
    type Output = BUint<N>;

    fn not(self) -> BUint<N> {
        (*self).not()
    }
}

impl<const N: usize> Rem for BUint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }
}

op_ref_impl!(Rem<BUint<N>> for BUint, rem);

impl<const N: usize> RemAssign for BUint<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

assign_ref_impl!(RemAssign<BUint<N>> for BUint, rem_assign);

impl<const N: usize> Shl<u32> for BUint<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        expect!(self.checked_shl(rhs), "attempt to shift left with overflow")
    }
}

op_ref_impl!(Shl<u32> for BUint, shl);

impl<const N: usize> ShlAssign<u32> for BUint<N> {
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shl(rhs);
    }
}

assign_ref_impl!(ShlAssign<u32> for BUint, shl_assign);

impl<const N: usize> Shr<u32> for BUint<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        expect!(self.checked_shr(rhs), "attempt to shift right with overflow")
    }
}

op_ref_impl!(Shr<u32> for BUint, shr);

impl<const N: usize> ShrAssign<u32> for BUint<N> {
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shr(rhs);
    }
}

assign_ref_impl!(ShrAssign<u32> for BUint, shr_assign);

use crate::BintTest;

all_shift_impls!(BUint);

impl<const N: usize> Sub for BUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        expect!(self.checked_sub(rhs), "attempt to subtract with overflow")
    }
}

op_ref_impl!(Sub<BUint<N>> for BUint, sub);

impl<const N: usize> SubAssign for BUint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

assign_ref_impl!(SubAssign<BUint<N>> for BUint, sub_assign);

#[cfg(test)]
mod tests {
    use crate::U128;
}