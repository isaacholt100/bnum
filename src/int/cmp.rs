use super::Bint;
use crate::uint::BUint;
use core::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

impl<const N: usize> const PartialEq for Bint<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        BUint::eq(&self.bits, &other.bits)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        !(self.eq(other))
    }
}

impl<const N: usize> Eq for Bint<N> {}

impl<const N: usize> const PartialOrd for Bint<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Less => true,
            _ => false,
        }
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Greater => true,
            _ => false,
        }
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Less | Ordering::Equal => true,
            _ => false,
        }
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Greater | Ordering::Equal => true,
            _ => false,
        }
    }
}

impl<const N: usize> const Ord for Bint<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.signed_digit();
        let s2 = other.signed_digit();
        if s1 == s2 {
            BUint::cmp(&self.bits, &other.bits)
        } else {
            if s1 > s2 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
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
        if let Ordering::Less = self.cmp(&min) {
            min
        } else if let Ordering::Greater = self.cmp(&max) {
            max
        } else {
            self
        }
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