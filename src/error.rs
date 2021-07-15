use core::fmt::{Display, self, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct TryFromIntError {
    pub from: &'static str,
    pub to: &'static str,
    pub reason: TryFromErrorReason,
}

impl Display for TryFromIntError {
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
    pub reason: ParseIntErrorReason,
}

impl Display for ParseIntError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ParseIntErrorReason::*;

        let message = match &self.reason {
            TooLarge => format!("Number is too large to fit in the target type"),
            Empty => format!("Can't parse integer from empty string"),
            InvalidDigit => format!("Invalid digit found in string"),
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseIntErrorReason {
    Empty,
    TooLarge,
    InvalidDigit,
}

pub struct ParseRationalError {
    pub reason: ParseRationalErrorReason,
}

impl Display for ParseRationalError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.reason {
            ParseRationalErrorReason::ParseIntError(e) => Display::fmt(e, f),
            ParseRationalErrorReason::ZeroDenominator => write!(f, "Denominator is zero"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseRationalErrorReason {
    ParseIntError(ParseIntError),
    ZeroDenominator,
}