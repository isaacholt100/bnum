use super::BInt;
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, SaturatingAdd, SaturatingMul, SaturatingSub, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, ToPrimitive, Signed, Zero, Pow, Saturating, AsPrimitive};
use num_integer::{Integer, Roots};
use core::convert::TryFrom;
use crate::ExpType;
use crate::digit::{self, Digit};
use crate::error;

impl<const N: usize> Bounded for BInt<N> {
    #[inline]
    fn min_value() -> Self {
        Self::MIN
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }
}

macro_rules! num_trait_impl {
    ($tr: ident, $method: ident, $ret: ty) => {
        impl<const N: usize> $tr for BInt<N> {
            #[inline]
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

impl<const N: usize> CheckedNeg for BInt<N> {
    #[inline]
    fn checked_neg(&self) -> Option<Self> {
        Self::checked_neg(*self)
    }
}

macro_rules! as_primitive_impl {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> AsPrimitive<$ty> for BInt<N> {
                #[inline]
                fn as_(self) -> $ty {
                    crate::As::as_(self)
                }
            }
        )*
    }
}

as_primitive_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

crate::macros::as_bigint_impl!([u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128, char, bool] as BInt);

impl<const N: usize> AsPrimitive<BInt<N>> for f32 {
    #[inline]
    fn as_(self) -> BInt<N> {
		BInt::cast_from(self)
    }
}

impl<const N: usize> AsPrimitive<BInt<N>> for f64 {
    #[inline]
    fn as_(self) -> BInt<N> {
		BInt::cast_from(self)
    }
}

use crate::BUint;
use crate::cast::CastFrom;

impl<const N: usize, const M: usize> AsPrimitive<BUint<M>> for BInt<N> {
    #[inline]
    fn as_(self) -> BUint<M> {
        BUint::<M>::cast_from(self)
    }
}

impl<const N: usize, const M: usize> AsPrimitive<BInt<M>> for BInt<N> {
    #[inline]
    fn as_(self) -> BInt<M> {
        BInt::<M>::cast_from(self)
    }
}

impl<const N: usize> CheckedShl for BInt<N> {
    #[inline]
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        Self::checked_shl(*self, rhs)
    }
}

impl<const N: usize> CheckedShr for BInt<N> {
    #[inline]
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        Self::checked_shr(*self, rhs)
    }
}

impl<const N: usize> WrappingNeg for BInt<N> {
    #[inline]
    fn wrapping_neg(&self) -> Self {
        Self::wrapping_neg(*self)
    }
}

impl<const N: usize> WrappingShl for BInt<N> {
    #[inline]
    fn wrapping_shl(&self, rhs: u32) -> Self {
        Self::wrapping_shl(*self, rhs as ExpType)
    }
}

impl<const N: usize> WrappingShr for BInt<N> {
    #[inline]
    fn wrapping_shr(&self, rhs: u32) -> Self {
        Self::wrapping_shr(*self, rhs as ExpType)
    }
}

impl<const N: usize> FromPrimitive for BInt<N> {
    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(n.into())
    }

    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        Some(n.into())
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        const UINT_BITS: usize = u128::BITS as usize;
        let mut out = BInt::ZERO;
        let mut i = 0;
        while i << digit::BIT_SHIFT < UINT_BITS {
            let d = (n >> (i << digit::BIT_SHIFT)) as Digit;
            if d != 0 {
                if i < N {
                    out.bits.digits[i] = d;
                } else {
                    return None;
                }
            }
            i += 1;
        }
		if Signed::is_negative(&out) {
			None
		} else {
			Some(out)
		}
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        const INT_BITS: usize = i128::BITS as usize;
        let mut out = if n.is_negative() {
			BInt::ZERO
		} else {
			BInt::NEG_ONE
		};
        let mut i = 0;
        while i << digit::BIT_SHIFT < INT_BITS {
            let d = (n >> (i << digit::BIT_SHIFT)) as Digit;
            if d != 0 {
                if i < N {
                    out.bits.digits[i] = d;
                } else {
                    return None;
                }
            }
            i += 1;
        }
		Some(out)
    }

    #[inline]
    fn from_f32(f: f32) -> Option<Self> {
        Self::try_from(f).ok()
    }

    #[inline]
    fn from_f64(f: f64) -> Option<Self> {
        Self::try_from(f).ok()
    }
}

impl<const N: usize> Integer for BInt<N> {
    #[inline]
    fn div_floor(&self, other: &Self) -> Self {
        *self / *other
    }

    #[inline]
    fn mod_floor(&self, other: &Self) -> Self {
        *self % *other
    }

    #[inline]
    fn gcd(&self, other: &Self) -> Self {
        if other.is_zero() {
            *self
        } else {
            other.gcd(&self.mod_floor(other))
        }
    }

    #[inline]
    fn lcm(&self, other: &Self) -> Self {
        self.div_floor(&self.gcd(other)) * *other
    }

    #[inline]
    fn divides(&self, other: &Self) -> bool {
        self.is_multiple_of(other)
    }

    #[inline]
    fn is_multiple_of(&self, other: &Self) -> bool {
        self.mod_floor(other).is_zero()
    }

    #[inline]
    fn is_even(&self) -> bool {
        self.bits.is_even()
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.bits.is_odd()
    }

    #[inline]
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        (self.div_floor(other), self.mod_floor(other))
    }
}

impl<const N: usize> MulAdd for BInt<N> {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl<const N: usize> MulAddAssign for BInt<N> {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

use crate::ParseIntError;

impl<const N: usize> Num for BInt<N> {
    type FromStrRadixErr = ParseIntError;

    #[inline]
    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Self::from_str_radix(string, radix)
    }
}

impl<const N: usize> One for BInt<N> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn is_one(&self) -> bool {
        self == &Self::ONE
    }
}

impl<const N: usize> Pow<ExpType> for BInt<N> {
    type Output = Self;

    #[inline]
    fn pow(self, exp: ExpType) -> Self {
        Self::pow(self, exp)
    }
}

impl<const N: usize> Roots for BInt<N> {
    #[inline]
    fn sqrt(&self) -> Self {
        if self.is_negative() {
            panic!(error::err_msg!("imaginary square root"))
        } else {
            Self::from_bits(self.bits.sqrt())
        }
    }

    #[inline]
    fn cbrt(&self) -> Self {
        if self.is_negative() {
            let out = Self::from_bits(self.unsigned_abs().cbrt());
            -out
        } else {
            Self::from_bits(self.bits.cbrt())
        }
    }

    #[inline]
    fn nth_root(&self, n: u32) -> Self {
        if self.is_negative() {
            if n.is_even() {
                panic!("{} imaginary root degree of {}", error::err_prefix!(), n)
            } else {
                let out = Self::from_bits(self.unsigned_abs().nth_root(n));
                -out
            }
        } else {
            Self::from_bits(self.bits.nth_root(n))
        }
    }
}

impl<const N: usize> Saturating for BInt<N> {
    #[inline]
    fn saturating_add(self, rhs: Self) -> Self {
        Self::saturating_add(self, rhs)
    }

    #[inline]
    fn saturating_sub(self, rhs: Self) -> Self {
        Self::saturating_sub(self, rhs)
    }
}

impl<const N: usize> ToPrimitive for BInt<N> {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        if self.is_negative() {
            let ones = Self::BITS - 64 + 1;
            if self.leading_ones() < ones {
                None
            } else {
                Some(self.as_())
            }
        } else {
            self.bits.to_i64()
        }
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        if self.is_negative() {
            let ones = Self::BITS - 128 + 1;
            if self.leading_ones() < ones {
                None
            } else {
                Some(self.as_())
            }
        } else {
            self.bits.to_i128()
        }
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        if self.is_negative() {
            None
        } else {
            self.bits.to_u64()
        }
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        if self.is_negative() {
            None
        } else {
            self.bits.to_u128()
        }
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some(self.as_())
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some(self.as_())
    }
}

impl<const N: usize> Signed for BInt<N> {
    #[inline]
    fn abs(&self) -> Self {
        Self::abs(*self)
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if *self <= *other {
            Self::ZERO
        } else {
            *self - *other
        }
    }

    #[inline]
    fn signum(&self) -> Self {
        Self::signum(*self)
    }

    #[inline]
    fn is_positive(&self) -> bool {
        Self::is_positive(*self)
    }

    #[inline]
    fn is_negative(&self) -> bool {
        Self::is_negative(*self)
    }
}

impl<const N: usize> Zero for BInt<N> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::{Roots, ToPrimitive, FromPrimitive, Integer};
	use crate::test::test_bignum;

	test_bignum! {
		function: <i128>::sqrt(a: ref &i128)
	}
	test_bignum! {
		function: <i128>::cbrt(a: ref &i128)
	}
	test_bignum! {
		function: <i128>::nth_root(u: ref &i128, n: u32),
		skip: n == 0
	}

	macro_rules! test_to_primitive {
		($($prim: ty), *) => {
			paste::paste! {
				$(
					test_bignum! {
						function: <i128>::[<to_ $prim>](u: ref &i128)
					}
				)*
			}
		};
	}

	test_to_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

	macro_rules! test_from_primitive {
		($($prim: ty), *) => {
			paste::paste! {
				$(
					test_bignum! {
						function: <i64>::[<from_ $prim>](u: $prim)
					}
				)*
			}
		};
	}

	test_from_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

	macro_rules! test_integer_trait {
		(<$primitive: ty> :: $($function: ident), *) => {
			$(
				test_bignum! {
					function: <$primitive as Integer>::$function(a: ref &$primitive, b: ref &$primitive)
				}
			)*
		};
	}

	test_integer_trait!(<i128>::div_floor, mod_floor, gcd, lcm, divides, is_multiple_of, div_rem);

	test_bignum! {
		function: <i128 as Integer>::is_even(a: ref &i128)
	}
	test_bignum! {
		function: <i128 as Integer>::is_odd(a: ref &i128)
	}
}