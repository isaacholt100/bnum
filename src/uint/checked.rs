use super::BUint;

const fn tuple_to_option<const N: usize>((int, overflow): (BUint<N>, bool)) -> Option<BUint<N>> {
    if overflow {
        None
    } else {
        Some(int)
    }
}

impl<const N: usize> BUint<N> {
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_div(rhs))
    }
    pub fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_div(rhs)
    }
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_rem(rhs))
    }
    pub fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_rem(rhs)
    }
    pub const fn checked_neg(self) -> Option<Self> {
        if self.is_zero() {
            Some(self)
        } else {
            None
        }
    }
    pub fn checked_shl(self, rhs: u32) -> Option<Self> {
        tuple_to_option(self.overflowing_shl(rhs))
    }
    pub fn checked_shr(self, rhs: u32) -> Option<Self> {
        tuple_to_option(self.overflowing_shr(rhs))
    }
    pub fn checked_pow(self, exp: u32) -> Option<Self> {
        tuple_to_option(self.overflowing_pow(exp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}