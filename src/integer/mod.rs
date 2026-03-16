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

#[cfg(feature = "rand")]
mod random;

mod saturating;
mod strict;
mod unchecked;
mod wrapping;
mod const_trait_fillers;

use crate::Byte;
use crate::digits::Digits;

use crate::OverflowMode;
use crate::doc;

#[cfg(feature = "serde")]
use ::{
    serde::{Deserialize, Serialize},
    serde_big_array::BigArray,
};

#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use core::default::Default;

/// A fixed-size integer type, generic over signedness, bit width, and overflow behaviour.
/// 
/// `Integer` has four const-generic parameters:
/// - `S`: determines whether the integer behaves as an unsigned integer (`S = false`), or a signed integer (`S = true`);
/// - `N`: specifies how many bytes should be used to store the integer. The bytes are stored in little endian order (least significant byte first).
/// - `B`: specifies the bit width of the integer. If `B = 0` (the default value), then the bit width of the integer is taken to be `N * 8`. Otherwise, the bit width is taken to be `B`, and in this case it is required that `N - 8 < B*8 <= N` (i.e. `N = B.div_ceil(8)`).
/// - `OM`: specifies the behaviour of the type when arithmetic overflow occurs. There are three valid overflow modes, each corresponding to a variant of the [`OverflowMode`] enum:
///    - `0` ([`Wrap`](OverflowMode::Wrap)): arithmetic operations wrap around on overflow.
///    - `1` ([`Panic`](OverflowMode::Panic)): arithmetic operations panic on overflow.
///    - `2` ([`Saturate`](OverflowMode::Saturate)): arithmetic operations saturate on overflow.  
/// By default, `OM` is set to `0` if the [`overflow-checks` flag](https://doc.rust-lang.org/cargo/reference/profiles.html#overflow-checks) is disabled, and `1` if `overflow-checks` is enabled.
/// 
/// `Integer` closely follows the API and behaviour of Rust's primitive integer types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize` and `isize`. The only differences are:
/// - The primitive integers are stored in native-endian byte order. `Integer`s are always stored in little-endian byte order.
/// - Primitive integers are serialised in [`serde`](https://docs.rs/serde/latest/serde/) as decimal strings. `Integer`s are serialised using [`derive(Serialize)`](https://docs.rs/serde/latest/serde/derive.Serialize.html), i.e. as a struct.
/// - The primitive integers panic on arithmetic overflow if [`overflow-checks`](https://doc.rust-lang.org/cargo/reference/profiles.html#overflow-checks) is enabled, and wrap around on overflow if `overflow-checks` is disabled. The overflow behaviour of `Integer` is determined by [`Self::OVERFLOW_MODE`]:
///    - [`Wrap`](OverflowMode::Wrap): arithmetic operations wrap around on overflow, so the behaviour is the same as the [`Wrapping(T)`](core::num::Wrapping) type in the standard library (i.e. the same as the primitive integer type behaviour when `overflow-checks` is disabled).
///    - [`Panic`](OverflowMode::Panic): arithmetic operations panic on overflow, so the behaviour is the same as the primitive integer type behaviour when `overflow-checks` is enabled.
///    - [`Saturate`](OverflowMode::Saturate): arithmetic operations saturate on overflow, so the behaviour is the same as the [`Saturating(T)`](core::num::Saturating) type in the standard library.
///    - [`OverflowMode::DEFAULT`]: the overflow behaviour is the same as the primitive integer type overflow behaviour.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(BorshSerialize, BorshDeserialize, BorshSchema)
)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "valuable", derive(valuable::Valuable))]
#[repr(transparent)]
pub struct Integer<const S: bool, const N: usize, const B: usize = 0, const OM: u8 = {OverflowMode::DEFAULT as u8}> {
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub(crate) bytes: [Byte; N],
}

/// Unsigned integer type with const-generic bit width and overflow behaviour.
/// 
/// By default, the overflow behaviour is the same as the primitive integer types (i.e. panicking when the `overflow-checks` flag is enabled, and wrapping when `overflow-checks` is disabled).
/// 
/// For more details on how the const-generic parameters are interpreted, see the documentation for [`Integer`].
pub type Uint<const N: usize, const B: usize = 0, const OM: u8 = {OverflowMode::DEFAULT as u8}> = Integer<false, N, B, OM>;

/// Signed integer type with const-generic bit width and overflow behaviour.
/// 
/// By default, the overflow behaviour is the same as the primitive integer types (i.e. panicking when the `overflow-checks` flag is enabled, and wrapping when `overflow-checks` is disabled).
/// 
/// For more details on how the const-generic parameters are interpreted, see the documentation for [`Integer`].
pub type Int<const N: usize, const B: usize = 0, const OM: u8 = {OverflowMode::DEFAULT as u8}> = Integer<true, N, B, OM>;

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
    const fn has_valid_pad_bits(&self) -> bool {
        if Self::LAST_BYTE_BITS == 8 {
            true
        } else {
            (self.bytes[N - 1] >> Self::LAST_BYTE_BITS) == 0
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
    pub(crate) const fn as_digits<D>(&self) -> &Digits<D, N> {
        Digits::from_integer_ref(&self)
    }

    #[inline]
    pub(crate) const fn to_digits<D>(self) -> Digits<D, N> {
        Digits::from_integer(self)
    }
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
    #[inline]
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        <Digits<u128, N> as quickcheck::Arbitrary>::arbitrary(g).to_integer()
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
