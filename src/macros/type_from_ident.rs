/// Returns a concrete instantiation of the [`Integer`](crate::Integer) type with the specified const-generic parameters from a type descriptor.
/// 
/// `t!` takes a type descriptor (which is an identifier fragment) and simply outputs [`Integer<S, N, B, OM>`](crate::Integer), where the values of the const-generic parameters `S`, `N`, `B` and `OM` are determined from the type descriptor.
/// 
/// A type descriptor has the following format: `<sign><bit_width><overflow_mode>?`:
/// - `<sign>` is either `I` (specifying a signed integer) or `U` (specifying an unsigned integer).
/// - `<bit_width>` is a decimal integer specifying the bit width of the integer type. The bit width must be at least `2` and at most `2^32 - 1`.
/// - `<overflow_mode>` is optional, and if specified must be one of:
///     - `w` for wrapping overflow mode ([`OverflowMode::Wrap`](crate::OverflowMode::Wrap)).
///     - `p` for panicking overflow mode ([`OverflowMode::Panic`](crate::OverflowMode::Panic)).
///     - `s` for saturating overflow mode ([`OverflowMode::Saturate`](crate::OverflowMode::Saturate)).
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
/// type MyInt = t!(I259p); // signed 259-bit integer with panicking overflow mode
/// type MyUint = t!(U633); // unsigned 633-bit integer with default overflow mode
/// type MyInt2 = t!(I538s); // signed 538-bit integer with saturating overflow mode
/// type MyUint2 = t!(U24w); // unsigned 24-bit integer with wrapping overflow mode
/// ```
/// 
/// The following examples will fail to compile, since the type descriptor is invalid. Note the type must be used in order to trigger the compile error.
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// type InvalidType = t!(A256); // invalid sign descriptor
/// 
/// dbg!(InvalidType::BITS);
/// ```
/// 
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// type InvalidType2 = t!(I1); // bit width too small
/// 
/// dbg!(InvalidType2::BITS);
/// ```
/// 
/// ```compile_fail
/// use bnum::prelude::*;
/// 
/// type InvalidType3 = t!(U1024x); // invalid overflow mode descriptor
/// 
/// dbg!(InvalidType3::BITS);
/// ```
#[macro_export]
// TODO: idea for floats: have struct FloatOrInteger with const generic param F (type bool, indicates whether float or int), other const generic params correspond to those of Float and Integer, then have a trait OutputType with an associated type Output, Output is Integer if F is false and Float if F is true. then the t macro returns <OutputType<...>::Output>.
macro_rules! t {
    ($ty: ident) => {
        $crate::Integer::<{
            $crate::__internal::get_signedness(stringify!($ty))
        }, {
            // we do the error handling here, as the processing of the bit width is the most computationally intensive (need to parse a string to an int, more cases to handle). it's very easy and fast to come up with a sign and overflow mode descriptors that are valid when the type descriptor is valid (the sign and overflow mode descriptors may be incorrect for an invalid type descriptor, but this does not matter as the error handling here will cause a panic anyway)
            const PARAMS_RESULT: Result<(bool, usize, u8), ($crate::__internal::TypeDescriptorError, &'static str)> = $crate::__internal::get_integer_params(stringify!($ty));
            // compiler doesn't say which const-generic parameter caused the error, so we can put all the error handling in one place, this means we won't forget to cover any of the possible errors
            match PARAMS_RESULT {
                Ok((_, bw, _)) => $crate::__internal::get_size_params_from_bits(bw).0,
                Err(_) => $crate::__internal_panic_type_descriptor_error!(PARAMS_RESULT),
            }
        }, {
            match $crate::__internal::get_integer_params(stringify!($ty)) {
                Ok((_, bw, _)) => $crate::__internal::get_size_params_from_bits(bw).1,
                _ => 0,
            }
        }, {
            $crate::__internal::get_overflow_mode(stringify!($ty))
        }>
    };
    ($($_: tt)*) => {
        compile_error!("expected integer type descriptor, e.g. `I256` or `U512w`");
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __internal_panic_type_descriptor_error {
    ($PARAMS_RESULT: ident) => {
        {
            const ERR_SRC: &'static str = match $PARAMS_RESULT {
                Err((_, source)) => source,
                _ => "",
            };

            use $crate::__internal::TypeDescriptorError::*;
            match $PARAMS_RESULT {
                Err((error_type, _)) => {
                    match error_type {
                        InvalidSuffix => {
                            $crate::concat_panic!("invalid integer type descriptor `", ERR_SRC, "`")
                        },
                        InvalidSignDescriptor => {
                            $crate::concat_panic!("invalid sign descriptor `", ERR_SRC, "` in type descriptor\nthe sign descriptor must be either `I` (for signed) or `U` (for unsigned)")
                        },
                        NoSignDescriptor => {
                            panic!("no sign descriptor specified in integer type descriptor")
                        },
                        BitWidthTooLarge => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for integer type\nthe width must be less than 2^32")
                        },
                        BitWidthTooSmall => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for integer type\nthe width must be at least 2")
                        },
                        NoBitWidthSpecified => {
                            panic!("no bit width specified for integer type descriptor")
                        },
                        InvalidOverflowModeDescriptor => {
                            $crate::concat_panic!("invalid overflow mode descriptor `", ERR_SRC, "` for integer type descriptor\nthe overflow mode descriptor must be either one of `w`, `p` or `s`, or omitted")
                        },
                    }
                },
                _ => unreachable!(),
            }
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
        b'w' => crate::OverflowMode::Wrap.to_u8(),
        b'p' => crate::OverflowMode::Panic.to_u8(),
        b's' => crate::OverflowMode::Saturate.to_u8(),
        _ => crate::OverflowMode::DEFAULT.to_u8(),
    }
}

#[doc(hidden)]
pub const fn get_integer_params(type_descriptor: &str) -> Result<(bool, usize, u8), (TypeDescriptorError, &str)> {
    use crate::OverflowMode;
    
    let bytes = type_descriptor.as_bytes();
    if bytes.is_empty() {
        return Err((TypeDescriptorError::InvalidSuffix, type_descriptor));
    }
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
            } else if usize::BITS > 32 && bw >= 2usize.wrapping_pow(32) { // wrapping pow as would overflow for narrower usizes
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
    let overflow_mode = if om_str.is_empty() {
        crate::OverflowMode::DEFAULT.to_u8()
    } else {
        match om_str.as_bytes() {
            b"w" => OverflowMode::Wrap.to_u8(),
            b"p" => OverflowMode::Panic.to_u8(),
            b"s" => OverflowMode::Saturate.to_u8(),
            _ => return Err((TypeDescriptorError::InvalidOverflowModeDescriptor, om_str)),
        }
    };
    Ok((is_signed, bw, overflow_mode))
}

#[doc(hidden)]
pub const fn get_integer_params_fallback(type_descriptor: &str) -> (bool, Result<bool, (TypeDescriptorError, &str)>, usize, usize, u8) {
    let (implicit, sign_result, n, b, om) = if type_descriptor.is_empty() {
        (true, Ok(false), 8, 0, 0)
    } else {
        let result = get_integer_params(type_descriptor);
        match result {
            Ok((s, bw, om)) => (false, Ok(s), get_size_params_from_bits(bw).0, get_size_params_from_bits(bw).1, om),
            Err(err) => (true, Err(err), 8, 0, 0),
        }
    };
    (implicit, sign_result, n, b, om)
}

#[doc(hidden)]
pub const fn get_size_params_from_bits(bits: usize) -> (usize, usize) {
    let bytes = bits.div_ceil(crate::Byte::BITS as usize);
    let b = if bits.is_multiple_of(crate::Byte::BITS as usize) {
        0
    } else {
        bits
    };
    (bytes, b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases_t_macro() {

    }
}