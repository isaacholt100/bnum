use super::Bint;

const fn saturate<const N: usize>((int, overflow): (Bint<N>, bool)) -> Bint<N> {
    if overflow {
        if int.is_negative() {
            Bint::<N>::MAX
        } else {
            Bint::<N>::MIN
        }
    } else {
        int
    }
}

impl<const N: usize> Bint<N> {
    pub const fn saturating_add(self, rhs: Self) -> Self {
        saturate(self.overflowing_add(rhs))
    }
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        saturate(self.overflowing_sub(rhs))
    }
    pub const fn saturating_neg(self) -> Self {
        saturate(self.overflowing_neg())
    }
    pub const fn saturating_abs(self) -> Self {
        saturate(self.overflowing_abs())
    }
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        saturate(self.overflowing_mul(rhs))
    }
    pub const fn saturating_pow(self, exp: u32) -> Self {
        saturate(self.overflowing_pow(exp))
    }
}

#[cfg(test)]
mod tests {
    use crate::I128;

    test_signed! {
        test_name: test_saturating_add,
        method: saturating_add(i128::MAX, i128::MAX)
    }
    test_signed! {
        test_name: test_saturating_sub,
        method: saturating_sub(i128::MIN, i128::MAX)
    }
    test_signed! {
        test_name: test_saturating_neg,
        method: saturating_neg(i128::MIN)
    }
    test_signed! {
        test_name: test_saturating_abs,
        method: saturating_abs(i128::MIN)
    }
}