pub type Digit = u64;

pub type SignedDigit = i64;

pub type DoubleDigit = u128;

pub type SignedDoubleDigit = i128;

pub const DIGIT_BITS: usize = 64;

pub const DIGIT_BITS_U32: u32 = DIGIT_BITS as u32;

pub const DIGIT_BYTES: usize = DIGIT_BITS / 8;

pub const DIGIT_BYTE_SHIFT: usize = DIGIT_BYTES.trailing_zeros() as usize;

pub const DIGIT_BIT_SHIFT: usize = DIGIT_BITS.trailing_zeros() as usize;
// This calculates log2 of DIGIT_BYTES as DIGIT_BYTES is guaranteed to only have one '1' bit.