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

pub const BITS: ExpType = Digit::BITS as ExpType;

pub const BITS_MINUS_1: ExpType = BITS - 1;

pub const BYTES: ExpType = BITS / 8;

// This calculates log2 of BYTES as BYTES is guaranteed to only have one '1' bit, since it must be a power of two.
pub const BYTE_SHIFT: ExpType = BYTES.trailing_zeros() as ExpType;

pub const BIT_SHIFT: ExpType = BITS.trailing_zeros() as ExpType;

#[inline]
pub const fn to_double_digit(low: Digit, high: Digit) -> DoubleDigit {
    ((high as DoubleDigit) << BITS) | low as DoubleDigit
}

/// (low, high)
#[inline]
pub const fn from_double_digit(double: DoubleDigit) -> (Digit, Digit) {
    (double as Digit, (double >> BITS) as Digit)
}

#[inline]
pub const fn carrying_add(a: Digit, b: Digit, carry: bool) -> (Digit, bool) {
	let (s1, o1) = a.overflowing_add(b);
	if carry {
		let (s2, o2) = s1.overflowing_add(1);
		(s2, o1 || o2)
	} else {
		(s1, o1)
	}
}

#[inline]
pub const fn borrowing_sub(a: Digit, b: Digit, borrow: bool) -> (Digit, bool) {
	let (s1, o1) = a.overflowing_sub(b);
	if borrow {
		let (s2, o2) = s1.overflowing_sub(1);
		(s2, o1 || o2)
	} else {
		(s1, o1)
	}
}

#[inline]
pub const fn carrying_mul(a: Digit, b: Digit, carry: Digit) -> (Digit, Digit) {
	let double = a as DoubleDigit * b as DoubleDigit + carry as DoubleDigit;
	from_double_digit(double)
}