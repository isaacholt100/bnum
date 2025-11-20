use super::Float;
use core::iter::{Iterator, Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

mod add;
#[cfg(feature = "nightly")]
mod div;
mod mul;
mod rem;
mod sub;

macro_rules! impl_assign_op {
    ($AssignTrait: ident, $assign_fn: ident, $op_fn: ident) => {
        impl<const W: usize, const MB: usize> $AssignTrait<Self> for Float<W, MB> {
            #[inline]
            fn $assign_fn(&mut self, rhs: Self) {
                *self = self.$op_fn(rhs);
            }
        }

        impl<const W: usize, const MB: usize> $AssignTrait<&Self> for Float<W, MB> {
            #[inline]
            fn $assign_fn(&mut self, rhs: &Self) {
                *self = self.$op_fn(*rhs);
            }
        }
    };
}

impl<const W: usize, const MB: usize> Add for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

crate::integer::ops::full_op_impl!(<const W: usize, const MB: usize> Add, AddAssign, Float<W, MB>, add, add_assign for Float<W, MB>);

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

impl<const W: usize, const MB: usize> Sub for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::sub(self, rhs)
    }
}

crate::integer::ops::full_op_impl!(<const W: usize, const MB: usize> Sub, SubAssign, Float<W, MB>, sub, sub_assign for Float<W, MB>);

impl<const W: usize, const MB: usize> Mul for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::mul(self, rhs)
    }
}

crate::uints::ops::full_op_impl!(<const W: usize, const MB: usize> Mul, MulAssign, Float<W, MB>, mul, mul_assign for Float<W, MB>);

impl<const W: usize, const MB: usize> Product for Float<W, MB> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const W: usize, const MB: usize> Product<&'a Self> for Float<W, MB> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

#[cfg(feature = "nightly")]
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

// crate::integer::ops::full_op_impl!(<const W: usize, const MB: usize> Div, DivAssign, Float<W, MB>, div, div_assign for Float<W, MB>);

impl<const W: usize, const MB: usize> Rem for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self::rem(self, rhs)
    }
}

crate::integer::ops::full_op_impl!(<const W: usize, const MB: usize> Rem, RemAssign, Float<W, MB>, rem, rem_assign for Float<W, MB>);

impl<const W: usize, const MB: usize> Neg for Float<W, MB> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self::neg(self)
    }
}

impl<const W: usize, const MB: usize> Neg for &Float<W, MB> {
    type Output = Float<W, MB>;

    #[inline]
    fn neg(self) -> Float<W, MB> {
        (*self).neg()
    }
}

#[cfg(test)]
crate::test::test_all_widths! {
    use crate::test::test_bignum;

    test_bignum! {
        function: <ftest as Add>::add(a: ftest, b: ftest),
        cases: [(1.3952888382785755e33, 1.466527384898436e33)]
    }
    test_bignum! {
        function: <ftest as Sub>::sub(a: ftest, b: ftest),
        cases: [(-0.0, 0.0)]
    }
    test_bignum! {
        function: <ftest as Mul>::mul(a: ftest, b: ftest),
        cases: [
            (5.6143642e23f64 as ftest, 35279.223f64 as ftest)
        ]
    }
    // test_bignum! {
    //     function: <ftest as Div>::div(a: ftest, b: ftest)
    // }
    test_bignum! {
        function: <ftest as Rem>::rem(a: ftest, b: ftest)
    }
    test_bignum! {
        function: <ftest as Neg>::neg(f: ftest)
    }
}
