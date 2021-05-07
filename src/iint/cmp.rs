use super::BIint;
use crate::sign::Sign;
use std::cmp::Ordering;

impl<const N: usize> PartialEq for BIint<N> {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.uint == other.uint
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
        match self.sign.cmp(&other.sign) {
            Ordering::Equal => {
                match self.sign {
                    Sign::Zero => Ordering::Equal,
                    Sign::Plus => self.uint.cmp(&other.uint),
                    Sign::Minus => other.uint.cmp(&self.uint),
                }
            },
            ord => ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compares_unequal_iints() {
        let a = BIint::<10>::from(9067405896904856i128);
        let b = BIint::<10>::from(3745099046045860456456i128);
        assert!(a < b);
        assert!(a <= b);
        let a = BIint::<10>::from(-3048309859043345898i128);
        let b = BIint::<10>::from(-3408505846845068008304850i128);
        assert!(a > b);
        assert!(a >= b);
        let a = BIint::<10>::from(856908459783590i128);
        let b = BIint::<10>::from(-384907458883908347834i128);
        assert!(a > b);
        assert!(a >= b);
    }

    #[test]
    fn it_compares_equal_iints() {
        let a = BIint::<10>::from(-39048590384345i64);
        let b = BIint::<10>::from(-39048590384345i64);
        assert!(a == b);
        assert!(a >= b);
        assert!(a <= b);
    }
}