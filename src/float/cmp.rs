use super::Float;
use crate::Int;
use crate::doc;
use core::cmp::{Ordering, PartialEq, PartialOrd};

/// Comparison methods.
impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
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

    #[must_use]
    #[inline]
    pub const fn total_cmp(&self, other: &Self) -> Ordering {
        let left = self.to_signed_bits();
        let right = other.to_signed_bits();
        if left.is_negative() && right.is_negative() {
            Int::cmp(&left, &right).reverse()
        } else {
            Int::cmp(&left, &right)
        }
    }
}

impl<const W: usize, const MB: usize, const RM: u8> PartialEq for Float<W, MB, RM> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Self::eq(&self, other)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> PartialOrd for Float<W, MB, RM> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Self::partial_cmp(&self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;

    crate::test::test_all! {
        testing floats;

        // Rust's `f32::max`/`f64::max` and `min` leave the sign of zero unspecified
        // when comparing `-0.0` and `+0.0` (e.g. x86_64 `maxsd`/`minsd` vs aarch64
        // `fmaxnm`/`fminnm`). Skip mixed signed zeros here and test them explicitly below.
        test_bignum! {
            function: <FTest>::max(a: FTestBase, b: FTestBase),
            skip: a == 0.0 && b == 0.0 && a.is_sign_negative() != b.is_sign_negative()
        }
        test_bignum! {
            function: <FTest>::min(a: FTestBase, b: FTestBase),
            skip: a == 0.0 && b == 0.0 && a.is_sign_negative() != b.is_sign_negative()
        }
        // #[cfg(nightly)]
        // test_bignum! {
        //     function: <FTest>::maximum(a: FTestBase, b: FTestBase)
        // }
        // #[cfg(nightly)]
        // test_bignum! {
        //     function: <FTest>::minimum(a: FTestBase, b: FTestBase)
        // }
        test_bignum! {
            function: <FTest>::clamp(a: FTestBase, b: FTestBase, c: FTestBase),
            skip: !(b <= c)
        }
        test_bignum! {
            function: <FTest>::total_cmp(a: ref &FTestBase, b: ref &FTestBase)
        }
        test_bignum! {
            function: <FTest as PartialOrd>::partial_cmp(a: ref &FTestBase, b: ref &FTestBase)
        }
        test_bignum! {
            function: <FTest as PartialEq>::eq(a: ref &FTestBase, b: ref &FTestBase)
        }

        #[test]
        fn maximum_minimum_and_const_cmp_fillers() {
            let vals: &[FTestBase] = &[
                FTestBase::NAN,
                FTestBase::NEG_INFINITY,
                -2.5,
                -0.0,
                0.0,
                2.5,
                FTestBase::INFINITY,
            ];
            for &a in vals {
                let fa = FTest::from(a);
                for &b in vals {
                    let fb = FTest::from(b);

                    // const trait fillers ne and ge
                    assert_eq!(FTest::ne(&fa, &fb), a != b);
                    assert_eq!(FTest::ge(&fa, &fb), a >= b);

                    // IEEE 754-2019 maximum / minimum semantics (NaN propagates, +0.0 > -0.0)
                    let max_r = fa.maximum(fb);
                    let min_r = fa.minimum(fb);
                    if a.is_nan() || b.is_nan() {
                        assert!(max_r.is_nan());
                        assert!(min_r.is_nan());
                    } else if a.total_cmp(&b).is_lt() {
                        assert_eq!(max_r.to_bits(), fb.to_bits());
                        assert_eq!(min_r.to_bits(), fa.to_bits());
                    } else {
                        assert_eq!(max_r.to_bits(), fa.to_bits());
                        assert_eq!(min_r.to_bits(), fb.to_bits());
                    }
                }
            }

            // Explicit signed-zero semantics for Float::max / Float::min (+0.0 > -0.0).
            assert_eq!(FTest::ZERO.max(FTest::NEG_ZERO).to_bits(), FTest::ZERO.to_bits());
            assert_eq!(FTest::NEG_ZERO.max(FTest::ZERO).to_bits(), FTest::ZERO.to_bits());
            assert_eq!(FTest::NEG_ZERO.max(FTest::NEG_ZERO).to_bits(), FTest::NEG_ZERO.to_bits());
            assert_eq!(FTest::ZERO.min(FTest::NEG_ZERO).to_bits(), FTest::NEG_ZERO.to_bits());
            assert_eq!(FTest::NEG_ZERO.min(FTest::ZERO).to_bits(), FTest::NEG_ZERO.to_bits());
            assert_eq!(FTest::ZERO.min(FTest::ZERO).to_bits(), FTest::ZERO.to_bits());
        }
    }
}
