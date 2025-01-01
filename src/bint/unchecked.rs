macro_rules! unchecked {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        crate::int::unchecked::impls!($BInt, I);
    };
}

#[cfg(test)]
crate::test::all_digit_tests! {
    crate::int::unchecked::tests!(itest);
}

use crate::doc;

crate::macro_impl!(unchecked);
