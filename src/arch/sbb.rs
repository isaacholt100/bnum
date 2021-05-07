#[cfg(all(use_addcarry, u64_digit))]
#[inline]
pub fn sbb(borrow: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    unsafe {
        super::arch::_subborrow_u64(borrow, a, b, out)
    }
}

#[cfg(not(use_addcarry))]
#[inline]
pub fn sbb(borrow: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    let diff = a as i128 - b as i128 - borrow as i128;
    *out = diff as u64;
    (diff < 0) as u8
}

#[cfg(all(use_addcarry, u64_digit))]
#[inline]
pub fn sub_borrow(borrow: u8, a: u64, b: u64) -> (u64, u8) {
    let mut out = 0;
    let carry = unsafe {
        super::arch::_subborrow_u64(borrow, a, b, out)
    };
    (carry, out)
}

#[cfg(not(use_addcarry))]
#[inline]
pub fn sub_borrow(borrow: u8, a: u64, b: u64) -> (u64, u8) {
    let diff = a as i128 - b as i128 - borrow as i128;
    (diff as u64, (diff < 0) as u8)
}

pub const fn sub_borrow_const(borrow: u8, a: u64, b: u64) -> (u64, u8) {
    let diff = a as i128 - b as i128 - borrow as i128;
    (diff as u64, (diff < 0) as u8)
}