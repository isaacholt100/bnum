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

        // TODO: need to use from_buf_radix because need to handle negation as well
        // have extra const gen param for skipping over underscores in from_buf_radix
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