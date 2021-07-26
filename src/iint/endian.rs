use super::BIint;
use crate::uint::BUint;
use crate::digit::{self, Digit, SignedDigit};
use core::mem::MaybeUninit;

macro_rules! set_digit {
    ($out_digits: ident, $i: expr, $digit: expr, $is_negative: expr, $sign_bits: expr) => {
        if $i == Self::N_MINUS_1 {
            if ($digit as SignedDigit).is_negative() == $is_negative {
                $out_digits[$i] = $digit;
            } else {
                return None;
            }
        } else if $i < N {
            $out_digits[$i] = $digit;
        } else if $digit != $sign_bits {
            return None;
        };
    }
}

impl<const N: usize> BIint<N> {
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
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        if len == 0 {
            return Some(Self::ZERO);
        }
        let is_negative = (slice[0] as i8).is_negative();
        let sign_bits = if is_negative {
            Digit::MAX
        } else {
            Digit::MIN
        };
        let mut out_digits = if is_negative {
            [Digit::MAX; N]
        } else {
            [0; N]
        };
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(len - digit::BYTES - (i << digit::BYTE_SHIFT)).copy_to_nonoverlapping(ptr, digit::BYTES);
                uninit.assume_init()
            };
            let digit = Digit::from_be_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        let rem = len & (digit::BYTES - 1);
        if rem == 0 {
            Some(Self::from_digits(out_digits))
        } else {
            let mut last_digit_bytes = [0; digit::BYTES];
            let mut j = 0;
            while j < rem {
                last_digit_bytes[digit::BYTES - rem + j] = slice[j];
                j += 1;
            }
            let digit = Digit::from_be_bytes(last_digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            Some(Self::from_digits(out_digits))
        }
    }
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        if len == 0 {
            return Some(Self::ZERO);
        }
        let is_negative = (slice[len - 1] as i8).is_negative();
        let sign_bits = if is_negative {
            Digit::MAX
        } else {
            Digit::MIN
        };
        let mut out_digits = if is_negative {
            [Digit::MAX; N]
        } else {
            [0; N]
        };
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(i << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES);
                uninit.assume_init()
            };
            let digit = Digit::from_le_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        if len & (digit::BYTES - 1) == 0 {
            Some(Self::from_digits(out_digits))
        } else {
            let mut last_digit_bytes = [0; digit::BYTES];
            let addition = exact << digit::BYTE_SHIFT;
            let mut j = 0;
            while j + addition < len {
                last_digit_bytes[j] = slice[j + addition];
                j += 1;
            }
            let digit = Digit::from_le_bytes(last_digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            Some(Self::from_digits(out_digits))
        }
    }
    /*pub const fn from_ne_slice(slice: &[u8]) -> Option<Self> {
        let option = BUint::from_ne_slice(slice);
        match option {
            None => None,
            Some(uint) => Some(Self {
                uint,
            })
        }
    }*/
}

#[cfg(feature = "nightly")]
impl<const N: usize> BIint<N> {
    uint_method! {
        fn to_be_bytes(self) -> [u8; N * digit::BYTES],
        fn to_le_bytes(self) -> [u8; N * digit::BYTES],
        fn to_ne_bytes(self) -> [u8; N * digit::BYTES]
    }
    pub const fn from_be_bytes(bytes: [u8; N * digit::BYTES]) -> Self {
        Self {
            uint: BUint::from_be_bytes(bytes),
        }
    }
    pub const fn from_le_bytes(bytes: [u8; N * digit::BYTES]) -> Self {
        Self {
            uint: BUint::from_le_bytes(bytes),
        }
    }
    pub const fn from_ne_bytes(bytes: [u8; N * digit::BYTES]) -> Self {
        Self {
            uint: BUint::from_ne_bytes(bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::I128;

    test_signed! {
        name: from_be,
        method: {
            from_be(-34958734895739475893434758937397458i128);
        }
    }
    test_signed! {
        name: from_le,
        method: {
            from_le(34053485789374589734957349573987458934i128);
        }
    }
    test_signed! {
        name: to_be,
        method: {
            to_be(938495078934875384738495787358743854i128);
        }
    }
    test_signed! {
        name: to_le,
        method: {
            to_le(-634985790475394859374957339475897443i128);
        }
    }

    #[cfg(feature = "nightly")]
    fn converter(bytes: [u8; 16]) -> [u8; 16] {
        bytes
    }

    #[cfg(feature = "nightly")]
    test_signed! {
        name: to_be_bytes,
        method: {
            to_be_bytes(-94564564534345004567495879873945739i128);
        },
        converter: converter
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        name: to_le_bytes,
        method: {
            to_le_bytes(4589674598797290345798374895793745019i128);
        },
        converter: converter
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        name: to_ne_bytes,
        method: {
            to_ne_bytes(-9547689209348758902934752103969375839i128);
        },
        converter: converter
    }

    #[cfg(feature = "nightly")]
    test_signed! {
        name: from_be_bytes,
        method: {
            from_be_bytes([4, 60, 57, 100, 57, 44, 39, 43, 5, 0, 200, 240, 233, 54, 79, 33]);
            from_be_bytes([50, 40, 31, 80, 150, 167, 205, 132, 254, 1, 56, 89, 189, 124, 67, 176]);
        }
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        name: from_le_bytes,
        method: {
            from_le_bytes([44, 30, 26, 88, 123, 105, 119, 251, 226, 218, 243, 10, 18, 5, 0, 9]);
            from_le_bytes([80, 13, 87, 0, 0, 43, 29, 68, 95, 100, 167, 222, 21, 32, 49, 163]);
        }
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        name: from_ne_bytes,
        method: {
            from_ne_bytes([55, 49, 2, 24, 88, 120, 160, 253, 1, 0, 9, 59, 48, 195, 167, 86]);
            from_ne_bytes([67, 0, 80, 53, 185, 196, 205, 68, 226, 58, 91, 58, 194, 139, 45, 183]);
        }
    }
    // TODO: test from_be_slice and from_le_slice
}