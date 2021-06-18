use super::BIint;

const fn saturate<const N: usize>((int, overflow): (BIint<N>, bool)) -> BIint<N> {
    if overflow {
        if int.is_negative() {
            BIint::<N>::MAX
        } else {
            BIint::<N>::MIN
        }
    } else {
        int
    }
}

impl<const N: usize> BIint<N> {
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