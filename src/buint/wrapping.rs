use super::BUintD8;
use crate::{digit, Digit, BIntD8};
use crate::errors::option_expect;
use crate::ExpType;
use crate::{doc, errors};

#[doc = doc::wrapping::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    #[doc = doc::wrapping::wrapping_add!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }

    #[doc = doc::wrapping::wrapping_add_signed!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add_signed(self, rhs: BIntD8<N>) -> Self {
        self.overflowing_add_signed(rhs).0
    }

    #[doc = doc::wrapping::wrapping_sub!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    #[doc = doc::wrapping::wrapping_mul!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

    #[doc = doc::wrapping::wrapping_div!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        option_expect!(
            self.checked_div(rhs),
            errors::err_msg!(errors::div_by_zero_message!())
        )
    }

    #[doc = doc::wrapping::wrapping_div_euclid!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    #[doc = doc::wrapping::wrapping_rem!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        option_expect!(
            self.checked_rem(rhs),
            errors::err_msg!(errors::rem_by_zero_message!())
        )
    }

    #[doc = doc::wrapping::wrapping_rem_euclid!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }

    #[doc = doc::wrapping::wrapping_neg!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    #[doc = doc::wrapping::wrapping_shl!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_shl(self, rhs: ExpType) -> Self {
        self.overflowing_shl(rhs).0
    }

    #[doc = doc::wrapping::wrapping_shr!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_shr(self, rhs: ExpType) -> Self {
        self.overflowing_shr(rhs).0
    }

    #[doc = doc::wrapping::wrapping_pow!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_pow(mut self, mut pow: ExpType) -> Self {
        // https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method
        if pow == 0 {
            return Self::ONE;
        }
        let mut y = Self::ONE;
        while pow > 1 {
            if pow & 1 == 1 {
                y = self.wrapping_mul(y);
            }
            self = self.wrapping_mul(self);
            pow >>= 1;
        }
        self.wrapping_mul(y)
    }

    #[doc = doc::wrapping::wrapping_next_power_of_two!(U 256)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::{test_bignum, types::*};

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn div_by_zero_panic() {
        let a = UTEST::MAX;
        let b = UTEST::ZERO;
        let _ = a.wrapping_div(b);
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn rem_by_zero_panic() {
        let a = UTEST::MAX;
        let b = UTEST::ZERO;
        let _ = a.wrapping_rem(b);
    }

    test_bignum! {
        function: <utest>::wrapping_add(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::wrapping_add_signed(a: utest, b: itest)
    }
    test_bignum! {
        function: <utest>::wrapping_sub(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::wrapping_mul(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::wrapping_div(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::wrapping_div_euclid(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::wrapping_rem(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::wrapping_rem_euclid(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::wrapping_neg(a: utest)
    }
    test_bignum! {
        function: <utest>::wrapping_shl(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::wrapping_shr(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::wrapping_pow(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::wrapping_next_power_of_two(a: utest),
        cases: [
            (utest::MAX)
        ]
    }
}
