use super::BUint;
use crate::tryops::TryOps;
use crate::iint::BIint;
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedShl, CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, SaturatingAdd, ToPrimitive, Unsigned, Zero};
use num::Integer;
use std::convert::TryInto;

impl<const N: usize> Bounded for BUint<N> {
    fn min_value() -> Self {
        Self::MIN
    }
    fn max_value() -> Self {
        Self::MAX
    }
}

macro_rules! num_trait_impl {
    ($tr: ident, $method: ident) => {
        impl<const N: usize> $tr for BUint<N> {
            fn $method(&self, rhs: &Self) -> Option<Self> {
                self.$method(rhs)
            }
        }
    }
}

num_trait_impl!(CheckedAdd, checked_add);
num_trait_impl!(CheckedDiv, checked_div);
num_trait_impl!(CheckedMul, checked_mul);
num_trait_impl!(CheckedRem, checked_rem);
num_trait_impl!(CheckedSub, checked_sub);

impl<const N: usize> CheckedShl for BUint<N> {
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        self.checked_shl(rhs)
    }
}

impl<const N: usize> CheckedShr for BUint<N> {
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        self.checked_shr(rhs)
    }
}

impl<const N: usize> FromPrimitive for BUint<N> {
    fn from_u64(int: u64) -> Option<Self> {
        Some(int.into())
    }
    fn from_i64(int: i64) -> Option<Self> {
        let int: BIint<N> = int.into();
        if let Ok(int) = int.try_into() {
            Some(int)
        } else {
            None
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
        self.digits[0].is_even()
    }
    fn is_odd(&self) -> bool {
        self.digits[0].is_odd()
    }
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        (self.div_floor(other), self.mod_floor(other))
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

impl<const N: usize> Num for BUint<N> {
    type FromStrRadixErr = crate::ParseIntError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Self::from_str_radix(string, radix)
    }
}

impl<const N: usize> One for BUint<N> {
    fn one() -> Self {
        Self::ONE
    }
    fn is_one(&self) -> bool {
        let mut i = 1;
        if self.digits[0] != 1 {
            return false;
        }
        while i < N {
            if self.digits[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl<const N: usize> SaturatingAdd for BUint<N> {
    fn saturating_add(&self, rhs: &Self) -> Self {
        let result = self.try_add(*rhs);
        match result {
            Ok(num) => num,
            Err(_) => Self::max_value(),
        }
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
            (self.digits[0] as u128) + ((self.digits[1] as u128) << 64)
        })
    }
}

impl<const N: usize> Unsigned for BUint<N> {}

impl<const N: usize> Zero for BUint<N> {
    fn zero() -> Self {
        Self::MIN
    }
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}