/// Create constant [`Integer`](crate::Integer) values from native integer literals.
/// 
/// `n!` converts integer literals to [`Integer`](crate::Integer) values at compile time. It supports literals in base 2, 8, 10 and 16:
/// - The prefix `0b` indicates a binary literal (base 2).
/// - The prefix `0o` indicates an octal literal (base 8).
/// - The prefix `0x` indicates a hexadecimal literal (base 16).
/// - Literals are treated as decimal literals (base 10) if no prefix is specified.
/// 
/// `n!` accepts two forms of integer literal as input:
/// 1. Suffix-free, e.g. `n!(0xABCDEF)`. In this case, the const-generic parameters of the created [`Integer`](crate::Integer) are left unspecified, so this must be used in a context where type inference can determine the parameters. For example: `let a: Integer<false, 16> = n!(0xABCDEF);` would be valid, but `let b = n!(0xABCDEF);` would trigger a compile error (unless `b` was subsequently used in a context which allowed for type inference).
/// 2. With a suffix, e.g. `n!(0xabcdefU128w)`. The suffix may be any valid argument to the [`t!`](crate::t) macro. The suffix is referred to as a _type descriptor_, as it specifies the values of the const-generic parameters of the created [`Integer`](crate::Integer). For more information about valid type descriptors, see the documentation of the [`t!`](crate::t) macro.
/// 
/// Invoking `n!` with invalid arguments will also trigger a compile error. This can happen if:
/// - The literal is out of range for the target type (works for inferred types and types specified by a suffix). Note that this will always cause a compile error, regardless of the overflow mode of the type.
/// - The literal contains an invalid digit.
/// - A `-` sign appears at the start of the literal when the type is unsigned, e.g. `n!(-123U256)`.
/// - The suffix is an invalid type descriptor.
/// 
/// 
/// # Examples
/// 
/// ```
/// use bnum::prelude::*; // n! and t! are re-exported in the prelude
/// 
/// let a: t!(U256) = n!(0xABCDEF); // type inferred from context
/// ```
/// 
/// ```
/// use bnum::prelude::*;
/// 
/// let b = n!(123456_I511s); // type specified by the suffix
/// // suffix specifies signed 511-bit integer with saturating overflow behaviour
/// // note that we don't need to define a type alias I511s here
/// ```
/// 
/// The following example will fail to compile, since the compiler is unable to infer the type of the integer:
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// let a = n!(0o7654321);
/// ```
/// 
/// The following example will fail to compile, since the literal is out of range for the specified type:
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// let c = n!(0x1000000U24);
/// ```
/// 
/// The following example will fail to compile, since the literal contains an invalid digit for the specified base:
/// ```compile_fail
/// use bnum::prelude::*;
/// use bnum::types::I256;
/// 
/// let d: I256 = n!(1234A);
/// ```
/// 
/// The following example will fail to compile, since the given type descriptor is invalid:
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// let e = n!(12345U1024x);
/// ```
// TODO: support other prefixes, e.g. 0t for base 3, 0q for base 4
#[macro_export]
macro_rules! n {
    ($literal_str: literal) => {
        const {
            const PARTS: (bool, u32, &[u8], &str) = $crate::__internal::get_negative_radix_digits_type_descriptor(stringify!($literal_str));
            let (negative, radix, digit_bytes, _) = PARTS;
            const TYPE_DESCRIPTOR_BYTES: &str = PARTS.3;
            const PARAMS: (bool, Result<bool, ($crate::__internal::TypeDescriptorError, &str)>, usize, usize, u8) = $crate::__internal::get_integer_params_fallback(TYPE_DESCRIPTOR_BYTES);

            const IMPLICIT: bool = PARAMS.0;
            const N: usize = PARAMS.2;
            const B: usize = PARAMS.3;
            const OM: u8 = PARAMS.4;
            const S: Result<bool, ($crate::__internal::TypeDescriptorError, &str)> = PARAMS.1;

            type Parser = $crate::__internal::IntLiteralParser<{ IMPLICIT }, { match S {
                Ok(s) => s,
                Err(_) => $crate::__internal_panic_type_descriptor_error!(S),
            } }, { N }, { B }, { OM }>; // surround the constants in curly braces as compiler gets confused if there is a type defined elsewhere with the same name (e.g. N, B)

            Parser::parse(negative, radix, digit_bytes)
        }
    };
    // ($ty: ident) => {
    //     $crate::Integer::<{
    //         $crate::__internal::get_signedness(stringify!($ty))
    //     }, {
    //         // we do the error handling here, as the processing of the bit width is the most computationally intensive (need to parse a string to an int, more cases to handle). it's very easy and fast to come up with a sign and overflow mode descriptors that are valid when the type descriptor is valid (the sign and overflow mode descriptors may be incorrect for an invalid type descriptor, but this does not matter as the error handling here will cause a panic anyway)
    //         const PARAMS_RESULT: Result<(bool, usize, u8), ($crate::__internal::TypeDescriptorError, &'static str)> = $crate::__internal::get_integer_params(stringify!($ty));
    //         // compiler doesn't say which const-generic parameter caused the error, so we can put all the error handling in one place, this means we won't forget to cover any of the possible errors
    //         match PARAMS_RESULT {
    //             Ok((_, bw, _)) => $crate::__internal::get_size_params_from_bits(bw).0,
    //             Err(_) => $crate::__internal_panic_type_descriptor_error!(PARAMS_RESULT),
    //         }
    //     }, {
    //         match $crate::__internal::get_integer_params(stringify!($ty)) {
    //             Ok((_, bw, _)) => $crate::__internal::get_size_params_from_bits(bw).1,
    //             _ => 0,
    //         }
    //     }, {
    //         $crate::__internal::get_overflow_mode(stringify!($ty))
    //     }>
    // };
    ($($_: tt)*) => {
        compile_error!("expected integer literal e.g. `0xFFFF`, `123456U1024s`, or `0o7654321_I101w`");
    };
}

/// Create constant [`Float`](crate::Float) values from decimal numeric literals.
///
/// `f!` converts decimal float (or integer) literals to [`Float`](crate::Float) values at compile
/// time, rounding according to the target type's [`RoundingMode`](crate::RoundingMode).
/// Underscores (`_`) may be used freely as digit separators.
///
/// `f!` accepts two forms of literal as input:
/// 1. Suffix-free, e.g. `f!(3.14159)` or `f!(-1_000.5e-2)`. In this case, the const-generic
///    parameters of the created [`Float`](crate::Float) are left unspecified, so this must be used
///    in a context where type inference can determine the parameters.
/// 2. With a float type descriptor suffix, e.g. `f!(3.14159F64)` or `f!(1.5_F16m7z)`. The suffix
///    may be any valid float type descriptor accepted by the [`t!`](crate::t) macro.
///
/// Invoking `f!` with an invalid literal or invalid float type descriptor will trigger a compile
/// error.
///
/// # Examples
///
/// ```
/// use bnum::prelude::*; // f! and t! are re-exported in the prelude
///
/// let a: t!(F64) = f!(3.141592653589793); // type inferred from context
/// let b = f!(1_234.56_e-2_F128p); // type specified by the suffix
/// const C: t!(F16m7z) = f!(-0.125); // evaluated at compile time
/// ```
///
/// The following example will fail to compile, since the compiler is unable to infer the type of
/// the float:
/// ```compile_fail
/// use bnum::prelude::*;
///
/// let a = f!(3.14);
/// ```
///
/// The following example will fail to compile, since the suffix is not a valid float type
/// descriptor:
/// ```compile_fail
/// use bnum::prelude::*;
///
/// let b = f!(3.14U64);
/// ```
///
/// The following example will fail to compile, since non-decimal literals are not supported:
/// ```compile_fail
/// use bnum::prelude::*;
///
/// let c: t!(F64) = f!(0x123);
/// ```
#[cfg(feature = "float")]
#[macro_export]
macro_rules! f {
    ($literal_str: literal) => {
        const {
            const PARTS: (&str, &str) =
                $crate::__internal::split_float_literal_and_type_descriptor(stringify!(
                    $literal_str
                ));
            const VALUE_STR: &str = PARTS.0;
            const TYPE_DESCRIPTOR: &str = PARTS.1;
            const PARAMS: (
                bool,
                Result<usize, ($crate::__internal::TypeDescriptorError, &str)>,
                usize,
                u8,
            ) = $crate::__internal::get_float_params_fallback(TYPE_DESCRIPTOR);

            const IMPLICIT: bool = PARAMS.0;
            const W_RESULT: Result<usize, ($crate::__internal::TypeDescriptorError, &str)> =
                PARAMS.1;
            const MB: usize = PARAMS.2;
            const RM: u8 = PARAMS.3;

            type Parser = $crate::__internal::FloatLiteralParser<
                { IMPLICIT },
                {
                    match W_RESULT {
                        Ok(w) => w,
                        Err(_) => $crate::__internal_panic_type_descriptor_error!(W_RESULT),
                    }
                },
                { MB },
                { RM },
            >;

            Parser::parse(VALUE_STR)
        }
    };
    ($($_: tt)*) => {
        compile_error!(
            "expected float literal e.g. `3.14`, `1.5e-10F64`, or `-1_000.25_F128p`"
        );
    };
}

use crate::{Integer, Uint};
use core::num::IntErrorKind;

#[doc(hidden)]
#[derive(Debug)]
pub enum ParseIntLiteralError {
    OutOfRange,
    NoDigits,
    InvalidDigit,
    UnsignedNegation,
}

// IMPLICIT = true means that parameters are not specified in the literal, so rely on type inference
#[doc(hidden)]
pub struct IntLiteralParser<const IMPLICIT: bool, const S: bool, const N: usize, const B: usize, const OM: u8>;

impl<const S: bool, const N: usize, const B: usize, const OM: u8> IntLiteralParser<true, S, N, B, OM> {
    #[doc(hidden)]
    #[inline]
    pub const fn parse<const R: bool, const M: usize, const A: usize, const OM1: u8>(negative: bool, radix: u32, literal_str_bytes: &[u8]) -> Integer<R, M, A, OM1> {
        Integer::from_literal_str(negative, radix, literal_str_bytes)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> IntLiteralParser<false, S, N, B, OM> {
    #[doc(hidden)]
    #[inline]
    pub const fn parse(negative: bool, radix: u32, literal_str_bytes: &[u8]) -> Integer<S, N, B, OM> {
        Integer::from_literal_str(negative, radix, literal_str_bytes)
    }
}

#[cfg(feature = "float")]
#[doc(hidden)]
pub struct FloatLiteralParser<const IMPLICIT: bool, const W: usize, const MB: usize, const RM: u8>;

#[cfg(feature = "float")]
impl<const W: usize, const MB: usize, const RM: u8> FloatLiteralParser<true, W, MB, RM> {
    #[doc(hidden)]
    #[inline]
    pub const fn parse<const W1: usize, const MB1: usize, const RM1: u8>(
        literal_str: &str,
    ) -> crate::Float<W1, MB1, RM1> {
        crate::Float::from_literal_str(literal_str)
    }
}

#[cfg(feature = "float")]
impl<const W: usize, const MB: usize, const RM: u8> FloatLiteralParser<false, W, MB, RM> {
    #[doc(hidden)]
    #[inline]
    pub const fn parse(literal_str: &str) -> crate::Float<W, MB, RM> {
        crate::Float::from_literal_str(literal_str)
    }
}

#[cfg(feature = "float")]
impl<const W: usize, const MB: usize, const RM: u8> crate::Float<W, MB, RM> {
    #[doc(hidden)]
    #[inline]
    pub const fn from_literal_str(literal_str: &str) -> Self {
        match Self::from_str_const::<true>(literal_str) {
            Ok(f) => f,
            Err(_) => panic!("invalid float literal"),
        }
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    #[doc(hidden)]
    #[inline]
    pub const fn from_literal_str(negative: bool, radix: u32, digit_bytes: &[u8]) -> Self {
        match Self::from_literal_str_checked(negative, radix, digit_bytes) {
            Ok(n) => n,
            Err(err) => match err {
                ParseIntLiteralError::OutOfRange => {
                    panic!("literal out of range for target type")
                }
                ParseIntLiteralError::NoDigits => {
                    panic!("no valid digits found for number")
                }
                ParseIntLiteralError::InvalidDigit => {
                    panic!("integer literal contains invalid digit")
                }
                ParseIntLiteralError::UnsignedNegation => {
                    panic!("cannot apply unary operator `-` to unsigned integer type")
                }
            },
        }
    }

    #[doc(hidden)]
    #[inline]
    pub const fn from_literal_str_checked(negative: bool, radix: u32, digit_bytes: &[u8]) -> Result<Self, ParseIntLiteralError> {
        if digit_bytes.is_empty() {
            return Err(ParseIntLiteralError::NoDigits);
        }
        if negative && !S {
            return Err(ParseIntLiteralError::UnsignedNegation);
        }

        match Uint::from_buf_radix::<true, true, true>(digit_bytes, radix) {
            Ok(uint) => {
                let out = uint.force_sign::<S>();
                if S && negative {
                    // no error iff out is positive or out is Self::MIN, i.e. ...
                    if uint.gt(&Self::MIN.force_sign()) {
                        Err(ParseIntLiteralError::OutOfRange)
                    } else {
                        Ok(out.wrapping_neg()) // needs to be wrapping_neg as we need to handle the Self::MIN case (Self::MIN is mapped to Self:MIN)
                    }
                } else {
                    if out.is_negative_internal() {
                        Err(ParseIntLiteralError::OutOfRange)
                    } else {
                        Ok(out)
                    }
                }
            }
            Err(err) => match err.kind() {
                IntErrorKind::Empty => Err(ParseIntLiteralError::NoDigits),
                IntErrorKind::InvalidDigit => Err(ParseIntLiteralError::InvalidDigit),
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                    Err(ParseIntLiteralError::OutOfRange)
                }
                _ => unreachable!(),
            },
        }
    }
}

#[doc(hidden)]
#[inline]
pub const fn get_negative_radix_digits_type_descriptor(literal_str: &str) -> (bool, u32, &[u8], &str) {
    let bytes = literal_str.as_bytes();
    let negative = !bytes.is_empty() && bytes[0] == b'-';
    let bytes = if negative { bytes.split_at(1).1 } else { bytes };
    let radix = if bytes.len() >= 2 {
        match (bytes[0], bytes[1]) {
            (b'0', b'b') => 2,
            (b'0', b'o') => 8,
            (b'0', b'x') => 16,
            _ => 10,
        }
    } else {
        10
    };
    let bytes = if radix == 10 {
        bytes
    } else {
        bytes.split_at(2).1
    };
    let mut last_digit_index = bytes.len(); // index of the last char in the string which is a digit, chars after this are considered part of the type descriptor
    // start as bytes.len() as there may be no type descriptor
    let mut i = 0; // start at end of string as there will be fewer iterations (assuming the type descriptor is shorter than the digits)
    while i < bytes.len() {
        let c = bytes[i] as char;
        let is_digit = match radix {
            16 => (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F') || c == '_',
            _ => (c >= '0' && c <= '9') || c == '_' ,
        };
        if !is_digit {
            last_digit_index = i;
            break;
        }
        i += 1;
    }
    let (digit_bytes, type_descriptor_bytes) = bytes.split_at(last_digit_index);

    let type_descriptor = unsafe { core::str::from_utf8_unchecked(type_descriptor_bytes) }; // SAFETY: type_descriptor_bytes is a slice of the original literal_str, which is valid UTF-8, so this is valid UTF-8
    (negative, radix, digit_bytes, type_descriptor)
}

#[cfg(feature = "float")]
#[doc(hidden)]
#[inline]
pub const fn split_float_literal_and_type_descriptor(literal_str: &str) -> (&str, &str) {
    let bytes = literal_str.as_bytes();
    let mut i = 0;
    if i < bytes.len() && bytes[i] == b'-' {
        i += 1;
    }
    // Must start with an ASCII decimal digit (rejects string/char/bool literals cleanly).
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        return (literal_str, "");
    }
    // Reject non-decimal integer prefixes (0x, 0b, 0o) as invalid float literals rather than bad suffixes.
    if i + 1 < bytes.len() && bytes[i] == b'0' {
        let next = bytes[i + 1];
        if next == b'x'
            || next == b'X'
            || next == b'b'
            || next == b'B'
            || next == b'o'
            || next == b'O'
        {
            return (literal_str, "");
        }
    }
    // Integer part: ASCII digits and underscores.
    while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
        i += 1;
    }
    // Optional fractional part: '.' followed by ASCII digits and underscores.
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
            i += 1;
        }
    }
    // Optional exponent part: ('e' | 'E') ('+' | '-')? (digit | '_')+
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        i += 1;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'_') {
            i += 1;
        }
    }
    literal_str.split_at(i)
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use crate::{t, n};
    
    #[test]
    fn cases_n_macro() {
        type I256 = t!(I256);
        let a: I256 = n!(0x_ABCDEF_);
        assert_eq!(a.to_str_radix(16), "abcdef");

        let b = n!(1_23_456U511s);
        assert_eq!(b.to_str_radix(10), "123456");

        let c = n!(0o123_45_670U257w);
        assert_eq!(c.to_str_radix(8), "12345670");

        type I24p = t!(I24p);
        let d: I24p = n!(0b101010111100110111_I24p);
        assert_eq!(d.to_str_radix(2), "101010111100110111");
    }
}

#[cfg(test)]
mod runtime_int_literal_tests {
    use super::*;
    use crate::t;

    #[test]
    fn test_get_negative_radix_digits_type_descriptor() {
        assert_eq!(
            get_negative_radix_digits_type_descriptor("0"),
            (false, 10, b"0".as_slice(), "")
        );
        assert_eq!(
            get_negative_radix_digits_type_descriptor("-42I8"),
            (true, 10, b"42".as_slice(), "I8")
        );
        assert_eq!(
            get_negative_radix_digits_type_descriptor("0b1010_U16"),
            (false, 2, b"1010_".as_slice(), "U16")
        );
        assert_eq!(
            get_negative_radix_digits_type_descriptor("0o77_U32w"),
            (false, 8, b"77_".as_slice(), "U32w")
        );
        assert_eq!(
            get_negative_radix_digits_type_descriptor("-0xAb_Cd_I64s"),
            (true, 16, b"Ab_Cd_".as_slice(), "I64s")
        );
        assert_eq!(
            get_negative_radix_digits_type_descriptor("12_34"),
            (false, 10, b"12_34".as_slice(), "")
        );
    }

    #[test]
    fn test_from_literal_str_checked_and_parsers() {
        type U8 = t!(U8);
        type I8 = t!(I8);

        assert_eq!(
            U8::from_literal_str_checked(false, 10, b"2_5_5").unwrap(),
            U8::MAX
        );
        assert_eq!(
            U8::from_literal_str_checked(false, 10, b"0_0_0").unwrap(),
            U8::ZERO
        );
        assert_eq!(
            I8::from_literal_str_checked(true, 10, b"128").unwrap(),
            I8::MIN
        );
        assert_eq!(
            I8::from_literal_str_checked(true, 10, b"42").unwrap(),
            I8::from_le_bytes((-42i8).to_le_bytes())
        );
        assert_eq!(
            I8::from_literal_str_checked(false, 16, b"7_F").unwrap(),
            I8::MAX
        );

        assert!(matches!(
            U8::from_literal_str_checked(false, 10, b""),
            Err(ParseIntLiteralError::NoDigits)
        ));
        assert!(matches!(
            U8::from_literal_str_checked(true, 10, b"1"),
            Err(ParseIntLiteralError::UnsignedNegation)
        ));
        assert!(matches!(
            U8::from_literal_str_checked(false, 10, b"12z"),
            Err(ParseIntLiteralError::InvalidDigit)
        ));
        assert!(matches!(
            I8::from_literal_str_checked(false, 10, b"128"),
            Err(ParseIntLiteralError::OutOfRange)
        ));
        assert!(matches!(
            I8::from_literal_str_checked(true, 10, b"129"),
            Err(ParseIntLiteralError::OutOfRange)
        ));
        // Delayed overflow in both power-of-two and non-power-of-two radixes with underscores
        assert!(matches!(
            U8::from_literal_str_checked(false, 16, b"1_00"),
            Err(ParseIntLiteralError::OutOfRange)
        ));
        assert!(matches!(
            U8::from_literal_str_checked(false, 10, b"1_000"),
            Err(ParseIntLiteralError::OutOfRange)
        ));

        // IntLiteralParser runtime paths (implicit and explicit)
        let p_imp: U8 = IntLiteralParser::<true, false, 8, 0, 0>::parse(false, 10, b"42");
        let p_exp: t!(U8w) = IntLiteralParser::<false, false, 1, 0, 0>::parse(false, 16, b"2A");
        assert_eq!(p_imp.to_bytes(), p_exp.to_bytes());
    }

    #[test]
    #[should_panic(expected = "literal out of range")]
    fn test_from_literal_str_panics_out_of_range() {
        let _ = <t!(U8)>::from_literal_str(false, 10, b"256");
    }

    #[test]
    #[should_panic(expected = "no valid digits")]
    fn test_from_literal_str_panics_no_digits() {
        let _ = <t!(U8)>::from_literal_str(false, 10, b"");
    }

    #[test]
    #[should_panic(expected = "invalid digit")]
    fn test_from_literal_str_panics_invalid_digit() {
        let _ = <t!(U8)>::from_literal_str(false, 10, b"9a");
    }

    #[test]
    #[should_panic(expected = "unsigned integer type")]
    fn test_from_literal_str_panics_unsigned_negation() {
        let _ = <t!(U8)>::from_literal_str(true, 10, b"1");
    }
}

#[cfg(all(test, feature = "float"))]
mod float_tests {
    use super::split_float_literal_and_type_descriptor;
    use crate::{t, Float, RoundingMode, Uint};
    use crate::types::{F16, F32, F64, F80, F128, F256};

    #[test]
    fn test_split_float_literal() {
        assert_eq!(split_float_literal_and_type_descriptor("3.14"), ("3.14", ""));
        assert_eq!(split_float_literal_and_type_descriptor("-3.14"), ("-3.14", ""));
        assert_eq!(split_float_literal_and_type_descriptor("3.14F64"), ("3.14", "F64"));
        assert_eq!(
            split_float_literal_and_type_descriptor("-1_000.25_5e-1_0_F128m112tp"),
            ("-1_000.25_5e-1_0_", "F128m112tp")
        );
        assert_eq!(split_float_literal_and_type_descriptor("42F32z"), ("42", "F32z"));
        assert_eq!(split_float_literal_and_type_descriptor("1."), ("1.", ""));
        assert_eq!(split_float_literal_and_type_descriptor("0x123"), ("0x123", ""));
        assert_eq!(split_float_literal_and_type_descriptor("0X123"), ("0X123", ""));
        assert_eq!(split_float_literal_and_type_descriptor("0b101"), ("0b101", ""));
        assert_eq!(split_float_literal_and_type_descriptor("0B101"), ("0B101", ""));
        assert_eq!(split_float_literal_and_type_descriptor("0o77"), ("0o77", ""));
        assert_eq!(split_float_literal_and_type_descriptor("0O77"), ("0O77", ""));
        assert_eq!(split_float_literal_and_type_descriptor("1.5e+2F64"), ("1.5e+2", "F64"));
        assert_eq!(split_float_literal_and_type_descriptor("1.5E-2F32"), ("1.5E-2", "F32"));
        assert_eq!(split_float_literal_and_type_descriptor("\"3.14\""), ("\"3.14\"", ""));
    }

    #[test]
    fn test_float_literal_parser_runtime() {
        use super::FloatLiteralParser;

        let a: F64 = FloatLiteralParser::<true, 8, 52, 0>::parse("1_234.5_6e-2");
        let b: F64 = FloatLiteralParser::<false, 8, 52, 0>::parse("1_234.5_6e-2");
        assert_eq!(a.to_bits(), b.to_bits());
        assert_eq!(a.to_bits(), F64::from(12.3456_f64).to_bits());

        // Exercise runtime Stage 3 midpoint with underscores
        let tie = F64::from_literal_str("1.000000000000000_11102230246251565404236316680908203125");
        assert_eq!(tie.to_bits(), F64::ONE.to_bits());
        let above = F64::from_literal_str("1.000000000000000_111022302462515654042363166809082031250000000000000001");
        assert_eq!(above.to_bits(), F64::ONE.to_bits().add(Uint::ONE));
    }

    #[test]
    #[should_panic(expected = "invalid float literal")]
    fn test_float_from_literal_str_invalid_panics() {
        let _ = F64::from_literal_str("not_a_float");
    }

    #[test]
    fn cases_f_macro_const_and_inference() {
        // Compile-time constant evaluation with type inference
        const PI_F64: F64 = f!(3.14159265358979323846);
        const PI_F32: F32 = f!(3.14159265358979323846);
        const NEG_ZERO_F64: F64 = f!(-0.0);
        const INT_LIKE_F64: F64 = f!(1_000_000);
        const TRAILING_DOT_F64: F64 = f!(42.);
        const UNDERSCORE_EXP_F64: F64 = f!(-1_2.3_4_e+1_0);

        assert_eq!(PI_F64.to_bits(), F64::from(core::f64::consts::PI).to_bits());
        assert_eq!(PI_F32.to_bits(), F32::from(core::f32::consts::PI).to_bits());
        assert_eq!(NEG_ZERO_F64.to_bits(), F64::NEG_ZERO.to_bits());
        assert_eq!(INT_LIKE_F64.to_bits(), F64::from(1_000_000.0_f64).to_bits());
        assert_eq!(TRAILING_DOT_F64.to_bits(), F64::from(42.0_f64).to_bits());
        assert_eq!(
            UNDERSCORE_EXP_F64.to_bits(),
            F64::from(-12.34e10_f64).to_bits()
        );

        // Explicit type descriptor suffixes (no type annotation needed)
        let a = f!(3.14159265358979323846_F64);
        assert_eq!(a.to_bits(), PI_F64.to_bits());

        let b = f!(1_024.5F32);
        assert_eq!(b.to_bits(), F32::from(1024.5_f32).to_bits());

        let c = f!(-0.5_F16);
        assert_eq!(c.to_bits(), F16::from_str("-0.5").unwrap().to_bits());

        let d = f!(123F80);
        assert_eq!(d.to_bits(), F80::from_str("123").unwrap().to_bits());

        let e = f!(1.5_e-10_F128);
        assert_eq!(e.to_bits(), F128::from_str("1.5e-10").unwrap().to_bits());

        let g = f!(0.25F256);
        assert_eq!(g.to_bits(), F256::QUARTER.to_bits());

        // Custom mantissa width (bfloat16) and non-standard IEEE fallback width (F24, F512)
        let bf16 = f!(1.5F16m7);
        assert_eq!(
            bf16.to_bits(),
            Float::<2, 7>::from_str("1.5").unwrap().to_bits()
        );

        let f24 = f!(2.5F24);
        assert_eq!(
            f24.to_bits(),
            Float::<3, 18>::from_str("2.5").unwrap().to_bits()
        );

        let f512 = f!(0.125F512);
        assert_eq!(
            f512.to_bits(),
            Float::<64, 488>::from_str("0.125").unwrap().to_bits()
        );
    }

    #[test]
    fn cases_f_macro_rounding_modes_and_exact_midpoints() {
        // Exact halfway tie in F16: 1 + 2^-11 = 1.00048828125 (even lower neighbor = 1.0)
        let one = F16::ONE.to_bits();
        let one_p1 = one.add(Uint::ONE);
        let neg_one = F16::NEG_ONE.to_bits();
        let neg_one_p1 = neg_one.add(Uint::ONE);

        // Positive even tie
        assert_eq!(f!(1.000_488_281_25F16).to_bits(), one);
        assert_eq!(f!(1.000_488_281_25F16te).to_bits(), one);
        assert_eq!(f!(1.000_488_281_25F16ta).to_bits(), one_p1);
        assert_eq!(f!(1.000_488_281_25F16p).to_bits(), one_p1);
        assert_eq!(f!(1.000_488_281_25F16n).to_bits(), one);
        assert_eq!(f!(1.000_488_281_25F16z).to_bits(), one);
        assert_eq!(f!(1.000_488_281_25F16a).to_bits(), one_p1);
        assert_eq!(f!(1.000_488_281_25F16o).to_bits(), one_p1);
        assert_eq!(f!(1.000_488_281_25F16tp).to_bits(), one_p1);
        assert_eq!(f!(1.000_488_281_25F16tn).to_bits(), one);
        assert_eq!(f!(1.000_488_281_25F16tz).to_bits(), one);
        assert_eq!(f!(1.000_488_281_25F16to).to_bits(), one_p1);

        // Negative even tie
        assert_eq!(f!(-1.00048828125F16tp).to_bits(), neg_one);
        assert_eq!(f!(-1.00048828125F16tn).to_bits(), neg_one_p1);
        assert_eq!(f!(-1.00048828125F16p).to_bits(), neg_one);
        assert_eq!(f!(-1.00048828125F16n).to_bits(), neg_one_p1);

        // Overflow & underflow obey rounding mode
        assert_eq!(f!(1e100F16).to_bits(), F16::INFINITY.to_bits());
        assert_eq!(f!(1e100F16z).to_bits(), F16::MAX.to_bits());
        assert_eq!(f!(1e100F16n).to_bits(), F16::MAX.to_bits());
        assert_eq!(f!(-1e100F16p).to_bits(), F16::MIN.to_bits());
        assert_eq!(f!(1e-100F16p).to_bits(), F16::MIN_POSITIVE_SUBNORMAL.to_bits());
        assert_eq!(f!(1e-100F16z).to_bits(), F16::ZERO.to_bits());

        // Type inference preserves rounding mode from target type
        let inferred_p: t!(F16m10p) = f!(1.000_488_281_25);
        assert_eq!(inferred_p.to_bits(), one_p1);
        assert_eq!(
            <t!(F16m10p)>::ROUNDING_MODE,
            RoundingMode::TowardPositive
        );

        // Faithful and stochastic produce valid bracketed results at compile time
        let faithful = f!(1.00048828125F16f);
        assert!(faithful.to_bits() == one || faithful.to_bits() == one_p1);
        let stoch = f!(1.00048828125F16s);
        assert!(stoch.to_bits() == one || stoch.to_bits() == one_p1);

        // Exact Stage 3 midpoint in F64 (2^-53 + 1 = 1 + half ULP at 1.0)
        const F64_HALF_ULP_TIE: F64 =
            f!(1.000000000000000_11102230246251565404236316680908203125);
        const F64_JUST_ABOVE_TIE: F64 =
            f!(1.000000000000000_111022302462515654042363166809082031250000000000000001);
        assert_eq!(F64_HALF_ULP_TIE.to_bits(), F64::ONE.to_bits());
        assert_eq!(
            F64_JUST_ABOVE_TIE.to_bits(),
            F64::ONE.to_bits().add(Uint::ONE)
        );
    }
}
