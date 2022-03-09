use super::Float;
use crate::digit::Digit;
use core::num::FpCategory;

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const fn is_sign_positive(self) -> bool {
        !self.is_sign_negative()
    }
    pub const fn is_sign_negative(self) -> bool {
        self.to_int().is_negative()
    }
    pub const fn is_finite(self) -> bool {
        (self.to_bits().wrapping_shl(1).leading_ones() as usize) < Self::EXPONENT_BITS
    }
    pub const fn is_infinite(self) -> bool {
        let bits = self.to_bits();
        bits == Self::INFINITY.to_bits() || bits == Self::NEG_INFINITY.to_bits()
    }
    pub const fn is_nan(self) -> bool {
        !(self.mantissa().is_zero() || self.is_finite())
    }
    pub const fn is_subnormal(self) -> bool {
        !self.is_zero() && self.exponent().is_zero()
    }
    pub const fn is_normal(self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < W - 1 {
            if self.words()[i] != 0 {
                return false;
            }
            i += 1;
        }
        let last = self.words()[W - 1];
        last.trailing_zeros() >= Digit::BITS - 1
    }
    pub const fn classify(self) -> FpCategory {
        // TODO: optimise this method better
        if self.is_finite() {
            if self.exponent().is_zero() {
                if self.is_zero() {
                    FpCategory::Zero
                } else {
                    FpCategory::Subnormal
                }
            } else {
                FpCategory::Normal
            }
        } else {
            if self.is_nan() {
                FpCategory::Nan
            } else {
                FpCategory::Infinite
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::F64;

    test_float! {
        function: is_sign_positive(a: f64)
    }
    test_float! {
        function: is_sign_negative(a: f64)
    }
    test_float! {
        function: is_finite(a: f64)
    }
    test_float! {
        function: is_infinite(a: f64)
    }
    test_float! {
        function: is_nan(a: f64)
    }
    test_float! {
        function: is_subnormal(a: f64)
    }
    test_float! {
        function: is_normal(a: f64)
    }
    test_float! {
        function: classify(a: f64)
    }
    #[test]
    fn is_zero() {
        let z1 = F64::ZERO;
        let z2 = F64::NEG_ZERO;
        assert!(z1.is_zero());
        assert!(z2.is_zero());
        assert!(!F64::ONE.is_zero());
    }
}