use super::Float;
use num_traits::{Bounded, ConstZero, ConstOne, One, Zero, Signed, Euclid};

impl<const W: usize, const MB: usize> Bounded for Float<W, MB> {
    #[inline]
    fn min_value() -> Self {
        Self::MIN
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }
}

impl<const W: usize, const MB: usize> ConstZero for Float<W, MB> {
    const ZERO: Self = Self::ZERO;
}

impl<const W: usize, const MB: usize> ConstOne for Float<W, MB> {
    const ONE: Self = Self::ONE;
}

impl<const W: usize, const MB: usize> Zero for Float<W, MB> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        Self::is_zero(&self)
    }
}

impl<const W: usize, const MB: usize> One for Float<W, MB> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn is_one(&self) -> bool {
        Self::ONE.eq(&self)
    }
}

// impl<const W: usize, const MB: usize> Signed for Float<W, MB> {
//     #[inline]
//     fn is_negative(&self) -> bool {
//         Self::is_sign_negative(*self)
//     }

//     #[inline]
//     fn is_positive(&self) -> bool {
//         Self::is_sign_positive(*self)
//     }

//     #[inline]
//     fn abs(&self) -> Self {
//         Self::abs(*self)
//     }

//     #[inline]
//     fn abs_sub(&self, other: &Self) -> Self {
//         if self <= other {
//             Self::ZERO
//         } else {
//             *self - *other
//         }
//     }

//     #[inline]
//     fn signum(&self) -> Self {
//         Self::signum(*self)
//     }
// }