// This file uses code adapted from Rust's core library: https://doc.rust-lang.org/core/ used under the MIT license.
// The original license file for this project can be found in this project's root at licenses/LICENSE-rust.

use core::fmt::{self, Display, Formatter};
use core::num::IntErrorKind;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseIntError {
    pub(crate) kind: IntErrorKind,
}

impl ParseIntError {
    pub const fn kind(&self) -> &IntErrorKind {
        &self.kind
    }
}

impl Display for ParseIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let msg = match &self.kind {
            IntErrorKind::Empty => "cannot parse integer from empty string",
            IntErrorKind::InvalidDigit => "invalid digit found in string",
            IntErrorKind::PosOverflow => "number too large to fit in target type",
            IntErrorKind::NegOverflow => "number too small to fit in target type",
            IntErrorKind::Zero => "number would be zero for non-zero type",
            _ => panic!("unsupported IntErrorKind variant"),
        };
        write!(f, "{} {}", super::err_prefix!(), msg)
    }
}
