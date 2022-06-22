use super::{BUint, ExpType};
use crate::{BInt, doc};

#[inline]
const fn saturate_up<const N: usize>((int, overflow): (BUint<N>, bool)) -> BUint<N> {
    if overflow {
        BUint::MAX
    } else {
        int
    }
}

#[inline]
const fn saturate_down<const N: usize>((int, overflow): (BUint<N>, bool)) -> BUint<N> {
    if overflow {
        BUint::MIN
    } else {
        int
    }
}

#[doc=doc::saturating::impl_desc!()]
impl<const N: usize> BUint<N> {
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        saturate_up(self.overflowing_add(rhs))
    }

    #[inline]
    pub const fn saturating_add_signed(self, rhs: BInt<N>) -> Self {
        if rhs.is_negative() {
            saturate_down(self.overflowing_add_signed(rhs))
        } else {
            saturate_up(self.overflowing_add_signed(rhs))
        }
    }

    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        saturate_down(self.overflowing_sub(rhs))
    }

    #[inline]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        saturate_up(self.overflowing_mul(rhs))
    }

    #[inline]
    pub const fn saturating_pow(self, exp: ExpType) -> Self {
        saturate_up(self.overflowing_pow(exp))
    }
}

#[cfg(test)]
mod tests {
	use crate::test::{test_bignum, types::*};

    test_bignum! {
		function: <utest>::saturating_add(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::saturating_add_signed(a: utest, b: itest)
    }
    test_bignum! {
		function: <utest>::saturating_sub(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::saturating_mul(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::saturating_pow(a: utest, b: u16)
    }
}