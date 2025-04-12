use super::BUintD8;

crate::int::unchecked::impls!(BUintD8, U);

#[cfg(test)]
mod tests {
    crate::int::unchecked::tests!(utest);
}

use crate::doc;
