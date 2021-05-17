use crate::digit::{Digit, SignedDoubleDigit, SignedDigit};

#[cfg(all(use_addcarry, Digit_digit))]
#[inline]
pub fn sbb(borrow: u8, a: Digit, b: Digit, out: &mut Digit) -> u8 {
    unsafe {
        super::arch::_subborrow_Digit(borrow, a, b, out)
    }
}

#[cfg(not(use_addcarry))]
#[inline]
pub fn sbb(borrow: u8, a: Digit, b: Digit, out: &mut Digit) -> u8 {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    *out = diff as Digit;
    (diff < 0) as u8
}

#[cfg(all(use_addcarry, Digit_digit))]
#[inline]
pub fn sub_borrow(borrow: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let mut out = 0;
    let carry = unsafe {
        super::arch::_subborrow_Digit(borrow, a, b, out)
    };
    (carry, out)
}

#[cfg(not(use_addcarry))]
#[inline]
pub fn sub_borrow(borrow: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    (diff as Digit, (diff < 0) as u8)
}

#[inline]
pub const fn sub_borrow_unsigned(borrow: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    (diff as Digit, (diff < 0) as u8)
}

#[inline]
pub const fn sub_borrow_signed(borrow: u8, a: SignedDigit, b: SignedDigit) -> (SignedDigit, bool) {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    (diff as SignedDigit, diff < 0)
}