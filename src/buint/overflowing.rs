use super::BUintD8;
use crate::doc;
use crate::ExpType;
use crate::{digit, BIntD8};

#[doc = doc::overflowing::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    #[doc = doc::overflowing::overflowing_add!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut carry = false;
        let mut i = 0;
        while i < Self::FULL_U128_DIGITS {
            let result = digit::carrying_add_u128(self.u128_digit(i), rhs.u128_digit(i), carry);
            out.set_u128_digit(i, result.0);
            carry = result.1;
            i += 1;
        }
        if Self::U128_DIGIT_REMAINDER != 0 {
            let (d, _) = digit::carrying_add_u128(self.u128_digit(Self::FULL_U128_DIGITS), rhs.u128_digit(Self::FULL_U128_DIGITS), carry);
            out.set_u128_digit(Self::FULL_U128_DIGITS, d);
            carry = (128 - d.leading_zeros()) > (Self::U128_DIGIT_REMAINDER as u32) * 8;
        }
        (out, carry)
    }

    #[doc = doc::overflowing::overflowing_add_signed!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add_signed(self, rhs: BIntD8<N>) -> (Self, bool) {
        let (sum, overflow) = self.overflowing_add(rhs.to_bits());
        (sum, rhs.is_negative() != overflow)
    }

    #[doc = doc::overflowing::overflowing_sub!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut borrow = false;
        let mut i = 0;
        while i < Self::U128_DIGITS { // the last full u128 digits cause an overflow iff the truncated last digits cause an overflow
            let result = digit::borrowing_sub_u128(self.u128_digit(i), rhs.u128_digit(i), borrow);
            out.set_u128_digit(i, result.0);
            borrow = result.1;
            i += 1;
        }
        (out, borrow)
    }

    #[doc = doc::overflowing::overflowing_mul!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        // TODO: implement a faster multiplication algorithm for large values of `N`
        self.long_mul(rhs)
    }

    #[doc = doc::overflowing::overflowing_div!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_div(rhs), false)
    }

    #[doc = doc::overflowing::overflowing_div_euclid!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    #[doc = doc::overflowing::overflowing_rem!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_rem(rhs), false)
    }

    #[doc = doc::overflowing::overflowing_rem_euclid!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }

    #[doc = doc::overflowing::overflowing_neg!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (a, b) = (self.not()).overflowing_add(Self::ONE);
        (a, !b)
    }

    #[doc = doc::overflowing::overflowing_shl!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_shl(self, rhs: ExpType) -> (Self, bool) {
        unsafe {
            if rhs >= Self::BITS {
                (
                    Self::unchecked_shl_internal(self, rhs & (Self::BITS - 1)),
                    true,
                )
            } else {
                (Self::unchecked_shl_internal(self, rhs), false)
            }
        }
    }

    #[doc = doc::overflowing::overflowing_shr!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_shr(self, rhs: ExpType) -> (Self, bool) {
        unsafe {
            if rhs >= Self::BITS {
                (
                    Self::unchecked_shr_internal(self, rhs & (Self::BITS - 1)),
                    true,
                )
            } else {
                (Self::unchecked_shr_internal(self, rhs), false)
            }
        }
    }

    #[doc = doc::overflowing::overflowing_pow!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_pow(mut self, mut pow: ExpType) -> (Self, bool) {
        // exponentiation by squaring
        if pow == 0 {
            return (Self::ONE, false);
        }
        let mut overflow = false;
        let mut y = Self::ONE;
        while pow > 1 {
            if pow & 1 == 1 {
                let (prod, o) = y.overflowing_mul(self);
                overflow |= o;
                y = prod;
            }
            let (prod, o) = self.overflowing_mul(self);
            overflow |= o;
            self = prod;
            pow >>= 1;
        }
        let (prod, o) = self.overflowing_mul(y);
        (prod, o || overflow)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::{test_bignum, types::*};

    test_bignum! {
        function: <utest>::overflowing_add(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::overflowing_sub(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::overflowing_mul(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::overflowing_div(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::overflowing_div_euclid(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::overflowing_rem(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::overflowing_rem_euclid(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::overflowing_neg(a: utest)
    }
    test_bignum! {
        function: <utest>::overflowing_shl(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::overflowing_shr(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::overflowing_pow(a: utest, b: u16)
    }
}
