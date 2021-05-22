use super::Bint;
use crate::uint::BUint;
use core::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

// When const traits are stable in Rust, the trait implementations can be replaced with these

impl<const N: usize> Bint<N> {
    pub const fn eq(&self, other: &Self) -> bool {
        self.signed_digit == other.signed_digit &&
        BUint::eq(&self.uint, &other.uint)
    }
    pub const fn cmp(&self, other: &Self) -> Ordering {
        if self.signed_digit == other.signed_digit {
            BUint::<N>::cmp(&self.uint, &other.uint)
        } else {
            if self.signed_digit > other.signed_digit {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
    pub const fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> PartialEq for Bint<N> {
    fn eq(&self, other: &Self) -> bool {
        Self::eq(self, other)
    }
}

impl<const N: usize> Eq for Bint<N> {}

impl<const N: usize> PartialOrd for Bint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Bint<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inequality() {
        let a = Bint::<2>::from(-2348273479989898i128);
        let b = Bint::<2>::from(-9049873947589473745i128);
        assert!(a > b);
        assert_ne!(a, b);

        let a = Bint::<2>::from(34578394758934759478789354i128);
        let b = Bint::<2>::from(3459374957834758394759782i128);
        assert!(a > b);
        assert_ne!(a, b);

        let a = Bint::<2>::from(-34578394758934759478789354i128);
        let b = Bint::<2>::from(3459374957834758394759782i128);
        assert!(b > a);
        assert_ne!(a, b);
    }

    #[test]
    fn test_equality() {
        let a = Bint::<2>::from(-9049873947589473745i128);
        let b = Bint::<2>::from(-9049873947589473745i128);
        assert_eq!(a, b);
        
        let a = Bint::<2>::from(34578394758934759478789354i128);
        let b = Bint::<2>::from(34578394758934759478789354i128);
        assert_eq!(a, b);
    }
}