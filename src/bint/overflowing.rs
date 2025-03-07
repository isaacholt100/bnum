use super::BIntD8;
use crate::{BUintD8, Digit};

use crate::digit;
use crate::errors::div_zero;
use crate::{doc, ExpType};

#[doc = doc::overflowing::impl_desc!()]
impl<const N: usize> BIntD8<N> {
    #[doc = doc::overflowing::overflowing_add!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let sum = Self::from_bits(self.bits.wrapping_add(rhs.bits));
        let overflow = match (self.is_negative(), rhs.is_negative()) {
            (false, false) => sum.is_negative(),
            (true, true) => !sum.is_negative(),
            _ => false,
        };
        (sum, overflow)
    }

    #[doc = doc::overflowing::overflowing_add_unsigned!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add_unsigned(self, rhs: BUintD8<N>) -> (Self, bool) {
        let rhs = Self::from_bits(rhs);
        let (sum, overflow) = self.overflowing_add(rhs);
        (sum, rhs.is_negative() != overflow)
    }

    #[doc = doc::overflowing::overflowing_sub!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let sub = Self::from_bits(self.bits.wrapping_sub(rhs.bits));
        let overflow = match (self.is_negative(), rhs.is_negative()) {
            (false, true) => sub.is_negative(),
            (true, false) => !sub.is_negative(),
            _ => false,
        };
        (sub, overflow)
    }

    #[doc = doc::overflowing::overflowing_sub_unsigned!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub_unsigned(self, rhs: BUintD8<N>) -> (Self, bool) {
        let rhs = Self::from_bits(rhs);
        let (sum, overflow) = self.overflowing_sub(rhs);
        (sum, rhs.is_negative() != overflow)
    }

    const BITS_MINUS_1: ExpType = (Self::BITS - 1) as ExpType;

    #[doc = doc::overflowing::overflowing_mul!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (uint, overflow) = self.unsigned_abs().overflowing_mul(rhs.unsigned_abs());
        let out = Self::from_bits(uint);
        if self.is_negative() == rhs.is_negative() {
            (out, overflow || out.is_negative())
        } else {
            match out.checked_neg() {
                Some(n) => (n, overflow || out.is_negative()),
                None => (out, overflow),
            }
        }
    }

    #[inline]
    pub(crate) const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
        if self.eq(&Self::MIN) && rhs.is_one() {
            return (self, Self::ZERO);
        }
        let (div, rem) = self.unsigned_abs().div_rem_unchecked(rhs.unsigned_abs());
        let (div, rem) = (Self::from_bits(div), Self::from_bits(rem));

        match (self.is_negative(), rhs.is_negative()) {
            (false, false) => (div, rem),
            (false, true) => (div.neg(), rem),
            (true, false) => (div.neg(), rem.neg()),
            (true, true) => (div, rem.neg()),
        }
    }

    #[doc = doc::overflowing::overflowing_div!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            div_zero!()
        }
        if self.eq(&Self::MIN) {
            if rhs.eq(&Self::NEG_ONE) {
                return (self, true);
            } else if rhs.is_one() {
                return (self, false);
            }
        }
        (self.div_rem_unchecked(rhs).0, false)
    }

    #[doc = doc::overflowing::overflowing_div_euclid!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            div_zero!()
        }
        if self.eq(&Self::MIN) {
            if rhs.eq(&Self::NEG_ONE) {
                return (self, true);
            } else if rhs.is_one() {
                return (self, false);
            }
        }
        let (div, rem) = self.div_rem_unchecked(rhs);
        if self.is_negative() {
            let r_neg = rhs.is_negative();
            if !rem.is_zero() {
                if r_neg {
                    return (div.add(Self::ONE), false);
                } else {
                    return (div.sub(Self::ONE), false);
                };
            }
        }
        (div, false)
    }

    #[doc = doc::overflowing::overflowing_rem!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            div_zero!()
        }
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            (Self::ZERO, true)
        } else {
            (self.div_rem_unchecked(rhs).1, false)
        }
    }

    #[doc = doc::overflowing::overflowing_rem_euclid!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            div_zero!()
        }
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            (Self::ZERO, true)
        } else {
            let mut rem = self.div_rem_unchecked(rhs).1;
            if rem.is_negative() {
                if rhs.is_negative() {
                    rem = rem.wrapping_sub(rhs);
                } else {
                    rem = rem.wrapping_add(rhs);
                }
            }
            (rem, false)
        }
    }

    #[doc = doc::overflowing::overflowing_neg!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        self.not().overflowing_add(Self::ONE)
    }

    #[doc = doc::overflowing::overflowing_shl!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_shl(self, rhs: ExpType) -> (Self, bool) {
        let (uint, overflow) = self.bits.overflowing_shl(rhs);
        (Self::from_bits(uint), overflow)
    }

    #[doc = doc::overflowing::overflowing_shr!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_shr(self, rhs: ExpType) -> (Self, bool) {
        let bits = self.to_bits();
        let (overflow, shift) = if rhs >= Self::BITS {
            (true, rhs & Self::BITS_MINUS_1)
        } else {
            (false, rhs)
        };
        let u = unsafe {
            if self.is_negative() {
                BUintD8::unchecked_shr_pad_internal::<true>(bits, shift)
            } else {
                BUintD8::unchecked_shr_pad_internal::<false>(bits, shift)
            }
        };
        (Self::from_bits(u), overflow)
    }

    #[doc = doc::overflowing::overflowing_abs!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }

    #[doc = doc::overflowing::overflowing_pow!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_pow(self, pow: ExpType) -> (Self, bool) {
        let (u, mut overflow) = self.unsigned_abs().overflowing_pow(pow);
        let out_neg = self.is_negative() && pow & 1 == 1;
        let mut out = Self::from_bits(u);
        if out_neg {
            out = out.wrapping_neg();
            overflow = overflow || !out.is_negative();
        } else {
            overflow = overflow || out.is_negative();
        }
        (out, overflow)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::{test_bignum, types::*};

    test_bignum! {
        function: <itest>::overflowing_add(a: itest, b: itest)
    }
    test_bignum! {
        function: <itest>::overflowing_sub(a: itest, b: itest)
    }
    test_bignum! {
        function: <itest>::overflowing_mul(a: itest, b: itest)
    }
    test_bignum! {
        function: <itest>::overflowing_div(a: itest, b: itest),
        skip: b == 0,
        cases: [
            (itest::MIN, -1i8),
            (itest::MIN, 1i8)
        ]
    }
    test_bignum! {
        function: <itest>::overflowing_div_euclid(a: itest, b: itest),
        skip: b == 0,
        cases: [
            (itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::overflowing_rem(a: itest, b: itest),
        skip: b == 0,
        cases: [
            (itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::overflowing_rem_euclid(a: itest, b: itest),
        skip: b == 0,
        cases: [
            (itest::MIN, -1i8)
        ]
    }
    test_bignum! {
        function: <itest>::overflowing_neg(a: itest),
        cases: [
            (0i8),
            (itest::MIN)
        ]
    }
    test_bignum! {
        function: <itest>::overflowing_shl(a: itest, b: u16)
    }
    test_bignum! {
        function: <itest>::overflowing_shr(a: itest, b: u16)
    }
    test_bignum! {
        function: <itest>::overflowing_abs(a: itest),
        cases: [
            (0i8),
            (itest::MIN)
        ]
    }
    test_bignum! {
        function: <itest>::overflowing_pow(a: itest, b: u16)
    }
}
