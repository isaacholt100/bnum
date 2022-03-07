use super::BUint;
use crate::digit::Digit;

impl<const N: usize> BUint<N> {
    pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
        let (a, b) = self.overflowing_add(rhs);
        let (c, d) = a.overflowing_add(Self::from(carry));
        (c, b || d)
    }

    pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
        let (a, b) = self.overflowing_sub(rhs);
        let (c, d) = a.overflowing_sub(Self::from(borrow));
        (c, b || d)
    }

    pub const fn widening_mul(self, rhs: Self) -> (Self, Self) {
        let mut low = Self::ZERO;
        let mut high = Self::ZERO;
        let mut carry: Digit;

        let mut i = 0;
        while i < N {
            carry = 0;
            let mut j = 0;
            while j < N {
                let index = i + j;
                let d = if index < N {
                    low.digits[index]
                } else {
                    high.digits[index - N]
                };
                let (new_digit, new_carry) = crate::arithmetic::mul_carry_unsigned(carry, d, self.digits[i], rhs.digits[j]);
                carry = new_carry;
                if index < N {
                    low.digits[index] = new_digit;
                } else {
                    high.digits[index - N] = new_digit;
                }
                j += 1;
            }
            i += 1;
        }

        (low, high)
    }

    pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
        let (low, high) = self.widening_mul(rhs);
        let (low, overflow) = low.overflowing_add(carry);
        if overflow {
            (low, high + BUint::ONE)
        } else {
            (low, high)
        }
    }
}