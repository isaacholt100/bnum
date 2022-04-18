use super::Bint;
use num_traits::ToPrimitive;
use core::convert::TryFrom;
use core::str::FromStr;
use crate::{TryFromIntError, ParseIntError};
use crate::digit::{Digit, self};
use crate::uint::BUint;
use crate::error::TryFromErrorReason::*;
use crate::{macros};

impl<const N: usize> FromStr for Bint<N> {
    type Err = ParseIntError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

macro_rules! from_int {
    ($($int: tt),*) => {
        $(impl<const N: usize> const From<$int> for Bint<N> {
            #[inline]
            fn from(int: $int) -> Self {
                let initial_digit = if int.is_negative() {
                    Digit::MAX
                } else {
                    0
                };
                let mut digits = [initial_digit; N];
                let mut i = 0;
                while i << digit::BIT_SHIFT < $int::BITS as usize {
                    let d = (int >> (i << digit::BIT_SHIFT)) as Digit;
                    if d != initial_digit {
                        digits[i] = d;
                    }
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
        $(impl<const N: usize> From<$from> for Bint<N> {
            #[inline]
            fn from(int: $from) -> Self {
                let out = Self::from_bits(int.into());
                if out.is_negative() {
                    panic!("too big")// TODO: make clearer
                }
                out
            }
        })*
    }
}

from_uint!(u8, u16, u32, usize, u64, u128);

impl<const N: usize> const From<bool> for Bint<N> {
    #[inline]
    fn from(small: bool) -> Self {
        if small {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

macros::all_try_int_impls!(Bint);

impl<const N: usize> TryFrom<BUint<N>> for Bint<N> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(u: BUint<N>) -> Result<Self, Self::Error> {
        if u.leading_ones() != 0 {
            Err(TryFromIntError {
                from: "BUint",
                to: "Bint",
                reason: TooLarge,   
            })
        } else {
            Ok(Self::from_bits(u))
        }
    }
}

impl<const N: usize> TryFrom<f32> for Bint<N> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(f: f32) -> Result<Self, Self::Error> {
        if f.is_sign_negative() {
            let x = BUint::try_from(-f)?;
            Ok(-Self::from_bits(x))
        } else {
            Ok(Self::from_bits(BUint::try_from(f)?))
        }
    }
}

impl<const N: usize> TryFrom<f64> for Bint<N> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if f.is_sign_negative() {
            let x = BUint::try_from(-f)?;
            if x > Self::MIN.to_bits() {
                return Err(TryFromIntError {
                    from: stringify!($float),
                    to: "Bint",
                    reason: TooLarge,
                });
            }
            Ok(-Self::from_bits(x))
        } else {
            let x = BUint::try_from(f)?;
            if x > Self::MIN.to_bits() {
                return Err(TryFromIntError {
                    from: stringify!($float),
                    to: "Bint",
                    reason: TooLarge,
                });
            }
            Ok(Self::from_bits(x))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::I128;
    use super::*;
    use crate::test;
    use core::convert::TryInto;

    test::test_from! {
        big: I128,
        primitive: i128,
        function: <From>::from,
        from_types: (i8, i16, i32, i64, i128, u8, u16, u32, u64, bool),
        converter: I128::from
    }

    fn result_ok_map<T: Into<I128>, E>(result: Result<T, E>) -> Option<I128> {
        result.ok().map(|u| u.into()) 
    }

    test::test_from! {
        big: I128,
        primitive: i128,
        function: <TryFrom>::try_from,
        from_types: (usize, isize),
        converter: result_ok_map
    }

    test::test_into! {
        big: I128,
        primitive: i128,
        function: <TryInto>::try_into,
        from_types: (u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, i128, isize),
        converter: Result::ok
    }
    // TODO: test float conversions
}