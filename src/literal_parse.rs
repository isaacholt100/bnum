#[macro_export]
macro_rules! n {
    ($lit: literal $suffix:ident) => {
        const { $crate::from_literal_str!(stringify!($lit), <$crate::nt!($suffix)>) }
        // const {
        //     const PARAMS_RESULT: Result<(bool, usize, u8), ($crate::literal_parse::BitWidthError, &'static str)> = $crate::literal_parse::get_integer_params(stringify!($suffix));
        //     const PARAMS: (bool, usize, u8) = match PARAMS_RESULT {
        //         Ok(params) => params,
        //         _ => (false, 0, 0), // dummy values as the errors will be handled below. we handle it like this because then the error messages the compiler gives are cleaner: just one error message rather than highlighting an error in each constant
        //     };
        //     const S: bool = PARAMS.0;
        //     const N: usize = PARAMS.1 / 8;
        //     const OM: u8 = PARAMS.2;

        //     const ERR_SRC: &'static str = match PARAMS_RESULT {
        //         Err((_, source)) => source,
        //         _ => "",
        //     };

        //     $crate::Integer::<{
        //         // perform the panicking here, as then the compiler gives cleaner error messages (doesn't show the source code of the macro). doesn't matter which const-generic parameter we put the error handling in, as the compiler doesn't say which parameter caused the error
        //         if let Err((error_type, _)) = PARAMS_RESULT {
        //             $crate::panic_bit_width_error!(error_type, ERR_SRC);
        //         }
        //         S
        //     }, N, OM>::from_literal_str(stringify!($lit))
        // }
    };
    ($lit: literal) => {
        const { $crate::from_literal_str!(stringify!($lit), $crate::Integer) }
    };
    ($($_: tt)*) => {
        compile_error!("expected integer literal followed by optional type descriptor, e.g. `0xFFFF` or `123456 I256`");
    };
}

#[macro_export]
macro_rules! nt {
    ($ty: ident) => {
        $crate::Integer::<{
            $crate::literal_parse::get_signedness(stringify!($ty))
        }, {
            // we do the error handling here, as the processing of the bit width is the most computationally intensive (need to parse a string to an int, more cases to handle). it's very easy and fast to come up with a sign and overflow mode descriptors that are valid when the type descriptor is valid (the sign and overflow mode descriptors may be incorrect for an invalid type descriptor, but this does not matter as the error handling here will cause a panic anyway)
            const PARAMS_RESULT: Result<(bool, usize, u8), ($crate::literal_parse::BitWidthError, &'static str)> = $crate::literal_parse::get_integer_params(stringify!($ty));
            // compiler doesn't say which const-generic parameter caused the error, so we can put all the error handling in one place, this means we won't forget to cover any of the possible errors
            const ERR_SRC: &'static str = match PARAMS_RESULT {
                Err((_, source)) => source,
                _ => "",
            };
            if let Err((error_type, _)) = PARAMS_RESULT {
                $crate::panic_bit_width_error!(error_type, ERR_SRC);
            }
            // TODO: separate into three functions, so we don't have to call it three times
            match $crate::literal_parse::get_integer_params(stringify!($ty)) {
                Ok((_, bw, _)) => bw / 8,
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
macro_rules! panic_bit_width_error {
    ($error_type: ident, $source: ident) => {
        match $error_type {
            $crate::literal_parse::BitWidthError::InvalidSuffix => {
                $crate::concat_panic!("invalid integer type descriptor `", $source, "`")
            },
            $crate::literal_parse::BitWidthError::InvalidSignDescriptor => {
                $crate::concat_panic!("invalid sign descriptor `", $source, "` in type descriptor\nthe sign descriptor must be either `I` (for signed) or `U` (for unsigned)")
            },
            $crate::literal_parse::BitWidthError::NoSignDescriptor => {
                panic!("no sign descriptor specified in integer type descriptor")
            },
            $crate::literal_parse::BitWidthError::BitWidthNotMultipleOf8 => {
                $crate::concat_panic!("invalid width `", $source, "` for integer type\nthe width must be a multiple of 8")
            },
            $crate::literal_parse::BitWidthError::BitWidthTooLarge => {
                $crate::concat_panic!("invalid width `", $source, "` for integer type\nthe width must be less than 2^32")
            },
            $crate::literal_parse::BitWidthError::BitWidthTooSmall => {
                $crate::concat_panic!("invalid width `", $source, "` for integer type\nthe width must be at least 2")
            },
            $crate::literal_parse::BitWidthError::NoBitWidthSpecified => {
                panic!("no bit width specified for integer type descriptor")
            },
            $crate::literal_parse::BitWidthError::InvalidOverflowModeDescriptor => {
                $crate::concat_panic!("invalid overflow mode descriptor `", $source, "` for integer type descriptor\nthe overflow mode descriptor must be either one of `w`, `p` or `s`, or omitted")
            },
        }
    };
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

#[test]
fn test_t_macro() {
    use crate::types::U256;
    type U = nt!(U12440s);
    let a: U256 = n!(123456);
    assert_eq!(U::BITS, 12440);
    assert!(U::MAX + U::MAX == U::MAX);
}

// pub(crate) use n;

#[doc(hidden)]
pub enum BitWidthError {
    InvalidSuffix,
    BitWidthNotMultipleOf8,
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
pub const fn get_integer_params(suffix: &str) -> Result<(bool, usize, u8), (BitWidthError, &str)> {
    use crate::OverflowMode;

    if suffix.len() < 1 {
        return Err((BitWidthError::InvalidSuffix, suffix));
    }
    let bytes = suffix.as_bytes();
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
        Some(1) => suffix.split_at(1),
        Some(0) => return Err((BitWidthError::NoSignDescriptor, "")), // no sign descriptor
        Some(idx) => return Err((BitWidthError::InvalidSignDescriptor, suffix.split_at(idx).0)), // more than one character for the sign descriptor, so must be invalid
        None => return Err((BitWidthError::NoBitWidthSpecified, "")),
    };
    assert!(last_numeric_index >= 1); // as now we know there must be at least one numeric character, and the first numeric character is at index 1
    let sign_char = bytes[0] as char;
    if sign_char != 'I' && sign_char != 'U' {
        return Err((BitWidthError::InvalidSignDescriptor, sign_str));
    }
    let is_signed = sign_char == 'I';
    let (bw_str, om_str) = rest.split_at(last_numeric_index); // not plus one as the index is according to suffix, not rest
    let bw = match usize::from_str_radix(bw_str, 10) {
        Ok(bw) => {
            if bw % 8 != 0 {
                return Err((BitWidthError::BitWidthNotMultipleOf8, bw_str));
            } else if bw < 2 {
                return Err((BitWidthError::BitWidthTooSmall, bw_str));
            } else if usize::BITS > 32 && bw >= (1 << 32) {
                return Err((BitWidthError::BitWidthTooLarge, bw_str));
            } else {
                bw
            }
        }
        Err(err) => {
            return match err.kind() {
                core::num::IntErrorKind::PosOverflow => {
                    Err((BitWidthError::BitWidthTooLarge, bw_str))
                }
                _ => Err((BitWidthError::InvalidSuffix, suffix)),
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
            _ => return Err((BitWidthError::InvalidOverflowModeDescriptor, om_str)),
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

#[derive(Debug)]
pub enum ParseIntLiteralError {
    OutOfRange,
    NoDigits,
    InvalidDigit,
    UnsignedNegation,
}

use crate::{Integer, Uint};

impl<const S: bool, const N: usize, const OM: u8> Integer<S, N, OM> {
    #[doc(hidden)]
    #[inline]
    pub const fn from_literal_str(literal_str: &str) -> Self {
        match Self::from_literal_str_checked(literal_str) {
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

    // or maybe just from_literal? not from_str_literal as that could be confused with a string literal
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
        match Uint::from_buf_radix_internal::<true, true, true>(bytes, radix) {
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
