pub trait TryOps where Self: Sized {
    type Error;

    fn try_add(self, rhs: Self) -> Result<Self, Self::Error>;
    fn try_div(self, rhs: Self) -> Result<Self, Self::Error>;
    fn try_mul(self, rhs: Self) -> Result<Self, Self::Error>;
    fn try_pow(self, exp: u32) -> Result<Self, Self::Error>;
    fn try_rem(self, rhs: Self) -> Result<Self, Self::Error>;
    fn try_shl(self, rhs: u32) -> Result<Self, Self::Error>;
    fn try_shr(self, rhs: u32) -> Result<Self, Self::Error>;
    fn try_sub(self, rhs: Self) -> Result<Self, Self::Error>;
}