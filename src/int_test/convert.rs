use super::BintTest;
use num_traits::ToPrimitive;
use std::convert::TryFrom;
use std::str::FromStr;
use crate::{TryFromIntError, ParseIntError};
use crate::digit::{Digit, SignedDigit, self};
use crate::uint::BUint;

impl<const N: usize> FromStr for BintTest<N> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

macro_rules! from_iint {
    ($($from: tt -> $as: ty), *) => {
        $(impl<const N: usize> From<$from> for BintTest<N> {
            fn from(int: $from) -> Self {
                (int as i128).into()
            }
        })*
    }
}

from_iint!(i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64);

impl<const N: usize> From<i128> for BintTest<N> {
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
        $(impl<const N: usize> From<$from> for BintTest<N> {
            fn from(int: $from) -> Self {
                Self {
                    uint: int.into(),
                }
            }
        })*
    }
}

from_uint!(u8, u16, u32, usize, u64, u128);

impl<const N: usize> From<bool> for BintTest<N> {
    fn from(small: bool) -> Self {
        if small {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}