macro_rules! strict {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        #[doc = doc::strict::impl_desc!()]
        impl<const N: usize> $BUint<N> {
            crate::int::strict::impls!(U);

            #[doc = doc::strict::strict_add_signed!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn strict_add_signed(self, rhs: $BInt<N>) -> Self {
                crate::errors::option_expect!(
                    self.checked_add_signed(rhs),
                    crate::errors::err_msg!("attempt to add with overflow")
                )
            }
        }
        
        #[cfg(test)]
        paste::paste! {
            mod [<$Digit _digit_tests>] {
                use crate::test::types::big_types::$Digit::*;

                crate::int::strict::tests!(utest);

                test_bignum! {
                    function: <utest>::strict_add_signed(a: utest, b: itest),
                    skip: a.checked_add_signed(b).is_none()
                }
            }
        }
    };
}

use crate::doc;

crate::macro_impl!(strict);
