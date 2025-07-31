/*
Most of the code in this file is adapted from the Rust `num_bigint` library, https://docs.rs/num-bigint/latest/num_bigint/, modified under the MIT license. The changes are released under either the MIT license or the Apache License 2.0, as described in the README. See LICENSE-MIT or LICENSE-APACHE at the project root.

The appropriate copyright notice for the `num_bigint` code is given below:
Copyright (c) 2014 The Rust Project Developers

The original license file and copyright notice for `num_bigint` can be found in this project's root at licenses/LICENSE-num-bigint.
*/

use super::Uint;
use crate::doc;
use crate::errors::ParseIntError;
use crate::ints::radix::assert_range;
use crate::{Digit, digit};
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "alloc")]
use core::iter::Iterator;
use core::num::IntErrorKind;
use core::str::FromStr;

#[inline]
const fn byte_to_digit<const FROM_STR: bool>(byte: u8) -> u8 {
    if FROM_STR {
        match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'z' => byte - b'a' + 10,
            b'A'..=b'Z' => byte - b'A' + 10,
            _ => u8::MAX,
        }
    } else {
        byte
    }
}

#[cfg(feature = "alloc")]
#[inline]
const fn digit_to_str_byte(digit: u8) -> u8 {
    if digit < 10 {
        digit + b'0'
    } else {
        digit + b'a' - 10
    }
}

#[doc = doc::radix::impl_desc!(Uint)]
impl<const N: usize> Uint<N> {
    #[cfg(feature = "alloc")]
    #[inline]
    const fn radix_base(radix: u32) -> (Digit, usize) {
        let mut power: usize = 1;
        let radix = radix as Digit;
        let mut base = radix;
        loop {
            match base.checked_mul(radix) {
                Some(n) => {
                    base = n;
                    power += 1;
                }
                None => return (base, power),
            }
        }
    }

    #[cfg(feature = "alloc")]
    #[inline]
    const fn radix_base_half(radix: u32) -> (Digit, usize) {
        const HALF_BITS_MAX: Digit = Digit::MAX >> (Digit::BITS / 2);

        let mut power: usize = 1;
        let radix = radix as Digit;
        let mut base = radix;
        loop {
            match base.checked_mul(radix) {
                Some(n) if n <= HALF_BITS_MAX => {
                    base = n;
                    power += 1;
                }
                _ => return (base, power),
            }
        }
    }

    /// Converts a byte slice in a given base to an integer. The input slice must contain ascii/utf8 characters in [0-9a-zA-Z].
    ///
    /// This function is equivalent to the [`from_str_radix`](#method.from_str_radix) function for a string slice equivalent to the byte slice and the same radix.
    ///
    /// Returns `None` if the conversion of the byte slice to string slice fails or if a digit is larger than or equal to the given radix, otherwise the integer is wrapped in `Some`.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let src = "394857hdgfjhsnkg947dgfjkeita";
    /// assert_eq!(U512::from_str_radix(src, 32).ok(), U512::parse_bytes(src.as_bytes(), 32));
    /// ```
    #[inline]
    pub const fn parse_bytes(buf: &[u8], radix: u32) -> Option<Self> {
        let s = crate::helpers::option_try!(crate::helpers::ok!(core::str::from_utf8(buf)));
        crate::helpers::ok!(Self::from_str_radix(s, radix))
    }

    /// Converts a slice of big-endian digits in the given radix to an integer. Each `u8` of the slice is interpreted as one digit of base `radix` of the number, so this function will return `None` if any digit is greater than or equal to `radix`, otherwise the integer is wrapped in `Some`.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let n = U512::from(34598748526857897975u128);
    /// let digits = n.to_radix_be(12);
    /// assert_eq!(Some(n), U512::from_radix_be(&digits, 12));
    /// ```
    #[inline]
    pub const fn from_radix_be(buf: &[u8], radix: u32) -> Option<Self> {
        assert_range!(radix, 256);
        if buf.is_empty() {
            return Some(Self::ZERO);
        }
        if radix == 256 {
            return Self::from_be_slice(buf);
        }

        crate::helpers::ok!(Self::from_buf_radix_internal::<false, true>(
            buf, radix, false
        ))
    }

    /// Converts a slice of little-endian digits in the given radix to an integer. Each `u8` of the slice is interpreted as one digit of base `radix` of the number, so this function will return `None` if any digit is greater than or equal to `radix`, otherwise the integer is wrapped in `Some`.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let n = U512::from(109837459878951038945798u128);
    /// let digits = n.to_radix_le(15);
    /// assert_eq!(Some(n), U512::from_radix_le(&digits, 15));
    /// ```
    #[inline]
    pub const fn from_radix_le(buf: &[u8], radix: u32) -> Option<Self> {
        assert_range!(radix, 256);
        if buf.is_empty() {
            return Some(Self::ZERO);
        }
        if radix == 256 {
            return Self::from_le_slice(buf);
        }

        crate::helpers::ok!(Self::from_buf_radix_internal::<false, false>(
            buf, radix, false
        ))
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
    /// This function panics if `radix` is not in the range from 2 to 36 inclusive.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// assert_eq!(U512::from_str_radix("A", 16), Ok(U512::TEN));
    /// ```
    #[inline]
    pub const fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
        Self::from_ascii_radix(src.as_bytes(), radix)
    }

    /// Parses an integer from an ASCII-byte slice with decimal digits.
    ///
    /// The characters are expected to be an optional + sign followed by only digits. Leading and trailing non-digit characters (including whitespace) represent an error. Underscores (which are accepted in Rust literals) also represent an error.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::types::U512;
    ///
    /// assert_eq!(U512::from_ascii(b"+10"), Ok(U512::TEN));
    /// ```
    #[inline]
    pub const fn from_ascii(src: &[u8]) -> Result<Self, ParseIntError> {
        Self::from_ascii_radix(src, 10)
    }

    /// Parses an integer from an ASCII-byte slice with digits in a given base.
    ///
    /// The characters are expected to be an optional `+` sign followed by only digits. Leading and trailing non-digit characters (including whitespace) represent an error. Underscores (which are accepted in Rust literals) also represent an error.
    /// Digits are a subset of these characters, depending on radix:
    /// 
    /// - `0-9`
    /// - `a-z`
    /// - `A-Z`
    ///
    /// # Panics
    ///
    /// This function panics if radix is not in the range from 2 to 36 inclusive.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// assert_eq!(U512::from_ascii_radix(b"A", 16), Ok(U512::TEN));
    /// ```
    #[inline]
    pub const fn from_ascii_radix(src: &[u8], radix: u32) -> Result<Self, ParseIntError> {
        assert_range!(radix, 36);
        if src.is_empty() {
            return Err(ParseIntError {
                kind: IntErrorKind::Empty,
            });
        }
        let leading_plus = src[0] == b'+';
        Self::from_buf_radix_internal::<true, true>(src, radix, leading_plus)
    }

    pub(crate) const fn from_buf_radix_internal<const FROM_STR: bool, const BE: bool>(
        buf: &[u8],
        radix: u32,
        leading_sign: bool,
    ) -> Result<Self, ParseIntError> {
        // TODO: can use u128
        if leading_sign && buf.len() == 1 {
            return Err(ParseIntError {
                kind: IntErrorKind::InvalidDigit,
            });
        }
        let input_digits_len = if leading_sign {
            buf.len() - 1
        } else {
            buf.len()
        };

        match radix {
            2 | 4 | 16 | 256 => {
                let mut out = Self::ZERO;
                let base_digits_per_digit = (Digit::BITS / radix.ilog2()) as usize;
                let full_digits = input_digits_len / base_digits_per_digit as usize;
                let remaining_digits = input_digits_len % base_digits_per_digit as usize;
                let radix_u8 = radix as u8;

                if full_digits > N || full_digits == N && remaining_digits != 0 {
                    let mut i = if leading_sign { 1 } else { 0 };
                    while i < N * base_digits_per_digit + if leading_sign { 1 } else { 0 } {
                        if byte_to_digit::<FROM_STR>(buf[i]) >= radix_u8 {
                            return Err(ParseIntError {
                                kind: IntErrorKind::InvalidDigit,
                            });
                        }
                        i += 1;
                    }
                    return Err(ParseIntError {
                        kind: IntErrorKind::PosOverflow,
                    });
                }

                let log2r = radix.ilog2();

                let mut i = 0;
                while i < full_digits {
                    let mut j = 0;
                    while j < base_digits_per_digit {
                        let idx = if BE {
                            buf.len() - 1 - (i * base_digits_per_digit + j)
                        } else {
                            i * base_digits_per_digit + j
                        };
                        let d = byte_to_digit::<FROM_STR>(buf[idx]);
                        if d >= radix_u8 {
                            return Err(ParseIntError {
                                kind: IntErrorKind::InvalidDigit,
                            });
                        }
                        out.digits[i] |= (d as Digit) << (j * log2r as usize);
                        j += 1;
                    }
                    i += 1;
                }
                let mut j = 0;
                while j < remaining_digits {
                    let idx = if BE {
                        buf.len() - 1 - (i * base_digits_per_digit + j)
                    } else {
                        i * base_digits_per_digit + j
                    };
                    let d = byte_to_digit::<FROM_STR>(buf[idx]);
                    if d >= radix_u8 {
                        return Err(ParseIntError {
                            kind: IntErrorKind::InvalidDigit,
                        });
                    }
                    out.digits[i] |= (d as Digit) << (j * log2r as usize);
                    j += 1;
                }
                Ok(out)
            }
            /*8 | 32 | 64 | 128*/
            0 => {
                // TODO: for now, we don't use this, as hard to get the errors right
                let mut out = Self::ZERO;
                let radix_u8 = radix as u8;
                let log2r = radix.ilog2();

                let mut index = 0;
                let mut shift = 0;

                let mut i = buf.len();
                let stop_index = if leading_sign { 1 } else { 0 };
                while i > stop_index {
                    i -= 1;
                    let idx = if BE { i } else { buf.len() - 1 - i };
                    let d = byte_to_digit::<FROM_STR>(buf[idx]);
                    if d >= radix_u8 {
                        return Err(ParseIntError {
                            kind: IntErrorKind::InvalidDigit,
                        });
                    }
                    out.digits[index] |= (d as Digit) << shift;
                    shift += log2r;
                    if shift >= Digit::BITS {
                        shift -= Digit::BITS;
                        let carry = (d as Digit) >> (log2r - shift);
                        index += 1;
                        if index == N {
                            if carry != 0 {
                                return Err(ParseIntError {
                                    kind: IntErrorKind::PosOverflow,
                                });
                            }
                            while i > stop_index {
                                i -= 1;
                                let idx = if BE { i } else { buf.len() - 1 - i };
                                let d = byte_to_digit::<FROM_STR>(buf[idx]);
                                if d != 0 {
                                    return Err(ParseIntError {
                                        kind: IntErrorKind::PosOverflow,
                                    });
                                }
                            }
                            return Ok(out);
                        } else {
                            out.digits[index] = carry;
                        }
                    }
                }
                Ok(out)
            }
            _ => {
                let (base, power) = Self::radix_base(radix);
                let r = input_digits_len % power;
                let split = if r == 0 { power } else { r };
                let radix_u8 = radix as u8;
                let mut out = Self::ZERO;
                let mut first: Digit = 0;
                let mut i = if leading_sign { 1 } else { 0 };
                while i < if leading_sign { split + 1 } else { split } {
                    let idx = if BE { i } else { buf.len() - 1 - i };
                    let d = byte_to_digit::<FROM_STR>(buf[idx]);
                    if d >= radix_u8 {
                        return Err(ParseIntError {
                            kind: IntErrorKind::InvalidDigit,
                        });
                    }
                    first = first * (radix as Digit) + d as Digit;
                    i += 1;
                }
                out.digits[0] = first;
                let mut start = i;
                while start < buf.len() {
                    let end = start + power;

                    let mut carry = 0;
                    let mut j = 0;
                    while j < N {
                        let (low, high) = digit::carrying_mul(out.digits[j], base, carry, 0);
                        carry = high;
                        out.digits[j] = low;
                        j += 1;
                    }
                    if carry != 0 {
                        while start < buf.len() && start < end {
                            // TODO: this isn't quite correct behaviour
                            let idx = if BE { start } else { buf.len() - 1 - start };
                            let d = byte_to_digit::<FROM_STR>(buf[idx]);
                            if d >= radix_u8 {
                                return Err(ParseIntError {
                                    kind: IntErrorKind::InvalidDigit,
                                });
                            }
                            start += 1;
                        }
                        return Err(ParseIntError {
                            kind: IntErrorKind::PosOverflow,
                        });
                    }

                    let mut n = 0;
                    j = start;
                    while j < end && j < buf.len() {
                        let idx = if BE { j } else { buf.len() - 1 - j };
                        let d = byte_to_digit::<FROM_STR>(buf[idx]);
                        if d >= radix_u8 {
                            return Err(ParseIntError {
                                kind: IntErrorKind::InvalidDigit,
                            });
                        }
                        n = n * (radix as Digit) + d as Digit;
                        j += 1;
                    }

                    out = match out.checked_add(Self::from_digit(n)) {
                        Some(out) => out,
                        None => {
                            return Err(ParseIntError {
                                kind: IntErrorKind::PosOverflow,
                            });
                        }
                    };
                    start = end;
                }
                Ok(out)
            }
        }
    }

    #[cfg(feature = "alloc")]
    /// Returns the integer as a string in the given radix.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let src = "abcdefghijklmnopqrstuvwxyz";
    /// let n = U512::from_str_radix(src, 36).unwrap();
    /// assert_eq!(n.to_str_radix(36), src);
    /// ```
    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        assert_range!(radix, 36);
        let mut out = Self::to_radix_be(self, radix);

        for byte in out.iter_mut() {
            *byte = digit_to_str_byte(*byte);
        }
        unsafe { String::from_utf8_unchecked(out) }
    }

    #[cfg(feature = "alloc")]
    /// Returns the integer in the given base in big-endian digit order.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let digits = &[3, 55, 60, 100, 5, 0, 5, 88];
    /// let n = U512::from_radix_be(digits, 120).unwrap();
    /// assert_eq!(n.to_radix_be(120), digits);
    /// ```
    #[inline]
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        let mut v = self.to_radix_le(radix);
        v.reverse();
        v
    }

    #[cfg(feature = "alloc")]
    /// Returns the integer in the given base in little-endian digit order.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let digits = &[1, 67, 88, 200, 55, 68, 87, 120, 178];
    /// let n = U512::from_radix_le(digits, 250).unwrap();
    /// assert_eq!(n.to_radix_le(250), digits);
    /// ```
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        // TODO: can use u128
        assert_range!(radix, 256);
        if self.is_zero() {
            vec![0]
        } else if radix.is_power_of_two() {
            if radix == 256 {
                return (&self.digits[0..=self.last_digit_index()])
                    .iter()
                    .copied()
                    .collect();
            }

            let bits = radix.ilog2();
            if Digit::BITS % bits == 0 {
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

    #[cfg(feature = "alloc")]
    fn to_bitwise_digits_le(self, bits: u32) -> Vec<u8> {
        // TODO: can use u128
        let last_digit_index = self.last_digit_index();
        let mask: Digit = (1 << bits) - 1;
        let digits_per_big_digit = Digit::BITS / bits;
        let digits = self.bits().div_ceil(bits);
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

    #[cfg(feature = "alloc")]
    fn to_inexact_bitwise_digits_le(self, bits: u32) -> Vec<u8> {
        // TODO: can use u128
        let mask: Digit = (1 << bits) - 1;
        let digits = self.bits().div_ceil(bits);
        let mut out = Vec::with_capacity(digits as usize);
        let mut r = 0;
        let mut rbits = 0;
        for c in self.digits {
            r |= c << rbits;
            rbits += Digit::BITS;

            while rbits >= bits {
                out.push((r & mask) as u8);
                r >>= bits;

                if rbits > Digit::BITS {
                    r = c >> (Digit::BITS - (rbits - bits));
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

    #[cfg(feature = "alloc")]
    fn to_radix_digits_le(self, radix: u32) -> Vec<u8> {
        let radix_digits = self.bits().div_ceil(radix.ilog2());
        let mut out = Vec::with_capacity(radix_digits as usize);
        let (base, power) = Self::radix_base_half(radix);
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

impl<const N: usize> FromStr for Uint<N> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::test_bignum;
    use core::num::IntErrorKind;
    use core::str::FromStr;

    test_bignum! {
        function: <utest>::from_str,
        cases: [
            ("398475394875230495745"),
            ("3984753948752304957423490785029749572977970985"),
            ("12345💩👍"),
            ("1234567890a"),
            ("")
        ]
    }

    #[cfg(feature = "nightly")]
    test_bignum! {
        function: <utest>::from_ascii,
        cases: [
            ("11111111".as_bytes()),
            ("10000000000000000000000000000000000".as_bytes()),
            ("12💩👍45".as_bytes()),
            ("b1234567890a".as_bytes()),
            ("".as_bytes())
        ]
    }

    #[cfg(feature = "nightly")]
    test_bignum! {
        function: <utest>::from_ascii_radix,
        cases: [
            ("+af7345asdofiuweor".as_bytes(), 35u32),
            ("+945hhdgi73945hjdfj".as_bytes(), 32u32),
            ("+3436847561345343455".as_bytes(), 9u32),
            ("+affe758457bc345540ac399".as_bytes(), 16u32),
            ("+affe758457bc345540ac39929334534ee34579234795".as_bytes(), 17u32),
            ("+3777777777777777777777777777777777777777777".as_bytes(), 8u32),
            ("+37777777777777777777777777777777777777777761".as_bytes(), 8u32),
            ("+1777777777777777777777".as_bytes(), 8u32),
            ("+17777777777777777777773".as_bytes(), 8u32),
            ("+2000000000000000000000".as_bytes(), 8u32),
            ("-234598734".as_bytes(), 10u32),
            ("g234ab".as_bytes(), 16u32),
            ("234£$2234".as_bytes(), 15u32),
            ("123456💯".as_bytes(), 30u32),
            ("3434💯34593487".as_bytes(), 12u32),
            ("💯34593487".as_bytes(), 11u32),
            ("abcdefw".as_bytes(), 32u32),
            ("1234ab".as_bytes(), 11u32),
            ("1234".as_bytes(), 4u32),
            ("010120101".as_bytes(), 2u32),
            ("10000000000000000".as_bytes(), 16u32),
            ("p8hrbe0mo0084i6vckj1tk7uvacnn4cm".as_bytes(), 32u32),
            ("".as_bytes(), 10u32)
        ]
    }

    test_bignum! {
        function: <utest>::from_str_radix,
        cases: [
            ("+af7345asdofiuweor", 35u32),
            ("+945hhdgi73945hjdfj", 32u32),
            ("+3436847561345343455", 9u32),
            ("+affe758457bc345540ac399", 16u32),
            ("+affe758457bc345540ac39929334534ee34579234795", 17u32),
            ("+3777777777777777777777777777777777777777777", 8u32),
            ("+37777777777777777777777777777777777777777761", 8u32),
            ("+1777777777777777777777", 8u32),
            ("+17777777777777777777773", 8u32),
            ("+2000000000000000000000", 8u32),
            ("-234598734", 10u32),
            ("g234ab", 16u32),
            ("234£$2234", 15u32),
            ("123456💯", 30u32),
            ("3434💯34593487", 12u32),
            ("💯34593487", 11u32),
            ("abcdefw", 32u32),
            ("1234ab", 11u32),
            ("1234", 4u32),
            ("010120101", 2u32),
            ("10000000000000000", 16u32),
            ("p8hrbe0mo0084i6vckj1tk7uvacnn4cm", 32u32),
            ("", 10u32)
        ]
    }

    #[cfg(feature = "alloc")]
    crate::test::quickcheck_from_to_radix!(utest, radix_be, 256);
    #[cfg(feature = "alloc")]
    crate::test::quickcheck_from_to_radix!(utest, radix_le, 256);
    #[cfg(feature = "alloc")]
    crate::test::quickcheck_from_to_radix!(utest, str_radix, 36);

    #[test]
    fn from_str_radix_empty() {
        let _ = UTEST::from_str_radix("", 10).unwrap_err().kind() == &IntErrorKind::Empty;
    }

    #[test]
    fn from_str_radix_invalid_char() {
        let _ = UTEST::from_str_radix("a", 10).unwrap_err().kind() == &IntErrorKind::InvalidDigit;
    }

    #[test]
    #[should_panic(expected = "Radix must be in range [2, 36]")]
    fn from_str_radix_invalid_radix() {
        let _ = UTEST::from_str_radix("1234", 37).unwrap();
    }

    #[test]
    #[should_panic(expected = "Radix must be in range [2, 256]")]
    fn from_radix_be_invalid_radix() {
        let _ = UTEST::from_radix_be(&[1], 257);
    }

    #[test]
    #[should_panic(expected = "Radix must be in range [2, 256]")]
    fn from_radix_le_invalid_radix() {
        let _ = UTEST::from_radix_le(&[1], 257);
    }

    #[test]
    fn parse_empty() {
        assert_eq!(UTEST::from_radix_be(&[], 10), Some(UTEST::ZERO));
        assert_eq!(UTEST::from_radix_le(&[], 10), Some(UTEST::ZERO));
    }

    #[cfg(feature = "alloc")]
    crate::test::quickcheck_from_str_radix!(utest, "+" | "");
    #[cfg(feature = "alloc")]
    crate::test::quickcheck_from_str!(utest);

    #[cfg(feature = "alloc")]
    #[test]
    fn parse_bytes() {
        use crate::Uint;

        let src = "134957dkbhadoinegrhi983475hdgkhgdhiu3894hfd";
        let u = Uint::<100>::parse_bytes(src.as_bytes(), 35).unwrap();
        let v = Uint::<100>::from_str_radix(src, 35).unwrap();
        assert_eq!(u, v);
        assert_eq!(v.to_str_radix(35), src);

        let bytes = b"345977fsuudf0350845";
        let option = Uint::<100>::parse_bytes(bytes, 20);
        assert!(option.is_none());
    }
}
