use super::BIint;
use num_traits::ToPrimitive;
use core::convert::TryFrom;
use core::str::FromStr;
use crate::{TryFromIntError, ParseIntError};
use crate::digit::{Digit, self};
use crate::uint::BUint;
use crate::error::TryFromErrorReason::*;
use crate::macros;

impl<const N: usize> FromStr for BIint<N> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

macro_rules! from_iint {
    ($($from: tt -> $as: ty), *) => {
        $(impl<const N: usize> From<$from> for BIint<N> {
            fn from(int: $from) -> Self {
                (int as i128).into()
            }
        })*
    }
}

from_iint!(i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64);

impl<const N: usize> From<i128> for BIint<N> {
    fn from(int: i128) -> Self {
        if int < 0 {
            let mut digits = [0; N];
            digits[0] = int as Digit;
            digits[N - 1] = (int >> digit::BITS) as Digit;
            Self {
                uint: digits.into(),
            }
        } else {
            Self {
                uint: (int as u128).into(),
            }
        }
    }
}

macro_rules! from_uint {
    ($($from: tt), *) => {
        $(impl<const N: usize> From<$from> for BIint<N> {
            fn from(int: $from) -> Self {
                Self {
                    uint: int.into(),
                }
            }
        })*
    }
}

from_uint!(u8, u16, u32, usize, u64, u128);

impl<const N: usize> From<bool> for BIint<N> {
    fn from(small: bool) -> Self {
        if small {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

macros::all_try_int_impls!(BIint);

impl<const N: usize> TryFrom<BUint<N>> for BIint<N> {
    type Error = TryFromIntError;

    fn try_from(u: BUint<N>) -> Result<Self, Self::Error> {
        if u.leading_ones() != 0 {
            Err(TryFromIntError {
                from: "BUint",
                to: "BIint",
                reason: TooLarge,   
            })
        } else {
            Ok(Self {
                uint: u,
            })
        }
    }
}

impl<const N: usize> TryFrom<f32> for BIint<N> {
    type Error = TryFromIntError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        if f < 0.0 {
            let x = BUint::try_from(-f)?;
            Ok(-Self::from_buint(x))
        } else {
            Ok(Self::from_buint(BUint::try_from(f)?))
        }
    }
}

impl<const N: usize> TryFrom<f64> for BIint<N> {
    type Error = TryFromIntError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if f < 0.0 {
            let x = BUint::try_from(-f)?;
            Ok(-Self::from_buint(x))
        } else {
            Ok(Self::from_buint(BUint::try_from(f)?))
        }
    }
}