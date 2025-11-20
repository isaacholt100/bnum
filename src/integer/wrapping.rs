use super::Uint;
use crate::Exponent;
use crate::{Integer, Int};
use crate::doc;

macro_rules! impl_desc {
    () => {
        "Wrapping arithmetic methods which act on `self`: `self.wrapping_...`. Each method returns of the calculation truncated to the number of bits of `self` (i.e. modulo `Self::MAX + 1`), except for the `wrapping_shl` and `wrapping_shr` methods, which return the value shifted by `rhs % Self::BITS`."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const OM: u8> Integer<S, N, OM> {
    /// Wrapping integer addition. Computes `self + rhs` and returns the result truncated to the bit width of `Self` (i.e. performs addition modulo $Self::MAX + 1$).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(n!(1 U1024).wrapping_add(n!(1)), n!(2));
    /// assert_eq!(U1024::MAX.wrapping_add(n!(1)), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
    
    /// Wrapping integer subtraction. Computes `self - rhs` and returns the result truncated to the bit width of `Self` (i.e. performs subtraction modulo $Self::MAX + 1$).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(n!(1 U256).wrapping_sub(n!(1)), n!(0));
    /// assert_eq!(n!(0 U256).wrapping_sub(n!(1)), U256::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    /// Wrapping integer multiplication. Computes `self * rhs` and returns the result truncated to the bit width of `Self` (i.e. performs multiplication modulo $Self::MAX + 1$).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(n!(1 U512).wrapping_mul(n!(1)), n!(1));
    /// assert_eq!(U512::power_of_two(511).wrapping_mul(n!(2)), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        // cast to unsigned as the calculation is faster (no need calculate absolute values)
        self.force_sign::<false>().overflowing_mul(rhs.force_sign()).0.force_sign()
    }

    /// Wrapping integer division. Since the calculation only involves unsigned integers, this is equivalent to normal division: `self / rhs`.
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
    /// assert_eq!(n!(5 U256).wrapping_div(n!(2)), n!(2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }

    /// Wrapping Euclidean division. Since the calculation only involves unsigned integers, this is equivalent to normal division: `self / rhs`.
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
    /// assert_eq!(n!(13 U2048).wrapping_div_euclid(n!(5)), n!(2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.overflowing_div_euclid(rhs).0
    }

    /// Wrapping integer remainder. Since the calculation only involves unsigned integers, this is equivalent to normal remainder operation `self % rhs`.
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }

    /// Wrapping Euclidean remainder. Since the calculation only involves unsigned integers, this is equivalent to normal remainder operation `self % rhs`.
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
    /// assert_eq!(n!(13 U512).wrapping_rem_euclid(n!(5)), n!(3));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.overflowing_rem_euclid(rhs).0
    }

    /// Wrapping negation (negation modulo $Self::MAX + 1$).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(n!(1 U256).wrapping_neg(), U256::MAX);
    /// assert_eq!(U256::MAX.wrapping_neg(), n!(1));
    /// assert_eq!(n!(0 U256).wrapping_neg(), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    /// Panic-free left shift. Returns `self << mask(rhs)`, where `mask(rhs)` is `rhs` modulo `Self::BITS`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(n!(1 U2048).wrapping_shl(1), n!(2));
    /// assert_eq!(n!(1 U2048).wrapping_shl(2049), n!(2));
    /// assert_eq!(n!(1 U2048).wrapping_shl(2048), n!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_shl(self, rhs: Exponent) -> Self {
        self.overflowing_shl(rhs).0
    }

    /// Panic-free right shift. Returns `self >> mask(rhs)`, where `mask(rhs)` is `rhs` modulo `Self::BITS`.
    /// 
    /// # Examples
    ///
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(n!(1 U1024).wrapping_shr(1), n!(0));
    /// assert_eq!(n!(2 U1024).wrapping_shr(1025), n!(1));
    /// assert_eq!(U1024::MAX.wrapping_shr(1024), U1024::MAX);
    /// assert_eq!(U1024::MAX.wrapping_shr(1023), n!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_shr(self, rhs: Exponent) -> Self {
        self.overflowing_shr(rhs).0
    }

    /// Wrapping exponentiation. Computes `self**exp` and returns the result truncated to the bit width of `Self` (i.e. performs exponentiation modulo $Self::MAX + 1$).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(n!(2 U512).wrapping_pow(10), n!(1024));
    /// assert_eq!(n!(2 U512).wrapping_pow(512), n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_pow(self, exp: Exponent) -> Self {
        // cast to unsigned as overflowing_pow for unsigned is faster (don't need to calculate abs)
        self.force_sign::<false>().overflowing_pow(exp).0.force_sign()
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const OM: u8> Uint<N, OM> {
    /// Wrapping integer addition with a signed integer of the same bit width. Computes `self + rhs` and returns the result truncated to the bit width of `Self` (i.e. performs addition modulo $Self::MAX + 1$).
    ///
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(n!(1 U512).wrapping_add_signed(n!(1)), n!(2));
    /// assert_eq!(U512::MAX.wrapping_add_signed(n!(1)), n!(0));
    /// assert_eq!(n!(1 U512).wrapping_add_signed(n!(-2)), U512::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add_signed(self, rhs: Int<N, OM>) -> Self {
        self.overflowing_add_signed(rhs).0
    }

    /// Wrapping integer subtraction with a signed integer of the same bit width. Computes `self - rhs` and returns the result truncated to the bit width of `Self` (i.e. performs subtraction modulo $Self::MAX + 1$).
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(n!(1 U2048).wrapping_sub_signed(n!(-1)), n!(2));
    /// assert_eq!(U2048::MAX.wrapping_sub_signed(n!(-1)), n!(0));
    /// assert_eq!(n!(1 U2048).wrapping_sub_signed(n!(2)), U2048::MAX);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_sub_signed(self, rhs: Int<N, OM>) -> Self {
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
    /// assert_eq!(n!(4 U256).wrapping_next_power_of_two(), n!(4));
    /// assert_eq!(n!(31 U256).wrapping_next_power_of_two(), n!(32));
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
impl<const N: usize, const OM: u8> Int<N, OM> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_add_unsigned(self, rhs: Uint<N, OM>) -> Self {
        self.overflowing_add_unsigned(rhs).0
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn wrapping_sub_unsigned(self, rhs: Uint<N, OM>) -> Self {
        self.overflowing_sub_unsigned(rhs).0
    }

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
            function: <utest>::wrapping_add_signed(a: utest, b: itest)
        }
        test_bignum! {
            function: <utest>::wrapping_sub_signed(a: utest, b: itest)
        }
        #[cfg(feature = "nightly")] // since wrapping_next_power_of_two is not stabilised yet
        test_bignum! {
            function: <utest>::wrapping_next_power_of_two(a: utest),
            cases: [
                (utest::MAX)
            ]
        }
    }
    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <itest>::wrapping_add_unsigned(a: itest, b: utest)
        }
        test_bignum! {
            function: <itest>::wrapping_sub_unsigned(a: itest, b: utest)
        }
        test_bignum! {
            function: <itest>::wrapping_abs(a: itest)
        }
    }
    crate::test::test_all! {
        testing integers;

        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn div_by_zero_panic() {
            let a = STEST::MAX;
            let b = STEST::ZERO;
            let _ = a.wrapping_div(b);
        }

        #[test]
        #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
        fn rem_by_zero_panic() {
            let a = STEST::MAX;
            let b = STEST::ZERO;
            let _ = a.wrapping_rem(b);
        }

        test_bignum! {
            function: <stest>::wrapping_add(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::wrapping_sub(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::wrapping_mul(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::wrapping_div(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::wrapping_div_euclid(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::wrapping_rem(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::wrapping_rem_euclid(a: stest, b: stest),
            skip: b == 0
        }
        test_bignum! {
            function: <stest>::wrapping_neg(a: stest)
        }
        test_bignum! {
            function: <stest>::wrapping_shl(a: stest, b: u16)
        }
        test_bignum! {
            function: <stest>::wrapping_shr(a: stest, b: u16)
        }
        test_bignum! {
            function: <stest>::wrapping_pow(a: stest, b: u16)
        }
    }
}
