use super::BIint;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::{expect, op_ref_impl, assign_ref_impl, all_shift_impls};
use crate::ExpType;

impl<const N: usize> BIint<N> {
    #[cfg(debug_assertions)]
    pub const fn add(self, rhs: Self) -> Self {
        expect!(self.checked_add(rhs), "attempt to add with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
    #[cfg(debug_assertions)]
    pub const fn sub(self, rhs: Self) -> Self {
        expect!(self.checked_sub(rhs), "attempt to subtract with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
    #[cfg(debug_assertions)]
    pub const fn neg(self) -> Self {
        expect!(self.checked_neg(), "attempt to negate with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn neg(self) -> Self {
        self.wrapping_neg()
    }
}

impl<const N: usize> Add<Self> for BIint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

op_ref_impl!(Add<BIint<N>> for BIint, add);

impl<const N: usize> AddAssign for BIint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

assign_ref_impl!(AddAssign<BIint<N>> for BIint, add_assign);

impl<const N: usize> BitAnd for BIint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self {
            uint: self.uint & rhs.uint,
        }
    }
}

op_ref_impl!(BitAnd<BIint<N>> for BIint, bitand);

impl<const N: usize> BitAndAssign for BIint<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

assign_ref_impl!(BitAndAssign<BIint<N>> for BIint, bitand_assign);

impl<const N: usize> BitOr for BIint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self {
            uint: self.uint | rhs.uint,
        }
    }
}

op_ref_impl!(BitOr<BIint<N>> for BIint, bitor);

impl<const N: usize> BitOrAssign for BIint<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

assign_ref_impl!(BitOrAssign<BIint<N>> for BIint, bitor_assign);

impl<const N: usize> BitXor for BIint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Self {
            uint: self.uint ^ rhs.uint,
        }
    }
}

op_ref_impl!(BitXor<BIint<N>> for BIint, bitxor);

impl<const N: usize> BitXorAssign for BIint<N> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

assign_ref_impl!(BitXorAssign<BIint<N>> for BIint, bitxor_assign);

impl<const N: usize> DivAssign for BIint<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

assign_ref_impl!(DivAssign<BIint<N>> for BIint, div_assign);

impl<const N: usize> Mul for BIint<N> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn mul(self, rhs: Self) -> Self {
        expect!(self.checked_mul(rhs), "attempt to multiply with overflow")
    }
    #[cfg(not(debug_assertions))]
    fn mul(self, rhs: Self) -> Self {
        self.wrapping_mul(rhs)
    }
}

op_ref_impl!(Mul<BIint<N>> for BIint, mul);

impl<const N: usize> MulAssign for BIint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

assign_ref_impl!(MulAssign<BIint<N>> for BIint, mul_assign);

impl<const N: usize> BIint<N> {
    pub const fn not(self) -> Self {
        Self {
            uint: self.uint.not(),
        }
    }
}

impl<const N: usize> Not for BIint<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self::not(self)
    }
}

impl<const N: usize> Not for &BIint<N> {
    type Output = BIint<N>;

    fn not(self) -> BIint<N> {
        (*self).not()
    }
}

impl<const N: usize> Neg for BIint<N> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn neg(self) -> Self {
        expect!(self.checked_neg(), "attempt to negative with overflow")
    }
    #[cfg(not(debug_assertions))]
    fn neg(self) -> Self {
        self.wrapping_neg()
    }
}

impl<const N: usize> Neg for &BIint<N> {
    type Output = BIint<N>;

    fn neg(self) -> BIint<N> {
        (*self).neg()
    }
}

impl<const N: usize> RemAssign for BIint<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

assign_ref_impl!(RemAssign<BIint<N>> for BIint, rem_assign);

impl<const N: usize> Shl<ExpType> for BIint<N> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn shl(self, rhs: ExpType) -> Self {
        expect!(self.checked_shl(rhs), "attempt to shift left with overflow")
    }
    #[cfg(not(debug_assertions))]
    fn shl(self, rhs: ExpType) -> Self {
        self.wrapping_shl(rhs)
    }
}

op_ref_impl!(Shl<ExpType> for BIint, shl);

impl<const N: usize> ShlAssign<ExpType> for BIint<N> {
    fn shl_assign(&mut self, rhs: ExpType) {
        *self = self.shl(rhs);
    }
}

assign_ref_impl!(ShlAssign<ExpType> for BIint, shl_assign);

impl<const N: usize> Shr<ExpType> for BIint<N> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn shr(self, rhs: ExpType) -> Self {
        expect!(self.checked_shr(rhs), "attempt to shift left with overflow")
    }
    #[cfg(not(debug_assertions))]
    fn shr(self, rhs: ExpType) -> Self {
        self.wrapping_shr(rhs)
    }
}

op_ref_impl!(Shr<ExpType> for BIint, shr);

impl<const N: usize> ShrAssign<ExpType> for BIint<N> {
    fn shr_assign(&mut self, rhs: ExpType) {
        *self = self.shr(rhs);
    }
}

assign_ref_impl!(ShrAssign<ExpType> for BIint, shr_assign);

use crate::uint::BUint;

all_shift_impls!(BIint);

impl<const N: usize> Sub for BIint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::sub(self, rhs)
    }
}

op_ref_impl!(Sub<BIint<N>> for BIint, sub);

impl<const N: usize> SubAssign for BIint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

assign_ref_impl!(SubAssign<BIint<N>> for BIint, sub_assign);