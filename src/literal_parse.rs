/// Create constant `Integer`s from native integer literals.
/// 
/// The `n!` macro parses integer literals to [`Integer`] values at compile time. It supports a similar integer literal syntax as Rust's built-in integer types:
/// - The prefix `0b` indicates a binary literal (base 2).
/// - The prefix `0o` indicates an octal literal (base 8).
/// - The prefix `0x` indicates a hexadecimal literal (base 16).
/// - Literals are parsed in as decimal literals (base 10) if no prefix is specified.
/// 
/// `n!` can be invoked in two ways:
/// 1. With just an integer literal, e.g. `n!(0xABCDEF)`. In this case, the const-generic parameters of the created [`Integer`] are left unspecified, so this must be used in a context where type inference can determine the parameters. For example: `let a: Integer<false, 16> = n!(0xABCDEF);` would be valid, but `let b = n!(0xABCDEF);` would trigger a compile error (unless `b` was subsequently used in a context which allowed for type inference).
/// 2. With an integer literal followed by a type descriptor, e.g. `n!(0xABCDEF U128w)`. The type descriptor may be any valid argument to the [`nt!`](crate::nt) macro. The type descriptor specifies the const-generic parameters of the created [`Integer`]. For more information about valid type descriptors, see the documentation for the [`nt!`](crate::nt) macro.
/// 
/// Invoking `n!` with invalid arguments will also trigger a compile error. This can happen if:
/// - The literal is out of range for the type (works for inferred types and types specified using a type descriptor). Note that this does not depend on the overflow mode of the type.
/// - The literal contains an invalid digits, e.g. `n!(0b102)` or `n!(1A U512)`.
/// - A `-` sign appears at the start of the literal when the type is unsigned, e.g. `n!(-123 U256)`.
/// - The type descriptor is invalid.
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
/// let b = n!(123456 I511s); // type specified using type descriptor
/// // type descriptor specifies signed 511-bit integer with saturating overflow behaviour
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
/// let c = n!(0x1000000 U24);
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
/// let e = n!(12345 U1024x);
/// ```
#[macro_export]
macro_rules! n {
    ($lit: literal $suffix:ident) => {
        const { $crate::from_literal_str!(stringify!($lit), <$crate::nt!($suffix)>) }
    };
    ($lit: literal) => {
        const { $crate::from_literal_str!(stringify!($lit), $crate::Integer) }
    };
    ($($_: tt)*) => {
        compile_error!("expected integer literal followed by optional type descriptor, e.g. `0xFFFF` or `123456 I256`");
    };
}

/// Creates an `Integer` type with the specified const-generic parameters from a type descriptor.
/// 
/// The `nt!` macro takes a type descriptor and simply outputs [`Integer<S, N, B, OM>`], where the const-generic parameters `S`, `N` and `OM` are determined from the type descriptor.
/// 
/// A type descriptor has the following format: `<sign><bit_width><overflow_mode>?`:
/// - `<sign>` is either `I` (specifying a signed integer) or `U` (specifying an unsigned integer).
/// - `<bit_width>` is a decimal integer specifying the bit width of the integer type. The bit width must be at least `2` and at most `2^32 - 1`.
/// - `<overflow_mode>` is optional, and if specified must be one of:
///     - `w` for wrapping overflow mode.
///     - `p` for panicking overflow mode.
///     - `s` for saturating overflow mode.
///     
///     If `<overflow_mode>` is omitted, the default overflow mode is used.
/// 
/// If the given type descriptor is not in this format, a compile error will be triggered.
/// 
/// If you want to create `Integer` values rather than types, use the [`n!`](crate::n) macro.
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
                Err((error_type, _)) => {
                    const ERR_SRC: &'static str = match PARAMS_RESULT {
                        Err((_, source)) => source,
                        _ => "",
                    };
                    match error_type {
                        $crate::literal_parse::TypeDescriptorError::InvalidSuffix => {
                            $crate::concat_panic!("invalid integer type descriptor `", ERR_SRC, "`")
                        },
                        $crate::literal_parse::TypeDescriptorError::InvalidSignDescriptor => {
                            $crate::concat_panic!("invalid sign descriptor `", ERR_SRC, "` in type descriptor\nthe sign descriptor must be either `I` (for signed) or `U` (for unsigned)")
                        },
                        $crate::literal_parse::TypeDescriptorError::NoSignDescriptor => {
                            panic!("no sign descriptor specified in integer type descriptor")
                        },
                        $crate::literal_parse::TypeDescriptorError::BitWidthTooLarge => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for integer type\nthe width must be less than 2^32")
                        },
                        $crate::literal_parse::TypeDescriptorError::BitWidthTooSmall => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for integer type\nthe width must be at least 2")
                        },
                        $crate::literal_parse::TypeDescriptorError::NoBitWidthSpecified => {
                            panic!("no bit width specified for integer type descriptor")
                        },
                        $crate::literal_parse::TypeDescriptorError::InvalidOverflowModeDescriptor => {
                            $crate::concat_panic!("invalid overflow mode descriptor `", ERR_SRC, "` for integer type descriptor\nthe overflow mode descriptor must be either one of `w`, `p` or `s`, or omitted")
                        },
                    }
                },
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

type A = nt!(asdf);

#[test]
fn ttt() {
    // panic!("{}", core::any::type_name::<A>());
}

struct Test<const N: usize>([u8; N]);

type C = Test<{todo!()}>;

#[doc(hidden)]
pub const fn concat_strs<'a, const LEN: usize>(msgs: &[&'a str]) -> [u8; LEN] {
    let mut i = 0;
    let mut write_index = 0;
    let mut buf: [u8; LEN] = [0; LEN];

    while i < msgs.len() {
        let msg_bytes = msgs[i].as_bytes();

        let mut j = 0;
        while j < msg_bytes.len() {
            buf[write_index] = msg_bytes[j];
            j += 1;
            write_index += 1;
        }
        i += 1;
    }
    assert!(write_index == LEN); // should have correctly determined LEN from the macro which called this function
    buf
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_panic {
    ($($msg: expr),+ $(,)?) => {
        {
            const LEN: usize = 0 $(+ $msg.as_bytes().len())+;
            const MSG_BUF: [u8; LEN] = $crate::literal_parse::concat_strs::<LEN>(&[$($msg),+]);
            const MSG_STR: &str = unsafe { core::str::from_utf8_unchecked(&MSG_BUF) }; // SAFETY: in concat_strs, we concatenated the strings byte slices directly, so the result is valid UTF-8
            panic!("{}", MSG_STR);
        }
    };
}

#[doc(hidden)]
pub enum TypeDescriptorError {
    InvalidSuffix,
    BitWidthTooLarge,
    BitWidthTooSmall,
    InvalidOverflowModeDescriptor,
    NoBitWidthSpecified,
    InvalidSignDescriptor,
    NoSignDescriptor,
}

#[doc(hidden)]
pub const fn get_signedness(suffix: &str) -> bool {
    !suffix.is_empty() && suffix.as_bytes()[0] as char == 'I'
}

#[doc(hidden)]
pub const fn get_overflow_mode(suffix: &str) -> u8 {
    match suffix.as_bytes()[suffix.len() - 1] {
        b'w' => crate::OverflowMode::Wrapping.to_u8(),
        b'p' => crate::OverflowMode::Panicking.to_u8(),
        b's' => crate::OverflowMode::Saturating.to_u8(),
        _ => crate::OverflowMode::DEFAULT.to_u8(),
    }
}

#[doc(hidden)]
pub const fn get_integer_params(type_descriptor: &str) -> Result<(bool, usize, u8), (TypeDescriptorError, &str)> {
    use crate::OverflowMode;

    if type_descriptor.len() < 1 {
        return Err((TypeDescriptorError::InvalidSuffix, type_descriptor));
    }
    let bytes = type_descriptor.as_bytes();
    let mut first_numeric_index = None;
    let mut last_numeric_index = 0;

    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as char;
        if first_numeric_index.is_none() {
            // first time we encounter a numeric character
            if c >= '0' && c <= '9' {
                first_numeric_index = Some(i);
                last_numeric_index = i;
            }
        } else {
            // now we have seen a numeric character, so break when we see a non-numeric character
            if c >= '0' && c <= '9' {
                last_numeric_index = i;
            } else {
                break;
            }
        }
        i += 1;
    }
    let (sign_str, rest) = match first_numeric_index {
        Some(1) => type_descriptor.split_at(1),
        Some(0) => return Err((TypeDescriptorError::NoSignDescriptor, "")), // no sign descriptor
        Some(idx) => return Err((TypeDescriptorError::InvalidSignDescriptor, type_descriptor.split_at(idx).0)), // more than one character for the sign descriptor, so must be invalid
        None => return Err((TypeDescriptorError::NoBitWidthSpecified, "")),
    };
    assert!(last_numeric_index >= 1); // as now we know there must be at least one numeric character, and the first numeric character is at index 1
    let sign_char = bytes[0] as char;
    if sign_char != 'I' && sign_char != 'U' {
        return Err((TypeDescriptorError::InvalidSignDescriptor, sign_str));
    }
    let is_signed = sign_char == 'I';
    let (bw_str, om_str) = rest.split_at(last_numeric_index); // not plus one as the index is according to suffix, not rest
    let bw = match usize::from_str_radix(bw_str, 10) {
        Ok(bw) => {
            if bw < 2 {
                return Err((TypeDescriptorError::BitWidthTooSmall, bw_str));
            } else if usize::BITS > 32 && bw >= (1 << 32) {
                return Err((TypeDescriptorError::BitWidthTooLarge, bw_str));
            } else {
                bw
            }
        }
        Err(err) => {
            return match err.kind() {
                core::num::IntErrorKind::PosOverflow => {
                    Err((TypeDescriptorError::BitWidthTooLarge, bw_str))
                }
                _ => Err((TypeDescriptorError::InvalidSuffix, type_descriptor)),
            };
        }
    };
    let overflow_mode = if om_str.len() == 0 {
        crate::OverflowMode::DEFAULT.to_u8()
    } else {
        match om_str.as_bytes() {
            b"w" => OverflowMode::Wrapping.to_u8(),
            b"p" => OverflowMode::Panicking.to_u8(),
            b"s" => OverflowMode::Saturating.to_u8(),
            _ => return Err((TypeDescriptorError::InvalidOverflowModeDescriptor, om_str)),
        }
    };
    Ok((is_signed, bw, overflow_mode))
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

#[doc(hidden)]
#[derive(Debug)]
pub enum ParseIntLiteralError {
    OutOfRange,
    NoDigits,
    InvalidDigit,
    UnsignedNegation,
}

use crate::{Integer, Uint};

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    #[doc(hidden)]
    #[inline]
    pub const fn from_literal_str_checked(literal_str: &str) -> Result<Self, ParseIntLiteralError> {
        // TODO: change to using custom error and panic
        // TODO: need to ignore underscores except at the start (but not the end)
        if literal_str.is_empty() {
            return Err(ParseIntLiteralError::NoDigits);
        }
        let bytes = literal_str.as_bytes();
        let negative = bytes[0] == b'-';
        if negative && !S {
            return Err(ParseIntLiteralError::UnsignedNegation);
        }
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

        use core::num::IntErrorKind;

        // TODO: need to use from_buf_radix because need to handle negation as well
        // have extra const gen param for skipping over underscores in from_buf_radix
        match Uint::from_buf_radix::<true, true, true>(bytes, radix) {
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
pub const fn get_size_params_from_bits(bits: usize) -> (usize, usize) {
    let bytes = bits.div_ceil(crate::Byte::BITS as usize);
    let b = if bits % (crate::Byte::BITS as usize) == 0 {
        0
    } else {
        bits
    };
    (bytes, b)
}