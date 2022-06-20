use super::BUint;
use crate::doc;
use crate::ExpType;
use crate::errors::ParseIntError;
use crate::digit::{self, Digit, DoubleDigit};
use core::iter::Iterator;
use alloc::string::String;
use alloc::vec::Vec;
use crate::radix_bases;
use core::num::IntErrorKind;
use crate::int::radix::assert_range;

const BITS_U8: u8 = digit::BITS as u8;

#[doc=doc::radix::impl_desc!(BUint)]
impl<const N: usize> BUint<N> {
    fn from_bitwise_digits_le<InnerIter, OuterIter>(iter: OuterIter, bits: u8) -> Option<Self>
    where
        InnerIter: Iterator<Item = u8>,
        OuterIter: Iterator<Item = InnerIter>,
    {
		// credit num_bigint source code
        let mut out = Self::ZERO;

        let iter = iter.map(|inner_iter| {
            inner_iter.fold(0, |acc, c| {
                (acc << bits) | c as Digit
            })
        });
        for (i, digit) in iter.enumerate() {
            if i < N {
                out.digits[i] = digit;
            } else if digit != 0 {
                return None;
            }
        }
        Some(out)
    }
    fn from_inexact_bitwise_digits_le<I>(iter: I, bits: u8) -> Option<Self>
    where
        I: Iterator<Item = u8>
    {
		// credit num_bigint source code
        let mut out = Self::ZERO;
        let mut digit = 0;
        let mut dbits = 0;
        let mut index = 0;

        for byte in iter {
            digit |= (byte as Digit) << dbits;
            dbits += bits;
            if dbits >= BITS_U8 {
                if index < N {
                    out.digits[index] = digit;
                    index += 1;
                    dbits -= BITS_U8;
                    digit = (byte as Digit) >> (bits - dbits);
                } else if digit != 0 {
                    return None;
                }
            }
        }
        if dbits > 0 && digit != 0 {
            if index < N {
                out.digits[index] = digit;
            } else {
                return None;
            }
        }
        Some(out)
    }
    const fn mac_with_carry(a: Digit, b: Digit, acc: &mut DoubleDigit) -> Digit {
		// credit num_bigint source code
        *acc += a as DoubleDigit * b as DoubleDigit;
        let lo = *acc as Digit;
        *acc >>= BITS_U8;
        lo
    }
    fn from_radix_digits_be<Head, TailInner, Tail>(head: Head, tail: Tail, radix: u32, base: Digit) -> Option<Self>
    where
        Head: Iterator<Item = u8>,
        TailInner: Iterator<Item = u8>,
        Tail: Iterator<Item = TailInner>,
    {
		// credit num_bigint source code
        let mut out = Self::ZERO;

        let radix = radix as Digit;
        let first = head.fold(0, |acc, d| {
            acc * radix + d as Digit
        });
        out.digits[0] = first;

        for chunk_iter in tail {
            let mut carry = 0;
            for digit in out.digits.iter_mut() {
                *digit = Self::mac_with_carry(*digit, base, &mut carry);
            }
            if carry != 0 {
                return None;
            }
            let n = chunk_iter.fold(0, |acc, d| {
                acc * radix + d as Digit
            });
            out = out.checked_add(n.into())?;
        }
        Some(out)
    }

    /// Converts a byte slice in a given base to an integer. The input slice must contain ascii/utf8 characters in [0-9a-zA-Z].
    /// 
    /// This function is equivalent to the `from_str_radix` function for a string slice equivalent to the byte slice and the same radix.
    /// 
    /// Returns `None` if the conversion of the byte slice to string slice fails or if a digit is larger than the given radix, otherwise the integer is wrapped in `Some`.
    /// 
    /// # Panics
    /// 
    /// This function panics if `radix` is not in the range from 2 to 36.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::BUint;
    /// 
    /// let src = "394857hdgfjhsnkg947dgfjkeita";
    /// assert_eq!(BUint::<4>::from_str_radix(src, 32).ok(), BUint::parse_bytes(src.as_bytes(), 32));
    /// ```
    pub fn parse_bytes(buf: &[u8], radix: u32) -> Option<Self> {
		// credit num_bigint source code
        let s = core::str::from_utf8(buf).ok()?;
        Self::from_str_radix(s, radix).ok()
    }
    
    /// Converts a slice of big-endian digits in the given radix to an integer. Each `u8` of the slice is interpreted as one digit of the number, so this function will return `None` if any digit is greater than or equal to `radix`, otherwise the integer is wrapped in `Some`.
    /// 
    /// # Panics
    /// 
    /// This function panics if `radix` is not in the range from 2 to 256.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::BUint;
    /// 
    /// let n = BUint::<2>::from(34598748526857897975u128);
    /// let digits = n.to_radix_be(12);
    /// assert_eq!(Some(n), BUint::from_radix_be(&digits, 12));
    /// ```
    pub fn from_radix_be(buf: &[u8], radix: u32) -> Option<Self> {
		// credit num_bigint source code
        assert_range!(radix, 256);
        if buf.is_empty() {
            return Some(Self::ZERO);
        }
        #[cfg(feature = "u8_digit")]
        if radix == 256 {
            return Self::from_be_slice(buf);
        }
        
        if radix != 256 && buf.iter().any(|&b| b >= radix as u8) {
            return None;
        }
        if radix.is_power_of_two() {
            let bits = ilog2(radix);
            if BITS_U8 % bits == 0 {
                let iter = buf
                    .rchunks((BITS_U8 / bits) as usize)
                    //.rev()
                    .map(|chunk| {
                        chunk
                            .iter()
                            .copied()
                    });
                Self::from_bitwise_digits_le(iter, bits)
            } else {
                Self::from_inexact_bitwise_digits_le(buf.iter().rev().copied(), bits)
            }
        } else {
            let (base, power) = radix_bases::get_radix_base::<false>(radix);
            let r = buf.len() % power;
            let i = if r == 0 {
                power
            } else {
                r
            };
            let (head, tail) = buf.split_at(i);
            let head = head.iter().copied();
            let tail = tail
                .chunks(power)
                .map(|chunk| {
                    chunk
                        .iter()
                        .copied()
                });
            Self::from_radix_digits_be(head, tail, radix, base)
        }
    }
    
    /// Converts a slice of little-endian digits in the given radix to an integer. Each `u8` of the slice is interpreted as one digit of the number, so this function will return `None` if any digit is greater than or equal to `radix`, otherwise the integer is wrapped in `Some`.
    /// 
    /// # Panics
    /// 
    /// This function panics if `radix` is not in the range from 2 to 256.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::BUint;
    /// 
    /// let n = BUint::<2>::from(109837459878951038945798u128);
    /// let digits = n.to_radix_le(15);
    /// assert_eq!(Some(n), BUint::from_radix_le(&digits, 15));
    /// ```
    pub fn from_radix_le(buf: &[u8], radix: u32) -> Option<Self> {
		// credit num_bigint source code
        assert_range!(radix, 256);
        if buf.is_empty() {
            return Some(Self::ZERO);
        }
        #[cfg(feature = "u8_digit")]
        if radix == 256 {
            return Self::from_le_slice(buf);
        }

        if radix != 256 && buf.iter().any(|&b| b >= radix as u8) {
            return None;
        }
        let out = if radix.is_power_of_two() {
            let bits = ilog2(radix);
            if BITS_U8 % bits == 0 {
                let iter = buf
                    .chunks((BITS_U8 / bits) as usize)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .rev()
                            .copied()
                    });
                Self::from_bitwise_digits_le(iter, bits)
            } else {
                Self::from_inexact_bitwise_digits_le(buf.iter().copied(), bits)
            }
        } else {
            let (base, power) = radix_bases::get_radix_base::<false>(radix);
            let r = buf.len() % power;
            let i = if r == 0 {
                power
            } else {
                r
            };
            let (tail, head) = buf.split_at(buf.len() - i);
            let head = head.iter().rev().copied();
            let tail = tail
                .rchunks(power)
                .map(|chunk| {
                    chunk
                        .iter()
                        .rev()
                        .copied()
                });
            Self::from_radix_digits_be(head, tail, radix, base)
        };
        out
    }
    const fn byte_to_digit(byte: u8) -> u8 {
		// credit num_bigint source code
        match byte {
            b'0' ..= b'9' => byte - b'0',
            b'a' ..= b'z' => byte - b'a' + 10,
            b'A' ..= b'Z' => byte - b'A' + 10,
            _ => u8::MAX,
        }
    }
    /// Converts a string slice in a given base to an integer.
    /// 
    /// The string is expected to be an optional `+` sign followed by digits. Leading and trailing whitespace represent an error. Digits are a subset of these characters, depending on `radix`:
    /// 
    /// - `0-9`
    /// - `a-z`
    /// - `A-Z`
    /// 
    /// # Panics
    /// 
    /// This function panics if `radix` is not in the range from 2 to 36.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::BUint;
    /// 
    /// assert_eq!(BUint::<2>::from_str_radix("A", 16), Ok(BUint::from(10u128)));
    /// ```
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
		// credit num_bigint source code
        assert_range!(radix, 36);
        let mut src = src;
        if src.starts_with('+') {
            src = &src[1..];
        }
        if src.is_empty() {
            return Err(ParseIntError {
                kind: IntErrorKind::Empty,
            });
        }
        let buf = src.as_bytes();
        let validate_src = || -> Result<&[u8], ParseIntError> {
            let radix = radix as u8;
            for &byte in buf {
                if Self::byte_to_digit(byte) >= radix {
                    return Err(ParseIntError {
                        kind: IntErrorKind::InvalidDigit,
                    });
                }
            }
            Ok(buf)
        };
        match radix {
            2 | 4 | 16 => {
                let buf = validate_src()?;
                let bits = ilog2(radix);
                let iter = buf
                    .rchunks((BITS_U8 / bits) as usize)
                    //.rev()
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|byte| Self::byte_to_digit(*byte))
                    });
                Self::from_bitwise_digits_le(iter, bits).ok_or(ParseIntError {
                    kind: IntErrorKind::PosOverflow,
                })
            },
            8 | 32 => {
                let bits = ilog2(radix);
                let buf = validate_src()?;
                let iter = buf
                    .iter()
                    .rev()
                    .map(|byte| Self::byte_to_digit(*byte));
                Self::from_inexact_bitwise_digits_le(iter, bits).ok_or(ParseIntError {
                    kind: IntErrorKind::PosOverflow,
                })
            },
            radix => {
                let (base, power) = radix_bases::get_radix_base::<false>(radix);
                let buf = validate_src()?;
                let r = buf.len() % power;
                let i = if r == 0 {
                    power
                } else {
                    r
                };
                let (head, tail) = buf.split_at(i);
                let head = head
                    .iter()
                    .map(|byte| Self::byte_to_digit(*byte));
                let tail = tail
                    .chunks(power)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|byte| Self::byte_to_digit(*byte))
                    });
                Self::from_radix_digits_be(head, tail, radix as u32, base).ok_or(ParseIntError {
                    kind: IntErrorKind::PosOverflow,
                })
            }
        }
    }

    /// Returns the integer as a string in the given radix.
    /// 
    /// # Panics
    /// 
    /// This function panics if `radix` is not in the range from 2 to 36.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::BUint;
    /// 
    /// let src = "934857djkfghhkdfgbf9345hdfkh";
    /// let n = BUint::<4>::from_str_radix(src, 36).unwrap();
    /// assert_eq!(n.to_str_radix(36), src);
    /// ```
    pub fn to_str_radix(&self, radix: u32) -> String {
		// credit num_bigint source code
        let mut out = Self::to_radix_be(self, radix);

        for byte in out.iter_mut() {
            if *byte < 10 {
                *byte += b'0';
            } else {
                *byte += b'a' - 10;
            }
        }
        unsafe {
            String::from_utf8_unchecked(out)
        }
    }

    /// Returns the integer in the given base in big-endian digit order.
    /// 
    /// # Panics
    /// 
    /// This function panics if `radix` is not in the range from 2 to 256.
    /// 
    /// ```
    /// use bnum::BUint;
    /// 
    /// let digits = &[3, 55, 60, 100, 5, 0, 5, 88];
    /// let n = BUint::<4>::from_radix_be(digits, 120).unwrap();
    /// assert_eq!(n.to_radix_be(120), digits);
    /// ```
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
		// credit num_bigint source code
        let mut v = self.to_radix_le(radix);
        v.reverse();
        v
    }

    /// Returns the integer in the given base in little-endian digit order.
    /// 
    /// # Panics
    /// 
    /// This function panics if `radix` is not in the range from 2 to 256.
    /// 
    /// ```
    /// use bnum::BUint;
    /// 
    /// let digits = &[1, 67, 88, 200, 55, 68, 87, 120, 178];
    /// let n = BUint::<4>::from_radix_le(digits, 250).unwrap();
    /// assert_eq!(n.to_radix_le(250), digits);
    /// ```
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
		// credit num_bigint source code
        if self.is_zero() {
            vec![0]
        } else if radix.is_power_of_two() {
            #[cfg(feature = "u8_digit")]
            if radix == 256 {
                return (&self.digits[0..=self.last_digit_index()]).to_vec();
            }
        
            let bits = ilog2(radix);
            if BITS_U8 % bits == 0 {
                self.to_bitwise_digits_le(bits)
            } else {
                self.to_inexact_bitwise_digits_le(bits)
            }
        } else if radix == 10 {
            self.to_radix_digits_le(10)
        } else {
            self.to_radix_digits_le(radix)
        }
    }

    fn to_bitwise_digits_le(self, bits: u8) -> Vec<u8> {
		// credit num_bigint source code
        let last_digit_index = self.last_digit_index();
        let mask: Digit = (1 << bits) - 1;
        let digits_per_big_digit = BITS_U8 / bits;
        let digits = div_ceil(self.bits(), bits as ExpType);
        let mut out = Vec::with_capacity(digits as usize);

        let mut r = self.digits[last_digit_index];

        for mut d in IntoIterator::into_iter(self.digits).take(last_digit_index) {
            for _ in 0..digits_per_big_digit {
                out.push((d & mask) as u8);
                d >>= bits;
            }
        }
        while r != 0 {
            out.push((r & mask) as u8);
            r >>= bits;
        }
        out
    }

    fn to_inexact_bitwise_digits_le(self, bits: u8) -> Vec<u8> {
		// credit num_bigint source code
        let mask: Digit = (1 << bits) - 1;
        let digits = div_ceil(self.bits(), bits as ExpType);
        let mut out = Vec::with_capacity(digits as usize);
        let mut r = 0;
        let mut rbits = 0;
        for c in self.digits {
            r |= c << rbits;
            rbits += BITS_U8;

            while rbits >= bits {
                out.push((r & mask) as u8);
                r >>= bits;

                if rbits > BITS_U8 {
                    r = c >> (BITS_U8 - (rbits - bits));
                }
                rbits -= bits;
            }
        }
        if rbits != 0 {
            out.push(r as u8);
        }
        while let Some(&0) = out.last() {
            out.pop();
        }
        out
    }

    fn to_radix_digits_le(self, radix: u32) -> Vec<u8> {
		// credit num_bigint source code
        let radix_digits = div_ceil(self.bits(), ilog2(radix) as ExpType);
        let mut out = Vec::with_capacity(radix_digits as usize);
        let (base, power) = radix_bases::get_radix_base::<true>(radix);
        let radix = radix as Digit;
        let mut copy = self;
        while copy.last_digit_index() > 0 {
            let (q, mut r) = copy.div_rem_digit(base);
            for _ in 0..power {
                out.push((r % radix) as u8);
                r /= radix;
            }
            copy = q;
        }
        let mut r = copy.digits[0];
        while r != 0 {
            out.push((r % radix) as u8);
            r /= radix;
        }
        out
    }
}

#[inline]
const fn ilog2(a: u32) -> u8 {
	31 - a.leading_zeros() as u8
}

#[inline]
const fn div_ceil(a: ExpType, b: ExpType) -> ExpType {
	if a % b == 0 {
		a / b
	} else {
		(a / b) + 1
	}
}

use core::str::FromStr;

impl<const N: usize> const FromStr for BUint<N> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
		// credit num_bigint source code
        let (base, power) = radix_bases::RADIX_BASE_10;
        let buf = src.as_bytes();
        let mut i = 0;
        while i < buf.len() {
            if Self::byte_to_digit(buf[i]) >= 10 {
                return Err(ParseIntError {
                    kind: IntErrorKind::InvalidDigit,
                });
            }
            i += 1;
        }
        
        let r = buf.len() % power;
        let split = if r == 0 {
            power
        } else {
            r
        };
        let mut out = Self::ZERO;
        let mut first: Digit = 0;
        i = 0;
        while i < split {
            first = first * 10 + Self::byte_to_digit(buf[i]) as Digit;
            i += 1;
        }
        out.digits[0] = first;
        let mut start = i;
        while start < buf.len() {
            let end = start + power;

            let mut carry = 0;
            let mut j = 0;
            while j < N {
                out.digits[j] = Self::mac_with_carry(out.digits[j], base, &mut carry);
                j += 1;
            }
            if carry != 0 {
                return Err(ParseIntError {
                    kind: IntErrorKind::PosOverflow,
                });
            }

            let mut n = 0;
            j = start;
            while j < end && j < buf.len() {
                n = n * 10 + Self::byte_to_digit(buf[j]) as Digit;
                j += 1;
            }

            out = match out.checked_add(Self::from_digit(n)) {
                Some(out) => out,
                None => return Err(ParseIntError {
                    kind: IntErrorKind::PosOverflow,
                }),
            };
            start = end;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::BUint;
    use crate::test::{test_bignum, quickcheck_from_to_radix};
    use core::str::FromStr;

	test_bignum! {
		function: <u128>::from_str,
		cases: [
            ("398475394875230495745"),
            ("3984753948752304957423490785029749572977970985")
        ]
	}

	test_bignum! {
		function: <u128>::from_str_radix,
		cases: [
            ("af7345asdofiuweor", 35u32),
            ("945hhdgi73945hjdfj", 32u32),
            ("3436847561345343455", 9u32),
            ("affe758457bc345540ac399", 16u32),
            ("affe758457bc345540ac39929334534ee34579234795", 17u32)
        ]
	}

    quickcheck_from_to_radix!(u128, radix_be, 255);
    quickcheck_from_to_radix!(u128, radix_le, 255);
    quickcheck_from_to_radix!(u128, str_radix, 36);

    #[test]
    fn from_to_radix_le() {
        let buf = &[23, 100, 45, 58, 44, 56, 55, 100, 76, 54, 10, 100, 100, 100, 100, 100, 200, 200, 200, 200, 255, 255, 255, 255, 255, 100, 100, 44, 60, 56, 48, 69, 160, 59, 50, 50, 200, 250, 250, 250, 250, 250, 240, 120];
        let u = BUint::<100>::from_radix_le(buf, 256).unwrap();
        let v = u.to_radix_le(256);
        assert_eq!(v, buf);

        let buf = &[34, 45, 32, 100, 53, 54, 65, 53, 0, 53];
        let option = BUint::<100>::from_radix_le(buf, 99);
        assert!(option.is_none());

        let buf = &[1, 0, 2, 3, 1, 0, 0, 2, 3, 1, 2, 3, 1, 0, 1, 2, 3, 1, 3, 1, 3, 1, 3, 2, 3, 2, 3, 1, 3, 2, 3, 1, 3, 2, 3, 2, 3, 1, 2, 3, 0, 0, 0, 2, 3];
        let u = BUint::<100>::from_radix_le(buf, 4).unwrap();
        let v = u.to_radix_le(4);
        assert_eq!(v, buf);
    }
    #[test]
    fn from_to_radix_be() {
        let buf = &[34, 57, 100, 184, 54, 40, 78, 10, 5, 200, 45, 67];
        let u = BUint::<100>::from_radix_be(buf, 201).unwrap();
        let v = u.to_radix_be(201);
        assert_eq!(v, buf);

        let buf = &[1, 0, 2, 3, 1, 0, 0, 2, 3, 1, 2, 3, 1, 0, 1, 2, 3, 1, 3, 1, 3, 1, 3, 2, 3, 2, 3, 1, 3, 2, 3, 1, 3, 2, 3, 2, 3, 1, 2, 3, 0, 0, 0, 2, 3];
        let u = BUint::<100>::from_radix_be(buf, 4).unwrap();
        let v = u.to_radix_be(4);
        assert_eq!(v, buf);

        let buf = &[100, 4, 0, 54, 45, 20, 200, 43];
        let option = BUint::<100>::from_radix_le(buf, 150);
        assert!(option.is_none());

        let buf = &[9, 5, 1, 5, 5, 1, 5, 9, 8, 7, 6, 4, 2, 5, 4, 2, 3, 4, 9, 0, 1, 2, 3, 4, 5, 1, 6, 6, 1, 6, 7, 1, 6, 5, 1, 5, 1, 6, 1, 7, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 1, 7, 1, 7, 1, 7, 1, 7, 5];
        let u = BUint::<100>::from_radix_be(buf, 10).unwrap();
        let v = u.to_radix_be(10);
        assert_eq!(v, buf);
    }
    #[test]
    fn from_to_str_radix() {
        let src = "34985789aasdfhoehdghjkh93485797df";
        let u = BUint::<100>::from_str_radix(src, 32).unwrap();
        let v = u.to_str_radix(32);
        assert_eq!(v, src);

        let src = "934579gfhjh394hdkg9845798";
        let result = BUint::<100>::from_str_radix(src, 18);
        assert!(result.is_err());

        let src = "120102301230102301230102030120321012";
        let u = BUint::<100>::from_str_radix(src, 4).unwrap();
        assert_eq!(u.to_str_radix(4), src);
    }
    #[test]
    fn parse_bytes() {
        let src = "134957dkbhadoinegrhi983475hdgkhgdhiu3894hfd";
        let u = BUint::<100>::parse_bytes(src.as_bytes(), 35).unwrap();
        let v = BUint::<100>::from_str_radix(src, 35).unwrap();
        assert_eq!(u, v);
        assert_eq!(v.to_str_radix(35), src);

        let bytes = b"345977fsuudf0350845";
        let option = BUint::<100>::parse_bytes(bytes, 20);
        assert!(option.is_none());
    }
}