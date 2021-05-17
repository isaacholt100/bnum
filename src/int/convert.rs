use super::Bint;
use num_traits::ToPrimitive;
use std::convert::TryFrom;
use std::str::FromStr;
use crate::{TryFromIntError, ParseIntError};
use crate::digit::{Digit, SignedDigit, DIGIT_BITS};
use crate::uint::BUint;

impl<const N: usize> FromStr for Bint<N> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

macro_rules! from_iint {
    ($($from: tt -> $as: ty), *) => {
        $(impl<const N: usize> From<$from> for Bint<N> {
            fn from(int: $from) -> Self {
                (int as i128).into()
            }
        })*
    }
}

from_iint!(i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64);

impl<const N: usize> From<i128> for Bint<N> {
    fn from(int: i128) -> Self {
        println!("{}", int as Digit);
        Self {
            signed_digit: (int >> DIGIT_BITS) as SignedDigit,
            uint: (int as Digit).into(),
        }
    }
}

macro_rules! from_uint {
    ($($from: tt), *) => {
        $(impl<const N: usize> From<$from> for Bint<N> {
            fn from(int: $from) -> Self {
                Self {
                    signed_digit: 0,
                    uint: int.into(),
                }
            }
        })*
    }
}

from_uint!(u8, u16, u32, usize, u64, u128);

impl<const N: usize> From<bool> for Bint<N> {
    fn from(small: bool) -> Self {
        if small {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}