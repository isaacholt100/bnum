use super::Uint;
use crate::{Integer, Int};
use crate::Exponent;
use crate::doc;

macro_rules! impl_desc {
    () => {
        "Saturating arithmetic methods which act on `self`: `self.saturating_...`. For each method, if overflow occurs, the largest or smallest value that can be represented by `Self` is returned instead."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Saturating integer addition. Computes `self + rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`, or `Self::MIN` if the result is too small to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    /// 
    /// assert_eq!(n!(1U1024).saturating_add(n!(1)), n!(2));
    /// assert_eq!(U1024::MAX.saturating_add(U1024::MAX), U1024::MAX);
    /// 
    /// assert_eq!(I1024::MIN.saturating_add(n!(-1)), I1024::MIN);
    /// assert_eq!(I1024::MAX.saturating_add(n!(1)), I1024::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(add) => add,
            None => {
                if self.is_negative_internal() {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
        }
    }

    /// Saturating integer subtraction. Computes `self - rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`, or `Self::MIN` if the result is too small to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(n!(1U256).saturating_sub(n!(1)), n!(0));
    /// assert_eq!(n!(1U256).saturating_sub(n!(2)), n!(0));
    /// 
    /// assert_eq!(I256::MIN.saturating_sub(I256::MAX), I256::MIN);
    /// assert_eq!(I256::MAX.saturating_sub(n!(-1)), I256::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        match self.checked_sub(rhs) {
            Some(sub) => sub,
            None => {
                if rhs.is_negative_internal() {
                    Self::MAX
                } else {
                    Self::MIN
                }
            }
        }
    }

    /// Saturating integer multiplication. Computes `self * rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`, or `Self::MIN` if the result is too small to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    /// 
    /// assert_eq!(n!(1U1024).saturating_mul(n!(1)), n!(1));
    /// assert_eq!(U1024::MAX.saturating_mul(n!(2)), U1024::MAX);
    /// 
    /// assert_eq!(I1024::MIN.saturating_mul(n!(2)), I1024::MIN);
    /// assert_eq!(I1024::MAX.saturating_mul(n!(3)), I1024::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        match self.checked_mul(rhs) {
            Some(mul) => mul,
            None => {
                if self.is_negative_internal() == rhs.is_negative_internal() {
                    Self::MAX
                } else {
                    Self::MIN
                }
            }
        }
    }

    /// Saturating integer division. The only time the result can saturate is if the integer is signed, and `self` is [`Self::MIN`] and `rhs` is `-1`, in which case the result is [`Self::MAX`].
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
    /// use bnum::types::I512;
    /// 
    /// assert_eq!(n!(5U512).saturating_div(n!(2)), n!(2));
    /// assert_eq!(I512::MIN.saturating_div(n!(-1)), I512::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_div(self, rhs: Self) -> Self {
        let (div, overflow) = self.overflowing_div(rhs);
        if overflow { Self::MAX } else { div }
    }

    /// Saturating Euclidean integer division. The only time the result can saturate is if the integer is signed, and `self` is [`Self::MIN`] and `rhs` is `-1`, in which case the result is [`Self::MAX`].
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
    /// use bnum::types::I512;
    /// 
    /// assert_eq!(n!(19U512).saturating_div_euclid(n!(5)), n!(3));
    /// assert_eq!(I512::MIN.saturating_div_euclid(n!(-1)), I512::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_div_euclid(self, rhs: Self) -> Self {
        let (div, overflow) = self.overflowing_div_euclid(rhs);
        if overflow { Self::MAX } else { div }
    }

    // these are not exported from the crate since the Rust primitives do not define them
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub(crate) const fn saturating_rem(self, rhs: Self) -> Self {
        self.rem(rhs)
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub(crate) const fn saturating_rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid(rhs)
    }

    /// Saturating exponentiation. Computes `self.pow(exp)`, returning `Self::MAX` if the result is too large to be represented by `Self`, or `Self::MIN` if the result is too small to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(n!(2U256).saturating_pow(8), n!(256));
    /// assert_eq!(n!(2U256).saturating_pow(256), U256::MAX);
    /// 
    /// assert_eq!(n!(-2I256).saturating_pow(257), I256::MIN);
    /// assert_eq!(n!(-2I256).saturating_pow(256), I256::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_pow(self, exp: Exponent) -> Self {
        match self.checked_pow(exp) {
            Some(pow) => pow,
            None => {
                if self.is_negative_internal() && exp % 2 != 0 {
                    Self::MIN
                } else {
                    Self::MAX
                }
            }
        }
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    #[inline]
    const fn saturate_up(o: Option<Self>) -> Self {
        match o {
            Some(int) => int,
            None => Self::MAX,
        }
    }

    #[inline]
    const fn saturate_down(o: Option<Self>) -> Self {
        match o {
            Some(int) => int,
            None => Self::MIN,
        }
    }

    /// Saturating addition with a signed integer of the same bit width. Computes `self + rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`, or `Self::MIN` if the result is negative.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(n!(1U512).saturating_add_signed(n!(-1)), n!(0));
    /// assert_eq!(U512::MAX.saturating_add_signed(n!(1)), U512::MAX);
    /// assert_eq!(n!(1U512).saturating_add_signed(n!(-2)), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_add_signed(self, rhs: Int<N, B, OM>) -> Self {
        if rhs.is_negative() {
            Self::saturate_down(self.checked_add_signed(rhs))
        } else {
            Self::saturate_up(self.checked_add_signed(rhs))
        }
    }

    /// Saturating subtraction with a signed integer of the same bit width. Computes `self - rhs`, returning `Self::MIN` if the result is negative, or `Self::MAX` if the result is too large to be represented by `Self`.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(n!(1U2048).saturating_sub_signed(n!(-1)), n!(2));
    /// assert_eq!(U2048::MAX.saturating_sub_signed(n!(-4)), U2048::MAX);
    /// assert_eq!(n!(1U2048).saturating_sub_signed(n!(3)), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_sub_signed(self, rhs: Int<N, B, OM>) -> Self
    {
        if rhs.is_negative() {
            Self::saturate_up(self.checked_sub_signed(rhs))
        } else {
            Self::saturate_down(self.checked_sub_signed(rhs))
        }
    }

    // not exported from the crate since the Rust primitives do not define it
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub(crate) const fn saturating_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::MAX,
        }
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    /// Saturating addition with an unsigned integer of the same bit width. Computes `self + rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{I512, U512};
    /// 
    /// assert_eq!(n!(-1I512).saturating_add_unsigned(n!(1)), n!(0));
    /// assert_eq!(n!(0I512).saturating_add_unsigned(U512::MAX), I512::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_add_unsigned(self, rhs: Uint<N, B, OM>) -> Self {
        match self.checked_add_unsigned(rhs) {
            Some(i) => i,
            None => Self::MAX,
        }
    }

    /// Saturating subtraction with an unsigned integer of the same bit width. Computes `self + rhs`, returning `Self::MIN` if the result is too small to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I1024;
    /// 
    /// assert_eq!(n!(1I1024).saturating_sub_unsigned(n!(1)), n!(0));
    /// assert_eq!(I1024::MIN.saturating_sub_unsigned(n!(1)), I1024::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_sub_unsigned(self, rhs: Uint<N, B, OM>) -> Self {
        match self.checked_sub_unsigned(rhs) {
            Some(i) => i,
            None => Self::MIN,
        }
    }

    /// Computes the absolute value of `self`, returning [`Self::MAX`] if the result is too large to be represented by `Self`. The only time this can happen is if `self` is [`Self::MIN`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I2048;
    /// 
    /// assert_eq!(n!(-7I2048).saturating_abs(), n!(7));
    /// assert_eq!(n!(22I2048).saturating_abs(), n!(22));
    /// assert_eq!(I2048::MIN.saturating_abs(), I2048::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_abs(self) -> Self {
        match self.checked_abs() {
            Some(abs) => abs,
            None => Self::MAX,
        }
    }

    /// Saturating negation. Computes `-self`, returning [`Self::MAX`] if the result is too large to be represented by `Self`. The only time this can happen is if `self` is [`Self::MIN`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I256;
    /// 
    /// assert_eq!(n!(7I256).saturating_neg(), n!(-7));
    /// assert_eq!(n!(-22I256).saturating_neg(), n!(22));
    /// assert_eq!(I256::MIN.saturating_neg(), I256::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_neg(self) -> Self {
        match self.checked_neg() {
            Some(abs) => abs,
            None => Self::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing unsigned;

        test_bignum! {
            function: <utest>::saturating_add_signed(a: utest, b: itest)
        }
        test_bignum! {
            function: <utest>::saturating_sub_signed(a: utest, b: itest)
        }
    }
    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <itest>::saturating_add_unsigned(a: itest, b: utest)
        }
        test_bignum! {
            function: <itest>::saturating_sub_unsigned(a: itest, b: utest)
        }
        test_bignum! {
            function: <itest>::saturating_neg(a: itest),
            cases: [
                (itest::MIN)
            ]
        }
        test_bignum! {
            function: <itest>::saturating_abs(a: itest),
            cases: [
                (itest::MIN)
            ]
        }
    }
    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <stest>::saturating_add(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::saturating_sub(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::saturating_mul(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::saturating_div(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::saturating_pow(a: stest, b: u16)
        }
    }
}
