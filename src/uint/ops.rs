use super::BUint;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::{expect, op_ref_impl, all_shift_impls};

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

impl<T, const N: usize> AddAssign<T> for BUint<N> where Self: Add<T, Output = Self> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

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

impl<T, const N: usize> BitAndAssign<T> for BUint<N> where Self: BitAnd<T, Output = Self> {
    fn bitand_assign(&mut self, rhs: T) {
        *self = self.bitand(rhs);
    }
}

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

impl<T, const N: usize> BitOrAssign<T> for BUint<N> where Self: BitOr<T, Output = Self> {
    fn bitor_assign(&mut self, rhs: T) {
        *self = BitOr::bitor(*self, rhs);
    }
}

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

impl<T, const N: usize> BitXorAssign<T> for BUint<N> where Self: BitXor<T, Output = Self> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self = self.bitxor(rhs);
    }
}

impl<const N: usize> Div for BUint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
}

op_ref_impl!(Div<BUint<N>> for BUint, div);

impl<T, const N: usize> DivAssign<T> for BUint<N> where Self: Div<T, Output = Self> {
    fn div_assign(&mut self, rhs: T) {
        *self = self.div(rhs);
    }
}

impl<const N: usize> Mul for BUint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        expect!(self.checked_mul(rhs), "attempt to multiply with overflow")
    }
}

op_ref_impl!(Mul<BUint<N>> for BUint, mul);

impl<T, const N: usize> MulAssign<T> for BUint<N> where Self: Mul<T, Output = Self> {
    fn mul_assign(&mut self, rhs: T) {
        *self = self.mul(rhs);
    }
}

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

impl<T, const N: usize> RemAssign<T> for BUint<N> where Self: Rem<T, Output = Self> {
    fn rem_assign(&mut self, rhs: T) {
        *self = self.rem(rhs);
    }
}

impl<const N: usize> Shl<u32> for BUint<N> {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self {
        expect!(self.checked_shl(rhs), "attempt to shift left with overflow")
    }
}

op_ref_impl!(Shl<u32> for BUint, shl);

impl<T, const N: usize> ShlAssign<T> for BUint<N> where Self: Shl<T, Output = Self> {
    fn shl_assign(&mut self, rhs: T) {
        *self = self.shl(rhs);
    }
}

impl<const N: usize> Shr<u32> for BUint<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self {
        expect!(self.checked_shr(rhs), "attempt to shift right with overflow")
    }
}

op_ref_impl!(Shr<u32> for BUint, shr);

impl<T, const N: usize> ShrAssign<T> for BUint<N> where Self: Shr<T, Output = Self> {
    fn shr_assign(&mut self, rhs: T) {
        *self = self.shr(rhs);
    }
}

use crate::iint::BIint;

all_shift_impls!(BUint);

impl<const N: usize> Sub for BUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        expect!(self.checked_sub(rhs), "attempt to subtract with overflow")
    }
}

op_ref_impl!(Sub<BUint<N>> for BUint, sub);

impl<T, const N: usize> SubAssign<T> for BUint<N> where Self: Sub<T, Output = Self> {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn test_bitand() {
        let a = 934539445645648753475987u128;
        let b = 9384592074589749679475697u128;
        assert_eq!(U128::from(a) & U128::from(b), U128::from(a & b));
    }
    #[test]
    fn test_bitor() {
        let a = 345797465893865897346983548797u128;
        let b = 23496529846782457694586979779465u128;
        assert_eq!(U128::from(a) | U128::from(b), U128::from(a | b));
    }
    #[test]
    fn test_bitxor() {
        let a = 1873649845684389645897456757697889u128;
        let b = 2384689734763458437865873468485789u128;
        assert_eq!(U128::from(a) ^ U128::from(b), U128::from(a ^ b));
    }
    #[test]
    fn test_not() {
        let a = 2903646984856974586794084057698457689u128;
        assert_eq!(!U128::from(a), U128::from(!a));
    }
}