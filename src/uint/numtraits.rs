use super::BUint;
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, SaturatingAdd, SaturatingMul, SaturatingSub, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, ToPrimitive, Unsigned, Zero, Pow};
use num_integer::{Integer, Roots};

impl<const N: usize> Bounded for BUint<N> {
    fn min_value() -> Self {
        Self::MIN
    }
    fn max_value() -> Self {
        Self::MAX
    }
}

macro_rules! num_trait_impl {
    ($tr: ident, $method: ident, $ret: ty) => {
        impl<const N: usize> $tr for BUint<N> {
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

impl<const N: usize> CheckedNeg for BUint<N> {
    fn checked_neg(&self) -> Option<Self> {
        Self::checked_neg(*self)
    }
}

impl<const N: usize> CheckedShl for BUint<N> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        Self::checked_shl(*self, rhs)
    }
}

impl<const N: usize> CheckedShr for BUint<N> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        Self::checked_shr(*self, rhs)
    }
}

impl<const N: usize> WrappingNeg for BUint<N> {
    fn wrapping_neg(&self) -> Self {
        Self::wrapping_neg(*self)
    }
}

impl<const N: usize> WrappingShl for BUint<N> {
    fn wrapping_shl(&self, rhs: u32) -> Self {
        Self::wrapping_shl(*self, rhs)
    }
}

impl<const N: usize> WrappingShr for BUint<N> {
    fn wrapping_shr(&self, rhs: u32) -> Self {
        Self::wrapping_shr(*self, rhs)
    }
}

impl<const N: usize> Pow<u32> for BUint<N> {
    type Output = Self;

    fn pow(self, exp: u32) -> Self {
        Self::pow(self, exp)
    }
}

use std::convert::TryFrom;

impl<const N: usize> FromPrimitive for BUint<N> {
    fn from_u64(int: u64) -> Option<Self> {
        Some(int.into())
    }
    fn from_i64(int: i64) -> Option<Self> {
        match u64::try_from(int) {
            Ok(int) => Some(int.into()),
            _ => None,
        }
    }
    fn from_u128(n: u128) -> Option<Self> {
        Some(n.into())
    }
    fn from_i128(n: i128) -> Option<Self> {
        match u64::try_from(n) {
            Ok(n) => Some(n.into()),
            _ => None,
        }
    }
}

impl<const N: usize> Integer for BUint<N> {
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
        self.digits[0] & 1 == 0
    }
    fn is_odd(&self) -> bool {
        self.digits[0] & 1 == 1
    }
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        Self::div_rem(*self, *rhs)
    }
}

impl<const N: usize> MulAdd for BUint<N> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl<const N: usize> MulAddAssign for BUint<N> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

use crate::ParseIntError;

impl<const N: usize> Num for BUint<N> {
    type FromStrRadixErr = ParseIntError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Self::from_str_radix(string, radix)
    }
}

impl<const N: usize> One for BUint<N> {
    fn one() -> Self {
        Self::ONE
    }
    fn is_one(&self) -> bool {
        self == &Self::ONE
    }
}

impl<const N: usize> Roots for BUint<N> {
    fn sqrt(&self) -> Self {
        Self::sqrt(self)
    }
    fn cbrt(&self) -> Self {
        Self::cbrt(self)
    }
    fn nth_root(&self, n: u32) -> Self {
        Self::nth_root(self, n)
    }
}

impl<const N: usize> ToPrimitive for BUint<N> {
    fn to_i64(&self) -> Option<i64> {
        match self.to_u64() {
            Some(int) => int.to_i64(),
            None => None,
        }
    }
    fn to_i128(&self) -> Option<i128> {
        match self.to_u128() {
            Some(int) => int.to_i128(),
            None => None,
        }
    }
    fn to_u64(&self) -> Option<u64> {
        let last_index = self.last_digit_index();
        if last_index > 0 {
            return None;
        }
        let first = if N > 0 {
            self.digits[0]
        } else {
            0u64
        };
        Some(first)
    }
    fn to_u128(&self) -> Option<u128> {
        let last_index = self.last_digit_index();
        if last_index > 1 {
            return None;
        }
        Some(if N == 0 {
            0
        } else if N == 1 {
            self.digits[0] as u128
        } else {
            (self.digits[0] as u128) | ((self.digits[1] as u128) << 64)
        })
    }
}

impl<const N: usize> Unsigned for BUint<N> {}

impl<const N: usize> Zero for BUint<N> {
    fn zero() -> Self {
        Self::ZERO
    }
    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}