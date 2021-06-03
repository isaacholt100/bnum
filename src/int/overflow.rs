use super::Bint;
use crate::arch;
use crate::digit::SignedDigit;

impl<const N: usize> Bint<N> {
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (uint, carry) = self.uint.overflowing_add(rhs.uint);
        let (signed_digit, carry) = arch::add_carry_signed(carry as u8, self.signed_digit, rhs.signed_digit);
        let out = Self {
            signed_digit,
            uint,
        };
        (out, carry != 0)
    }
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (uint, carry) = self.uint.overflowing_sub(rhs.uint);
        let (signed_digit, carry) = arch::sub_borrow_signed(carry as u8, self.signed_digit, rhs.signed_digit);
        let out = Self {
            signed_digit,
            uint,
        };
        (out, carry)
    }
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        todo!()
    }
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        todo!()
    }
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        todo!()
    }
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        todo!()
    }
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        todo!()
    }
    pub const fn overflowing_neg(self) -> (Self, bool) {
        if self.is_zero() {
            (self, false)
        } else {
            self.not().overflowing_add(Self::ONE)
        }
    }
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        todo!()
    }
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        todo!()
    }
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::I128;

    fn converter(tuple: (i128, bool)) -> (I128, bool) {
        (tuple.0.into(), tuple.1)
    }

    test_signed! {
        test_name: test_overflowing_add_1,
        method: overflowing_add(-934875934758937458934734533455i128, 347539475983475893475893475973458i128),
        converter: converter
    }
    test_signed! {
        test_name: test_overflowing_add_2,
        method: overflowing_add(-934875934758937458934734533455i128, -347539475983475893475893475973458i128),
        converter: converter
    }
    test_signed! {
        test_name: test_overflowing_add_3,
        method: overflowing_add(934875934758937458934734533455i128, -3475395983475893475893475973458i128),
        converter: converter
    }
    test_signed! {
        test_name: test_overflowing_add_4,
        method: overflowing_add(-1i128, 1i128),
        converter: converter
    }
    test_signed! {
        test_name: test_overflowing_neg,
        method: overflowing_neg(i64::MIN as i128),
        converter: converter
    }
}