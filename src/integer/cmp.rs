use crate::Integer;
use core::cmp::{Ord, Ordering, PartialOrd, PartialEq};

impl<const S: bool, const N: usize, const B: usize, const OM: u8> PartialEq for Integer<S, N, B, OM> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Self::eq(self, other)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Eq for Integer<S, N, B, OM> {}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> PartialOrd for Integer<S, N, B, OM> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Ord for Integer<S, N, B, OM> {
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
mod tests {
    use crate::test::test_bignum;
    use core::cmp::Ord;

    crate::test::test_all! {
        testing integers;
        
        test_bignum! {
            function: <stest as PartialEq>::eq(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as PartialOrd>::partial_cmp(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as PartialOrd>::lt(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as PartialOrd>::le(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as PartialOrd>::gt(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as PartialOrd>::ge(a: ref &stest, b: ref &stest)
        }

        test_bignum! {
            function: <stest>::cmp(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest>::max(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::min(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest>::clamp(a: stest, min: stest, max: stest),
            skip: min > max
        }

        test_bignum! {
            function: <stest as Ord>::cmp(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as Ord>::max(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest as Ord>::min(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest as Ord>::clamp(a: stest, min: stest, max: stest),
            skip: min > max
        }
    }
}

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <utest>::eq(a: ref &utest, b: ref &utest)
    }
    test_bignum! {
        function: <itest>::eq(a: ref &itest, b: ref &itest)
    }
    test_bignum! {
        function: <utest>::cmp(a: ref &utest, b: ref &utest)
    }
}
