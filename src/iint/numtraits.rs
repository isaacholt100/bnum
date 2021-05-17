use super::BIint;
use crate::sign::Sign;
use crate::uint::BUint;
use crate::tryops::TryOps;
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, Signed, ToPrimitive, Zero};
use num_integer::Integer;
use std::convert::TryFrom;

impl<const N: usize> Bounded for BIint<N> {
    fn min_value() -> Self {
        Self {
            uint: BUint::max_value(),
            sign: Sign::Minus,
        }
    }
    fn max_value() -> Self {
        Self {
            uint: BUint::max_value(),
            sign: Sign::Plus,
        }
    }
}

impl<const N: usize> CheckedAdd for BIint<N> {
    fn checked_add(&self, rhs: &Self) -> Option<Self> {
        self.try_add(*rhs).ok()
    }
}

impl<const N: usize> CheckedDiv for BIint<N> {
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        self.try_div(*rhs).ok()
    }
}

impl<const N: usize> CheckedMul for BIint<N> {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        self.try_mul(*rhs).ok()
    }
}

impl<const N: usize> CheckedNeg for BIint<N> {
    fn checked_neg(&self) -> Option<Self> {
        use std::ops::Neg;
        Some(self.neg())
    }
}

impl<const N: usize> CheckedRem for BIint<N> {
    fn checked_rem(&self, rhs: &Self) -> Option<Self> {
        self.try_rem(*rhs).ok()
    }
}

impl<const N: usize> CheckedSub for BIint<N> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        self.try_sub(*rhs).ok()
    }
}

impl<const N: usize> FromPrimitive for BIint<N> {
    fn from_u64(int: u64) -> Option<Self> {
        Some(int.into())
    }
    fn from_i64(int: i64) -> Option<Self> {
        Some(int.into())
    }
}

impl<const N: usize> Integer for BIint<N> {
    fn div_floor(&self, other: &Self) -> Self {
        *self / *other
    }
    fn mod_floor(&self, other: &Self) -> Self {
        *self % *other
    }
    fn gcd(&self, other: &Self) -> Self {
        Self {
            uint: self.uint.gcd(&other.uint),
            sign: Sign::Plus,
        }
    }
    fn lcm(&self, other: &Self) -> Self {
        Self {
            uint: self.uint.lcm(&other.uint),
            sign: Sign::Plus,
        }
    }
    fn divides(&self, other: &Self) -> bool {
        self.is_multiple_of(other)
    }
    fn is_multiple_of(&self, other: &Self) -> bool {
        self.uint.is_multiple_of(&other.uint)
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

impl<const N: usize> MulAdd for BIint<N> {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl<const N: usize> MulAddAssign for BIint<N> {
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

impl<const N: usize> Num for BIint<N> {
    type FromStrRadixErr = crate::ParseIntError;

    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if string.starts_with("-") {
            let uint = BUint::from_str_radix(&string[1..], radix)?;
            Ok(Self {
                uint,
                sign: uint.sign_or_zero(Sign::Minus),
            })
        } else {
            let uint = BUint::from_str_radix(string, radix)?;
            Ok(Self {
                uint,
                sign: uint.sign_or_zero(Sign::Plus),
            })
        }
    }
}

impl<const N: usize> One for BIint<N> {
    fn one() -> Self {
        Self {
            uint: BUint::one(),
            sign: Sign::Plus,
        }
    }
    fn is_one(&self) -> bool {
        self.sign == Sign::Plus && self.uint.is_one()
    }
}

impl<const N: usize> Signed for BIint<N> {
    fn abs(&self) -> Self {
        if self.is_zero() {
            Self::zero()
        } else {
            Self {
                uint: self.uint,
                sign: Sign::Plus,
            }
        }
    }
    fn abs_sub(&self, other: &Self) -> Self {
        use std::ops::Sub;
        if self <= other {
            Self::zero()
        } else {
            self.sub(*other)
        }
    }
    fn signum(&self) -> Self {
        use std::ops::Neg;
        match self.sign {
            Sign::Plus => Self::one(),
            Sign::Zero => Self::zero(),
            Sign::Minus => Self::one().neg(),
        }
    }
    fn is_positive(&self) -> bool {
        self.sign == Sign::Plus
    }
    fn is_negative(&self) -> bool {
        self.sign == Sign::Minus
    }
}

impl<const N: usize> ToPrimitive for BIint<N> {
    fn to_i64(&self) -> Option<i64> {
        let uint = self.uint.to_u64()?;
        let iint = uint.to_i64()?;
        if self.is_negative() {
            Some(-iint)
        } else {
            Some(iint)
        }
    }
    fn to_i128(&self) -> Option<i128> {
        let uint = self.uint.to_u128()?;
        let iint = uint.to_i128()?;
        if self.is_negative() {
            Some(-iint)
        } else {
            Some(iint)
        }
    }
    fn to_u64(&self) -> Option<u64> {
        u64::try_from(*self).ok()
    }
    fn to_u128(&self) -> Option<u128> {
        u128::try_from(*self).ok()
    }
}

impl<const N: usize> Zero for BIint<N> {
    fn zero() -> Self {
        Self {
            uint: BUint::zero(),
            sign: Sign::Zero,
        }
    }
    fn is_zero(&self) -> bool {
        self.sign == Sign::Zero // self.uint.is_zero()
    }
}