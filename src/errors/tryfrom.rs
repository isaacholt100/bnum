use core::fmt::{self, Display, Formatter};
use core::error::Error;

/// The error type that is returned when a failed conversion from an integer occurs.
///
/// This error will occur for example when using the [`TryFrom`] trait to attempt to convert a negative [`i32`] to a [`Uint`](crate::Uint).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TryFromIntError(pub(crate) ());

impl Error for TryFromIntError {}

const TRY_FROM_INT_ERROR_MESSAGE: &str = concat!(
    super::err_prefix!(),
    "out of range integral type conversion attempted"
);

impl Display for TryFromIntError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", TRY_FROM_INT_ERROR_MESSAGE)
    }
}

/// The error type that is returned when a failed conversion from a [`char`] occurs.
///
/// This error will occur when using the [`TryFrom`] trait to attempt to convert an out of range [`char`] to an [`Integer`](crate::Integer).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TryFromCharError(pub(crate) ());

impl Error for TryFromCharError {}

const TRY_FROM_CHAR_ERROR_MESSAGE: &str = concat!(
    super::err_prefix!(),
    "unicode code point out of range"
);

impl Display for TryFromCharError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", TRY_FROM_CHAR_ERROR_MESSAGE)
    }
}
