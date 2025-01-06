use super::BIntD8;
use crate::{BUintD8, Digit};

macro_rules! from_int {
    ($($int: tt),*) => {
        $(
            impl<const N: usize> From<$int> for BIntD8<N> {
                #[inline]
                fn from(int: $int) -> Self {
                    let mut out = if int.is_negative() {
                        !Self::ZERO
                    } else {
                        Self::ZERO
                    };
                    let mut i = 0;
                    while i << crate::digit::BIT_SHIFT < $int::BITS as usize {
                        let d = (int >> (i << crate::digit::BIT_SHIFT)) as Digit;
                        out.bits.digits[i] = d;
                        i += 1;
                    }
                    out
                }
            }
        )*
    }
}

macro_rules! from_uint {
    ($($from: tt), *) => {
        $(
            impl<const N: usize> From<$from> for BIntD8<N> {
                #[inline]
                fn from(int: $from) -> Self {
                    let out = Self::from_bits(BUintD8::from(int));
                    out
                }
            }
        )*
    }
}

macro_rules! int_try_from_bint {
    { $($int: ty), * }  => {
        $(
            impl<const N: usize> TryFrom<BIntD8<N>> for $int {
                type Error = TryFromIntError;

                fn try_from(int: BIntD8<N>) -> Result<$int, Self::Error> {
                    let neg = int.is_negative();
                    let (mut out, padding) = if neg {
                        (-1, Digit::MAX)
                    } else {
                        (0, Digit::MIN)
                    };
                    let mut i = 0;
                    if Digit::BITS > <$int>::BITS {
                        let small = int.bits.digits[i] as $int;
                        let trunc = small as Digit;
                        if int.bits.digits[i] != trunc {
                            return Err(TryFromIntError(()));
                        }
                        out = small;
                        i = 1;
                    } else {
                        if neg {
                            loop {
                                let shift = i << digit::BIT_SHIFT;
                                if i >= N || shift >= <$int>::BITS as usize {
                                    break;
                                }
                                out &= !((!int.bits.digits[i]) as $int << shift);
                                i += 1;
                            }
                        } else {
                            loop {
                                let shift = i << digit::BIT_SHIFT;
                                if i >= N || shift >= <$int>::BITS as usize {
                                    break;
                                }
                                out |= int.bits.digits[i] as $int << shift;
                                i += 1;
                            }
                        }
                    }

                    while i < N {
                        if int.bits.digits[i] != padding {
                            return Err(TryFromIntError(()));
                        }
                        i += 1;
                    }

                    if out.is_negative() != neg {
                        return Err(TryFromIntError(()));
                    }

                    Ok(out)
                }
            }
        )*
    };
}

macro_rules! uint_try_from_bint {
    ($($uint: ty), *) => {
        $(
            impl<const N: usize> TryFrom<BIntD8<N>> for $uint {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(int: BIntD8<N>) -> Result<$uint, Self::Error> {
                    if int.is_negative() {
                        Err(TryFromIntError(()))
                    } else {
                        <$uint>::try_from(int.bits)
                    }
                }
            }
        )*
    };
}

use crate::cast::CastFrom;
use crate::digit;
use crate::errors::{ParseIntError, TryFromIntError};
use core::str::FromStr;

impl<const N: usize> FromStr for BIntD8<N> {
    type Err = ParseIntError;

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

from_int!(i8, i16, i32, i64, i128, isize);

from_uint!(u8, u16, u32, u64, u128, usize);

impl<const N: usize> From<bool> for BIntD8<N> {
    #[inline]
    fn from(small: bool) -> Self {
        Self::cast_from(small)
    }
}

int_try_from_bint!(i8, i16, i32, i64, i128, isize);
uint_try_from_bint!(u8, u16, u32, u64, u128, usize);

// impl_const! {
//     impl<const N: usize> const TryFrom<BUintD8<N>> for BIntD8<N> {
//         type Error = TryFromIntError;

//         #[inline]
//         fn try_from(u: BUintD8<N>) -> Result<Self, Self::Error> {
//             if u.leading_ones() != 0 {
//                 Err(TryFromIntError(()))
//             } else {
//                 Ok(Self::from_bits(u))
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::test;
    use crate::test::cast_types::*;
    use crate::test::types::*;
    use crate::BTryFrom;

    test::test_btryfrom!(itest; TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10);

    #[cfg(test_int_bits = "128")]
    test::test_from! {
        function: <itest as TryFrom>::try_from,
        from_types: (i8, i16, i32, i64, i128, u8, u16, u32, u64, bool, usize, isize)
    }

    #[cfg(test_int_bits = "64")]
    test::test_from! {
        function: <itest as TryFrom>::try_from,
        from_types: (i8, i16, i32, i64, u8, u16, u32, bool, isize)
    }

    test::test_into! {
        function: <itest as TryInto>::try_into,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }
}
