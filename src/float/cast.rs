use crate::cast::float::{FloatCastHelper, FloatMantissa, ConvertFloatParts};
use crate::helpers::{Zero, Bits};

use super::{Float, FloatExponent, UnsignedFloatExponent};
use crate::Exponent;
use crate::cast::CastFrom;
use crate::{Int, Integer, Uint};

impl<const N: usize, const B: usize> FloatMantissa for Uint<N, B> {
    #[inline]
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

macro_rules! uint_as_float {
    ($($uint: ident), *) => {
        $(
            impl<const W: usize, const MB: usize> CastFrom<$uint> for Float<W, MB> {
                #[inline]
                fn cast_from(value: $uint) -> Self {
                    crate::cast::float::cast_float_from_uint(value)
                }
            }
        )*
    };
}

uint_as_float!(u8, u16, u32, u64, u128, usize);

macro_rules! int_as_float {
    ($($int: ty), *) => {
        $(
            impl<const W: usize, const MB: usize> CastFrom<$int> for Float<W, MB> {
                fn cast_from(value: $int) -> Self {
                    let pos_cast = Self::cast_from(value.unsigned_abs());
                    if value.is_negative() {
                        -pos_cast
                    } else {
                        pos_cast
                    }
                }
            }
        )*
    };
}

int_as_float!(i8, i16, i32, i64, i128, isize);

impl<const W: usize, const MB: usize, const S: bool, const N: usize, const B: usize, const OM: u8>
    CastFrom<Integer<S, N, B, OM>> for Float<W, MB>
{
    fn cast_from(value: Integer<S, N, B, OM>) -> Self {
        if !S {
            return crate::cast::float::cast_float_from_uint(value.force::<false, B, _>());
        }
        let f = Self::cast_from(value.unsigned_abs_internal());
        if value.is_negative_internal() { -f } else { f }
    }
}

impl<const W: usize, const MB: usize, const N: usize> CastFrom<Float<W, MB>> for Int<N> {
    fn cast_from(value: Float<W, MB>) -> Self {
        crate::integer::cast::cast_int_from_float!(value)
    }
}

macro_rules! float_as_int {
    ($($int: ty; $uint: ty), *) => {
        $(
            impl<const W: usize, const MB: usize> CastFrom<Float<W, MB>> for $int {
                #[inline]
                fn cast_from(value: Float<W, MB>) -> Self {
                    if value.is_sign_negative() {
                        let u = <$uint>::cast_from(-value);
                        if u >= Self::MIN as $uint {
                            Self::MIN
                        } else {
                            -(u as $int)
                        }
                    } else {
                        let u = <$uint>::cast_from(value);
                        let i = u as $int;
                        if i.is_negative() {
                            Self::MAX
                        } else {
                            i
                        }
                    }
                }
            }
        )*
    };
}

float_as_int!(i8; u8, i16; u16, i32; u32, i64; u64, i128; u128, isize; usize);

macro_rules! float_as_uint {
    ($($uint: ident $(<$N: ident>)?), *) => {
        $(
            impl<const W: usize, const MB: usize $(, const $N: usize)?> CastFrom<Float<W, MB>> for $uint $(<$N>)? {
                #[inline]
                fn cast_from(value: Float<W, MB>) -> Self {
                    crate::cast::float::cast_uint_from_float(value)
                }
            }
        )*
    };
}

float_as_uint!(Uint<N>, u8, u16, u32, u64, u128, usize);

impl<const W: usize, const MB: usize> ConvertFloatParts for Float<W, MB> {
    type Mantissa = Uint<W>;
    type SignedExp = FloatExponent;
    type UnsignedExp = UnsignedFloatExponent;

    #[inline]
    fn into_raw_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
        Self::into_raw_parts(self)
    }

    #[inline]
    fn into_biased_parts(self) -> (bool, Self::UnsignedExp, Self::Mantissa) {
        Self::into_biased_parts(self)
    }

    #[inline]
    fn into_signed_biased_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_signed_biased_parts(self)
    }

    #[inline]
    fn into_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_signed_parts(self)
    }

    #[inline]
    fn into_normalised_signed_parts(self) -> (bool, Self::SignedExp, Self::Mantissa) {
        Self::into_normalised_signed_parts(self)
    }

    #[inline]
    fn from_raw_parts(sign: bool, exponent: Self::UnsignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_raw_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_biased_parts(
        sign: bool,
        exponent: Self::UnsignedExp,
        mantissa: Self::Mantissa,
    ) -> Self {
        Self::from_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_signed_biased_parts(
        sign: bool,
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
    ) -> Self {
        Self::from_signed_biased_parts(sign, exponent, mantissa)
    }

    #[inline]
    fn from_signed_parts(sign: bool, exponent: Self::SignedExp, mantissa: Self::Mantissa) -> Self {
        Self::from_signed_parts(sign, exponent, mantissa)
    }
}

impl<const W: usize, const MB: usize> FloatCastHelper for Float<W, MB> {
    const MANTISSA_DIGITS: Exponent = Self::MANTISSA_DIGITS as Exponent;
    const MAX_EXP: FloatExponent = Self::MAX_EXP;
    const INFINITY: Self = Self::INFINITY;
    const ZERO: Self = Self::ZERO;

    #[inline]
    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        Self::is_infinite(*self)
    }
}

trait FloatCastFromFloatHelper: FloatCastHelper {
    const NEG_ZERO: Self;
    const MIN_SUBNORMAL_EXP: <Self as ConvertFloatParts>::SignedExp;
    const EXPONENT_BITS: Exponent;

    fn round_exponent_mantissa<const TIES_EVEN: bool>(
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
        shift: Exponent,
    ) -> (Self::SignedExp, Self::Mantissa);
    fn from_normalised_signed_parts(
        sign: bool,
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
    ) -> Self;
}

impl<const W: usize, const MB: usize> FloatCastFromFloatHelper for Float<W, MB> {
    const NEG_ZERO: Self = Self::NEG_ZERO;
    const MIN_SUBNORMAL_EXP: FloatExponent = Self::MIN_SUBNORMAL_EXP;
    const EXPONENT_BITS: Exponent = Self::EXPONENT_BITS;

    #[inline]
    fn round_exponent_mantissa<const TIES_EVEN: bool>(
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
        shift: Exponent,
    ) -> (Self::SignedExp, Self::Mantissa) {
        Self::round_exponent_mantissa::<TIES_EVEN>(exponent, mantissa, shift)
    }

    #[inline]
    fn from_normalised_signed_parts(
        sign: bool,
        exponent: Self::SignedExp,
        mantissa: Self::Mantissa,
    ) -> Self {
        Self::from_normalised_signed_parts(sign, exponent, mantissa)
    }
}

macro_rules! impl_float_cast_from_float_helper_for_primitive_float {
    ($float_type: ty, $mantissa_type: ty, $float_bit_width: expr) => {
        // $(
            impl FloatCastFromFloatHelper for $float_type {
                const NEG_ZERO: Self = -0.0;
                const MIN_SUBNORMAL_EXP: <Self as ConvertFloatParts>::SignedExp = Self::MIN_EXP + 1 - Self::MANTISSA_DIGITS as <Self as ConvertFloatParts>::SignedExp;
                const EXPONENT_BITS: Exponent = $float_bit_width - Self::MANTISSA_DIGITS as Exponent;

                #[inline]
                fn round_exponent_mantissa<const TIES_EVEN: bool>(
                    mut exponent: Self::SignedExp,
                    mantissa: Self::Mantissa,
                    shift: Exponent,
                ) -> (Self::SignedExp, Self::Mantissa) {
                    let mut shifted_mantissa = mantissa >> shift;
                    if !TIES_EVEN {
                        return (exponent, shifted_mantissa); // if not TIES_EVEN, then we truncate
                    }
                    let discarded_shifted_bits =
                        mantissa & (<$mantissa_type>::MAX >> ($float_bit_width - shift));
                    if Bits::bit(&discarded_shifted_bits, shift - 1) {
                        // in this case, the discarded portion is at least a half
                        if shifted_mantissa % 2 == 1 || !discarded_shifted_bits.is_power_of_two() {
                            // in this case, ties to even says we round up. checking if not a power of two tells us that there is at least one bit set to 1 (after the most significant bit set to 1). we check in this order as is_odd is O(1) whereas is_power_of_two is O(N)
                            shifted_mantissa = shifted_mantissa + 1;
                            if Bits::bit(&shifted_mantissa, shift) {
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
                    debug_assert!(mantissa == 0 || Bits::bit_width(&mantissa) == Self::MANTISSA_DIGITS);
                    if exponent < Self::MIN_EXP - 1 {
                        let shift = (Self::MIN_EXP - 1 - exponent) as Exponent;
                        let (out_exponent, out_mantissa) =
                            Self::round_exponent_mantissa::<true>(Self::MIN_EXP - 1, mantissa, shift);

                        Self::from_signed_parts(sign, out_exponent, out_mantissa)
                    } else {
                        Self::from_signed_parts(sign, exponent, mantissa)
                    }
                }
            }
        // )*
    };
}

impl_float_cast_from_float_helper_for_primitive_float!(f32, u32, 32);
impl_float_cast_from_float_helper_for_primitive_float!(f64, u64, 64);

fn cast_float_from_float<T, U>(f: T) -> U
where
    T: FloatCastFromFloatHelper,
    U: FloatCastFromFloatHelper,
    U::Mantissa: CastFrom<T::Mantissa>,
    U::SignedExp: CastFrom<T::SignedExp>,
    T::SignedExp: CastFrom<U::SignedExp>,
{
    // deal with zero cases as this means mantissa must have leading one
    let (sign, mut exponent, mantissa) = f.into_normalised_signed_parts();
    if mantissa == T::Mantissa::ZERO {
        return if sign { U::NEG_ZERO } else { U::ZERO };
    }
    if exponent == T::MAX_EXP {
        // the float is either infinity or NaN
        let out_mantissa = if T::MANTISSA_DIGITS <= U::MANTISSA_DIGITS {
            U::Mantissa::cast_from(mantissa) << (U::MANTISSA_DIGITS - T::MANTISSA_DIGITS)
        } else {
            U::Mantissa::cast_from(mantissa >> (T::MANTISSA_DIGITS - U::MANTISSA_DIGITS))
        };
        return U::from_normalised_signed_parts(sign, U::MAX_EXP, out_mantissa);
    }
    let out_mantissa = if T::MANTISSA_DIGITS <= U::MANTISSA_DIGITS {
        // in this case, the mantissa can be converted exactly
        U::Mantissa::cast_from(mantissa) << (U::MANTISSA_DIGITS - T::MANTISSA_DIGITS)
    } else {
        let (e, m) = T::round_exponent_mantissa::<true>(
            exponent,
            mantissa,
            T::MANTISSA_DIGITS - U::MANTISSA_DIGITS,
        );
        exponent = e;

        U::Mantissa::cast_from(m)
    };

    let out_exponent = if T::EXPONENT_BITS <= U::EXPONENT_BITS {
        // in this case, we will never have overflow
        U::SignedExp::cast_from(exponent)
    } else {
        if T::SignedExp::cast_from(U::MAX_EXP) <= exponent {
            // exponent is too large to fit into output exponent
            return if sign { -U::INFINITY } else { U::INFINITY };
        }
        if exponent < T::SignedExp::cast_from(U::MIN_SUBNORMAL_EXP) {
            return if sign { U::NEG_ZERO } else { U::ZERO };
        }
        U::SignedExp::cast_from(exponent)
    };
    U::from_normalised_signed_parts(sign, out_exponent, out_mantissa)
}

impl<const W1: usize, const MB1: usize, const W2: usize, const MB2: usize> CastFrom<Float<W2, MB2>>
    for Float<W1, MB1>
{
    #[inline]
    fn cast_from(value: Float<W2, MB2>) -> Self {
        cast_float_from_float(value)
    }
}

macro_rules! primitive_and_big_float_cast {
    ($($primitive_float_type: ty), *) => {
        $(
            impl<const W: usize, const MB: usize> CastFrom<$primitive_float_type> for Float<W, MB> {
                #[inline]
                fn cast_from(value: $primitive_float_type) -> Self {
                    cast_float_from_float(value)
                }
            }

            impl<const W: usize, const MB: usize> CastFrom<Float<W, MB>> for $primitive_float_type {
                #[inline]
                fn cast_from(value: Float<W, MB>) -> Self {
                    cast_float_from_float(value)
                }
            }
        )*
    };
}

primitive_and_big_float_cast!(f32, f64);

#[cfg(test)]
mod tests {
    use crate::cast::{CastFrom, CastTo};
    use crate::test::cast_types::*;
    use crate::test::{test_from, test_into};

    crate::test::test_all! {
        testing floats;

        test_from! {
            function: <ftest as CastFrom>::cast_from,
            from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, UTEST, ITEST, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
        }

        test_into! {
            function: <ftest as CastTo>::cast_to,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64)
        }

        // crate::ints::cast::test_cast_to_bigint!(ftest; UTESTD8, UTESTD16, UTESTD32, UTESTD64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, ITESTD8, ITESTD16, ITESTD32, ITESTD64, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8);
    }
}
