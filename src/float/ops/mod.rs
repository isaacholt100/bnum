use super::Float;
use core::iter::{Iterator, Product, Sum};
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

mod add;
mod sub;
mod mul;
mod div;
mod rem;

impl<const W: usize, const MB: usize> Add for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

impl<const W: usize, const MB: usize> Sub for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::sub(self, rhs)
    }
}

crate::int::ops::op_ref_impl!(Add<Float<N, MB>> for Float<N, MB>, add);

impl<const W: usize, const MB: usize> Sum for Float<W, MB> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const W: usize, const MB: usize> Sum<&'a Self> for Float<W, MB> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a + *b)
    }
}

crate::int::ops::op_ref_impl!(Sub<Float<N, MB>> for Float<N, MB>, sub);

impl<const W: usize, const MB: usize> Mul for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::mul(self, rhs)
    }
}

impl<const W: usize, const MB: usize> Product for Float<W, MB>
where
    [(); W * 2]:,
{
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const W: usize, const MB: usize> Product<&'a Self> for Float<W, MB>
where
    [(); W * 2]:,
{
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

impl<const W: usize, const MB: usize> Div for Float<W, MB>
where
    [(); W * 2]:,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

// crate::int::ops::op_ref_impl!(Div<Float<N, MB>> for Float<N, MB>, div);

impl<const W: usize, const MB: usize> Rem for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

crate::int::ops::op_ref_impl!(Rem<Float<N, MB>> for Float<N, MB>, rem);

crate::nightly::impl_const! {
    impl<const W: usize, const MB: usize> const Neg for Float<W, MB> {
        type Output = Self;

        #[inline]
        fn neg(self) -> Self {
            Self::neg(self)
        }
    }
}

crate::nightly::impl_const! {
    impl<const W: usize, const MB: usize> const Neg for &Float<W, MB> {
        type Output = Float<W, MB>;

        #[inline]
        fn neg(self) -> Float<W, MB> {
            (*self).neg()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest as Add>::add(a: ftest, b: ftest)
    }
    test_bignum! {
        function: <ftest as Sub>::sub(a: ftest, b: ftest)
    }
    test_bignum! {
        function: <ftest as Mul>::mul(a: ftest, b: ftest),
        cases: [
            (5.6143642e23f64 as ftest, 35279.223f64 as ftest)
        ]
    }
    test_bignum! {
        function: <ftest as Div>::div(a: ftest, b: ftest)
    }
    test_bignum! {
        function: <ftest as Rem>::rem(a: ftest, b: ftest)
    }
    test_bignum! {
        function: <ftest as Neg>::neg(f: ftest)
    }
}
