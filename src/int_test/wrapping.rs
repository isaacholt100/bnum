use super::BintTest;

impl<const N: usize> BintTest<N> {
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.overflowing_div_euclid(rhs).0
    }
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.overflowing_rem_euclid(rhs).0
    }
    pub const fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        self.overflowing_shl(rhs).0
    }
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        self.overflowing_shr(rhs).0
    }
    pub const fn wrapping_pow(self, exp: u32) -> Self {
        self.overflowing_shl(exp).0
    }
    pub const fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }
}