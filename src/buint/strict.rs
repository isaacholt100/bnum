use super::BUintD8;
use crate::{digit, Digit, BIntD8};

#[doc = doc::strict::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    crate::int::strict::impls!(U);

    #[doc = doc::strict::strict_add_signed!(U)]
    #[must_use = doc::must_use_op!()]
    #[inline]
    pub const fn strict_add_signed(self, rhs: BIntD8<N>) -> Self {
        crate::errors::option_expect!(
            self.checked_add_signed(rhs),
            crate::errors::err_msg!("attempt to add with overflow")
        )
    }
}

#[cfg(test)]
mod tests {
    crate::int::strict::tests!(utest);

    test_bignum! {
        function: <utest>::strict_add_signed(a: utest, b: itest),
        skip: a.checked_add_signed(b).is_none()
    }
}

use crate::doc;
