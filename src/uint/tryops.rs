use super::BUint;
use crate::tryops::TryOps;

impl<const N: usize> TryOps for BUint<N> {
    type Error = &'static str;

    fn try_add(self, rhs: Self) -> Result<Self, Self::Error> {
        let mut out = Self::default();
        self.add_mut(&rhs, &mut out)?;
        Ok(out)
    }
    fn try_div(self, rhs: Self) -> Result<Self, Self::Error> {
        Err("test error")
    }
    fn try_mul(self, rhs: Self) -> Result<Self, Self::Error> {
        Err("test error")
    }
    fn try_pow(self, exp: u32) -> Result<Self, Self::Error> {
        Err("test error")
    }
    fn try_rem(self, rhs: Self) -> Result<Self, Self::Error> {
        Err("test error")
    }
    fn try_shl(self, rhs: u32) -> Result<Self, Self::Error> {
        Err("test error")
    }
    fn try_shr(self, rhs: u32) -> Result<Self, Self::Error> {
        Err("test error")
    }
    fn try_sub(self, rhs: Self) -> Result<Self, Self::Error> {
        let mut out = Self::default();
        self.sub_mut(&rhs, &mut out)?;
        Ok(out)
    }
}