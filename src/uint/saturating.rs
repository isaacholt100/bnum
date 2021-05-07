use super::BUint;

const fn saturate_up<const N: usize>((int, overflow): (BUint<N>, bool)) -> BUint<N> {
    if overflow {
        BUint::<N>::MAX
    } else {
        int
    }
}

const fn saturate_down<const N: usize>((int, overflow): (BUint<N>, bool)) -> BUint<N> {
    if overflow {
        BUint::<N>::MIN
    } else {
        int
    }
}

impl<const N: usize> BUint<N> {
    pub fn saturating_add(self, rhs: Self) -> Self {
        saturate_up(self.overflowing_add(rhs))
    }
    pub fn saturating_sub(self, rhs: Self) -> Self {
        saturate_down(self.overflowing_sub(rhs))
    }
    pub fn saturating_mul(self, rhs: Self) -> Self {
        saturate_up(self.overflowing_mul(rhs))
    }
    pub fn saturating_pow(self, exp: u32) -> Self {
        saturate_up(self.overflowing_pow(exp))
    }
}