macro_rules! impl_as_primitive_big_num_for_primitive {
    ($Int: ident; $($ty: ty), *) => {
        $(
            impl<const N: usize> AsPrimitive<$ty> for $Int<N> {
                #[inline]
                fn as_(self) -> $ty {
                    <$ty>::cast_from(self)
                }
            }
        )*
    }
}

pub(crate) use impl_as_primitive_big_num_for_primitive;

macro_rules! num_trait_impl {
    ($Int: ident, $tr: ident, $method: ident, $ret: ty) => {
        impl<const N: usize> $tr for $Int<N> {
            #[inline]
            fn $method(&self, rhs: &Self) -> $ret {
                Self::$method(*self, *rhs)
            }
        }
    };
}

pub(crate) use num_trait_impl;

macro_rules! as_bigint_impl {
    ([$($ty: ty), *] as $Big: ident) => {
        $(
            impl<const N: usize> AsPrimitive<$Big<N>> for $ty {
                #[inline]
                fn as_(self) -> $Big<N> {
                    $Big::cast_from(self)
                }
            }
        )*
    }
}

pub(crate) use as_bigint_impl;

macro_rules! to_primitive {
    ($primitive: ty, $method: ident) => {
        #[inline]
        fn $method(&self) -> Option<$primitive> {
            (*self).try_into().ok()
        }
    };
}

pub(crate) use to_primitive;

macro_rules! impls {
    ($Int: ident) => {
        use num_traits::ops::overflowing::{OverflowingAdd, OverflowingSub, OverflowingMul};
        use num_traits::{
            AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedEuclid, CheckedMul, CheckedNeg,
            CheckedRem, CheckedShl, CheckedShr, CheckedSub, ConstOne, ConstZero, Euclid, FromBytes,
            FromPrimitive, MulAdd, MulAddAssign, Num, One, Pow, PrimInt, Saturating, SaturatingAdd,
            SaturatingMul, SaturatingSub, ToBytes, ToPrimitive, WrappingAdd, WrappingMul,
            WrappingNeg, WrappingShl, WrappingShr, WrappingSub, Zero,
        };

        impl<const N: usize> Bounded for $Int<N> {
            #[inline]
            fn min_value() -> Self {
                Self::MIN
            }

            #[inline]
            fn max_value() -> Self {
                Self::MAX
            }
        }

        use crate::ints::numtraits::num_trait_impl;

        num_trait_impl!($Int, CheckedAdd, checked_add, Option<Self>);
        num_trait_impl!($Int, CheckedDiv, checked_div, Option<Self>);
        num_trait_impl!($Int, CheckedMul, checked_mul, Option<Self>);
        num_trait_impl!($Int, CheckedRem, checked_rem, Option<Self>);
        num_trait_impl!($Int, CheckedSub, checked_sub, Option<Self>);

        num_trait_impl!($Int, SaturatingAdd, saturating_add, Self);
        num_trait_impl!($Int, SaturatingMul, saturating_mul, Self);
        num_trait_impl!($Int, SaturatingSub, saturating_sub, Self);

        num_trait_impl!($Int, WrappingAdd, wrapping_add, Self);
        num_trait_impl!($Int, WrappingMul, wrapping_mul, Self);
        num_trait_impl!($Int, WrappingSub, wrapping_sub, Self);

        num_trait_impl!($Int, OverflowingAdd, overflowing_add, (Self, bool));
        num_trait_impl!($Int, OverflowingSub, overflowing_sub, (Self, bool));
        num_trait_impl!($Int, OverflowingMul, overflowing_mul, (Self, bool));

        impl<const N: usize> CheckedNeg for $Int<N> {
            #[inline]
            fn checked_neg(&self) -> Option<Self> {
                Self::checked_neg(*self)
            }
        }

        impl<const N: usize> CheckedShl for $Int<N> {
            #[inline]
            fn checked_shl(&self, rhs: u32) -> Option<Self> {
                Self::checked_shl(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> CheckedShr for $Int<N> {
            #[inline]
            fn checked_shr(&self, rhs: u32) -> Option<Self> {
                Self::checked_shr(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> CheckedEuclid for $Int<N> {
            #[inline]
            fn checked_div_euclid(&self, rhs: &Self) -> Option<Self> {
                Self::checked_div_euclid(*self, *rhs)
            }

            #[inline]
            fn checked_rem_euclid(&self, rhs: &Self) -> Option<Self> {
                Self::checked_rem_euclid(*self, *rhs)
            }
        }

        impl<const N: usize> Euclid for $Int<N> {
            #[inline]
            fn div_euclid(&self, rhs: &Self) -> Self {
                Self::div_euclid(*self, *rhs)
            }

            #[inline]
            fn rem_euclid(&self, rhs: &Self) -> Self {
                Self::rem_euclid(*self, *rhs)
            }
        }

        impl<const N: usize> WrappingNeg for $Int<N> {
            #[inline]
            fn wrapping_neg(&self) -> Self {
                Self::wrapping_neg(*self)
            }
        }

        impl<const N: usize> WrappingShl for $Int<N> {
            #[inline]
            fn wrapping_shl(&self, rhs: u32) -> Self {
                Self::wrapping_shl(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> WrappingShr for $Int<N> {
            #[inline]
            fn wrapping_shr(&self, rhs: u32) -> Self {
                Self::wrapping_shr(*self, rhs as ExpType)
            }
        }

        impl<const N: usize> Pow<ExpType> for $Int<N> {
            type Output = Self;

            #[inline]
            fn pow(self, exp: ExpType) -> Self {
                Self::pow(self, exp)
            }
        }

        impl<const N: usize> Saturating for $Int<N> {
            #[inline]
            fn saturating_add(self, rhs: Self) -> Self {
                Self::saturating_add(self, rhs)
            }

            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self {
                Self::saturating_sub(self, rhs)
            }
        }

        use crate::ints::numtraits::to_primitive;

        impl<const N: usize> ToPrimitive for $Int<N> {
            to_primitive!(u8, to_u8);
            to_primitive!(u16, to_u16);
            to_primitive!(u32, to_u32);
            to_primitive!(u64, to_u64);
            to_primitive!(u128, to_u128);
            to_primitive!(usize, to_usize);
            to_primitive!(i8, to_i8);
            to_primitive!(i16, to_i16);
            to_primitive!(i32, to_i32);
            to_primitive!(i64, to_i64);
            to_primitive!(i128, to_i128);
            to_primitive!(isize, to_isize);

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                Some(self.as_())
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                Some(self.as_())
            }
        }

        crate::ints::numtraits::impl_as_primitive_big_num_for_primitive!($Int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        crate::ints::numtraits::as_bigint_impl!([u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128, char, bool, f32, f64] as $Int);

        impl<const N: usize, const M: usize> AsPrimitive<crate::Uint<M>> for $Int<N> {
            #[inline]
            fn as_(self) -> crate::Uint<M> {
                crate::Uint::<M>::cast_from(self)
            }
        }

        #[cfg(feature = "signed")]
        impl<const N: usize, const M: usize> AsPrimitive<crate::Int<M>> for $Int<N> {
            #[inline]
            fn as_(self) -> crate::Int<M> {
                crate::Int::<M>::cast_from(self)
            }
        }

        impl<const N: usize> FromBytes for $Int<N> {
            type Bytes = [u8; N];

            #[inline]
            fn from_be_bytes(bytes: &[u8; N]) -> Self {
                Self::from_be_bytes(*bytes)
            }

            #[inline]
            fn from_le_bytes(bytes: &[u8; N]) -> Self {
                Self::from_le_bytes(*bytes)
            }

            #[inline]
            fn from_ne_bytes(bytes: &[u8; N]) -> Self {
                Self::from_ne_bytes(*bytes)
            }
        }

        impl<const N: usize> ToBytes for $Int<N> {
            type Bytes = [u8; N];

            #[inline]
            fn to_be_bytes(&self) -> [u8; N] {
                Self::to_be_bytes(*self)
            }

            #[inline]
            fn to_le_bytes(&self) -> [u8; N] {
                Self::to_le_bytes(*self)
            }

            #[inline]
            fn to_ne_bytes(&self) -> [u8; N] {
                Self::to_ne_bytes(*self)
            }
        }

        impl<const N: usize> MulAdd for $Int<N> {
            type Output = Self;

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                (self * a) + b
            }
        }

        impl<const N: usize> MulAddAssign for $Int<N> {
            #[inline]
            fn mul_add_assign(&mut self, a: Self, b: Self) {
                *self = self.mul_add(a, b);
            }
        }

        impl<const N: usize> Num for $Int<N> {
            type FromStrRadixErr = crate::errors::ParseIntError;

            #[inline]
            fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                Self::from_str_radix(string, radix)
            }
        }

        impl<const N: usize> num_traits::NumCast for $Int<N> {
            fn from<T: ToPrimitive>(_n: T) -> Option<Self> {
                panic!(concat!(crate::errors::err_prefix!(), "`num_traits::NumCast` trait is not supported for ", stringify!($Int)))
            }
        }

        impl<const N: usize> One for $Int<N> {
            #[inline]
            fn one() -> Self {
                Self::ONE
            }

            #[inline]
            fn is_one(&self) -> bool {
                Self::is_one(&self)
            }
        }

        impl<const N: usize> ConstOne for $Int<N> {
            const ONE: Self = Self::ONE;
        }

        impl<const N: usize> Zero for $Int<N> {
            #[inline]
            fn zero() -> Self {
                Self::ZERO
            }

            #[inline]
            fn is_zero(&self) -> bool {
                Self::is_zero(&self)
            }
        }

        impl<const N: usize> ConstZero for $Int<N> {
            const ZERO: Self = Self::ZERO;
        }
    }
}

pub(crate) use impls;

macro_rules! prim_int_method {
    { $(fn $name: ident ($($arg: ident $(: $ty: ty)?), *) -> $ret: ty;) * } => {
        $(
            #[inline]
            fn $name($($arg $(: $ty)?), *) -> $ret {
                Self::$name($($arg), *)
            }
        )*
    };
}

pub(crate) use prim_int_method;

macro_rules! prim_int_methods {
    () => {
        crate::ints::numtraits::prim_int_method! {
            fn count_ones(self) -> u32;
            fn count_zeros(self) -> u32;
            fn leading_zeros(self) -> u32;
            fn trailing_zeros(self) -> u32;
            fn rotate_left(self, n: u32) -> Self;
            fn rotate_right(self, n: u32) -> Self;
            fn swap_bytes(self) -> Self;
            fn from_be(x: Self) -> Self;
            fn from_le(x: Self) -> Self;
            fn to_be(self) -> Self;
            fn to_le(self) -> Self;
            fn pow(self, exp: u32) -> Self;
            fn leading_ones(self) -> u32;
            fn trailing_ones(self) -> u32;
            fn reverse_bits(self) -> Self;
        }
    };
}

pub(crate) use prim_int_methods;

#[cfg(test)]
macro_rules! test_to_primitive {
    ($int: ty; $($prim: ty), *) => {
        paste::paste! {
            $(
                test_bignum! {
                    function: <$int>::[<to_ $prim>](u: ref &$int)
                }
            )*
        }
    };
}

#[cfg(test)]
pub(crate) use test_to_primitive;

#[cfg(test)]
macro_rules! test_from_primitive {
    ($int: ty; $($prim: ty), *) => {
        paste::paste! {
            $(
                test_bignum! {
                    function: <$int>::[<from_ $prim>](u: $prim),
                    cases: [
                        (<$int>::MIN as $prim)
                    ]
                }
            )*
        }
    };
}

#[cfg(test)]
pub(crate) use test_from_primitive;

#[cfg(test)]
macro_rules! tests {
    ($int: ty) => {
        use num_traits::PrimInt;
        use crate::test::{test_bignum, TestConvert};

        crate::test::test_into! {
            function: <$int as AsPrimitive>::as_,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64)
        }

        test_bignum! {
            function: <$int as CheckedAdd>::checked_add(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as CheckedSub>::checked_sub(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as CheckedMul>::checked_mul(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as CheckedDiv>::checked_div(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as CheckedRem>::checked_rem(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as CheckedNeg>::checked_neg(a: ref &$int)
        }
        test_bignum! {
            function: <$int as CheckedShl>::checked_shl(a: ref &$int, b: u8)
        }
        test_bignum! {
            function: <$int as CheckedShr>::checked_shr(a: ref &$int, b: u8)
        }
        test_bignum! {
            function: <$int as CheckedEuclid>::checked_div_euclid(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as CheckedEuclid>::checked_rem_euclid(a: ref &$int, b: ref &$int)
        }

        test_bignum! {
            function: <$int as Euclid>::div_euclid(a: ref &$int, b: ref &$int),
            skip: a.checked_div_euclid(b).is_none()
        }
        test_bignum! {
            function: <$int as Euclid>::rem_euclid(a: ref &$int, b: ref &$int),
            skip: a.checked_rem_euclid(b).is_none()
        }

        test_bignum! {
            function: <$int as SaturatingAdd>::saturating_add(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as SaturatingSub>::saturating_sub(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as SaturatingMul>::saturating_mul(a: ref &$int, b: ref &$int)
        }

        test_bignum! {
            function: <$int as Saturating>::saturating_add(a: $int, b: $int)
        }
        test_bignum! {
            function: <$int as Saturating>::saturating_sub(a: $int, b: $int)
        }

        test_bignum! {
            function: <$int as WrappingAdd>::wrapping_add(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as OverflowingAdd>::overflowing_add(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as WrappingSub>::wrapping_sub(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as OverflowingSub>::overflowing_sub(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as WrappingMul>::wrapping_mul(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as OverflowingMul>::overflowing_mul(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as WrappingNeg>::wrapping_neg(a: ref &$int)
        }
        test_bignum! {
            function: <$int as WrappingShl>::wrapping_shl(a: ref &$int, b: u16)
        }
        test_bignum! {
            function: <$int as WrappingShr>::wrapping_shr(a: ref &$int, b: u16)
        }

        test_bignum! {
            function: <$int as One>::is_one(a: ref &$int)
        }
        test_bignum! {
            function: <$int as Zero>::is_zero(a: ref &$int)
        }

        test_bignum! {
            function: <$int as MulAdd>::mul_add(a: $int, b: $int, c: $int),
            skip: a.checked_mul(b).map(|d| d.checked_add(c)).flatten().is_none()
        }

        paste::paste! {
            #[test]
            fn one() {
                assert_eq!([<$int:upper>]::ONE, TestConvert::into([<$int:upper>]::one()));
            }

            #[test]
            fn zero() {
                assert_eq!([<$int:upper>]::ZERO, TestConvert::into([<$int:upper>]::zero()));
            }

            #[test]
            fn min_value() {
                assert_eq!([<$int:upper>]::MIN, TestConvert::into([<$int:upper>]::min_value()));
            }

            #[test]
            fn max_value() {
                assert_eq!([<$int:upper>]::MAX, TestConvert::into([<$int:upper>]::max_value()));
            }
        }

        test_bignum! {
            function: <$int>::sqrt(a: ref &$int),
            skip: {
                #[allow(unused_comparisons)]
                let cond = a < 0;

                cond
            }
        }
        test_bignum! {
            function: <$int>::cbrt(a: ref &$int)
        }
        test_bignum! {
            function: <$int>::nth_root(a: ref &$int, n: u32),
            skip: n == 0 || {
                #[allow(unused_comparisons)]
                let cond = a < 0;

                n & 1 == 0 && cond || (n == 1 && cond && a == <$int>::MIN) // second condition is due to an error in the num_integer crate, which incorrectly panics when calculating the first root of i128::MIN
            }
        }

        use crate::ints::numtraits::{test_to_primitive, test_from_primitive};

        test_to_primitive!($int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        test_from_primitive!($int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        test_bignum! {
            function: <$int as Integer>::div_floor(a: ref &$int, b: ref &$int),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <$int as Integer>::mod_floor(a: ref &$int, b: ref &$int),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <$int as Integer>::lcm(a: ref &$int, b: ref &$int),
            skip: {
                #[allow(unused_comparisons)]
                let cond = a.checked_mul(b).is_none() || (a < 0 && a == <$int>::MIN) || (b < 0 && b == <$int>::MIN); // lcm(a, b) <= a * b
                cond
            },
            cases: [(ref &(1 as $int), ref &(-1i8 as $int))]
        }
        test_bignum! {
            function: <$int as Integer>::gcd(a: ref &$int, b: ref &$int),
            skip: {
                #[allow(unused_comparisons)]
                let cond = <$int>::MIN < 0 && (a == <$int>::MIN && (b == <$int>::MIN || b == 0)) || (b == <$int>::MIN && (a == <$int>::MIN || a == 0));
                cond
            }
        }
        test_bignum! {
            function: <$int as Integer>::is_multiple_of(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as Integer>::is_even(a: ref &$int)
        }
        test_bignum! {
            function: <$int as Integer>::is_odd(a: ref &$int)
        }
        test_bignum! {
            function: <$int as Integer>::div_rem(a: ref &$int, b: ref &$int),
            skip: b.is_zero()
        }

        test_bignum! {
            function: <$int as PrimInt>::unsigned_shl(a: $int, n: u8),
            skip: n >= <$int>::BITS as u8
        }
        test_bignum! {
            function: <$int as PrimInt>::unsigned_shr(a: $int, n: u8),
            skip: n >= <$int>::BITS as u8
        }
        test_bignum! {
            function: <$int as PrimInt>::signed_shl(a: $int, n: u8),
            skip: n >= <$int>::BITS as u8
        }
        test_bignum! {
            function: <$int as PrimInt>::signed_shr(a: $int, n: u8),
            skip: n >= <$int>::BITS as u8
        }
    };
}

#[cfg(test)]
pub(crate) use tests;
