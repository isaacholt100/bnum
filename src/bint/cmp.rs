use super::BInt;
use crate::buint::BUint;
use crate::nightly::impl_const;
use core::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

impl_const! {
    impl<const N: usize> const PartialEq for BInt<N> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            BUint::eq(&self.bits, &other.bits)
        }
    }
}

impl_const! {
    impl<const N: usize> const Eq for BInt<N> {}
}

impl_const! {
    impl<const N: usize> const PartialOrd for BInt<N> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

impl_const! {
    impl<const N: usize> const Ord for BInt<N> {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            let s1 = self.signed_digit();
            let s2 = other.signed_digit();

            // Don't use match here as `cmp` is not yet const for primitive integers
            #[allow(clippy::comparison_chain)]
            if s1 == s2 {
                BUint::cmp(&self.bits, &other.bits)
            } else if s1 > s2 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }

        #[inline]
        fn max(self, other: Self) -> Self {
            match self.cmp(&other) {
                Ordering::Less | Ordering::Equal => other,
                _ => self,
            }
        }

        #[inline]
        fn min(self, other: Self) -> Self {
            match self.cmp(&other) {
                Ordering::Less | Ordering::Equal => self,
                _ => other,
            }
        }

        #[inline]
        fn clamp(self, min: Self, max: Self) -> Self {
            assert!(min <= max);
            if let Ordering::Less = self.cmp(&min) {
                min
            } else if let Ordering::Greater = self.cmp(&max) {
                max
            } else {
                self
            }
        }
    }
}

crate::int::cmp::tests!(itest);
