use super::BUint;
use crate::ParseIntError;
use crate::digit::{self, Digit};

const BITS: u8 = digit::BITS as u8;

macro_rules! assert_range {
    ($radix: expr, $max: expr) => {
        assert!(2 <= $radix && $radix <= $max, "Radix must be in range [2, {}]", $max)
    }
}

fn last_set_bit(n: u32) -> u8 {
    ((std::mem::size_of_val(&n) as u8) << 3) - n.leading_zeros() as u8
}
fn ilog2(n: u32) -> u8 {
    last_set_bit(n) - 1
}

impl<const N: usize> BUint<N> {
    fn from_bitwise_digits_le(buf: &[u8], bits: u8) -> Option<Self> {
        let mut out = Self::ZERO;
        let digits_per_big_digit = BITS / bits;
        for (i, digit) in buf
            .chunks(digits_per_big_digit.into())
            .map(|chunk| {
                chunk
                    .iter()
                    .rev()
                    .fold(0, |acc, &c| {
                        (acc << bits) | c as Digit
                    })
            })
            .enumerate()
        {
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            }
        };
        Some(out)
    }
    /*fn to_bitwise_digits_le(&self, bits: u8) -> Vec<u8> {
        let mask: Digit = (1 << bits) - 1;
        let digits_per_big_digit = BITS / bits;
        let 
    }
    fn to_str_radix_reversed(&self, radix: u32) -> Vec<u8> {
        assert_range!(radix);
        if self.is_zero() {
            return vec![b'0'];
        }
        let mut res = self.to_radix_le(radix);
        for r in &mut res {
            if r < &mut 10 {
                *r += b'0';
            } else {
                *r += b'a' - 10;
            }
        }
        res
    }*/
    pub fn parse_bytes(buf: &[u8], radix: u32) -> Option<Self> {
        let s = std::str::from_utf8(buf).ok()?;
        Self::from_str_radix(s, radix).ok()
    }
    pub fn from_radix_be(buf: &[u8], radix: u32) -> Option<Self> {
        assert_range!(radix, 256);
        todo!()
    }
    pub fn from_radix_le(buf: &[u8], radix: u32) -> Option<Self> {
        assert_range!(radix, 256);
        todo!()
    }
    pub fn to_str_radix(&self, radix: u32) -> String {
        todo!()
    }
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        todo!()
    }
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        /*if self.is_zero() {
            vec![0]
        } else if radix.is_power_of_two() {
            let bits = ilog2(radix);
            if BITS % bits == 0 {
                self.to_bitwise_digits_le(bits)
            } else {
                self.to_inexact_bitwise_digits_le(bits)
            }
        } else if radix == 10 {
            self.to_radix_digits_le(10)
        } else {
            self.to_radix_digits_le(radix)
        }*/
        todo!()
    }
    fn from_str_bitwise_radix(src: &str, radix: u32, chunk_size: usize) -> Result<Self, ParseIntError> {
        let mut out = Self::ZERO;
        let mut index = 0;
        let mut i = src.len();
        while i > 0 {
            if index == N {
                return Err("too large");
            }
            let end = i;
            i = i.saturating_sub(chunk_size);
            let digit = match Digit::from_str_radix(&src[i..end], radix) {
                Ok(digit) => digit,
                Err(err) => return Err("invalid"),
            };
            out.digits[index] = digit;
            index += 1;
        }
        Ok(out)
    }
    fn from_inexact_bitwise_digits_le(buf: &[u8], radix: u8, bits: u8) -> Result<Self, ParseIntError> {
        let mut out = Self::ZERO;
        let mut digit = 0;
        let mut dbits = 0;
        let mut index = 0;
        for &byte in buf.iter().rev() {
            let byte = Self::byte_to_digit(byte, radix)?;
            digit |= Digit::from(byte) << dbits;
            dbits += bits;
            if dbits >= BITS {
                if index < N {
                    out.digits[index] = digit;
                    index += 1;
                    dbits -= BITS;
                    digit = Digit::from(byte) >> (bits - dbits);
                } else if digit != 0 {
                    return Err("overflow");
                }
            }
        }
        if dbits > 0 && digit != 0 {
            if index < N {
                out.digits[index] = digit;
            } else {
                return Err("overflow");
            }
        }
        Ok(out)
    }
    fn byte_to_digit(byte: u8, radix: u8) -> Result<u8, ParseIntError> {
        let digit = match byte {
            b'0' ..= b'9' => byte - b'0',
            b'a' ..= b'z' => byte - b'a' + 10,
            b'A' ..= b'Z' => byte - b'A' + 10,
            _ => u8::MAX,
        };
        if digit < radix {
            Ok(digit)
        } else {
            Err("invalid digit")
        }
    }
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
        assert_range!(radix, 36);
        let mut src = src;
        if src.starts_with('+') {
            let tail = &src[1..];
            if tail.starts_with('+') {
                return Err("invalid digit");
            } else {
                src = tail;
            }
        }
        if src.is_empty() {
            return Err("empty string");
        }
        if src.starts_with('_') {
            return Err("invalid digit");
        }
        match radix {
            2 => Self::from_str_bitwise_radix(src, radix, digit::BITS),
            4 => Self::from_str_bitwise_radix(src, radix, digit::BITS >> 1),
            16 => Self::from_str_bitwise_radix(src, radix, digit::BITS >> 2),
            8 => Self::from_inexact_bitwise_digits_le(src.as_bytes(), 8, 3),
            32 => Self::from_inexact_bitwise_digits_le(src.as_bytes(), 32, 4),
            radix => {
                todo!()
            }
        }
        /*let mut v = Vec::with_capacity(src.len());
        for byte in src.bytes() {
            let digit = match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'z' => byte - b'a' + 10,
                b'A'..=b'Z' => byte - b'A' + 10,
                b'_' => continue,
            }
        }*/
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn test_parse() {
        let s = "3777777777777777777777777777777777";
        assert_eq!(U128::from_str_bitwise_radix("ffffffffffff34534fffffff3454", 16, 16).unwrap(), u128::from_str_radix("ffffffffffff34534fffffff3454", 16).unwrap().into());

        let bytes = s.as_bytes();
        assert_eq!(U128::from_inexact_bitwise_digits_le(bytes, 8, 3).unwrap(), u128::from_str_radix(s, 8).unwrap().into());
    }
}