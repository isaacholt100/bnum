use super::Int;

#[doc = doc::unchecked::impl_desc!()]
impl<const N: usize> Int<N> {
    crate::ints::unchecked::impls!(I);
}

#[cfg(test)]
crate::test::test_all_widths! {
    crate::ints::unchecked::tests!(itest);
}

use crate::doc;
