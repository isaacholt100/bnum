use crate::Exponent;
use crate::helpers::{Bits, One, Zero};
use core::ops::{Add, BitAnd, Neg, Shl, Shr};

mod float_from_uint;
mod uint_from_float;
pub use float_from_uint::*;
pub use uint_from_float::*;

pub trait FloatMantissa:
    Sized
    + Shl<Exponent, Output = Self>
    + Shr<Exponent, Output = Self>
    + Add<Self, Output = Self>
    + BitAnd<Self, Output = Self>
    + PartialEq
    + Bits
    + One
    + Zero
{
    fn is_power_of_two(self) -> bool;
}

macro_rules! impl_float_mantissa_for_uint {
    ($($uint: ty), *) => {
        $(
            impl FloatMantissa for $uint {
                #[inline]
                fn is_power_of_two(self) -> bool {
                    Self::is_power_of_two(self)
                }
            }
        )*
    };
}

impl_float_mantissa_for_uint!(u32, u64);

pub trait ConvertFloatParts {
    type Mantissa: FloatMantissa;
    type UnsignedExp;
    type SignedExp: PartialEq + PartialOrd;

    fn into_raw_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa);
    fn into_biased_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa);
    fn into_signed_biased_parts(self) -> (bool, Self::SignedExp, Self::Mantissa);
    fn into_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa);
    fn into_normalised_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa);

    fn from_raw_parts(sign: bool, exponent: Self::UnsignedExp, mantissa: Self::Mantissa) -> Self;
    fn from_biased_parts(sign: bool, exponent: Self::UnsignedExp, mantissa: Self::Mantissa)
    -> Self;
    fn from_signed_biased_parts(
        sign: bool,
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
    ) -> Self;
    fn from_signed_parts(sign: bool, exponent: Self::SignedExp, mantissa: Self::Mantissa) -> Self;
}

macro_rules! impl_convert_float_parts_for_primitive_float {
    ($float_type: ty, $unsigned_exponent_type: ty, $signed_exponent_type: ty, $mantissa_type: ty, $float_bit_width: literal) => {
        impl ConvertFloatParts for $float_type {
            type Mantissa = $mantissa_type;
            type UnsignedExp = $unsigned_exponent_type;
            type SignedExp = $signed_exponent_type;

            #[inline]
            fn into_raw_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
                let sign = self.is_sign_negative();
                const SIGN_MASK: $mantissa_type = <$mantissa_type>::MAX >> 1;
                let exp = (self.to_bits() & SIGN_MASK) >> (<$float_type>::MANTISSA_DIGITS - 1);
                let mant = self.to_bits()
                    & (<$mantissa_type>::MAX
                        >> (<$mantissa_type>::BITS - (<$float_type>::MANTISSA_DIGITS - 1)));

                (sign, exp as _, mant)
            }

            #[inline]
            fn into_biased_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
                let (sign, exp, mant) = self.into_raw_parts();
                if exp == 0 {
                    (sign, 1, mant)
                } else {
                    (
                        sign,
                        exp,
                        mant | (1 << (<$float_type>::MANTISSA_DIGITS - 1)),
                    )
                }
            }

            #[inline]
            fn into_signed_biased_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
                let (sign, exp, mant) = self.into_biased_parts();
                (sign, exp as Self::SignedExp, mant)
            }

            #[inline]
            fn into_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
                let (sign, exp, mant) = self.into_signed_biased_parts();
                const EXP_BIAS: $signed_exponent_type = <$float_type>::MAX_EXP - 1;
                (sign, exp - EXP_BIAS, mant)
            }

            #[inline]
            fn into_normalised_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
                let (sign, exp, mant) = self.into_signed_parts();
                let shift = Self::MANTISSA_DIGITS - Bits::bit_width(&mant);
                if mant == 0 || shift == 0 {
                    (sign, exp, mant)
                } else {
                    let normalised_mant = mant >> shift;
                    let normalised_exp = exp - (shift as Self::SignedExp);

                    (sign, normalised_exp, normalised_mant)
                }
            }

            #[inline]
            fn from_raw_parts(
                sign: bool,
                exponent: Self::UnsignedExp,
                mantissa: Self::Mantissa,
            ) -> Self {
                debug_assert!(Bits::bit_width(&mantissa) <= <$float_type>::MANTISSA_DIGITS - 1);

                let mut bits =
                    (exponent as $mantissa_type) << (<$float_type>::MANTISSA_DIGITS - 1) | mantissa;
                if sign {
                    bits |= 1 << ($float_bit_width - 1);
                }
                Self::from_bits(bits)
            }

            #[inline]
            fn from_biased_parts(
                sign: bool,
                mut exponent: Self::UnsignedExp,
                mut mantissa: Self::Mantissa,
            ) -> Self {
                debug_assert!(exponent != 0);

                if mantissa.bit(Self::MANTISSA_DIGITS - 1) {
                    mantissa ^= 1 << (Self::MANTISSA_DIGITS - 1);
                } else {
                    debug_assert!(exponent == 1);
                    exponent = 0;
                }
                Self::from_raw_parts(sign, exponent, mantissa)
            }

            #[inline]
            fn from_signed_biased_parts(
                sign: bool,
                exponent: Self::SignedExp,
                mantissa: Self::Mantissa,
            ) -> Self {
                debug_assert!(!exponent.is_negative());
                let exponent = exponent as Self::UnsignedExp;
                Self::from_biased_parts(sign, exponent, mantissa)
            }

            #[inline]
            fn from_signed_parts(
                sign: bool,
                exponent: Self::SignedExp,
                mantissa: Self::Mantissa,
            ) -> Self {
                const EXP_BIAS: <$float_type as ConvertFloatParts>::SignedExp =
                    <$float_type>::MAX_EXP - 1;
                let exponent = exponent + EXP_BIAS;
                Self::from_signed_biased_parts(sign, exponent, mantissa)
            }
        }
    };
}

impl_convert_float_parts_for_primitive_float!(f32, u32, i32, u32, 32);
impl_convert_float_parts_for_primitive_float!(f64, u32, i32, u64, 64);

pub trait FloatCastHelper: Neg<Output = Self> + ConvertFloatParts + PartialEq {
    const MANTISSA_DIGITS: Exponent;
    // const EXPONENT_BITS: Exponent = Self::BITS - Self::MANTISSA_DIGITS;
    const MAX_EXP: <Self as ConvertFloatParts>::SignedExp;
    const INFINITY: Self;
    const ZERO: Self;

    fn is_nan(&self) -> bool;
    fn is_infinite(&self) -> bool;
}

macro_rules! impl_float_cast_helper_for_primitive_float {
    ($float_type: ty, $mantissa_type: ty, $exponent_type: ty, $float_type_bit_width: literal) => {
        impl FloatCastHelper for $float_type {
            const MANTISSA_DIGITS: Exponent = Self::MANTISSA_DIGITS as Exponent;
            const MAX_EXP: <Self as ConvertFloatParts>::SignedExp = Self::MAX_EXP;
            const INFINITY: Self = Self::INFINITY;
            const ZERO: Self = 0.0;

            #[inline]
            fn is_nan(&self) -> bool {
                Self::is_nan(*self)
            }

            #[inline]
            fn is_infinite(&self) -> bool {
                Self::is_infinite(*self)
            }
        }
    };
}

impl_float_cast_helper_for_primitive_float!(f32, u32, i32, 32);
impl_float_cast_helper_for_primitive_float!(f64, u64, i32, 64);
