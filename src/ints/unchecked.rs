macro_rules! impls {
    ($sign: ident) => {
        /// Unchecked integer addition. Computes `self + rhs` without checking for overflow, resulting in undefined behavior if overflow occurs.
        /// 
        /// `a.unchecked_add(b)` is equivalent to `a.checked_add(b).unwrap_unchecked()`.
        /// 
        /// # Safety
        /// 
        /// This results in undefined behaviour if overflow occurs, i.e. when [`checked_add`](Self::checked_add) would return `None`.
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
            unsafe { self.checked_add(rhs).unwrap_unchecked() }
        }


        /// Unchecked integer subtraction. Computes `self - rhs` without checking for overflow, resulting in undefined behavior if overflow occurs.
        /// 
        /// `a.unchecked_sub(b)` is equivalent to `a.checked_sub(b).unwrap_unchecked()`.
        /// 
        /// # Safety
        /// 
        /// This results in undefined behaviour if overflow occurs, i.e. when [`checked_sub`](Self::checked_sub) would return `None`.
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
            unsafe { self.checked_sub(rhs).unwrap_unchecked() }
        }

        /// Unchecked integer multiplication. Computes `self * rhs` without checking for overflow, resulting in undefined behavior if overflow occurs.
        /// 
        /// `a.unchecked_mul(b)` is equivalent to `a.checked_mul(b).unwrap_unchecked()`.
        /// 
        /// # Safety
        ///
        /// This results in undefined behaviour if overflow occurs, i.e. when [`checked_mul`](Self::checked_mul) would return `None`.
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
            unsafe { self.checked_mul(rhs).unwrap_unchecked() }
        }

        /// Unchecked left shift. Computes `self << rhs` without checking for overflow, resulting in undefined behavior if overflow occurs.
        /// 
        /// `a.unchecked_shl(b)` is equivalent to `a.checked_shl(b).unwrap_unchecked()`.
        /// 
        /// # Safety
        /// 
        /// This results in undefined behaviour if `rhs` is greater than or equal to `Self::BITS`, i.e. when [`checked_shl`](Self::checked_shl) would return `None`.
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_shl(self, rhs: crate::ExpType) -> Self {
            unsafe { self.checked_shl(rhs).unwrap_unchecked() }
        }

        /// Unchecked right shift. Computes `self >> rhs` without checking for overflow, resulting in undefined behavior if overflow occurs.
        /// 
        /// `a.unchecked_shr(b)` is equivalent to `a.checked_shr(b).unwrap_unchecked()`.
        /// 
        /// # Safety
        ///
        /// This results in undefined behaviour if `rhs` is greater than or equal to `Self::BITS`, i.e. when [`checked_shr`](Self::checked_shr) would return `None`.
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_shr(self, rhs: crate::ExpType) -> Self {
            unsafe { self.checked_shr(rhs).unwrap_unchecked() }
        }
    };
}

pub(crate) use impls;

#[cfg(test)]
macro_rules! tests {
    ($int: ty) => {
        use crate::test::test_bignum;

        test_bignum! {
            function: unsafe <$int>::unchecked_add(a: $int, b: $int),
            skip: a.checked_add(b).is_none()
        }
        test_bignum! {
            function: unsafe <$int>::unchecked_sub(a: $int, b: $int),
            skip: a.checked_sub(b).is_none()
        }
        test_bignum! {
            function: unsafe <$int>::unchecked_mul(a: $int, b: $int),
            skip: a.checked_mul(b).is_none()
        }
        #[cfg(feature = "nightly")] // since unchecked_shifts are not stable yet
        test_bignum! {
            function: unsafe <$int>::unchecked_shl(a: $int, b: u8),
            skip: a.checked_shl(b as u32).is_none()
        }
        #[cfg(feature = "nightly")] // since unchecked_shifts are not stable yet
        test_bignum! {
            function: unsafe <$int>::unchecked_shr(a: $int, b: u8),
            skip: a.checked_shr(b as u32).is_none()
        }
    };
}

#[cfg(test)]
pub(crate) use tests;
