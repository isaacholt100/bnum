use super::{BUint, ExpType};
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::{expect, op_ref_impl, all_shift_impls};
use crate::digit::Digit;

use crate::arithmetic;

impl<const N: usize> const Add<Digit> for BUint<N> {
    type Output = Self;

    fn add(self, rhs: Digit) -> Self {
        let mut out = Self::ZERO;
        let result = arithmetic::add_carry_unsigned(0, self.digits[0], rhs);
        out.digits[0] = result.0;
        let mut carry = result.1;
        let mut i = 1;
        while i < N {
            let result = arithmetic::add_carry_unsigned(carry, self.digits[i], 0);
            out.digits[i] = result.0;
            carry = result.1;
            i += 1;
        }
        out
    }
}

impl<const N: usize> const Add<Self> for BUint<N> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn add(self, rhs: Self) -> Self {
        expect!(self.checked_add(rhs), "attempt to add with overflow")
    }
    #[cfg(not(debug_assertions))]
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

op_ref_impl!(Add<BUint<N>> for BUint, add);

impl<T, const N: usize> const AddAssign<T> for BUint<N> where Self: ~const Add<T, Output = Self> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<const N: usize> const BitAnd for BUint<N> {
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

impl<T, const N: usize> const BitAndAssign<T> for BUint<N> where Self: ~const BitAnd<T, Output = Self> {
    fn bitand_assign(&mut self, rhs: T) {
        *self = BitAnd::bitand(*self, rhs);
    }
}

impl<const N: usize> const BitOr for BUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
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

impl<T, const N: usize> const BitOrAssign<T> for BUint<N> where Self: ~const BitOr<T, Output = Self> {
    fn bitor_assign(&mut self, rhs: T) {
        *self = BitOr::bitor(*self, rhs);
    }
}

impl<const N: usize> const BitXor for BUint<N> {
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

impl<T, const N: usize> const BitXorAssign<T> for BUint<N> where Self: ~const BitXor<T, Output = Self> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self = BitXor::bitxor(*self, rhs);
    }
}

impl<const N: usize> const Div for BUint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
}

impl<const N: usize> const Div<Digit> for BUint<N> {
    type Output = Self;

    fn div(self, rhs: Digit) -> Self {
        self.div_rem_digit(rhs).0
    }
}

op_ref_impl!(Div<BUint<N>> for BUint, div);

impl<T, const N: usize> const DivAssign<T> for BUint<N> where Self: ~const Div<T, Output = Self> {
    fn div_assign(&mut self, rhs: T) {
        *self = self.div(rhs);
    }
}

impl<const N: usize> const Mul for BUint<N> {
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

op_ref_impl!(Mul<BUint<N>> for BUint, mul);

impl<T, const N: usize> const MulAssign<T> for BUint<N> where Self: ~const Mul<T, Output = Self> {
    fn mul_assign(&mut self, rhs: T) {
        *self = self.mul(rhs);
    }
}

impl<const N: usize> const Not for BUint<N> {
    type Output = Self;

    fn not(self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = !self.digits[i];
            i += 1;
        }
        out
    }
}

impl<const N: usize> const Not for &BUint<N> {
    type Output = BUint<N>;

    fn not(self) -> BUint<N> {
        (*self).not()
    }
}

impl<const N: usize> const Rem for BUint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }
}

op_ref_impl!(Rem<BUint<N>> for BUint, rem);

impl<T, const N: usize> const RemAssign<T> for BUint<N> where Self: ~const Rem<T, Output = Self> {
    fn rem_assign(&mut self, rhs: T) {
        *self = self.rem(rhs);
    }
}

impl<const N: usize> const Shl<ExpType> for BUint<N> {
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

op_ref_impl!(Shl<ExpType> for BUint, shl);

impl<T, const N: usize> const ShlAssign<T> for BUint<N> where Self: ~const Shl<T, Output = Self> {
    fn shl_assign(&mut self, rhs: T) {
        *self = self.shl(rhs);
    }
}

impl<const N: usize> const Shr<ExpType> for BUint<N> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn shr(self, rhs: ExpType) -> Self {
        expect!(self.checked_shr(rhs), "attempt to shift right with overflow")
    }
    #[cfg(not(debug_assertions))]
    fn shr(self, rhs: ExpType) -> Self {
        self.wrapping_shr(rhs)
    }
}

op_ref_impl!(Shr<ExpType> for BUint, shr);

impl<T, const N: usize> const ShrAssign<T> for BUint<N> where Self: ~const Shr<T, Output = Self> {
    fn shr_assign(&mut self, rhs: T) {
        *self = self.shr(rhs);
    }
}

use crate::int::Bint;

all_shift_impls!(BUint);

impl<const N: usize> const Sub for BUint<N> {
    type Output = Self;

    #[cfg(debug_assertions)]
    fn sub(self, rhs: Self) -> Self {
        expect!(self.checked_sub(rhs), "attempt to subtract with overflow")
    }
    #[cfg(not(debug_assertions))]
    fn sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
}

op_ref_impl!(Sub<BUint<N>> for BUint, sub);

impl<T, const N: usize> const SubAssign<T> for BUint<N> where Self: ~const Sub<T, Output = Self> {
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn bitand() {
        let a = 934539445645648753475987u128;
        let b = 9384592074589749679475697u128;
        assert_eq!(U128::from(a) & U128::from(b), U128::from(a & b));
    }
    #[test]
    fn bitor() {
        let a = 345797465893865897346983548797u128;
        let b = 23496529846782457694586979779465u128;
        assert_eq!(U128::from(a) | U128::from(b), U128::from(a | b));
    }
    #[test]
    fn bitxor() {
        let a = 1873649845684389645897456757697889u128;
        let b = 2384689734763458437865873468485789u128;
        assert_eq!(U128::from(a) ^ U128::from(b), U128::from(a ^ b));
    }
    #[test]
    fn not() {
        let a = 2903646984856974586794084057698457689u128;
        assert_eq!(!U128::from(a), U128::from(!a));
    }
}