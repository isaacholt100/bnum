macro_rules! from_uint {
    ($BUint: ident, $Digit: ident; $($uint: tt),*) => {
        $(impl_const! {
            impl<const N: usize> const From<$uint> for $BUint<N> {
                #[inline]
                fn from(int: $uint) -> Self {
                    const UINT_BITS: usize = $uint::BITS as usize;
                    let mut out = Self::ZERO;
                    let mut i = 0;
                    while i << crate::digit::$Digit::BIT_SHIFT < UINT_BITS {
                        let d = (int >> (i << crate::digit::$Digit::BIT_SHIFT)) as $Digit;
                        if d != 0 {
                            out.digits[i] = d;
                        }
                        i += 1;
                    }
                    out
                }
            }
        })*
    }
}

macro_rules! try_from_iint {
    ($BUint: ident; $($int: tt -> $uint: tt),*) => {
        $(impl_const! {
            impl<const N: usize> const TryFrom<$int> for $BUint<N> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(int: $int) -> Result<Self, Self::Error> {
                    if int.is_negative() {
                        return Err(TryFromIntError(()));
                    }
                    let bits = int as $uint;
                    Ok(Self::from(bits))
                }
            }
        })*
    }
}

macro_rules! try_from_buint {
    ($BUint: ident, $Digit: ident; $($int: ty), *) => {
        $(crate::nightly::impl_const! {
            impl<const N: usize> const TryFrom<$BUint<N>> for $int {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(u: $BUint<N>) -> Result<$int, Self::Error> {
                    let mut out = 0;
                    let mut i = 0;
                    if $Digit::BITS > <$int>::BITS {
                        let small = u.digits[i] as $int;
                        let trunc = small as $Digit;
                        if u.digits[i] != trunc {
                            return Err(TryFromIntError(()));
                        }
                        out = small;
                        i = 1;
                    } else {
                        loop {
                            let shift = i << crate::digit::$Digit::BIT_SHIFT;
                            if i >= N || shift >= <$int>::BITS as usize {
                                break;
                            }
                            out |= u.digits[i] as $int << shift;
                            i += 1;
                        }
                    }

                    #[allow(unused_comparisons)]
                    if out < 0 {
                        return Err(TryFromIntError(()));
                    }

                    while i < N {
                        if u.digits[i] != 0 {
                            return Err(TryFromIntError(()));
                        }
                        i += 1;
                    }
                    
                    Ok(out)
                }
            }
        })*
    };
}

macro_rules! uint_try_from_uint {
    ($Trait: ident; $To: ident; $($From: ident $(<$N: ident>)?), *) => {
        $(
            impl<$(const $N: usize,)? const M: usize> $Trait<$From $(<$N>)?> for $To<M> {
                type Error = TryFromIntError;
            
                fn try_from(from: $From $(<$N>)?) -> Result<Self, Self::Error> {
                    if $From $(::<$N>)?::BITS <= Self::BITS || $From $(::<$N>)?::BITS - from.leading_zeros() <= Self::BITS {
                        Ok(Self::cast_from(from))
                    } else {
                        Err(TryFromIntError(()))
                    }
                }
            }
        )*
    };
}

macro_rules! uint_try_from_int {
    ($Trait: ident; $To: ident; $($From: ident $(<$N: ident>)?), *) => {
        $(
            impl<$(const $N: usize,)? const M: usize> $Trait<$From $(<$N>)?> for $To<M> {
                type Error = TryFromIntError;
            
                fn try_from(from: $From $(<$N>)?) -> Result<Self, Self::Error> {
                    if from.is_negative() {
                        Err(TryFromIntError(()))
                    } else {
                        if $From $(::<$N>)?::BITS.saturating_sub(1) <= Self::BITS || $From $(::<$N>)?::BITS - from.leading_zeros() <= Self::BITS {
                            Ok(Self::cast_from(from))
                        } else {
                            Err(TryFromIntError(()))
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! int_try_from_uint {
    ($Trait: ident; $To: ident; $($From: ident $(<$N: ident>)?), *) => {
        $(
            impl<$(const $N: usize,)? const M: usize> $Trait<$From $(<$N>)?> for $To<M> {
                type Error = TryFromIntError;
            
                fn try_from(from: $From $(<$N>)?) -> Result<Self, Self::Error> {
                    if $From $(::<$N>)?::BITS <= Self::BITS - 1 || $From $(::<$N>)?::BITS - from.leading_zeros() <= Self::BITS - 1 { // Self::BITS - 1 as otherwise return value would be negative
                        Ok(Self::cast_from(from))
                    } else {
                        Err(TryFromIntError(()))
                    }
                }
            }
        )*
    };
}

macro_rules! int_try_from_int {
    ($Trait: ident; $To: ident; $($From: ident $(<$N: ident>)?), *) => {
        $(
            impl<$(const $N: usize,)? const M: usize> $Trait<$From $(<$N>)?> for $To<M> {
                type Error = TryFromIntError;
            
                fn try_from(from: $From $(<$N>)?) -> Result<Self, Self::Error> {
                    if $From $(::<$N>)?::BITS <= Self::BITS {
                        return Ok(Self::cast_from(from));
                    }
                    if from.is_negative() {
                        if $From $(::<$N>)?::BITS - from.leading_ones() <= Self::BITS - 1 {
                            Ok(Self::cast_from(from))
                        } else {
                            Err(TryFromIntError(()))
                        }
                    } else {
                        if $From $(::<$N>)?::BITS - from.leading_zeros() <= Self::BITS - 1 {
                            Ok(Self::cast_from(from))
                        } else {
                            Err(TryFromIntError(()))
                        }
                    }
                }
            }
        )*
    };
}

use crate::BTryFrom;

macro_rules! mixed_try_from {
    ($BUint: ident, $BInt: ident) => {
        uint_try_from_uint!(BTryFrom; $BUint; BUint<N>, BUintD32<N>, BUintD16<N>, BUintD8<N>/*, u8, u16, u32, u64, u128, usize*/);
        uint_try_from_int!(BTryFrom; $BUint; BInt<N>, BIntD32<N>, BIntD16<N>, BIntD8<N>/*, i8, i16, i32, i64, i128, isize*/);
        int_try_from_uint!(BTryFrom; $BInt; BUint<N>, BUintD32<N>, BUintD16<N>, BUintD8<N>/*, u8, u16, u32, u64, u128, usize*/);
        int_try_from_int!(BTryFrom; $BInt; BInt<N>, BIntD32<N>, BIntD16<N>, BIntD8<N>/*, i8, i16, i32, i64, i128, isize*/);
    };
}

use crate::cast::CastFrom;
use crate::errors::TryFromIntError;
use crate::nightly::impl_const;

macro_rules! convert {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        impl_const! {
            impl<const N: usize> const From<bool> for $BUint<N> {
                #[inline]
                fn from(small: bool) -> Self {
                    Self::cast_from(small)
                }
            }
        }

        impl_const! {
            impl<const N: usize> const From<char> for $BUint<N> {
                #[inline]
                fn from(c: char) -> Self {
                    Self::cast_from(c)
                }
            }
        }

        from_uint!($BUint, $Digit; u8, u16, u32, u64, u128, usize);

        try_from_iint!($BUint; i8 -> u8, i16 -> u16, i32 -> u32, isize -> usize, i64 -> u64, i128 -> u128);

        try_from_buint!($BUint, $Digit; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

        mixed_try_from!($BUint, $BInt);

        impl_const! {
            impl<const N: usize> const From<[$Digit; N]> for $BUint<N> {
                #[inline]
                fn from(digits: [$Digit; N]) -> Self {
                    Self::from_digits(digits)
                }
            }
        }

        impl_const! {
            impl<const N: usize> const From<$BUint<N>> for [$Digit; N] {
                #[inline]
                fn from(uint: $BUint<N>) -> Self {
                    uint.digits
                }
            }
        }

        #[cfg(test)]
        paste::paste! {
            mod [<$Digit _digit_tests>] {
                use crate::test::types::big_types::$Digit::*;
                use crate::test::{self, types::utest};
                use crate::test::cast_types::*;
                use super::BTryFrom;

                test::test_btryfrom!(utest; UTESTD8, UTESTD16, UTESTD32, UTESTD64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, ITESTD8, ITESTD16, ITESTD32, ITESTD64, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10/*, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize*/);

                test::test_from! {
                    function: <utest as TryFrom>::try_from,
                    from_types: (u8, u16, u32, u64, bool, char, i8, i16, i32, i64, isize, usize) // TODO: when we can use TryFrom for conversions between bnum ints, we can just add the list of test types here, same as in the casting tests
                }

                test::test_into! {
                    function: <utest as TryInto>::try_into,
                    into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
                }
            }
        }
    };
}

crate::macro_impl!(convert);
