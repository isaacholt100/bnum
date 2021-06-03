use super::BintTest;
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
    uint_method! {
        fn to_be_bytes(self) -> [u8; N * 8],
        fn to_le_bytes(self) -> [u8; N * 8],
        fn to_ne_bytes(self) -> [u8; N * 8]
    }
    pub const fn from_be_bytes(bytes: [u8; N * 8]) -> Self {
        Self {
            uint: BUint::from_be_bytes(bytes),
        }
    }
    pub const fn from_le_bytes(bytes: [u8; N * 8]) -> Self {
        Self {
            uint: BUint::from_le_bytes(bytes),
        }
    }
    pub const fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        Self {
            uint: BUint::from_ne_bytes(bytes),
        }
    }
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let option = BUint::from_be_slice(slice);
        match option {
            None => None,
            Some(uint) => Some(Self {
                uint,
            })
        }
    }
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        let option = BUint::from_le_slice(slice);
        match option {
            None => None,
            Some(uint) => Some(Self {
                uint,
            })
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

#[cfg(test)]
mod tests {
    use crate::I128Test;

    test_signed! {
        test_name: test_from_be,
        method: {
            from_be(-34958734895739475893434758937397458i128);
        }
    }
    test_signed! {
        test_name: test_from_le,
        method: {
            from_le(34053485789374589734957349573987458934i128);
        }
    }
    test_signed! {
        test_name: test_to_be,
        method: {
            to_be(938495078934875384738495787358743854i128);
        }
    }
    test_signed! {
        test_name: test_to_le,
        method: {
            to_le(-634985790475394859374957339475897443i128);
        }
    }

    fn converter(bytes: [u8; 16]) -> [u8; 16] {
        bytes
    }

    test_signed! {
        test_name: test_to_be_bytes,
        method: {
            to_be_bytes(-94564564534345004567495879873945739i128);
        },
        converter: converter
    }
    test_signed! {
        test_name: test_to_le_bytes,
        method: {
            to_le_bytes(4589674598797290345798374895793745019i128);
        },
        converter: converter
    }
    test_signed! {
        test_name: test_to_ne_bytes,
        method: {
            to_ne_bytes(-9547689209348758902934752103969375839i128);
        },
        converter: converter
    }
}