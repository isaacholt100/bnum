use super::BUint;
use crate::digit::Digit;

impl<const N: usize> BUint<N> {
    #[inline]
    pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
        let (a, b) = self.overflowing_add(rhs);
        let (c, d) = a.overflowing_add(Self::from(carry));
        (c, b || d)
    }

    #[inline]
    pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
        let (a, b) = self.overflowing_sub(rhs);
        let (c, d) = a.overflowing_sub(Self::from(borrow));
        (c, b || d)
    }

    #[inline]
    pub const fn widening_mul(self, rhs: Self) -> (Self, Self) {
        let mut low = Self::ZERO;
        let mut high = Self::ZERO;
        let mut carry: Digit;

        let mut i = 0;
        while i < N {
            carry = 0;
            let mut j = 0;
            while j < N - i {
                let index = i + j;
                let d = low.digits[index];
                let (new_digit, new_carry) = super::carrying_mul(self.digits[i], rhs.digits[j], carry, d);//self.digits[i].carrying_mul(rhs.digits[j], carry + d);
                carry = new_carry;
                low.digits[index] = new_digit;
                j += 1;
            }
            while j < N {
                let index = i + j - N;
                let d = high.digits[index];
                let (new_digit, new_carry) = super::carrying_mul(self.digits[i], rhs.digits[j], carry, d);//self.digits[i].carrying_mul(rhs.digits[j], carry + d);
                carry = new_carry;
                high.digits[index] = new_digit;
                j += 1;
            }
            high.digits[i] = carry;
            i += 1;
        }

        (low, high)
    }

    #[inline]
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

#[cfg(test)]
mod tests {
    use crate::test;

    test_unsigned! {
        function: carrying_add(a: u128, rhs: u128, carry: bool),
        cases: [
            (u128::MAX, 1u128, true),
            (u128::MAX, 1u128, false)
        ]
    }

    test_unsigned! {
        function: borrowing_sub(a: u128, rhs: u128, carry: bool),
        cases: [
            (0u128, 1u128, false),
            (0u128, 1u128, true)
        ]
    }

    test::test_big_num! {
        big: U64,
        primitive: u64,
        function: widening_mul,
        cases: [
            (u64::MAX, u64::MAX)
        ],
        quickcheck: (a: u64, rhs: u64)
    }

    test::test_big_num! {
        big: U64,
        primitive: u64,
        function: carrying_mul,
        cases: [
            (u64::MAX, u64::MAX, u64::MAX),
            (u64::MAX, u64::MAX, 1u64)
        ],
        quickcheck: (a: u64, rhs: u64, carry: u64)
    }
}