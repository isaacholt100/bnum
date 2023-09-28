// TODO: implement casts from and to float for primitive types and buint, bint
use super::Float;
use crate::cast::CastFrom;
use crate::doc;
use crate::{BUintD8, BUintD16, BUintD32, BUint, BIntD8, BIntD16, BIntD32, BInt};
use crate::ExpType;
use crate::buint::as_float::{CastToFloatConsts, cast_float_from_uint};
use crate::buint::as_float;

macro_rules! uint_as_float {
    ($($uint: ident $(<$N: ident>)?), *) => {
        $(
            impl<const W: usize, const MB: usize $(, const $N: usize)?> CastFrom<$uint $(<$N>)?> for Float<W, MB> {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: $uint $(<$N>)?) -> Self {
                    cast_float_from_uint(from)
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
    ($($bint: ident), *) => {
        $(
            impl<const W: usize, const MB: usize, const N: usize> CastFrom<$bint<N>> for Float<W, MB> {
                fn cast_from(from: $bint<N>) -> Self {
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

macro_rules! float_as_bint {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        impl<const W: usize, const MB: usize, const N: usize> CastFrom<Float<W, MB>> for $BInt<N> {
            crate::bint::cast::bint_cast_from_float!(Float<W, MB>, $BUint<N>);
        }
    };
}

crate::macro_impl!(float_as_bint);

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

macro_rules! impl_mantissa_for_buint {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        impl<const N: usize> as_float::Mantissa for $BUint<N> {
            const ONE: Self = Self::ONE;
            const TWO: Self = Self::TWO;
            const MAX: Self = Self::MAX;
            const BITS: ExpType = Self::BITS;

            #[inline]
            fn bit(&self, n: ExpType) -> bool {
                Self::bit(&self, n)
            }

            #[inline]
            fn shl(self, n: ExpType) -> Self {
                Self::shl(self, n)
            }

            #[inline]
            fn shr(self, n: ExpType) -> Self {
                Self::shr(self, n)
            }

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self::add(self, rhs)
            }

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self::sub(self, rhs)
            }

            #[inline]
            fn leading_zeros(self) -> ExpType {
                Self::leading_zeros(self)
            }

            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self::bitand(self, rhs)
            }

            #[inline]
            fn gt(&self, rhs: &Self) -> bool {
                Self::gt(&self, &rhs)
            }
        }
    };
}

pub(crate) use impl_mantissa_for_buint;

crate::macro_impl!(impl_mantissa_for_buint);

use crate::buint::float_as::{uint_cast_from_float, CastUintFromFloatHelper};

impl<const W: usize, const MB: usize> CastUintFromFloatHelper for Float<W, MB> {
    type M = BUintD8<W>;
    type E = BIntD8<W>;

    #[inline]
    fn is_nan(&self) -> bool {
        Self::is_nan(*self)
    }

    #[inline]
    fn is_sign_negative(&self) -> bool {
        Self::is_sign_negative(*self)
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        Self::is_infinite(*self)
    }

    #[inline]
    fn decode(self) -> (Self::M, Self::E) {
        let (_, exp, mant) = self.to_parts_biased();
        let exp = BIntD8::from_bits(exp) - Self::EXP_BIAS - BIntD8::cast_from(Self::MB);
        (mant, exp)
    }
}

impl<const W: usize, const MB: usize> CastToFloatConsts for Float<W, MB> {
    type M = BUintD8<W>;

    const ZERO: Self = Self::ZERO;
    const MANTISSA_DIGITS: ExpType = Self::MANTISSA_DIGITS as ExpType;
    const MAX_EXP: Self::M = Self::MAX_EXP.to_bits();
    const INFINITY: Self = Self::INFINITY;

    fn from_raw_parts(exp: Self::M, mant: Self::M) -> Self {
        Self::from_raw_parts(false, exp, mant & Self::MANTISSA_MASK)
    }
}

macro_rules! float_as_uint {
    ($($uint: ident $(<$N: ident>)?), *) => {
        $(
            impl<const W: usize, const MB: usize $(, const $N: usize)?> CastFrom<Float<W, MB>> for $uint $(<$N>)? {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: Float<W, MB>) -> Self {
                    uint_cast_from_float(from)
                }
            }
        )*
    };
}

float_as_uint!(BUintD8<N>, BUintD16<N>, BUintD32<N>, BUint<N>, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::CastFrom;
    use crate::cast::CastTo;
    use crate::test::{test_from, test_into};
    use crate::test::types::{ftest, FTEST};
    use crate::test::cast_types::*;

    #[test]
    fn test_cast() {
        let a = 1234034598347589374u128;
        let b = FTEST::cast_from(a);
        let c = ftest::cast_from(a);
        assert_eq!(b, c.into());
    }

    test_from! {
        function: <ftest as CastFrom>::cast_from,
        from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, UTESTD8, UTESTD16, UTESTD32, UTESTD64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, ITESTD8, ITESTD16, ITESTD32, ITESTD64, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8)
    }

    test_into! {
        function: <ftest as CastTo>::cast_to,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }

    crate::int::cast::test_cast_to_bigint!(ftest; UTESTD8, UTESTD16, UTESTD32, UTESTD64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, ITESTD8, ITESTD16, ITESTD32, ITESTD64, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8);
}
