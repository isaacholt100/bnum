use super::Float;
use crate::doc;

mod sqrt;

/*
All functions:
mul_add, div_euclid, rem_euclid, powi, powf, exp, exp2, ln, log, log2, log10, cbrt, hypot, sin, cos, tan, asin, acos, atan, atan2, sin_cos, exp_m1, ln_1p, sinh, cosh, tanh, asinh, acosh, atanh, to_degrees, to_radians
*/

/*
TODO: acos, acosh, asin, asinh, atan, atan2, atanh, cbrt, cos, cosh, exp, exp2, exp_m1, gamma, hypot, ln, ln_1p, ln_gamma, log, log10, log2, midpoint, mul_add, powf, recip, round_ties_even, tan, tanh, to_degrees, to_radians,
*/

#[doc = doc::math::impl_desc!()]
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[doc = doc::math::abs!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn abs(self) -> Self {
        if self.is_sign_negative() {
            self.neg()
        } else {
            self
        }
    }

    #[doc = doc::math::sqrt!(F)]
    #[must_use = doc::must_use_op!(float)]
    pub fn sqrt(self) -> Self {
        self.sqrt_internal()
    }

    #[doc = doc::math::div_euclid!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub fn div_euclid(self, rhs: Self) -> Self
    where
        [(); W * 2]:,
    {
        let div = (self / rhs).trunc();
        if self % rhs < Self::ZERO {
            return if rhs > Self::ZERO {
                div - Self::ONE
            } else {
                div + Self::ONE
            };
        }
        div
    }

    #[doc = doc::math::rem_euclid!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        let rem = self % rhs;
        if rem < Self::NEG_ZERO {
            rem + rhs.abs()
        } else {
            rem
        }
    }

    #[doc = doc::math::powi!(F)]
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub fn powi(mut self, n: i32) -> Self
    where
        [(); W * 2]:,
    {
        if n == 0 {
            return Self::ONE;
        }
        let mut n_abs = n.unsigned_abs(); // unsigned abs since otherwise overflow could occur (if n == i32::MIN)
        let mut y = Self::ONE;
        while n_abs > 1 {
            if n_abs & 1 == 1 {
                // out = out * self;
                y = y * self;
            }
            self = self * self;
            n_abs >>= 1;
        }
        if n.is_negative() {
            Self::ONE / (self * y)
        } else {
            self * y
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::abs(f: ftest)
    }
    test_bignum! {
        function: <ftest>::sqrt(f: ftest)
    }
    test_bignum! {
        function: <ftest>::div_euclid(f1: ftest, f2: ftest)
    }
    test_bignum! {
        function: <ftest>::rem_euclid(f1: ftest, f2: ftest)
    }
    test_bignum! {
        function: <ftest>::powi(f: ftest, n: i32)
    }
}