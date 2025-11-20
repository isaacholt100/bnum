use crate::{Integer, Int};

macro_rules! impl_desc {
    () => {
        "Unchecked arithmetic methods which act on `self`: `self.unchecked_...`. Each method results in undefined behavior if overflow occurs (i.e. when the checked equivalent would return `None`). These methods should therefore only be used when it can be guaranteed that overflow will not occur. If you want to avoid panicking in arithmetic in debug mode, use the wrapping equivalents instead."
    };
}

#[doc = impl_desc!()]
impl<const S: bool, const N: usize, const OM: u8> Integer<S, N, OM> {
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
    pub const unsafe fn unchecked_shl(self, rhs: crate::Exponent) -> Self {
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
    pub const unsafe fn unchecked_shr(self, rhs: crate::Exponent) -> Self {
        unsafe { self.checked_shr(rhs).unwrap_unchecked() }
    }
}

#[doc = concat!("(Signed integers only.) ", impl_desc!())]
impl<const N: usize, const OM: u8> Int<N, OM> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const unsafe fn unchecked_neg(self) -> Self {
        unsafe { self.checked_neg().unwrap_unchecked() }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    
    crate::test::test_all! {
        testing integers;

        test_bignum! {
            function: unsafe <stest>::unchecked_add(a: stest, b: stest),
            skip: a.checked_add(b).is_none()
        }
        test_bignum! {
            function: unsafe <stest>::unchecked_sub(a: stest, b: stest),
            skip: a.checked_sub(b).is_none()
        }
        test_bignum! {
            function: unsafe <stest>::unchecked_mul(a: stest, b: stest),
            skip: a.checked_mul(b).is_none()
        }
        #[cfg(feature = "nightly")] // since unchecked_shifts are not stable yet
        test_bignum! {
            function: unsafe <stest>::unchecked_shl(a: stest, b: u8),
            skip: a.checked_shl(b as u32).is_none()
        }
        #[cfg(feature = "nightly")] // since unchecked_shifts are not stable yet
        test_bignum! {
            function: unsafe <stest>::unchecked_shr(a: stest, b: u8),
            skip: a.checked_shr(b as u32).is_none()
        }
    }
    crate::test::test_all! {
        testing signed;

        #[cfg(feature = "nightly")] // since unchecked_neg is not stable yet
        test_bignum! {
            function: unsafe <itest>::unchecked_neg(a: itest),
            skip: a.checked_neg().is_none()
        }
    }
}

use crate::doc;
