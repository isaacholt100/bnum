use super::Uint;
use crate::ExpType;
use crate::doc;

#[doc = doc::saturating::impl_desc!()]
impl<const N: usize> Uint<N> {
    #[inline]
    const fn saturate_up((int, overflow): (Uint<N>, bool)) -> Uint<N> {
        if overflow { Uint::MAX } else { int }
    }

    #[inline]
    const fn saturate_down((int, overflow): (Uint<N>, bool)) -> Uint<N> {
        if overflow { Uint::MIN } else { int }
    }

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
        Self::saturate_up(self.overflowing_add(rhs))
    }

    #[cfg(feature = "signed")]
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
    pub const fn saturating_add_signed(self, rhs: crate::Int<N>) -> Self {
        if rhs.is_negative() {
            Self::saturate_down(self.overflowing_add_signed(rhs))
        } else {
            Self::saturate_up(self.overflowing_add_signed(rhs))
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
        Self::saturate_down(self.overflowing_sub(rhs))
    }

    #[cfg(feature = "signed")]
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
    pub const fn saturating_sub_signed(self, rhs: crate::Int<N>) -> Self
    {
        if rhs.is_negative() {
            Self::saturate_up(self.overflowing_sub_signed(rhs))
        } else {
            Self::saturate_down(self.overflowing_sub_signed(rhs))
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
        Self::saturate_up(self.overflowing_mul(rhs))
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
        self.div_euclid(rhs)
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
        Self::saturate_up(self.overflowing_pow(exp))
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::saturating_add(a: utest, b: utest)
    }
    #[cfg(feature = "signed")]
    test_bignum! {
        function: <utest>::saturating_add_signed(a: utest, b: itest)
    }
    test_bignum! {
        function: <utest>::saturating_sub(a: utest, b: utest)
    }
    #[cfg(all(feature = "signed", feature = "nightly"))] // since mixed_integer_ops_unsigned_sub is not stabilised yet
    test_bignum! {
        function: <utest>::saturating_sub_signed(a: utest, b: itest)
    }
    test_bignum! {
        function: <utest>::saturating_mul(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::saturating_div(a: utest, b: utest),
        skip: b == 0
    }
    test_bignum! {
        function: <utest>::saturating_pow(a: utest, b: u16)
    }
}
