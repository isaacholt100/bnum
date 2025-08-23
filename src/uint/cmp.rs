use super::Uint;
use core::cmp::{Ord, Ordering, PartialOrd};

impl<const N: usize> PartialOrd for Uint<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Uint<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp(self, other)
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self::max(self, other)
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self::min(self, other)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self::clamp(self, min, max)
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    crate::ints::cmp::tests!(utest);
}

#[cfg(test)]
crate::test::test_all_widths_against_old_types! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::eq(a: ref &utest, b: ref &utest)
    }
    test_bignum! {
        function: <utest as PartialEq>::eq(a: ref &utest, b: ref &utest)
    }
    test_bignum! {
        function: <utest as Ord>::cmp(a: ref &utest, b: ref &utest)
    }
}
