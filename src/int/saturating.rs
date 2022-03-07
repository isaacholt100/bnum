use super::Bint;
use crate::{ExpType, BUint};

impl<const N: usize> Bint<N> {
    pub const fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(add) => add,
            None => {
                if self.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
        }
    }
    pub const fn saturating_add_unsigned(self, rhs: BUint<N>) -> Self {
        match self.checked_add_unsigned(rhs) {
            Some(i) => i,
            None => Self::MAX,
        }
    }
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        match self.checked_sub(rhs) {
            Some(add) => add,
            None => {
                if self.is_negative() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
        }
    }
    pub const fn saturating_sub_unsigned(self, rhs: BUint<N>) -> Self {
        match self.checked_sub_unsigned(rhs) {
            Some(i) => i,
            None => Self::MIN,
        }
    }
    pub const fn saturating_neg(self) -> Self {
        match self.checked_neg() {
            Some(abs) => abs,
            None => Self::MAX,
        }
    }
    pub const fn saturating_abs(self) -> Self {
        match self.checked_abs() {
            Some(abs) => abs,
            None => Self::MAX,
        }
    }
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        match self.checked_mul(rhs) {
            Some(mul) => mul,
            None => {
                if self.is_negative() == rhs.is_negative() {
                    Self::MAX
                } else {
                    Self::MIN
                }
            }
        }
    }
    pub const fn saturating_pow(self, exp: ExpType) -> Self {
        match self.checked_pow(exp) {
            Some(pow) => pow,
            None => {
                if self.is_negative() && exp & 1 != 0 {
                    Self::MIN
                } else {
                    Self::MAX
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    test_signed! {
        function: saturating_add(a: i128, b: i128),
        cases: [
            (i128::MAX, 2),
            (i128::MIN, -1),
            (275869749506754546i128, 4275689745096754896785i128)
        ]
    }
    test_signed! {
        function: saturating_sub(a: i128, b: i128),
        cases: [
            (i128::MAX, -5),
            (i128::MIN, i128::MAX),
            (27456873894567457667567i128, 784569026784526789475698i128)
        ]
    }
    test_signed! {
        function: saturating_neg(a: i128),
        cases: [
            (i128::MAX),
            (i128::MIN),
            (-2568974589675445698456i128)
        ]
    }
    test_signed! {
        function: saturating_abs(a: i128),
        cases: [
            (i128::MAX),
            (i128::MIN),
            (-7635479863709875678056409486i128)
        ]
    }
    test_signed! {
        function: saturating_mul(a: i128, b: i128),
        cases: [
            (i128::MAX, -5),
            (i128::MAX, -1),
            (i128::MIN, -1)
        ]
    }
    test_signed! {
        function: saturating_pow(a: i128, b: u16),
        cases: [
            (55i128, 12 as u16),
            (3678i128, 123 as u16),
            (i128::MIN, 5 as u16)
        ]
    }
}