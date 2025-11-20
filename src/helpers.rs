use crate::Exponent;
use crate::Uint;

pub trait Bits {
    const BITS: Exponent;

    fn bits(&self) -> Exponent;
    fn bit(&self, index: Exponent) -> bool;
}

macro_rules! impl_bits_for_uint {
    ($($uint: ty), *) => {
        $(impl Bits for $uint {
            const BITS: Exponent = Self::BITS as Exponent;

            #[inline]
            fn bits(&self) -> Exponent {
                (Self::BITS - self.leading_zeros()) as Exponent
            }

            #[inline]
            fn bit(&self, index: Exponent) -> bool {
                self & (1 << index) != 0
            }
        })*
    };
}

impl_bits_for_uint!(u8, u16, u32, u64, u128, usize);

impl<const N: usize, const OM: u8> Bits for Uint<N, OM> {
    const BITS: Exponent = Self::BITS;

    #[inline]
    fn bits(&self) -> Exponent {
        Self::bits(&self)
    }

    #[inline]
    fn bit(&self, index: Exponent) -> bool {
        Self::bit(&self, index)
    }
}

pub trait Zero: Sized + PartialEq {
    const ZERO: Self;

    #[inline]
    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

pub trait One: Sized + PartialEq {
    const ONE: Self;
}

macro_rules! impl_zero_for_int {
    ($($int: ty), *) => {
        $(impl Zero for $int {
            const ZERO: Self = 0;
        })*
    };
}

impl_zero_for_int!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_one_for_int {
    ($($uint: ty), *) => {
        $(impl One for $uint {
            const ONE: Self = 1;
        })*
    };
}

impl_one_for_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

impl<const N: usize, const OM: u8> Zero for Uint<N, OM> {
    const ZERO: Self = Self::ZERO;
}

impl<const N: usize, const OM: u8> One for Uint<N, OM> {
    const ONE: Self = Self::ONE;
}

#[inline(always)]
pub const fn tuple_to_option<T: Copy>((int, overflow): (T, bool)) -> Option<T> {
    if overflow { None } else { Some(int) }
}

macro_rules! ok {
    { $e: expr } => {
        match $e {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };
}

pub(crate) use ok;