use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) enum FloatErrorKind {
    Empty,
    Invalid,
}

/// The error type that is returned when parsing a float from an invalid source.
///
/// This error can occur when e.g. the [`from_str`](crate::Float::from_str) or
/// [`FromStr::from_str`](core::str::FromStr::from_str) methods of [`Float`](crate::Float)
/// are called with an invalid input string.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ParseFloatError {
    pub(crate) kind: FloatErrorKind,
}

impl Error for ParseFloatError {}

impl ParseFloatError {
    pub(crate) const EMPTY: Self = Self {
        kind: FloatErrorKind::Empty,
    };

    pub(crate) const INVALID: Self = Self {
        kind: FloatErrorKind::Invalid,
    };

    pub(crate) const fn description(&self) -> &str {
        match self.kind {
            FloatErrorKind::Empty => "cannot parse float from empty string",
            FloatErrorKind::Invalid => "invalid float literal",
        }
    }
}

impl Display for ParseFloatError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", super::err_prefix!(), self.description())
    }
}
