use super::BUint;
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

// When const traits are stable in Rust, the trait implementations can be replaced with these

impl<const N: usize> BUint<N> {
    pub const fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < N {
            if self.digits[i] != other.digits[i] {
                return false;
            }
            i += 1;
        }
        true
    }
    pub const fn cmp(&self, other: &Self) -> Ordering {
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
    pub const fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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