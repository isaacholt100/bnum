macro_rules! from_int {
	($BInt: ident, $Digit: ident; $($int: tt),*) => {
		$(impl_const! {
			impl<const N: usize> const From<$int> for $BInt<N> {
				#[inline]
				fn from(int: $int) -> Self {
					let mut out = if int.is_negative() {
						!Self::ZERO
					} else {
						Self::ZERO
					};
					let mut i = 0;
					while i << crate::digit::$Digit::BIT_SHIFT < $int::BITS as usize {
						let d = (int >> (i << crate::digit::$Digit::BIT_SHIFT)) as $Digit;
						out.bits.digits[i] = d;
						i += 1;
					}
					out
				}
			}
		})*
	}
}

macro_rules! from_uint {
	($BInt: ident, $BUint: ident; $($from: tt), *) => {
		$(impl_const! {
			impl<const N: usize> const From<$from> for $BInt<N> {
				#[inline]
				fn from(int: $from) -> Self {
					let out = Self::from_bits($BUint::from(int));
					out
				}
			}
		})*
	}
}

use crate::cast::CastFrom;
use crate::errors::TryFromErrorReason::*;
use crate::errors::{ParseIntError, TryFromIntError};
use crate::nightly::impl_const;
use core::str::FromStr;

macro_rules! convert {
	($BUint: ident, $BInt: ident, $Digit: ident) => {
		impl<const N: usize> FromStr for $BInt<N> {
			type Err = ParseIntError;

			#[inline]
			fn from_str(src: &str) -> Result<Self, Self::Err> {
				Self::from_str_radix(src, 10)
			}
		}

		from_int!($BInt, $Digit; i8, i16, i32, i64, i128, isize);

		from_uint!($BInt, $BUint; u8, u16, u32, u64, u128, usize);

		impl_const! {
			impl<const N: usize> const From<bool> for $BInt<N> {
				#[inline]
				fn from(small: bool) -> Self {
					Self::cast_from(small)
				}
			}
		}

		crate::int::convert::all_try_int_impls!($BInt, $Digit);

		impl_const! {
			impl<const N: usize> const TryFrom<$BUint<N>> for $BInt<N> {
				type Error = TryFromIntError;

				#[inline]
				fn try_from(u: $BUint<N>) -> Result<Self, Self::Error> {
					if u.leading_ones() != 0 {
						Err(TryFromIntError {
							from: "$BUint",
							to: "$BInt",
							reason: TooLarge,
						})
					} else {
						Ok(Self::from_bits(u))
					}
				}
			}
		}

		#[cfg(test)]
		paste::paste! {
			mod [<$Digit _digit_tests>] {
				use crate::test::types::big_types::$Digit::*;
				use crate::test::types::I128;
				use crate::test;

				test::test_from! {
					function: <i128 as From>::from,
					from_types: (i8, i16, i32, i64, i128, u8, u16, u32, u64, bool)
				}

				test::test_from! {
					function: <i128 as TryFrom>::try_from,
					from_types: (usize, isize)
				}

				test::test_into! {
					function: <i128 as TryInto>::try_into,
					into_types: (u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, i128, isize)
				}
			}
		}
	};
}

crate::macro_impl!(convert);
