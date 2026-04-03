use crate::Integer;
use core::cmp::{Ord, Ordering, PartialOrd};

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
            function: <STest as PartialEq>::eq(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as PartialOrd>::partial_cmp(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as PartialOrd>::lt(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as PartialOrd>::le(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as PartialOrd>::gt(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as PartialOrd>::ge(a: ref &STest, b: ref &STest)
        }

        test_bignum! {
            function: <STest>::cmp(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest>::max(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest>::min(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest>::clamp(a: STest, min: STest, max: STest),
            skip: min > max
        }

        test_bignum! {
            function: <STest as Ord>::cmp(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as Ord>::max(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Ord>::min(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Ord>::clamp(a: STest, min: STest, max: STest),
            skip: min > max
        }
    }
}

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <UTest>::eq(a: ref &UTest, b: ref &UTest)
    }
    test_bignum! {
        function: <ITest>::eq(a: ref &ITest, b: ref &ITest)
    }
    test_bignum! {
        function: <UTest>::cmp(a: ref &UTest, b: ref &UTest)
    }
}
