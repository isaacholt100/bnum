mod bigint_helpers;
mod bits;
pub mod cast;
mod checked;
mod cmp;
mod const_trait_fillers;
mod consts;
mod convert;
mod div;
mod endian;
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

use crate::Digit;
use crate::{WideDigits, WideDigitsMut};

use crate::ExpType;
use crate::doc;
// use core::mem::MaybeUninit;

#[cfg(feature = "serde")]
use ::{
    serde::{Deserialize, Serialize},
    serde_big_array::BigArray,
};

#[cfg(feature = "borsh")]
use ::{
    // alloc::string::ToString,
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
};

use core::default::Default;

use core::iter::{Iterator, Product, Sum};

/// Unsigned integer type composed of `u8` digits, of arbitrary fixed size which must be known at compile time.
///
/// Digits are stored in little endian (least significant digit first). This integer type aims to exactly replicate the behaviours of Rust's built-in unsigned integer types: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`. The const generic parameter `N` is the number of `u8` digits that are stored.
///
#[doc = doc::arithmetic_doc!(Uint)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "borsh",
    derive(BorshSerialize, BorshDeserialize, BorshSchema)
)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "valuable", derive(valuable::Valuable))]
#[repr(transparent)]
pub struct Uint<const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub(crate) digits: [Digit; N],
}

#[cfg(feature = "zeroize")]
impl<const N: usize> zeroize::DefaultIsZeroes for Uint<N> {}

impl<const N: usize> Uint<N> {
    // #[inline(always)]
    // pub(crate) const fn digit(&self, index: usize) -> Digit {
    //     self.digits[index]
    // }

    /// Returns the digits stored in `self` as an array. Digits are little endian (least significant digit first).
    #[must_use]
    #[inline(always)]
    pub const fn digits(&self) -> &[Digit; N] {
        &self.digits
    }

    /// Returns the digits stored in `self` as a mutable array. Digits are little endian (least significant digit first).
    #[must_use]
    #[inline(always)]
    pub const fn digits_mut(&mut self) -> &mut [Digit; N] {
        &mut self.digits
    }

    const ASSERT_IS_VALID: () = {
        assert!(N != 0, "bnum types cannot be zero-sized");
        // `Self::BITS` must be at most `u32::MAX` (i.e. 2^32 - 1)
        // so `N` must be at most (2^32 - 1) / 8 = 2^29 - 1/8 = 2^29 - 1 (rounded down)
        if usize::BITS > 29 {
            // otherwise, since `N` is a usize, `N` must be at most `usize::MAX <= 2^29 - 1`
            assert!(N < (1 << 29), "bnum types must be less than 2^29 bytes");
        }
    };

    /// Creates a new unsigned integer from the given array of digits. Digits are stored as little endian (least significant digit first).
    #[must_use]
    #[inline(always)]
    pub const fn from_digits(digits: [Digit; N]) -> Self {
        // this is the only method where `Self` is explicitly constructed, all other methods use this one indirectly. thus, we can make all assertions about whether `N` is a valid size here.
        const { Self::ASSERT_IS_VALID };
        Self { digits }
    }

    /// Creates a new unsigned integer from the given digit. The given digit is stored as the least significant digit.
    #[must_use]
    #[inline(always)]
    pub const fn from_digit(digit: Digit) -> Self {
        let mut out = Self::ZERO;
        out.digits[0] = digit; 
        out
    }

    #[inline]
    pub(crate) const fn as_wide_digits(&self) -> WideDigits<N, false, false> {
        WideDigits::new(&self.digits)
    }

    #[inline]
    pub(crate) const fn as_wide_digits_mut(&mut self) -> WideDigitsMut<N, false, false> {
        WideDigitsMut::new(&mut self.digits)
    }
}

pub struct U128Digits<'a, const N: usize> {
    bits: &'a Uint<N>,
}

impl<'a, const N: usize> U128Digits<'a, N> {
    const LAST_DIGIT_BYTES: usize = if N % 16 == 0 { 16 } else { N % 16 };
    const LAST_LE_DIGIT_OFFSET: usize = N - Self::LAST_DIGIT_BYTES;

    #[inline]
    pub const fn new(bits: &'a Uint<N>) -> Self {
        Self { bits }
    }

    #[inline]
    pub const unsafe fn get_at_offset(&self, offset: usize) -> u128 {
        let mut bytes = [0; 16];
        let c = N - offset;
        let count = if c > 16 { 16 } else { c }; // this is a bit hack for min(c, 16)
        unsafe {
            self.bits
                .digits
                .as_ptr()
                .add(offset)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), count);
        }
        u128::from_le_bytes(bytes)
    }

    #[inline]
    pub const unsafe fn get(&self, index: usize) -> u128 {
        if index == Uint::<N>::U128_DIGITS - 1 {
            return self.last();
        }
        let mut bytes = [0; 16];
        unsafe {
            self.bits
                .digits
                .as_ptr()
                .add(index * 16)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), 16);
        }
        u128::from_le_bytes(bytes)
    }

    #[inline]
    pub const unsafe fn get_with_correct_count(&self, index: usize) -> u128 {
        unsafe { self.get_at_offset(index * 16) }
    }

    #[inline]
    pub const fn last_padded<const ONES: bool>(&self) -> u128 {
        let mut bytes = if ONES { [u8::MAX; 16] } else { [0; 16] };
        unsafe {
            self.bits
                .digits
                .as_ptr()
                .add(Self::LAST_LE_DIGIT_OFFSET)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::LAST_DIGIT_BYTES);
        }
        u128::from_le_bytes(bytes)
    }

    #[inline]
    pub const fn last(&self) -> u128 {
        self.last_padded::<false>()
    }
}

pub struct U128DigitsMut<'a, const N: usize> {
    bits: &'a mut Uint<N>,
}

impl<'a, const N: usize> U128DigitsMut<'a, N> {
    #[inline]
    pub const fn new(bits: &'a mut Uint<N>) -> Self {
        Self { bits }
    }

    #[inline]
    pub const unsafe fn set(&mut self, index: usize, value: u128) {
        if index == Uint::<N>::U128_DIGITS - 1 {
            return self.set_last(value);
        }
        let out_bytes = value.to_le_bytes();
        unsafe {
            out_bytes
                .as_ptr()
                .copy_to_nonoverlapping(self.bits.digits.as_mut_ptr().add(index * 16), 16);
        }
    }

    #[inline]
    pub const unsafe fn set_at_offset(&mut self, offset: usize, value: u128) {
        let out_bytes = value.to_le_bytes();
        let c = N - offset;
        let count = if c > 16 { 16 } else { c };
        unsafe {
            out_bytes
                .as_ptr()
                .copy_to_nonoverlapping(self.bits.digits.as_mut_ptr().add(offset), count);
        }
    }

    #[inline]
    pub const fn set_last(&mut self, value: u128) {
        let bytes = value.to_le_bytes();
        unsafe {
            self.bits
                .digits
                .as_mut_ptr()
                .add(U128Digits::<N>::LAST_LE_DIGIT_OFFSET)
                .copy_from_nonoverlapping(bytes.as_ptr(), U128Digits::<N>::LAST_DIGIT_BYTES);
        }
    }
}

impl<const N: usize> Uint<N> {
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

impl<const N: usize> Default for Uint<N> {
    #[doc = doc::default!()]
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const N: usize> Product<Self> for Uint<N> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for Uint<N> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for Uint<N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for Uint<N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl<const N: usize> quickcheck::Arbitrary for Uint<N> {
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
crate::test::test_all_widths! {
    use crate::test::{debug_skip, test_bignum};

    crate::ints::tests!(utest);

    test_bignum! {
        function: <utest>::next_power_of_two(a: utest),
        skip: debug_skip!(a.checked_next_power_of_two().is_none())
    }
    test_bignum! {
        function: <utest>::is_power_of_two(a: utest)
    }
    #[cfg(all(feature = "signed", feature = "nightly"))]
    test_bignum! {
        function: <utest>::cast_signed(a: utest)
    }

    #[test]
    fn digits() {
        let a = UTEST::MAX;
        let digits = *a.digits();
        assert_eq!(a, UTEST::from_digits(digits));
    }

    use crate::cast::{As, CastFrom};

    #[test]
    fn bit() {
        let u = UTEST::cast_from(0b001010100101010101u64);
        assert!(u.bit(0));
        assert!(!u.bit(1));
        // assert!(!u.bit(17));
        // assert!(!u.bit(16));
        assert!(u.bit(15));
    }

    #[test]
    fn set_bit() {
        let mut u = UTEST::cast_from(0b001010100101010101u64);
        u.set_bit(1, true);
        assert!(u.bit(1));
        u.set_bit(1, false);
        assert!(!u.bit(1));
        u.set_bit(14, false);
        assert!(!u.bit(14));
        u.set_bit(14, true);
        assert!(u.bit(14));
    }

    #[test]
    fn is_zero() {
        assert!(UTEST::MIN.is_zero());
        assert!(!UTEST::MAX.is_zero());
        assert!(!UTEST::ONE.is_zero());
    }

    #[test]
    fn is_one() {
        assert!(UTEST::ONE.is_one());
        assert!(!UTEST::MAX.is_one());
        assert!(!UTEST::ZERO.is_one());
        let mut digits = *crate::Uint::<2>::MAX.digits();
        digits[0] = 1;
        let b = crate::Uint::<2>::from_digits(digits);
        assert!(!b.is_one());
    }

    #[test]
    fn bits() {
        let u = UTEST::cast_from(0b1010100101010101u128);
        assert_eq!(u.bits(), 16);

        let u = UTEST::power_of_two(7);
        assert_eq!(u.bits(), 8);
    }

    #[test]
    fn default() {
        assert_eq!(UTEST::default(), utest::default().as_());
    }

    #[test]
    fn sum() {
        let v = vec![
            &UTEST::ZERO,
            &UTEST::ONE,
            &UTEST::TWO,
            &UTEST::THREE,
            &UTEST::FOUR,
        ];
        assert_eq!(UTEST::TEN, v.iter().copied().sum());
        assert_eq!(UTEST::TEN, v.into_iter().sum());
    }

    #[test]
    fn product() {
        let v = vec![&UTEST::ONE, &UTEST::TWO, &UTEST::THREE];
        assert_eq!(UTEST::SIX, v.iter().copied().sum());
        assert_eq!(UTEST::SIX, v.into_iter().sum());
    }
}

#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::is_power_of_two(a: utest)
    }
}

// implementation if we don't have alloc, as otherwise can't call assert_eq! (since this requires Debug)
#[cfg(all(test, not(feature = "alloc")))]
impl<const N: usize> core::fmt::Debug for Uint<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{:02x}", digit)?;
        }
        if self.is_zero() {
            write!(f, "0")?;
        }
        Ok(())
    }
}
