use super::Uint;

#[doc = doc::unchecked::impl_desc!()]
impl<const N: usize> Uint<N> {
    crate::ints::unchecked::impls!(U);
}

#[cfg(test)]
crate::test::test_all_widths! {
    crate::ints::unchecked::tests!(utest);
}

use crate::doc;
