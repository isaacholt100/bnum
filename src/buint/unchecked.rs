macro_rules! unchecked {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        crate::int::unchecked::impls!($BUint, U);

        #[cfg(test)]
        paste::paste! {
            mod [<$Digit _digit_tests>] {
                use crate::test::types::big_types::$Digit::*;
                crate::int::unchecked::tests!(utest);
            }
        }
    };
}

use crate::doc;

crate::macro_impl!(unchecked);
