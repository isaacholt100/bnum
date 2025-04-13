use super::BIntD8;
use crate::BUintD8;

macro_rules! int_try_from_to_primitive_int {
    ($($int: ty),*) => {
        $(
            impl<const N: usize> TryFrom<$int> for BIntD8<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(from: $int) -> Result<Self, Self::Error> {
                    crate::int::convert::int_try_from_int(from)
                }
            }

            impl<const N: usize> TryFrom<BIntD8<N>> for $int {
                type Error = TryFromIntError;

                fn try_from(from: BIntD8<N>) -> Result<Self, Self::Error> {
                    crate::int::convert::int_try_from_int(from)
                }
            }
        )*
    }
}

int_try_from_to_primitive_int!(i8, i16, i32, i64, i128, isize);

macro_rules! int_try_from_to_primitive_uint {
    ($($uint: ty), *) => {
        $(
            impl<const N: usize> TryFrom<$uint> for BIntD8<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: $uint) -> Result<Self, Self::Error> {
                    crate::int::convert::int_try_from_uint(uint)
                }
            }

            impl<const N: usize> TryFrom<BIntD8<N>> for $uint {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(int: BIntD8<N>) -> Result<$uint, Self::Error> {
                    crate::int::convert::uint_try_from_int(int)
                }
            }
        )*
    }
}

int_try_from_to_primitive_uint!(u8, u16, u32, u64, u128, usize);

use crate::cast::CastFrom;
use crate::errors::{ParseIntError, TryFromIntError};
use core::str::FromStr;

impl<const N: usize> FromStr for BIntD8<N> {
    type Err = ParseIntError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

impl<const N: usize> From<bool> for BIntD8<N> {
    #[inline]
    fn from(small: bool) -> Self {
        Self::cast_from(small)
    }
}

impl<const N: usize, const M: usize> TryFrom<BIntD8<N>> for BUintD8<M> {
    type Error = TryFromIntError;

    fn try_from(from: BIntD8<N>) -> Result<Self, Self::Error> {
        crate::int::convert::uint_try_from_int(from)
    }
}

impl<const N: usize, const M: usize> TryFrom<BUintD8<N>> for BIntD8<M> {
    type Error = TryFromIntError;

    fn try_from(from: BUintD8<N>) -> Result<Self, Self::Error> {
        crate::int::convert::int_try_from_uint(from)
    }
}

impl<const M: usize, const N: usize> crate::BTryFrom<BIntD8<M>> for BIntD8<N> {
    type Error = TryFromIntError;

    fn try_from(from: BIntD8<M>) -> Result<Self, Self::Error> {
        crate::int::convert::int_try_from_int(from)
    }
}

#[cfg(test)]
mod tests {
    use crate::test;
    use crate::test::cast_types::*;
    use crate::test::types::*;
    use crate::BTryFrom;

    test::test_btryfrom!(itest; TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10);

    test::test_from! {
        function: <itest as TryFrom>::try_from,
        from_types: (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10)
    }

    test::test_into! {
        function: <itest as TryInto>::try_into,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }
}
