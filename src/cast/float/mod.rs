use crate::helpers::Bits;
use crate::ExpType;
use core::ops::{Add, BitAnd, Neg, Shl, Shr};

mod float_from_uint;
mod uint_from_float;
pub use float_from_uint::*;
pub use uint_from_float::*;

pub trait FloatMantissa:
    Sized
    + Shl<ExpType, Output = Self>
    + Shr<ExpType, Output = Self>
    + Add<Self, Output = Self>
    + BitAnd<Self, Output = Self>
    + PartialEq
    + Bits
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;

    fn leading_zeros(self) -> ExpType;
    fn checked_shr(self, n: ExpType) -> Option<Self>;
    fn is_power_of_two(self) -> bool;
}

macro_rules! impl_float_mantissa_for_uint {
    ($($uint: ty), *) => {
        $(
            impl FloatMantissa for $uint {
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const TWO: Self = 2;
                const MAX: Self = Self::MAX;

                #[inline]
                fn leading_zeros(self) -> ExpType {
                    Self::leading_zeros(self) as ExpType
                }

                #[inline]
                fn checked_shr(self, n: ExpType) -> Option<Self> {
                    Self::checked_shr(self, n as u32)
                }

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
    fn round_exponent_mantissa<const TIES_EVEN: bool>(
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
        shift: ExpType,
    ) -> (Self::SignedExp, Self::Mantissa);
    fn from_normalised_signed_parts(
        sign: bool,
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
    ) -> Self;
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
                use crate::helpers::Bits;
                let shift = Self::MANTISSA_DIGITS - mant.bits();
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
                debug_assert!(mantissa.bits() <= <$float_type>::MANTISSA_DIGITS - 1);

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

            #[inline]
            fn round_exponent_mantissa<const TIES_EVEN: bool>(
                mut exponent: Self::SignedExp,
                mantissa: Self::Mantissa,
                shift: ExpType,
            ) -> (Self::SignedExp, Self::Mantissa) {
                let mut shifted_mantissa = mantissa >> shift;
                if !TIES_EVEN {
                    return (exponent, shifted_mantissa); // if not TIES_EVEN, then we truncate
                }
                let discarded_shifted_bits =
                    mantissa & (<$mantissa_type>::MAX >> ($float_bit_width - shift));
                if discarded_shifted_bits.bit(shift - 1) {
                    // in this case, the discarded portion is at least a half
                    if shifted_mantissa & 1 == 1 || !discarded_shifted_bits.is_power_of_two() {
                        // in this case, ties to even says we round up. checking if not a power of two tells us that there is at least one bit set to 1 (after the most significant bit set to 1). we check in this order as is_odd is O(1) whereas is_power_of_two is O(N)
                        shifted_mantissa = shifted_mantissa + 1;
                        if shifted_mantissa.bit(shift) {
                            // check for overflow (with respect to the mantissa bit width)
                            exponent += 1;
                            shifted_mantissa = shifted_mantissa >> 1;
                        }
                    }
                }
                (exponent, shifted_mantissa)
            }

            #[inline]
            fn from_normalised_signed_parts(
                sign: bool,
                exponent: Self::SignedExp,
                mantissa: Self::Mantissa,
            ) -> Self {
                use crate::helpers::Bits;

                debug_assert!(mantissa == 0 || mantissa.bits() == Self::MANTISSA_DIGITS);
                if exponent < Self::MIN_EXP - 1 {
                    let shift = (Self::MIN_EXP - 1 - exponent) as ExpType;
                    let (out_exponent, out_mantissa) =
                        Self::round_exponent_mantissa::<true>(Self::MIN_EXP - 1, mantissa, shift);

                    Self::from_signed_parts(sign, out_exponent, out_mantissa)
                } else {
                    Self::from_signed_parts(sign, exponent, mantissa)
                }
            }
        }
    };
}

impl_convert_float_parts_for_primitive_float!(f32, u32, i32, u32, 32);
impl_convert_float_parts_for_primitive_float!(f64, u32, i32, u64, 64);

pub trait FloatCastHelper: Neg<Output = Self> + ConvertFloatParts + PartialEq {
    const BITS: ExpType;
    const MANTISSA_DIGITS: ExpType;
    const EXPONENT_BITS: ExpType = Self::BITS - Self::MANTISSA_DIGITS;
    const MAX_EXP: <Self as ConvertFloatParts>::SignedExp;
    const MIN_SUBNORMAL_EXP: <Self as ConvertFloatParts>::SignedExp;
    const INFINITY: Self;
    const ZERO: Self;
    const NEG_ZERO: Self;

    fn is_nan(&self) -> bool;
    fn is_infinite(&self) -> bool;
}

macro_rules! impl_cast_float_from_float_helper_for_primitive_float {
    ($float_type: ty, $mantissa_type: ty, $exponent_type: ty, $float_type_bit_width: literal) => {
        impl FloatCastHelper for $float_type {
            const BITS: ExpType = $float_type_bit_width;
            const MANTISSA_DIGITS: ExpType = Self::MANTISSA_DIGITS as ExpType;
            const MAX_EXP: <Self as ConvertFloatParts>::SignedExp = Self::MAX_EXP;
            const MIN_SUBNORMAL_EXP: <Self as ConvertFloatParts>::SignedExp =
                Self::MIN_EXP + 1 - Self::MANTISSA_DIGITS as <Self as ConvertFloatParts>::SignedExp;
            const INFINITY: Self = Self::INFINITY;
            const ZERO: Self = 0.0;
            const NEG_ZERO: Self = -0.0;

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

impl_cast_float_from_float_helper_for_primitive_float!(f32, u32, i32, 32);
impl_cast_float_from_float_helper_for_primitive_float!(f64, u64, i32, 64);
