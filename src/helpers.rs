use crate::ExpType;

pub trait Bits {
    const BITS: ExpType;

    fn bits(&self) -> ExpType;
    fn bit(&self, index: ExpType) -> bool;
}

macro_rules! impl_bits_for_uint {
    ($($uint: ty), *) => {
        $(impl Bits for $uint {
            const BITS: ExpType = Self::BITS as ExpType;

            #[inline]
            fn bits(&self) -> ExpType {
                (Self::BITS - self.leading_zeros()) as ExpType
            }

            #[inline]
            fn bit(&self, index: ExpType) -> bool {
                self & (1 << index) != 0
            }
        })*
    };
}

impl_bits_for_uint!(u8, u16, u32, u64, u128, usize);

impl<const N: usize> crate::helpers::Bits for crate::BUintD8<N> {
    const BITS: ExpType = Self::BITS;

    #[inline]
    fn bits(&self) -> ExpType {
        Self::bits(&self)
    }

    #[inline]
    fn bit(&self, index: ExpType) -> bool {
        Self::bit(&self, index)
    }
}

pub trait Zero: Sized + PartialEq {
    const ZERO: Self;

    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

pub trait One: Sized + PartialEq {
    const ONE: Self;

    fn is_one(&self) -> bool {
        self == &Self::ONE
    }
}

macro_rules! impl_zero_for_uint {
    ($($uint: ty), *) => {
        $(impl Zero for $uint {
            const ZERO: Self = 0;
        })*
    };
}

impl_zero_for_uint!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_one_for_int {
    ($($uint: ty), *) => {
        $(impl One for $uint {
            const ONE: Self = 1;
        })*
    };
}

impl_one_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const N: usize> crate::helpers::Zero for crate::BUintD8<N> {
    const ZERO: Self = Self::ZERO;
}

impl<const N: usize> crate::helpers::One for crate::BUintD8<N> {
    const ONE: Self = Self::ONE;
}

#[inline]
pub const fn tuple_to_option<T: Copy>((int, overflow): (T, bool)) -> Option<T> {
    if overflow {
        None
    } else {
        Some(int)
    }
}
