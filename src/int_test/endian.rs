use super::BintTest;
use crate::digit::{self, SignedDigit};
use crate::uint::BUint;

impl<const N: usize> BintTest<N> {
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
    pub const fn to_be_bytes(self) -> [u8; N * 8] where crate::BUint<{Self::BYTES}>: Sized {
        self.uint.to_be_bytes()
    }
    pub const fn to_le_bytes(self) -> [u8; N * 8] where crate::BUint<{Self::BYTES}>: Sized {
        self.uint.to_le_bytes()
    }
    #[cfg(target_endian = "big")]
    pub const fn to_ne_bytes(self) -> [u8; Self::BYTES] {
        self.to_be_bytes()
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn to_ne_bytes(self) -> [u8; N * 8] where crate::BUint<{Self::BYTES}>: Sized {
        self.to_le_bytes()
    }
    /*pub const fn from_be_bytes(bytes: [u8; Self::BYTES]) -> Self where [u8; Self::UINT_BYTES]: Sized {
        let signed_bytes_ptr = (&bytes[0..BYTES]).as_ptr() as *const [u8; BYTES];
        let signed_bytes: [u8; BYTES] = *signed_bytes_ptr;
        let uint_bytes_ptr = (&bytes[BYTES..]).as_ptr() as *const [u8; Self::UINT_BYTES];
        let uint_bytes: [u8; Self::UINT_BYTES] = *uint_bytes_ptr;
        Self {
            signed_digit: SignedDigit::from_be_bytes(signed_bytes),
            uint: BUint::from_be_bytes(uint_bytes)
        }
    }
    pub const fn from_le_bytes(bytes: [u8; Self::BYTES]) -> Self {
        let mut int = Self::ZERO;
        let mut i = 0;
        while i < N {
            let digit_bytes = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3], bytes[i + 4], bytes[i + 5], bytes[i + 6], bytes[i + 7]];
            int.digits[i] = Digit::from_le_bytes(digit_bytes);
            i += BYTES;
        }
        int
    }
    #[cfg(target_endian = "big")]
    pub const fn from_ne_bytes(bytes: [u8; Self::BYTES]) -> Self {
        Self::from_be_bytes(bytes)
    }
    #[cfg(not(target_endian = "big"))]
    pub const fn from_ne_bytes(bytes: [u8; Self::BYTES]) -> Self {
        Self::from_le_bytes(bytes)
    }*/
}

#[cfg(test)]
mod tests {
    use crate::I128Test;

    test_signed! {
        test_name: test_from_be,
        method: from_be(-34958734895739475893434758937397458i128)
    }
    test_signed! {
        test_name: test_from_le,
        method: from_le(34053485789374589734957349573987458934i128)
    }
    test_signed! {
        test_name: test_to_be,
        method: to_be(938495078934875384738495787358743854i128)
    }
    test_signed! {
        test_name: test_to_le,
        method: to_le(-634985790475394859374957339475897443i128)
    }

    fn converter(bytes: [u8; 16]) -> [u8; 16] {
        bytes
    }

    test_signed! {
        test_name: test_to_be_bytes,
        method: to_be_bytes(-94564564534345004567495879873945739i128),
        converter: converter
    }
    test_signed! {
        test_name: test_to_le_bytes,
        method: to_le_bytes(4589674598797290345798374895793745019i128),
        converter: converter
    }
    test_signed! {
        test_name: test_to_ne_bytes,
        method: to_ne_bytes(-9547689209348758902934752103969375839i128),
        converter: converter
    }
}