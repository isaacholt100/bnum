#[doc(hidden)]
#[macro_export]
macro_rules! panic_type_descriptor_error {
    ($PARAMS_RESULT: ident) => {
        {
            const ERR_SRC: &'static str = match $PARAMS_RESULT {
                Err((_, source)) => source,
                _ => "",
            };
            match $PARAMS_RESULT {
                Err((error_type, _)) => {
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
    let result = get_integer_params(type_descriptor);
    let (implicit, sign_result, n, b, om) = if type_descriptor.is_empty() {
        (true, Ok(false), 8, 0, 0)
    } else {
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
    let b = if bits % (crate::Byte::BITS as usize) == 0 {
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
    fn test_get_size_params() {
        for (bits, n, b) in [
            (1, 1, 1),
            (7, 1, 7),
            (8, 1, 0),
            (9, 2, 9),
            (15, 2, 15),
            (16, 2, 0),
            (17, 3, 17),
            (31, 4, 31),
            (32, 4, 0),
            (33, 5, 33),
            (255, 32, 255),
            (256, 32, 0),
            (257, 33, 257),
        ] {
            assert_eq!(get_size_params_from_bits(bits), (n, b));
        }
    }
}