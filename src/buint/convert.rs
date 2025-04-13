use super::BUintD8;
use crate::Digit;

macro_rules! primitive_uint_try_from_into_uint {
    ($($uint: ty), *) => {
        $(
            impl<const N: usize> TryFrom<BUintD8<N>> for $uint {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: BUintD8<N>) -> Result<Self, Self::Error> {
                    crate::int::convert::uint_try_from_uint(uint)
                }
            }

            impl<const N: usize> TryFrom<$uint> for BUintD8<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: $uint) -> Result<Self, Self::Error> {
                    crate::int::convert::uint_try_from_uint(uint)
                }
            }
        )*
    }
}

primitive_uint_try_from_into_uint!(u8, u16, u32, u64, u128, usize);

macro_rules! uint_try_from_to_primitive_int {
    ($($int: ty),*) => {
        $(
            impl<const N: usize> TryFrom<$int> for BUintD8<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    crate::int::convert::uint_try_from_int(int)
                }
            }

            impl<const N: usize> TryFrom<BUintD8<N>> for $int {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: BUintD8<N>) -> Result<Self, Self::Error> {
                    crate::int::convert::int_try_from_uint(uint)
                }
            }
        )*
    }
}
uint_try_from_to_primitive_int!(i8, i16, i32, i64, i128, isize);

impl<const N: usize, const M: usize> BTryFrom<BUintD8<M>> for BUintD8<N> {
    type Error = TryFromIntError;

    fn try_from(from: BUintD8<M>) -> Result<Self, Self::Error> {
        crate::int::convert::uint_try_from_uint(from)
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
