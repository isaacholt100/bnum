use core::fmt::{self, Display, Formatter};

/// The error type that is returned when a failed conversion from an integer occurs.
///
/// This error will occur for example when using the [`TryFrom`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html) trait to convert a negative [`i32`] to a [`BUint`](crate::BUint).
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
            TooLarge => format!("`{}` is too large to convert to `{}`", self.from, self.to),
            Negative => format!("can't convert negative `{}` to `{}`", self.from, self.to),
        };
        write!(f, "{} {}", super::err_prefix!(), message)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TryFromErrorReason {
    TooLarge,
    Negative,
}
