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

        #[test]
        fn bounded_zero_one_and_as_primitive() {
            use crate::test::TestConvert;

            assert_eq!(<FTest as Bounded>::min_value().to_bits(), FTest::MIN.to_bits());
            assert_eq!(<FTest as Bounded>::max_value().to_bits(), FTest::MAX.to_bits());
            assert_eq!(<FTest as Zero>::zero().to_bits(), FTest::ZERO.to_bits());
            assert_eq!(<FTest as One>::one().to_bits(), FTest::ONE.to_bits());

            let f = FTest::from(42.5 as FTestBase);
            assert_eq!(<FTest as AsPrimitive<u8>>::as_(f), 42u8);
            assert_eq!(<FTest as AsPrimitive<u16>>::as_(f), 42u16);
            assert_eq!(<FTest as AsPrimitive<u32>>::as_(f), 42u32);
            assert_eq!(<FTest as AsPrimitive<u64>>::as_(f), 42u64);
            assert_eq!(<FTest as AsPrimitive<u128>>::as_(f), 42u128);
            assert_eq!(<FTest as AsPrimitive<usize>>::as_(f), 42usize);
            assert_eq!(<FTest as AsPrimitive<i8>>::as_(f), 42i8);
            assert_eq!(<FTest as AsPrimitive<i16>>::as_(f), 42i16);
            assert_eq!(<FTest as AsPrimitive<i32>>::as_(f), 42i32);
            assert_eq!(<FTest as AsPrimitive<i64>>::as_(f), 42i64);
            assert_eq!(<FTest as AsPrimitive<i128>>::as_(f), 42i128);
            assert_eq!(<FTest as AsPrimitive<isize>>::as_(f), 42isize);
            assert_eq!(<FTest as AsPrimitive<f32>>::as_(f), 42.5f32);
            assert_eq!(<FTest as AsPrimitive<f64>>::as_(f), 42.5f64);

            let f42 = FTest::from(42.0 as FTestBase);
            assert_eq!(<u8 as AsPrimitive<FTest>>::as_(42u8).to_bits(), f42.to_bits());
            assert_eq!(<u16 as AsPrimitive<FTest>>::as_(42u16).to_bits(), f42.to_bits());
            assert_eq!(<u32 as AsPrimitive<FTest>>::as_(42u32).to_bits(), f42.to_bits());
            assert_eq!(<u64 as AsPrimitive<FTest>>::as_(42u64).to_bits(), f42.to_bits());
            assert_eq!(<u128 as AsPrimitive<FTest>>::as_(42u128).to_bits(), f42.to_bits());
            assert_eq!(<usize as AsPrimitive<FTest>>::as_(42usize).to_bits(), f42.to_bits());
            assert_eq!(<i8 as AsPrimitive<FTest>>::as_(42i8).to_bits(), f42.to_bits());
            assert_eq!(<i16 as AsPrimitive<FTest>>::as_(42i16).to_bits(), f42.to_bits());
            assert_eq!(<i32 as AsPrimitive<FTest>>::as_(42i32).to_bits(), f42.to_bits());
            assert_eq!(<i64 as AsPrimitive<FTest>>::as_(42i64).to_bits(), f42.to_bits());
            assert_eq!(<i128 as AsPrimitive<FTest>>::as_(42i128).to_bits(), f42.to_bits());
            assert_eq!(<isize as AsPrimitive<FTest>>::as_(42isize).to_bits(), f42.to_bits());
            assert_eq!(<f32 as AsPrimitive<FTest>>::as_(42.5f32).to_bits(), f.to_bits());
            assert_eq!(<f64 as AsPrimitive<FTest>>::as_(42.5f64).to_bits(), f.to_bits());

            // Integer -> Float and Float -> Float AsPrimitive
            let u_one = UTest::ONE;
            assert_eq!(<UTest as AsPrimitive<FTest>>::as_(u_one).to_bits(), FTest::ONE.to_bits());
            let i_neg = ITest::ZERO - ITest::ONE;
            assert_eq!(<ITest as AsPrimitive<FTest>>::as_(i_neg).to_bits(), FTest::NEG_ONE.to_bits());
            let f16: crate::types::F16 = <FTest as AsPrimitive<crate::types::F16>>::as_(FTest::ONE);
            assert_eq!(f16.to_bits(), crate::types::F16::ONE.to_bits());
            assert_eq!(
                TestConvert::into(<crate::types::F16 as AsPrimitive<FTest>>::as_(f16)),
                TestConvert::into(FTest::ONE)
            );
        }
    }
}
