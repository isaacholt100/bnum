use super::BUint;
use core::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

impl<const N: usize> const PartialEq for BUint<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < N {
            if self.digits[i] != other.digits[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        !(self.eq(other))
    }
}

impl<const N: usize> Eq for BUint<N> {}

impl<const N: usize> const PartialOrd for BUint<N> {
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

impl<const N: usize> const Ord for BUint<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let mut i = N;
        while i > 0 {
            i -= 1;
            let a = self.digits[i];
            let b = other.digits[i];
            if a > b {
                return Ordering::Greater;
            } else if a < b {
                return Ordering::Less;
            }
        }
        Ordering::Equal
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
    use crate::U128;

    #[test]
    fn it_compares_unequal_uints() {
        let a = U128::from(3459303849058334845904u128);
        let b = U128::from(98349593794583490573480980u128);
        assert!(a < b);
        assert!(a <= b);
    }

    #[test]
    fn it_compares_equal_uints() {
        let a = U128::from(3459303849058334845904u128);
        let b = U128::from(3459303849058334845904u128);
        assert!(a == b);
        assert!(a >= b);
        assert!(a <= b);
    }
}