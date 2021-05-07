use super::BUint;
use crate::arch;

impl<const N: usize> BUint<N> {
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::MIN;
        let mut carry = 0u8;
        let mut i = 0;
        while i < N {
            let result = arch::add_carry(carry, self.digits[i], rhs.digits[i]);
            out.digits[i] = result.0;
            carry = result.1;
            i += 1;
        }
        (out, carry != 0)
    }
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::MIN;
        let mut borrow = 0u8;
        let mut i = 0;
        while i < N {
            let result = arch::sub_borrow(borrow, self.digits[i], rhs.digits[i]);
            out.digits[i] = result.0;
            borrow = result.1;
            i += 1;
        }
        (out, borrow != 0)
    }
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        // TODO: implement
        (Self::MIN, false)
    }
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        // TODO: implement
        (Self::MIN, false)
    }
    pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }
    pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        // TODO: implement
        (Self::MIN, false)
    }
    pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }
    pub fn overflowing_neg(self) -> (Self, bool) {
        if self.is_zero() {
            (Self::MIN, false)
        } else {
            (!self + Self::ONE, true)
        }
    }
    pub fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        // TODO: implement
        (Self::MIN, false)
    }
    pub fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        // TODO: implement
        (Self::MIN, false)
    }
    pub fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        // TODO: implement
        (Self::MIN, false)
    }
}