use super::Uint;

macro_rules! uint_try_from_into_primitive_uint {
    ($($uint: ty), *) => {
        $(
            impl<const N: usize> TryFrom<Uint<N>> for $uint {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: Uint<N>) -> Result<Self, Self::Error> {
                    crate::ints::convert::uint_try_from_uint(uint)
                }
            }

            impl<const N: usize> TryFrom<$uint> for Uint<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: $uint) -> Result<Self, Self::Error> {
                    crate::ints::convert::uint_try_from_uint(uint)
                }
            }
        )*
    }
}

uint_try_from_into_primitive_uint!(u8, u16, u32, u64, u128, usize);

macro_rules! uint_try_from_into_primitive_int {
    ($($int: ty),*) => {
        $(
            impl<const N: usize> TryFrom<$int> for Uint<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    crate::ints::convert::uint_try_from_int(int)
                }
            }

            impl<const N: usize> TryFrom<Uint<N>> for $int {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(uint: Uint<N>) -> Result<Self, Self::Error> {
                    crate::ints::convert::int_try_from_uint(uint)
                }
            }
        )*
    }
}
uint_try_from_into_primitive_int!(i8, i16, i32, i64, i128, isize);

impl<const N: usize, const M: usize> BTryFrom<Uint<M>> for Uint<N> {
    type Error = TryFromIntError;

    fn try_from(from: Uint<M>) -> Result<Self, Self::Error> {
        crate::ints::convert::uint_try_from_uint(from)
    }
}

use crate::cast::CastFrom;
use crate::errors::{TryFromCharError, TryFromIntError};

impl<const N: usize> From<bool> for Uint<N> {
    #[inline]
    fn from(small: bool) -> Self {
        Self::cast_from(small)
    }
}

impl<const N: usize> TryFrom<char> for Uint<N> {
    type Error = TryFromCharError;
    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        <Self as TryFrom<u32>>::try_from(u32::from(c)).map_err(|_| TryFromCharError(()))
    }
}

use crate::BTryFrom;

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::BTryFrom;
    use crate::test::cast_types::*;
    use crate::test;

    test::test_btryfrom!(utest; TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10);

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
    #[cfg(feature = "signed")]
    test::test_into! {
        function: <utest as TryInto>::try_into,
        into_types: (TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
    }

    test::test_bignum! {
        function: <utest as TryInto<u8>>::try_into,
        cases: [(128u8)]
    }
}
