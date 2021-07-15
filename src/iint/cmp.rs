use super::BIint;
use crate::uint::BUint;
use core::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

// When const traits are stable in Rust, the trait implementations can be replaced with these

impl<const N: usize> BIint<N> {
    pub const fn eq(&self, other: &Self) -> bool {
        BUint::eq(&self.uint, &other.uint)
    }
    pub const fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.signed_digit();
        let s2 = other.signed_digit();
        if s1 == s2 {
            BUint::cmp(&self.uint, &other.uint)
        } else {
            if s1 > s2 {
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

impl<const N: usize> PartialEq for BIint<N> {
    fn eq(&self, other: &Self) -> bool {
        Self::eq(self, other)
    }
}

impl<const N: usize> Eq for BIint<N> {}

impl<const N: usize> PartialOrd for BIint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for BIint<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp(self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::I128;

    #[test]
    fn inequality() {
        let a = I128::from(-2348273479989898i128);
        let b = I128::from(-9049873947589473745i128);
        assert!(a > b);
        assert_ne!(a, b);

        let a = I128::from(34578394758934759478789354i128);
        let b = I128::from(3459374957834758394759782i128);
        assert!(a > b);
        assert_ne!(a, b);

        let a = I128::from(-34578394758934759478789354i128);
        let b = I128::from(3459374957834758394759782i128);
        assert!(b > a);
        assert_ne!(a, b);
    }

    #[test]
    fn equality() {
        let a = I128::from(-9049873947589473745i128);
        let b = I128::from(-9049873947589473745i128);
        assert_eq!(a, b);
        
        let a = I128::from(34578394758934759478789354i128);
        let b = I128::from(34578394758934759478789354i128);
        assert_eq!(a, b);
    }
}