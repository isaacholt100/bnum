use super::BintTest;
use crate::digit;
use crate::uint::BUint;
use alloc::string::String;
use alloc::vec::Vec;
use crate::error::{ParseIntError, ParseIntErrorReason::*};

const BITS: u8 = digit::BITS as u8;

macro_rules! assert_range {
    ($radix: expr, $max: expr) => {
        assert!(2 <= $radix && $radix <= $max, "Radix must be in range [2, {}]", $max)
    }
}

impl<const N: usize> BintTest<N> {
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
    // Might change so it outputs a '-' sign if necessary instead of two's complement representation
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
    use crate::I128Test;

    test_signed! {
        test_name: test_from_str_radix,
        method: {
            from_str_radix("-3459dsdhtert98345", 31u32);
        },
        converter: |result: Result<i128, core::num::ParseIntError>| -> Result<I128Test, crate::ParseIntError> {
            Ok(result.unwrap().into())
        }
    }
}