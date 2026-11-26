/// Returns a concrete instantiation of the [`Integer`](crate::Integer) or [`Float`](crate::Float) type with the specified const-generic parameters from a type descriptor.
///
/// `t!` takes a type descriptor (which is an identifier fragment) and outputs either [`Integer<S, N, B, OM>`](crate::Integer) or [`Float<W, MB, RM>`](crate::Float), where the const-generic parameters are determined from the type descriptor.
///
/// # Integer type descriptors
///
/// An integer type descriptor has the following format: `<sign><bit_width><overflow_mode>?`:
/// - `<sign>` is either `I` (specifying a signed integer) or `U` (specifying an unsigned integer).
/// - `<bit_width>` is a decimal integer specifying the bit width of the integer type. The bit width must be at least `2` and at most `2^32 - 1`.
/// - `<overflow_mode>` is optional, and if specified must be one of:
///     - `w` for wrapping overflow mode ([`OverflowMode::Wrap`](crate::OverflowMode::Wrap)).
///     - `p` for panicking overflow mode ([`OverflowMode::Panic`](crate::OverflowMode::Panic)).
///     - `s` for saturating overflow mode ([`OverflowMode::Saturate`](crate::OverflowMode::Saturate)).
///
///     If `<overflow_mode>` is omitted, the default overflow mode ([`OverflowMode::DEFAULT`](crate::OverflowMode::DEFAULT)) is used.
///
/// # Float type descriptors
///
/// When the `float` feature is enabled, `t!` also accepts float type descriptors of the form: `F<bit_width>(m<mantissa_bits>)?<rounding_mode>?`:
/// - `F` specifies a floating-point type ([`Float`](crate::Float)).
/// - `<bit_width>` is a decimal integer specifying the total bit width of the float type. It must be a multiple of `8`, at least `16`, and less than `2^32`.
/// - `m<mantissa_bits>` is optional and specifies the number of explicit mantissa bits `MB` (must satisfy `2 < MB < bit_width - 2`). If omitted, `MB` defaults to `10`, `23`, `52`, `64`, `112`, `236` for bit widths `16`, `32`, `64`, `80`, `128`, `256` respectively, and falls back to the IEEE 754-2019 binary interchange format formula (`MB = bit_width - 1 - (round(4 * log2(bit_width)) - 13)`) for other widths.
/// - `<rounding_mode>` is optional, and if specified must be one of:
///     - `te` for [`RoundingMode::TiesToEven`](crate::RoundingMode::TiesToEven) (default when omitted).
///     - `ta` for [`RoundingMode::TiesToAway`](crate::RoundingMode::TiesToAway).
///     - `tp` for [`RoundingMode::TiesTowardPositive`](crate::RoundingMode::TiesTowardPositive).
///     - `tn` for [`RoundingMode::TiesTowardNegative`](crate::RoundingMode::TiesTowardNegative).
///     - `tz` for [`RoundingMode::TiesTowardZero`](crate::RoundingMode::TiesTowardZero).
///     - `to` for [`RoundingMode::TiesToOdd`](crate::RoundingMode::TiesToOdd).
///     - `p` for [`RoundingMode::TowardPositive`](crate::RoundingMode::TowardPositive).
///     - `n` for [`RoundingMode::TowardNegative`](crate::RoundingMode::TowardNegative).
///     - `z` for [`RoundingMode::TowardZero`](crate::RoundingMode::TowardZero).
///     - `a` for [`RoundingMode::AwayFromZero`](crate::RoundingMode::AwayFromZero).
///     - `o` for [`RoundingMode::ToOdd`](crate::RoundingMode::ToOdd).
///     - `f` for [`RoundingMode::Faithful`](crate::RoundingMode::Faithful).
///     - `s` for [`RoundingMode::Stochastic`](crate::RoundingMode::Stochastic).
///
/// If the given type descriptor is not in this format, a compile error will be triggered when the type is used (when the type is unused, no compile error will be triggered).
///
/// If you want to create [`Integer`](crate::Integer) values rather than types, use the [`n!`](crate::n) macro. For [`Float`](crate::Float) values, use the `f!` macro (with the `float` feature enabled).
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
/// ```
/// # #[cfg(feature = "float")]
/// # fn main() {
/// use bnum::prelude::*;
///
/// type Half = t!(F16); // Float<2, 10> (ties to even)
/// type BFloat16 = t!(F16m7); // Float<2, 7> (ties to even)
/// type DoubleZero = t!(F64z); // Float<8, 52, 4> (toward zero)
/// type QuadAway = t!(F128m112ta); // Float<16, 112, 1> (ties to away)
/// # let _: Half = f!(1.5);
/// # let _: BFloat16 = f!(1.5);
/// # let _: DoubleZero = f!(1.5);
/// # let _: QuadAway = f!(1.5);
/// # }
/// # #[cfg(not(feature = "float"))]
/// # fn main() {}
/// ```
///
/// The following examples will fail to compile, since the type descriptor is invalid. Note the type must be used in order to trigger the compile error.
/// ```compile_fail
/// use bnum::prelude::*;
///
/// type InvalidType = t!(A256); // invalid type prefix
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
///
/// ```compile_fail
/// use bnum::prelude::*;
///
/// type InvalidFloat = t!(F20); // float bit width must be a multiple of 8
///
/// let _ = core::mem::size_of::<InvalidFloat>();
/// ```
#[macro_export]
macro_rules! t {
    ($ty: ident) => {
        <$crate::__internal::FloatOrInteger<
            { $crate::__internal::is_float_type_descriptor(stringify!($ty)) },
            { $crate::__internal::get_signedness(stringify!($ty)) },
            {
                // Perform all validation and error reporting in one const-generic parameter so
                // every invalid descriptor produces a single clear compile-time panic.
                const PARAMS_RESULT: Result<
                    (bool, bool, usize, usize, u8),
                    ($crate::__internal::TypeDescriptorError, &'static str),
                > = $crate::__internal::get_type_descriptor_params(stringify!($ty));
                match PARAMS_RESULT {
                    Ok((_, _, n, _, _)) => n,
                    Err(_) => $crate::__internal_panic_type_descriptor_error!(PARAMS_RESULT),
                }
            },
            {
                match $crate::__internal::get_type_descriptor_params(stringify!($ty)) {
                    Ok((_, _, _, b_or_mb, _)) => b_or_mb,
                    _ => 0,
                }
            },
            {
                match $crate::__internal::get_type_descriptor_params(stringify!($ty)) {
                    Ok((_, _, _, _, mode)) => mode,
                    _ => 0,
                }
            },
        > as $crate::__internal::TypeDescriptorOutput>::Output
    };
    ($($_: tt)*) => {
        compile_error!("expected type descriptor, e.g. `I256`, `U512w`, or `F64`");
    };
}

#[doc(hidden)]
pub trait TypeDescriptorOutput {
    type Output;
}

#[doc(hidden)]
pub struct FloatOrInteger<
    const IS_FLOAT: bool,
    const S: bool,
    const N: usize,
    const B_OR_MB: usize,
    const MODE: u8,
>;

impl<const S: bool, const N: usize, const B: usize, const OM: u8> TypeDescriptorOutput
    for FloatOrInteger<false, S, N, B, OM>
{
    type Output = crate::Integer<S, N, B, OM>;
}

#[cfg(feature = "float")]
impl<const S: bool, const N: usize, const MB: usize, const RM: u8> TypeDescriptorOutput
    for FloatOrInteger<true, S, N, MB, RM>
{
    type Output = crate::Float<N, MB, RM>;
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
                            $crate::concat_panic!("invalid type descriptor `", ERR_SRC, "`")
                        },
                        InvalidSignDescriptor => {
                            $crate::concat_panic!("invalid sign descriptor `", ERR_SRC, "` in integer type descriptor\nthe sign descriptor must be either `I` (for signed) or `U` (for unsigned)")
                        },
                        NoSignDescriptor => {
                            panic!("no sign descriptor specified in integer type descriptor")
                        },
                        InvalidTypePrefix => {
                            $crate::concat_panic!("invalid type prefix `", ERR_SRC, "` in type descriptor\nthe prefix must be `I` (for signed integer), `U` (for unsigned integer), or `F` (for float)")
                        },
                        NoTypePrefix => {
                            panic!("no type prefix specified in type descriptor")
                        },
                        InvalidFloatPrefix => {
                            $crate::concat_panic!("invalid float type prefix `", ERR_SRC, "` in type descriptor\nthe prefix must be `F`")
                        },
                        NoFloatPrefix => {
                            panic!("no type prefix specified in float type descriptor")
                        },
                        BitWidthTooLarge => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for integer type\nthe width must be less than 2^32")
                        },
                        BitWidthTooSmall => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for integer type\nthe width must be at least 2")
                        },
                        NoBitWidthSpecified => {
                            panic!("no bit width specified for type descriptor")
                        },
                        InvalidOverflowModeDescriptor => {
                            $crate::concat_panic!("invalid overflow mode descriptor `", ERR_SRC, "` for integer type descriptor\nthe overflow mode descriptor must be either one of `w`, `p` or `s`, or omitted")
                        },
                        FloatFeatureNotEnabled => {
                            panic!("float type descriptors require the `float` feature to be enabled")
                        },
                        FloatBitWidthTooSmall => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for float type\nthe width must be at least 16")
                        },
                        FloatBitWidthTooLarge => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for float type\nthe width must be less than 2^32")
                        },
                        FloatBitWidthNotMultipleOf8 => {
                            $crate::concat_panic!("invalid width `", ERR_SRC, "` for float type\nthe width must be a multiple of 8")
                        },
                        NoMantissaBitsSpecified => {
                            panic!("no mantissa bit width specified after `m` in float type descriptor")
                        },
                        MantissaBitsTooSmall => {
                            $crate::concat_panic!("invalid mantissa width `", ERR_SRC, "` for float type\nthe mantissa width must be greater than 2")
                        },
                        MantissaBitsTooLarge => {
                            $crate::concat_panic!("invalid mantissa width `", ERR_SRC, "` for float type\nthe mantissa width must be less than `bit_width - 2`")
                        },
                        InvalidRoundingModeDescriptor => {
                            $crate::concat_panic!("invalid rounding mode descriptor `", ERR_SRC, "` for float type descriptor\nthe rounding mode descriptor must be one of `te`, `ta`, `tp`, `tn`, `tz`, `to`, `p`, `n`, `z`, `a`, `o`, `f`, `s`, or omitted")
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
    InvalidTypePrefix,
    NoTypePrefix,
    InvalidFloatPrefix,
    NoFloatPrefix,
    FloatFeatureNotEnabled,
    FloatBitWidthTooSmall,
    FloatBitWidthTooLarge,
    FloatBitWidthNotMultipleOf8,
    NoMantissaBitsSpecified,
    MantissaBitsTooSmall,
    MantissaBitsTooLarge,
    InvalidRoundingModeDescriptor,
}

#[doc(hidden)]
pub const fn is_float_type_descriptor(suffix: &str) -> bool {
    cfg!(feature = "float") && !suffix.is_empty() && suffix.as_bytes()[0] == b'F'
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

/// Computes the default number of explicit mantissa bits for a float of `bits` total bits.
///
/// Matches `F16` (10), `F32` (23), `F64` (52), `F80` (64), `F128` (112), and `F256` (236),
/// and uses the IEEE 754-2019 binary interchange format formula
/// `w = round(4 * log2(k)) - 13`, `MB = k - 1 - w` for all other bit widths.
#[cfg(feature = "float")]
#[doc(hidden)]
pub const fn default_float_mantissa_bits(bits: usize) -> usize {
    match bits {
        16 => 10,
        32 => 23,
        64 => 52,
        80 => 64,
        128 => 112,
        256 => 236,
        _ => {
            // Exact integer computation of round(4 * log2(k)):
            // For integer k > 0, k^8 is never an odd power of 2, so
            // 2^(2n - 1) < k^8 < 2^(2n + 1) iff n = bit_width(k^8) / 2.
            // Since k < 2^32, k^4 < 2^128 and k^8 < 2^256 (no overflow).
            let k2 = (bits as u128) * (bits as u128);
            let k4 = k2 * k2;
            let lo = k4 as u64 as u128;
            let hi = (k4 >> 64) as u64 as u128;
            let ll = lo * lo;
            let lh = lo * hi;
            let hh = hi * hi;
            let mid = (ll >> 64) + (lh & 0xffff_ffff_ffff_ffff) * 2;
            let k8_hi = hh + (lh >> 64) * 2 + (mid >> 64);
            let k8_lo = (ll & 0xffff_ffff_ffff_ffff) | (mid << 64);
            let bit_width = if k8_hi != 0 {
                256 - k8_hi.leading_zeros() as usize
            } else {
                128 - k8_lo.leading_zeros() as usize
            };
            let exp_bits = (bit_width / 2) - 13;
            bits - 1 - exp_bits
        }
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

#[cfg(feature = "float")]
#[doc(hidden)]
pub const fn get_float_params(
    type_descriptor: &str,
) -> Result<(usize, usize, u8), (TypeDescriptorError, &str)> {
    use crate::RoundingMode;

    let bytes = type_descriptor.as_bytes();
    if bytes.is_empty() {
        return Err((TypeDescriptorError::InvalidSuffix, type_descriptor));
    }

    let mut first_numeric_index = None;
    let mut last_numeric_index = 0;

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if first_numeric_index.is_none() {
            if b.is_ascii_digit() {
                first_numeric_index = Some(i);
                last_numeric_index = i;
            }
        } else if b.is_ascii_digit() {
            last_numeric_index = i;
        } else {
            break;
        }
        i += 1;
    }

    let (prefix_str, rest) = match first_numeric_index {
        Some(1) => type_descriptor.split_at(1),
        Some(0) => return Err((TypeDescriptorError::NoFloatPrefix, "")),
        Some(idx) => {
            return Err((
                TypeDescriptorError::InvalidFloatPrefix,
                type_descriptor.split_at(idx).0,
            ));
        }
        None => return Err((TypeDescriptorError::NoBitWidthSpecified, "")),
    };
    assert!(last_numeric_index >= 1);

    if bytes[0] != b'F' {
        return Err((TypeDescriptorError::InvalidFloatPrefix, prefix_str));
    }

    let (bw_str, after_bw) = rest.split_at(last_numeric_index);
    let bw = match usize::from_str_radix(bw_str, 10) {
        Ok(bw) => {
            if bw < 16 {
                return Err((TypeDescriptorError::FloatBitWidthTooSmall, bw_str));
            } else if usize::BITS > 32 && bw >= 2usize.wrapping_pow(32) {
                return Err((TypeDescriptorError::FloatBitWidthTooLarge, bw_str));
            } else if bw % 8 != 0 {
                return Err((TypeDescriptorError::FloatBitWidthNotMultipleOf8, bw_str));
            } else {
                bw
            }
        }
        Err(err) => {
            return match err.kind() {
                core::num::IntErrorKind::PosOverflow => {
                    Err((TypeDescriptorError::FloatBitWidthTooLarge, bw_str))
                }
                _ => Err((TypeDescriptorError::InvalidSuffix, type_descriptor)),
            };
        }
    };

    let (mb, rm_str) = if !after_bw.is_empty() && after_bw.as_bytes()[0] == b'm' {
        let after_m = after_bw.split_at(1).1;
        let after_m_bytes = after_m.as_bytes();
        let mut mb_len = 0;
        while mb_len < after_m_bytes.len() && after_m_bytes[mb_len].is_ascii_digit() {
            mb_len += 1;
        }
        if mb_len == 0 {
            return Err((TypeDescriptorError::NoMantissaBitsSpecified, after_bw));
        }
        let (mb_str, rm_part) = after_m.split_at(mb_len);
        let mb = match usize::from_str_radix(mb_str, 10) {
            Ok(mb) => {
                if mb <= 2 {
                    return Err((TypeDescriptorError::MantissaBitsTooSmall, mb_str));
                } else if mb >= bw - 2 {
                    return Err((TypeDescriptorError::MantissaBitsTooLarge, mb_str));
                } else {
                    mb
                }
            }
            Err(err) => {
                return match err.kind() {
                    core::num::IntErrorKind::PosOverflow => {
                        Err((TypeDescriptorError::MantissaBitsTooLarge, mb_str))
                    }
                    _ => Err((TypeDescriptorError::InvalidSuffix, type_descriptor)),
                };
            }
        };
        (mb, rm_part)
    } else {
        (default_float_mantissa_bits(bw), after_bw)
    };

    let rounding_mode = if rm_str.is_empty() {
        RoundingMode::DEFAULT.to_u8()
    } else {
        match rm_str.as_bytes() {
            b"te" => RoundingMode::TiesToEven.to_u8(),
            b"ta" => RoundingMode::TiesToAway.to_u8(),
            b"p" => RoundingMode::TowardPositive.to_u8(),
            b"n" => RoundingMode::TowardNegative.to_u8(),
            b"z" => RoundingMode::TowardZero.to_u8(),
            b"a" => RoundingMode::AwayFromZero.to_u8(),
            b"o" => RoundingMode::ToOdd.to_u8(),
            b"tp" => RoundingMode::TiesTowardPositive.to_u8(),
            b"tn" => RoundingMode::TiesTowardNegative.to_u8(),
            b"tz" => RoundingMode::TiesTowardZero.to_u8(),
            b"to" => RoundingMode::TiesToOdd.to_u8(),
            b"f" => RoundingMode::Faithful.to_u8(),
            b"s" => RoundingMode::Stochastic.to_u8(),
            _ => {
                return Err((
                    TypeDescriptorError::InvalidRoundingModeDescriptor,
                    rm_str,
                ));
            }
        }
    };

    Ok((bw / 8, mb, rounding_mode))
}

#[doc(hidden)]
pub const fn get_type_descriptor_params(
    type_descriptor: &str,
) -> Result<(bool, bool, usize, usize, u8), (TypeDescriptorError, &str)> {
    let bytes = type_descriptor.as_bytes();
    if bytes.is_empty() {
        return Err((TypeDescriptorError::InvalidSuffix, type_descriptor));
    }

    let mut first_numeric_index = None;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            first_numeric_index = Some(i);
            break;
        }
        i += 1;
    }

    match first_numeric_index {
        Some(1) => match bytes[0] {
            b'I' | b'U' => match get_integer_params(type_descriptor) {
                Ok((s, bw, om)) => {
                    let (n, b) = get_size_params_from_bits(bw);
                    Ok((false, s, n, b, om))
                }
                Err(err) => Err(err),
            },
            b'F' => {
                #[cfg(feature = "float")]
                {
                    match get_float_params(type_descriptor) {
                        Ok((w, mb, rm)) => Ok((true, false, w, mb, rm)),
                        Err(err) => Err(err),
                    }
                }
                #[cfg(not(feature = "float"))]
                {
                    Err((TypeDescriptorError::FloatFeatureNotEnabled, type_descriptor))
                }
            }
            _ => Err((
                TypeDescriptorError::InvalidTypePrefix,
                type_descriptor.split_at(1).0,
            )),
        },
        Some(0) => Err((TypeDescriptorError::NoTypePrefix, "")),
        Some(idx) => Err((
            TypeDescriptorError::InvalidTypePrefix,
            type_descriptor.split_at(idx).0,
        )),
        None => Err((TypeDescriptorError::NoBitWidthSpecified, "")),
    }
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

#[cfg(feature = "float")]
#[doc(hidden)]
pub const fn get_float_params_fallback(
    type_descriptor: &str,
) -> (
    bool,
    Result<usize, (TypeDescriptorError, &str)>,
    usize,
    u8,
) {
    if type_descriptor.is_empty() {
        (true, Ok(8), 52, 0)
    } else {
        match get_float_params(type_descriptor) {
            Ok((w, mb, rm)) => (false, Ok(w), mb, rm),
            Err(err) => (true, Err(err), 52, 0),
        }
    }
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
        assert_eq!(<t!(U256)>::BITS, 256);
        assert_eq!(<t!(I259p)>::BITS, 259);
        assert_eq!(<t!(U24w)>::OVERFLOW_MODE, crate::OverflowMode::Wrap);
        assert_eq!(<t!(I512s)>::OVERFLOW_MODE, crate::OverflowMode::Saturate);

        #[cfg(not(feature = "float"))]
        assert!(matches!(
            super::get_type_descriptor_params("F64"),
            Err((super::TypeDescriptorError::FloatFeatureNotEnabled, "F64"))
        ));
    }

    #[test]
    fn cases_type_descriptor_helpers_runtime() {
        use crate::OverflowMode;

        assert_eq!(is_float_type_descriptor("F64"), cfg!(feature = "float"));
        assert!(!is_float_type_descriptor("U64"));
        assert!(!is_float_type_descriptor(""));

        assert!(get_signedness("I64"));
        assert!(!get_signedness("U64"));
        assert!(!get_signedness(""));

        assert_eq!(get_overflow_mode("U64w"), OverflowMode::Wrap.to_u8());
        assert_eq!(get_overflow_mode("U64p"), OverflowMode::Panic.to_u8());
        assert_eq!(get_overflow_mode("U64s"), OverflowMode::Saturate.to_u8());
        assert_eq!(get_overflow_mode("U64"), OverflowMode::DEFAULT.to_u8());

        assert_eq!(get_size_params_from_bits(8), (1, 0));
        assert_eq!(get_size_params_from_bits(64), (8, 0));
        assert_eq!(get_size_params_from_bits(17), (3, 17));
        assert_eq!(get_size_params_from_bits(259), (33, 259));

        // get_integer_params valid cases
        assert_eq!(
            get_integer_params("U64").ok(),
            Some((false, 64, OverflowMode::DEFAULT.to_u8()))
        );
        assert_eq!(
            get_integer_params("I259p").ok(),
            Some((true, 259, OverflowMode::Panic.to_u8()))
        );
        assert_eq!(
            get_integer_params("U24w").ok(),
            Some((false, 24, OverflowMode::Wrap.to_u8()))
        );
        assert_eq!(
            get_integer_params("I512s").ok(),
            Some((true, 512, OverflowMode::Saturate.to_u8()))
        );

        // get_integer_params error cases
        assert!(matches!(
            get_integer_params(""),
            Err((TypeDescriptorError::InvalidSuffix, ""))
        ));
        assert!(matches!(
            get_integer_params("64"),
            Err((TypeDescriptorError::NoSignDescriptor, ""))
        ));
        assert!(matches!(
            get_integer_params("UU64"),
            Err((TypeDescriptorError::InvalidSignDescriptor, "UU"))
        ));
        assert!(matches!(
            get_integer_params("X64"),
            Err((TypeDescriptorError::InvalidSignDescriptor, "X"))
        ));
        assert!(matches!(
            get_integer_params("U"),
            Err((TypeDescriptorError::NoBitWidthSpecified, ""))
        ));
        assert!(matches!(
            get_integer_params("U1"),
            Err((TypeDescriptorError::BitWidthTooSmall, "1"))
        ));
        assert!(matches!(
            get_integer_params("U4294967296"),
            Err((TypeDescriptorError::BitWidthTooLarge, "4294967296"))
        ));
        assert!(matches!(
            get_integer_params("U99999999999999999999999999999999999"),
            Err((
                TypeDescriptorError::BitWidthTooLarge,
                "99999999999999999999999999999999999"
            ))
        ));
        assert!(matches!(
            get_integer_params("U64x"),
            Err((TypeDescriptorError::InvalidOverflowModeDescriptor, "x"))
        ));

        // get_type_descriptor_params
        assert!(matches!(
            get_type_descriptor_params(""),
            Err((TypeDescriptorError::InvalidSuffix, ""))
        ));
        assert_eq!(
            get_type_descriptor_params("U64w").ok(),
            Some((false, false, 8, 0, OverflowMode::Wrap.to_u8()))
        );
        assert_eq!(
            get_type_descriptor_params("I17s").ok(),
            Some((false, true, 3, 17, OverflowMode::Saturate.to_u8()))
        );
        assert!(matches!(
            get_type_descriptor_params("U1"),
            Err((TypeDescriptorError::BitWidthTooSmall, "1"))
        ));
        assert!(matches!(
            get_type_descriptor_params("64"),
            Err((TypeDescriptorError::NoTypePrefix, ""))
        ));
        assert!(matches!(
            get_type_descriptor_params("X64"),
            Err((TypeDescriptorError::InvalidTypePrefix, "X"))
        ));
        assert!(matches!(
            get_type_descriptor_params("UU64"),
            Err((TypeDescriptorError::InvalidTypePrefix, "UU"))
        ));
        assert!(matches!(
            get_type_descriptor_params("ABC"),
            Err((TypeDescriptorError::NoBitWidthSpecified, ""))
        ));

        // get_integer_params_fallback
        assert!(matches!(
            get_integer_params_fallback(""),
            (true, Ok(false), 8, 0, 0)
        ));
        assert!(matches!(
            get_integer_params_fallback("I17s"),
            (false, Ok(true), 3, 17, om) if om == OverflowMode::Saturate.to_u8()
        ));
        assert!(matches!(
            get_integer_params_fallback("bad"),
            (true, Err((TypeDescriptorError::NoBitWidthSpecified, "")), 8, 0, 0)
        ));
    }

    #[cfg(feature = "float")]
    #[test]
    fn cases_t_macro_float() {
        use crate::{Float, RoundingMode};
        use core::any::TypeId;

        // Standard IEEE/extended float widths with default mantissa bits & rounding mode
        assert_eq!(TypeId::of::<t!(F16)>(), TypeId::of::<crate::types::F16>());
        assert_eq!(TypeId::of::<t!(F32)>(), TypeId::of::<crate::types::F32>());
        assert_eq!(TypeId::of::<t!(F64)>(), TypeId::of::<crate::types::F64>());
        assert_eq!(TypeId::of::<t!(F80)>(), TypeId::of::<crate::types::F80>());
        assert_eq!(TypeId::of::<t!(F128)>(), TypeId::of::<crate::types::F128>());
        assert_eq!(TypeId::of::<t!(F256)>(), TypeId::of::<crate::types::F256>());

        // IEEE 754-2019 interchange formula fallback for non-standard widths
        assert_eq!(TypeId::of::<t!(F24)>(), TypeId::of::<Float<3, 18>>());
        assert_eq!(TypeId::of::<t!(F40)>(), TypeId::of::<Float<5, 31>>());
        assert_eq!(TypeId::of::<t!(F48)>(), TypeId::of::<Float<6, 38>>());
        assert_eq!(TypeId::of::<t!(F160)>(), TypeId::of::<Float<20, 143>>());
        assert_eq!(TypeId::of::<t!(F192)>(), TypeId::of::<Float<24, 174>>());
        assert_eq!(TypeId::of::<t!(F512)>(), TypeId::of::<Float<64, 488>>());
        assert_eq!(TypeId::of::<t!(F1024)>(), TypeId::of::<Float<128, 996>>());

        // Verify formula agrees with f64::log2 across a broad range of non-special widths
        for bytes in 3..=256usize {
            let bits = bytes * 8;
            if matches!(bits, 16 | 32 | 64 | 80 | 128 | 256) {
                continue;
            }
            let expected_w = (4.0 * (bits as f64).log2()).round() as usize - 13;
            let expected_mb = bits - 1 - expected_w;
            assert_eq!(
                default_float_mantissa_bits(bits),
                expected_mb,
                "mismatch for bits={bits}"
            );
        }

        // Custom mantissa widths
        assert_eq!(TypeId::of::<t!(F16m7)>(), TypeId::of::<Float<2, 7>>());
        assert_eq!(TypeId::of::<t!(F16m3)>(), TypeId::of::<Float<2, 3>>());
        assert_eq!(TypeId::of::<t!(F16m13)>(), TypeId::of::<Float<2, 13>>());
        assert_eq!(TypeId::of::<t!(F32m10)>(), TypeId::of::<Float<4, 10>>());
        assert_eq!(TypeId::of::<t!(F64m52)>(), TypeId::of::<Float<8, 52>>());

        // All 13 rounding mode suffixes (with and without explicit mantissa width)
        assert_eq!(<t!(F64)>::ROUNDING_MODE, RoundingMode::TiesToEven);
        assert_eq!(<t!(F64te)>::ROUNDING_MODE, RoundingMode::TiesToEven);
        assert_eq!(<t!(F64ta)>::ROUNDING_MODE, RoundingMode::TiesToAway);
        assert_eq!(<t!(F64p)>::ROUNDING_MODE, RoundingMode::TowardPositive);
        assert_eq!(<t!(F64n)>::ROUNDING_MODE, RoundingMode::TowardNegative);
        assert_eq!(<t!(F64z)>::ROUNDING_MODE, RoundingMode::TowardZero);
        assert_eq!(<t!(F64a)>::ROUNDING_MODE, RoundingMode::AwayFromZero);
        assert_eq!(<t!(F64o)>::ROUNDING_MODE, RoundingMode::ToOdd);
        assert_eq!(<t!(F64tp)>::ROUNDING_MODE, RoundingMode::TiesTowardPositive);
        assert_eq!(<t!(F64tn)>::ROUNDING_MODE, RoundingMode::TiesTowardNegative);
        assert_eq!(<t!(F64tz)>::ROUNDING_MODE, RoundingMode::TiesTowardZero);
        assert_eq!(<t!(F64to)>::ROUNDING_MODE, RoundingMode::TiesToOdd);
        assert_eq!(<t!(F64f)>::ROUNDING_MODE, RoundingMode::Faithful);
        assert_eq!(<t!(F64s)>::ROUNDING_MODE, RoundingMode::Stochastic);

        assert_eq!(
            TypeId::of::<t!(F16m7z)>(),
            TypeId::of::<Float<2, 7, { RoundingMode::TowardZero as u8 }>>()
        );
        assert_eq!(
            TypeId::of::<t!(F128m112tp)>(),
            TypeId::of::<Float<16, 112, { RoundingMode::TiesTowardPositive as u8 }>>()
        );

        // Error cases for get_float_params
        assert!(matches!(
            get_float_params(""),
            Err((TypeDescriptorError::InvalidSuffix, ""))
        ));
        assert!(matches!(
            get_float_params("F"),
            Err((TypeDescriptorError::NoBitWidthSpecified, ""))
        ));
        assert!(matches!(
            get_float_params("64"),
            Err((TypeDescriptorError::NoFloatPrefix, ""))
        ));
        assert!(matches!(
            get_float_params("U64"),
            Err((TypeDescriptorError::InvalidFloatPrefix, "U"))
        ));
        assert!(matches!(
            get_float_params("FF64"),
            Err((TypeDescriptorError::InvalidFloatPrefix, "FF"))
        ));
        assert!(matches!(
            get_float_params("F8"),
            Err((TypeDescriptorError::FloatBitWidthTooSmall, "8"))
        ));
        assert!(matches!(
            get_float_params("F20"),
            Err((TypeDescriptorError::FloatBitWidthNotMultipleOf8, "20"))
        ));
        assert!(matches!(
            get_float_params("F4294967296"),
            Err((TypeDescriptorError::FloatBitWidthTooLarge, "4294967296"))
        ));
        assert!(matches!(
            get_float_params("F64m"),
            Err((TypeDescriptorError::NoMantissaBitsSpecified, "m"))
        ));
        assert!(matches!(
            get_float_params("F64mp"),
            Err((TypeDescriptorError::NoMantissaBitsSpecified, "mp"))
        ));
        assert!(matches!(
            get_float_params("F16m2"),
            Err((TypeDescriptorError::MantissaBitsTooSmall, "2"))
        ));
        assert!(matches!(
            get_float_params("F16m14"),
            Err((TypeDescriptorError::MantissaBitsTooLarge, "14"))
        ));
        assert!(matches!(
            get_float_params("F64x"),
            Err((TypeDescriptorError::InvalidRoundingModeDescriptor, "x"))
        ));
        assert!(matches!(
            get_float_params("F64m52m10"),
            Err((TypeDescriptorError::InvalidRoundingModeDescriptor, "m10"))
        ));
        assert!(matches!(
            get_float_params("F99999999999999999999999999999999999"),
            Err((
                TypeDescriptorError::FloatBitWidthTooLarge,
                "99999999999999999999999999999999999"
            ))
        ));
        assert!(matches!(
            get_float_params("F64m99999999999999999999999999999999999"),
            Err((
                TypeDescriptorError::MantissaBitsTooLarge,
                "99999999999999999999999999999999999"
            ))
        ));

        // Runtime coverage for default_float_mantissa_bits (special widths + k8_hi != 0)
        assert_eq!(default_float_mantissa_bits(16), 10);
        assert_eq!(default_float_mantissa_bits(32), 23);
        assert_eq!(default_float_mantissa_bits(64), 52);
        assert_eq!(default_float_mantissa_bits(80), 64);
        assert_eq!(default_float_mantissa_bits(128), 112);
        assert_eq!(default_float_mantissa_bits(256), 236);
        let big_bits = 65536usize;
        let expected_w = (4.0 * (big_bits as f64).log2()).round() as usize - 13;
        assert_eq!(default_float_mantissa_bits(big_bits), big_bits - 1 - expected_w);

        // Runtime coverage for all rounding modes in get_float_params
        for (suffix, expected_rm) in [
            ("F64", RoundingMode::DEFAULT),
            ("F64te", RoundingMode::TiesToEven),
            ("F64ta", RoundingMode::TiesToAway),
            ("F64p", RoundingMode::TowardPositive),
            ("F64n", RoundingMode::TowardNegative),
            ("F64z", RoundingMode::TowardZero),
            ("F64a", RoundingMode::AwayFromZero),
            ("F64o", RoundingMode::ToOdd),
            ("F64tp", RoundingMode::TiesTowardPositive),
            ("F64tn", RoundingMode::TiesTowardNegative),
            ("F64tz", RoundingMode::TiesTowardZero),
            ("F64to", RoundingMode::TiesToOdd),
            ("F64f", RoundingMode::Faithful),
            ("F64s", RoundingMode::Stochastic),
        ] {
            assert_eq!(get_float_params(suffix).ok(), Some((8, 52, expected_rm.to_u8())));
        }

        // Runtime coverage for get_type_descriptor_params with float descriptors
        assert_eq!(
            get_type_descriptor_params("F16m7z").ok(),
            Some((true, false, 2, 7, RoundingMode::TowardZero.to_u8()))
        );
        assert!(matches!(
            get_type_descriptor_params("F8"),
            Err((TypeDescriptorError::FloatBitWidthTooSmall, "8"))
        ));

        // Runtime coverage for get_float_params_fallback
        assert!(matches!(
            get_float_params_fallback(""),
            (true, Ok(8), 52, 0)
        ));
        assert!(matches!(
            get_float_params_fallback("F16m7z"),
            (false, Ok(2), 7, rm) if rm == RoundingMode::TowardZero.to_u8()
        ));
        assert!(matches!(
            get_float_params_fallback("F8"),
            (true, Err((TypeDescriptorError::FloatBitWidthTooSmall, "8")), 52, 0)
        ));
    }
}