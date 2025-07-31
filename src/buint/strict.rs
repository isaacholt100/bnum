use super::Uint;

#[doc = doc::strict::impl_desc!()]
impl<const N: usize> Uint<N> {
    crate::ints::strict::impls!(U);

    #[cfg(feature = "signed")]
    #[doc = doc::strict::strict_add_signed!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_signed(self, rhs: crate::Int<N>) -> Self {
        self.checked_add_signed(rhs)
            .expect(crate::errors::err_msg!("attempt to add with overflow"))
    }
}

#[cfg(all(test, feature = "nightly"))] // since strict_overflow_ops are not stable yet
crate::test::test_all_widths! {
    crate::ints::strict::tests!(utest);

    #[cfg(feature = "signed")]
    test_bignum! {
        function: <utest>::strict_add_signed(a: utest, b: itest),
        skip: a.checked_add_signed(b).is_none()
    }
}

use crate::doc;
