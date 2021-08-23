use crate::digit::{Digit, DoubleDigit, SignedDigit, SignedDoubleDigit, self};

#[inline]
pub const fn add_carry_unsigned(carry: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let sum = a as DoubleDigit + b as DoubleDigit + carry as DoubleDigit;
    (sum as Digit, (sum >> digit::BITS) as u8)
}

#[inline]
pub const fn add_carry_signed(carry: u8, a: SignedDigit, b: SignedDigit) -> (SignedDigit, bool) {
    let sum = a as SignedDoubleDigit + b as SignedDoubleDigit + carry as SignedDoubleDigit;
    (sum as SignedDigit, sum > SignedDigit::MAX as SignedDoubleDigit || sum < SignedDigit::MIN as SignedDoubleDigit)
}

#[inline]
pub const fn sub_borrow_unsigned(borrow: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    (diff as Digit, (diff < 0) as u8)
}

#[inline]
pub const fn sub_borrow_signed(borrow: u8, a: SignedDigit, b: SignedDigit) -> (SignedDigit, bool) {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    (diff as SignedDigit, diff > SignedDigit::MAX as SignedDoubleDigit || diff < SignedDigit::MIN as SignedDoubleDigit)
}

/// Tuple of (product, carry)
#[inline]
pub const fn mul_carry_unsigned(carry: Digit, current: Digit, a: Digit, b: Digit) -> (Digit, Digit) {
    let prod = carry as DoubleDigit + current as DoubleDigit + (a as DoubleDigit) * (b as DoubleDigit);
    (prod as Digit, (prod >> digit::BITS) as Digit)
}