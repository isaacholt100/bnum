use super::{Uint, Integer};
use crate::doc;
use crate::ExpType;
use crate::digit::Digit;
use crate::errors;
use crate::Int;

macro_rules! impl_desc {
    () => {
        "Mathematical methods."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize> Integer<S, N> {
    #[inline(always)]
    pub(crate) const fn is_negative_internal(&self) -> bool {
        S && (self.bytes[N - 1] as i8).is_negative()
    }

    #[inline(always)]
    pub(crate) const fn unsigned_abs_internal(self) -> Uint<N> {
        if self.is_negative_internal() {
            self.wrapping_neg().force_sign::<false>()
        } else {
            self.force_sign::<false>()
        }
    }

    /// Returns `self` raised to the power of `exp`. In debug builds, this method is equivalent to [`strict_pow`](Self::strict_pow). In release builds, this method is equivalent to [`wrapping_pow`](Self::wrapping_pow).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    ///
    /// assert_eq!(3.as_::<U256>().pow(5), 243.as_());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn pow(self, exp: ExpType) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_pow(exp)
        } else {
            self.wrapping_pow(exp)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_div_euclid(rhs)
        } else {
            self.wrapping_div_euclid(rhs)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_rem_euclid(rhs)
        } else {
            self.wrapping_rem_euclid(rhs)
        }
    }

    /// Returns `true` if and only if `self == 2^k` for some integer `k`.
    #[must_use]
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
        if self.is_negative_internal() {
            return false;
        }
        let mut i = 0;
        let mut ones = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                ones += self.as_wide_digits().get(i).count_ones();
                if ones > 1 {
                    return false;
                }
                i += 1;
            }
        }
        ones == 1
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn midpoint(self, rhs: Self) -> Self {
        // see section 2.5: Average of Two Integers in Hacker's Delight
        let x = self.bitxor(rhs);
        let t = self.bitand(rhs).add(x.shr(1));
        if t.is_negative_internal() && x.bytes[0] % 2 == 1 {
            // t is negative and x is odd
            t.add(Self::ONE)
        } else {
            t
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog2(self) -> ExpType {
        self.checked_ilog2()
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog10(self) -> ExpType {
        self.checked_ilog10()
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog(self, base: Self) -> ExpType {
        if base.le(&Self::ONE) {
            panic!("{}", errors::err_msg!(errors::invalid_log_base_message!()));
        }
        self.checked_ilog(base)
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn abs_diff(self, other: Self) -> Uint<N> {
        let diff = if self.lt(&other) {
            other.wrapping_sub(self)
        } else {
            self.wrapping_sub(other)
        };
        Uint::from_bytes(diff.bytes)
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
        let rem = self.wrapping_rem_euclid(rhs);
        if rem.is_zero() {
            self
        } else if rem.is_negative_internal() == rhs.is_negative_internal() {
            self.add(rhs.sub(rem))
        } else {
            self.sub(rem)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div_floor(self, rhs: Self) -> Self {
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(
                crate::errors::div_by_zero_message!()
            ));
        }
        if !S {
            return self.div(rhs);
        }
        let (div, rem) = self.div_rem_unchecked(rhs);
        if self.is_negative_internal() == rhs.is_negative_internal() || rem.is_zero() {
            div
        } else {
            div.sub(Self::ONE)
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div_ceil(self, rhs: Self) -> Self {
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(
                crate::errors::div_by_zero_message!()
            ));
        }
        let (div, rem) = self.div_rem_unchecked(rhs);
        if self.is_negative_internal() != rhs.is_negative_internal() || rem.is_zero() {
            div
        } else {
            div.add(Self::ONE)
        }
    }

    /// Returns whether or not `self` equals zero.
    #[must_use]
    #[inline]
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                if self.as_wide_digits().get(i) != 0 {
                    return false;
                }
                i += 1;
            }
        }
        true
    }

    /// Returns whether or not `self` equals one.
    #[must_use]
    #[inline]
    pub const fn is_one(&self) -> bool {
        if S && Self::BITS == 1 {
            return false;
        }
        if Self::U128_DIGITS == 1 {
            return self.as_wide_digits().last() == 1;
        }
        unsafe {
            if self.as_wide_digits().get(0) != 1 {
                return false;
            }
            let mut i = 1;
            while i < Self::U128_DIGITS {
                if self.as_wide_digits().get(i) != 0 {
                    return false;
                }
                i += 1;
            }
        }
        true
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize> Uint<N> {
    /// Casts `self` to a signed integer type of the same bit width, leaving the memory representation unchanged.
    ///
    /// This is function equivalent to using the [`As`](crate::cast::As) trait to cast `self` to [`Int<N>`](crate::Int).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(U256::MAX.cast_signed(), I256::NEG_ONE);
    /// assert_eq!(U256::ZERO.cast_signed(), I256::ZERO);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cast_signed(self) -> crate::Int<N> {
        self.force_sign::<true>()
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.checked_next_power_of_two().expect(errors::err_msg!(
                "attempt to calculate next power of two with overflow"
            ))
        } else {
            self.wrapping_next_power_of_two()
        }
    }

    /// Returns an integer whose value is `2.pow(power)`. This is faster than using a shift left on `Self::ONE` or using the [`pow`](Self::pow) function.
    ///
    /// # Panics
    ///
    /// This function will panic if `power` is greater than or equal to `Self::BITS`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::types::U256;
    ///
    /// assert_eq!(U256::power_of_two(11), U256::ONE << 11);
    /// ```
    #[must_use]
    #[inline]
    pub const fn power_of_two(power: ExpType) -> Self {
        assert!(
            power < Self::BITS,
            crate::errors::err_msg!("power of two must be less than `Self::BITS`")
        );

        let mut out = Self::ZERO;
        out.bytes[power as usize / Digit::BITS as usize] = 1 << (power % Digit::BITS);
        out
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize> Int<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cast_unsigned(self) -> Uint<N> {
        Uint::from_bytes(self.bytes)
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn unsigned_abs(self) -> Uint<N> {
        if self.is_negative() {
            self.wrapping_neg().cast_unsigned()
        } else {
            self.cast_unsigned()
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn abs(self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_abs()
        } else {
            self.wrapping_abs()
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn signum(self) -> Self {
        if self.is_negative() {
            Self::NEG_ONE
        } else if self.is_zero() {
            Self::ZERO
        } else {
            Self::ONE
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn is_positive(self) -> bool {
        let signed_digit = self.signed_digit();
        signed_digit.is_positive() || (signed_digit == 0 && !self.is_zero())
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn is_negative(self) -> bool {
        self.signed_digit().is_negative()
    }
}

use core::iter::{Iterator, Product, Sum};

impl<const S: bool, const N: usize> Product<Self> for Integer<S, N> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const S: bool, const N: usize> Product<&'a Self> for Integer<S, N> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const S: bool, const N: usize> Sum<Self> for Integer<S, N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const S: bool, const N: usize> Sum<&'a Self> for Integer<S, N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::{test_bignum, debug_skip};
    use crate::cast::CastFrom;

    crate::test::test_all! {
        testing integers;

        #[test]
        fn is_zero() {
            assert!(UTEST::MIN.is_zero());
            assert!(!UTEST::MAX.is_zero());
            assert!(!UTEST::ONE.is_zero());
        }

        #[test]
        fn is_one() {
            assert!(UTEST::ONE.is_one());
            assert!(!UTEST::MAX.is_one());
            assert!(!UTEST::ZERO.is_one());
            let mut digits = *crate::Uint::<2>::MAX.as_bytes();
            digits[0] = 1;
            let b = crate::Uint::<2>::from_bytes(digits);
            assert!(!b.is_one());
        }

        #[test]
        fn sum() {
            let v = vec![
                UTEST::ZERO,
                UTEST::ONE,
                UTEST::from_byte(2),
                UTEST::from_byte(3),
                UTEST::from_byte(4),
            ];

            assert_eq!(UTEST::from_byte(10), v.iter().sum());
            assert_eq!(UTEST::from_byte(10), v.into_iter().sum());
        }

        #[test]
        fn product() {
            let v = vec![UTEST::ONE, UTEST::from_byte(2), UTEST::from_byte(3)];

            assert_eq!(UTEST::from_byte(6), v.iter().sum());
            assert_eq!(UTEST::from_byte(6), v.into_iter().sum());
        }

        test_bignum! {
            function: <stest>::pow(a: stest, b: u16),
            skip: crate::test::debug_skip!(a.checked_pow(b as u32).is_none())
        }
        test_bignum! {
            function: <stest>::div_euclid(a: stest, b: stest),
            skip: a.checked_div_euclid(b).is_none()
        }
        test_bignum! {
            function: <stest>::rem_euclid(a: stest, b: stest),
            skip: a.checked_rem_euclid(b).is_none()
        }
        test_bignum! {
            function: <stest>::abs_diff(a: stest, b: stest)
        }
        #[cfg(feature = "nightly")] // as num_midpoint_signed not yet stabilised
        test_bignum! {
            function: <stest>::midpoint(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::ilog(a: stest, base: stest),
            skip: a <= 0 || base <= 1
        }
        test_bignum! {
            function: <stest>::ilog2(a: stest),
            skip: a <= 0
        }
        test_bignum! {
            function: <stest>::ilog10(a: stest),
            skip: a <= 0
        }
        #[cfg(feature = "nightly")] // since int_roundings are not stable yet
        test_bignum! {
            function: <stest>::next_multiple_of(a: stest, b: stest),
            skip: crate::test::debug_skip!(a.checked_next_multiple_of(b).is_none()) || b == 0
        }
        #[cfg(feature = "nightly")] // since int_roundings are not stable yet
        test_bignum! {
            function: <stest>::div_floor(a: stest, b: stest),
            skip: a.checked_div(b).is_none()
        }
        #[cfg(feature = "nightly")] // since int_roundings are not stable yet
        test_bignum! {
            function: <stest>::div_ceil(a: stest, b: stest),
            skip: a.checked_div(b).is_none()
        }
    }
    crate::test::test_all! {
        testing unsigned;

        test_bignum! {
            function: <utest>::is_power_of_two(a: utest)
        }
        test_bignum! {
            function: <utest>::next_power_of_two(a: utest),
            skip: debug_skip!(a.checked_next_power_of_two().is_none())
        }
        test_bignum! {
            function: <utest>::cast_signed(a: utest)
        }
    }
    crate::test::test_all! {
        testing signed;

        #[test]
        fn is_power_of_two() {
            assert!(!ITEST::cast_from(-1273i16).is_power_of_two());
            assert!(!ITEST::cast_from(8945i16).is_power_of_two());
            assert!(ITEST::cast_from(1i16 << 14).is_power_of_two());
        }

        test_bignum! {
            function: <itest>::unsigned_abs(a: itest),
            cases: [
                (itest::MIN)
            ]
        }
        test_bignum! {
            function: <itest>::abs(a: itest),
            skip: debug_skip!(a == itest::MIN)
        }
        test_bignum! {
            function: <itest>::signum(a: itest)
        }
        test_bignum! {
            function: <itest>::is_positive(a: itest)
        }
        test_bignum! {
            function: <itest>::is_negative(a: itest)
        }
        test_bignum! {
            function: <itest>::cast_unsigned(a: itest)
        }
    }
}

#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::is_power_of_two(a: utest)
    }
}