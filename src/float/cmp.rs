use super::Float;
use crate::BIntD8;
use crate::doc;
use core::cmp::{Ordering, PartialEq, PartialOrd};

#[doc = doc::cmp::impl_desc!()]
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[doc = doc::cmp::max!(F)]
    #[must_use = doc::must_use_op!(comparison)]
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

    #[doc = doc::cmp::min!(F)]
    #[must_use = doc::must_use_op!(comparison)]
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

    #[doc = doc::cmp::maximum!(F)]
    #[must_use = doc::must_use_op!(comparison)]
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

    #[doc = doc::cmp::minimum!(F)]
    #[must_use = doc::must_use_op!(comparison)]
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

    #[doc = doc::cmp::clamp!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.le(&max));
        let mut x = self;
        if x.lt(&min) {
            x = min;
        }
        if x.gt(&max) {
            x = max;
        }
        x
    }
    
    #[doc = doc::cmp::total_cmp!(F)]
    #[must_use]
    #[inline]
    pub const fn total_cmp(&self, other: &Self) -> Ordering {
        let left = self.to_signed_bits();
        let right = other.to_signed_bits();
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
            Self::eq(&self, other)
        }
    }
}

crate::nightly::impl_const! {
    impl<const W: usize, const MB: usize> const PartialOrd for Float<W, MB> {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Self::partial_cmp(&self, other)
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
