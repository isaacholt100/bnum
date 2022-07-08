// This file uses code adapted from Rust's core library: https://doc.rust-lang.org/core/ used under the MIT license.
// The original license file for this project can be found in this project's root at licenses/LICENSE-rust.

use core::fmt::{self, Debug, Display, Formatter};
use core::num::IntErrorKind;

/// The error type that is returned when parsing an integer from an invalid source.
///
/// This error will occur when the `from_str_radix` or `FromStr::from_str` methods of `BUint` and `BInt` are called with an invalid input string.
#[derive(PartialEq, Eq, Clone)]
pub struct ParseIntError {
    pub(crate) kind: IntErrorKind,
}

impl ParseIntError {
    /// Returns the enum [`IntErrorKind`](https://doc.rust-lang.org/core/num/enum.IntErrorKind.html), which shows the reason that the parsing input was invalid.
    pub const fn kind(&self) -> &IntErrorKind {
        &self.kind
    }

    const fn description(&self) -> &str {
        match &self.kind {
            IntErrorKind::Empty => "cannot parse integer from empty string",
            IntErrorKind::InvalidDigit => "invalid digit found in string",
            IntErrorKind::PosOverflow => "number too large to fit in target type",
            IntErrorKind::NegOverflow => "number too small to fit in target type",
            IntErrorKind::Zero => "number would be zero for non-zero type",
            _ => panic!("unsupported IntErrorKind variant"),
        }
    }
}

impl Display for ParseIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", super::err_prefix!(), self.description())
    }
}

impl Debug for ParseIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self, f)
    }
}
