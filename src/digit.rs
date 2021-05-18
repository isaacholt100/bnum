pub type Digit = u64;

pub type SignedDigit = i64;

pub type DoubleDigit = u128;

pub type SignedDoubleDigit = i128;

pub const BITS: usize = 64;

pub const BITS_U32: u32 = BITS as u32;

pub const BYTES: usize = BITS / 8;

pub const BYTE_SHIFT: usize = BYTES.trailing_zeros() as usize;
// This calculates log2 of BYTES as BYTES is guaranteed to only have one '1' bit.

pub const BIT_SHIFT: usize = BITS.trailing_zeros() as usize;

pub const MAX: Digit = Digit::MAX;

pub const MIN: Digit = Digit::MIN;

pub const HALF_BITS: usize = BITS / 2;

pub const HALF: Digit = (1 << HALF_BITS) - 1;

pub const fn to_double_digit(high: Digit, low: Digit) -> DoubleDigit {
    low as DoubleDigit | ((high as DoubleDigit) << BITS)
}