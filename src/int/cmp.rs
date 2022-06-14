use super::Bint;
use crate::uint::BUint;
use core::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};

impl<const N: usize> const PartialEq for Bint<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        BUint::eq(&self.bits, &other.bits)
    }
}

impl<const N: usize> Eq for Bint<N> {}

impl<const N: usize> const PartialOrd for Bint<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> const Ord for Bint<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.signed_digit();
        let s2 = other.signed_digit();
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

#[cfg(test)]
mod tests {
	use crate::test::test_bignum;

    test_bignum! {
        function: <i128>::eq(a: ref &i128, b: ref &i128)
    }
    test_bignum! {
        function: <i128>::partial_cmp(a: ref &i128, b: ref &i128)
    }

    test_bignum! {
        function: <i128>::cmp(a: ref &i128, b: ref &i128)
    }
    test_bignum! {
        function: <i128>::max(a: i128, b: i128)
    }
    test_bignum! {
        function: <i128>::min(a: i128, b: i128)
    }
    test_bignum! {
        function: <i128>::clamp(a: i128, min: i128, max: i128),
        skip: min > max
    }
}