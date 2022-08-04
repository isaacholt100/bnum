use crate::errors;
use crate::ExpType;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

macro_rules! ops {
	($BUint: ident, $BInt: ident, $Digit: ident) => {
		crate::nightly::impl_const! {
			impl<const N: usize> const Neg for $BInt<N> {
				type Output = Self;

				#[inline]
				fn neg(self) -> Self {
					#[cfg(debug_assertions)]
					return errors::option_expect!(self.checked_neg(), errors::err_msg!("attempt to negate with overflow"));

					#[cfg(not(debug_assertions))]
					self.wrapping_neg()
				}
			}
		}

		crate::nightly::impl_const! {
			impl<const N: usize> const Neg for &$BInt<N> {
				type Output = $BInt<N>;

				#[inline]
				fn neg(self) -> $BInt<N> {
					(*self).neg()
				}
			}
		}

		crate::nightly::impl_const! {
			impl<const N: usize> const BitAnd for $BInt<N> {
				type Output = Self;

				#[inline]
				fn bitand(self, rhs: Self) -> Self {
					Self::from_bits(self.bits & rhs.bits)
				}
			}
		}

		crate::nightly::impl_const! {
			impl<const N: usize> const BitOr for $BInt<N> {
				type Output = Self;

				#[inline]
				fn bitor(self, rhs: Self) -> Self {
					Self::from_bits(self.bits | rhs.bits)
				}
			}
		}

		crate::nightly::impl_const! {
			impl<const N: usize> const BitXor for $BInt<N> {
				type Output = Self;

				#[inline]
				fn bitxor(self, rhs: Self) -> Self {
					Self::from_bits(self.bits ^ rhs.bits)
				}
			}
		}

		crate::nightly::impl_const! {
			impl<const N: usize> const Div for $BInt<N> {
				type Output = Self;

				fn div(self, rhs: Self) -> Self {
					if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
						panic!(crate::errors::err_msg!("attempt to divide with overflow"))
					} else {
						if rhs.is_zero() {
							errors::div_zero!()
						}
						self.div_rem_unchecked(rhs).0
					}
				}
			}
		}

		crate::nightly::impl_const! {
			impl<const N: usize> const Not for $BInt<N> {
				type Output = Self;

				#[inline]
				fn not(self) -> Self {
					Self::from_bits(!self.bits)
				}
			}
		}

		crate::nightly::impl_const! {
			impl<const N: usize> const Rem for $BInt<N> {
				type Output = Self;

				fn rem(self, rhs: Self) -> Self {
					if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
						panic!(crate::errors::err_msg!("attempt to calculate remainder with overflow"))
					} else {
						if rhs.is_zero() {
							errors::rem_zero!()
						}
						self.div_rem_unchecked(rhs).1
					}
				}
			}
		}

		crate::int::ops::impls!($BInt, $BUint, $BInt);

		#[cfg(test)]
		paste::paste! {
			mod [<$Digit _digit_tests>] {
				use super::*;
				use crate::test::{debug_skip, test_bignum, types::itest};
				use crate::test::types::big_types::$Digit::*;

				crate::int::ops::tests!(itest);

				test_bignum! {
					function: <itest>::neg(a: itest),
					skip: debug_skip!(a == itest::MIN)
				}
			}
		}
	};
}

crate::macro_impl!(ops);
