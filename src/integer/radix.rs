/*
Most of the code in this file is adapted from the Rust `num_bigint` library, https://docs.rs/num-bigint/latest/num_bigint/, modified under the MIT license. The changes are released under either the MIT license or the Apache License 2.0, as described in the README. See LICENSE-MIT or LICENSE-APACHE at the project root.

The appropriate copyright notice for the `num_bigint` code is given below:
Copyright (c) 2014 The Rust Project Developers

The original license file and copyright notice for `num_bigint` can be found in this project's root at licenses/LICENSE-num-bigint.
*/

use crate::{Uint, Integer};
use crate::errors::ParseIntError;
use crate::{Byte, digit};
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "alloc")]
use core::iter::Iterator;
use core::num::IntErrorKind;

macro_rules! assert_range {
    ($radix: expr, $max: expr) => {
        assert!(
            $radix >= 2 && $radix <= $max,
            crate::errors::err_msg!(concat!(
                "Radix must be in range [2, ",
                stringify!($max),
                "]"
            ))
        )
    };
}

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

/// Returns the maximum power of `radix` that fits in a `u64`, together with the associated exponent
#[inline]
const fn max_radix_power(radix: u32) -> (u64, usize) {
    let mut power: u64 = radix as u64;
    let mut exponent = 1;
    loop {
        match power.checked_mul(radix as u64) {
            Some(n) => {
                power = n;
                exponent += 1;
            }
            None => return (power, exponent),
        }
    }
}

// we index using the radix itself
const MAX_RADIX_POWERS: [(u64, usize); 257] = {
    let mut arr = [(0, 0); 257];
    let mut i = 2;
    while i <= 256 {
        arr[i] = max_radix_power(i as u32);
        i += 1;
    }
    arr
};

struct DigitsIter<'a, const SKIP_UNDERSCORES: bool, const ASCII: bool, const BE: bool> {
    buf: &'a [u8],
    index: usize,
}

impl<'a, const SKIP_UNDERSCORES: bool, const ASCII: bool, const BE: bool> DigitsIter<'a, SKIP_UNDERSCORES, ASCII, BE> {
    #[inline]
    const fn new(buf: &'a [u8]) -> Self {
        Self { buf, index: if BE { 0 } else { buf.len() } }
    }
}

impl<'a, const SKIP_UNDERSCORES: bool, const ASCII: bool, const BE: bool> DigitsIter<'a, SKIP_UNDERSCORES, ASCII, BE> {
    #[inline]
    const fn next_be(&mut self) -> Option<u8> {
        while self.index < self.buf.len() {
            let b = self.buf[self.index];
            self.index += 1;
            if SKIP_UNDERSCORES && b == b'_' {
                continue;
            }
            return Some(byte_to_digit::<ASCII>(b));
        }
        None
    }

    #[inline]
    const fn next_le(&mut self) -> Option<u8> {
        while self.index > 0 {
            self.index -= 1;
            let b = self.buf[self.index];
            if SKIP_UNDERSCORES && b == b'_' {
                continue;
            }
            return Some(byte_to_digit::<ASCII>(b));
        }
        None
    }

    #[inline]
    const fn next(&mut self) -> Option<u8> {
        if BE {
            self.next_be()
        } else {
            self.next_le()
        }
    }
}

macro_rules! impl_desc {
    () => {
        "Methods which convert integers to and from strings of digits in a given radix (base)."
    };
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    #[inline] 
    fn to_digits_le(self, radix: u32) -> Vec<u8> {
        let mut digits = Vec::with_capacity(Self::BITS.div_ceil(radix.ilog2()) as usize); // log_r (2^B) = B log_r (2) = B/log_2 (r)
        let mut current = self;
        let radix_u64 = radix as u64;
        let (max_pow, max_pow_exponent) = MAX_RADIX_POWERS[radix as usize];
        loop {
            let (q, mut r) = current.div_rem_u64(max_pow);
            if q.is_zero() {
                while r != 0 {
                    digits.push((r % radix_u64) as u8);
                    r /= radix_u64;
                }
                return digits;
            }
            for _ in 0..max_pow_exponent {
                digits.push((r % radix_u64) as u8); // guaranteed to fit into u8 as radix_u64 <= 256
                r /= radix_u64;
            }
            current = q;
        }
    }

    #[inline]
    fn to_exact_bitwise_digits_le(mut self, bits: u32) -> Vec<u8> {
        let mask = (u32::MAX >> (32 - bits)) as u8;
        let mut digits = Vec::with_capacity(Self::BITS.div_ceil(bits) as usize);
        debug_assert!(mask.trailing_ones() == bits);
        debug_assert!(mask.count_ones() == bits); // mask is l low-order 1s
        let num_non_zero_digits = self.bits().div_ceil(8) as usize;
        let digits_per_big_digit = u8::BITS / bits;

        // let mut i = 0;
        for d in &mut self.bytes[0..num_non_zero_digits - 1] {
            // let mut d = unsafe { self.digits[i] };
            for _ in 0..digits_per_big_digit {
                let digit = *d & mask; // can truncate to u32 as this is equivalent to bitand-ing with zeros
                digits.push(digit);
                *d >>= bits;
            }

            // i += 1;
        }
        let mut d = self.bytes[num_non_zero_digits - 1];
        while d != 0 {
            let digit = d & mask; // can truncate to u32 as this is equivalent to bitand-ing with zeros
            digits.push(digit);
            d >>= bits;
        }
        digits
    }

    #[inline]
    fn to_inexact_bitwise_digits_le2(self, bits: u32) -> Vec<u8> {
        let mut digits = Vec::with_capacity(Self::BITS.div_ceil(bits) as usize);
        let mask = u32::MAX >> (32 - bits);
        debug_assert!(mask.trailing_ones() == bits);
        debug_assert!(mask.count_ones() == bits);

        let num_non_zero_digits = self.bits().div_ceil(128) as usize; // number of non-zero u128 digits
        let mut i = 0;
        while i < num_non_zero_digits {
            let mut d = unsafe { self.as_wide_digits().get(i) };
            for _ in 0..(u128::BITS / bits) {
                let digit = d as u32 & mask; // can truncate to u32 as this is equivalent to bitand-ing with zeros
                digits.push(digit as u8);
                d >>= bits;
            }

            i += 1;
        }

        digits
    }

    #[cfg(feature = "alloc")]
    #[inline]
    const fn radix_base(radix: u32) -> (Byte, usize) {
        let mut power: usize = 1;
        let radix = radix as Byte;
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

    /// Converts a slice of big-endian digits in the given radix to an integer. Each `u8` of the slice is interpreted as one digit of base `radix` of the number, so this function will return `None` if any digit is greater than or equal to `radix`, or if the integer represented by the digits is too large to be represented by `Self`. Otherwise, the integer is wrapped in `Some`.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    ///
    /// let n = U512::MAX;
    /// let digits = n.to_radix_be(12);
    /// assert_eq!(Some(n), U512::from_radix_be(&digits, 12));
    /// 
    /// let a = U512::from_radix_be(&[4, 3, 2, 1], 11).unwrap();
    /// let b: U512 = n!(1)*n!(11).pow(0) + n!(2)*n!(11).pow(1) + n!(3)*n!(11).pow(2) + n!(4)*n!(11).pow(3);
    /// 
    /// assert_eq!(a, b); // 4*11^3 + 3*11^2 + 2*11^1 + 1*11^0
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

        crate::helpers::ok!(Self::from_buf_radix_internal::<false, true, false>(
            buf, radix
        ))
    }

    /// Converts a slice of little-endian digits in the given radix to an integer. Each `u8` of the slice is interpreted as one digit of base `radix` of the number, so this function will return `None` if any digit is greater than or equal to `radix`, or if the integer represented by the digits is too large to be represented by `Self`. Otherwise, the integer is wrapped in `Some`.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    ///
    /// let n = U512::MAX;
    /// let digits = n.to_radix_le(15);
    /// assert_eq!(Some(n), U512::from_radix_le(&digits, 15));
    /// 
    /// let a = U512::from_radix_le(&[5, 6, 7, 8], 18).unwrap();
    /// let b: U512 = n!(5)*n!(18).pow(0) + n!(6)*n!(18).pow(1) + n!(7)*n!(18).pow(2) + n!(8)*n!(18).pow(3);
    /// 
    /// assert_eq!(a, b); // 8*18^3 + 7*18^2 + 6*18^1 + 5*18^0
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

        crate::helpers::ok!(Self::from_buf_radix_internal::<false, false, false>(
            buf, radix
        ))
    }

    pub(crate) const fn from_buf_radix_internal<const FROM_STR: bool, const BE: bool, const SKIP_UNDERSCORES: bool>(
        buf: &[u8],
        radix: u32
    ) -> Result<Self, ParseIntError> {
        let input_digits_len = buf.len();

        match radix {
            2 | 4 | 16 | 256 => {
                let mut out = Self::ZERO;
                let base_digits_per_digit = (Byte::BITS / radix.ilog2()) as usize;
                let full_digits = input_digits_len / base_digits_per_digit as usize;
                let remaining_digits = input_digits_len % base_digits_per_digit as usize;
                let radix_u8 = radix as u8;

                if full_digits > N || full_digits == N && remaining_digits != 0 {
                    let mut index = 0;
                    let mut digits_visited = 0;
                    while digits_visited < N * base_digits_per_digit {
                        if SKIP_UNDERSCORES && buf[index] == b'_' {
                            index += 1;
                            continue;
                        }
                        if byte_to_digit::<FROM_STR>(buf[index]) >= radix_u8 {
                            return Err(ParseIntError {
                                kind: IntErrorKind::InvalidDigit,
                            });
                        }
                        index += 1;
                        digits_visited += 1;
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
                        out.bytes[i] |= (d as Byte) << (j * log2r as usize);
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
                    out.bytes[i] |= (d as Byte) << (j * log2r as usize);
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
                let stop_index = 0;
                while i > stop_index {
                    i -= 1;
                    let idx = if BE { i } else { buf.len() - 1 - i };
                    let d = byte_to_digit::<FROM_STR>(buf[idx]);
                    if d >= radix_u8 {
                        return Err(ParseIntError {
                            kind: IntErrorKind::InvalidDigit,
                        });
                    }
                    out.bytes[index] |= (d as Byte) << shift;
                    shift += log2r;
                    if shift >= Byte::BITS {
                        shift -= Byte::BITS;
                        let carry = (d as Byte) >> (log2r - shift);
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
                            out.bytes[index] = carry;
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
                let mut first: Byte = 0;
                let mut i = 0;
                while i < split {
                    let idx = if BE { i } else { buf.len() - 1 - i };
                    let d = byte_to_digit::<FROM_STR>(buf[idx]);
                    if d >= radix_u8 {
                        return Err(ParseIntError {
                            kind: IntErrorKind::InvalidDigit,
                        });
                    }
                    first = first * (radix as Byte) + d as Byte;
                    i += 1;
                }
                out.bytes[0] = first;
                let mut start = i;
                while start < buf.len() {
                    let end = start + power;

                    let mut carry = 0;
                    let mut j = 0;
                    while j < N {
                        let (low, high) = digit::carrying_mul(out.bytes[j], base, carry, 0);
                        carry = high;
                        out.bytes[j] = low;
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
                        n = n * (radix as Byte) + d as Byte;
                        j += 1;
                    }

                    out = match out.checked_add(Self::from_byte(n)) {
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
        assert_range!(radix, 256);
        if self.is_zero() {
            vec![0]
        } else if radix.is_power_of_two() {
            let bits = radix.ilog2();
            if u128::BITS % bits == 0 {
                self.to_bitwise_digits_le(bits)
            } else {
                self.to_inexact_bitwise_digits_le(bits)
            }
        } else if radix == 10 {
            self.to_digits_le(10)
        } else {
            self.to_digits_le(radix)
        }
    }

    #[cfg(feature = "alloc")]
    fn to_bitwise_digits_le(self, bits: u32) -> Vec<u8> {
        // no need to use wider digits, as that would just increase the number of iterations in the inner for loop (so total number of iters is the same)
        let self_bits = self.bits();
        let last_digit_index = self_bits.div_ceil(8) as usize - 1;
        let mask: Byte = (1 << bits) - 1;
        let digits_per_big_digit = Byte::BITS / bits;
        let digits = self_bits.div_ceil(bits);

        let mut digits = Vec::with_capacity(digits as usize);

        for mut d in self.bytes.into_iter().take(last_digit_index) {
            for _ in 0..digits_per_big_digit {
                digits.push((d & mask) as u8);
                d >>= bits;
            }
        }
        let mut r = unsafe { *self.bytes.get_unchecked(last_digit_index) };
        while r != 0 {
            digits.push((r & mask) as u8);
            r >>= bits;
        }
        digits
    }

    #[cfg(feature = "alloc")]
    fn to_inexact_bitwise_digits_le(self, bits: u32) -> Vec<u8> {
        // TODO: can use u128
        let mask: Byte = (1 << bits) - 1;
        let digits = self.bits().div_ceil(bits);
        let mut out = Vec::with_capacity(digits as usize);
        let mut r = 0;
        let mut rbits = 0;
        for c in self.bytes {
            r |= c << rbits;
            rbits += Byte::BITS;

            while rbits >= bits {
                out.push((r & mask) as u8);
                r >>= bits;

                if rbits > Byte::BITS {
                    r = c >> (Byte::BITS - (rbits - bits));
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
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Converts a string slice in a given base to an integer.
    ///
    /// The string is expected to be an optional `+` (or `-` if the integer is signed) sign followed by digits. Leading and trailing whitespace represent an error. Underscores (which are accepted in Rust literals) also represent an error.
    /// 
    /// Digits are a subset of these characters, depending on `radix`:
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
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(U512::from_str_radix("A", 16), Ok(n!(10)));
    /// assert_eq!(I512::from_str_radix("-B", 16), Ok(n!(-11)));
    /// ```
    #[inline]
    pub const fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
        Self::from_ascii_radix(src.as_bytes(), radix)
    }

    /// Parses an integer from an ASCII-byte slice with decimal digits.
    ///
    /// The characters are expected to be an optional `+` (or `-` if the integer is signed) sign followed by only digits. Leading and trailing non-digit characters (including whitespace) represent an error. Underscores (which are accepted in Rust literals) also represent an error.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(U512::from_ascii(b"+10"), Ok(n!(10 U512)));
    /// assert_eq!(I512::from_ascii(b"-1234"), Ok(n!(-1234 I512)));
    /// ```
    #[inline]
    pub const fn from_ascii(src: &[u8]) -> Result<Self, ParseIntError> {
        Self::from_ascii_radix(src, 10)
    }

    /// Parses an integer from an ASCII-byte slice with digits in a given base.
    ///
    /// The characters are expected to be an optional `+` sign followed by only digits. Leading and trailing non-digit characters (including whitespace) represent an error. Underscores (which are accepted in Rust literals) also represent an error.
    /// 
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
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(U512::from_ascii_radix(b"A", 16), Ok(n!(10)));
    /// assert_eq!(I512::from_ascii_radix(b"-C", 16), Ok(n!(-12)));
    /// ```
    #[inline]
    pub const fn from_ascii_radix(src: &[u8], radix: u32) -> Result<Self, ParseIntError> {
        assert_range!(radix, 36);
        if src.is_empty() {
            return Err(ParseIntError {
                kind: IntErrorKind::Empty,
            });
        }
        if src.len() == 1 && (src[0] == b'+' || (S && src[0] == b'-')) {
            return Err(ParseIntError {
                kind: IntErrorKind::InvalidDigit,
            });
        }

        let (src, negative) = if S && src[0] == b'-' {
            (src.split_at(1).1, true)
        } else if src[0] == b'+' {
            (src.split_at(1).1, false)
        } else {
            (src, false)
        };
        match Uint::from_buf_radix_internal::<true, true, false>(src, radix) {
            Ok(uint) => {
                let out = uint.force_sign::<S>();
                if S && negative {
                    // no error iff out is positive or out is Self::MIN, i.e. ...
                    if uint.gt(&Self::MIN.force_sign()) {
                        Err(ParseIntError {
                            kind: IntErrorKind::NegOverflow,
                        })
                    } else {
                        Ok(out.wrapping_neg()) // needs to be wrapping_neg as we need to handle the Self::MIN case (Self::MIN is mapped to Self:MIN)
                    }
                } else {
                    if out.is_negative_internal() {
                        Err(ParseIntError {
                            kind: IntErrorKind::PosOverflow,
                        })
                    } else {
                        Ok(out)
                    }
                }
            },
            Err(err) => {
                match err.kind() {
                    IntErrorKind::PosOverflow if S && negative => {
                        Err(ParseIntError {
                            kind: IntErrorKind::NegOverflow,
                        })
                    },
                    _ => Err(err),
                }
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
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// let src = "abcdefghijklmnopqrstuvwxyz";
    /// let n = U512::from_str_radix(src, 36).unwrap();
    /// assert_eq!(n.to_str_radix(36), src);
    /// 
    /// let a: I512 = n!(-0o123456701234567);
    /// assert_eq!(a.to_str_radix(8), "-123456701234567");
    /// ```
    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        if self.is_negative_internal() {
            return format!("-{}", self.unsigned_abs_internal().to_str_radix(radix));
        }

        assert_range!(radix, 36);

        let mut out = self.force_sign::<false>().to_radix_be(radix);

        for byte in out.iter_mut() {
            *byte = digit_to_str_byte(*byte);
        }

        unsafe { String::from_utf8_unchecked(out) }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use core::num::IntErrorKind;
    use core::str::FromStr;

    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <stest>::from_str,
            cases: [
                ("398475394875230495745"),
                ("3984753948752304957423490785029749572977970985"),
                ("12345💩👍"),
                ("1234567890a"),
                (""),
                ("+10"),
                ("-10"),
                ("1234567890"),
                ("-1234567890"),
                ("+1234567890"),
                ("-12345678901234567890"),
                ("+12345678901234567890"),
                ("-9223372036854775808"),
                ("+9223372036854775807")
            ]
        }

        #[cfg(feature = "nightly")] // since int_from_ascii not stable yet
        test_bignum! {
            function: <stest>::from_ascii,
            cases: [
                ("11111111".as_bytes()),
                ("10000000000000000000000000000000000".as_bytes()),
                ("12💩👍45".as_bytes()),
                ("b1234567890a".as_bytes()),
                ("".as_bytes()),
                ("+10".as_bytes()),
                ("-10".as_bytes()),
                ("1234567890".as_bytes()),
                ("-1234567890".as_bytes()),
                ("+1234567890".as_bytes()),
                ("-12345678901234567890".as_bytes()),
                ("+12345678901234567890".as_bytes()),
                ("-9223372036854775808".as_bytes()),
                ("+9223372036854775807".as_bytes()),
                ("0".as_bytes())
            ]
        }

        #[cfg(feature = "nightly")] // since int_from_ascii not stable yet
        test_bignum! {
            function: <stest>::from_ascii_radix,
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
                ("".as_bytes(), 10u32),
                ("-14359abcasdhfkdgdfgsde".as_bytes(), 34u32),
                ("+23797984569ahgkhhjdskjdfiu".as_bytes(), 32u32),
                ("-253613132341435345".as_bytes(), 7u32),
                ("+23467abcad47790809ef37".as_bytes(), 16u32),
                ("-712930769245766867875986646".as_bytes(), 10u32),
                ("-😱234292".as_bytes(), 36u32),
                ("-+345934758".as_bytes(), 13u32),
                ("12💯12".as_bytes(), 15u32),
                ("gap gap".as_bytes(), 36u32),
                ("-9223372036854775809".as_bytes(), 10u32),
                ("-1000000000000000000001".as_bytes(), 8u32),
                ("+1000000000000000000001".as_bytes(), 8u32),
                ("-8000000000000001".as_bytes(), 16u32),
                ("+-23459374".as_bytes(), 15u32),
                ("8000000000000000".as_bytes(), 16u32),
                ("".as_bytes(), 10u32)
            ]
        }

        test_bignum! {
            function: <stest>::from_str_radix,
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
                ("", 10u32),
                ("-14359abcasdhfkdgdfgsde", 34u32),
                ("+23797984569ahgkhhjdskjdfiu", 32u32),
                ("-253613132341435345", 7u32),
                ("+23467abcad47790809ef37", 16u32),
                ("-712930769245766867875986646", 10u32),
                ("-😱234292", 36u32),
                ("-+345934758", 13u32),
                ("12💯12", 15u32),
                ("gap gap", 36u32),
                ("-9223372036854775809", 10u32),
                ("-1000000000000000000001", 8u32),
                ("+1000000000000000000001", 8u32),
                ("-8000000000000001", 16u32),
                ("+-23459374", 15u32),
                ("8000000000000000", 16u32),
                ("", 10u32)
            ]
        }

        #[cfg(feature = "alloc")]
        crate::test::quickcheck_from_to_radix!(stest, str_radix, 36);
        #[cfg(feature = "alloc")]
        crate::test::quickcheck_from_str!(stest);

        #[test]
        fn from_str_radix_empty() {
            let _ = STEST::from_str_radix("", 10).unwrap_err().kind() == &IntErrorKind::Empty;
        }

        #[test]
        fn from_str_radix_invalid_char() {
            let _ = STEST::from_str_radix("a", 10).unwrap_err().kind() == &IntErrorKind::InvalidDigit;
        }

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 36]")]
        fn from_str_radix_invalid_radix() {
            let _ = STEST::from_str_radix("1234", 37).unwrap();
        }
    }

    crate::test::test_all! {
        testing unsigned;

        #[cfg(feature = "alloc")]
        crate::test::quickcheck_from_to_radix!(stest, radix_be, 256);
        #[cfg(feature = "alloc")]
        crate::test::quickcheck_from_to_radix!(stest, radix_le, 256);
        #[cfg(feature = "alloc")]
        crate::test::quickcheck_from_str_radix!(utest, "+" | "");

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 256]")]
        fn from_radix_be_invalid_radix() {
            let _ = STEST::from_radix_be(&[1], 257);
        }

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 256]")]
        fn from_radix_le_invalid_radix() {
            let _ = STEST::from_radix_le(&[1], 257);
        }

        #[test]
        fn parse_empty() {
            assert_eq!(STEST::from_radix_be(&[], 10), Some(STEST::ZERO));
            assert_eq!(STEST::from_radix_le(&[], 10), Some(STEST::ZERO));
        }
    }

    crate::test::test_all! {
        testing signed;

        #[cfg(feature = "alloc")]
        crate::test::quickcheck_from_str_radix!(itest, "+" | "-");
    }
}

// #[cfg(test)]
// crate::test::test_all_widths_against_old_types! {
//     use crate::test::test_bignum;
//     use crate::test::Radix;

//     test_bignum! {
//         function: <utest>::to_str_radix(a: ref &utest, b: Radix<36>)
//     }
// }
