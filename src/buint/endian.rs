use super::BUintD8;
use crate::{digit, Digit};
use crate::doc;
// use core::mem::MaybeUninit;

#[doc = doc::endian::impl_desc!(BUintD8)]
impl<const N: usize> BUintD8<N> {
    #[doc = doc::endian::from_be!(U 256)]
    #[must_use]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        return x;
        #[cfg(not(target_endian = "big"))]
        x.swap_bytes()
    }

    #[doc = doc::endian::from_le!(U 256)]
    #[must_use]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        return x;
        #[cfg(not(target_endian = "little"))]
        x.swap_bytes()
    }

    #[doc = doc::endian::to_be!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be(self) -> Self {
        Self::from_be(self)
    }

    #[doc = doc::endian::to_le!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le(self) -> Self {
        Self::from_le(self)
    }

    /// Create an integer value from a slice of bytes in big endian. The value is wrapped in an `Option` as the integer represented by the slice of bytes may represent an integer too large to be represented by the type.
    ///
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros at the start so that it's length equals `Self::BYTES`.
    ///
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless leading zeros from the slice can be removed until the length of the slice equals `Self::BYTES`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::U128;
    ///
    /// let value_from_array = U128::from(u128::from_be_bytes([0, 0, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]));
    /// // using the `from_be_bytes` method on the primitve `u128` here instead of on `U128` as `from_be_bytes` is currently only available in bnum on nightly
    /// let value_from_slice = U128::from_be_slice(&[0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]).unwrap();
    /// let value_from_long_slice = U128::from_be_slice(&[0, 0, 0, 0, 0, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]).unwrap();
    ///
    /// assert_eq!(value_from_array, value_from_slice);
    /// assert_eq!(value_from_array, value_from_long_slice);
    ///
    /// let invalid_slice = &[0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90];
    /// assert_eq!(U128::from_be_slice(invalid_slice), None);
    /// ```
    #[must_use]
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        let mut out = Self::ZERO;
        // let slice_ptr = slice.as_ptr();
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
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            };
            i += 1;
        }
        let rem = len & (digit::BYTES as usize - 1);
        if rem == 0 {
            Some(out)
        } else {
            let mut last_digit_bytes = [0; digit::BYTES as usize];
            let mut j = 0;
            while j < rem {
                last_digit_bytes[digit::BYTES as usize - rem + j] = slice[j];
                j += 1;
            }
            let digit = Digit::from_be_bytes(last_digit_bytes);
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            };
            Some(out)
        }
    }

    /// Creates an integer value from a slice of bytes in little endian. The value is wrapped in an `Option` as the bytes may represent an integer too large to be represented by the type.
    ///
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros at the end so that it's length equals `Self::BYTES`.
    ///
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless trailing zeros from the slice can be removed until the length of the slice equals `Self::BYTES`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::U128;
    ///
    /// let value_from_array = U128::from(u128::from_le_bytes([0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0, 0]));
    /// // using the `from_le_bytes` method on the primitve `u128` here instead of on `U128` as `from_le_bytes` is currently only available in bnum on nightly
    /// let value_from_slice = U128::from_le_slice(&[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56]).unwrap();
    /// let value_from_long_slice = U128::from_le_slice(&[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0, 0, 0, 0, 0, 0]).unwrap();
    ///
    /// assert_eq!(value_from_array, value_from_slice);
    /// assert_eq!(value_from_array, value_from_long_slice);
    ///
    /// let invalid_slice = &[0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90];
    /// assert_eq!(U128::from_le_slice(invalid_slice), None);
    /// ```
    #[must_use]
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        let mut out = Self::ZERO;
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
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            };
            i += 1;
        }
        if len & (digit::BYTES as usize - 1) == 0 {
            Some(out)
        } else {
            let mut last_digit_bytes = [0; digit::BYTES as usize];
            let addition = exact << digit::BYTE_SHIFT;
            let mut j = 0;
            while j + addition < len {
                last_digit_bytes[j] = slice[j + addition];
                j += 1;
            }
            let digit = Digit::from_le_bytes(last_digit_bytes);
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            };
            Some(out)
        }
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_be_bytes!(U)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; Self::BYTES_USIZE] {
        let mut bytes = [0; Self::BYTES_USIZE];
        let mut i = N;
        while i > 0 {
            let digit_bytes = self.digits[N - i].to_be_bytes();
            i -= 1;
            let mut j = 0;
            while j < digit::BYTES as usize {
                bytes[(i << digit::BYTE_SHIFT) + j] = digit_bytes[j];
                j += 1;
            }
        }
        bytes
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_le_bytes!(U)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; Self::BYTES_USIZE] {
        // Strangely, this is slightly faster than direct transmutation by either `mem::transmute_copy` or `ptr::read`.
        // Also, initialising the bytes with zeros is faster than using MaybeUninit.
        // The Rust compiler is probably being very smart and optimizing this code.
        // The same goes for `to_be_bytes`.
        let mut bytes = [0; Self::BYTES_USIZE];
        let mut i = 0;
        while i < N {
            let digit_bytes = self.digits[i].to_le_bytes();
            let mut j = 0;
            while j < digit::BYTES as usize {
                bytes[(i << digit::BYTE_SHIFT) + j] = digit_bytes[j];
                j += 1;
            }
            i += 1;
        }
        bytes
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::to_ne_bytes!(U)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; Self::BYTES_USIZE] {
        #[cfg(target_endian = "big")]
        return self.to_be_bytes();
        #[cfg(not(target_endian = "big"))]
        self.to_le_bytes()
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_be_bytes!(U)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; Self::BYTES_USIZE]) -> Self {
        let mut out = Self::ZERO;
        // let arr_ptr = bytes.as_ptr();
        let mut i = 0;
        while i < N {
            let mut digit_bytes = [0u8; digit::BYTES as usize];
            let init_index = N * digit::BYTES as usize - digit::BYTES as usize;
            let mut j = init_index;
            while j < N * digit::BYTES as usize {
                digit_bytes[j - init_index] = bytes[j - (i << digit::BYTE_SHIFT)];
                j += 1;
            }
            out.digits[i] = Digit::from_be_bytes(digit_bytes);
            i += 1;
        }
        out
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_le_bytes!(U)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; Self::BYTES_USIZE]) -> Self {
        let mut out = Self::ZERO;
        // let arr_ptr = bytes.as_ptr();
        let mut i = 0;
        while i < N {
            let mut digit_bytes = [0u8; digit::BYTES as usize];
            let init_index = i << digit::BYTE_SHIFT;
            let mut j = init_index;
            while j < init_index + digit::BYTES as usize {
                digit_bytes[j - init_index] = bytes[j];
                j += 1;
            }
            out.digits[i] = Digit::from_le_bytes(digit_bytes);
            i += 1;
        }
        out
    }

    #[cfg(feature = "nightly")]
    #[doc = doc::endian::from_ne_bytes!(U)]
    #[doc = doc::requires_feature!("nightly")]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; Self::BYTES_USIZE]) -> Self {
        #[cfg(target_endian = "big")]
        return Self::from_be_bytes(bytes);

        #[cfg(not(target_endian = "big"))]
        Self::from_le_bytes(bytes)
    }

    pub(crate) const BYTES_USIZE: usize = N * digit::BYTES as usize;
}

#[cfg(test)]
mod tests {
    use crate::test::{test_bignum, types::*};

    crate::int::endian::tests!(utest);
}
