use super::Uint;
use crate::doc;
use crate::ExpType;
use crate::digit::Digit;
use crate::errors;

impl<const N: usize> Uint<N> {
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

    // #[must_use = doc::must_use_op!()]
    // #[inline]
    // pub const fn modpow(mut self, mut exp: ExpType, modulus: Self) -> Self {
    //     // exponentiation by squaring
    //     if modulus.is_zero() {
    //         panic!(errors::err_msg!(errors::rem_by_zero_message!()));
    //     }
    //     if exp == 0 {
    //         return Self::ONE;
    //     }
    //     let mut y = Self::ONE;
    //     let overflow_rem = Self::MAX.wrapping_rem(modulus).wrapping_add(Self::ONE); // overflow isn't possible here as the remainder must be < Self::MAX.
    //     while exp > 1 {
    //         if exp % 2 == 1 {
    //             let (low, high) = self.widening_mul(y); // self < 2^(Self::BITS), so high must be < modulus, given that y < modulus.
    //             y
    //             y = self.wrapping_mul(y);
    //         }
    //         self = self.wrapping_mul(self);
    //         exp /= 2;
    //     }
    //     self.wrapping_mul(y)
    // }

    #[doc = doc::div_euclid!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div_euclid(rhs)
    }

    #[doc = doc::rem_euclid!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem_euclid(rhs)
    }

    #[doc = doc::doc_comment! {
        U 256,
        "Returns `true` if and only if `self == 2^k` for some integer `k`.",

        "let n = " stringify!(U256) "::from(1u16 << 14);\n"
        "assert!(n.is_power_of_two());\n"
        "let m = " stringify!(U256) "::from(100u8);\n"
        "assert!(!m.is_power_of_two());"
    }]
    #[must_use]
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
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

    #[doc = doc::next_power_of_two!(U 256, "0", "ZERO")]
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

    #[doc = doc::midpoint!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn midpoint(self, rhs: Self) -> Self {
        // see section 2.5: Average of Two Integers in Hacker's Delight
        self.bitand(rhs).add(self.bitxor(rhs).shr(1))
    }

    #[doc = doc::ilog2!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog2(self) -> ExpType {
        self.checked_ilog2()
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    #[doc = doc::ilog10!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog10(self) -> ExpType {
        self.checked_ilog10()
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    #[doc = doc::ilog!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn ilog(self, base: Self) -> ExpType {
        if base.le(&Self::ONE) {
            panic!("{}", errors::err_msg!(errors::invalid_log_base_message!()));
        }
        self.checked_ilog(base)
            .expect(errors::err_msg!(errors::non_positive_log_message!()))
    }

    #[doc = doc::abs_diff!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn abs_diff(self, other: Self) -> Self {
        if self.lt(&other) {
            other.wrapping_sub(self)
        } else {
            self.wrapping_sub(other)
        }
    }

    #[doc = doc::next_multiple_of!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
        let rem = self.wrapping_rem(rhs);
        if rem.is_zero() {
            self
        } else {
            self.add(rhs.sub(rem))
        }
    }

    #[doc = doc::div_floor!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div_floor(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    #[doc = doc::div_ceil!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn div_ceil(self, rhs: Self) -> Self {
        let (div, rem) = self.div_rem(rhs);
        if rem.is_zero() {
            div
        } else {
            div.add(Self::ONE)
        }
    }

    #[doc = doc::is_zero!(U 256)]
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

    #[doc = doc::is_one!(U 256)]
    #[must_use]
    #[inline]
    pub const fn is_one(&self) -> bool {
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
    
    #[cfg(feature = "signed")]
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
        crate::Int::from_bits(self)
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
        out.digits[power as usize / Digit::BITS as usize] = 1 << (power % Digit::BITS);
        out
    }
}