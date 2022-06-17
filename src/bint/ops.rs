use super::BInt;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Rem, DivAssign, Mul, MulAssign, Neg, Not, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::macros::{option_expect};
use crate::ExpType;
use crate::error;
        
impl<const N: usize> const Neg for BInt<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_neg(), error::err_msg!("attempt to negate with overflow"));

        #[cfg(not(debug_assertions))]
        self.wrapping_neg()
    }
}

impl<const N: usize> const Neg for &BInt<N> {
    type Output = BInt<N>;

    #[inline]
    fn neg(self) -> BInt<N> {
        (*self).neg()
    }
}
        
impl<const N: usize> const BitAnd for BInt<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self::from_bits(self.bits & rhs.bits)
    }
}

impl<const N: usize> const BitOr for BInt<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self::from_bits(self.bits | rhs.bits)
    }
}

impl<const N: usize> const BitXor for BInt<N> {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self::from_bits(self.bits ^ rhs.bits)
    }
}
        
impl<const N: usize> const Not for BInt<N> {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self::from_bits(!self.bits)
    }
}

crate::int::ops::impls!(BInt);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test::{test_bignum, debug_skip};

	crate::int::ops::tests!(i128);

	test_bignum! {
		function: <i128>::neg(a: i128),
		skip: debug_skip!(a == i128::MIN)
	}
}