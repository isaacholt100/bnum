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
        method: {
            saturating_add(i128::MAX, 2);
            saturating_add(i128::MIN, -1);
            saturating_add(275869749506754546i128, 4275689745096754896785i128);
        }
    }
    test_signed! {
        function: saturating_sub(a: i128, b: i128),
        method: {
            saturating_sub(i128::MAX, -5);
            saturating_sub(i128::MIN, i128::MAX);
            saturating_sub(27456873894567457667567i128, 784569026784526789475698i128);
        }
    }
    test_signed! {
        function: saturating_neg(a: i128),
        method: {
            saturating_neg(i128::MAX);
            saturating_neg(i128::MIN);
            saturating_neg(-2568974589675445698456i128);
            saturating_neg(8245069278956798745967i128);
        }
    }
    test_signed! {
        function: saturating_abs(a: i128),
        method: {
            saturating_abs(i128::MAX);
            saturating_abs(i128::MIN);
            saturating_abs(-7635479863709875678056409486i128);
            saturating_abs(17295692798567459867458967i128);
        }
    }
    test_signed! {
        function: saturating_mul(a: i128, b: i128),
        method: {
            saturating_mul(i128::MAX, -5);
            saturating_mul(i128::MAX, -1);
            saturating_mul(i128::MIN, -1);
            saturating_mul(-456979846894564i128, -4594957698i128);
        }
    }
    test_signed! {
        function: saturating_pow(a: i128, b: u16),
        method: {
            saturating_pow(55i128, 12 as u16);
            saturating_pow(3678i128, 123 as u16);
            saturating_pow(i128::MIN, 5 as u16);
            saturating_pow(-49654697456i128, 5674 as u16);
        }
    }
}