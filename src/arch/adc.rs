use crate::digit::{Digit, DoubleDigit, SignedDigit, SignedDoubleDigit, self};

#[cfg(use_addcarry)]
#[inline]
pub fn adc(carry: u8, a: Digit, b: Digit, out: &mut Digit) -> u8 {
    unsafe {
        super::arch::_addcarry_u64(carry, a, b, out)
    }
}

#[cfg(not(use_addcarry))]
#[inline]
pub fn adc(carry: u8, a: Digit, b: Digit, out: &mut Digit) -> u8 {
    let sum = a as DoubleDigit + b as DoubleDigit + carry as DoubleDigit;
    *out = sum as Digit;
    (sum >> digit::BITS) as u8
}

/*#[cfg(feature = "intrinsics")]
#[inline]
pub fn add_carry(carry: u8, a: u64, b: u64) -> (u64, u8) {
    let mut out = 0;
    let carry = unsafe {
        super::arch::_addcarry_u64(carry, a, b, out)
    };
    (carry, out)
}*/

//#[cfg(not(feature = "intrinsics"))]
#[inline]
pub const fn add_carry_unsigned(carry: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let sum = a as DoubleDigit + b as DoubleDigit + carry as DoubleDigit;
    (sum as Digit, (sum >> digit::BITS) as u8)
}

#[inline]
pub const fn add_carry_signed(carry: u8, a: SignedDigit, b: SignedDigit) -> (SignedDigit, u8) {
    let sum = a as SignedDoubleDigit + b as SignedDoubleDigit + carry as SignedDoubleDigit;
    (sum as SignedDigit, (sum > SignedDigit::MAX as SignedDoubleDigit) as u8)
}