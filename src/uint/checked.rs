use super::Uint;
use crate::ExpType;
use crate::doc;
use crate::helpers::tuple_to_option;
use crate::digit;

#[doc = doc::checked::impl_desc!()]
impl<const N: usize> Uint<N> {
    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred.
    ///
    /// # Examples
    /// 
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    ///
    /// assert_eq!(1.as_::<U1024>().checked_add(1.as_()), Some(2.as_()));
    /// assert_eq!(U1024::MAX.checked_add(U1024::ONE), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }

    #[cfg(feature = "signed")]
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
    /// assert_eq!(1.as_::<U512>().checked_add_signed(1.as_()), Some(2.as_()));
    /// assert_eq!(U512::MAX.checked_add_signed(I512::ONE), None);
    /// assert_eq!(1.as_::<U512>().checked_add_signed(-2.as_::<I512>()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add_signed(self, rhs: crate::Int<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_signed(rhs))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(1.as_::<U256>().checked_sub(1.as_()), Some(0.as_()));
    /// assert_eq!(U256::MIN.checked_sub(U256::ONE), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }

    #[cfg(feature = "signed")]
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
    /// assert_eq!(1.as_::<U2048>().checked_sub_signed(-1.as_()), Some(2.as_()));
    /// assert_eq!(U2048::MAX.checked_sub_signed(-1.as_()), None);
    /// assert_eq!(1.as_::<U2048>().checked_sub_signed(2.as_::<I2048>()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub_signed(self, rhs: crate::Int<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_signed(rhs))
    }

    #[cfg(feature = "signed")]
    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if the result cannot be represented by an `Int<N>`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U1024, I1024};;
    /// 
    /// assert_eq!(1.as_::<U1024>().checked_signed_diff(2.as_()), -1.as_::<I1024>());
    /// assert_eq!(U1024::MAX.checked_signed_diff(U1024::ONE), None);
    /// assert_eq!(2.as_::<U1024>().checked_signed_diff(1.as_()), Some(1.as_::<I1024>()));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_signed_diff(self, rhs: Self) -> Option<crate::Int<N>> {
        let (out, overflow) = self.overflowing_sub(rhs);
        let out = out.cast_signed();
        if overflow == out.is_negative() {
            Some(out)
        } else {
            None
        }
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(1.as_::<U512>().checked_mul(1.as_()), Some(1.as_()));
    /// assert_eq!(U512::MAX.checked_mul(3.as_()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }

    #[inline]
    pub(crate) const fn div_rem_u64(self, rhs: u64) -> (Self, u64) {
        let mut out = Self::ZERO;
        let mut rem: u64 = 0;
        let mut i = N.div_ceil(8);
        while i > 0 {
            i -= 1;
            let d = unsafe { self.as_wide_digits().u64_digit(i) };
            let (q, r) = digit::div_rem_wide_u64(d, rem, rhs);
            rem = r;
            unsafe { out.as_wide_digits_mut().set_u64_digit(i, q) };
        }
        (out, rem)
    }

    #[inline]
    pub(crate) const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
        use core::cmp::Ordering;

        match self.cmp(&rhs) {
            Ordering::Less => (Self::ZERO, self),
            Ordering::Equal => (Self::ONE, Self::ZERO),
            Ordering::Greater => {
                let bit_width = rhs.bits();
                if bit_width <= 64 {
                    let d = unsafe { rhs.as_wide_digits().u64_digit(0) };
                    let (div, rem) = self.div_rem_u64(d);
                    let mut out = Self::ZERO;
                    unsafe { out.as_wide_digits_mut().set_u64_digit(0, rem) };
                    (div, out)
                } else {
                    // if rhs.is_power_of_two() {
                    //     return (self.wrapping_shr(rhs.ilog2()), self.bitand(rhs.wrapping_sub(Self::ONE)));
                    // }
                    self.basecase_div_rem(rhs, rhs.bits().div_ceil(8) as usize)
                }
            }
        }
    }

    #[inline]
    pub(crate) const fn div_rem(self, rhs: Self) -> (Self, Self) {
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!(
                crate::errors::div_by_zero_message!()
            ));
        } else {
            self.div_rem_unchecked(rhs)
        }
    }

    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs` is zero.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(5.as_::<U256>().checked_div(2.as_()), Some(2.as_()));
    /// assert_eq!(1.as_::<U256>().checked_div(0.as_()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).0)
        }
    }

    /// Checked Euclidean division. Computes `self.div_euclid(rhs)`, returning `None` if `rhs` is zero.
    /// 
    /// Note that this is equivalent to `self.checked_div(rhs)`, since the division only involves non-negative integers.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(13.as_::<U2048>().checked_div_euclid(5.as_()), Some(2.as_()));
    /// assert_eq!(U2048::MAX.checked_div_euclid(0.as_()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_div(rhs)
    }

    /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs` is zero.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(5.as_::<U1024>().checked_rem(2.as_()), Some(1.as_()));
    /// assert_eq!(1.as_::<U1024>().checked_rem(0.as_()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).1)
        }
    }

    /// Checked Euclidean remainder. Computes `self.rem_euclid(rhs)`, returning `None` if `rhs` is zero.
    ///
    /// Note that this is equivalent to `self.checked_rem(rhs)`, since the calculation only involves non-negative integers.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(13.as_::<U512>().checked_rem_euclid(5.as_()), Some(3.as_()));
    /// assert_eq!(U512::MAX.checked_rem_euclid(0.as_()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_rem(rhs)
    }

    /// Checked negation. Computes `-self`, returning `None` if `self` is not zero.
    /// 
    /// Note that negating any strictly positive integer will result in an overflow.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(1.as_::<U256>().checked_neg(), None);
    /// assert_eq!(0.as_::<U256>().checked_neg(), Some(0.as_()));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        if self.is_zero() { Some(self) } else { None }
    }

    /// Checked left shift. Computes `self << rhs`, returning `None` if `rhs` is greater than or equal to `Self::BITS`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(1.as_::<U2048>().checked_shl(1), Some(2.as_()));
    /// assert_eq!(1.as_::<U2048>().checked_shl(2048), None);
    /// assert_eq!(2.as_::<U2048>().checked_shl(2047), Some(0.as_()));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            unsafe { Some(Self::unchecked_shl_internal(self, rhs)) }
        }
    }

    /// Checked right shift. Computes `self >> rhs`, returning `None` if `rhs` is greater than or equal to `Self::BITS`.
    /// 
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    ///
    /// assert_eq!(1.as_::<U1024>().checked_shr(1), Some(0.as_()));
    /// assert_eq!(U1024::MAX.checked_shr(1024), None);
    /// assert_eq!(U1024::MAX.checked_shr(1023), Some(1.as_()));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            unsafe { Some(Self::unchecked_shr_internal(self, rhs)) }
        }
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None` if overflow occurred.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(2.as_::<U512>().checked_pow(10), Some(1024.as_()));
    /// assert_eq!(2.as_::<U512>().checked_pow(512), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_pow(mut self, mut exp: ExpType) -> Option<Self> {
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

    /// Computes the smallest integer multiple of `rhs` that is greater than or equal to `self`. Returns `None` if `rhs` is zero, or if the result is too large to be represented by `Self`.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(4.as_::<U256>().checked_next_multiple_of(2.as_()), Some(4.as_()));
    /// assert_eq!(17.as_::<U256>().checked_next_multiple_of(7.as_()), Some(21.as_()));
    /// assert_eq!(1.as_::<U256>().checked_next_multiple_of(0.as_()), None);
    /// assert_eq!(U256::MAX.checked_next_multiple_of(2.as_()), None);
    /// assert_eq!(U256::MAX.checked_next_multiple_of(1.as_()), Some(U256::MAX));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        match self.checked_rem(rhs) {
            Some(rem) => {
                if rem.is_zero() {
                    // `rhs` divides `self` exactly so just return `self`
                    Some(self)
                } else {
                    // `next_multiple = floor(self / rhs) * rhs + rhs = (self - rem) + rhs`
                    self.checked_add(rhs.sub(rem))
                }
            }
            None => None,
        }
    }

    /// Computes base-2 logarithm of `self`, rounded down; i.e., the largest integer `n` such that `2^n <= self`.
    /// 
    /// Returns `None` if `self` is zero.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(1.as_::<U2048>().checked_ilog2(), Some(0));
    /// assert_eq!(U2048::MAX.checked_ilog2(), Some(2047));
    /// assert_eq!(0.as_::<U2048>().checked_ilog2(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_ilog2(self) -> Option<ExpType> {
        self.bits().checked_sub(1)
    }

    #[inline]
    const fn iilog(m: ExpType, b: Self, k: Self) -> (ExpType, Self) {
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

    /// Computes base-10 logarithm of `self`, rounded down; i.e., the largest integer `n` such that `10^n <= self`.
    /// 
    /// Returns `None` if `self` is zero.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U1024;
    /// 
    /// assert_eq!(1.as_::<U1024>().checked_ilog10(), Some(0));
    /// assert_eq!(1000.as_::<U1024>().checked_ilog10(), Some(2));
    /// assert_eq!(0.as_::<U1024>().checked_ilog10(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_ilog10(self) -> Option<ExpType> {
        if self.is_zero() {
            return None;
        }
        if Self::TEN.gt(&self) {
            return Some(0);
        }
        Some(Self::iilog(1, Self::TEN, self.div_rem_u64(10).0).0)
    }

    /// Computes logarithm of `self` with the given `base`, rounded down; i.e., the largest integer `n` such that `base^n <= self`.
    /// 
    /// Returns `None` if `self` is zero, or if `base` is less than 2.
    /// 
    /// Note that you should use `checked_ilog2` or `checked_ilog10` for base-2 or base-10 logarithms, respectively, as they are more efficient.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(9.as_::<U512>().checked_ilog(3.as_()), Some(2));
    /// assert_eq!(1.as_::<U512>().checked_ilog(1.as_()), None);
    /// assert_eq!(0.as_::<U512>().checked_ilog(0.as_()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_ilog(self, base: Self) -> Option<ExpType> {
        use core::cmp::Ordering;
        match base.cmp(&Self::TWO) {
            Ordering::Less => None,
            Ordering::Equal => self.checked_ilog2(),
            Ordering::Greater => {
                if self.is_zero() {
                    return None;
                }
                if base.gt(&self) {
                    return Some(0);
                }
                Some(Self::iilog(1, base, self.div(base)).0)
            }
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
    /// assert_eq!(16.as_::<U256>().checked_next_power_of_two(), Some(16.as_()));
    /// assert_eq!(17.as_::<U256>().checked_next_power_of_two(), Some(32.as_()));
    /// assert_eq!(U256::MAX.checked_next_power_of_two(), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_power_of_two() {
            return Some(self);
        }
        let bits = self.bits();
        if bits == Self::BITS {
            return None;
        }
        Some(Self::power_of_two(bits))
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::checked_add(a: utest, b: utest),
        cases: [
            (utest::MAX, 1u8)
        ]
    }
    #[cfg(feature = "signed")]
    test_bignum! {
        function: <utest>::checked_add_signed(a: utest, b: itest)
    }
    test_bignum! {
        function: <utest>::checked_sub(a: utest, b: utest)
    }
    #[cfg(all(feature = "signed", feature = "nightly"))] // since mixed_integer_ops_unsigned_sub is not stabilised yet
    test_bignum! {
        function: <utest>::checked_sub_signed(a: utest, b: itest)
    }
    #[cfg(feature = "nightly")] // since unsigned_signed_diff is not stabilised yet
    test_bignum! {
        function: <utest>::checked_signed_diff(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::checked_mul(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::checked_div(a: utest, b: utest),
        cases: [
            (16777217u32 as utest, 16777216u32 as utest),
            (65536u32 as utest, 256u32 as utest),
            // (65792u32 as utest, 257u32 as utest),
            (328622u32 as utest, 10000u32 as utest), // tests the unlikely condition in the division algorithm at step D5
            (2074086u32 as utest, 76819u32 as utest) // tests the unlikely condition in the division algorithm at step D5,
        ]
    }
    test_bignum! {
        function: <utest>::checked_div_euclid(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::checked_rem(a: utest, b: utest),
        cases: [
            (65536u32 as utest, 256u32 as utest)
        ]
    }
    test_bignum! {
        function: <utest>::checked_rem_euclid(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::checked_neg(a: utest)
    }
    test_bignum! {
        function: <utest>::checked_shl(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::checked_shr(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::checked_pow(a: utest, b: u16)
    }
    test_bignum! {
        function: <utest>::checked_ilog(a: utest, b: utest),
        cases: [
            (2u8, 60u8),
            (utest::MAX, 2u8)
        ]
    }
    test_bignum! {
        function: <utest>::checked_ilog2(a: utest)
    }
    test_bignum! {
        function: <utest>::checked_ilog10(a: utest)
    }
    test_bignum! {
        function: <utest>::checked_next_power_of_two(a: utest),
        cases: [
            (utest::MAX)
        ]
    }
}

#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::checked_div(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::checked_rem(a: utest, b: utest)
    }
}