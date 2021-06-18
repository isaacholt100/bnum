use super::BIint;
use crate::macros::checked_pow;

const fn tuple_to_option<const N: usize>((int, overflow): (BIint<N>, bool)) -> Option<BIint<N>> {
    if overflow {
        None
    } else {
        Some(int)
    }
}

impl<const N: usize> BIint<N> {
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_div(rhs))
    }
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_div_euclid(rhs))
    }
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_rem(rhs))
    }
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_rem_euclid(rhs))
    }
    pub const fn checked_neg(self) -> Option<Self> {
        tuple_to_option(self.overflowing_neg())
    }
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        tuple_to_option(self.overflowing_shl(rhs))
    }
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        tuple_to_option(self.overflowing_shr(rhs))
    }
    pub const fn checked_abs(self) -> Option<Self> {
        tuple_to_option(self.overflowing_abs())
    }
    checked_pow!();
}