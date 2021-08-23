use super::BIint;
use num_traits::ToPrimitive;
use core::convert::TryFrom;
use core::str::FromStr;
use crate::{TryFromIntError, ParseIntError};
use crate::digit::{Digit, self};
use crate::uint::BUint;
use crate::error::TryFromErrorReason::*;
use crate::{macros, ExpType};

impl<const N: usize> FromStr for BIint<N> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

macro_rules! from_int {
    ($($int: tt),*) => {
        $(impl<const N: usize> From<$int> for BIint<N> {
            fn from(int: $int) -> Self {
                const UINT_BITS: ExpType = $int::BITS as ExpType;
                let mut digits = if int.is_negative() {
                    [Digit::MAX; N]
                } else {
                    [0; N]
                };
                let mut i = 0;
                while i << digit::BIT_SHIFT < UINT_BITS {
                    digits[i] = (int >> (i << digit::BIT_SHIFT)) as Digit;
                    i += 1;
                }
                Self::from_digits(digits)
            }
        })*
    }
}

from_int!(i8, i16, i32, isize, i64, i128);

macro_rules! from_uint {
    ($($from: tt), *) => {
        $(impl<const N: usize> From<$from> for BIint<N> {
            fn from(int: $from) -> Self {
                let out = Self {
                    uint: int.into(),
                };
                if out.is_negative() {
                    panic!("too big")
                }
                out
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
        if f.is_sign_negative() {
            let x = BUint::try_from(-f)?;
            Ok(-Self::from_bits(x))
        } else {
            Ok(Self::from_bits(BUint::try_from(f)?))
        }
    }
}

impl<const N: usize> TryFrom<f64> for BIint<N> {
    type Error = TryFromIntError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if f < 0.0 {
            let x = BUint::try_from(-f)?;
            Ok(-Self::from_bits(x))
        } else {
            Ok(Self::from_bits(BUint::try_from(f)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::I128;
    use super::*;
    // TODO: write tests
}