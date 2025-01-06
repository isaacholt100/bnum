use super::BUintD8;
use crate::{digit, Digit};

crate::int::unchecked::impls!(BUintD8, U);

#[cfg(test)]
mod tests {
    use crate::test::types::*;

    crate::int::unchecked::tests!(utest);
}

use crate::doc;
