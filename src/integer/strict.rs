use crate::{Int, Integer, Uint};

macro_rules! impl_desc {
    () => {
        "Strict arithmetic methods which act on `self`: `self.strict_...`. Each method will always panic if overflow or division by zero occurs (i.e. when the checked equivalent would return `None`), regardless of [`Self::OVERFLOW_MODE`]."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Strict integer addition. Computes `self + rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(2 U24).strict_add(n!(3 U24)), n!(5 U24));
    /// assert_eq!(n!(-2 I24).strict_add(n!(-3 I24)), n!(-5 I24));
    /// ```
    /// The following examples will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// let _ = U256::MAX.strict_add(n!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect(crate::errors::err_msg!("attempt to add with overflow"))
    }

    /// Strict integer addition. Computes `self + rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(5 U24).strict_sub(n!(2 U24)), n!(3 U24));
    /// assert_eq!(n!(-5 I24).strict_sub(n!(-2 I24)), n!(-3 I24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::I512;
    /// 
    /// let _ = I512::MIN.strict_sub(n!(1));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect(crate::errors::err_msg!("attempt to subtract with overflow"))
    }

    /// Strict integer addition. Computes `self + rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(2 U24).strict_mul(n!(3 U24)), n!(6 U24));
    /// assert_eq!(n!(-2 I24).strict_mul(n!(3 I24)), n!(-6 I24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// let _ = U1024::MAX.strict_mul(n!(2));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs)
            .expect(crate::errors::err_msg!("attempt to multiply with overflow"))
    }

    /// Strict integer addition. Computes `self + rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is `0` or if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// Overflow can only occur if the integers are signed, and `self` is [`Self::MIN`] and `rhs` is `-1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(9 U24).strict_div(n!(4)), n!(2));
    /// assert_eq!(n!(-9 I24).strict_div(n!(4)), n!(-2));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::I2048;
    /// 
    /// let _ = I2048::MIN.strict_div(n!(-1));
    /// ```
    /// The following example will panic due to division by zero:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// 
    /// let _ = n!(10 U2048).strict_div(n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_div(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to divide with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::div_by_zero_message!()));
        }
        self.div_rem_unchecked(rhs).0
    }

    /// Strict integer addition. Computes `self + rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is `0` or if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// Overflow can only occur if the integers are signed, and `self` is [`Self::MIN`] and `rhs` is `-1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(9 U24).strict_div_euclid(n!(4)), n!(2));
    /// assert_eq!(n!(-9 I24).strict_div_euclid(n!(4)), n!(-3));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::I256;
    /// 
    /// let _ = I256::MIN.strict_div_euclid(n!(-1));
    /// ```
    /// The following example will panic due to division by zero:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// 
    /// let _ = n!(10 U256).strict_div_euclid(n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_div_euclid(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to divide with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::div_by_zero_message!()));
        }
        self.div_rem_euclid_unchecked(rhs).0
    }

    /// Strict integer remainder. Computes `self % rhs`, panicking if the division results in overflow.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is `0` or if overflow occurs in the division, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// Overflow can only occur if the integers are signed, and `self` is [`Self::MIN`] and `rhs` is `-1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(9 U24).strict_rem(n!(4)), n!(1));
    /// assert_eq!(n!(-9 I24).strict_rem(n!(4)), n!(-1));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::I512;
    /// 
    /// let _ = I512::MIN.strict_rem(n!(-1));
    /// ```
    /// The following example will panic due to division by zero:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// 
    /// let _ = n!(10 U512).strict_rem(n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_rem(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to calculate the remainder with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::rem_by_zero_message!()));
        }
        self.div_rem_unchecked(rhs).1
    }

    /// Strict integer remainder. Computes `self % rhs`, panicking if the division results in overflow.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is `0` or if overflow occurs in the division, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// Overflow can only occur if the integers are signed, and `self` is [`Self::MIN`] and `rhs` is `-1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(9 U24).strict_rem_euclid(n!(4)), n!(1));
    /// assert_eq!(n!(-9 I24).strict_rem_euclid(n!(4)), n!(3));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::I1024;
    /// 
    /// let _ = I1024::MIN.strict_rem_euclid(n!(-1));
    /// ```
    /// The following example will panic due to division by zero:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// 
    /// let _ = n!(10 U1024).strict_rem_euclid(n!(0));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
        if self.is_division_overflow(&rhs) {
            panic!(crate::errors::err_msg!("attempt to calculate the remainder with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(crate::errors::rem_by_zero_message!()));
        }
        self.div_rem_euclid_unchecked(rhs).1
    }

    /// Strict negation. Computes `-self`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// For unsigned integers, overflow will occur unless `self` is `0`. For signed integers, overflow will only occur if `self` is [`Self::MIN`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(0 U24).strict_neg(), n!(0));
    /// assert_eq!(n!(5 I24).strict_neg(), n!(-5));
    /// ```
    /// The following examples will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// 
    /// let _ = n!(1 U256).strict_neg();
    /// ```
    /// 
    /// ```should_panic
    /// use bnum::types::I256;
    /// 
    /// let _ = I256::MIN.strict_neg();
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_neg(self) -> Self {
        self.checked_neg()
            .expect(crate::errors::err_msg!("attempt to negate with overflow"))
    }

    /// Strict left shift. Computes `self << rhs`, panicking if `rhs` is greater than or equal to [`Self::BITS`].
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is greater than or equal to [`Self::BITS`], regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(3 U24).strict_shl(2), n!(12 U24));
    /// assert_eq!(n!(-3 I24).strict_shl(2), n!(-12 I24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// 
    /// let _ = n!(1 U512).strict_shl(512);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_shl(self, rhs: crate::Exponent) -> Self {
        self.checked_shl(rhs).expect(crate::errors::err_msg!(
            "attempt to shift left with overflow"
        ))
    }

    /// Strict right shift. Computes `self >> rhs`, panicking if `rhs` is greater than or equal to [`Self::BITS`].
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is greater than or equal to [`Self::BITS`], regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(17 U24).strict_shr(2), n!(4 U24));
    /// assert_eq!(n!(-23 I24).strict_shr(2), n!(-6 I24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::types::I1024;
    /// 
    /// let _ = I1024::MAX.strict_shr(1024);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_shr(self, rhs: crate::Exponent) -> Self {
        self.checked_shr(rhs).expect(crate::errors::err_msg!(
            "attempt to shift right with overflow"
        ))
    }

    /// Strict exponentiation. Computes `self.pow(exp)`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(2 U24).strict_pow(3), n!(8));
    /// assert_eq!(n!(-3 I24).strict_pow(5), n!(-243));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// 
    /// let _ = n!(2 U2048).strict_pow(2048);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_pow(self, exp: crate::Exponent) -> Self {
        self.checked_pow(exp).expect(crate::errors::err_msg!(
            "attempt to calculate power with overflow"
        ))
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    /// Strict addition with a signed integer of the same bit width. Computes `self + rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(5 U24).strict_add_signed(n!(-2 I24)), n!(3 U24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// let _ = U256::MIN.strict_add_signed(n!(-1 I256));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_signed(self, rhs: Int<N, B, OM>) -> Self {
        self.checked_add_signed(rhs)
            .expect(crate::errors::err_msg!("attempt to add with overflow"))
    }

    /// Strict subtraction by a signed integer of the same bit width. Computes `self - rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(5 U24).strict_sub_signed(n!(-2 I24)), n!(7 U24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// let _ = U256::MAX.strict_sub_signed(n!(-1 I256));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_sub_signed(self, rhs: Int<N, B, OM>) -> Self {
        self.checked_sub_signed(rhs)
            .expect(crate::errors::err_msg!("attempt to subtract with overflow"))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub(crate) const fn strict_next_power_of_two(self) -> Self {
        self.checked_next_power_of_two()
            .expect(crate::errors::err_msg!("attempt to calculate next power of two with overflow"))
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    /// Strict absolute value. Computes `self.abs()`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs (i.e. if `self` is [`Self::MIN`]), regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(-5 I24).strict_abs(), n!(5 I24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::types::I512;
    /// 
    /// let _ = I512::MIN.strict_abs();
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_abs(self) -> Self {
        self.checked_abs()
            .expect(crate::errors::err_msg!("attempt to negate with overflow"))
    }

    /// Strict addition with an unsigned integer of the same bit width. Computes `self + rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(5 I24).strict_add_unsigned(n!(2 U24)), n!(7 I24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::I1024;
    /// 
    /// let _ = I1024::MAX.strict_add_unsigned(n!(1 U1024));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_unsigned(self, rhs: Uint<N, B, OM>) -> Self {
        self.checked_add_unsigned(rhs)
            .expect(crate::errors::err_msg!("attempt to add with overflow"))
    }

    /// Strict subtraction by an unsigned integer of the same bit width. Computes `self - rhs`, panicking if overflow occurs.
    /// 
    /// # Panics
    /// 
    /// This function will panic if overflow occurs, regardless of [`Self::OVERFLOW_MODE`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(5 I24).strict_sub_unsigned(n!(7 U24)), n!(-2 I24));
    /// ```
    /// The following example will panic due to overflow:
    /// ```should_panic
    /// use bnum::prelude::*;
    /// use bnum::types::I2048;
    /// 
    /// let _ = I2048::MIN.strict_sub_unsigned(n!(1 U2048));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_sub_unsigned(self, rhs: Uint<N, B, OM>) -> Self {
        self.checked_sub_unsigned(rhs)
            .expect(crate::errors::err_msg!("attempt to subtract with overflow"))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing unsigned;

        test_bignum! {
            function: <utest>::strict_add_signed(a: utest, b: itest),
            skip: a.checked_add_signed(b).is_none()
        }
        test_bignum! {
            function: <utest>::strict_sub_signed(a: utest, b: itest),
            skip: a.checked_sub_signed(b).is_none()
        }
    }
    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <itest>::strict_abs(a: itest),
            skip: a.checked_abs().is_none()
        }
        test_bignum! {
            function: <itest>::strict_add_unsigned(a: itest, b: utest),
            skip: a.checked_add_unsigned(b).is_none()
        }
        test_bignum! {
            function: <itest>::strict_sub_unsigned(a: itest, b: utest),
            skip: a.checked_sub_unsigned(b).is_none()
        }
    }
    crate::test::test_all! {
        testing integers;
        
        test_bignum! {
            function: <stest>::strict_add(a: stest, b: stest),
            skip: a.checked_add(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_sub(a: stest, b: stest),
            skip: a.checked_sub(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_mul(a: stest, b: stest),
            skip: a.checked_mul(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_div(a: stest, b: stest),
            skip: a.checked_div(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_div_euclid(a: stest, b: stest),
            skip: a.checked_div_euclid(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_rem(a: stest, b: stest),
            skip: a.checked_rem(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_rem_euclid(a: stest, b: stest),
            skip: a.checked_rem_euclid(b).is_none()
        }
        test_bignum! {
            function: <stest>::strict_neg(a: stest),
            skip: a.checked_neg().is_none()
        }
        test_bignum! {
            function: <stest>::strict_shl(a: stest, b: u8),
            skip: a.checked_shl(b as u32).is_none()
        }
        test_bignum! {
            function: <stest>::strict_shr(a: stest, b: u8),
            skip: a.checked_shr(b as u32).is_none()
        }
        test_bignum! {
            function: <stest>::strict_pow(a: stest, b: u8),
            skip: a.checked_pow(b as u32).is_none()
        }
    }
}

use crate::doc;
