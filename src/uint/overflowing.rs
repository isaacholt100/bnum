use super::Uint;
use crate::ExpType;
use crate::{Integer, Int};
use crate::digit;
use crate::doc;

macro_rules! impl_desc {
    () => {
        "Overflowing arithmetic methods which act on `self`: `self.overflowing_...`. Each method returns a tuple of type `(Self, bool)` where the first item of the tuple is the result of wrapping variant of the method (`self.wrapping_...`), and the second item is a boolean which indicates whether overflow would have occurred."
    };
}


#[doc = impl_desc!()]
impl<const S: bool, const N: usize> Integer<S, N> {
    /// Returns a tuple of the addition along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(1.as_::<U1024>().overflowing_add(1.as_()), (2.as_(), false));
    /// assert_eq!(U1024::MAX.overflowing_add(U1024::ONE), (0.as_(), true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        if S {
            let sum = self.force_sign::<false>().overflowing_add(rhs.force_sign()).0.force_sign(); // we don't care about the overflow here, and we use this instead of wrapping_add, since that would lead to infinite recursion
            let overflow = match (self.is_negative_internal(), rhs.is_negative_internal()) {
                (false, false) => sum.is_negative_internal(),
                (true, true) => !sum.is_negative_internal(),
                _ => false,
            };
            return (sum, overflow);
        }
        let mut out = Self::ZERO;
        let mut carry = false;
        let mut i = 0;
        let mut result = (0, false);

        unsafe {
            while i < Self::U128_DIGITS {
                result = digit::carrying_add_u128(
                    self.as_wide_digits().get(i),
                    rhs.as_wide_digits().get(i),
                    carry,
                );
                out.as_wide_digits_mut().set(i, result.0);
                carry = result.1;
                i += 1;
            }
        }
        if Self::U128_DIGIT_REMAINDER != 0 {
            carry = (128 - result.0.leading_zeros()) > (Self::U128_DIGIT_REMAINDER as u32) * 8;
        }
        (out, carry)
    }

    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(1.as_::<U256>().overflowing_sub(1.as_()), (0.as_(), false));
    /// assert_eq!(U256::MIN.overflowing_sub(U256::ONE), (U256::MAX, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        if S {
            let sub = self.force_sign::<false>().overflowing_sub(rhs.force_sign()).0.force_sign(); // we don't care about the overflow here, and we use this instead of wrapping_sub, since that would lead to infinite recursion
            let overflow = match (self.is_negative_internal(), rhs.is_negative_internal()) {
                (false, true) => sub.is_negative_internal(),
                (true, false) => !sub.is_negative_internal(),
                _ => false,
            };
            return (sub, overflow);
        }

        let mut out = Self::ZERO;
        let mut borrow = false;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                // the last full u128 digits cause an overflow iff the truncated last digits cause an overflow
                let result = digit::borrowing_sub_u128(
                    self.as_wide_digits().get(i),
                    rhs.as_wide_digits().get(i),
                    borrow,
                );
                out.as_wide_digits_mut().set(i, result.0);
                borrow = result.1;
                i += 1;
            }
        }
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
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(1.as_::<U512>().overflowing_mul(1.as_()), (1.as_(), false));
    /// assert_eq!(U512::power_of_two(511).overflowing_mul(2.as_()), (0.as_(), true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        if S {
            // TODO: might be possible to do this without using abs, as the wrapping calculation is the same as just casting to unsigned
            let (uint, overflow) = self.unsigned_abs_internal().overflowing_mul(rhs.unsigned_abs_internal());
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
        let (m, o) = self.force_sign().long_mul(rhs.force_sign());
        (m.force_sign(), o)
    }

    /// Returns a tuple of the division along with a boolean indicating whether an arithmetic overflow would occur. Note that the second item of the tuple is always `false` since the division only involves non-negative integers.
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
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(5.as_::<U256>().overflowing_div(2.as_()), (2.as_(), false));
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

    /// Returns a tuple of the Euclidean division along with a boolean indicating whether an arithmetic overflow would occur. Note that the second item of the tuple is always `false` since the division only involves non-negative integers.
    /// 
    /// Note that this is equivalent to `self.overflowing_div(rhs)`, since the division only involves non-negative integers.
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
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(13.as_::<U2048>().overflowing_div_euclid(5.as_()), (2.as_(), false));
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

    /// Returns a tuple of the remainder along with a boolean indicating whether an arithmetic overflow would occur. Note that the second item of the tuple is always `false` since the calculation only involves non-negative integers.
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
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(5.as_::<U1024>().overflowing_rem(2.as_()), (1.as_(), false));
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

    /// Returns a tuple of the Euclidean remainder along with a boolean indicating whether an arithmetic overflow would occur. Note that the second item of the tuple is always `false` since the calculation only involves non-negative integers.
    /// 
    /// Note that this is equivalent to `self.overflowing_rem(rhs)`, since the calculation only involves non-negative integers.
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
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(13.as_::<U512>().overflowing_rem_euclid(5.as_()), (3.as_(), false));
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
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(1.as_::<U256>().overflowing_neg(), (U256::MAX, true));
    /// assert_eq!(0.as_::<U256>().overflowing_neg(), (0.as_(), false));
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
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(1.as_::<U2048>().overflowing_shl(1), (2.as_(), false));
    /// assert_eq!(1.as_::<U2048>().overflowing_shl(2049), (2.as_(), true));
    /// assert_eq!(1.as_::<U2048>().overflowing_shl(2048), (1.as_(), true));
    /// ```
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

    /// Returns a tuple of the right shift along with a boolean indicating whether `rhs` is greater than or equal to `Self::BITS`. If `rhs >= Self::BITS` then the returned value is `self` right-shifted by `rhs % Self::BITS`.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(1.as_::<U1024>().overflowing_shr(1), (0.as_(), false));
    /// assert_eq!(2.as_::<U1024>().overflowing_shr(1025), (1.as_(), true));
    /// assert_eq!(U1024::MAX.overflowing_shr(1024), (U1024::MAX, true));
    /// assert_eq!(U1024::MAX.overflowing_shr(1023), (1.as_(), false));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_shr(self, rhs: ExpType) -> (Self, bool) {
        let (overflow, shift) = if rhs >= Self::BITS {
            (true, rhs % Self::BITS) // can't use & as bits may not be power of two
        } else {
            (false, rhs)
        };
        let out = unsafe {
            if self.is_negative_internal() {
                self.unchecked_shr_pad_internal::<true>(shift)
            } else {
                self.unchecked_shr_pad_internal::<false>(shift)
            }
        };
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
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(2.as_::<U512>().overflowing_pow(10), (1024.as_(), false));
    /// assert_eq!(2.as_::<U512>().overflowing_pow(512), (0.as_(), true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_pow(mut self, mut exp: ExpType) -> (Self, bool) {
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
impl<const N: usize> Uint<N> {
    /// Returns a tuple of the addition (with a signed integer of the same bit width) along with a boolean indicating whether an arithmetic overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(1.as_::<U512>().overflowing_add_signed(1.as_()), (2.as_(), false));
    /// assert_eq!(U512::MAX.overflowing_add_signed(U512::ONE), (0.as_(), true));
    /// assert_eq!(1.as_::<U512>().overflowing_add_signed(-2.as_()), (U512::MAX, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add_signed(self, rhs: crate::Int<N>) -> (Self, bool) {
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
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(1.as_::<U2048>().overflowing_sub_signed(-1.as_()), (2.as_(), false));
    /// assert_eq!(U2048::MAX.overflowing_sub_signed(-1.as_()), (0.as_(), true));
    /// assert_eq!(1.as_::<U2048>().overflowing_sub_signed(2.as_()), (U2048::MAX, true));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub_signed(self, rhs: crate::Int<N>) -> (Self, bool) {
        let (diff, overflow) = self.overflowing_sub(rhs.cast_unsigned());
        (diff, rhs.is_negative() != overflow)
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize> Int<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_add_unsigned(self, rhs: Uint<N>) -> (Self, bool) {
        let rhs = rhs.cast_signed();
        let (sum, overflow) = self.overflowing_add(rhs);
        (sum, rhs.is_negative() != overflow)
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn overflowing_sub_unsigned(self, rhs: Uint<N>) -> (Self, bool) {
        let rhs = rhs.cast_signed();
        let (sum, overflow) = self.overflowing_sub(rhs);
        (sum, rhs.is_negative() != overflow)
    }

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
            function: <stest>::overflowing_pow(a: stest, b: u16)
        }
    }
}

#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
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
        function: <utest>::overflowing_shl(a: utest, b: u16),
        cases: [
            (utest::from_str_radix("20550931191544903", 10).unwrap(), 56u16)
        ]
    }
    test_bignum! {
        function: <itest>::overflowing_shr(a: itest, b: u16)
    }
}