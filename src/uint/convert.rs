use super::BUint;
use core::convert::{TryFrom, TryInto};
use crate::TryFromIntError;
use crate::error::TryFromErrorReason::*;
use crate::digit::{self, Digit};
use crate::macros::all_try_int_impls;
use crate::ExpType;
use core::{f32, f64};

impl<const N: usize> const From<bool> for BUint<N> {
    #[inline]
    fn from(small: bool) -> Self {
        if small {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

impl<const N: usize> const From<char> for BUint<N> {
    #[inline]
    fn from(c: char) -> Self {
        let u = c as u32;
        Self::from(u)
    }
}

//use crate::bound::{Assert, IsTrue};

macro_rules! from_uint {
    ($($uint: tt),*) => {
        $(
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

#[inline]
fn decode_f32(f: f32) -> (u64, i16, i8) {
    let bits = f.to_bits();
    let sign = if bits >> 31 == 0 { 1 } else { -1 };
    let mut exponent = ((bits >> 23) & 0xff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0x7fffff) << 1
    } else {
        (bits & 0x7fffff) | 0x800000
    };
    exponent -= 127 + 23;
    (mantissa as u64, exponent, sign)
}

#[inline]
fn decode_f64(f: f64) -> (u64, i16, i8) {
    let bits = f.to_bits();
    let sign = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

macro_rules! try_from_float {
    ($float: ty, $decoder: ident) => {
        impl<const N: usize> TryFrom<$float> for BUint<N> {
            type Error = TryFromIntError;
        
            #[inline]
            fn try_from(f: $float) -> Result<Self, Self::Error> {
                if !f.is_finite() {
                    return Err(TryFromIntError {
                        from: stringify!($float),
                        to: "BUint",
                        reason: NotFinite,
                    });
                }
                let f = f.trunc();
                if f == 0.0 {
                    return Ok(Self::ZERO);
                }
                use core::cmp::Ordering;
                let (mantissa, exponent, sign) = $decoder(f);
                if sign == -1 {
                    return Err(TryFromIntError {
                        from: stringify!($float),
                        to: "BUint",
                        reason: Negative,
                    });
                }
                let out = Self::from(mantissa);
                match exponent.cmp(&0) {
                    Ordering::Greater => {
                        if out.bits() + exponent as ExpType >= Self::BITS {
                            Err(TryFromIntError {
                                from: stringify!($float),
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
    }
}

try_from_float!(f32, decode_f32);
try_from_float!(f64, decode_f64);

macro_rules! try_from_iint {
    ($($int: tt -> $uint: tt),*) => {
        $(impl<const N: usize> TryFrom<$int> for BUint<N> {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(int: $int) -> Result<Self, Self::Error> {
                let bits: $uint = int
                    .try_into()
                    .ok()
                    .ok_or(TryFromIntError {
                        from: stringify!($int),
                        to: "BUint",
                        reason: Negative,
                    })?;
                Ok(Self::from(bits))
            }
        })*
    }
}

try_from_iint!(i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64, i128 -> u128);

all_try_int_impls!(BUint);

impl<const N: usize> const From<[Digit; N]> for BUint<N> {
    #[inline]
    fn from(digits: [Digit; N]) -> Self {
        Self::from_digits(digits)
    }
}

impl<const N: usize> const From<BUint<N>> for [Digit; N] {
    #[inline]
    fn from(uint: BUint<N>) -> Self {
        uint.digits
    }
}

#[cfg(test)]
mod tests {
    use crate::types::U128;
    use crate::test;

    test::test_from! {
        big: U128,
        primitive: u128,
        function: <From>::from,
        from_types: (u8, u16, u32, u64, u128, bool, char),
        converter: U128::from
    }

    fn result_ok_map<T: Into<U128>, E>(result: Result<T, E>) -> Option<U128> {
        result.ok().map(|u| u.into()) 
    }

    test::test_from! {
        big: U128,
        primitive: u128,
        function: <TryFrom>::try_from,
        from_types: (i8, i16, i32, i64, i128, isize, usize),
        converter: result_ok_map
    }

    test::test_into! {
        big: U128,
        primitive: u128,
        function: <TryInto>::try_into,
        from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize),
        converter: Result::ok
    }
	#[test]
	fn test_into() {
		let u = -1i128;
		let a: Result<u128, _> = u.try_into();
		let b: Result<U128, _> = u.try_into();
		//panic!("{:?}", a);
		assert_eq!(b.is_ok(), a.is_ok())
	}
	/*test::test_float_conv! {
        big: U128,
        primitive: u128,
        test_name: to_f32,
        function: <TryInto<f32>>::try_into,
        from: u128
    }*/
    // TODO: test float conversions
}