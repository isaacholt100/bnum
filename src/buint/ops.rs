use super::{BUint, ExpType};
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::impl_ops;
use crate::digit::Digit;

impl<const N: usize> const Add<Digit> for BUint<N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Digit) -> Self {
        let mut out = Self::ZERO;
        let result = self.digits[0].carrying_add(rhs, false);
        out.digits[0] = result.0;
        let mut carry = result.1;
        let mut i = 1;
        while i < N {
            let result = self.digits[0].carrying_add(0, carry);
            out.digits[i] = result.0;
            carry = result.1;
            i += 1;
        }
        out
    }
}

impl<const N: usize> const BitAnd for BUint<N> {
    type Output = Self;

    #[inline]
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

impl<const N: usize> const BitOr for BUint<N> {
    type Output = Self;

    #[inline]
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

impl<const N: usize> const BitXor for BUint<N> {
    type Output = Self;

    #[inline]
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

impl<const N: usize> const Div for BUint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
}

impl<const N: usize> const Div<Digit> for BUint<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Digit) -> Self {
        self.div_rem_digit(rhs).0
    }
}

impl<const N: usize> const Rem<Digit> for BUint<N> {
    type Output = Digit;

    #[inline]
    fn rem(self, rhs: Digit) -> Digit {
        self.div_rem_digit(rhs).1
    }
}

impl<const N: usize> const Not for BUint<N> {
    type Output = Self;

    #[inline]
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

impl<const N: usize> const Rem for BUint<N> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }
}

impl_ops!(BUint);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test::test_bignum;

	test_bignum! {
        function: <u128 as Add>::add(a: u128, b: u128),
		skip: a.checked_add(b).is_none()
    }

	test_bignum! {
		function: <u128 as BitAnd>::bitand(a: u128, b: u128)
	}

	test_bignum! {
        function: <u128 as BitOr>::bitor(a: u128, b: u128)
    }

	test_bignum! {
        function: <u128 as BitXor>::bitxor(a: u128, b: u128)
    }
    
    test_bignum! {
        function: <u128 as Div>::div(a: u128, b: u128),
		skip: a.checked_div(b).is_none()
    }
    
    test_bignum! {
        function: <u128 as Rem>::rem(a: u128, b: u128),
		skip: a.checked_rem(b).is_none()
    }
    
    test_bignum! {
        function: <u128 as Not>::not(a: u128)
    }

	test_bignum! {
        function: <u128 as Sub>::sub(a: u128, b: u128),
		skip: a.checked_sub(b).is_none()
    }

	test_bignum! {
        function: <u128 as Mul>::mul(a: u128, b: u128),
		skip: a.checked_mul(b).is_none()
    }
}