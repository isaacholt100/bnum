use super::Uint;
use crate::ExpType;
use crate::{Integer, Int};
use crate::doc;
use crate::helpers::tuple_to_option;

macro_rules! impl_desc {
    () => {
        "Checked arithmetic methods which act on `self`: `self.checked_...`. Each method cannot panic and returns an `Option<Self>`. `None` is returned when overflow would have occurred or there was an attempt to divide by zero or calculate a remainder with a divisor of zero."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize> Integer<S, N> {
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
    /// use bnum::types::U1024;
    ///
    /// assert_eq!(1.as_::<U1024>().checked_add(1.as_()), Some(2.as_()));
    /// assert_eq!(U1024::MAX.checked_add(U1024::ONE), None);
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
    /// use bnum::types::U256;
    /// 
    /// assert_eq!(1.as_::<U256>().checked_sub(1.as_()), Some(0.as_()));
    /// assert_eq!(U256::MIN.checked_sub(U256::ONE), None);
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
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(1.as_::<U512>().checked_mul(1.as_()), Some(1.as_()));
    /// assert_eq!(U512::MAX.checked_mul(3.as_()), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        Self::tuple_to_option(self.overflowing_mul(rhs))
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
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
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
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
            None
        } else {
            Some(self.div_rem_euclid_unchecked(rhs).0)
        }
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
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
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
        if rhs.is_zero() || self.is_division_overflow(&rhs) {
            None
        } else {
            Some(self.div_rem_euclid_unchecked(rhs).1)
        }
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
    /// use bnum::types::U2048;
    /// 
    /// assert_eq!(1.as_::<U2048>().checked_shl(1), Some(2.as_()));
    /// assert_eq!(1.as_::<U2048>().checked_shl(2048), None);
    /// assert_eq!(2.as_::<U2048>().checked_shl(2047), Some(0.as_()));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
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
    /// use bnum::types::U1024;
    ///
    /// assert_eq!(1.as_::<U1024>().checked_shr(1), Some(0.as_()));
    /// assert_eq!(U1024::MAX.checked_shr(1024), None);
    /// assert_eq!(U1024::MAX.checked_shr(1023), Some(1.as_()));
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
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
    /// use bnum::types::U512;
    /// 
    /// assert_eq!(2.as_::<U512>().checked_pow(10), Some(1024.as_()));
    /// assert_eq!(2.as_::<U512>().checked_pow(512), None);
    /// ```
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_pow(mut self, mut exp: ExpType) -> Option<Self> {
        if S {
            return match self.unsigned_abs_internal().checked_pow(exp) {
                Some(u) => {
                    let out = u.force_sign();
                    let neg = self.is_negative_internal();
                    if !neg || exp % 2 == 0 {
                        if out.is_negative_internal() { None } else { Some(out) }
                    } else {
                        let out = out.wrapping_neg();
                        if !out.is_negative_internal() { None } else { Some(out) }
                    }
                }
                None => None,
            };
        }
        // TODO: see if Rust compiler can optimise use of overflowing_pow here
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
        // match self.checked_rem(rhs) {
        //     Some(rem) => {
        //         if rem.is_zero() {
        //             // `rhs` divides `self` exactly so just return `self`
        //             Some(self)
        //         } else {
        //             // `next_multiple = floor(self / rhs) * rhs + rhs = (self - rem) + rhs`
        //             self.checked_add(rhs.sub(rem))
        //         }
        //     }
        //     None => None,
        // }
        if rhs.is_zero() {
            return None;
        }
        let rem = self.wrapping_rem_euclid(rhs); // TODO: we are checking that rhs is non-zero twice here
        if rem.is_zero() {
            return Some(self);
        }
        if rem.is_negative_internal() == rhs.is_negative_internal() {
            self.checked_add(rhs.sub(rem)) // `next_multiple = floor(self / rhs) * rhs + rhs = (self - rem) + rhs`
        } else {
            self.checked_sub(rem)
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
        if self.is_negative_internal() {
            None
        } else {
            self.force_sign::<false>().bits().checked_sub(1)
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
        if S && Self::BITS <= 2 {
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
impl<const N: usize> Uint<N> {
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

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize> Int<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_add_unsigned(self, rhs: Uint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_unsigned(rhs))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_sub_unsigned(self, rhs: Uint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_unsigned(rhs))
    }

    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
        tuple_to_option(self.overflowing_abs())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;

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
            function: <stest>::checked_shr(a: stest, b: u16)
        }
        test_bignum! {
            function: <stest>::checked_pow(a: stest, b: u16)
        }
        #[cfg(feature = "nightly")] // since int_roundings are not stable yet
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
        #[cfg(feature = "nightly")] // since unsigned_signed_diff is not stabilised yet
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
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::checked_div(a: utest, b: utest)
    }
    test_bignum! {
        function: <utest>::checked_rem(a: utest, b: utest)
    }
}