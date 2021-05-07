use super::BUint;

impl<const N: usize> BUint<N> {
    pub const fn from_be(x: Self) -> Self {
        if cfg!(target_endian = "big") {
            x
        } else {
            x.swap_bytes()
        }
    }
    pub const fn from_le(x: Self) -> Self {
        if cfg!(target_endian = "little") {
            x
        } else {
            x.swap_bytes()
        }
    }
    pub const fn to_be(self) -> Self {
        if cfg!(target_endian = "big") {
            self
        } else {
            self.swap_bytes()
        }
    }
    pub const fn to_le(self) -> Self {
        if cfg!(target_endian = "little") {
            self
        } else {
            self.swap_bytes()
        }
    }
    /*pub const fn to_be_bytes(self) -> [u8; N * 8] {
        let mut bytes = [0; N * 8];
        let mut i = 0;
        while i < N {
            let digit_bytes = self.digits[i].to_be_bytes();
            let mut j = 0;
            while j < 64 >> 3 {
                bytes[((N - i - 1) << 3) + j] = digit_bytes[j];
                j += 1;
            }
            i += 1;
        }
        bytes
    }
    pub const fn to_le_bytes(self) -> [u8; N * 8] {
        let mut bytes = [0; N * 8];
        let mut i = 0;
        while i < N {
            let digit_bytes = self.digits[i].to_le_bytes();
            let mut j = 0;
            while j < 64 >> 3 {
                bytes[(i << 3) + j] = digit_bytes[j];
                j += 1;
            }
            i += 1;
        }
        bytes
    }
    pub const fn to_ne_bytes(self) -> [u8; N * 8] {
        if cfg!(target_endian = "big") {
            self.to_be_bytes()
        } else {
            self.to_le_bytes()
        }
    }
    pub const fn from_be_bytes(bytes: [u8; N * 8]) -> Self {
        let mut int = Self::MIN;
        let mut i = 0;
        while i < N {
            let digit_bytes = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3], bytes[i + 4], bytes[i + 5], bytes[i + 6], bytes[i + 7]];
            int.digits[i] = u64::from_le_bytes(digit_bytes);
            i += 8;
        }
        int
    }
    pub const fn from_le_bytes(bytes: [u8; N * 8]) -> Self {
        let mut int = Self::MIN;
        let mut i = 0;
        while i < N {
            {
                let i = N - 1 - i;
                let digit_bytes = [bytes[i - 7], bytes[i - 6], bytes[i - 5], bytes[i - 4], bytes[i - 3], bytes[i - 2], bytes[i - 1], bytes[i]];
                int.digits[i] = u64::from_be_bytes(digit_bytes);
            }
            i += 8;
        }
        int
    }
    pub const fn from_ne_bytes(bytes: [u8; N * 8]) -> Self {
        if cfg!(target_endian = "big") {
            Self::from_be_bytes(bytes)
        } else {
            Self::from_le_bytes(bytes)
        }
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    test_with_u128!((convert)234889034774398590845348573498570345u128, test_from_be, from_be);
    test_with_u128!((convert)374598340857345349875907438579348534u128, test_from_le, from_le);
    test_with_u128!((convert)938495078934875384738495787358743854u128, test_to_be, to_be);
    test_with_u128!((convert)634985790475394859374957339475897443u128, test_to_le, to_le);
    //test_with_u128!(883497884590834905834758374950859884u128, test_to_be_bytes, to_be_bytes);
    //test_with_u128!(349587309485908349057389485093457397u128, test_to_le_bytes, to_le_bytes);
    //test_with_u128!(123423345734905803845939847534085908u128, test_to_ne_bytes, to_ne_bytes);
    #[test]
    fn test_from_be_bytes() {
        let a = 23547843905834589345903845984384598u128;
        let buint = BUint::<2>::from(a);
        // Current causes compiler crash due to instability of const generics
        //assert_eq!(buint, BUint::<2>::from_be_bytes(buint.to_be_bytes()));
        //assert_eq!(BUint::<2>::from_be_bytes([4; 16]), u128::from_be_bytes([4; 16]).into());
    }
    #[test]
    fn test_from_le_bytes() {
        let a = 34590834563845890504549985948845098454u128;
        let buint = BUint::<2>::from(a);
        // Current causes compiler crash due to instability of const generics
        //assert_eq!(buint, BUint::<2>::from_le_bytes(buint.to_le_bytes()));
        //assert_eq!(BUint::<2>::from_le_bytes([4; 16]), u128::from_le_bytes([4; 16]).into());
    }
    #[test]
    fn test_from_ne_bytes() {
        let a = 9876757883495934598734924753734758883u128;
        let buint = BUint::<2>::from(a);
        // Current causes compiler crash due to instability of const generics
        //assert_eq!(buint, BUint::<2>::from_ne_bytes(buint.to_ne_bytes()));
        //assert_eq!(BUint::<2>::from_ne_bytes([4; 16]), u128::from_ne_bytes([4; 16]).into());
    }
}