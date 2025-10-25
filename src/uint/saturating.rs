use super::Uint;
use crate::{Integer, Int};
use crate::ExpType;
use crate::doc;

macro_rules! impl_desc {
    () => {
        "Saturating arithmetic methods which act on `self`: `self.saturating_...`. For each method, if overflow occurs, the largest or smallest value that can be represented by `Self` is returned instead."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize> Integer<S, N> {
    /// Saturating integer addition. Computes `self + rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(U1024::ONE.saturating_add(U1024::ONE), 2.as_());
    /// assert_eq!(U1024::MAX.saturating_add(U1024::MAX), U1024::MAX);
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

    /// Saturating integer subtraction. Computes `self - rhs`, returning `Self::ZERO` if the result is negative.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(U256::ONE.saturating_sub(U256::ONE), 0.as_());
    /// assert_eq!(U256::ONE.saturating_sub(2.as_()), 0.as_());
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

    /// Saturating integer multiplication. Computes `self * rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(U1024::ONE.saturating_mul(1.as_()), 1.as_());
    /// assert_eq!(U1024::MAX.saturating_mul(2.as_()), U1024::MAX);
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

    /// Saturating integer division. Since the calculation only involves non-negative integers, overflowing cannot occur, so this is equivalent to `self / rhs`.
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
    /// assert_eq!(5.as_::<U512>().saturating_div(2.as_()), 2.as_());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_div(self, rhs: Self) -> Self {
        let (div, overflow) = self.overflowing_div(rhs);
        if overflow { Self::MAX } else { div }
    }

    /// Saturating exponentiation. Computes `self.pow(exp)`, returning `Self::MAX` if the result is too large to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(2.as_::<U256>().saturating_pow(8), 256.as_());
    /// assert_eq!(2.as_::<U256>().saturating_pow(256), U256::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_pow(self, exp: ExpType) -> Self {
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
impl<const N: usize> Uint<N> {
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

    /// Saturating addition with a signed integer of the same bit width. Computes `self + rhs`, returning `Self::MAX` if the result is too large to be represented by `Self`, or `Self::ZERO` if the result is negative.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(U512::ONE.saturating_add_signed(-1.as_()), 0.as_());
    /// assert_eq!(U512::MAX.saturating_add_signed(1.as_()), U512::MAX);
    /// assert_eq!(U512::ONE.saturating_add_signed(-2.as_()), 0.as_());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_add_signed(self, rhs: Int<N>) -> Self {
        if rhs.is_negative() {
            Self::saturate_down(self.checked_add_signed(rhs))
        } else {
            Self::saturate_up(self.checked_add_signed(rhs))
        }
    }

    /// Saturating subtraction with a signed integer of the same bit width. Computes `self - rhs`, returning `Self::ZERO` if the result is negative, or `Self::MAX` if the result is too large to be represented by `Self`.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(U2048::ONE.saturating_sub_signed(-1.as_()), 2.as_());
    /// assert_eq!(U2048::MAX.saturating_sub_signed(-4.as_()), U2048::MAX);
    /// assert_eq!(U2048::ONE.saturating_sub_signed(3.as_()), 0.as_());
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_sub_signed(self, rhs: Int<N>) -> Self
    {
        if rhs.is_negative() {
            Self::saturate_up(self.checked_sub_signed(rhs))
        } else {
            Self::saturate_down(self.checked_sub_signed(rhs))
        }
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize> Int<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_add_unsigned(self, rhs: Uint<N>) -> Self {
        match self.checked_add_unsigned(rhs) {
            Some(i) => i,
            None => Self::MAX,
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_sub_unsigned(self, rhs: Uint<N>) -> Self {
        match self.checked_sub_unsigned(rhs) {
            Some(i) => i,
            None => Self::MIN,
        }
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn saturating_abs(self) -> Self {
        match self.checked_abs() {
            Some(abs) => abs,
            None => Self::MAX,
        }
    }

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
