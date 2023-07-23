use super::Float;
use crate::{BIntD8, BUintD8};
use core::cmp::{Ordering, PartialEq, PartialOrd};

impl<const W: usize, const MB: usize> Float<W, MB> {
    crate::nightly::const_fns! {
        #[inline]
        pub const fn max(self, other: Self) -> Self {
            handle_nan!(other; self);
            handle_nan!(self; other);
            if let Ordering::Less = self.total_cmp(&other) {
                other
            } else {
                self
            }
        }

        #[inline]
        pub const fn min(self, other: Self) -> Self {
            handle_nan!(other; self);
            handle_nan!(self; other);
            if let Ordering::Greater = self.total_cmp(&other) {
                other
            } else {
                self
            }
        }
    }

    #[inline]
    pub const fn maximum(self, other: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(other; other);
        if let Ordering::Less = self.total_cmp(&other) {
            other
        } else {
            self
        }
    }

    #[inline]
    pub const fn minimum(self, other: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(other; other);
        if let Ordering::Greater = self.total_cmp(&other) {
            other
        } else {
            self
        }
    }

    //crate::nightly::const_fns! {
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }
    //}

    #[inline]
    pub const fn total_cmp(&self, other: &Self) -> Ordering {
        let left = self.to_int();
        let right = other.to_int();
        if left.is_negative() && right.is_negative() {
            BIntD8::cmp(&left, &right).reverse()
        } else {
            BIntD8::cmp(&left, &right)
        }
    }
}

crate::nightly::impl_const! {
    impl<const W: usize, const MB: usize> const PartialEq for Float<W, MB> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            handle_nan!(false; self, other);
            (self.is_zero() && other.is_zero()) || BUintD8::eq(&self.to_bits(), &other.to_bits())
        }
    }
}

crate::nightly::impl_const! {
    impl<const W: usize, const MB: usize> const PartialOrd for Float<W, MB> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            handle_nan!(None; self, other);
            if self.is_zero() && other.is_zero() {
                return Some(Ordering::Equal);
            }
            Some(self.total_cmp(other))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::max(a: ftest, b: ftest),
        cases: [(0.0, -0.0), (-0.0, 0.0)]
    }
    test_bignum! {
        function: <ftest>::min(a: ftest, b: ftest),
        cases: [(0.0, -0.0), (-0.0, 0.0)]
    }
    test_bignum! {
        function: <ftest>::maximum(a: ftest, b: ftest),
        cases: [(0.0, -0.0), (-0.0, 0.0)]
    }
    test_bignum! {
        function: <ftest>::minimum(a: ftest, b: ftest),
        cases: [(0.0, -0.0), (-0.0, 0.0)]
    }
    test_bignum! {
        function: <ftest>::clamp(a: ftest, b: ftest, c: ftest),
        skip: !(b <= c)
    }

    test_bignum! {
        function: <ftest>::total_cmp(a: ref &ftest, b: ref &ftest)
    }
    test_bignum! {
        function: <ftest>::partial_cmp(a: ref &ftest, b: ref &ftest)
    }
    test_bignum! {
        function: <ftest>::eq(a: ref &ftest, b: ref &ftest)
    }
}
