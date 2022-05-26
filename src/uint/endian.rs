use super::BUint;
use crate::digit::{Digit, self};
use core::mem::MaybeUninit;
use crate::doc;

impl<const N: usize> BUint<N> {
    #[doc=doc::from_be!(BUint::<2>)]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
        #[cfg(target_endian = "big")]
        return x;
        #[cfg(not(target_endian = "big"))]
        x.swap_bytes()
    }

    #[doc=doc::from_le!(BUint::<2>)]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
        #[cfg(target_endian = "little")]
        return x;
        #[cfg(not(target_endian = "little"))]
        x.swap_bytes()
    }

    #[doc=doc::to_be!(BUint::<2>)]
    #[inline]
    pub const fn to_be(self) -> Self {
        Self::from_be(self)
    }

    #[doc=doc::to_be!(BUint::<2>)]
    #[inline]
    pub const fn to_le(self) -> Self {
        Self::from_le(self)
    }

    /// Create a native endian integer value from a slice of bytes in big endian. The value is wrapped in an `Option` as the integer represented by the slice of bytes may be too large to be represented by the type.
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
    #[inline]
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        let mut out = Self::ZERO;
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(len - digit::BYTES as usize - (i << digit::BYTE_SHIFT)).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
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
    #[inline]
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        let mut out = Self::ZERO;
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(i << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
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
}

#[cfg(feature = "nightly")]
impl<const N: usize> BUint<N> {
    #[doc=doc::to_be_bytes!(BUint::<2>, "u")]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N * digit::BYTES as usize] {
        let mut bytes = [0; N * digit::BYTES as usize];
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

    #[doc=doc::to_le_bytes!(BUint::<2>, "u")]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N * digit::BYTES as usize] {
        let mut bytes = [0; N * digit::BYTES as usize];
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

    #[doc=doc::to_ne_bytes!(Bint::<2>, "u")]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N * digit::BYTES as usize] {
        #[cfg(target_endian = "big")]
        return self.to_be_bytes();
        #[cfg(not(target_endian = "big"))]
        self.to_le_bytes()
    }

    #[doc=doc::from_be_bytes!(BUint::<2>, "u")]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        let mut out = Self::ZERO;
        let arr_ptr = bytes.as_ptr();
        let mut i = 0;
        while i < N {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                arr_ptr.add((Self::N_MINUS_1 - i) << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
                uninit.assume_init()
            };
            out.digits[i] = Digit::from_be_bytes(digit_bytes);
            i += 1;
        }
        out
    }

    #[doc=doc::from_le_bytes!(BUint::<2>, "u")]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        let mut out = Self::ZERO;
        let arr_ptr = bytes.as_ptr();
        let mut i = 0;
        while i < N {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                arr_ptr.add(i << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
                uninit.assume_init()
            };
            out.digits[i] = Digit::from_le_bytes(digit_bytes);
            i += 1;
        }
        out
    }

    #[doc=doc::from_ne_bytes!(BUint::<2>, "u")]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        #[cfg(target_endian = "big")]
        return Self::from_be_bytes(bytes);

        #[cfg(not(target_endian = "big"))]
        Self::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;
    use crate::test::U8ArrayWrapper;

    test_unsigned! {
        function: from_be(a: u128),
        cases: [
            (234889034774398590845348573498570345u128),
            (349857348957u128)
        ]
    }
    test_unsigned! {
        function: from_le(a: u128),
        cases: [
            (374598340857345349875907438579348534u128),
            (9875474394587u128)
        ]
    }
    test_unsigned! {
        function: to_be(a: u128),
        cases: [
            (938495078934875384738495787358743854u128),
            (394567834657835489u128)
        ]
    }
    test_unsigned! {
        function: to_le(a: u128),
        cases: [
            (634985790475394859374957339475897443u128),
            (12379045679853u128)
        ]
    }

    #[cfg(feature = "nightly")]
    test_unsigned! {
        function: to_be_bytes(a: u128),
        cases: [
            (883497884590834905834758374950859884u128),
            (456747598769u128)
        ]
    }
    #[cfg(feature = "nightly")]
    test_unsigned! {
        function: to_le_bytes(a: u128),
        cases: [
            (349587309485908349057389485093457397u128),
            (4985679837455u128)
        ]
    }
    #[cfg(feature = "nightly")]
    test_unsigned! {
        function: to_ne_bytes(a: u128),
        cases: [
            (123423345734905803845939847534085908u128),
            (685947586789335u128)
        ]
    }

    #[cfg(feature = "nightly")]
    test_unsigned! {
        function: from_be_bytes(a: U8ArrayWrapper<16>),
        cases: [
            ([3, 5, 44, 253, 55, 110, 64, 53, 54, 78, 0, 8, 91, 16, 25, 42]),
            ([0, 0, 0, 0, 30, 0, 64, 53, 54, 78, 0, 8, 91, 16, 25, 42])
        ]
    }
    #[cfg(feature = "nightly")]
    test_unsigned! {
        function: from_le_bytes(a: U8ArrayWrapper<16>),
        cases: [
            ([15, 65, 44, 30, 115, 200, 244, 167, 44, 6, 9, 11, 90, 56, 77, 150]),
            ([0, 0, 0, 0, 0, 200, 244, 167, 44, 6, 9, 11, 90, 56, 77, 150])
        ]
    }
    #[cfg(feature = "nightly")]
    test_unsigned! {
        function: from_ne_bytes(a: U8ArrayWrapper<16>),
        cases: [
            ([73, 80, 2, 24, 160, 188, 204, 45, 33, 88, 4, 68, 230, 180, 145, 32]),
            ([0, 0, 0, 0, 0, 188, 204, 45, 33, 88, 4, 68, 230, 180, 0, 0])
        ]
    }

    #[test]
    fn from_be_slice() {
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

        assert_eq!(U128::from_be_slice(&[]), Some(U128::ZERO));
    }

    #[test]
    fn from_le_slice() {
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

        assert_eq!(U128::from_le_slice(&[]), Some(U128::ZERO));
    }

    /*#[test]
    fn from_ne_slice() {
        let arr = [65, 50, 100, 45, 224, 55, 10, 6, 88, 150, 230, 1, 0, 53, 90, 110];
        assert_eq!(U128::from_be_bytes(arr), U128::from_be_slice(&arr[..]).unwrap());
    }*/
}