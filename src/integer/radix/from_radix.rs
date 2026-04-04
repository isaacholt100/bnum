use crate::errors::ParseIntError;
use crate::integer::radix::assert_range;
use crate::{Byte, Integer, Uint};
use core::num::IntErrorKind;

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

// iterates through the digits in the buffer, starting with the most significant digit
// the BE parameter indicates whether the digits in the buffer are in big-endian order (i.e. whether to iterate from start or end of buffer)
struct BufferDigitsIter<'a, const SKIP_UNDERSCORES: bool, const ASCII: bool, const BE: bool> {
    buf: &'a [u8],
    index: usize,
}

impl<'a, const SKIP_UNDERSCORES: bool, const ASCII: bool, const BE: bool>
    BufferDigitsIter<'a, SKIP_UNDERSCORES, ASCII, BE>
{
    #[inline]
    const fn new(buf: &'a [u8]) -> Self {
        Self { buf, index: 0 }
    }
}

impl<'a, const SKIP_UNDERSCORES: bool, const ASCII: bool, const BE: bool>
    BufferDigitsIter<'a, SKIP_UNDERSCORES, ASCII, BE>
{
    #[inline]
    const fn next(&mut self) -> Option<u8> {
        while self.index < self.buf.len() {
            let idx = if BE {
                self.index
            } else {
                self.buf.len() - 1 - self.index
            }; // we want to read most significant digits first. so if radix digits are given in big-endian, start from the index 0, otherwise start from the end
            let b = self.buf[idx];
            self.index += 1;
            if SKIP_UNDERSCORES && b == b'_' {
                continue;
            }
            return Some(byte_to_digit::<ASCII>(b));
        }
        None
    }
}

macro_rules! impl_desc {
    () => {
        "Methods which convert to integers from strings and lists of digits in a given radix (base)."
    };
}

#[doc = impl_desc!()]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    const fn from_buf_radix_power_of_two<
        const SKIP_UNDERSCORES: bool,
        const ASCII: bool,
        const BE: bool,
        const EXACT: bool,
    >(
        buf: &[u8],
        radix: u32,
    ) -> Result<Self, ParseIntError> {
        let radix_log2 = radix.ilog2() as usize;
        let mut radix_digits = BufferDigitsIter::<SKIP_UNDERSCORES, ASCII, BE>::new(buf);

        let mut out = Self::ZERO;
        let mut length = buf.len();
        let mut only_leading_zeros = true;
        let mut overflow = false;
        let mut bit_width = 0;
        let mut i = 0;
        while let Some(digit) = radix_digits.next() {
            if digit >= radix as u8 {
                // invalid digit
                return Err(ParseIntError {
                    kind: IntErrorKind::InvalidDigit,
                });
            }
            if only_leading_zeros {
                if digit == 0 {
                    length -= 1;
                    continue;
                }
                only_leading_zeros = false;
                if SKIP_UNDERSCORES {
                    let mut num_underscores = 0;
                    let mut i = buf.len() - length;
                    while i < buf.len() {
                        if buf[i] == b'_' {
                            num_underscores += 1;
                        }
                        i += 1;
                    }
                    length -= num_underscores;
                }
                if length == 0 {
                    return Ok(Self::ZERO);
                }
                let target_bit_width = (length - 1) * radix_log2 + digit.ilog2() as usize + 1;
                bit_width = digit.ilog2() as usize + 1;
                overflow = target_bit_width > Self::BITS as usize;
                i = length;
            }

            if !overflow {
                i -= 1;
                // we are setting bits from position i * radix_log2 to i * radix_log2 + radix_log2 - 1 (inclusive)
                let byte_index = (i * radix_log2) / Byte::BITS as usize;

                let bit_shift = (i * radix_log2) % Byte::BITS as usize;

                if !EXACT && bit_shift != 0 && byte_index != N - 1 {
                    // some of the bits of digit may have been shifted out
                    // these bits are...
                    let carry_bits = digit >> (Byte::BITS as usize - bit_shift);
                    out.bytes[byte_index + 1] |= carry_bits;
                    // if byte_index == N - 1, then carry_bits = 0, as we have already checked for overflow
                }

                out.bytes[byte_index] |= digit << bit_shift;
            } else {
                if !SKIP_UNDERSCORES && bit_width > Self::BITS as usize {
                    
                    return Err(ParseIntError {
                        kind: IntErrorKind::PosOverflow,
                    });
                }
                bit_width += radix_log2; // increment after the check, as we already initialised bit_width as the bit width of the first non-zero digit
            }
        }
        if SKIP_UNDERSCORES && overflow { // we didn't return overflow error if parsing a literal, so return it now. guaranteed to have an overflow error if SKIP_UNDERSCORES is true
            return Err(ParseIntError {
                kind: IntErrorKind::PosOverflow,
            });
        }

        Ok(out)
    }

    const fn from_buf_radix_non_power_of_two<
        const SKIP_UNDERSCORES: bool,
        const ASCII: bool,
        const BE: bool,
    >(
        buf: &[u8],
        radix: u32,
    ) -> Result<Self, ParseIntError> {
        let mut radix_digits = BufferDigitsIter::<SKIP_UNDERSCORES, ASCII, BE>::new(buf);

        let mut out = Self::ZERO;
        let mut overflow = false;

        while let Some(digit) = radix_digits.next() {
            if digit >= radix as u8 {
                // invalid digit
                return Err(ParseIntError {
                    kind: IntErrorKind::InvalidDigit,
                });
            }
            let (new_out, o) = out.mul_u128_digit(radix as u128);
            overflow |= o;
            if !SKIP_UNDERSCORES && overflow {
                return Err(ParseIntError {
                    kind: IntErrorKind::PosOverflow,
                });
            }
            out = new_out;
            match out.checked_add(Self::from_byte(digit)) { // checked_add is necessary here
                Some(n) => out = n,
                None => {
                    if SKIP_UNDERSCORES {
                        overflow = true; // delay returning overflow error when parsing literals as want to check if every digit is valid firstf
                    } else {
                        return Err(ParseIntError {
                            kind: IntErrorKind::PosOverflow,
                        });
                    }
                }
            };
        }
        if SKIP_UNDERSCORES && overflow { // if we get to this stage (if SKIP_UNDERSCORES is true), then there is overflow and there were no invalid digits, so return overflow error
            return Err(ParseIntError {
                kind: IntErrorKind::PosOverflow,
            });
        }
        Ok(out)
    }

    pub(crate) const fn from_buf_radix<const SKIP_UNDERSCORES: bool, const ASCII: bool, const BE: bool>(
        buf: &[u8],
        radix: u32,
    ) -> Result<Self, ParseIntError> {
        match radix {
            2 | 4 | 16 | 256 => {
                Self::from_buf_radix_power_of_two::<SKIP_UNDERSCORES, ASCII, BE, true>(buf, radix)
            }
            8 | 32 | 64 | 128 => {
                Self::from_buf_radix_power_of_two::<SKIP_UNDERSCORES, ASCII, BE, false>(buf, radix)
            }
            _ => Self::from_buf_radix_non_power_of_two::<SKIP_UNDERSCORES, ASCII, BE>(buf, radix),
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

        crate::helpers::ok!(Self::from_buf_radix::<false, false, true>(
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

        crate::helpers::ok!(Self::from_buf_radix::<false, false, false>(
            buf, radix
        ))
    }
}

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
    /// assert_eq!(U512::from_ascii(b"+10"), Ok(n!(10U512)));
    /// assert_eq!(I512::from_ascii(b"-1234"), Ok(n!(-1234I512)));
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
        match Uint::from_buf_radix::<false, true, true>(src, radix) {
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
            }
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow if S && negative => Err(ParseIntError {
                    kind: IntErrorKind::NegOverflow,
                }),
                _ => Err(err),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use core::num::IntErrorKind;
    use core::str::FromStr;
    use alloc::string::ToString;

    #[cfg(feature = "alloc")]
    macro_rules! quickcheck_from_str {
        ($TestType: ty) => {
            quickcheck::quickcheck! {
                fn quickcheck_from_str(n: $TestType) -> bool {
                    use crate::alloc::string::ToString;
                    use core::str::FromStr;

                    let s = n.to_string();
                    let (actual, expected) = crate::test::results!(<$TestType>::from_str(&s));

                    actual == expected
                }
            }
        };
    }

    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <STest>::from_str,
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

        #[cfg(nightly)] // since int_from_ascii not stable yet
        test_bignum! {
            function: <STest>::from_ascii,
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

        #[cfg(nightly)] // since int_from_ascii not stable yet
        test_bignum! {
            function: <STest>::from_ascii_radix,
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
            function: <STest>::from_str_radix,
            cases: [
                ("+af7345asdofiuweor", 35u32),
                ("+945hhdgi73945hjdfj", 32u32),
                ("+3436847561345343455", 9u32),
                ("+affe758457bc345540ac399", 16u32),
                ("+", 10u32),
                ("-", 10u32),
                ("+affe758457bc345540ac39929334534ee34579234795", 17u32),
                ("+3777777777777777777777777777777777777777777", 8u32),
                ("+37777777777777777777777777777777777777777761", 8u32),
                ("+1777777777777777777777", 8u32),
                ("+17777777777777777777773", 8u32),
                ("+2000000000000000000000", 8u32),
                ("-234598734", 10u32),
                ("234ab", 16u32),
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
        quickcheck_from_str!(STest);

        #[test]
        fn from_str_radix_empty() {
            assert_eq!(STest::from_str_radix("", 10).unwrap_err().kind(), &IntErrorKind::Empty);

            let test_msg = STest::from_str_radix("", 10).unwrap_err().to_string();
            assert!(test_msg.contains("attempt to parse integer from empty string"));
        }

        #[test]
        fn from_str_radix_invalid_digit() {
            assert_eq!(STest::from_str_radix("a", 10).unwrap_err().kind(), &IntErrorKind::InvalidDigit);

            let test_msg = STest::from_str_radix("a", 10).unwrap_err().to_string();
            assert!(test_msg.contains("attempt to parse integer from string containing invalid digit"));
        }

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 36]")]
        fn from_str_radix_invalid_radix_37() {
            let _ = STest::from_str_radix("1234", 37).unwrap();
        }

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 36]")]
        fn from_str_radix_invalid_radix_0() {
            let _ = STest::from_str_radix("1234", 0).unwrap();
        }

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 36]")]
        fn from_str_radix_invalid_radix_1() {
            let _ = STest::from_str_radix("1234", 1).unwrap();
        }

        #[test]
        fn from_str_radix_pos_overflow() {
            let large_literal = format!("{}0", STest::MAX); // this is 10x the max value so should overflow
            assert_eq!(STest::from_str_radix(&large_literal, 10).unwrap_err().kind(), &IntErrorKind::PosOverflow);

            let test_msg = STest::from_str_radix(&large_literal, 10).unwrap_err().to_string();
            assert!(test_msg.contains("attempt to parse integer too large to be represented by the target type"));
        }
    }

    #[cfg(feature = "alloc")]
    macro_rules! quickcheck_from_str_radix {
        { $TestType: ty, $sign1: literal | $sign2: literal } => {
            quickcheck::quickcheck! {
                fn quickcheck_from_str_radix(buf: [u8; <$TestType>::BITS as usize / 4], radix: crate::test::Radix<36>, leading_sign: bool) -> quickcheck::TestResult {
                    use alloc::string::String;

                    let radix = radix.0;

                    fn byte_to_char(b: u8) -> char {
                        if b < 10 {
                            (b + 48) as char
                        } else {
                            (b + 87) as char
                        }
                    }

                    let leading_sign = if leading_sign {
                        $sign1
                    } else {
                        $sign2
                    };

                    let mut s2 = buf.into_iter().map(|b| byte_to_char(b % radix as u8)).collect::<String>();
                    s2.insert_str(0, leading_sign);

                    let (actual, expected) = crate::test::results!(<$TestType>::from_str_radix(&s2, radix as u32));

                    quickcheck::TestResult::from_bool(actual == expected)
                }
            }
        }
    }

    crate::test::test_all! {
        testing unsigned;

        #[cfg(feature = "alloc")]
        quickcheck_from_str_radix!(UTest, "+" | "");

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 256]")]
        fn from_radix_be_invalid_radix() {
            let _ = UTest::from_radix_be(&[1], 257);
        }

        #[test]
        #[should_panic(expected = "Radix must be in range [2, 256]")]
        fn from_radix_le_invalid_radix() {
            let _ = UTest::from_radix_le(&[1], 257);
        }

        #[test]
        fn parse_empty() {
            assert_eq!(UTest::from_radix_be(&[], 10), Some(UTest::ZERO));
            assert_eq!(UTest::from_radix_le(&[], 10), Some(UTest::ZERO));
        }
    }

    crate::test::test_all! {
        testing signed;

        #[test]
        fn from_str_radix_neg_overflow() {
            let large_literal = format!("-{}0", STest::MAX); // this is -10x the max value so should have negative overflow
            assert_eq!(STest::from_str_radix(&large_literal, 10).unwrap_err().kind(), &IntErrorKind::NegOverflow);

            let test_msg = STest::from_str_radix(&large_literal, 10).unwrap_err().to_string();

            assert!(test_msg.contains("attempt to parse integer too small to be represented by the target type"));
        }

        #[cfg(feature = "alloc")]
        quickcheck_from_str_radix!(ITest, "+" | "-");
    }

    // TODO: custom bit width tests for from_str_radix
}
