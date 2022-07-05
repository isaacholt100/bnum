use super::BInt;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Rem, DivAssign, Mul, MulAssign, Neg, Not, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use crate::ExpType;
        
impl<const N: usize> const Neg for BInt<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        #[cfg(debug_assertions)]
        return crate::errors::option_expect!(self.checked_neg(), crate::errors::err_msg!("attempt to negate with overflow"));

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
	use crate::test::{test_bignum, debug_skip, types::itest};

	crate::int::ops::tests!(itest);

	test_bignum! {
		function: <itest>::neg(a: itest),
		skip: debug_skip!(a == itest::MIN)
	}
}