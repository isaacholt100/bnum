use super::BInt;
use crate::buint::BUint;
use crate::digit::{self, Digit};
use crate::errors::TryFromErrorReason::*;
use crate::errors::{ParseIntError, TryFromIntError};
use crate::nightly::impl_const;
use crate::cast::CastFrom;
use core::str::FromStr;

impl<const N: usize> FromStr for BInt<N> {
    type Err = ParseIntError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

macro_rules! from_int {
    ($($int: tt),*) => {
        $(impl_const! {
			impl<const N: usize> const From<$int> for BInt<N> {
				#[inline]
				fn from(int: $int) -> Self {
					let mut out = if int.is_negative() {
						!Self::ZERO
					} else {
						Self::ZERO
					};
					let mut i = 0;
					while i << digit::BIT_SHIFT < $int::BITS as usize {
						let d = (int >> (i << digit::BIT_SHIFT)) as Digit;
						out.bits.digits[i] = d;
						i += 1;
					}
					out
				}
			}
		})*
    }
}

from_int!(i8, i16, i32, i64, i128, isize);

macro_rules! from_uint {
    ($($from: tt), *) => {
        $(impl_const! {
			impl<const N: usize> const From<$from> for BInt<N> {
				#[inline]
				fn from(int: $from) -> Self {
					let out = Self::from_bits(BUint::from(int));
					out
				}
			}
		})*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);

impl_const! {
    impl<const N: usize> const From<bool> for BInt<N> {
        #[inline]
        fn from(small: bool) -> Self {
            Self::cast_from(small)
        }
    }
}

crate::int::convert::all_try_int_impls!(BInt);

impl_const! {
    impl<const N: usize> const TryFrom<BUint<N>> for BInt<N> {
        type Error = TryFromIntError;

        #[inline]
        fn try_from(u: BUint<N>) -> Result<Self, Self::Error> {
            if u.leading_ones() != 0 {
                Err(TryFromIntError {
                    from: "BUint",
                    to: "BInt",
                    reason: TooLarge,
                })
            } else {
                Ok(Self::from_bits(u))
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
