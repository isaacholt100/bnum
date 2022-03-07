use super::BUint;

impl<const N: usize> BUint<N> {
    pub unsafe fn unchecked_add(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_unchecked()
    }

    pub unsafe fn unchecked_sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs).unwrap_unchecked()
    }

    pub unsafe fn unchecked_mul(self, rhs: Self) -> Self {
        self.checked_add(rhs).unwrap_unchecked()
    }
    
    pub unsafe fn unchecked_shl(self, rhs: Self) -> Self {
        let rhs = rhs.to_exp_type().unwrap_unchecked();
        self.checked_shl(rhs).unwrap_unchecked()
    }
    
    pub unsafe fn unchecked_shr(self, rhs: Self) -> Self {
        let rhs = rhs.to_exp_type().unwrap_unchecked();
        self.checked_shr(rhs).unwrap_unchecked()
    }
}