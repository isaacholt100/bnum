use super::Int;

#[doc = doc::bigint_helpers::impl_desc!()]
impl<const N: usize> Int<N> {
    crate::ints::bigint_helpers::impls!(I);
}

#[cfg(all(test, feature = "nightly"))] // since bigint_helper_methods are not stable yet
crate::test::test_all_widths! {
    crate::ints::bigint_helpers::tests!(itest);
}

use crate::doc;
