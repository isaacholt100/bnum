use core::fmt::{self, Debug, Display, Formatter};
use core::num::IntErrorKind;
use core::error::Error;

/// The error type that is returned when parsing an integer from an invalid source.
///
/// This error can occur when the [`from_str_radix`](crate::Integer::from_str_radix) or [`FromStr::from_str`](core::str::FromStr::from_str) methods of [`Integer`](crate::Integer) are called with an invalid input string.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ParseIntError {
    pub(crate) kind: IntErrorKind,
}

impl Error for ParseIntError {}

impl ParseIntError {
    /// Returns the enum [`IntErrorKind`], which indicates the reason that the parsing input was invalid.
    pub const fn kind(&self) -> &IntErrorKind {
        &self.kind
    }

    pub(crate) const fn description(&self) -> &str {
        match &self.kind {
            IntErrorKind::Empty => "attempt to parse integer from empty string",
            IntErrorKind::InvalidDigit => {
                "attempt to parse integer from string containing invalid digit"
            }
            IntErrorKind::PosOverflow => {
                "attempt to parse integer too large to be represented by the target type"
            }
            IntErrorKind::NegOverflow => {
                "attempt to parse integer too small to be represented by the target type"
            }
            IntErrorKind::Zero => {
                "attempt to parse the integer `0` which cannot be represented by the target type"
                // for now this should never occur, unless we implement NonZeroU... types
            }
            _ => panic!("unsupported `IntErrorKind` variant"), // necessary as `IntErrorKind` is non-exhaustive
        }
    }
}

impl Display for ParseIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", super::err_prefix!(), self.description())
    }
}
