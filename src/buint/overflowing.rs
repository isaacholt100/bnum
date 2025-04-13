use super::BUintD8;
use crate::digit;
use crate::digit::SignedDigit;
use crate::doc;
use crate::ExpType;

#[doc = doc::overflowing::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    #[doc = doc::overflowing::overflowing_add!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut carry = false;
        let mut i = 0;
        unsafe {
            while i < Self::FULL_U128_DIGITS {
                let result = digit::carrying_add_u128(
                    self.as_u128_digits().get(i),
                    rhs.as_u128_digits().get(i),
                    carry,
                );
                out.as_u128_digits_mut().set(i, result.0);
                carry = result.1;
                i += 1;
            }
        }
        if Self::U128_DIGIT_REMAINDER != 0 {
            let (d, _) = digit::carrying_add_u128(
                self.as_u128_digits().last(),
                rhs.as_u128_digits().last(),
                carry,
            );
            out.as_u128_digits_mut().set_last(d);
            carry = (128 - d.leading_zeros()) > (Self::U128_DIGIT_REMAINDER as u32) * 8;
        }
        (out, carry)
    }

    #[cfg(feature = "signed")]
    #[doc = doc::overflowing::overflowing_add_signed!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add_signed(self, rhs: crate::BIntD8<N>) -> (Self, bool) {
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
        unsafe {
            while i < Self::U128_DIGITS - 1 {
                // the last full u128 digits cause an overflow iff the truncated last digits cause an overflow
                let result = digit::borrowing_sub_u128(
                    self.as_u128_digits().get(i),
                    rhs.as_u128_digits().get(i),
                    borrow,
                );
                out.as_u128_digits_mut().set(i, result.0);
                borrow = result.1;
                i += 1;
            }
        }
        let result = digit::borrowing_sub_u128(
            self.as_u128_digits().last(),
            rhs.as_u128_digits().last(),
            borrow,
        );
        out.as_u128_digits_mut().set_last(result.0);
        (out, result.1)
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
                (Self::unchecked_shl_internal(self, rhs % Self::BITS), true)
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
                (Self::unchecked_shr_internal(self, rhs % Self::BITS), true)
            } else {
                (Self::unchecked_shr_internal(self, rhs), false)
            }
        }
    }

    #[inline]
    pub(crate) const fn overflowing_shr_signed(self, rhs: ExpType) -> (Self, bool) {
        let (overflow, shift) = if rhs >= Self::BITS {
            (true, rhs & Self::BITS_MINUS_1)
        } else {
            (false, rhs)
        };
        let u = unsafe {
            if (self.digits[N - 1] as SignedDigit).is_negative() {
                self.unchecked_shr_pad_internal::<true>(shift)
            } else {
                self.unchecked_shr_pad_internal::<false>(shift)
            }
        };
        (u, overflow)
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
        function: <utest>::overflowing_mul(a: utest, b: utest),
        cases: [(256u16, 1u16)]
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
