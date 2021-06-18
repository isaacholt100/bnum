use super::Float;
use num_traits::{Float as FloatTrait};
use core::num::FpCategory;

/*impl<const N: usize> FloatTrait for Float<N> {
    fn nan() -> Self {
        Self::NAN
    }
    fn infinity() -> Self {
        Self::POS_INFINITY
    }
    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }
    fn neg_zero() -> Self {
        Self::NEG_ZERO
    }
    fn min_value() -> Self {
        Self::MIN
    }
    fn min_positive_value() -> Self {
        Self::MIN_POSITIVE
    }
    fn max_value() -> Self {
        Self::MAX
    }
    fn is_nan(self) -> bool {
        Self::is_nan(&self)
    }
    fn is_infinite(self) -> bool {
        Self::is_infinite(&self)
    }
    fn is_finite(self) -> bool {
        Self::is_finite(&self)
    }
    fn is_normal(self) -> bool {
        Self::is_finite(&self) && self.is_zero()
    }
    fn classify(self) -> FpCategory {
        Self::classify(&self)
    }
    fn floor(self) -> Self {
        todo!()
    }
    fn ceil(self) -> Self {
        todo!()
    }

}*/