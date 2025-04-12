use super::BIntD8;

#[doc = doc::bigint_helpers::impl_desc!()]
impl<const N: usize> BIntD8<N> {
    crate::int::bigint_helpers::impls!(I);
}

#[cfg(all(test, feature = "nightly"))] // since bigint_helper_methods are not stable yet
mod tests {
    use crate::test::types::itest;

    crate::int::bigint_helpers::tests!(itest);
}

use crate::doc;
