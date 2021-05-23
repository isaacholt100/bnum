use super::BUint;
use num_traits::ToPrimitive;
use core::convert::{TryFrom, TryInto};
use core::str::FromStr;
use crate::{TryFromIntError, ParseIntError};
use crate::digit;

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

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(impl<const N: usize> From<$uint> for BUint<N> {
            fn from(int: $uint) -> Self {
                let mut out = Self::ZERO;
                out.digits[0] = int as u64;
                out
            }
        })*
    }
}

from_uint!(u8, u16, u32, usize, u64);

impl<const N: usize> TryFrom<f64> for BUint<N> {
    type Error = TryFromIntError;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if !f.is_finite() {
            return Err("not finite");
        }
        let f = f.trunc();
        if f == 0.0 {
            return Ok(Self::ZERO);
        }
        use num_traits::float::FloatCore;
        use core::cmp::Ordering;
        let (mantissa, exponent, sign) = FloatCore::integer_decode(f);
        if sign == -1 {
            return Err("negative float");
        }
        let out = Self::from(mantissa);
        match exponent.cmp(&0) {
            Ordering::Greater => {
                if out.bits() + exponent as usize >= Self::BITS {
                    Err("too large")
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
                let uint: $uint = int.try_into().ok().ok_or("Can't convert negative integer to uint")?;
                Ok(Self::from(uint))
            }
        })*
    }
}

try_from_iint!(i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64, i128 -> u128);

impl<const N: usize> From<u128> for BUint<N> {
    fn from(int: u128) -> Self {
        let (high, low) = digit::from_double_digit(int);

        let mut out = Self::ZERO;
        out.digits[0] = low;
        out.digits[1] = high;
        out
    }
}

macro_rules! impl_try_int {
    ($int: tt, $method: ident, $err: expr) => {
        impl<const N: usize> TryFrom<BUint<N>> for $int {
            type Error = TryFromIntError;
        
            fn try_from(uint: BUint<N>) -> Result<Self, Self::Error> {
                uint.$method().ok_or($err)
            }
        }
    }
}

impl_try_int!(u128, to_u128, "BUint is too large to cast to u128");
impl_try_int!(u64, to_u64, "BUint is too large to cast to u64");
impl_try_int!(usize, to_usize, "BUint is too large to cast to usize");
impl_try_int!(u32, to_u32, "BUint is too large to cast to u32");
impl_try_int!(u16, to_u16, "BUint is too large to cast to u16");
impl_try_int!(u8, to_u8, "BUint is too large to cast to u8");

impl_try_int!(i128, to_i128, "BUint is too large to cast to i128");
impl_try_int!(i64, to_i64, "BUint is too large to cast to i64");
impl_try_int!(isize, to_isize, "BUint is too large to cast to isize");
impl_try_int!(i32, to_i32, "BUint is too large to cast to i32");
impl_try_int!(i16, to_i16, "BUint is too large to cast to i16");
impl_try_int!(i8, to_i8, "BUint is too large to cast to i8");

impl<const N: usize> TryFrom<BUint<N>> for f32 {
    type Error = TryFromIntError;

    fn try_from(uint: BUint<N>) -> Result<Self, Self::Error> {
        Ok(uint.to_f32().unwrap())
    }
}

impl<const N: usize> TryFrom<BUint<N>> for f64 {
    type Error = TryFromIntError;

    fn try_from(uint: BUint<N>) -> Result<Self, Self::Error> {
        Ok(uint.to_f64().unwrap())
    }
}

impl<const N: usize> From<[u64; N]> for BUint<N> {
    fn from(digits: [u64; N]) -> Self {
        Self::from_digits(digits)
    }
}

impl<const N: usize> From<BUint<N>> for [u64; N] {
    fn from(uint: BUint<N>) -> Self {
        uint.digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::U128;
    use crate::digit::Digit;

    #[test]
    fn test_from_bool() {
        assert_eq!(U128::from(true), u128::from(true).into());
        assert_eq!(U128::from(false), u128::from(false).into());
    }

    #[test]
    fn test_from_char() {
        assert_eq!(U128::from('c'), u128::from('c').into());
    }

    test_unsigned! {
        test_name: test_from_str,
        method: {
            from_str("398475394875230495745");
        },
        converter: |result| {
            match result {
                Ok(u) => Ok(U128::from(u)),
                Err(_) => Err("")
            }
        }
    }

    #[test]
    fn it_converts_u8() {
        let u = 33u8;
        let a = U128::from(u);
        assert_eq!(a.digits[0], u.into());
        let into: u8 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
    }

    #[test]
    fn it_converts_u16() {
        let u = 48975u16;
        let a = U128::from(u);
        assert_eq!(a.digits[0], u.into());
        let into: u16 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
    }

    #[test]
    fn it_converts_u32() {
        let u = 903487869u32;
        let a = U128::from(u);
        assert_eq!(a.digits[0], u.into());
        let into: u32 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
    }

    #[test]
    fn it_converts_usize() {
        let u = 437948958usize;
        let a = U128::from(u);
        let into: usize = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
    }

    #[test]
    fn it_converts_u64() {
        let u = 9374563574234910234u64;
        let a = U128::from(u);
        let into: u64 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
    }

    #[test]
    fn it_converts_u128() {
        let u = 236543085093475734905834958390485903384u128;
        let a = U128::from(u);
        let into: u128 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 1);
    }

    #[test]
    fn it_converts_i8() {
        let u = 85i8;
        let a = U128::try_from(u).unwrap();
        assert_eq!(a.digits[0], u as Digit);
        let into: i8 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i16() {
        let u = 23422i16;
        let a = U128::try_from(u).unwrap();
        assert_eq!(a.digits[0], u as Digit);
        let into: i16 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i32() {
        let u = 5678943i32;
        let a = U128::try_from(u).unwrap();
        assert_eq!(a.digits[0], u as Digit);
        let into: i32 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_isize() {
        let u = 3284739isize;
        let a = U128::try_from(u).unwrap();
        let into: isize = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i64() {
        let u = 37458903849053498i64;
        let a = U128::try_from(u).unwrap();
        let into: i64 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 0);
        assert!(U128::try_from(-u).is_err());
    }

    #[test]
    fn it_converts_i128() {
        let u = 34759384858348039485094853849345352454i128;
        let a = U128::try_from(u).unwrap();
        let into: i128 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert_eq!(a.last_digit_index(), 1);
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