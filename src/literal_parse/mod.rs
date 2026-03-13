mod digits;
mod type_descriptor;
mod panic;

pub use digits::*;
pub use type_descriptor::*;
pub use panic::*;

/// Returns a concrete instantiation of the [`Integer`](crate::Integer) type with the specified const-generic parameters from a type descriptor.
/// 
/// The `nt!` macro takes a type descriptor (which is an identifier fragment) and simply outputs [`Integer<S, N, B, OM>`](crate::Integer), where the const-generic parameters `S`, `N`, `B` and `OM` are determined from the type descriptor.
/// 
/// A type descriptor has the following format: `<sign><bit_width><overflow_mode>?`:
/// - `<sign>` is either `I` (specifying a signed integer) or `U` (specifying an unsigned integer).
/// - `<bit_width>` is a decimal integer specifying the bit width of the integer type. The bit width must be at least `2` and at most `2^32 - 1`.
/// - `<overflow_mode>` is optional, and if specified must be one of:
///     - `w` for wrapping overflow mode ([`OverflowMode::Wrapping`](crate::OverflowMode::Wrapping)).
///     - `p` for panicking overflow mode ([`OverflowMode::Panicking`](crate::OverflowMode::Panicking)).
///     - `s` for saturating overflow mode ([`OverflowMode::Saturating`](crate::OverflowMode::Saturating)).
///     
///     If `<overflow_mode>` is omitted, the default overflow mode ([`OverflowMode::DEFAULT`](crate::OverflowMode::DEFAULT)) is used.
/// 
/// If the given type descriptor is not in this format, a compile error will be triggered when the type is used (when the type is unused, no compile-error will be triggered).
/// 
/// If you want to create [`Integer`](crate::Integer) values rather than types, use the [`n!`](crate::n) macro.
/// 
/// # Examples
/// 
/// ```
/// use bnum::prelude::*;
/// 
/// type MyInt = nt!(I259p); // signed 259-bit integer with panicking overflow mode
/// type MyUint = nt!(U633); // unsigned 633-bit integer with default overflow mode
/// type MyInt2 = nt!(I538s); // signed 538-bit integer with saturating overflow mode
/// type MyUint2 = nt!(U24w); // unsigned 24-bit integer with wrapping overflow mode
/// ```
/// 
/// The following examples will fail to compile, since the type descriptor is invalid. Note the type must be used in order to trigger the compile error.
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// type InvalidType = nt!(A256); // invalid sign descriptor
/// 
/// dbg!(InvalidType::BITS);
/// ```
/// 
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// type InvalidType2 = nt!(I1); // bit width too small
/// 
/// dbg!(InvalidType2::BITS);
/// ```
/// 
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// type InvalidType3 = nt!(U1024x); // invalid overflow mode descriptor
/// 
/// dbg!(InvalidType3::BITS);
/// ```
#[macro_export]
macro_rules! nt {
    ($ty: ident) => {
        $crate::Integer::<{
            $crate::literal_parse::get_signedness(stringify!($ty))
        }, {
            // we do the error handling here, as the processing of the bit width is the most computationally intensive (need to parse a string to an int, more cases to handle). it's very easy and fast to come up with a sign and overflow mode descriptors that are valid when the type descriptor is valid (the sign and overflow mode descriptors may be incorrect for an invalid type descriptor, but this does not matter as the error handling here will cause a panic anyway)
            const PARAMS_RESULT: Result<(bool, usize, u8), ($crate::literal_parse::TypeDescriptorError, &'static str)> = $crate::literal_parse::get_integer_params(stringify!($ty));
            // compiler doesn't say which const-generic parameter caused the error, so we can put all the error handling in one place, this means we won't forget to cover any of the possible errors
            match PARAMS_RESULT {
                Ok((_, bw, _)) => $crate::literal_parse::get_size_params_from_bits(bw).0,
                Err(_) => $crate::panic_type_descriptor_error!(PARAMS_RESULT),
            }
        }, {
            match $crate::literal_parse::get_integer_params(stringify!($ty)) {
                Ok((_, bw, _)) => $crate::literal_parse::get_size_params_from_bits(bw).1,
                _ => 0,
            }
        }, {
            $crate::literal_parse::get_overflow_mode(stringify!($ty))
        }>
    };
    ($($_: tt)*) => {
        compile_error!("expected integer type descriptor, e.g. `I256` or `U512w`");
    };
}

// inside a macro not a function, so that the panic happens at the call site
#[doc(hidden)]
#[macro_export]
macro_rules! from_literal_str {
    ($literal_str: expr, $($ty: tt)+) => { // this and not $ty: ty, since for some reason Rust needs to the generic params provided when we wrap in <...>
        match $($ty)+::from_literal_str_checked($literal_str) {
            Ok(n) => n,
            Err(err) => match err {
                $crate::literal_parse::ParseIntLiteralError::OutOfRange => {
                    panic!("literal out of range for target type")
                }
                $crate::literal_parse::ParseIntLiteralError::NoDigits => {
                    panic!("no valid digits found for number")
                }
                $crate::literal_parse::ParseIntLiteralError::InvalidDigit => {
                    panic!("integer literal contains invalid digit")
                }
                $crate::literal_parse::ParseIntLiteralError::UnsignedNegation => {
                    panic!("cannot apply unary operator `-` to unsigned integer type")
                }
            },
        }
    };
}

/// Create constant [`Integer`](crate::Integer) values from native integer literals.
/// 
/// The `n!` macro converts integer literals to [`Integer`](crate::Integer) values at compile time. It supports a similar integer literal syntax as Rust's built-in integer types:
/// - The prefix `0b` indicates a binary literal (base 2).
/// - The prefix `0o` indicates an octal literal (base 8).
/// - The prefix `0x` indicates a hexadecimal literal (base 16).
/// - Literals are treated as decimal literals (base 10) if no prefix is specified.
/// 
/// `n!` accepts two forms of integer literal as input:
/// 1. Suffix-free, e.g. `n!(0xABCDEF)`. In this case, the const-generic parameters of the created [`Integer`](crate::Integer) are left unspecified, so this must be used in a context where type inference can determine the parameters. For example: `let a: Integer<false, 16> = n!(0xABCDEF);` would be valid, but `let b = n!(0xABCDEF);` would trigger a compile error (unless `b` was subsequently used in a context which allowed for type inference).
/// 2. With a suffix, e.g. `n!(0xabcdefU128w)`. The suffix may be any valid argument to the [`nt!`](crate::nt) macro. The suffix is referred to as a _type descriptor_, as it specifies the const-generic parameters of the created [`Integer`](crate::Integer). For more information about valid type descriptors, see the documentation for the [`nt!`](crate::nt) macro.
/// 
/// Invoking `n!` with invalid arguments will also trigger a compile error. This can happen if:
/// - The literal is out of range for the target type (works for inferred types and types specified by a suffix). Note that this will always cause a compile error, regardless of the overflow mode of the type.
/// - The literal contains an invalid digits.
/// - A `-` sign appears at the start of the literal when the type is unsigned, e.g. `n!(-123U256)`.
/// - The suffix is an invalid type descriptor.
/// 
/// 
/// # Examples
/// 
/// ```
/// use bnum::prelude::*; // n! is re-exported in the prelude
/// 
/// let a: Integer<false, 16> = n!(0xABCDEF); // type inferred from context
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
// TODO: support other prefixes, e.g. 0t for base 3, 0q for base 4, 0z for base 36, etc.
#[macro_export]
macro_rules! n {
    ($literal_str: literal) => {
        const {
            const PARTS: (bool, u32, &[u8], &str) = $crate::literal_parse::get_negative_radix_digits_type_descriptor(stringify!($literal_str));
            let (negative, radix, digit_bytes, _) = PARTS;
            const TYPE_DESCRIPTOR_BYTES: &str = PARTS.3;
            const PARAMS: (bool, Result<bool, ($crate::literal_parse::TypeDescriptorError, &str)>, usize, usize, u8) = $crate::literal_parse::get_integer_params_fallback(TYPE_DESCRIPTOR_BYTES);

            const IMPLICIT: bool = PARAMS.0;
            const N: usize = PARAMS.2;
            const B: usize = PARAMS.3;
            const OM: u8 = PARAMS.4;
            const S: Result<bool, ($crate::literal_parse::TypeDescriptorError, &str)> = PARAMS.1;

            type Parser = $crate::literal_parse::IntLiteralParser<{ IMPLICIT }, { match S {
                Ok(s) => s,
                Err(_) => $crate::panic_type_descriptor_error!(S),
            } }, { N }, { B }, { OM }>; // surround the constants in curly braces as compiler gets confused if there is a type defined elsewhere with the same name (e.g. N, B)

            Parser::parse(negative, radix, digit_bytes)
        }
    };
    ($ty: ident) => {
        $crate::Integer::<{
            $crate::literal_parse::get_signedness(stringify!($ty))
        }, {
            // we do the error handling here, as the processing of the bit width is the most computationally intensive (need to parse a string to an int, more cases to handle). it's very easy and fast to come up with a sign and overflow mode descriptors that are valid when the type descriptor is valid (the sign and overflow mode descriptors may be incorrect for an invalid type descriptor, but this does not matter as the error handling here will cause a panic anyway)
            const PARAMS_RESULT: Result<(bool, usize, u8), ($crate::literal_parse::TypeDescriptorError, &'static str)> = $crate::literal_parse::get_integer_params(stringify!($ty));
            // compiler doesn't say which const-generic parameter caused the error, so we can put all the error handling in one place, this means we won't forget to cover any of the possible errors
            match PARAMS_RESULT {
                Ok((_, bw, _)) => $crate::literal_parse::get_size_params_from_bits(bw).0,
                Err(_) => $crate::panic_type_descriptor_error!(PARAMS_RESULT),
            }
        }, {
            match $crate::literal_parse::get_integer_params(stringify!($ty)) {
                Ok((_, bw, _)) => $crate::literal_parse::get_size_params_from_bits(bw).1,
                _ => 0,
            }
        }, {
            $crate::literal_parse::get_overflow_mode(stringify!($ty))
        }>
    };
    ($($_: tt)*) => {
        compile_error!("expected integer literal or integer type descriptor, e.g. `0xFFFF`, `123456U1024s`, `I256` or `U512w`");
    };
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_n_macro() {
        type I256 = nt!(I256);
        let a: I256 = n!(0x_ABCDEF_);
        assert_eq!(a.to_str_radix(16), "abcdef");

        let b = n!(1_23_456U511s);
        assert_eq!(b.to_str_radix(10), "123456");

        let c = n!(0o123_45_670U257w);
        assert_eq!(c.to_str_radix(8), "12345670");

        type I24p = nt!(I24p);
        let d: I24p = n!(0b101010111100110111_I24p);
        assert_eq!(d.to_str_radix(2), "101010111100110111");
    }
}
