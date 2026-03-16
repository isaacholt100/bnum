use super::Uint;
use crate::Exponent;
use crate::doc;
use crate::{Int, Integer};

macro_rules! impl_desc {
    () => {
        "Overflowing arithmetic methods which act on `self`: `self.overflowing_...`. Each method returns a tuple of type `(Self, bool)` where the first item of the tuple is the result of wrapping variant of the method (`self.wrapping_...`), and the second item is a boolean which indicates whether overflow would have occurred."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(1U1024).overflowing_add(n!(1)), (n!(2), false));
    /// assert_eq!(U1024::MAX.overflowing_add(n!(1)), (n!(0), true));
    ///
    /// assert_eq!(I1024::MIN.overflowing_add(n!(-1)), (I1024::MAX, true));
    /// assert_eq!(I1024::MAX.overflowing_add(n!(1)), (I1024::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        if S {
            let sum = self
                .force_sign::<false>()
                .overflowing_add(rhs.force_sign())
                .0
                .force_sign(); // we don't care about the overflow here, and we use this instead of wrapping_add, since that would lead to infinite recursion
            let overflow = match (self.is_negative_internal(), rhs.is_negative_internal()) {
                (false, false) => sum.is_negative_internal(),
                (true, true) => !sum.is_negative_internal(),
                _ => false,
            };
            return (sum, overflow);
        }
        let (out, carry) = self.to_digits::<u128>().overflowing_add(rhs.to_digits());

        let mut out = out.to_integer();
        let overflow = carry || !out.has_valid_pad_bits();
        out.set_sign_bits(); // in case of carry, need to set sign bits
        (out, overflow)
    }

    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(n!(1U256).overflowing_sub(n!(1)), (n!(0), false));
    /// assert_eq!(U256::MIN.overflowing_sub(n!(1)), (U256::MAX, true));
    ///
    /// assert_eq!(I256::MIN.overflowing_sub(n!(1)), (I256::MAX, true));
    /// assert_eq!(I256::MAX.overflowing_sub(n!(-1)), (I256::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        if S {
            let sub = self
                .force_sign::<false>()
                .overflowing_sub(rhs.force_sign())
                .0
                .force_sign(); // we don't care about the overflow here, and we use this instead of wrapping_sub, since that would lead to infinite recursion
            let overflow = match (self.is_negative_internal(), rhs.is_negative_internal()) {
                (false, true) => sub.is_negative_internal(),
                (true, false) => !sub.is_negative_internal(),
                _ => false,
            };
            return (sub, overflow);
        }

        let (out, borrow) = self.to_digits::<u128>().overflowing_sub(rhs.to_digits());
        let mut out = out.to_integer();
        // the last full u128 digits cause an overflow iff the truncated last digits cause an overflow, so don't need further checks for overflow if Self::U128_BITS_REMAINDER != 0
        out.set_sign_bits(); // in case of borrow, need to set sign bits

        (out, borrow)
    }

    /// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(n!(1U512).overflowing_mul(n!(1)), (n!(1), false));
    /// assert_eq!(U512::power_of_two(511).overflowing_mul(n!(2)), (n!(0), true));
    ///
    /// assert_eq!(n!(-3I512).overflowing_mul(n!(-7)), (n!(21), false));
    /// assert_eq!(I512::MIN.overflowing_mul(n!(-1)), (I512::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        if S {
            // TODO: might be possible to do this without using abs, as the wrapping calculation is the same as just casting to unsigned
            let (uint, overflow) = self
                .unsigned_abs_internal()
                .overflowing_mul(rhs.unsigned_abs_internal());
            let out = uint.force_sign();
            return if self.is_negative_internal() == rhs.is_negative_internal() {
                (out, overflow || out.is_negative_internal())
            } else {
                match out.checked_neg() {
                    Some(n) => (n, overflow || out.is_negative_internal()),
                    None => (out, overflow),
                }
            };
        }
        // TODO: implement a faster multiplication algorithm for large values of `N`
        let a = self.to_digits::<u128>();
        let b = rhs.to_digits::<u128>();
        
        let (out, mut overflow) = a.overflowing_mul(b);
        let mut out = out.to_integer();

        overflow |= !out.has_valid_pad_bits();
        out.set_sign_bits(); // in case of overflow, need to set sign bits

        (out, overflow)
    }

    /// Returns a tuple of the division along with a boolean indicating whether overflow occurred. Note that this can only happen for signed integers, when `self` is [`Self::MIN`] and `rhs` is `-1`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(5U1024).overflowing_div(n!(2)), (n!(2), false));
    /// assert_eq!(n!(-23I1024).overflowing_div(n!(4)), (n!(-5), false));
    /// assert_eq!(I1024::MIN.overflowing_div(n!(-1)), (I1024::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(
                crate::errors::div_by_zero_message!()
            ));
        }
        if self.is_division_overflow(&rhs) {
            return (self, true);
        }
        (self.div_rem_unchecked(rhs).0, false)
    }

    /// Returns a tuple of the Euclidean division along with a boolean indicating whether overflow occurred. Note that this can only happen for signed integers, when `self` is [`Self::MIN`] and `rhs` is `-1`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    ///
    /// assert_eq!(n!(13U2048).overflowing_div_euclid(n!(5)), (n!(2), false));
    /// assert_eq!(n!(-23I2048).overflowing_div_euclid(n!(4)), (n!(-6), false));
    /// assert_eq!(I2048::MIN.overflowing_div_euclid(n!(-1)), (I2048::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(
                crate::errors::div_by_zero_message!()
            ));
        }
        if self.is_division_overflow(&rhs) {
            return (self, true);
        }
        (self.div_rem_euclid_unchecked(rhs).0, false)
    }

    /// Returns a tuple of the remainder along with a boolean indicating whether overflow occurred during division. Note that this can only happen for signed integers, when `self` is [`Self::MIN`] and `rhs` is `-1`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(5U1024).overflowing_rem(n!(2)), (n!(1), false));
    /// assert_eq!(n!(-23I1024).overflowing_rem(n!(4)), (n!(-3), false));
    /// assert_eq!(I1024::MIN.overflowing_rem(n!(-1)), (n!(0), true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(
                crate::errors::rem_by_zero_message!()
            ));
        }
        if self.is_division_overflow(&rhs) {
            (Self::ZERO, true)
        } else {
            (self.div_rem_unchecked(rhs).1, false)
        }
    }

    /// Returns a tuple of the Euclidean remainder along with a boolean indicating whether overflow occurred during division. Note that this can only happen for signed integers, when `self` is [`Self::MIN`] and `rhs` is `-1`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(n!(13U512).overflowing_rem_euclid(n!(5)), (n!(3), false));
    /// assert_eq!(n!(-23I512).overflowing_rem_euclid(n!(4)), (n!(1), false));
    /// assert_eq!(I512::MIN.overflowing_rem_euclid(n!(-1)), (n!(0), true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(
                crate::errors::rem_by_zero_message!()
            ));
        }
        if self.is_division_overflow(&rhs) {
            (Self::ZERO, true)
        } else {
            (self.div_rem_euclid_unchecked(rhs).1, false)
        }
    }

    /// Returns a tuple of `!self + 1` along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// Note that the second item of the tuple will be `true` if `self` is not zero.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(n!(1U256).overflowing_neg(), (U256::MAX, true));
    /// assert_eq!(n!(0U256).overflowing_neg(), (n!(0), false));
    ///
    /// assert_eq!(n!(1I256).overflowing_neg(), (n!(-1), false));
    /// assert_eq!(I256::MIN.overflowing_neg(), (I256::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (a, b) = self.not().overflowing_add(Self::ONE);
        (a, b == S)
    }

    /// Returns a tuple of the left shift along with a boolean indicating whether `rhs` is greater than or equal to `Self::BITS`. If `rhs >= Self::BITS` then the returned value is `self` left-shifted by `rhs % Self::BITS`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    ///
    /// assert_eq!(n!(1U2048).overflowing_shl(1), (n!(2), false));
    /// assert_eq!(n!(1U2048).overflowing_shl(2049), (n!(2), true));
    /// assert_eq!(n!(1U2048).overflowing_shl(2048), (n!(1), true));
    ///
    /// assert_eq!(n!(-2I2048).overflowing_shl(3), (n!(-16), false));
    /// assert_eq!(n!(-2I2048).overflowing_shl(2051), (n!(-16), true));
    /// assert_eq!(n!(-2I2048).overflowing_shl(2048), (n!(-2), true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_shl(self, rhs: Exponent) -> (Self, bool) {
        unsafe {
            (Self::unchecked_shl_internal(self, rhs % Self::BITS), rhs >= Self::BITS)
        }
    }

    /// Returns a tuple of the right shift along with a boolean indicating whether `rhs` is greater than or equal to `Self::BITS`. If `rhs >= Self::BITS` then the returned value is `self` right-shifted by `rhs % Self::BITS`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(1U1024).overflowing_shr(1), (n!(0), false));
    /// assert_eq!(n!(2U1024).overflowing_shr(1025), (n!(1), true));
    /// assert_eq!(U1024::MAX.overflowing_shr(1024), (U1024::MAX, true));
    /// assert_eq!(U1024::MAX.overflowing_shr(1023), (n!(1), false));
    ///
    /// assert_eq!(n!(-4I1024).overflowing_shr(2), (n!(-1), false));
    /// assert_eq!(I1024::MIN.overflowing_shr(1023), (n!(-1), false));
    /// assert_eq!(I1024::MIN.overflowing_shr(1024), (I1024::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_shr(self, rhs: Exponent) -> (Self, bool) {
        let (overflow, shift) = if rhs >= Self::BITS {
            (true, rhs % Self::BITS) // can't use & as bits may not be power of two
        } else {
            (false, rhs)
        };
        let out = unsafe { self.unchecked_shr_internal(shift) };
        (out, overflow)
    }

    /// Returns a tuple of the exponentiation along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(n!(2U512).overflowing_pow(10), (n!(1024), false));
    /// assert_eq!(n!(2U512).overflowing_pow(512), (n!(0), true));
    ///
    /// assert_eq!(n!(-7I512).overflowing_pow(3), (n!(-343), false));
    /// assert_eq!(n!(-2I512).overflowing_pow(511), (I512::MIN, false));
    /// assert_eq!(n!(2I512).overflowing_pow(511), (I512::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_pow(mut self, mut exp: Exponent) -> (Self, bool) {
        if S {
            // TODO: if we can speed up overflowing_mul for signed, then don't need this condition
            let (u, mut overflow) = self.unsigned_abs_internal().overflowing_pow(exp);
            let out_neg = self.is_negative_internal() && exp % 2 == 1;
            let mut out = u.force_sign();
            if out_neg {
                out = out.wrapping_neg();
                overflow = overflow || !out.is_negative_internal();
            } else {
                overflow = overflow || out.is_negative_internal();
            }
            return (out, overflow);
        }
        // exponentiation by squaring
        if exp == 0 {
            return (Self::ONE, false);
        }
        let mut overflow = false;
        let mut y = Self::ONE;
        while exp > 1 {
            if exp % 2 == 1 {
                let (prod, o) = y.overflowing_mul(self);
                overflow |= o;
                y = prod;
            }
            let (prod, o) = self.overflowing_mul(self);
            overflow |= o;
            self = prod;
            exp >>= 1;
        }
        let (prod, o) = self.overflowing_mul(y);
        (prod, o || overflow)
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    /// Returns a tuple of the addition (with a signed integer of the same bit width) along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(n!(1U512).overflowing_add_signed(n!(1)), (n!(2), false));
    /// assert_eq!(U512::MAX.overflowing_add_signed(n!(1)), (n!(0), true));
    /// assert_eq!(n!(1U512).overflowing_add_signed(n!(-2)), (U512::MAX, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add_signed(self, rhs: Int<N, B, OM>) -> (Self, bool) {
        let (sum, overflow) = self.overflowing_add(rhs.cast_unsigned());
        (sum, rhs.is_negative() != overflow)
    }

    /// Returns a tuple of the subtraction (with a signed integer of the same bit width) along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    ///
    /// assert_eq!(n!(1U2048).overflowing_sub_signed(n!(-1)), (n!(2), false));
    /// assert_eq!(U2048::MAX.overflowing_sub_signed(n!(-1)), (n!(0), true));
    /// assert_eq!(n!(1U2048).overflowing_sub_signed(n!(2)), (U2048::MAX, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub_signed(self, rhs: Int<N, B, OM>) -> (Self, bool) {
        let (diff, overflow) = self.overflowing_sub(rhs.cast_unsigned());
        (diff, rhs.is_negative() != overflow)
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    /// Returns a tuple of the subtraction (with an unsigned integer of the same bit width) along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(n!(-1I256).overflowing_add_unsigned(n!(1)), (n!(0), false));
    /// assert_eq!(I256::MAX.overflowing_add_unsigned(n!(1)), (I256::MIN, true));
    /// assert_eq!(I256::MIN.overflowing_add_unsigned(U256::MAX), (I256::MAX, false));
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add_unsigned(self, rhs: Uint<N, B, OM>) -> (Self, bool) {
        let rhs = rhs.cast_signed();
        let (sum, overflow) = self.overflowing_add(rhs);
        (sum, rhs.is_negative() != overflow)
    }

    /// Returns a tuple of the subtraction (with an unsigned integer of the same bit width) along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// assert_eq!(n!(1I512).overflowing_sub_unsigned(n!(1)), (n!(0), false));
    /// assert_eq!(I512::MIN.overflowing_sub_unsigned(n!(1)), (I512::MAX, true));
    /// assert_eq!(I512::MAX.overflowing_sub_unsigned(U512::MAX), (I512::MIN, false));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub_unsigned(self, rhs: Uint<N, B, OM>) -> (Self, bool) {
        let rhs = rhs.cast_signed();
        let (sum, overflow) = self.overflowing_sub(rhs);
        (sum, rhs.is_negative() != overflow)
    }

    /// Returns a tuple of the absolute value of `self` along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned (this can only happen when `self` equals `Self::MIN`, in which case `Self::MIN` is returned).
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I1024;
    ///
    /// assert_eq!(n!(-123I1024).overflowing_abs(), (n!(123), false));
    /// assert_eq!(n!(456I1024).overflowing_abs(), (n!(456), false));
    /// assert_eq!(I1024::MIN.overflowing_abs(), (I1024::MIN, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing unsigned;

        test_bignum! {
            function: <utest>::overflowing_add_signed(a: utest, b: itest)
        }
        test_bignum! {
            function: <utest>::overflowing_sub_signed(a: utest, b: itest)
        }
    }

    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <itest>::overflowing_add_unsigned(a: itest, b: utest)
        }
        test_bignum! {
            function: <itest>::overflowing_sub_unsigned(a: itest, b: utest)
        }
        test_bignum! {
            function: <itest>::overflowing_abs(a: itest),
            cases: [
                (0i8),
                (itest::MIN)
            ]
        }
    }

    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <stest>::overflowing_add(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::overflowing_sub(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::overflowing_mul(a: stest, b: stest),
            cases: [(256u16, 1u16)]
        }
        test_bignum! {
            function: <stest>::overflowing_div(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::overflowing_div_euclid(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::overflowing_rem(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::overflowing_rem_euclid(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::overflowing_neg(a: stest)
        }
        test_bignum! {
            function: <stest>::overflowing_shl(a: stest, b: u16)
        }
        test_bignum! {
            function: <stest>::overflowing_shr(a: stest, b: u16)
        }
        test_bignum! {
            function: <stest>::overflowing_pow(a: stest, b: u16),
            cases: [(2, 512)]
        }
    }
}

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

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
        function: <utest>::overflowing_shr(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::overflowing_shl(a: utest, b: u16)
    }
    test_bignum! {
        function: <itest>::overflowing_shr(a: itest, b: u16)
    }
}
