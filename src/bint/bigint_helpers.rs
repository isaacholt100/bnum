macro_rules! bigint_helpers {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        #[doc = doc::bigint_helpers::impl_desc!()]
        impl<const N: usize> $BInt<N> {
            crate::int::bigint_helpers::impls!(I);
        }

        #[cfg(test)]
        paste::paste! {
            mod [<$Digit _digit_tests>] {
                use crate::test::types::big_types::$Digit::*;
                crate::int::bigint_helpers::tests!(itest);
            }
        }
    };
}

use crate::doc;

crate::macro_impl!(bigint_helpers);
