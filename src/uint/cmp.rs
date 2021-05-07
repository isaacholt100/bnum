use super::BUint;
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

impl<const N: usize> PartialEq for BUint<N> {
    fn eq(&self, other: &Self) -> bool {
        self.digits
            .iter()
            .zip(other.digits.iter())
            .all(|(a, b)| a == b)
    }
}

impl<const N: usize> Eq for BUint<N> {}

impl<const N: usize> PartialOrd for BUint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for BUint<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.digits.iter().zip(other.digits.iter()).rev() {
            match a.cmp(&b) {
                Ordering::Greater => {
                    return Ordering::Greater;
                },
                Ordering::Less => {
                    return Ordering::Less;
                },
                _ => {}
            }
        }
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compares_unequal_uints() {
        let a = BUint::<2>::from(3459303849058334845904u128);
        let b = BUint::<2>::from(98349593794583490573480980u128);
        assert!(a < b);
        assert!(a <= b);
    }

    #[test]
    fn it_compares_equal_uints() {
        let a = BUint::<2>::from(3459303849058334845904u128);
        let b = BUint::<2>::from(3459303849058334845904u128);
        assert!(a == b);
        assert!(a >= b);
        assert!(a <= b);
    }
}