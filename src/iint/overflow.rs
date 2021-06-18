use super::BIint;
use crate::arithmetic;
use crate::digit::{SignedDigit, Digit};
use crate::macros::{overflowing_pow, div_zero, rem_zero, op_ref_impl};

impl<const N: usize> BIint<N> {
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut digits = [0; N];
        let mut carry = 0u8;

        let self_digits = self.digits();
        let rhs_digits = rhs.digits();

        let mut i = 0;
        while i < Self::N_MINUS_1 {
            let (sum, c) = arithmetic::add_carry_unsigned(carry, self_digits[i], rhs_digits[i]);
            digits[i] = sum;
            carry = c;
            i += 1;
        }
        let (sum, carry) = arithmetic::add_carry_signed(
            carry,
            self_digits[Self::N_MINUS_1] as SignedDigit,
            rhs_digits[Self::N_MINUS_1] as SignedDigit
        );
        digits[Self::N_MINUS_1] = sum as Digit;

        (Self::from_digits(digits), carry != 0)
    }
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut digits = [0; N];
        let mut borrow = 0u8;

        let self_digits = self.digits();
        let rhs_digits = rhs.digits();

        let mut i = 0;
        while i < Self::N_MINUS_1 {
            let (sub, b) = arithmetic::sub_borrow_unsigned(borrow, self_digits[i], rhs_digits[i]);
            digits[i] = sub;
            borrow = b;
            i += 1;
        }
        let (sub, borrow) = arithmetic::sub_borrow_signed(
            borrow,
            self_digits[Self::N_MINUS_1] as SignedDigit,
            rhs_digits[Self::N_MINUS_1] as SignedDigit
        );
        digits[Self::N_MINUS_1] = sub as Digit;

        (Self::from_digits(digits), borrow)
    }
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (uint, overflow) = self.unsigned_abs().overflowing_mul(rhs.unsigned_abs());
        let out = Self {
            uint,
        };
        if self.is_negative() && rhs.is_negative() {
            (out, overflow || out.is_negative())
        } else {
            let out = out.neg();
            (out, overflow || out.is_positive())
        }
    }
    const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
        let (div, rem) = self.unsigned_abs().div_rem_unchecked(rhs.unsigned_abs());
        if self.is_negative() {
            if rhs.is_negative() {
                let div = Self {
                    uint: div,
                };
                let rem = Self {
                    uint: rem,
                };
                (div, rem.neg())
            } else {
                let div = Self {
                    uint: div,
                };
                let rem = Self {
                    uint: rem,
                };
                (div.neg(), rem.neg())
            }
        } else {
            if rhs.is_negative() {
                let div = Self {
                    uint: div,
                };
                let rem = Self {
                    uint: rem,
                };
                (div.neg(), rem)
            } else {
                let div = Self {
                    uint: div,
                };
                let rem = Self {
                    uint: rem,
                };
                (div, rem)
            }
        }
    }
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            (self, true)
        } else {
            if rhs.is_zero() {
                div_zero!()
            }
            (self.div_rem_unchecked(rhs).0, false)
        }
    }
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            (self, true)
        } else {
            if rhs.is_zero() {
                div_zero!()
            }
            let (div, rem) = self.div_rem_unchecked(rhs);
            if self.is_negative() {
                if rem.is_zero() {
                    (div, false)
                } else {
                    let div = if div.is_negative() {
                        div.sub(Self::ONE)
                    } else {
                        div.add(Self::ONE)
                    };
                    (div, false)
                }
            } else {
                (div, false)
            }
        }
    }
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            (self, true)
        } else {
            if rhs.is_zero() {
                div_zero!()
            }
            (self.div_rem_unchecked(rhs).1, false)
        }
    }
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            (self, true)
        } else {
            if rhs.is_zero() {
                rem_zero!()
            }
            let rem = self.div_rem_unchecked(rhs).1;
            if rem.is_negative() {
                (rem.add(rhs), false)
            } else {
                (rem, false)
            }
        }
    }
    pub const fn overflowing_neg(self) -> (Self, bool) {
        if self.is_zero() {
            (self, false)
        } else {
            self.not().overflowing_add(Self::ONE)
        }
    }
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        let (uint, overflow) = self.uint.overflowing_shl(rhs);
        (Self { uint }, overflow)
    }
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        let (uint, overflow) = self.uint.overflowing_shr(rhs);
        (Self { uint }, overflow)
    }
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }
    overflowing_pow!();
}

use core::ops::{Div, Rem};

impl<const N: usize> Div for BIint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            panic!("attempt to divide with overflow")
        } else {
            if rhs.is_zero() {
                div_zero!()
            }
            self.div_rem_unchecked(rhs).0
        }
    }
}

op_ref_impl!(Div<BIint<N>> for BIint, div);

impl<const N: usize> Rem for BIint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            panic!("attempt to calculate remainder with overflow")
        } else {
            if rhs.is_zero() {
                rem_zero!()
            }
            self.div_rem_unchecked(rhs).1
        }
    }
}

op_ref_impl!(Rem<BIint<N>> for BIint, rem);

#[cfg(test)]
mod tests {
    use crate::I128;

    fn converter(tuple: (i128, bool)) -> (I128, bool) {
        (tuple.0.into(), tuple.1)
    }

    test_signed! {
        test_name: test_overflowing_add,
        method: {
            overflowing_add(-934875934758937458934734533455i128, 347539475983475893475893475973458i128);
            overflowing_add(i128::MAX, i128::MAX);
            overflowing_add(934875934758937458934734533455i128, -3475395983475893475893475973458i128);
            overflowing_add(-1i128, 1i128);
        },
        converter: converter
    }
    test_signed! {
        test_name: test_overflowing_neg,
        method: {
            overflowing_neg(i64::MIN as i128);
        },
        converter: converter
    }
}