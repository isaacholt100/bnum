use super::BUint;

impl<const N: usize> BUint<N> {
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
        expect!(self.checked_div(rhs), "attempt to divide by zero")
    }
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        expect!(self.checked_rem(rhs), "attempt to calculate the remainder with a divisor of zero")
    }
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
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
}

#[cfg(test)]
mod tests {
    use crate::U128;
    
    test_unsigned! {
        test_name: test_wrapping_add,
        method: {
            wrapping_add(u128::MAX - 394857938475u128, 3947587348957384975893475983744567797u128);
            wrapping_add(984756897982709347597234977937u128, 4957698475906748597694574094567944u128);
        }
    }
    test_unsigned! {
        test_name: test_wrapping_sub_with_overflow,
        method: {
            wrapping_sub(34593475897340985709493475u128, 3947587348957384975893475983744567797u128);
            wrapping_sub(1030495898347598730975979834759739457u128, 4957698475906748597694574094567944u128);
        }
    }
}