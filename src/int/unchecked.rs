use super::Int;

#[doc = doc::unchecked::impl_desc!()]
impl<const N: usize> Int<N> {
    crate::ints::unchecked::impls!(I);

    #[doc = doc::unchecked::unchecked_neg!(I)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const unsafe fn unchecked_neg(self) -> Self {
        unsafe { self.checked_neg().unwrap_unchecked() }
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    crate::ints::unchecked::tests!(itest);

    #[cfg(feature = "nightly")] // since unchecked_neg is not stable yet
    test_bignum! {
        function: unsafe <itest>::unchecked_neg(a: itest),
        skip: a.checked_neg().is_none()
    }
}

use crate::doc;
