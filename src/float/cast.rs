use crate::cast::float::{FloatCastHelper, FloatMantissa};

use super::{Float, FloatExponent};
use crate::cast::CastFrom;
use crate::doc;
use crate::{BUintD8, BUintD16, BUintD32, BUint, BIntD8, BIntD16, BIntD32, BInt};
use crate::ExpType;

macro_rules! uint_as_float {
    ($($uint: ident $(<$N: ident>)?), *) => {
        $(
            impl<const W: usize, const MB: usize $(, const $N: usize)?> CastFrom<$uint $(<$N>)?> for Float<W, MB> {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: $uint $(<$N>)?) -> Self {
                    crate::cast::float::cast_float_from_uint(from)
                }
            }
        )*
    };
}

uint_as_float!(u8, u16, u32, u64, u128, usize, BUintD8<N>, BUintD16<N>, BUintD32<N>, BUint<N>);

macro_rules! int_as_float {
    ($($int: ty), *) => {
        $(
            impl<const W: usize, const MB: usize> CastFrom<$int> for Float<W, MB> {
                fn cast_from(from: $int) -> Self {
                    let pos_cast = Self::cast_from(from.unsigned_abs());
                    if from.is_negative() {
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

macro_rules! bint_as_float {
    ($(BIntD8: ident), *) => {
        $(
            impl<const W: usize, const MB: usize, const N: usize> CastFrom<BIntD8<N>> for Float<W, MB> {
                fn cast_from(from: BIntD8<N>) -> Self {
                    let pos_cast = Self::cast_from(from.unsigned_abs());
                    if from.is_negative() {
                        -pos_cast
                    } else {
                        pos_cast
                    }
                }
            }
        )*
    };
}

bint_as_float!(BIntD8, BIntD16, BIntD32, BInt);

        impl<const W: usize, const MB: usize, const N: usize> CastFrom<Float<W, MB>> for BIntD8<N> {
            crate::bint::cast::bint_cast_from_float!(Float<W, MB>, BUintD8<N>);
        }
    };
}

macro_rules! float_as_int {
    ($($int: ty; $uint: ty), *) => {
        $(
            impl<const W: usize, const MB: usize> CastFrom<Float<W, MB>> for $int {
                #[inline]
                fn cast_from(from: Float<W, MB>) -> Self {
                    if from.is_sign_negative() {
                        let u = <$uint>::cast_from(-from);
                        if u >= Self::MIN as $uint {
                            Self::MIN
                        } else {
                            -(u as $int)
                        }
                    } else {
                        let u = <$uint>::cast_from(from);
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
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(value: Float<W, MB>) -> Self {
                    crate::cast::float::cast_uint_from_float(value)
                    // uint_cast_from_float(from)
                }
            }
        )*
    };
}

float_as_uint!(BUintD8<N>, BUintD16<N>, BUintD32<N>, BUint<N>, u8, u16, u32, u64, u128, usize);

impl<const W: usize, const MB: usize> FloatCastHelper for Float<W, MB> {
    const BITS: ExpType = Self::BITS;
    const MANTISSA_DIGITS: ExpType = Self::MANTISSA_DIGITS as ExpType;
    const MAX_EXP: FloatExponent = Self::MAX_EXP;
    const MIN_SUBNORMAL_EXP: FloatExponent = Self::MIN_SUBNORMAL_EXP;
    const INFINITY: Self = Self::INFINITY;
    const ZERO: Self = Self::ZERO;
    const NEG_ZERO: Self = Self::NEG_ZERO;

    #[inline]
    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        Self::is_infinite(*self)
    }
}

fn cast_float_from_float<T, U>(f: T) -> U
where
    T: FloatCastHelper,
    U: FloatCastHelper,
    U::Mantissa: CastFrom<T::Mantissa>,
    U::SignedExp: CastFrom<T::SignedExp>,
    T::SignedExp: CastFrom<U::SignedExp>
{
    // deal with zero cases as this means mantissa must have leading one
    let (sign, mut exponent, mantissa) = f.into_normalised_signed_parts();
    if mantissa == T::Mantissa::ZERO {
        return if sign {
            U::NEG_ZERO
        } else {
            U::ZERO
        };
    }
    if exponent == T::MAX_EXP { // the float is either infinity or NaN
        let out_mantissa = if T::MANTISSA_DIGITS <= U::MANTISSA_DIGITS {
            U::Mantissa::cast_from(mantissa) << (U::MANTISSA_DIGITS - T::MANTISSA_DIGITS)
        } else {
            U::Mantissa::cast_from(mantissa >> (T::MANTISSA_DIGITS - U::MANTISSA_DIGITS))
        };
        return U::from_normalised_signed_parts(sign, U::MAX_EXP, out_mantissa);
    }
    let out_mantissa = if T::MANTISSA_DIGITS <= U::MANTISSA_DIGITS { // in this case, the mantissa can be converted exactly
        U::Mantissa::cast_from(mantissa) << (U::MANTISSA_DIGITS - T::MANTISSA_DIGITS)
    } else {
        let (e, m) = T::round_exponent_mantissa::<true>(exponent, mantissa, T::MANTISSA_DIGITS - U::MANTISSA_DIGITS);
        exponent = e;

        U::Mantissa::cast_from(m)
    };

    let out_exponent = if T::EXPONENT_BITS <= U::EXPONENT_BITS { // in this case, we will never have overflow or underflow
        U::SignedExp::cast_from(exponent)
    } else {
        if T::SignedExp::cast_from(U::MAX_EXP) <= exponent { // exponent is too large to fit into output exponent
            return if sign {
                -U::INFINITY
            } else {
                U::INFINITY
            };
        }
        if exponent < T::SignedExp::cast_from(U::MIN_SUBNORMAL_EXP) {
            return if sign {
                U::NEG_ZERO
            } else {
                U::ZERO
            };
        }
        U::SignedExp::cast_from(exponent)
    };
    U::from_normalised_signed_parts(sign, out_exponent, out_mantissa)
}

impl<const W1: usize, const MB1: usize, const W2: usize, const MB2: usize> CastFrom<Float<W2, MB2>> for Float<W1, MB1> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(from: Float<W2, MB2>) -> Self {
        cast_float_from_float(from)
    }
}

macro_rules! primitive_and_big_float_cast {
    ($($primitive_float_type: ty), *) => {
        $(
            impl<const W: usize, const MB: usize> CastFrom<$primitive_float_type> for Float<W, MB> {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: $primitive_float_type) -> Self {
                    cast_float_from_float(from)
                }
            }

            impl<const W: usize, const MB: usize> CastFrom<Float<W, MB>> for $primitive_float_type {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: Float<W, MB>) -> Self {
                    cast_float_from_float(from)
                }
            }
        )*
    };
}

primitive_and_big_float_cast!(f32, f64);

#[cfg(test)]
mod tests {
    use super::CastFrom;
    use crate::cast::CastTo;
    use crate::test::{test_from, test_into};
    use crate::test::types::{ftest, FTEST};
    use crate::test::cast_types::*;

    test_from! {
        function: <ftest as CastFrom>::cast_from,
        from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, UTESTD8, UTESTD16, UTESTD32, UTESTD64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, ITESTD8, ITESTD16, ITESTD32, ITESTD64, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
    }

    test_into! {
        function: <ftest as CastTo>::cast_to,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64)
    }

    #[test]
    fn test_cast_float() {
        use crate::cast::As;
        let f1 = FTEST::from_bits(3472883712u32.as_());
        let f2 = f32::from_bits(3472883712u32);
        dbg!(f2);
        let u1 = u32::cast_from(f1);
        let u2 = u32::cast_from(f2);
        println!("{:?}", u1);
        println!("{:?}", u2);
    }

    // crate::int::cast::test_cast_to_bigint!(ftest; UTESTD8, UTESTD16, UTESTD32, UTESTD64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, ITESTD8, ITESTD16, ITESTD32, ITESTD64, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8);
}
