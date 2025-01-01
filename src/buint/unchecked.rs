macro_rules! unchecked {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        crate::int::unchecked::impls!($BUint, U);
    };
}

#[cfg(test)]
crate::test::all_digit_tests! {
    use crate::test::types::*;

    crate::int::unchecked::tests!(utest);
}

use crate::doc;

crate::macro_impl!(unchecked);
