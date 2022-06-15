use core::fmt::{Display, self, Formatter};
use core::num::IntErrorKind;

macro_rules! err_prefix {
	() => {
		"(bnum)"
	};
}

pub(crate) use err_prefix;

macro_rules! err_msg {
	($msg: literal) => {
		concat!(crate::error::err_prefix!(), " ", $msg)
	}
}

pub(crate) use err_msg;

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
            TooLarge => format!("(bnum) {} is too large to convert to {}", self.from, self.to),
            Negative => format!("(bnum) can't convert negative {} to {}", self.from, self.to),
            NotFinite => format!("(bnum) can't convert type {} which is not finite to type {}", self.from, self.to),
        };
        write!(f, "{} {}", err_prefix!(), message)
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
		let msg = match &self.kind {
            IntErrorKind::Empty => "(bnum) cannot parse integer from empty string",
            IntErrorKind::InvalidDigit => "(bnum) invalid digit found in string",
            IntErrorKind::PosOverflow => "(bnum) number too large to fit in target type",
            IntErrorKind::NegOverflow => "(bnum) number too small to fit in target type",
            IntErrorKind::Zero => "(bnum) number would be zero for non-zero type",
            _ => panic!("(bnum) unsupported IntErrorKind variant"),
        };
        write!(f, "{} {}", err_prefix!(), msg)
    }
}