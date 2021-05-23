use super::BUint;
use crate::digit::{Digit, self};
use core::mem::MaybeUninit;

impl<const N: usize> BUint<N> {
    #[cfg(target_endian = "big")]
    pub const fn from_be(x: Self) -> Self {
        x
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn from_be(x: Self) -> Self {
        x.swap_bytes()
    }
    #[cfg(target_endian = "little")]
    pub const fn from_le(x: Self) -> Self {
        x
    }
    #[cfg(not(target_endian = "little"))]
    pub const fn from_le(x: Self) -> Self {
        x.swap_bytes()
    }
    #[cfg(target_endian = "big")]
    pub const fn to_be(self) -> Self {
        self
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn to_be(self) -> Self {
        self.swap_bytes()
    }
    #[cfg(target_endian = "little")]
    pub const fn to_le(self) -> Self {
        self
    }
    #[cfg(not(target_endian = "little"))]
    pub const fn to_le(self) -> Self {
        self.swap_bytes()
    }
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
    #[cfg(target_endian = "big")]
    pub const fn to_ne_bytes(self) -> [u8; N * 8] {
        self.to_be_bytes()
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn to_ne_bytes(self) -> [u8; N * 8] {
        self.to_le_bytes()
    }
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
    #[cfg(target_endian = "big")]
    pub const fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::from_be_bytes(bytes)
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::from_le_bytes(bytes)
    }
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
    #[cfg(target_endian = "big")]
    pub const fn from_ne_slice(bytes: &[u8]) -> Option<Self> {
        Self::from_be_slice(bytes)
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn from_ne_slice(bytes: &[u8]) -> Option<Self> {
        Self::from_le_slice(bytes)
    }
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
    fn test_from_ne_slice() {
        let arr = [65, 50, 100, 45, 224, 55, 10, 6, 88, 150, 230, 1, 0, 53, 90, 110];
        assert_eq!(U128::from_be_bytes(arr), U128::from_be_slice(&arr[..]).unwrap());
    }
}