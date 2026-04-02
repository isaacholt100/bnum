use super::Uint;
use crate::Exponent;
use crate::{Integer, Int};
use crate::doc;
use crate::helpers::tuple_to_option;

macro_rules! impl_desc {
    () => {
        "Checked arithmetic methods which act on `self`: `self.checked_...`. Each method cannot panic and returns an `Option<Self>`. `None` is returned when overflow would have occurred or there was an attempt to divide by zero or calculate a remainder with a divisor of zero."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    #[inline(always)]
    const fn tuple_to_option(t: (Self, bool)) -> Option<Self> {
        if t.1 {
            None
        } else {
            Some(t.0)
        }
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(1U1024).checked_add(n!(1)), Some(n!(2)));
    /// assert_eq!(U1024::MAX.checked_add(n!(1)), None);
    /// 
    /// assert_eq!(n!(1I1024).checked_add(n!(-1)), Some(n!(0)));
    /// assert_eq!(I1024::MIN.checked_add(n!(-1)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        Self::tuple_to_option(self.overflowing_add(rhs))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(n!(1U256).checked_sub(n!(1)), Some(n!(0)));
    /// assert_eq!(U256::MIN.checked_sub(n!(1)), None);
    /// 
    /// assert_eq!(n!(-1I256).checked_sub(n!(1)), Some(n!(-2)));
    /// assert_eq!(I256::MAX.checked_sub(n!(-1)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        Self::tuple_to_option(self.overflowing_sub(rhs))
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    /// 
    /// assert_eq!(n!(1U512).checked_mul(n!(1)), Some(n!(1)));
    /// assert_eq!(U512::MAX.checked_mul(n!(3)), None);
    /// 
    /// assert_eq!(n!(2I512).checked_mul(n!(3)), Some(n!(6)));
    /// assert_eq!(I512::MIN.checked_mul(n!(-1)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        Self::tuple_to_option(self.overflowing_mul(rhs))
    }

    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs` is zero, or if the division would overflow (this is only possible for signed integers).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(n!(5U256).checked_div(n!(2)), Some(n!(2)));
    /// assert_eq!(n!(1U256).checked_div(n!(0)), None);
    /// 
    /// assert_eq!(n!(-13I256).checked_div(n!(5)), Some(n!(-2)));
    /// assert_eq!(I256::MIN.checked_div(n!(-1)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).0)
        }
    }

    /// Checked Euclidean division. Computes `self.div_euclid(rhs)`, returning `None` if `rhs` is zero or if the division would overflow (this is only possible for signed integers).
    /// 
    /// For unsigned integers, this is equivalent to `self.checked_div(rhs)`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    /// 
    /// assert_eq!(n!(13U2048).checked_div_euclid(n!(5)), Some(n!(2)));
    /// assert_eq!(U2048::MAX.checked_div_euclid(n!(0)), None);
    /// 
    /// assert_eq!(n!(-13I2048).checked_div_euclid(n!(5)), Some(n!(-3)));
    /// assert_eq!(I2048::MIN.checked_div_euclid(n!(-1)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
            None
        } else {
            Some(self.div_rem_euclid_unchecked(rhs).0)
        }
    }

    /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs` is zero or if computing the remainder would result in overflow (this is only possible for signed integers).
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    /// 
    /// assert_eq!(n!(5U1024).checked_rem(n!(2)), Some(n!(1)));
    /// assert_eq!(n!(1U1024).checked_rem(n!(0)), None);
    /// 
    /// assert_eq!(n!(-13I1024).checked_rem(n!(5)), Some(n!(-3)));
    /// assert_eq!(I1024::MIN.checked_rem(n!(-1)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).1)
        }
    }

    /// Checked Euclidean remainder. Computes `self.rem_euclid(rhs)`, returning `None` if `rhs` is zero or if computing the remainder would result in overflow (this is only possible for signed integers).
    ///
    /// For unsigned integers, this is equivalent to `self.checked_rem(rhs)`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    /// 
    /// assert_eq!(n!(13U512).checked_rem_euclid(n!(5)), Some(n!(3)));
    /// assert_eq!(U512::MAX.checked_rem_euclid(n!(0)), None);
    /// 
    /// assert_eq!(n!(-13I512).checked_rem_euclid(n!(5)), Some(n!(2)));
    /// assert_eq!(I512::MIN.checked_rem_euclid(n!(-1)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
            None
        } else {
            Some(self.div_rem_euclid_unchecked(rhs).1)
        }
    }

    /// Checked negation. Computes `-self`, returning `None` if overflow occurred.
    /// 
    /// For unsigned integers, overflow occurs if `self` is non-zero. For signed integers, overflow occurs if `self` is equal to [`Self::MIN`].
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(n!(1U256).checked_neg(), None);
    /// assert_eq!(n!(0U256).checked_neg(), Some(n!(0)));
    /// 
    /// assert_eq!(n!(1I256).checked_neg(), Some(n!(-1)));
    /// assert_eq!(I256::MIN.checked_neg(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        if !S {
            if self.is_zero() { Some(self) } else { None } // this is faster than calling overflowing_neg
        } else {
            Self::tuple_to_option(self.overflowing_neg())
        }
    }

    /// Checked left shift. Computes `self << rhs`, returning `None` if `rhs` is greater than or equal to `Self::BITS`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    /// 
    /// assert_eq!(n!(1U2048).checked_shl(1), Some(n!(2)));
    /// assert_eq!(n!(1U2048).checked_shl(2048), None);
    /// assert_eq!(n!(2U2048).checked_shl(2047), Some(n!(0)));
    /// 
    /// assert_eq!(n!(-1I2048).checked_shl(2), Some(n!(-4)));
    /// assert_eq!(n!(-1I2048).checked_shl(2048), None);
    /// assert_eq!(n!(-1).checked_shl(2047), Some(I2048::MIN));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_shl(self, rhs: Exponent) -> Option<Self> {
        Self::tuple_to_option(self.overflowing_shl(rhs))
    }

    /// Checked right shift. Computes `self >> rhs`, returning `None` if `rhs` is greater than or equal to `Self::BITS`.
    /// 
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};
    ///
    /// assert_eq!(n!(1U1024).checked_shr(1), Some(n!(0)));
    /// assert_eq!(U1024::MAX.checked_shr(1024), None);
    /// assert_eq!(U1024::MAX.checked_shr(1023), Some(n!(1)));
    /// 
    /// assert_eq!(n!(-1I1024).checked_shr(1), Some(n!(-1)));
    /// assert_eq!(I1024::MIN.checked_shr(1024), None);
    /// assert_eq!(I1024::MIN.checked_shr(I1024::BITS - 1), Some(n!(-1)));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_shr(self, rhs: Exponent) -> Option<Self> {
        Self::tuple_to_option(self.overflowing_shr(rhs))
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    /// 
    /// assert_eq!(n!(2U512).checked_pow(10), Some(n!(1024)));
    /// assert_eq!(n!(2U512).checked_pow(512), None);
    /// 
    /// assert_eq!(n!(-2I512).checked_pow(3), Some(n!(-8)));
    /// assert_eq!(n!(-2).checked_pow(511), Some(I512::MIN));
    /// assert_eq!(n!(2I512).checked_pow(512), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline(always)]
    pub const fn checked_pow(mut self, mut exp: Exponent) -> Option<Self> {
        if S {
            return match self.unsigned_abs_internal().checked_pow(exp) {
                Some(u) => {
                    let out = u.force_sign();
                    let neg = self.is_negative_internal();
                    if !neg || exp.is_multiple_of(2) {
                        if out.is_negative_internal() { None } else { Some(out) }
                    } else {
                        let out = out.wrapping_neg();
                        if !out.is_negative_internal() { None } else { Some(out) }
                    }
                }
                None => None,
            };
        }
        // https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method
        if exp == 0 {
            return Some(Self::ONE);
        }
        let mut y = Self::ONE;
        while exp > 1 {
            if exp % 2 == 1 {
                y = match self.checked_mul(y) {
                    Some(m) => m,
                    None => return None,
                };
            }
            self = match self.checked_mul(self) {
                Some(m) => m,
                None => return None,
            };
            exp >>= 1;
        }
        self.checked_mul(y)
    }

    /// If `rhs` is positive, computes the smallest integer multiple of `rhs` that is greater than or equal to `self`. If `rhs` is negative, computes the largest integer multiple of `rhs` that is less than or equal to `self`.
    /// 
    /// Returns `None` if `rhs` is zero, or if the result is too large or too small to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(n!(4U256).checked_next_multiple_of(n!(2)), Some(n!(4)));
    /// assert_eq!(n!(17U256).checked_next_multiple_of(n!(7)), Some(n!(21)));
    /// assert_eq!(n!(1U256).checked_next_multiple_of(n!(0)), None);
    /// assert_eq!(U256::MAX.checked_next_multiple_of(n!(2)), None);
    /// assert_eq!(U256::MAX.checked_next_multiple_of(n!(1)), Some(U256::MAX));
    /// 
    /// assert_eq!(n!(-9I256).checked_next_multiple_of(n!(-3)), Some(n!(-9)));
    /// assert_eq!(n!(-10I256).checked_next_multiple_of(n!(4)), Some(n!(-8)));
    /// assert_eq!(n!(-17I256).checked_next_multiple_of(n!(-4)), Some(n!(-20)));
    /// assert_eq!(n!(83I256).checked_next_multiple_of(n!(-6)), Some(n!(78)));
    /// assert_eq!(I256::MIN.checked_next_multiple_of(n!(2)), Some(I256::MIN));
    /// assert_eq!(n!(0I256).checked_next_multiple_of(n!(3)), Some(n!(0)));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            return None;
        }
        let rem = self.wrapping_rem_euclid(rhs);
        if rem.is_zero() {
            return Some(self);
        }
        if rem.is_negative_internal() == rhs.is_negative_internal() {
            self.checked_add(rhs.sub(rem)) // `next_multiple = floor(self / rhs) * rhs + rhs = (self - rem) + rhs`
        } else {
            self.checked_sub(rem)
        }
    }

    /// Computes base-2 logarithm of `self`, rounded down, i.e. the largest integer `n` such that `2^n <= self`.
    /// 
    /// Returns `None` if `self` is less than or equal to zero.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U2048, I2048};
    /// 
    /// assert_eq!(n!(1U2048).checked_ilog2(), Some(0));
    /// assert_eq!(U2048::MAX.checked_ilog2(), Some(2047));
    /// assert_eq!(n!(0U2048).checked_ilog2(), None);
    /// 
    /// assert_eq!(I2048::MAX.checked_ilog2(), Some(2046));
    /// assert_eq!(n!(-1I2048).checked_ilog2(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_ilog2(self) -> Option<Exponent> {
        if self.is_negative_internal() {
            None
        } else {
            self.force_sign::<false>().bit_width().checked_sub(1)
        }
    }

    /// Computes base-10 logarithm of `self`, rounded down, i.e. the largest integer `n` such that `10^n <= self`.
    /// 
    /// Returns `None` if `self` is less than or equal to zero.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// 
    /// assert_eq!(n!(1U1024).checked_ilog10(), Some(0));
    /// assert_eq!(n!(1000U1024).checked_ilog10(), Some(3));
    /// assert_eq!(n!(0U1024).checked_ilog10(), None);
    /// 
    /// assert_eq!(n!(100I1024).checked_ilog10(), Some(2));
    /// assert_eq!(n!(-1I1024).checked_ilog10(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_ilog10(self) -> Option<Exponent> {
        if self.is_negative_internal() || self.is_zero() {
            return None;
        }
        if Self::BITS <= 3 || (S && Self::BITS == 4) {
            // in this case, ten cannot be represented by Self, so result will always be 0
            return Some(0);
        }
        if const { Self::from_byte(10) }.gt(&self) {
            return Some(0);
        }
        Some(Uint::iilog(1, const { Uint::from_byte(10) }, self.force_sign().div_rem_u64(10).0).0)
    }

    /// Computes logarithm of `self` to the given `base`, rounded down, i.e. the largest integer `n` such that `base^n <= self`.
    /// 
    /// Returns `None` if `self` is less than or equal to zero, or if `base` is less than 2.
    /// 
    /// Note that you should use `checked_ilog2` or `checked_ilog10` for base-2 or base-10 logarithms, respectively, as they are more efficient.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    /// 
    /// assert_eq!(n!(9U512).checked_ilog(n!(3)), Some(2));
    /// assert_eq!(n!(1U512).checked_ilog(n!(1)), None);
    /// assert_eq!(n!(0U512).checked_ilog(n!(0)), None);
    /// 
    /// assert_eq!(n!(65I512).checked_ilog(n!(4)), Some(3));
    /// assert_eq!(n!(2I512).checked_ilog(n!(-1)), None);
    /// assert_eq!(I512::MIN.checked_ilog(n!(2)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_ilog(self, base: Self) -> Option<Exponent> {
        if S && Self::BITS <= 2 || Self::BITS <= 1 {
            // in this case, can't represent two by the type, so log is undefined
            return None;
        }
        if base.is_negative_internal() || self.is_negative_internal() {
            return None;
        }
        use core::cmp::Ordering;
        match base.cmp(&const { Self::from_byte(2) }) {
            Ordering::Less => None,
            Ordering::Equal => self.checked_ilog2(),
            Ordering::Greater => {
                if self.is_zero() {
                    return None;
                }
                if base.gt(&self) {
                    return Some(0);
                }
                let base = base.force_sign();
                Some(Uint::iilog(1, base, self.force_sign().div(base)).0)
            }
        }
    }
}

#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    #[inline]
    const fn iilog(m: Exponent, b: Self, k: Self) -> (Exponent, Self) {
        // https://people.csail.mit.edu/jaffer/III/iilog.pdf
        if b.gt(&k) {
            (m, k)
        } else {
            let (new, q) = Self::iilog(m << 1, b.mul(b), k.div_rem_unchecked(b).0);
            if b.gt(&q) {
                (new, q)
            } else {
                (new + m, q.div(b))
            }
        }
    }

    /// Checked addition with a signed integer of the same bit width. Computes `self + rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(n!(1U512).checked_add_signed(n!(1)), Some(n!(2)));
    /// assert_eq!(U512::MAX.checked_add_signed(n!(1)), None);
    /// assert_eq!(n!(1U512).checked_add_signed(n!(-2)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add_signed(self, rhs: Int<N, B, OM>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_signed(rhs))
    }

    /// Checked subtraction by a signed integer of the same bit width. Computes `self - rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(n!(1U2048).checked_sub_signed(n!(-1)), Some(n!(2)));
    /// assert_eq!(U2048::MAX.checked_sub_signed(n!(-1)), None);
    /// assert_eq!(n!(1U2048).checked_sub_signed(n!(2)), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub_signed(self, rhs: Int<N, B, OM>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_signed(rhs))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if the result cannot be represented by an `Int<N, B, OM>`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(n!(1U1024).checked_signed_diff(n!(2)), Some(n!(-1)));
    /// assert_eq!(U1024::MAX.checked_signed_diff(n!(1)), None);
    /// assert_eq!(n!(2U1024).checked_signed_diff(n!(1)), Some(n!(1)));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_signed_diff(self, rhs: Self) -> Option<Int<N, B, OM>> {
        let (out, overflow) = self.overflowing_sub(rhs);
        let out = out.cast_signed();
        if overflow == out.is_negative() {
            Some(out)
        } else {
            None
        }
    }

    /// Computes the smallest power of two that is greater than or equal to `self`. Returns `None` if the result is too large to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(n!(16U256).checked_next_power_of_two(), Some(n!(16)));
    /// assert_eq!(n!(17U256).checked_next_power_of_two(), Some(n!(32)));
    /// assert_eq!(U256::MAX.checked_next_power_of_two(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_power_of_two() {
            return Some(self);
        }
        let bits = self.bit_width();
        if bits == Self::BITS {
            return None;
        }
        Some(Self::power_of_two(bits))
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    /// Checked addition with an unsigned integer of the same bit width. Computes `self + rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{I512, U512};
    /// 
    /// assert_eq!(n!(-1I512).checked_add_unsigned(n!(1)), Some(n!(0)));
    /// assert_eq!(I512::MIN.checked_add_unsigned(U512::MAX), Some(I512::MAX));
    /// assert_eq!(n!(0I512).checked_add_unsigned(U512::MAX), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add_unsigned(self, rhs: Uint<N, B, OM>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_unsigned(rhs))
    }

    /// Checked subtraction by an unsigned integer of the same bit width. Computes `self - rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{I1024, U1024};
    /// 
    /// assert_eq!(n!(-1I1024).checked_sub_unsigned(n!(1)), Some(n!(-2)));
    /// assert_eq!(n!(0I1024).checked_sub_unsigned(U1024::MAX), None);
    /// assert_eq!(I1024::MAX.checked_sub_unsigned(U1024::MAX), Some(I1024::MIN));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub_unsigned(self, rhs: Uint<N, B, OM>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_unsigned(rhs))
    }

    /// Checked absolute value. Computes the absolute value of `self`, returning `None` if `self` is equal to [`Self::MIN`].
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::I2048;
    /// 
    /// assert_eq!(n!(-123I2048).checked_abs(), Some(n!(123)));
    /// assert_eq!(n!(456I2048).checked_abs(), Some(n!(456)));
    /// assert_eq!(I2048::MIN.checked_abs(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
        tuple_to_option(self.overflowing_abs())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::n;

    #[test]
    fn test_narrow_widths_ilog10_is_zero() {
        let a = n!(7U3);
        assert_eq!(a.checked_ilog10(), Some(0));

        let b = n!(3U2);
        assert_eq!(b.checked_ilog10(), Some(0));

        let c = n!(6I4);
        assert_eq!(c.checked_ilog10(), Some(0));
    }

    #[test]
    fn test_narrow_width_checked_ilog_is_none() {
        let a = n!(1I2);
        assert_eq!(a.checked_ilog(a), None);
    }

    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: <stest>::checked_add(a: stest, b: stest),
            cases: [
                (stest::MAX, 1u8)
            ]
        }
        test_bignum! {
            function: <stest>::checked_sub(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::checked_mul(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::checked_div(a: stest, b: stest),
            cases: [
                (16777217u32 as stest, 16777216u32 as stest),
                (65536u32 as stest, 256u32 as stest),
                // (65792u32 as stest, 257u32 as stest),
                (328622u32 as stest, 10000u32 as stest), // tests the unlikely condition in the division algorithm at step D5
                (2074086u32 as stest, 76819u32 as stest) // tests the unlikely condition in the division algorithm at step D5,
            ]
        }
        test_bignum! {
            function: <stest>::checked_div_euclid(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::checked_rem(a: stest, b: stest),
            cases: [
                (65536u32 as stest, 256u32 as stest)
            ]
        }
        test_bignum! {
            function: <stest>::checked_rem_euclid(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::checked_neg(a: stest)
        }
        test_bignum! {
            function: <stest>::checked_shl(a: stest, b: u16)
        }
        test_bignum! {
            function: <stest>::checked_shr(a: stest, b: u16),
            cases: [
                (stest::MIN, stest::BITS as u16 - 1)
            ]
        }
        test_bignum! {
            function: <stest>::checked_pow(a: stest, b: u16),
            cases: [(2, 512)]
        }
        #[cfg(nightly)] // since int_roundings are not stable yet
        test_bignum! {
            function: <stest>::checked_next_multiple_of(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::checked_ilog(a: stest, b: stest),
            cases: [
                (2u8, 60u8),
                (stest::MAX, 2u8)
            ]
        }
        test_bignum! {
            function: <stest>::checked_ilog2(a: stest)
        }
        test_bignum! {
            function: <stest>::checked_ilog10(a: stest)
        }
    }

    crate::test::test_all! {
        testing unsigned;

        test_bignum! {
            function: <utest>::checked_add_signed(a: utest, b: itest)
        }
        test_bignum! {
            function: <utest>::checked_sub_signed(a: utest, b: itest)
        }
        test_bignum! {
            function: <utest>::checked_signed_diff(a: utest, b: utest)
        }
        test_bignum! {
            function: <stest>::checked_next_power_of_two(a: stest),
            cases: [
                (stest::MAX)
            ]
        }
    }

    crate::test::test_all! {
        testing signed;
        
        test_bignum! {
            function: <itest>::checked_add_unsigned(a: itest, b: utest)
        }
        test_bignum! {
            function: <itest>::checked_sub_unsigned(a: itest, b: utest)
        }
    }
}

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::checked_div(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::checked_rem(a: utest, b: utest)
    }
}