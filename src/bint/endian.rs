use super::BIntD8;
use crate::{BUintD8, Digit};

use crate::digit;
use crate::doc;
// use core::mem::MaybeUninit;

macro_rules! set_digit {
    ($out_digits: ident, $i: expr, $digit: expr, $is_negative: expr, $sign_bits: expr) => {
        if $i == Self::N_MINUS_1 {
            if ($digit as digit::SignedDigit).is_negative() == $is_negative {
                $out_digits[$i] = $digit;
            } else {
                return None;
            }
        } else if $i < N {
            $out_digits[$i] = $digit;
        } else if $digit != $sign_bits {
            return None;
        };
    };
}

#[doc = doc::endian::impl_desc!(BIntD8)]
impl<const N: usize> BIntD8<N> {
    #[doc = doc::endian::from_be!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
        Self::from_bits(BUintD8::from_be(x.bits))
    }

    #[doc = doc::endian::from_le!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
        Self::from_bits(BUintD8::from_le(x.bits))
    }

    #[doc = doc::endian::to_be!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be(self) -> Self {
        Self::from_be(self)
    }

    #[doc = doc::endian::to_le!(I 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le(self) -> Self {
        Self::from_le(self)
    }

    /// Create an integer value from a slice of bytes in big endian. The value is wrapped in an [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) as the integer represented by the slice of bytes may represent an integer too large to be represented by the type.
    ///
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros or ones at the start so that it's length equals `Self::BYTES`. It is padded with ones if the bytes represent a negative integer, otherwise it is padded with zeros.
    ///
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless the bytes represent a non-negative integer and leading zeros from the slice can be removed until the length of the slice equals `Self::BYTES`, or if the bytes represent a negative integer and leading ones from the slice can be removed until the length of the slice equals `Self::BYTES`.
    ///
    /// For examples, see the
    #[doc = concat!("[`from_be_slice`](crate::", stringify!(BUintD8), "::from_be_slice)")]
    /// method documentation for
    #[doc = concat!("[`", stringify!(BUintD8), "`](crate::", stringify!(BUintD8), ").")]
    #[must_use = doc::must_use_op!()]
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        if len == 0 {
            return Some(Self::ZERO);
        }
        let is_negative = (slice[0] as i8).is_negative();
        let sign_bits = if is_negative { Digit::MAX } else { Digit::MIN };
        let mut out_digits = if is_negative { [Digit::MAX; N] } else { [0; N] };
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut digit_bytes = [0u8; digit::BYTES as usize];
            let init_index = len - digit::BYTES as usize;
            let mut j = init_index;
            while j < slice.len() {
                digit_bytes[j - init_index] = slice[j - (i << digit::BYTE_SHIFT)];
                j += 1;
            }
            let digit = Digit::from_be_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        let rem = len & (digit::BYTES as usize - 1);
        if rem == 0 {
            Some(Self::from_bits(BUintD8::from_digits(out_digits)))
        } else {
            let pad_byte = if is_negative { u8::MAX } else { 0 };
            let mut last_digit_bytes = [pad_byte; digit::BYTES as usize];
            let mut j = 0;
            while j < rem {
                last_digit_bytes[digit::BYTES as usize - rem + j] = slice[j];
                j += 1;
            }
            let digit = Digit::from_be_bytes(last_digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            Some(Self::from_bits(BUintD8::from_digits(out_digits)))
        }
    }

    /// Creates an integer value from a slice of bytes in little endian. The value is wrapped in an [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) as the bytes may represent an integer too large to be represented by the type.
    ///
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros or ones at the end so that it's length equals `Self::BYTES`. It is padded with ones if the bytes represent a negative integer, otherwise it is padded with zeros.
    ///
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless the bytes represent a non-negative integer and trailing zeros from the slice can be removed until the length of the slice equals `Self::BYTES`, or if the bytes represent a negative integer and trailing ones from the slice can be removed until the length of the slice equals `Self::BYTES`.
    ///
    /// For examples, see the
    #[doc = concat!("[`from_le_slice`](crate::", stringify!(BUintD8), "::from_le_slice)")]
    /// method documentation for
    #[doc = concat!("[`", stringify!(BUintD8), "`](crate::", stringify!(BUintD8), ").")]
    #[must_use = doc::must_use_op!()]
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        if len == 0 {
            return Some(Self::ZERO);
        }
        let is_negative = (slice[len - 1] as i8).is_negative();
        let sign_bits = if is_negative { Digit::MAX } else { Digit::MIN };
        let mut out_digits = [sign_bits; N];
        // let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut digit_bytes = [0u8; digit::BYTES as usize];
            let init_index = i << digit::BYTE_SHIFT;
            let mut j = init_index;
            while j < init_index + digit::BYTES as usize {
                digit_bytes[j - init_index] = slice[j];
                j += 1;
            }

            let digit = Digit::from_le_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        if len & (digit::BYTES as usize - 1) == 0 {
            Some(Self::from_bits(BUintD8::from_digits(out_digits)))
        } else {
            let pad_byte = if is_negative { u8::MAX } else { 0 };
            let mut last_digit_bytes = [pad_byte; digit::BYTES as usize];
            let addition = exact << digit::BYTE_SHIFT;
            let mut j = 0;
            while j + addition < len {
                last_digit_bytes[j] = slice[j + addition];
                j += 1;
            }
            let digit = Digit::from_le_bytes(last_digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            Some(Self::from_bits(BUintD8::from_digits(out_digits)))
        }
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_be_bytes!(I)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; BUintD8::<N>::BYTES_USIZE] {
        self.bits.to_be_bytes()
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_le_bytes!(I)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; BUintD8::<N>::BYTES_USIZE] {
        self.bits.to_le_bytes()
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_ne_bytes!(I)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; BUintD8::<N>::BYTES_USIZE] {
        self.bits.to_ne_bytes()
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_be_bytes!(I)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; BUintD8::<N>::BYTES_USIZE]) -> Self {
        Self::from_bits(BUintD8::from_be_bytes(bytes))
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_le_bytes!(I)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; BUintD8::<N>::BYTES_USIZE]) -> Self {
        Self::from_bits(BUintD8::from_le_bytes(bytes))
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_ne_bytes!(I)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; BUintD8::<N>::BYTES_USIZE]) -> Self {
        Self::from_bits(BUintD8::from_ne_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::*;

    crate::int::endian::tests!(itest);
}
