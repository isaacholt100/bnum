#[cfg(feature = "u8_digit")]
mod types {
    pub type Digit = u8;
    
    pub type SignedDigit = i8;
    
    pub type DoubleDigit = u16;
    
    pub type SignedDoubleDigit = i16;
}
#[cfg(not(feature = "u8_digit"))]
mod types {
    pub type Digit = u64;

    pub type SignedDigit = i64;

    pub type DoubleDigit = u128;

    pub type SignedDoubleDigit = i128;
}

use crate::ExpType;

pub use types::*;

//pub const HALF_MAX: Digit = Digit::MAX / 2;

pub const BITS: ExpType = Digit::BITS as ExpType;

pub const BITS_MINUS_1: ExpType = BITS - 1;

pub const BYTES: ExpType = BITS / 8;

pub const BYTE_SHIFT: ExpType = BYTES.trailing_zeros() as ExpType;
// This calculates log2 of BYTES as BYTES is guaranteed to only have one '1' bit.

pub const BIT_SHIFT: ExpType = BITS.trailing_zeros() as ExpType;

pub const HALF_BITS: ExpType = BITS / 2;

pub const HALF: Digit = (1 << HALF_BITS) - 1;

pub const fn to_double_digit(high: Digit, low: Digit) -> DoubleDigit {
    ((high as DoubleDigit) << BITS) | low as DoubleDigit
}