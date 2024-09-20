macro_rules! strict {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        #[doc = doc::strict::impl_desc!()]
        impl<const N: usize> $BInt<N> {
            crate::int::strict::impls!(I);

            #[doc = doc::strict::strict_abs!(I)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn strict_abs(self) -> Self {
                crate::errors::option_expect!(
                    self.checked_abs(),
                    crate::errors::err_msg!("attempt to negate with overflow")
                )
            }

            #[doc = doc::strict::strict_add_unsigned!(I)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn strict_add_unsigned(self, rhs: $BUint<N>) -> Self {
                crate::errors::option_expect!(
                    self.checked_add_unsigned(rhs),
                    crate::errors::err_msg!("attempt to add with overflow")
                )
            }

            #[doc = doc::strict::strict_sub_unsigned!(I)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn strict_sub_unsigned(self, rhs: $BUint<N>) -> Self {
                crate::errors::option_expect!(
                    self.checked_sub_unsigned(rhs),
                    crate::errors::err_msg!("attempt to subtract with overflow")
                )
            }
        }

        #[cfg(test)]
        paste::paste! {
            mod [<$Digit _digit_tests>] {
                use crate::test::types::big_types::$Digit::*;
                crate::int::strict::tests!(itest);
                
                test_bignum! {
                    function: <itest>::strict_abs(a: itest),
                    skip: a.checked_abs().is_none()
                }
                test_bignum! {
                    function: <itest>::strict_add_unsigned(a: itest, b: utest),
                    skip: a.checked_add_unsigned(b).is_none()
                }
                test_bignum! {
                    function: <itest>::strict_sub_unsigned(a: itest, b: utest),
                    skip: a.checked_sub_unsigned(b).is_none()
                }
            }
        }
    };
}

use crate::doc;

crate::macro_impl!(strict);
