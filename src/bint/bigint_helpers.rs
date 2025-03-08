macro_rules! bigint_helpers {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        #[doc = doc::bigint_helpers::impl_desc!()]
        impl<const N: usize> $BInt<N> {
            crate::int::bigint_helpers::impls!(I);
        }
    };
}

#[cfg(all(test, feature = "nightly"))] // as bigint_helpers not stabilised yet
crate::test::all_digit_tests! {
    use crate::test::types::itest;
    
    crate::int::bigint_helpers::tests!(itest);
}

use crate::doc;

crate::macro_impl!(bigint_helpers);
