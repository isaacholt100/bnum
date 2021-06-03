use super::BUint;
use crate::digit::{Digit, self};
use core::mem::MaybeUninit;

impl<const N: usize> BUint<N> {

    /// Converts an integer from big endian to the target’s endianness.
    /// 
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(BUint::from_be(n), n);
    /// } else {
    ///     assert_eq!(BUint::from_be(n), n.swap_bytes());
    /// }
    /// ```
    #[cfg(target_endian = "big")]
    pub const fn from_be(x: Self) -> Self {
        x
    }

    /// Converts an integer from big endian to the target’s endianness.
    /// 
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(BUint::from_be(n), n);
    /// } else {
    ///     assert_eq!(BUint::from_be(n), n.swap_bytes());
    /// }
    /// ```
    #[cfg(not(target_endian = "big"))]
    pub const fn from_be(x: Self) -> Self {
        x.swap_bytes()
    }

    /// Converts an integer from little endian to the target’s endianness.
    /// 
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(BUint::from_le(n), n);
    /// } else {
    ///     assert_eq!(BUint::from_le(n), n.swap_bytes());
    /// }
    /// ```
    #[cfg(target_endian = "little")]
    pub const fn from_le(x: Self) -> Self {
        x
    }

    /// Converts an integer from little endian to the target’s endianness.
    /// 
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(BUint::from_le(n), n);
    /// } else {
    ///     assert_eq!(BUint::from_le(n), n.swap_bytes());
    /// }
    /// ```
    #[cfg(not(target_endian = "little"))]
    pub const fn from_le(x: Self) -> Self {
        x.swap_bytes()
    }

    /// Converts `self` from big endian to the target’s endianness.
    /// 
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(n.to_be(), n);
    /// } else {
    ///     assert_eq!(n.to_be(), n.swap_bytes());
    /// }
    /// ```
    #[cfg(target_endian = "big")]
    pub const fn to_be(self) -> Self {
        self
    }

    /// Converts `self` from big endian to the target’s endianness.
    /// 
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(n.to_be(), n);
    /// } else {
    ///     assert_eq!(n.to_be(), n.swap_bytes());
    /// }
    /// ```
    #[cfg(not(target_endian = "big"))]
    pub const fn to_be(self) -> Self {
        self.swap_bytes()
    }

    /// Converts `self` from little endian to the target’s endianness.
    /// 
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(n.to_le(), n);
    /// } else {
    ///     assert_eq!(n.to_le(), n.swap_bytes());
    /// }
    /// ```
    #[cfg(target_endian = "little")]
    pub const fn to_le(self) -> Self {
        self
    }

    /// Converts `self` from little endian to the target’s endianness.
    /// 
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x1Au128);
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(n.to_le(), n);
    /// } else {
    ///     assert_eq!(n.to_le(), n.swap_bytes());
    /// }
    /// ```
    #[cfg(not(target_endian = "little"))]
    pub const fn to_le(self) -> Self {
        self.swap_bytes()
    }

    /// Returns the memory representation of this integer as a byte array in big-endian byte order.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let bytes = BUint::<2>::from(0x12345678901234567890123456789012u128).to_be_bytes();
    /// assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]);
    /// ```
    pub const fn to_be_bytes(self) -> [u8; N * 8] {
        let mut bytes = [0; N * 8];
        let mut i = N;
        while i > 0 {
            let digit_bytes = self.digits[N - i].to_be_bytes();
            i -= 1;
            let mut j = 0;
            while j < digit::BYTES {
                bytes[(i << digit::BYTE_SHIFT) + j] = digit_bytes[j];
                j += 1;
            }
        }
        bytes
    }

    /// Returns the memory representation of this integer as a byte array in little-endian byte order.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let bytes = BUint::<2>::from(0x12345678901234567890123456789012u128).to_le_bytes();
    /// assert_eq!(bytes, [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);
    /// ```
    pub const fn to_le_bytes(self) -> [u8; N * 8] {
        let mut bytes = [0; N * 8];
        let mut i = 0;
        while i < N {
            let digit_bytes = self.digits[i].to_le_bytes();
            let mut j = 0;
            while j < digit::BYTES {
                bytes[(i << digit::BYTE_SHIFT) + j] = digit_bytes[j];
                j += 1;
            }
            i += 1;
        }
        bytes
    }

    /// Return the memory representation of this integer as a byte array in native byte order.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let bytes = BUint::<2>::from(0x12345678901234567890123456789012u128).to_ne_bytes();
    /// assert_eq!(
    ///     bytes,
    ///     if cfg!(target_endian = "big") {
    ///         [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]
    ///     } else {
    ///         [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    ///     }
    /// );
    /// ```
    #[cfg(target_endian = "big")]
    pub const fn to_ne_bytes(self) -> [u8; N * 8] {
        self.to_be_bytes()
    }

    /// Return the memory representation of this integer as a byte array in native byte order.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let bytes = BUint::<2>::from(0x12345678901234567890123456789012u128).to_ne_bytes();
    /// assert_eq!(
    ///     bytes,
    ///     if cfg!(target_endian = "big") {
    ///         [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]
    ///     } else {
    ///         [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    ///     }
    /// );
    /// ```
    #[cfg(not(target_endian = "big"))]
    pub const fn to_ne_bytes(self) -> [u8; N * 8] {
        self.to_le_bytes()
    }

    /// Create a native endian integer value from its representation as a byte array in big endian.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// let value = BUint::<2>::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]);
    /// 
    /// assert_eq!(value, BUint::from(0x12345678901234567890123456789012u128));
    /// ```
    pub const fn from_be_bytes(bytes: [u8; N * 8]) -> Self {
        let mut out = Self::ZERO;
        let arr_ptr = bytes.as_ptr();
        let mut i = 0;
        while i < N {
            let mut uninit = MaybeUninit::<[u8; 8]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                arr_ptr.add((Self::N_MINUS_1 - i) << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES);
                uninit.assume_init()
            };
            out.digits[i] = Digit::from_be_bytes(digit_bytes);
            i += 1;
        }
        out
    }

    /// Create a native endian integer value from its representation as a byte array in little endian.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// let value = BUint::<2>::from_le_bytes([0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]);
    /// 
    /// assert_eq!(value, BUint::from(0x12345678901234567890123456789012u128));
    /// ```
    pub const fn from_le_bytes(bytes: [u8; N * 8]) -> Self {
        let mut out = Self::ZERO;
        let arr_ptr = bytes.as_ptr();
        let mut i = 0;
        while i < N {
            let mut uninit = MaybeUninit::<[u8; 8]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                arr_ptr.add(i << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES);
                uninit.assume_init()
            };
            out.digits[i] = Digit::from_le_bytes(digit_bytes);
            i += 1;
        }
        out
    }

    /// Create a native endian integer value from its representation as a byte array in native endianness.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// let value = BUint<2>::from_ne_bytes(if cfg!(target_endian = "big") {
    ///     [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]
    /// } else {
    ///     [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    /// });
    /// 
    /// assert_eq!(value, BUint::from(0x12345678901234567890123456789012u128));
    /// ```
    #[cfg(target_endian = "big")]
    pub const fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::from_be_bytes(bytes)
    }

    /// Create a native endian integer value from its representation as a byte array in native endianness.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// let value = BUint::<2>::from_ne_bytes(if cfg!(target_endian = "big") {
    ///     [0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]
    /// } else {
    ///     [0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]
    /// });
    /// 
    /// assert_eq!(value, BUint::from(0x12345678901234567890123456789012u128));
    /// ```
    #[cfg(not(target_endian = "big"))]
    pub const fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::from_le_bytes(bytes)
    }

    /// Create a native endian integer value from a slice of bytes in big endian. The value is wrapped in an `Option` as the bytes may be too large to be represented by the type.
    /// 
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros at the start so that it's length equals `Self::BYTES`.
    /// 
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless leading zeros from the slice can be removed until the length of the slice equals `Self::BYTES`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let value_from_array = BUint::<2>::from_be_bytes([0, 0, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]);
    /// let value_from_slice = BUint::<2>::from_be_slice(&[0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]).unwrap();
    /// let value_from_long_slice = BUint::<2>::from_be_slice(&[0, 0, 0, 0, 0, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]).unwrap();
    /// 
    /// assert_eq!(value_from_array, value_from_slice);
    /// assert_eq!(value_from_array, value_from_long_slice);
    /// 
    /// let invalid_slice = &[0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90];
    /// assert_eq!(BUint::<2>::from_be_slice(invalid_slice), None);
    /// ```
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        let mut out = Self::ZERO;
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; 8]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(len - 8 - (i << digit::BYTE_SHIFT)).copy_to_nonoverlapping(ptr, digit::BYTES);
                uninit.assume_init()
            };
            let digit = Digit::from_be_bytes(digit_bytes);
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            };
            i += 1;
        }
        let rem = len & (digit::BYTES - 1);
        if rem == 0 {
            Some(out)
        } else {
            let mut last_digit_bytes = [0; digit::BYTES];
            let mut j = 0;
            while j < rem {
                last_digit_bytes[8 - rem + j] = slice[j];
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

    /// Create a native endian integer value from a slice of bytes in little endian. The value is wrapped in an `Option` as the bytes may be too large to be represented by the type.
    /// 
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros at the end so that it's length equals `Self::BYTES`.
    /// 
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless trailing zeros from the slice can be removed until the length of the slice equals `Self::BYTES`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let value_from_array = BUint::<2>::from_le_bytes([0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0, 0]);
    /// let value_from_slice = BUint::<2>::from_le_slice(&[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56]).unwrap();
    /// let value_from_long_slice = BUint::<2>::from_le_slice(&[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0, 0, 0, 0, 0, 0]).unwrap();
    /// 
    /// assert_eq!(value_from_array, value_from_slice);
    /// assert_eq!(value_from_array, value_from_long_slice);
    /// 
    /// let invalid_slice = &[0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90];
    /// assert_eq!(BUint::<2>::from_le_slice(invalid_slice), None);
    /// ```
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        let mut out = Self::ZERO;
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; 8]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(i << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES);
                uninit.assume_init()
            };
            let digit = Digit::from_le_bytes(digit_bytes);
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            };
            i += 1;
        }
        if len & (digit::BYTES - 1) == 0 {
            Some(out)
        } else {
            let mut last_digit_bytes = [0; digit::BYTES];
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
    /*
    /// Create a native endian integer value from a slice of bytes in native endianness.
    #[cfg(target_endian = "big")]
    pub const fn from_ne_slice(bytes: &[u8]) -> Option<Self> {
        Self::from_be_slice(bytes)
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn from_ne_slice(bytes: &[u8]) -> Option<Self> {
        Self::from_le_slice(bytes)
    }*/
}

#[cfg(test)]
mod tests {
    use crate::U128;

    test_unsigned! {
        test_name: test_from_be,
        method: {
            from_be(234889034774398590845348573498570345u128);
            from_be(349857348957u128);
        }
    }
    test_unsigned! {
        test_name: test_from_le,
        method: {
            from_le(374598340857345349875907438579348534u128);
            from_le(9875474394587u128);
        }
    }
    test_unsigned! {
        test_name: test_to_be,
        method: {
            to_be(938495078934875384738495787358743854u128);
            to_be(394567834657835489u128);
        }
    }
    test_unsigned! {
        test_name: test_to_le,
        method: {
            to_le(634985790475394859374957339475897443u128);
            to_le(12379045679853u128);
        }
    }

    fn converter(bytes: [u8; 16]) -> [u8; 16] {
        bytes
    }

    test_unsigned! {
        test_name: test_to_be_bytes,
        method: {
            to_be_bytes(883497884590834905834758374950859884u128);
            to_be_bytes(456747598769u128);
        },
        converter: converter
    }
    test_unsigned! {
        test_name: test_to_le_bytes,
        method: {
            to_le_bytes(349587309485908349057389485093457397u128);
            to_le_bytes(4985679837455u128);
        },
        converter: converter
    }
    test_unsigned! {
        test_name: test_to_ne_bytes,
        method: {
            to_ne_bytes(123423345734905803845939847534085908u128);
            to_ne_bytes(685947586789335u128);
        },
        converter: converter
    }

    test_unsigned! {
        test_name: test_from_be_bytes,
        method: {
            from_be_bytes([3, 5, 44, 253, 55, 110, 64, 53, 54, 78, 0, 8, 91, 16, 25, 42]);
            from_be_bytes([0, 0, 0, 0, 30, 0, 64, 53, 54, 78, 0, 8, 91, 16, 25, 42]);
        }
    }
    test_unsigned! {
        test_name: test_from_le_bytes,
        method: {
            from_le_bytes([15, 65, 44, 30, 115, 200, 244, 167, 44, 6, 9, 11, 90, 56, 77, 150]);
            from_le_bytes([0, 0, 0, 0, 0, 200, 244, 167, 44, 6, 9, 11, 90, 56, 77, 150]);
        }
    }
    test_unsigned! {
        test_name: test_from_ne_bytes,
        method: {
            from_ne_bytes([73, 80, 2, 24, 160, 188, 204, 45, 33, 88, 4, 68, 230, 180, 145, 32]);
            from_ne_bytes([0, 0, 0, 0, 0, 188, 204, 45, 33, 88, 4, 68, 230, 180, 0, 0]);
        }
    }

    #[test]
    fn test_from_be_slice() {
        let arr = [73, 80, 2, 24, 160, 188, 204, 45, 33, 88, 4, 68, 230, 180, 145, 32];
        assert_eq!(U128::from_be_bytes(arr), U128::from_be_slice(&arr[..]).unwrap());
        let mut arr2 = arr;
        arr2[0] = 0;
        arr2[1] = 0;
        arr2[2] = 0;
        assert_eq!(U128::from_be_bytes(arr2), U128::from_be_slice(&arr2[2..]).unwrap());
        let mut v = arr.to_vec();
        v.insert(0, 0);
        v.insert(0, 0);
        v.insert(0, 0);
        assert_eq!(U128::from_be_bytes(arr), U128::from_be_slice(&v).unwrap());
        v.push(4);
        assert_eq!(U128::from_be_slice(&v), None);
    }

    #[test]
    fn test_from_le_slice() {
        let arr = [73, 80, 2, 24, 160, 188, 204, 45, 33, 88, 4, 68, 230, 180, 145, 32];
        assert_eq!(U128::from_le_bytes(arr), U128::from_le_slice(&arr[..]).unwrap());
        let mut arr2 = arr;
        arr2[15] = 0;
        arr2[14] = 0;
        arr2[13] = 0;
        assert_eq!(U128::from_le_bytes(arr2), U128::from_le_slice(&arr2[..13]).unwrap());
        let mut v = arr.to_vec();
        v.extend(vec![0, 0, 0, 0, 0].into_iter());
        assert_eq!(U128::from_le_bytes(arr), U128::from_le_slice(&v).unwrap());
        v.insert(0, 4);
        assert_eq!(U128::from_le_slice(&v), None);
    }

    /*#[test]
    fn test_from_ne_slice() {
        let arr = [65, 50, 100, 45, 224, 55, 10, 6, 88, 150, 230, 1, 0, 53, 90, 110];
        assert_eq!(U128::from_be_bytes(arr), U128::from_be_slice(&arr[..]).unwrap());
    }*/
}