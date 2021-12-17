use super::BUint;
use num_traits::ToPrimitive;
use core::convert::{TryFrom, TryInto};
use core::str::FromStr;
use crate::{TryFromIntError, ParseIntError};
use crate::error::TryFromErrorReason::*;
use crate::digit::{self, Digit};
use crate::macros::all_try_int_impls;

impl<const N: usize> FromStr for BUint<N> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

impl<const N: usize> From<bool> for BUint<N> {
    fn from(small: bool) -> Self {
        if small {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

impl<const N: usize> From<char> for BUint<N> {
    fn from(c: char) -> Self {
        let u: u32 = c.into();
        u.into()
    }
}

use crate::bound::{Assert, IsTrue};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
            impl<const N: usize> From<$uint> for BUint<N> {
                fn from(int: $uint) -> Self {
                    const UINT_BITS: usize = $uint::BITS as usize;
                    let mut out = Self::ZERO;
                    let mut i = 0;
                    while i << digit::BIT_SHIFT < UINT_BITS {
                        let d = (int >> (i << digit::BIT_SHIFT)) as Digit;
                        if d != 0 {
                            out.digits[i] = d;
                        }
                        //out.digits[i] = (int >> (i << digit::BIT_SHIFT)) as Digit;
                        i += 1;
                    }
                    out
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, usize, u64, u128);

impl<const N: usize> TryFrom<f64> for BUint<N> {
    type Error = TryFromIntError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if !f.is_finite() {
            return Err(TryFromIntError {
                from: "f64",
                to: "BUint",
                reason: NotFinite,
            });
        }
        let f = f.trunc();
        if f == 0.0 {
            return Ok(Self::ZERO);
        }
        use num_traits::float::FloatCore;
        use core::cmp::Ordering;
        let (mantissa, exponent, sign) = FloatCore::integer_decode(f);
        if sign == -1 {
            return Err(TryFromIntError {
                from: "f64",
                to: "BUint",
                reason: Negative,
            });
        }
        let out = Self::from(mantissa);
        match exponent.cmp(&0) {
            Ordering::Greater => {
                if out.bits() + exponent as usize >= Self::BITS {
                    Err(TryFromIntError {
                        from: "f64",
                        to: "BUint",
                        reason: TooLarge,
                    })
                } else {
                    Ok(out << exponent)
                }
            },
            Ordering::Equal => Ok(out),
            Ordering::Less => Ok(out >> (-exponent)),
        }
    }
}

impl<const N: usize> TryFrom<f32> for BUint<N> {
    type Error = TryFromIntError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        Self::try_from(f as f64)
    }
}

macro_rules! try_from_iint {
    ($($iint: tt -> $uint: tt),*) => {
        $(impl<const N: usize> TryFrom<$iint> for BUint<N> {
            type Error = TryFromIntError;

            fn try_from(int: $iint) -> Result<Self, Self::Error> {
                let uint: $uint = int
                    .try_into()
                    .ok()
                    .ok_or(TryFromIntError {
                        from: stringify!($iint),
                        to: "BUint",
                        reason: Negative,
                    })?;
                Ok(Self::from(uint))
            }
        })*
    }
}

try_from_iint!(i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64, i128 -> u128);

all_try_int_impls!(BUint);

impl<const N: usize> TryFrom<BUint<N>> for f32 {
    type Error = TryFromIntError;

    fn try_from(uint: BUint<N>) -> Result<Self, Self::Error> {
        Ok(uint.as_f32())
    }
}

impl<const N: usize> TryFrom<BUint<N>> for f64 {
    type Error = TryFromIntError;

    fn try_from(uint: BUint<N>) -> Result<Self, Self::Error> {
        Ok(uint.as_f64())
    }
}

impl<const N: usize> From<[Digit; N]> for BUint<N> {
    fn from(digits: [Digit; N]) -> Self {
        Self::from_digits(digits)
    }
}

impl<const N: usize> From<BUint<N>> for [Digit; N] {
    fn from(uint: BUint<N>) -> Self {
        uint.digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::U128;

    #[test]
    fn from_bool() {
        assert_eq!(U128::from(true), u128::from(true).into());
        assert_eq!(U128::from(false), u128::from(false).into());
    }

    #[test]
    fn from_char() {
        assert_eq!(U128::from('c'), u128::from('c').into());
    }

    test_unsigned! {
        name: from_str,
        method: {
            from_str("398475394875230495745");
        },
        converter: |result| {
            match result {
                Ok(u) => Ok(U128::from(u)),
                Err(_) => unreachable!()
            }
        }
    }

    #[test]
    fn it_converts_u8() {
        let u = 33u8;
        let a = U128::from(u);
        let into: u8 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_u16() {
        let u = 48975u16;
        let a = U128::from(u);
        let into: u16 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_u32() {
        let u = 903487869u32;
        let a = U128::from(u);
        let into: u32 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_usize() {
        let u = 437948958usize;
        let a = U128::from(u);
        let into: usize = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_u64() {
        let u = 9374563574234910234u64;
        let a = U128::from(u);
        let into: u64 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_u128() {
        let u = 236543085093475734905834958390485903384u128;
        let a = U128::from(u);
        let into: u128 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_i8() {
        let u = 85i8;
        let a = U128::try_from(u).unwrap();
        let into: i8 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i16() {
        let u = 23422i16;
        let a = U128::try_from(u).unwrap();
        let into: i16 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i32() {
        let u = 5678943i32;
        let a = U128::try_from(u).unwrap();
        let into: i32 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_isize() {
        let u = 3284739isize;
        let a = U128::try_from(u).unwrap();
        let into: isize = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i64() {
        let u = 37458903849053498i64;
        let a = U128::try_from(u).unwrap();
        let into: i64 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i128() {
        let u = 34759384858348039485094853849345352454i128;
        let a = U128::try_from(u).unwrap();
        let into: i128 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_f32() {
        let f = 394346435456455456658798.9585998f32;
        let a = U128::try_from(f).unwrap();
        let u = f as u128;
        assert_eq!(a, u.into());
        assert_eq!(a.to_f32().unwrap(), u as f32);
    }

    #[test]
    fn it_converts_f64() {
        let f = 456475983445645655463447569.4585f64;
        let a = U128::try_from(f).unwrap();
        let u = f as u128;
        assert_eq!(a, u.into());
        assert_eq!(a.to_f64().unwrap(), u as f64);
    }
}