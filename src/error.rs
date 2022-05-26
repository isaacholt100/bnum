use core::fmt::{Display, self, Formatter};
use core::num::IntErrorKind;

// TODO: improve errors readability

#[derive(Debug, PartialEq, Eq)]
pub struct TryFromIntError {
    pub from: &'static str,
    pub to: &'static str,
    pub reason: TryFromErrorReason,
}

impl Display for TryFromIntError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use TryFromErrorReason::*;

        let message = match &self.reason {
            TooLarge => format!("{} is too large to convert to {}", self.from, self.to),
            Negative => format!("Can't convert negative {} to {}", self.from, self.to),
            NotFinite => format!("Can't convert type {} which is not finite to type {}", self.from, self.to),
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TryFromErrorReason {
    TooLarge,
    Negative,
    NotFinite,
}


#[derive(Debug, PartialEq, Eq)]
pub struct ParseIntError {
    pub(super) kind: IntErrorKind,
}

impl ParseIntError {
    pub const fn kind(&self) -> &IntErrorKind {
        &self.kind
    }
}

impl fmt::Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self.kind {
            IntErrorKind::Empty => "cannot parse integer from empty string",
            IntErrorKind::InvalidDigit => "invalid digit found in string",
            IntErrorKind::PosOverflow => "number too large to fit in target type",
            IntErrorKind::NegOverflow => "number too small to fit in target type",
            IntErrorKind::Zero => "number would be zero for non-zero type",
            _ => panic!("unsupported IntErrorKind variant"),
        })
    }
}