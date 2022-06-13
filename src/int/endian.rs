use super::Bint;
use crate::uint::BUint;
use crate::digit::{self, Digit, SignedDigit};
use core::mem::MaybeUninit;
use crate::doc;

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

#[doc=doc::endian::impl_desc!(Bint)]
impl<const N: usize> Bint<N> {
    #[doc=doc::from_be!(Bint::<2>)]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
        Self::from_bits(BUint::from_be(x.bits))
    }

    #[doc=doc::from_le!(Bint::<2>)]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
        Self::from_bits(BUint::from_le(x.bits))
    }

    #[doc=doc::to_be!(Bint::<2>)]
    #[inline]
    pub const fn to_be(self) -> Self {
        Self::from_be(self)
    }

    #[doc=doc::to_le!(Bint::<2>)]
    #[inline]
    pub const fn to_le(self) -> Self {
        Self::from_le(self)
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
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(len - digit::BYTES as usize - (i << digit::BYTE_SHIFT)).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
                uninit.assume_init()
            };
            let digit = Digit::from_be_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        let rem = len & (digit::BYTES as usize - 1);
        if rem == 0 {
            Some(Self::from_digits(out_digits))
        } else {
            let mut last_digit_bytes = [0; digit::BYTES as usize];
            let mut j = 0;
            while j < rem {
                last_digit_bytes[digit::BYTES as usize - rem + j] = slice[j];
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
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(i << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
                uninit.assume_init()
            };
            let digit = Digit::from_le_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        if len & (digit::BYTES as usize - 1) == 0 {
            Some(Self::from_digits(out_digits))
        } else {
            let mut last_digit_bytes = [0; digit::BYTES as usize];
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
	
    #[doc=doc::to_be_bytes!(Bint::<2>, "i")]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N * digit::BYTES as usize] {
        self.bits.to_be_bytes()
    }

    #[doc=doc::to_le_bytes!(Bint::<2>, "i")]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N * digit::BYTES as usize] {
        self.bits.to_le_bytes()
    }

    #[doc=doc::to_ne_bytes!(Bint::<2>, "i")]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N * digit::BYTES as usize] {
        self.bits.to_ne_bytes()
    }

    #[doc=doc::from_be_bytes!(Bint::<2>, "i")]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_be_bytes(bytes))
    }

    #[doc=doc::from_le_bytes!(Bint::<2>, "i")]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_le_bytes(bytes))
    }

    #[doc=doc::from_ne_bytes!(Bint::<2>, "i")]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_ne_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::U8ArrayWrapper;

    test_signed! {
        function: from_be(a: i128),
        cases: [
            (-34958734895739475893434758937397458i128)
        ]
    }
    test_signed! {
        function: from_le(a: i128),
        cases: [
            (34053485789374589734957349573987458934i128)
        ]
    }
    test_signed! {
        function: to_be(a: i128),
        cases: [
            (938495078934875384738495787358743854i128)
        ]
    }
    test_signed! {
        function: to_le(a: i128),
        cases: [
            (-634985790475394859374957339475897443i128)
        ]
    }

    #[cfg(feature = "nightly")]
    test_signed! {
        function: to_be_bytes(a: i128),
        cases: [
            (-94564564534345004567495879873945739i128)
        ]
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        function: to_le_bytes(a: i128),
        cases: [
            (4589674598797290345798374895793745019i128)
        ]
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        function: to_ne_bytes(a: i128),
        cases: [
            (-9547689209348758902934752103969375839i128)
        ]
    }

    #[cfg(feature = "nightly")]
    test_signed! {
        function: from_be_bytes(a: U8ArrayWrapper<16>),
        cases: [
            ([4, 60, 57, 100, 57, 44, 39, 43, 5, 0, 200, 240, 233, 54, 79, 33]),
            ([50, 40, 31, 80, 150, 167, 205, 132, 254, 1, 56, 89, 189, 124, 67, 176])
        ]
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        function: from_le_bytes(a: U8ArrayWrapper<16>),
        cases: [
            ([44, 30, 26, 88, 123, 105, 119, 251, 226, 218, 243, 10, 18, 5, 0, 9]),
            ([80, 13, 87, 0, 0, 43, 29, 68, 95, 100, 167, 222, 21, 32, 49, 163])
        ]
    }
    #[cfg(feature = "nightly")]
    test_signed! {
        function: from_ne_bytes(a: U8ArrayWrapper<16>),
        cases: [
            ([55, 49, 2, 24, 88, 120, 160, 253, 1, 0, 9, 59, 48, 195, 167, 86]),
            ([67, 0, 80, 53, 185, 196, 205, 68, 226, 58, 91, 58, 194, 139, 45, 183])
        ]
    }
    // TODO: test from_be_slice and from_le_slice
}