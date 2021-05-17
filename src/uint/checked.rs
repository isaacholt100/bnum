use super::BUint;
use crate::digit::Digit;

const fn tuple_to_option<const N: usize>((int, overflow): (BUint<N>, bool)) -> Option<BUint<N>> {
    if overflow {
        None
    } else {
        Some(int)
    }
}

impl<const N: usize> BUint<N> {
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }
    const fn div_rem_digit(self, digit: Digit) -> (Self, Self) {
        unimplemented!()
    }
    const fn div_rem_core(self, rhs: Self) -> (Self, Self) {
        unimplemented!()
    }
    const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
        if self.is_zero() {
            return (Self::ZERO, Self::ZERO);
        }
        if rhs.last_digit_index() == 0 {
            let first_digit = rhs.digits[0];
            if first_digit == 1 {
                return (self, Self::ZERO);
            }
            return self.div_rem_digit(first_digit);
        }

        use std::cmp::Ordering;

        match self.cmp(&rhs) {
            Ordering::Less => (Self::ZERO, rhs),
            Ordering::Equal => (Self::ONE, Self::ZERO),
            Ordering::Greater => {
                self.div_rem_core(rhs)
            }
        }
    }
    pub const fn div_rem(self, rhs: Self) -> (Self, Self) {
        if rhs.is_zero() {
            panic!("attempt to divide by zero")
        } else {
            self.div_rem_unchecked(rhs)
        }
    }
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).0)
        }
    }
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_div(rhs)
    }
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).1)
        }
    }
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_rem(rhs)
    }
    pub const fn checked_neg(self) -> Option<Self> {
        if self.is_zero() {
            Some(self)
        } else {
            None
        }
    }
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs as usize >= Self::BITS {
            None
        } else {
            Some(self.unchecked_shl(rhs))
        }
    }
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        tuple_to_option(self.overflowing_shr(rhs))
    }
    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
        tuple_to_option(self.overflowing_pow(exp))
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    fn converter(prim_result: Option<u128>) -> Option<U128> {
        match prim_result {
            Some(u) => Some(U128::from(u)),
            None => None,
        }
    }

    test_unsigned! {
        test_name: test_checked_add,
        method: checked_add(238732748937u128, 23583048508u128),
        converter: converter
    }
    test_unsigned! {
        test_name: test_checked_add_overflow,
        method: checked_add(u128::MAX, 1u128),
        converter: converter
    }

    test_unsigned! {
        test_name: test_checked_sub,
        method: checked_sub(334534859834905830u128, 93745873457u128),
        converter: converter
    }
    test_unsigned! {
        test_name: test_checked_sub_overflow,
        method: checked_sub(23423423u128, 209834908234898u128),
        converter: converter
    }
    
    // TODO: test other checked methods
}