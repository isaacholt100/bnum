use super::Uint;
use crate::Exponent;
use crate::{Integer, Int};
use crate::doc;

macro_rules! impl_desc {
    () => {
        "Wrap arithmetic methods which act on `self`: `self.wrapping_...`. Each method returns of the calculation truncated to the number of bits of `self` (i.e. modulo `Self::MAX + 1`), except for the `wrapping_shl` and `wrapping_shr` methods, which return the value shifted by `rhs % Self::BITS`."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Wrap integer addition. Computes `self + rhs` modulo `Self::MAX + 1`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    /// 
    /// assert_eq!(n!(1U1024).wrapping_add(n!(1)), n!(2));
    /// assert_eq!(U1024::MAX.wrapping_add(n!(1)), n!(0));
    /// 
    /// assert_eq!(I1024::MIN.wrapping_add(n!(-1)), I1024::MAX);
    /// assert_eq!(I1024::MAX.wrapping_add(n!(1)), I1024::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
    
    /// Wrap integer subtraction. Computes `self - rhs` modulo `Self::MAX + 1`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(n!(1U256).wrapping_sub(n!(1)), n!(0));
    /// assert_eq!(n!(0U256).wrapping_sub(n!(1)), U256::MAX);
    /// 
    /// assert_eq!(I256::MIN.wrapping_sub(n!(1)), I256::MAX);
    /// assert_eq!(I256::MAX.wrapping_sub(n!(-1)), I256::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    /// Wrap integer multiplication. Computes `self * rhs` modulo `Self::MAX + 1`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    /// 
    /// assert_eq!(n!(1U512).wrapping_mul(n!(1)), n!(1));
    /// assert_eq!(U512::power_of_two(511).wrapping_mul(n!(2)), n!(0));
    /// 
    /// assert_eq!(I512::MIN.wrapping_mul(n!(-1)), I512::MIN);
    /// assert_eq!(I512::MIN.wrapping_mul(I512::MIN), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        // cast to unsigned as the calculation is faster (no need calculate absolute values)
        self.force_sign::<false>().overflowing_mul(rhs.force_sign()).0.force_sign()
    }

    /// Wrap integer division. Note that wrap around can only occur for signed integers, when `self` is [`Self::MIN`] and `rhs` is `-1`.
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
    /// assert_eq!(n!(5U1024).wrapping_div(n!(2)), n!(2));
    /// 
    /// assert_eq!(n!(-47I1024).wrapping_div(n!(-5)), n!(9));
    /// assert_eq!(I1024::MIN.wrapping_div(n!(-1)), I1024::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }

    /// Wrap Euclidean division. Note that wrap around can only occur for signed integers, when `self` is [`Self::MIN`] and `rhs` is `-1`. In this case, the function returns [`Self::MIN`].
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
    /// assert_eq!(n!(13U2048).wrapping_div_euclid(n!(5)), n!(2));
    /// 
    /// assert_eq!(n!(-13I2048).wrapping_div_euclid(n!(5)), n!(-3));
    /// assert_eq!(I2048::MIN.wrapping_div_euclid(n!(-1)), I2048::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.overflowing_div_euclid(rhs).0
    }

    /// Wrap integer remainder. This is equivalent to `self.strict_rem(rhs)`.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is zero.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// use bnum::types::I256;
    /// 
    /// assert_eq!(n!(13U256).wrapping_rem(n!(5)), n!(3));
    /// assert_eq!(I256::MIN.wrapping_rem(n!(-1)), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }

    /// Wrap Euclidean remainder. This is equivalent to `self.strict_rem_euclid(rhs)`.
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
    /// assert_eq!(n!(13U512).wrapping_rem_euclid(n!(5)), n!(3));
    /// assert_eq!(I512::MIN.wrapping_rem_euclid(n!(-1)), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.overflowing_rem_euclid(rhs).0
    }

    /// Wrap (modular) negation. Computes `-self` modulo `Self::MAX + 1`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    /// 
    /// assert_eq!(n!(1U1024).wrapping_neg(), U1024::MAX);
    /// assert_eq!(U1024::MAX.wrapping_neg(), n!(1));
    /// assert_eq!(n!(0U1024).wrapping_neg(), n!(0));
    /// 
    /// assert_eq!(n!(1I1024).wrapping_neg(), n!(-1));
    /// assert_eq!(I1024::MIN.wrapping_neg(), I1024::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    /// Panic-free left shift. Returns `self << (rhs % Self::BITS)`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    /// 
    /// assert_eq!(n!(1U2048).wrapping_shl(1), n!(2));
    /// assert_eq!(n!(1U2048).wrapping_shl(2049), n!(2));
    /// assert_eq!(n!(1U2048).wrapping_shl(2048), n!(1));
    /// 
    /// assert_eq!(n!(-2I2048).wrapping_shl(1), n!(-4));
    /// assert_eq!(n!(-2I2048).wrapping_shl(2049), n!(-4));
    /// assert_eq!(n!(-2I2048).wrapping_shl(2048), n!(-2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_shl(self, rhs: Exponent) -> Self {
        self.overflowing_shl(rhs).0
    }

    /// Panic-free right shift. Returns `self >> (rhs % Self::BITS)`.
    /// 
    /// # Examples
    ///
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    /// 
    /// assert_eq!(n!(1U1024).wrapping_shr(1), n!(0));
    /// assert_eq!(n!(2U1024).wrapping_shr(1025), n!(1));
    /// assert_eq!(U1024::MAX.wrapping_shr(1024), U1024::MAX);
    /// assert_eq!(U1024::MAX.wrapping_shr(1023), n!(1));
    /// 
    /// assert_eq!(n!(-4I1024).wrapping_shr(1), n!(-2));
    /// assert_eq!(I1024::MIN.wrapping_shr(2048), I1024::MIN);
    /// assert_eq!(I1024::MIN.wrapping_shr(2047), n!(-1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_shr(self, rhs: Exponent) -> Self {
        self.overflowing_shr(rhs).0
    }

    /// Wrap exponentiation. Computes `self.pow(exp)` modulo `Self::MAX + 1`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I512;
    /// 
    /// assert_eq!(n!(2U512).wrapping_pow(10), n!(1024));
    /// assert_eq!(n!(2U512).wrapping_pow(512), n!(0));
    /// 
    /// assert_eq!(n!(-2I512).wrapping_pow(512), n!(0));
    /// assert_eq!(n!(2I512).wrapping_pow(511), I512::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_pow(self, exp: Exponent) -> Self {
        // cast to unsigned as overflowing_pow for unsigned is faster (don't need to calculate abs)
        self.force_sign::<false>().overflowing_pow(exp).0.force_sign()
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    /// Wrap integer addition with a signed integer of the same bit width. Computes `self + rhs` modulo `Self::MAX + 1`.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(n!(1U512).wrapping_add_signed(n!(1)), n!(2));
    /// assert_eq!(U512::MAX.wrapping_add_signed(n!(1)), n!(0));
    /// assert_eq!(n!(1U512).wrapping_add_signed(n!(-2)), U512::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add_signed(self, rhs: Int<N, B, OM>) -> Self {
        self.overflowing_add_signed(rhs).0
    }

    /// Wrap integer subtraction with a signed integer of the same bit width. Computes `self - rhs` modulo `Self::MAX + 1`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(n!(1U2048).wrapping_sub_signed(n!(-1)), n!(2));
    /// assert_eq!(U2048::MAX.wrapping_sub_signed(n!(-1)), n!(0));
    /// assert_eq!(n!(1U2048).wrapping_sub_signed(n!(2)), U2048::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_sub_signed(self, rhs: Int<N, B, OM>) -> Self {
        self.overflowing_sub_signed(rhs).0
    }

    /// Returns the smallest power of two greater than or equal to `self`. If the next power of two is greater than `Self::MAX`, the return value is wrapped to zero.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(n!(4U256).wrapping_next_power_of_two(), n!(4));
    /// assert_eq!(n!(31U256).wrapping_next_power_of_two(), n!(32));
    /// assert_eq!(U256::MAX.wrapping_next_power_of_two(), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::ZERO,
        }
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    /// Wrap integer addition with an unsigned integer of the same bit width. Computes `self + rhs` modulo `Self::MAX + 1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    /// 
    /// assert_eq!(I512::MIN.wrapping_add_unsigned(U512::MAX), I512::MAX);
    /// assert_eq!(n!(0I512).wrapping_add_unsigned(U512::MAX), n!(-1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add_unsigned(self, rhs: Uint<N, B, OM>) -> Self {
        self.overflowing_add_unsigned(rhs).0
    }

    /// Wrap integer subtraction with an unsigned integer of the same bit width. Computes `self - rhs` modulo `Self::MAX + 1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    /// 
    /// assert_eq!(I1024::MAX.wrapping_sub_unsigned(U1024::MAX), I1024::MIN);
    /// assert_eq!(n!(0I1024).wrapping_sub_unsigned(U1024::MAX), n!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_sub_unsigned(self, rhs: Uint<N, B, OM>) -> Self {
        self.overflowing_sub_unsigned(rhs).0
    }

    /// Wrap absolute value. Computes `self.abs()` modulo `Self::MAX + 1`. Note that overflow can only occur when `self` is [`Self::MIN`], in which case the function returns [`Self::MIN`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I2048;
    /// 
    /// assert_eq!(n!(-123I2048).wrapping_abs(), n!(123));
    /// assert_eq!(I2048::MIN.wrapping_abs(), I2048::MIN);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing unsigned;

        test_bignum! {
            function: <UTest>::wrapping_add_signed(a: UTest, b: ITest)
        }
        test_bignum! {
            function: <UTest>::wrapping_sub_signed(a: UTest, b: ITest)
        }
        #[cfg(nightly)] // since wrapping_next_power_of_two is not stabilised yet
        test_bignum! {
            function: <UTest>::wrapping_next_power_of_two(a: UTest),
            cases: [
                (UTest::MAX)
            ]
        }
    }
    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <ITest>::wrapping_add_unsigned(a: ITest, b: UTest)
        }
        test_bignum! {
            function: <ITest>::wrapping_sub_unsigned(a: ITest, b: UTest)
        }
        test_bignum! {
            function: <ITest>::wrapping_abs(a: ITest)
        }
    }
    crate::test::test_all! {
        testing integers;

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn div_by_zero_panic() {
            let a = STest::MAX;
            let b = STest::ZERO;
            let _ = a.wrapping_div(b);
        }

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
        fn rem_by_zero_panic() {
            let a = STest::MAX;
            let b = STest::ZERO;
            let _ = a.wrapping_rem(b);
        }

        test_bignum! {
            function: <STest>::wrapping_add(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest>::wrapping_sub(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest>::wrapping_mul(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest>::wrapping_div(a: STest, b: STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest>::wrapping_div_euclid(a: STest, b: STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest>::wrapping_rem(a: STest, b: STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest>::wrapping_rem_euclid(a: STest, b: STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest>::wrapping_neg(a: STest)
        }
        test_bignum! {
            function: <STest>::wrapping_shl(a: STest, b: u16)
        }
        test_bignum! {
            function: <STest>::wrapping_shr(a: STest, b: u16)
        }
        test_bignum! {
            function: <STest>::wrapping_pow(a: STest, b: u16)
        }
    }
}
