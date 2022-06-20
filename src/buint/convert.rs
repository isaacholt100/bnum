use super::BUint;
use crate::As;
use crate::errors::{TryFromErrorReason::*, TryFromIntError};
use crate::digit::{self, Digit};
use crate::ExpType;
use core::{f32, f64};

impl<const N: usize> const From<bool> for BUint<N> {
    #[inline]
    fn from(small: bool) -> Self {
        small.as_()
    }
}

impl<const N: usize> const From<char> for BUint<N> {
    #[inline]
    fn from(c: char) -> Self {
        let u = c as u32;
        Self::from(u)
    }
}

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
                        i += 1;
                    }
                    out
                }
            }
        )*
    }
}

from_uint!(u8, u16, u32, usize, u64, u128);

macro_rules! decode_float {
	($name: ident, $f: ty, $u: ty) => {
		pub fn $name(f: $f) -> ($u, i16) {
			const BITS: u32 = core::mem::size_of::<$f>() as u32 * 8;
			const MANT_MASK: $u = <$u>::MAX >> (BITS - (<$f>::MANTISSA_DIGITS - 1));
			const EXP_MASK: $u = <$u>::MAX >> 1;
			const BIAS: i16 = <$f>::MAX_EXP as i16 - 1;

			let bits = f.to_bits();
			let exp = ((bits & EXP_MASK) >> (<$f>::MANTISSA_DIGITS - 1)) as i16;
			let mut mant = bits & MANT_MASK;
			if exp != 0 {
				mant |= (1 << (<$f>::MANTISSA_DIGITS - 1));
			}
			(mant, exp - (BIAS + <$f>::MANTISSA_DIGITS as i16 - 1))
		}
	};
}

decode_float!(decode_f32, f32, u32);
decode_float!(decode_f64, f64, u64);

const fn u32_bits(u: u32) -> ExpType {
    32 - u.leading_zeros() as ExpType
}

const fn u64_bits(u: u64) -> ExpType {
    64 - u.leading_zeros() as ExpType
}

macro_rules! try_from_float {
    ($float: ty, $decoder: ident, $mant_bits: ident) => {
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
				if f == 0.0 {
					return Ok(Self::ZERO);
				}
				if f.is_sign_negative() {
					return Err(TryFromIntError {
                        from: stringify!($float),
                        to: "BUint",
                        reason: Negative,
                    });
				}
				let (mut mant, exp) = $decoder(f);
				if exp.is_negative() {
					mant = mant.checked_shr((-exp) as ExpType).unwrap_or(0);
					if $mant_bits(mant) > Self::BITS {
						return Err(TryFromIntError {
							from: stringify!($float),
							to: "BUint",
							reason: TooLarge,
						});
					}
					Ok(mant.as_())
				} else {
					if $mant_bits(mant) + exp as ExpType > Self::BITS {
						return Err(TryFromIntError {
							from: stringify!($float),
							to: "BUint",
							reason: TooLarge,
						});
					}
					Ok(mant.as_::<Self>() << exp)
				}
            }
        }
    }
}

try_from_float!(f32, decode_f32, u32_bits);
try_from_float!(f64, decode_f64, u64_bits);

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

crate::int::convert::all_try_int_impls!(BUint);

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