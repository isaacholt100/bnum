macro_rules! impls {
    ($sign: ident) => {
        #[doc = doc::unchecked::unchecked_add!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
            unsafe { self.checked_add(rhs).unwrap_unchecked() }
        }

        #[doc = doc::unchecked::unchecked_sub!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
            unsafe { self.checked_sub(rhs).unwrap_unchecked() }
        }

        #[doc = doc::unchecked::unchecked_mul!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
            unsafe { self.checked_mul(rhs).unwrap_unchecked() }
        }

        #[doc = doc::unchecked::unchecked_shl!($sign)]
        #[must_use = doc::must_use_op!()]
        #[inline]
        pub const unsafe fn unchecked_shl(self, rhs: crate::ExpType) -> Self {
            unsafe { self.checked_shl(rhs).unwrap_unchecked() }
        }

        #[doc = doc::unchecked::unchecked_shr!($sign)]
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
        use crate::test::{test_bignum, types::*};

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
