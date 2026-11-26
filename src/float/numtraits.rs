use super::Float;
use crate::cast::CastFrom;
use crate::Integer;
use num_traits::{AsPrimitive, Bounded, ConstOne, ConstZero, One, Zero, float::TotalOrder};

impl<const W: usize, const MB: usize, const RM: u8> Bounded for Float<W, MB, RM> {
    #[inline]
    fn min_value() -> Self {
        Self::MIN
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }
}

impl<const W: usize, const MB: usize, const RM: u8> ConstZero for Float<W, MB, RM> {
    const ZERO: Self = Self::ZERO;
}

impl<const W: usize, const MB: usize, const RM: u8> ConstOne for Float<W, MB, RM> {
    const ONE: Self = Self::ONE;
}

impl<const W: usize, const MB: usize, const RM: u8> Zero for Float<W, MB, RM> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        Self::is_zero(&self)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> One for Float<W, MB, RM> {
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
            impl<const W: usize, const MB: usize, const RM: u8> AsPrimitive<$primitive> for Float<W, MB, RM> {
                #[inline]
                fn as_(self) -> $primitive {
                    <$primitive>::cast_from(self)
                }
            }

            impl<const W: usize, const MB: usize, const RM: u8> AsPrimitive<Float<W, MB, RM>> for $primitive {
                #[inline]
                fn as_(self) -> Float<W, MB, RM> {
                    Float::cast_from(self)
                }
            }
        )*
    };
}

impl_as_primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl<const W: usize, const MB: usize, const RM: u8, const S: bool, const N: usize, const B: usize, const OM: u8>
    AsPrimitive<Float<W, MB, RM>> for Integer<S, N, B, OM>
{
    #[inline]
    fn as_(self) -> Float<W, MB, RM> {
        Float::cast_from(self)
    }
}

// impl<const W: usize, const MB: usize, const S: bool, const N: usize, const B: usize, const OM: u8>
//     AsPrimitive<Integer<S, N, B, OM>> for Float<W, MB>
// {
//     #[inline]
//     fn as_(self) -> Integer<S, N, B, OM> {
//         Integer::cast_from(self)
//     }
// }

impl<const W1: usize, const MB1: usize, const RM1: u8, const W2: usize, const MB2: usize, const RM2: u8>
    AsPrimitive<Float<W2, MB2, RM2>> for Float<W1, MB1, RM1>
{
    #[inline]
    fn as_(self) -> Float<W2, MB2, RM2> {
        Float::cast_from(self)
    }
}

impl<const W: usize, const MB: usize, const RM: u8> TotalOrder for Float<W, MB, RM> {
    #[inline]
    fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {
        Self::total_cmp(&self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use super::*;

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest as Zero>::is_zero(a: ref &FTestBase)
        }
        test_bignum! {
            function: <FTest as One>::is_one(a: ref &FTestBase)
        }
        test_bignum! {
            function: <FTest as TotalOrder>::total_cmp(a: ref &FTestBase, b: ref &FTestBase)
        }
    }
}
