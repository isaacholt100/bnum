use super::BUint;
use crate::digit::{Digit, DIGIT_BYTES, DIGIT_BYTE_SHIFT};

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
            while j < DIGIT_BYTES {
                bytes[(i << DIGIT_BYTE_SHIFT) + j] = digit_bytes[j];
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
            while j < DIGIT_BYTES {
                bytes[(i << DIGIT_BYTE_SHIFT) + j] = digit_bytes[j];
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
    pub const fn const_from_be_bytes(bytes: [u8; N * 8]) -> Self {
        let mut int = Self::ZERO;
        let mut i = 0;
        while i < N {
            let j = Self::N_MINUS_1 - i;
            let digit_bytes = [bytes[j - 7], bytes[j - 6], bytes[j - 5], bytes[j - 4], bytes[j - 3], bytes[j - 2], bytes[j - 1], bytes[j]];
            int.digits[i] = Digit::from_be_bytes(digit_bytes);
            i += DIGIT_BYTES;
        }
        int
    }
    pub fn from_be_bytes(bytes: [u8; N * 8]) -> Self {
        let mut int = Self::ZERO;
        let mut i = 0;
        while i < N {
            let j = N - i;
            let ptr = (&bytes[(j - DIGIT_BYTES)..j]).as_ptr() as *const [u8; DIGIT_BYTES];
            let digit_bytes: [u8; DIGIT_BYTES] = unsafe { *ptr };
            int.digits[i] = Digit::from_be_bytes(digit_bytes);
            i += DIGIT_BYTES;
        }
        int
    }
    pub const fn const_from_le_bytes(bytes: [u8; N * 8]) -> Self {
        let mut int = Self::ZERO;
        let mut i = 0;
        while i < N {
            let digit_bytes = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3], bytes[i + 4], bytes[i + 5], bytes[i + 6], bytes[i + 7]];
            int.digits[i] = Digit::from_le_bytes(digit_bytes);
            i += DIGIT_BYTES;
        }
        int
    }

    pub fn from_le_bytes(bytes: [u8; N * 8]) -> Self {
        let mut int = Self::ZERO;
        let mut i = 0;
        while i < N {
            let digit_bytes = unsafe {
                *((&bytes[i..(i + 8)]).as_ptr() as *const [u8; DIGIT_BYTES])
            };
            int.digits[i] = Digit::from_le_bytes(digit_bytes);
            i += DIGIT_BYTES;
        }
        int
    }
    #[cfg(target_endian = "big")]
    pub const fn const_from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::const_from_be_bytes(bytes)
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn const_from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::const_from_le_bytes(bytes)
    }
    #[cfg(target_endian = "big")]
    pub fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::from_be_bytes(bytes)
    }
    #[cfg(not(target_endian = "big"))]
    pub fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    test_unsigned! {
        test_name: test_from_be,
        method: from_be(234889034774398590845348573498570345u128)
    }
    test_unsigned! {
        test_name: test_from_le,
        method: from_le(374598340857345349875907438579348534u128)
    }
    test_unsigned! {
        test_name: test_to_be,
        method: to_be(938495078934875384738495787358743854u128)
    }
    test_unsigned! {
        test_name: test_to_le,
        method: to_le(634985790475394859374957339475897443u128)
    }

    fn convert(bytes: [u8; 16]) -> [u8; 16] {
        bytes
    }

    test_unsigned! {
        test_name: test_to_be_bytes,
        method: to_be_bytes(883497884590834905834758374950859884u128),
        converter: convert
    }
    test_unsigned! {
        test_name: test_to_le_bytes,
        method: to_le_bytes(349587309485908349057389485093457397u128),
        converter: convert
    }
    test_unsigned! {
        test_name: test_to_ne_bytes,
        method: to_ne_bytes(123423345734905803845939847534085908u128),
        converter: convert
    }
    
    #[test]
    fn test_from_be_bytes() {
        let a = 23547843905834589345903845984384598u128;
        let buint = U128::from(a);
        // Current causes compiler crash due to instability of const generics
        //assert_eq!(buint, U128::from_be_bytes(buint.to_be_bytes()));
        //assert_eq!(U128::from_be_bytes([4; 16]), u128::from_be_bytes([4; 16]).into());
    }
    #[test]
    fn test_from_le_bytes() {
        let a = 34590834563845890504549985948845098454u128;
        let buint = U128::from(a);
        // Current causes compiler crash due to instability of const generics
        //assert_eq!(buint, U128::from_le_bytes(buint.to_le_bytes()));
        //assert_eq!(U128::from_le_bytes([4; 16]), u128::from_le_bytes([4; 16]).into());
    }
    #[test]
    fn test_from_ne_bytes() {
        let a = 9876757883495934598734924753734758883u128;
        let buint = U128::from(a);
        // Current causes compiler crash due to instability of const generics
        //assert_eq!(buint, U128::from_ne_bytes(buint.to_ne_bytes()));
        //assert_eq!(U128::from_ne_bytes([4; 16]), u128::from_ne_bytes([4; 16]).into());
    }
}