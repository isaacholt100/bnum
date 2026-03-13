use crate::Exponent;
use crate::OverflowMode;
use crate::doc;
use crate::errors;
use crate::{Byte, Int, Integer, Uint};

macro_rules! impl_desc {
    () => {
        "Mathematical methods."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    #[inline(always)]
    pub(crate) const fn is_negative_internal(&self) -> bool {
        S && self.bit(Self::BITS - 1)
    }

    #[inline(always)]
    pub(crate) const fn unsigned_abs_internal(self) -> Uint<N, B, OM> {
        if self.is_negative_internal() {
            self.wrapping_neg().force_sign::<false>()
        } else {
            self.force_sign::<false>()
        }
    }

    /// Returns `self` raised to the power of `exp`.
    ///
    /// # Overflow behaviour
    ///
    /// - If [`Self::OVERFLOW_MODE`] is [`Wrapping`](OverflowMode::Wrapping), this method is equivalent to [`wrapping_pow`](Self::wrapping_pow).
    /// - If [`Self::OVERFLOW_MODE`] is [`Panicking`](OverflowMode::Panicking), this method is equivalent to [`strict_pow`](Self::strict_pow).
    /// - If [`Self::OVERFLOW_MODE`] is [`Saturating`](OverflowMode::Saturating), this method is equivalent to [`saturating_pow`](Self::saturating_pow).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(n!(3U256).pow(5), n!(243));
    /// assert_eq!(n!(-7I256).pow(3), n!(-343));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn pow(self, exp: Exponent) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_pow(exp),
            OverflowMode::Panicking => self.strict_pow(exp),
            OverflowMode::Saturating => self.saturating_pow(exp),
        }
    }

    /// Returns the [Euclidean quotient](https://en.wikipedia.org/wiki/Euclidean_division) of `self` by `rhs`.
    ///
    /// # Overflow behaviour
    ///
    /// - If [`Self::OVERFLOW_MODE`] is [`Wrapping`](OverflowMode::Wrapping), this method is equivalent to [`wrapping_div_euclid`](Self::wrapping_div_euclid).
    /// - If [`Self::OVERFLOW_MODE`] is [`Panicking`](OverflowMode::Panicking), this method is equivalent to [`strict_div_euclid`](Self::strict_div_euclid).
    /// - If [`Self::OVERFLOW_MODE`] is [`Saturating`](OverflowMode::Saturating), this method is equivalent to [`saturating_div_euclid`](Self::saturating_div_euclid).
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(37U512).div_euclid(n!(8)), n!(4));
    /// assert_eq!(n!(-37I512).div_euclid(n!(8)), n!(-5));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_div_euclid(rhs),
            OverflowMode::Panicking => self.strict_div_euclid(rhs),
            OverflowMode::Saturating => self.saturating_div_euclid(rhs),
        }
    }

    /// Returns the [Euclidean remainder](https://en.wikipedia.org/wiki/Euclidean_division) of `self` by `rhs`. This is always equivalent to `self.strict_rem_euclid(rhs)`, regardless of `Self::OVERFLOW_MODE`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(37U1024).rem_euclid(n!(8)), n!(5));
    /// assert_eq!(n!(-37I1024).rem_euclid(n!(-8)), n!(3));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_rem_euclid(rhs),
            OverflowMode::Panicking => self.strict_rem_euclid(rhs),
            OverflowMode::Saturating => self.saturating_rem_euclid(rhs),
        }
    }

    /// Returns `true` if and only if `self == 2^k` for some integer `k`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert!(n!(16U2048).is_power_of_two());
    /// assert!(!n!(-8I2048).is_power_of_two());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
        if self.is_negative_internal() {
            return false;
        }
        self.to_digits::<u32>().is_power_of_two() // u32 is fastest
    }

    #[inline]
    pub(crate) const fn is_even(&self) -> bool {
        (self.bytes[0] & 1) == 0
    }

    /// Computes the arithmetic mean of `self` and `rhs`, rounded towards zero (i.e. `(self + rhs) / 2`), without the possibility of overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(10U256).midpoint(n!(22)), n!(16));
    /// assert_eq!(n!(12U256).midpoint(n!(21)), n!(16));
    /// assert_eq!(n!(-10I256).midpoint(n!(2)), n!(-4));
    /// assert_eq!(n!(-13I256).midpoint(n!(0)), n!(-6));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn midpoint(self, rhs: Self) -> Self {
        // see section 2.5: Average of Two Integers in Hacker's Delight
        let x = self.bitxor(rhs);
        let t = self.bitand(rhs).add(x.shr(1));
        if t.is_negative_internal() && !x.is_even() {
            // t is negative and x is odd
            t.add(Self::ONE)
        } else {
            t
        }
    }

    /// Computes the base-2 logarithm of `self`, rounded down, i.e. the largest integer `n` such that `2^n <= self`.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is less than or equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(16U512).ilog2(), 4);
    /// assert_eq!(n!(20I512).ilog2(), 4);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog2(self) -> Exponent {
        self.checked_ilog2()
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    /// Computes the base-10 logarithm of `self`, rounded down, i.e. the largest integer `n` such that `10^n <= self`.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is less than or equal to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(1000U512).ilog10(), 3);
    /// assert_eq!(n!(9999I512).ilog10(), 3);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog10(self) -> Exponent {
        self.checked_ilog10()
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    /// Computes logarithm of `self` to the given `base`, rounded down, i.e. the largest integer `n` such that `base^n <= self`.
    ///
    /// Note that you should use `ilog2` or `ilog10` for base-2 or base-10 logarithms respectively, as these are more efficient.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is less than or equal to zero, or if `base` is less than 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(243U1024).ilog(n!(3)), 5);
    /// assert_eq!(n!(124I1024).ilog(n!(5)), 2);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog(self, base: Self) -> Exponent {
        if base.le(&Self::ONE) {
            panic!("{}", errors::err_msg!(errors::invalid_log_base_message!()));
        }
        self.checked_ilog(base)
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    /// Computes the absolute difference between `self` and `other`, i.e. `|self - other|`, without the possibility of overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(12U2048).abs_diff(n!(30)), n!(18));
    /// assert_eq!(n!(-12I2048).abs_diff(n!(5)), n!(17));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn abs_diff(self, other: Self) -> Uint<N, B, OM> {
        let diff = if self.lt(&other) {
            other.wrapping_sub(self)
        } else {
            self.wrapping_sub(other)
        };
        diff.force_sign()
    }

    /// If `rhs` is positive, computes the smallest integer multiple of `rhs` that is greater than or equal to `self`. If `rhs` is negative, computes the largest integer multiple of `rhs` that is less than or equal to `self`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    ///
    /// This function will also panic if overflow occurs and [`Self::OVERFLOW_MODE`] is [`Panicking`](OverflowMode::Panicking).
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(20U256).next_multiple_of(n!(6)), n!(24));
    /// assert_eq!(n!(0U256).next_multiple_of(n!(5)), n!(0));
    ///
    /// assert_eq!(n!(18I256).next_multiple_of(n!(-4)), n!(16));
    /// assert_eq!(n!(-17I256).next_multiple_of(n!(-11)), n!(-22));
    /// ```
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

    /// Computes the quotient of `self` by `rhs`, rounding the result towards negative infinity.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero. For signed integers, it will also panic if `self` is [`Self::MIN`] and `rhs` is `-1`, since this would overflow. This behaviour is not affected by [`Self::OVERFLOW_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(37U512).div_floor(n!(8)), n!(4));
    /// assert_eq!(n!(-37I512).div_floor(n!(8)), n!(-5));
    /// ```
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
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to divide with overflow"));
        }
        let (div, rem) = self.div_rem_unchecked(rhs);
        if self.is_negative_internal() == rhs.is_negative_internal() || rem.is_zero() {
            div
        } else {
            div.sub(Self::ONE)
        }
    }

    /// Computes the quotient of `self` by `rhs`, rounding the result towards positive infinity.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero. For signed integers, it will also panic if `self` is [`Self::MIN`] and `rhs` is `-1`, since this would overflow. This behaviour is not affected by [`Self::OVERFLOW_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(37U512).div_ceil(n!(8)), n!(5));
    /// assert_eq!(n!(-37I512).div_ceil(n!(8)), n!(-4));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert!(n!(0U1024).is_zero());
    /// assert!(n!(0I1024).is_zero());
    ///
    /// assert!(!n!(1U1024).is_zero());
    /// assert!(!n!(-1I1024).is_zero());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.as_digits::<u64>().eq(&Self::ZERO.as_digits()) // u8 is as fast for random inputs but much slower when input is zero
    }

    /// Returns whether or not `self` equals one.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert!(n!(1U2048).is_one());
    /// assert!(n!(1I2048).is_one());
    ///
    /// assert!(!n!(0U2048).is_one());
    /// assert!(!n!(-1I2048).is_one());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_one(&self) -> bool {
        if S && Self::BITS == 1 {
            return false;
        }
        self.as_digits::<u64>().eq(&Self::ONE.as_digits())
    }

    #[inline]
    pub const fn isqrt(self) -> Self {
        // Newton's method
        // TODO: use Karatsuba square algorithm for larger bit widths
        if self.is_negative_internal() {
            panic!(crate::errors::err_msg!("imaginary square root"))
        }
        if self.is_zero() {
            return Self::ZERO;
        }
        let u = self.force_sign::<false>();
        let mut x = Uint::power_of_two(u.bit_width() / 2 + 1);
        loop {
            let y = x.midpoint(u.div(x)); // can't have overflow as x is strictly decreasing with each iteration
            if y.ge(&x) {
                return x.force_sign();
            }
            x = y;
        }
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    /// Casts `self` to a signed integer type of the same bit width, leaving the memory representation unchanged.
    ///
    /// This is function equivalent to using the [`As`](crate::cast::As) trait to cast `self` to [`Int<N, B, OM>`](crate::Int).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    ///
    /// assert_eq!(U256::MAX.cast_signed(), n!(-1I256));
    /// assert_eq!(n!(0U256).cast_signed(), n!(0I256));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cast_signed(self) -> Int<N, B, OM> {
        self.force_sign()
    }

    /// Returns the smallest power of two greater than or equal to `self`.
    ///
    /// # Panics
    ///
    /// This function will panic if [`Self::OVERFLOW_MODE`] is [`Panicking`](OverflowMode::Panicking) and the result would be too large to be represented by `Self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(20U256).next_power_of_two(), n!(32));
    /// assert_eq!(n!(16U256).next_power_of_two(), n!(16));
    /// assert_eq!(n!(0U256).next_power_of_two(), n!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_next_power_of_two(),
            OverflowMode::Panicking => self.strict_next_power_of_two(),
            OverflowMode::Saturating => self.saturating_next_power_of_two(),
        }
    }

    /// Returns an integer whose value is `2.pow(power)`. This is faster than using a shift left on `Self::ONE` or using the [`pow`](Self::pow) function.
    ///
    /// # Panics
    ///
    /// This function will panic if `power` is greater than or equal to [`Self::BITS`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    ///
    /// assert_eq!(U256::power_of_two(11), n!(1) << 11);
    /// ```
    #[must_use]
    #[inline]
    pub const fn power_of_two(power: Exponent) -> Self {
        assert!(
            power < Self::BITS,
            crate::errors::err_msg!("power of two must be less than `Self::BITS`")
        );

        let mut out = Self::ZERO;
        out.bytes[power as usize / Byte::BITS as usize] = 1 << (power % Byte::BITS);
        out
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    /// Casts `self` to an unsigned integer type of the same bit width, leaving the memory representation unchanged.
    ///
    /// This is function equivalent to using the [`As`](crate::cast::As) trait to cast `self` to [`Uint<N, B, OM>`](crate::Uint).
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    ///
    /// assert_eq!(n!(-1).cast_unsigned(), U512::MAX);
    /// assert_eq!(n!(0I512).cast_unsigned(), n!(0U512));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn cast_unsigned(self) -> Uint<N, B, OM> {
        self.force_sign()
    }

    /// Returns the absolute value of `self` as an unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I1024;
    ///
    /// assert_eq!(n!(-20I1024).unsigned_abs(), n!(20));
    /// assert_eq!(n!(15I1024).unsigned_abs(), n!(15));
    /// assert_eq!(I1024::MIN.unsigned_abs(), I1024::MIN.cast_unsigned());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn unsigned_abs(self) -> Uint<N, B, OM> {
        if self.is_negative() {
            self.wrapping_neg().cast_unsigned()
        } else {
            self.cast_unsigned()
        }
    }

    /// Returns the absolute value of `self`.
    ///
    /// # Overflow behaviour
    ///
    /// - If [`Self::OVERFLOW_MODE`] is [`Wrapping`](OverflowMode::Wrapping), this function will return `Self::MIN` on overflow (i.e. when `self` is `Self::MIN`).
    /// - If [`Self::OVERFLOW_MODE`] is [`Panicking`](OverflowMode::Panicking), this function will panic on overflow.
    /// - If [`Self::OVERFLOW_MODE`] is [`Saturating`](OverflowMode::Saturating), this function will return `Self::MAX` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(-20I2048).abs(), n!(20));
    /// assert_eq!(n!(15I2048).abs(), n!(15));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn abs(self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_abs(),
            OverflowMode::Panicking => self.strict_abs(),
            OverflowMode::Saturating => self.saturating_abs(),
        }
    }

    /// Returns the sign of `self` as a signed integer, i.e. `1` if `self` is positive, `-1` if `self` is negative, and `0` if `self` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert_eq!(n!(42I256).signum(), n!(1));
    /// assert_eq!(n!(-7I256).signum(), n!(-1));
    /// assert_eq!(n!(0I256).signum(), n!(0));
    /// ```
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

    /// Returns whether or not `self` is (strictly) positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert!(n!(314I512).is_positive());
    /// assert!(!n!(-159I512).is_positive());
    /// assert!(!n!(0I512).is_positive());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn is_positive(self) -> bool {
        let signed_digit = self.signed_digit();
        signed_digit.is_positive() || (signed_digit == 0 && !self.is_zero())
    }

    /// Returns whether or not `self` is (strictly) negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    ///
    /// assert!(n!(-271I512).is_negative());
    /// assert!(!n!(828I512).is_negative());
    /// assert!(!n!(0I512).is_negative());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn is_negative(self) -> bool {
        self.signed_digit().is_negative()
    }
}

use core::iter::{Iterator, Product, Sum};

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Product<Self>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const S: bool, const N: usize, const B: usize, const OM: u8> Product<&'a Self>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Sum<Self>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const S: bool, const N: usize, const B: usize, const OM: u8> Sum<&'a Self>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use crate::cast::CastFrom;
    use crate::test::{debug_skip, test_bignum};

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
            let mut digits = *crate::Uint::<2, 0>::MAX.as_bytes();
            digits[0] = 1;
            let b = crate::Uint::<2, 0>::from_bytes(digits);
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
        #[cfg(nightly)] // since int_roundings are not stable yet
        test_bignum! {
            function: <stest>::next_multiple_of(a: stest, b: stest),
            skip: crate::test::debug_skip!(a.checked_next_multiple_of(b).is_none()) || b == 0
        }
        #[cfg(nightly)] // since int_roundings are not stable yet
        test_bignum! {
            function: <stest>::div_floor(a: stest, b: stest),
            skip: a.checked_div(b).is_none()
        }
        #[cfg(nightly)] // since int_roundings are not stable yet
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
        test_bignum! {
            function: <utest>::isqrt(a: utest)
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
        test_bignum! {
            function: <itest>::isqrt(a: itest),
            skip: a < 0
        }
    }
}

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::is_power_of_two(a: utest)
    }
}
