mod bigint_helpers;
mod bits;
mod bytes;
pub mod cast;
mod checked;
mod cmp;
mod const_trait_fillers;
mod consts;
mod convert;
mod div;
#[cfg(feature = "alloc")]
mod fmt;
mod mask;
mod math;
mod mul;
#[cfg(feature = "numtraits")]
mod numtraits;
mod ops;
mod overflowing;
mod radix;
mod saturating;
mod strict;
mod unchecked;
mod wrapping;

use crate::Byte;
use crate::{WideDigits, WideDigitsMut};

use crate::ExpType;
use crate::doc;

#[cfg(feature = "serde")]
use ::{
    serde::{Deserialize, Serialize},
    serde_big_array::BigArray,
};

#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use core::default::Default;

/// A fixed-size integer type generic over signedness and byte width.
/// 
/// `Integer` has two const-generic parameters:
/// - a boolean, `S`, which determines whether the integer behaves as an unsigned integer (`S = false`), or a signed integer (`S = true`);
/// - a `usize`, `N`, which specifies how many bytes the integer should contain. The bytes are stored in little endian (least significant byte first).
/// 
/// `Integer` aims to exactly replicate the behaviours of Rust's built-in unsigned integer types: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`.
///
/// `Integer` implements all the arithmetic traits from the [`core::ops`](https://doc.rust-lang.org/core/ops/) module. The behaviour of the implementation of these traits is the same as for Rust's primitive integers - i.e. in debug mode it panics on overflow, and in release mode it performs two's complement wrapping (see <https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow>). However, an attempt to divide by zero or calculate a remainder with a divisor of zero will always panic, unless the [`checked_`](#method.checked_div) methods are used, which never panic.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(BorshSerialize, BorshDeserialize, BorshSchema)
)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "valuable", derive(valuable::Valuable))]
#[repr(transparent)]
pub struct Integer<const S: bool, const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub(crate) bytes: [Byte; N],
}

pub type Uint<const N: usize> = Integer<false, N>;
pub type Int<const N: usize> = Integer<true, N>;

#[cfg(feature = "zeroize")]
impl<const S: bool, const N: usize> zeroize::DefaultIsZeroes for Integer<S, N> {}

impl<const S: bool, const N: usize> Integer<S, N> {
    #[inline(always)]
    pub(crate) const fn force_sign<const R: bool>(self) -> Integer<R, N> {
        Integer::from_bytes(self.bytes)
    }

    /// Creates a new unsigned integer from the given digit. The given digit is stored as the least significant digit.
    #[must_use]
    #[inline(always)]
    const fn from_byte(byte: Byte) -> Self {
        let mut out = Self::ZERO;
        out.bytes[0] = byte;
        out
    }

    #[inline]
    pub(crate) const fn as_wide_digits(&self) -> WideDigits<N, false, false> {
        WideDigits::new(&self.bytes)
    }

    #[inline]
    pub(crate) const fn as_wide_digits_mut(&mut self) -> WideDigitsMut<N, false, false> {
        WideDigitsMut::new(&mut self.bytes)
    }
}

impl<const S: bool, const N: usize> Integer<S, N> {
    const U128_DIGITS: usize = N.div_ceil(16);
    pub(crate) const FULL_U128_DIGITS: usize = N / 16;
    pub(crate) const U128_DIGIT_REMAINDER: usize = N % 16;
    pub(crate) const LAST_DIGIT_BYTES: usize = if Self::U128_DIGIT_REMAINDER == 0 {
        16
    } else {
        Self::U128_DIGIT_REMAINDER
    };
    pub(crate) const U128_BITS_REMAINDER: ExpType = Self::BITS % 128;
}

impl<const N: usize> Int<N> {
    #[inline(always)]
    pub(crate) const fn signed_digit(&self) -> i8 {
        self.bytes[N - 1] as _
    }
}

impl<const S: bool, const N: usize> Default for Integer<S, N> {
    #[doc = doc::default!()]
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl<const S: bool, const N: usize> quickcheck::Arbitrary for Integer<S, N> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                let a = <u128 as quickcheck::Arbitrary>::arbitrary(g);
                out.as_wide_digits_mut().set(i, a);
                i += 1;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::cast::As;
    
    crate::test::test_all! {
        testing integers;

        #[test]
        fn default() {
            assert_eq!(UTEST::default(), utest::default().as_());
        }
    }
}
