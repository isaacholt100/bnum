#[cfg(use_addcarry)]
#[inline]
pub fn adc(carry: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    unsafe {
        super::arch::_addcarry_u64(carry, a, b, out)
    }
}

#[cfg(not(use_addcarry))]
#[inline]
pub fn adc(carry: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    let sum = a as u128 + b as u128 + carry as u128;
    *out = sum as u64;
    (sum >> 64) as u8
}

#[cfg(use_addcarry)]
#[inline]
pub fn add_carry(carry: u8, a: u64, b: u64) -> (u64, u8) {
    let mut out = 0;
    let carry = unsafe {
        super::arch::_addcarry_u64(carry, a, b, out)
    };
    (carry, out)
}

#[cfg(not(use_addcarry))]
#[inline]
pub fn add_carry(carry: u8, a: u64, b: u64) -> (u64, u8) {
    let sum = a as u128 + b as u128 + carry as u128;
    (sum as u64, (sum >> 64) as u8)
}

pub const fn add_carry_const(carry: u8, a: u64, b: u64) -> (u64, u8) {
    let sum = a as u128 + b as u128 + carry as u128;
    (sum as u64, (sum >> 64) as u8)
}