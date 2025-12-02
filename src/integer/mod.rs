mod consts;
mod bigint_helpers;
mod bits;
mod bytes;
pub mod cast;
mod checked;
mod cmp;
mod convert;
mod div;
#[cfg(feature = "alloc")]
mod fmt;
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
mod const_trait_fillers;

use crate::Byte;
use crate::{WideDigits, WideDigitsMut};

use crate::Exponent;
use crate::doc;

#[cfg(feature = "serde")]
use ::{
    serde::{Deserialize, Serialize},
    serde_big_array::BigArray,
};

#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use core::default::Default;

/// A fixed-size integer type, generic over signedness, byte-width, and overflow behaviour.
/// 
/// `Integer` has three const-generic parameters:
/// - `S`, of type `bool`, which determines whether the integer behaves as an unsigned integer (`S = false`), or a signed integer (`S = true`);
/// - `N`, of type `usize`, which specifies how many bytes the integer should contain. The bytes are stored in little endian (least significant byte first).
/// - `OM`, of type `u8`, which specifies the behaviour of the type when arithmetic overflow occurs. There are three possible modes:
///    - `0` (wrapping): arithmetic operations wrap around on overflow.
///    - `1` (panicking): arithmetic operations panic on overflow.
///    - `2` (saturating): arithmetic operations saturate on overflow.
/// By default, `OM` is set to `0` if the `overflow-checks` flag is disabled, and `1` if the `overflow-checks` flag is enabled. The enum [`OverflowMode`] has variants corresponding to each mode.
/// 
/// `Integer` aims to exactly replicate the API and behaviour of Rust's built-in integer types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize` and `isize`.
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
pub struct Integer<const S: bool, const N: usize, const B: usize = 0, const OM: u8 = {crate::OverflowMode::DEFAULT.to_u8()}> {
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub(crate) bytes: [Byte; N],
}

pub type Uint<const N: usize, const B: usize = 0, const OM: u8 = {crate::OverflowMode::DEFAULT.to_u8()}> = Integer<false, N, B, OM>;

pub type Int<const N: usize, const B: usize = 0, const OM: u8 = {crate::OverflowMode::DEFAULT.to_u8()}> = Integer<true, N, B, OM>;

#[cfg(feature = "zeroize")]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> zeroize::DefaultIsZeroes for Integer<S, N, B, OM> {}
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    pub(crate) const LAST_BYTE_BITS: u32 = if Self::BITS % Byte::BITS == 0 {
        Byte::BITS as _
    } else {
        Self::BITS % Byte::BITS
    };

    pub(crate) const LAST_BYTE_PAD_BITS: u32 = Byte::BITS - Self::LAST_BYTE_BITS;

    #[inline(always)]
    const fn widen(self) -> Integer<S, N, 0, OM> {
        self.force()
    }

    #[inline(always)]
    const fn set_sign_bits(&mut self) {
        self.set_pad_bits(self.is_negative_internal());
    }

    #[inline(always)]
    const fn set_pad_bits(&mut self, value: bool) {
        if Self::LAST_BYTE_PAD_BITS != 0 {
            if value {
                // set pad bits
                self.bytes[N - 1] |= Byte::MAX << Self::LAST_BYTE_BITS; // 11..100..0
            } else {
                // clear pad bits
                self.bytes[N - 1] &= !(Byte::MAX << Self::LAST_BYTE_BITS); // 00..011..1
            }
        }
    }

    #[inline(always)]
    pub(crate) const fn force_sign<const R: bool>(self) -> Integer<R, N, B, OM> {
        let mut out = Integer::from_bytes(self.bytes);
        if R != S {
            out.set_sign_bits();
        }
        out
    }

    #[inline(always)]
    pub(crate) const fn force<const R: bool, const A: usize, const RO: u8>(self) -> Integer<R, N, A, RO> {
        let mut out = Integer::from_bytes(self.bytes);
        if R != S {
            out.set_sign_bits();
        }
        out
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

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    const U128_DIGITS: usize = (Self::BITS as usize).div_ceil(128);
    pub(crate) const U128_DIGIT_REMAINDER: usize = N % 16;
    pub(crate) const LAST_DIGIT_BYTES: usize = if Self::U128_DIGIT_REMAINDER == 0 {
        16
    } else {
        Self::U128_DIGIT_REMAINDER
    };
    pub(crate) const U128_BITS_REMAINDER: Exponent = Self::BITS % 128;
}

impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    #[inline(always)]
    pub(crate) const fn signed_digit(&self) -> i8 {
        self.bytes[N - 1] as _
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Default for Integer<S, N, B, OM> {
    #[doc = doc::default!()]
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> quickcheck::Arbitrary for Integer<S, N, B, OM> {
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

// implementation if we don't have alloc, as otherwise can't call assert_eq! (since this requires Debug)
#[cfg(all(test, not(feature = "alloc")))]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> core::fmt::Debug for Integer<S, N, B, OM> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for bytes in self.bytes.iter().rev() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
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
