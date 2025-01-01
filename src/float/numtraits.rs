use super::Float;
use num_traits::{Bounded, ConstZero, ConstOne, One, Zero, AsPrimitive, float::TotalOrder};
use crate::cast::CastFrom;

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

macro_rules! impl_as_primitive {
    ($($primitive: ty), *) => {
        $(
            impl<const W: usize, const MB: usize> AsPrimitive<$primitive> for Float<W, MB> {
                #[inline]
                fn as_(self) -> $primitive {
                    <$primitive>::cast_from(self)
                }
            }

            impl<const W: usize, const MB: usize> AsPrimitive<Float<W, MB>> for $primitive {
                #[inline]
                fn as_(self) -> Float<W, MB> {
                    Float::cast_from(self)
                }
            }
        )*
    };
}

impl_as_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl<const W: usize, const MB: usize> TotalOrder for Float<W, MB> {
    #[inline]
    fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {
        Self::total_cmp(&self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};
    use super::*;

    test_bignum! {
        function: <ftest as Zero>::is_zero(a: ref &ftest)
    }
    test_bignum! {
        function: <ftest as One>::is_one(a: ref &ftest)
    }
    test_bignum! {
        function: <ftest as TotalOrder>::total_cmp(a: ref &ftest, b: ref &ftest)
    }
}