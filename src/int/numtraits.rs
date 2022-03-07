use super::Bint;
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, SaturatingAdd, SaturatingMul, SaturatingSub, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, ToPrimitive, Signed, Zero, Pow, Saturating, AsPrimitive};
use num_integer::{Integer, Roots};
use core::convert::TryFrom;
use crate::ExpType;

impl<const N: usize> Bounded for Bint<N> {
    fn min_value() -> Self {
        Self::MIN
    }
    fn max_value() -> Self {
        Self::MAX
    }
}

macro_rules! num_trait_impl {
    ($tr: ident, $method: ident, $ret: ty) => {
        impl<const N: usize> $tr for Bint<N> {
            fn $method(&self, rhs: &Self) -> $ret {
                Self::$method(*self, *rhs)
            }
        }
    }
}

num_trait_impl!(CheckedAdd, checked_add, Option<Self>);
num_trait_impl!(CheckedDiv, checked_div, Option<Self>);
num_trait_impl!(CheckedMul, checked_mul, Option<Self>);
num_trait_impl!(CheckedRem, checked_rem, Option<Self>);
num_trait_impl!(CheckedSub, checked_sub, Option<Self>);

num_trait_impl!(SaturatingAdd, saturating_add, Self);
num_trait_impl!(SaturatingMul, saturating_mul, Self);
num_trait_impl!(SaturatingSub, saturating_sub, Self);

num_trait_impl!(WrappingAdd, wrapping_add, Self);
num_trait_impl!(WrappingMul, wrapping_mul, Self);
num_trait_impl!(WrappingSub, wrapping_sub, Self);

impl<const N: usize> CheckedNeg for Bint<N> {
    fn checked_neg(&self) -> Option<Self> {
        Self::checked_neg(*self)
    }
}

impl AsPrimitive<char> for Bint<0> {
    fn as_(self) -> char {
        0u8 as char
    }
}

macro_rules! as_primitive_impl {
    ($ty: ty, $method: ident) => {
        impl<const N: usize> AsPrimitive<$ty> for Bint<N> {
            fn as_(self) -> $ty {
                self.$method()
            }
        }
    }
}

as_primitive_impl!(f32, as_f32);
as_primitive_impl!(f64, as_f64);
as_primitive_impl!(u8, as_u8);
as_primitive_impl!(u16, as_u16);
as_primitive_impl!(u32, as_u32);
as_primitive_impl!(usize, as_usize);
as_primitive_impl!(u64, as_u64);
as_primitive_impl!(u128, as_u128);
as_primitive_impl!(i8, as_i8);
as_primitive_impl!(i16, as_i16);
as_primitive_impl!(i32, as_i32);
as_primitive_impl!(isize, as_isize);
as_primitive_impl!(i64, as_i64);
as_primitive_impl!(i128, as_i128);

macro_rules! as_biint {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> AsPrimitive<Bint<N>> for $ty {
                fn as_(self) -> Bint<N> {
                    Bint {
                        uint: self.as_(),
                    }
                }
            }
        )*
    }
}

as_biint!(u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128, char, bool);

impl<const N: usize> AsPrimitive<Bint<N>> for f32 {
    fn as_(self) -> Bint<N> {
        Bint::try_from(self).unwrap_or(if self.is_sign_negative() {
            Bint::MIN
        } else {
            Bint::MAX
        })
    }
}

impl<const N: usize> AsPrimitive<Bint<N>> for f64 {
    fn as_(self) -> Bint<N> {
        Bint::try_from(self).unwrap_or(if self.is_sign_negative() {
            Bint::MIN
        } else {
            Bint::MAX
        })
    }
}

use crate::BUint;

#[cfg(feature = "nightly")]
impl<const N: usize, const M: usize> AsPrimitive<BUint<M>> for Bint<N> where [(); M.saturating_sub(N)]: Sized {
    fn as_(self) -> BUint<M> {
        self.as_buint::<M>()
    }
}

#[cfg(feature = "nightly")]
impl<const N: usize, const M: usize> AsPrimitive<Bint<M>> for Bint<N> where [(); M.saturating_sub(N)]: Sized {
    fn as_(self) -> Bint<M> {
        self.as_biint::<M>()
    }
}

use core::convert::TryInto;

impl<const N: usize> CheckedShl for Bint<N> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        Self::checked_shl(*self, rhs.try_into().ok()?)
    }
}

impl<const N: usize> CheckedShr for Bint<N> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        Self::checked_shr(*self, rhs.try_into().ok()?)
    }
}

impl<const N: usize> WrappingNeg for Bint<N> {
    fn wrapping_neg(&self) -> Self {
        Self::wrapping_neg(*self)
    }
}

impl<const N: usize> WrappingShl for Bint<N> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        Self::wrapping_shl(*self, rhs as ExpType)
    }
}

impl<const N: usize> WrappingShr for Bint<N> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        Self::wrapping_shr(*self, rhs as ExpType)
    }
}

impl<const N: usize> FromPrimitive for Bint<N> {
    fn from_u64(n: u64) -> Option<Self> {
        Some(n.into())
    }
    fn from_i64(n: i64) -> Option<Self> {
        Some(n.into())
    }
    fn from_u128(n: u128) -> Option<Self> {
        Some(n.into())
    }
    fn from_i128(n: i128) -> Option<Self> {
        Some(n.into())
    }
    fn from_f32(f: f32) -> Option<Self> {
        Self::try_from(f).ok()
    }
    fn from_f64(f: f64) -> Option<Self> {
        Self::try_from(f).ok()
    }
}

impl<const N: usize> Integer for Bint<N> {
    fn div_floor(&self, other: &Self) -> Self {
        *self / *other
    }
    fn mod_floor(&self, other: &Self) -> Self {
        *self % *other
    }
    fn gcd(&self, other: &Self) -> Self {
        if other.is_zero() {
            *self
        } else {
            other.gcd(&self.mod_floor(other))
        }
    }
    fn lcm(&self, other: &Self) -> Self {
        self.div_floor(&self.gcd(other)) * *other
    }
    fn divides(&self, other: &Self) -> bool {
        self.is_multiple_of(other)
    }
    fn is_multiple_of(&self, other: &Self) -> bool {
        self.mod_floor(other).is_zero()
    }
    fn is_even(&self) -> bool {
        self.uint.is_even()
    }
    fn is_odd(&self) -> bool {
        self.uint.is_odd()
    }
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        (self.div_floor(other), self.mod_floor(other))
    }
}

impl<const N: usize> MulAdd for Bint<N> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl<const N: usize> MulAddAssign for Bint<N> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

use crate::ParseIntError;

impl<const N: usize> Num for Bint<N> {
    type FromStrRadixErr = ParseIntError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Self::from_str_radix(string, radix)
    }
}

impl<const N: usize> One for Bint<N> {
    fn one() -> Self {
        Self::ONE
    }
    fn is_one(&self) -> bool {
        self == &Self::ONE
    }
}

impl<const N: usize> Pow<ExpType> for Bint<N> {
    type Output = Self;

    fn pow(self, exp: ExpType) -> Self {
        Self::pow(self, exp)
    }
}

impl<const N: usize> Roots for Bint<N> {
    fn sqrt(&self) -> Self {
        if self.is_negative() {
            panic!("imaginary square root")
        } else {
            Self {
                uint: self.uint.sqrt()
            }
        }
    }
    fn cbrt(&self) -> Self {
        if self.is_negative() {
            let out = Self {
                uint: self.unsigned_abs().cbrt(),
            };
            -out
        } else {
            Self {
                uint: self.uint.cbrt(),
            }
        }
    }
    fn nth_root(&self, n: u32) -> Self {
        if self.is_negative() {
            if n.is_even() {
                panic!("imaginary root degree of {}", n)
            } else {
                let out = Self {
                    uint: self.unsigned_abs().nth_root(n),
                };
                -out
            }
        } else {
            Self {
                uint: self.uint.nth_root(n),
            }
        }
    }
}

impl<const N: usize> Saturating for Bint<N> {
    fn saturating_add(self, rhs: Self) -> Self {
        Self::saturating_add(self, rhs)
    }
    fn saturating_sub(self, rhs: Self) -> Self {
        Self::saturating_sub(self, rhs)
    }
}

impl<const N: usize> ToPrimitive for Bint<N> {
    fn to_i64(&self) -> Option<i64> {
        if self.is_negative() {
            let ones = Self::BITS - 64 + 1;
            if (self.leading_ones() as usize) < ones {
                None
            } else {
                Some(self.as_i64())
                //Some(self.digits()[0] as i64)
            }
        } else {
            self.uint.to_i64()
        }
    }
    fn to_i128(&self) -> Option<i128> {
        if self.is_negative() {
            let ones = Self::BITS - 128 + 1;
            if (self.leading_ones() as usize) < ones {
                None
            } else {
                Some(self.as_i128())
            }
        } else {
            self.uint.to_i128()
        }
    }
    fn to_u64(&self) -> Option<u64> {
        if self.is_negative() {
            None
        } else {
            self.uint.to_u64()
        }
    }
    fn to_u128(&self) -> Option<u128> {
        if self.is_negative() {
            None
        } else {
            self.uint.to_u128()
        }
    }
    fn to_f32(&self) -> Option<f32> {
        Some(self.as_f32())
    }
    fn to_f64(&self) -> Option<f64> {
        Some(self.as_f64())
    }
}

impl<const N: usize> Signed for Bint<N> {
    fn abs(&self) -> Self {
        Self::abs(*self)
    }
    fn abs_sub(&self, other: &Self) -> Self {
        if *self <= *other {
            Self::ZERO
        } else {
            *self - *other
        }
    }
    fn signum(&self) -> Self {
        Self::signum(*self)
    }
    fn is_positive(&self) -> bool {
        Self::is_positive(*self)
    }
    fn is_negative(&self) -> bool {
        Self::is_negative(*self)
    }
}

impl<const N: usize> Zero for Bint<N> {
    fn zero() -> Self {
        Self::ZERO
    }
    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}