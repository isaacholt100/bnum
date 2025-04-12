use super::BUintD8;
use crate::Digit;

macro_rules! primitive_uint_try_from_into_uint {
    ($($uint: ty), *) => {
        $(
            impl<const N: usize> TryFrom<BUintD8<N>> for $uint {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: BUintD8<N>) -> Result<Self, Self::Error> {
                    if BUintD8::<N>::BITS <= Self::BITS || uint.leading_zeros_at_least_threshold(BUintD8::<N>::BITS - Self::BITS) {
                        Ok(Self::cast_from(uint))
                    } else {
                        Err(TryFromIntError(()))
                    }
                }
            }

            impl<const N: usize> TryFrom<$uint> for BUintD8<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: $uint) -> Result<Self, Self::Error> {
                    if <$uint>::BITS <= Self::BITS || uint.leading_zeros() >= <$uint>::BITS - Self::BITS {
                        Ok(Self::cast_from(uint))
                    } else {
                        Err(TryFromIntError(()))
                    }
                }
            }
        )*
    }
}

primitive_uint_try_from_into_uint!(u8, u16, u32, u64, u128, usize);

macro_rules! uint_try_from_primitive_int {
    ($($int: ty),*) => {
        $(
            impl<const N: usize> TryFrom<$int> for BUintD8<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    if int.is_negative() {
                        return Err(TryFromIntError(()));
                    }
                    if <$int>::BITS - 1 <= Self::BITS || int.leading_zeros() >= <$int>::BITS - Self::BITS {
                        Ok(Self::cast_from(int))
                    } else {
                        Err(TryFromIntError(()))
                    }
                }
            }
        )*
    }
}
uint_try_from_primitive_int!(i8, i16, i32, i64, i128, isize);

macro_rules! primitive_int_try_from_uint {
    ($($int: ty), *) => {
        $(
            impl<const N: usize> TryFrom<BUintD8<N>> for $int {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: BUintD8<N>) -> Result<Self, Self::Error> {
                    if BUintD8::<N>::BITS <= Self::BITS - 1 || uint.leading_zeros_at_least_threshold(BUintD8::<N>::BITS - Self::BITS + 1) {
                        Ok(Self::cast_from(uint))
                    } else {
                        Err(TryFromIntError(()))
                    }
                }
            }
        )*
    };
}

primitive_int_try_from_uint!(i8, i16, i32, i64, i128, isize);

impl<const N: usize, const M: usize> BTryFrom<BUintD8<M>> for BUintD8<N> {
    type Error = TryFromIntError;

    fn try_from(from: BUintD8<M>) -> Result<Self, Self::Error> {
        if BUintD8::<M>::BITS <= Self::BITS || BUintD8::<M>::BITS - from.leading_zeros() <= Self::BITS {
            Ok(Self::cast_from(from))
        } else {
            Err(TryFromIntError(()))
        }
    }
}

use crate::cast::CastFrom;
use crate::errors::TryFromIntError;

impl<const N: usize> From<bool> for BUintD8<N> {
    #[inline]
    fn from(small: bool) -> Self {
        Self::cast_from(small)
    }
}

// TODO: make this TryFrom instead
impl<const N: usize> From<char> for BUintD8<N> {
    #[inline]
    fn from(c: char) -> Self {
        Self::cast_from(c)
    }
}

use crate::BTryFrom;

impl<const N: usize> From<[Digit; N]> for BUintD8<N> {
    #[inline]
    fn from(digits: [Digit; N]) -> Self {
        Self::from_digits(digits)
    }
}

impl<const N: usize> From<BUintD8<N>> for [Digit; N] {
    #[inline]
    fn from(uint: BUintD8<N>) -> Self {
        uint.digits
    }
}

#[cfg(test)]
mod tests {
    use crate::test::cast_types::*;
    use crate::test::{self, types::*};
    use crate::BTryFrom;

    test::test_btryfrom!(utest; TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10/*, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize*/);

    test::test_from! {
        function: <utest as TryFrom>::try_from,
        from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char) // TODO: when we can use TryFrom for conversions between bnum ints, we can just add the list of test types here, same as in the casting tests
    }
    #[cfg(feature = "signed")]
    test::test_from! {
        function: <utest as TryFrom>::try_from,
        from_types: (TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
    }

    test::test_into! {
        function: <utest as TryInto>::try_into,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }
}
