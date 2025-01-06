use super::BUintD8;
use crate::{digit, Digit, BIntD8};
use crate::doc;
use crate::ExpType;

#[doc = doc::saturating::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    #[inline]
    const fn saturate_up((int, overflow): (BUintD8<N>, bool)) -> BUintD8<N> {
        if overflow {
            BUintD8::MAX
        } else {
            int
        }
    }

    #[inline]
    const fn saturate_down((int, overflow): (BUintD8<N>, bool)) -> BUintD8<N> {
        if overflow {
            BUintD8::MIN
        } else {
            int
        }
    }

    #[doc = doc::saturating::saturating_add!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self::saturate_up(self.overflowing_add(rhs))
    }

    #[doc = doc::saturating::saturating_add_signed!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_add_signed(self, rhs: BIntD8<N>) -> Self {
        if rhs.is_negative() {
            Self::saturate_down(self.overflowing_add_signed(rhs))
        } else {
            Self::saturate_up(self.overflowing_add_signed(rhs))
        }
    }

    #[doc = doc::saturating::saturating_sub!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self::saturate_down(self.overflowing_sub(rhs))
    }

    #[doc = doc::saturating::saturating_mul!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self::saturate_up(self.overflowing_mul(rhs))
    }

    #[doc = doc::saturating::saturating_div!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_div(self, rhs: Self) -> Self {
        self.div_euclid(rhs)
    }

    #[doc = doc::saturating::saturating_pow!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_pow(self, exp: ExpType) -> Self {
        Self::saturate_up(self.overflowing_pow(exp))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::*;

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
        function: <utest>::saturating_div(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::saturating_pow(a: utest, b: u16)
    }
}
