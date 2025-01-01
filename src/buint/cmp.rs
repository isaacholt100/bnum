use core::cmp::{Ord, Ordering, PartialOrd};

macro_rules! cmp {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        impl<const N: usize> PartialOrd for $BUint<N> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<const N: usize> Ord for $BUint<N> {
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
    };
}

#[cfg(test)]
crate::test::all_digit_tests! {
    crate::int::cmp::tests!(utest);
}

crate::macro_impl!(cmp);
