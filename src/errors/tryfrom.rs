// This file uses code adapted from Rust's core library: https://doc.rust-lang.org/core/ used under the MIT license.
// The original license file for this project can be found in this project's root at licenses/LICENSE-rust.

use core::fmt::{self, Display, Formatter};

/// The error type that is returned when a failed conversion between two integers occurs.
///
/// This error will occur for example when using the `TryFrom` trait to convert an `i32` to a `BUint`.
#[derive(Debug, PartialEq, Eq)]
pub struct TryFromIntError {
    pub(crate) from: &'static str,
    pub(crate) to: &'static str,
    pub(crate) reason: TryFromErrorReason,
}

impl Display for TryFromIntError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use TryFromErrorReason::*;
        let message = match &self.reason {
            TooLarge => format!("{} is too large to convert to {}", self.from, self.to),
            Negative => format!("can't convert negative {} to {}", self.from, self.to),
            NotFinite => format!(
                "can't convert type {} which is not finite to type {}",
                self.from, self.to
            ),
        };
        write!(f, "{} {}", super::err_prefix!(), message)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TryFromErrorReason {
    TooLarge,
    Negative,
    NotFinite,
}
