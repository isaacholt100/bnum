use super::Bint;
use crate::uint::BUint;
use alloc::string::String;
use alloc::vec::Vec;
use crate::error::{ParseIntError, ParseIntErrorReason::*};

macro_rules! assert_range {
    ($radix: expr, $max: expr) => {
        assert!(2 <= $radix && $radix <= $max, "Radix must be in range [2, {}]", $max)
    }
}

impl<const N: usize> Bint<N> {
    pub fn parse_bytes(buf: &[u8], radix: u32) -> Option<Self> {
        let s = core::str::from_utf8(buf).ok()?;
        Self::from_str_radix(s, radix).ok()
    }
    pub fn from_radix_be(buf: &[u8], radix: u32) -> Option<Self> {
        match BUint::from_radix_be(buf, radix) {
            None => None,
            Some(uint) => Some(Self { uint }),
        }
    }
    pub fn from_radix_le(buf: &[u8], radix: u32) -> Option<Self> {
        assert_range!(radix, 256);
        match BUint::from_radix_le(buf, radix) {
            None => None,
            Some(uint) => Some(Self { uint }),
        }
    }
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
        assert_range!(radix, 36);
        let mut src = src;
        let mut negative = false;
        if src.starts_with('-') {
            src = &src[1..];
            negative = true;
            if src.starts_with('+') {
                return Err(ParseIntError {
                    reason: InvalidDigit,
                });
            }
        }
        let uint = BUint::from_str_radix(src, radix)?;
        if uint.bit(Self::BITS - 1) {
            Err(ParseIntError {
                reason: TooLarge,
            })
        } else {
            if negative {
                let abs_value = Self {
                    uint,
                };
                Ok(abs_value.neg())
            } else {
                Ok(Self {
                    uint,
                })
            }
        }
    }
    pub fn to_str_radix(&self, radix: u32) -> String {
        if self.is_negative() {
            format!("-{}", self.unsigned_abs().to_str_radix(radix))
        } else {
            self.uint.to_str_radix(radix)
        }
    }
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        self.uint.to_radix_be(radix)
    }
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        self.uint.to_radix_le(radix)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bint, I128};
    use crate::test;

    test::test_big_num! {
        big: I128,
        primitive: i128,
        function: from_str_radix,
        method: {
            from_str_radix("-14359abcasdhfkdgdfgsde", 34u32);
            from_str_radix("23797984569ahgkhhjdskjdfiu", 32u32);
            from_str_radix("-253613132341435345", 7u32);
            from_str_radix("23467abcad47790809ef37", 16u32);
            from_str_radix("-712930769245766867875986646", 10u32);
        },
        converter: |result: Result<i128, core::num::ParseIntError>| -> Result<I128, crate::ParseIntError> {
            Ok(result.unwrap().into())
        }
    }
    #[test]
    fn from_to_radix_le() {
        let buf = &[61, 45, 48, 20, 37, 59, 53, 28, 28, 52, 54, 13, 44, 3, 46, 42, 20, 46, 37, 32, 13, 27, 47, 30, 33, 25, 3, 32, 4, 54, 53, 6, 44, 25, 10, 22, 33, 48, 7, 17];
        let u = Bint::<100>::from_radix_le(buf, 64).unwrap();
        let v = u.to_radix_le(64);
        assert_eq!(v, buf);

        let buf = &[33, 34, 61, 53, 74, 67, 54, 62, 22, 29, 4, 2, 43, 73, 74, 24, 8, 74, 65, 3, 78];
        let option = Bint::<100>::from_radix_le(buf, 78);
        assert!(option.is_none());

        let buf = &[1, 3, 3, 0, 2, 1, 2, 3, 0, 4, 1, 2, 0, 0, 0, 0, 3, 2, 0, 1, 0, 4, 1, 3, 1, 4, 3, 3, 3, 4, 1, 2, 2, 1, 3, 0, 2, 1, 2, 3, 1, 1, 0, 2, 2, 1, 1, 2, 1, 0, 0, 0, 3, 3, 3, 0, 0, 4, 4, 2];
        let u = Bint::<100>::from_radix_le(buf, 5).unwrap();
        let v = u.to_radix_le(5);
        assert_eq!(v, buf);
    }
    #[test]
    fn from_to_radix_be() {
        let buf = &[29, 89, 92, 118, 69, 140, 141, 70, 71, 76, 66, 13, 30, 28, 38, 145, 40, 7, 57, 18, 25, 65, 150, 119, 155, 18, 64, 76, 106, 87];
        let u = Bint::<100>::from_radix_be(buf, 157).unwrap();
        let v = u.to_radix_be(157);
        assert_eq!(v, buf);

        let buf = &[1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1];
        let u = Bint::<100>::from_radix_be(buf, 2).unwrap();
        let v = u.to_radix_be(2);
        assert_eq!(v, buf);

        let buf = &[91, 167, 5, 99, 61, 38, 158, 149, 115, 79, 13, 118, 53, 16, 144, 123, 70, 81, 78, 61, 39, 6, 34, 95, 98, 23, 175, 182];
        let option = Bint::<100>::from_radix_le(buf, 180);
        assert!(option.is_none());

        let buf = &[39, 90, 119, 93, 95, 7, 70, 81, 3, 100, 39, 107, 98, 31, 61, 5, 36, 19, 18, 124, 4, 77, 119, 17, 121, 116, 24, 35];
        let u = Bint::<100>::from_radix_be(buf, 128).unwrap();
        let v = u.to_radix_be(128);
        assert_eq!(v, buf);
    }
    #[test]
    fn from_to_str_radix() {
        let src = "-293487598aashkhkhakb8345cbvjkus";
        let u = Bint::<100>::from_str_radix(src, 35).unwrap();
        let v = u.to_str_radix(35);
        assert_eq!(v, src);

        let src = "zzzzzzzzzzzzzzzzzzzzzzzzz";
        let result = Bint::<1>::from_str_radix(src, 36);
        assert!(result.is_err());

        let invalid = "inval_id string";
        let result = Bint::<1>::from_str_radix(invalid, 36);
        assert!(result.is_err());

        let src = "72954hslfhbui79845y6audfgiu984h5ihhhdfg";
        let u = Bint::<100>::from_str_radix(src, 36).unwrap();
        assert_eq!(u.to_str_radix(36), src);
    }
    #[test]
    fn parse_bytes() {
        let src = "1797972456987acbdead7889";
        let u = Bint::<100>::parse_bytes(src.as_bytes(), 16).unwrap();
        let v = Bint::<100>::from_str_radix(src, 16).unwrap();
        assert_eq!(u, v);
        assert_eq!(v.to_str_radix(16), src);

        let bytes = b"279874657dgfhjh";
        let option = Bint::<100>::parse_bytes(bytes, 11);
        assert!(option.is_none());
    }
}