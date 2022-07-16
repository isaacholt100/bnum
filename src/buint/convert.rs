use super::BUint;
use crate::digit::{self, Digit};
use crate::errors::{TryFromErrorReason::*, TryFromIntError};
use crate::nightly::impl_const;
use crate::cast::CastFrom;

impl_const! {
    impl<const N: usize> const From<bool> for BUint<N> {
        #[inline]
        fn from(small: bool) -> Self {
            Self::cast_from(small)
        }
    }
}

impl_const! {
    impl<const N: usize> const From<char> for BUint<N> {
        #[inline]
        fn from(c: char) -> Self {
            Self::cast_from(c)
        }
    }
}

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(impl_const! {
            impl<const N: usize> const From<$uint> for BUint<N> {
                #[inline]
                fn from(int: $uint) -> Self {
                    const UINT_BITS: usize = $uint::BITS as usize;
                    let mut out = Self::ZERO;
                    let mut i = 0;
                    while i << digit::BIT_SHIFT < UINT_BITS {
                        let d = (int >> (i << digit::BIT_SHIFT)) as Digit;
                        if d != 0 {
                            out.digits[i] = d;
                        }
                        i += 1;
                    }
                    out
                }
            }
		})*
    }
}

from_uint!(u8, u16, u32, u64, u128, usize);
// TODO: decide whether it should be TryFrom<usize> or From<usize>, same for BInt

macro_rules! try_from_iint {
    ($($int: tt -> $uint: tt),*) => {
        $(impl_const! {
			impl<const N: usize> const TryFrom<$int> for BUint<N> {
				type Error = TryFromIntError;

				#[inline]
				fn try_from(int: $int) -> Result<Self, Self::Error> {
					if int.is_negative() {
						return Err(TryFromIntError {
							from: stringify!($int),
							to: "BUint",
							reason: Negative,
						});
					}
					let bits = int as $uint;
					Ok(Self::from(bits))
				}
			}
		})*
    }
}

try_from_iint!(i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64, i128 -> u128);

crate::int::convert::all_try_int_impls!(BUint);

impl_const! {
    impl<const N: usize> const From<[Digit; N]> for BUint<N> {
        #[inline]
        fn from(digits: [Digit; N]) -> Self {
            Self::from_digits(digits)
        }
    }
}

impl_const! {
    impl<const N: usize> const From<BUint<N>> for [Digit; N] {
        #[inline]
        fn from(uint: BUint<N>) -> Self {
            uint.digits
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test;

    test::test_from! {
        function: <u128 as From>::from,
        from_types: (u8, u16, u32, u64, u128, bool, char)
    }

    test::test_from! {
        function: <u128 as TryFrom>::try_from,
        from_types: (i8, i16, i32, i64, i128, isize, usize)
    }

    test::test_into! {
        function: <u128 as TryInto>::try_into,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }
}
