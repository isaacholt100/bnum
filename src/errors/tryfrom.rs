use core::fmt::{self, Display, Formatter};

/// The error type that is returned when a failed conversion from an integer occurs.
///
/// This error will occur for example when using the [`TryFrom`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html) trait to convert a negative [`i32`] to a [`Uint`](crate::Uint).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TryFromIntError(pub(crate) ());

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

/// The error type that is returned when a failed conversion from a `char` occurs.
///
/// This error will occur when using the [`TryFrom`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html) trait to convert a `char` to a [`Uint`](crate::Uint) or an [`Int`](crate::Int).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TryFromCharError(pub(crate) ());

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
