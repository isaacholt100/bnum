use super::{BUint, ExpType};
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, SaturatingAdd, SaturatingMul, SaturatingSub, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, ToPrimitive, Unsigned, Zero, Pow, Saturating, AsPrimitive};
use num_integer::{Integer, Roots};
use crate::digit::{self, Digit};
use crate::cast::CastFrom;

impl<const N: usize> Bounded for BUint<N> {
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
        impl<const N: usize> $tr for BUint<N> {
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

impl<const N: usize> CheckedNeg for BUint<N> {
    #[inline]
    fn checked_neg(&self) -> Option<Self> {
        Self::checked_neg(*self)
    }
}

use core::convert::TryInto;

impl<const N: usize> CheckedShl for BUint<N> {
    #[inline]
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        Self::checked_shl(*self, rhs.try_into().ok()?)
    }
}

impl<const N: usize> CheckedShr for BUint<N> {
    #[inline]
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        Self::checked_shr(*self, rhs.try_into().ok()?)
    }
}

impl<const N: usize> WrappingNeg for BUint<N> {
    #[inline]
    fn wrapping_neg(&self) -> Self {
        Self::wrapping_neg(*self)
    }
}

impl<const N: usize> WrappingShl for BUint<N> {
    #[inline]
    fn wrapping_shl(&self, rhs: u32) -> Self {
        Self::wrapping_shl(*self, rhs as ExpType)
    }
}

impl<const N: usize> WrappingShr for BUint<N> {
    #[inline]
    fn wrapping_shr(&self, rhs: u32) -> Self {
        Self::wrapping_shr(*self, rhs as ExpType)
    }
}

impl<const N: usize> Pow<ExpType> for BUint<N> {
    type Output = Self;

    #[inline]
    fn pow(self, exp: ExpType) -> Self {
        Self::pow(self, exp)
    }
}

use core::convert::TryFrom;

macro_rules! as_primitive_impl {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> AsPrimitive<$ty> for BUint<N> {
                #[inline]
                fn as_(self) -> $ty {
                    crate::As::as_(self)
                }
            }
        )*
    }
}

as_primitive_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! as_buint_impl {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> AsPrimitive<BUint<N>> for $ty {
                #[inline]
                fn as_(self) -> BUint<N> {
					BUint::cast_from(self)
                }
            }
        )*
    }
}

as_buint_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char, f32, f64);

impl<const N: usize, const M: usize> AsPrimitive<BUint<M>> for BUint<N> {
    #[inline]
    fn as_(self) -> BUint<M> {
        BUint::<M>::cast_from(self)
    }
}

use crate::Bint;

impl<const N: usize, const M: usize> AsPrimitive<Bint<M>> for BUint<N> {
    #[inline]
    fn as_(self) -> Bint<M> {
        Bint::<M>::cast_from(self)
    }
}

impl<const N: usize> FromPrimitive for BUint<N> {
    #[inline]
    fn from_u64(int: u64) -> Option<Self> {
        const UINT_BITS: usize = u64::BITS as usize;
        let mut out = BUint::ZERO;
        let mut i = 0;
        while i << digit::BIT_SHIFT < UINT_BITS {
            let d = (int >> (i << digit::BIT_SHIFT)) as Digit;
            if d != 0 {
                if i < N {
                    out.digits[i] = d;
                } else {
                    return None;
                }
            }
            i += 1;
        }
        Some(out)
    }

    #[inline]
    fn from_i64(int: i64) -> Option<Self> {
        match u64::try_from(int) {
            Ok(int) => Self::from_u64(int),
            _ => None,
        }
    }

    #[inline]
    fn from_u128(int: u128) -> Option<Self> {
        const UINT_BITS: usize = u128::BITS as usize;
        let mut out = BUint::ZERO;
        let mut i = 0;
        while i << digit::BIT_SHIFT < UINT_BITS {
            let d = (int >> (i << digit::BIT_SHIFT)) as Digit;
            if d != 0 {
                if i < N {
                    out.digits[i] = d;
                } else {
                    return None;
                }
            }
            i += 1;
        }
        Some(out)
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        match u128::try_from(n) {
            Ok(n) => Self::from_u128(n),
            _ => None,
        }
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

impl<const N: usize> Integer for BUint<N> {
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
        self.digits[0] & 1 == 0
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.digits[0] & 1 == 1
    }

    #[inline]
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        Self::div_rem(*self, *rhs)
    }
}

impl<const N: usize> MulAdd for BUint<N> {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl<const N: usize> MulAddAssign for BUint<N> {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

use crate::ParseIntError;

impl<const N: usize> Num for BUint<N> {
    type FromStrRadixErr = ParseIntError;

    #[inline]
    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Self::from_str_radix(string, radix)
    }
}

impl<const N: usize> One for BUint<N> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn is_one(&self) -> bool {
        self == &Self::ONE
    }
}

/*impl<const N: usize> NumCast for BUint<N> {
    fn from<T: Zero>(n: T) -> Option<Self> {
        todo!()
    }
}*/

/*impl<const N: usize> PrimInt for BUint<N> {
    fn count_ones(self) -> u32 {

    }
    fn count_zeros(self) -> u32 {
        
    }
    fn leading_zeros(self) -> u32 {

    }
    fn trailing_zeros(self) -> u32 {

    }
    fn rotate_left(self, n: u32) -> Self {

    }
    fn rotate_right(self, n: u32) -> Self {

    }
    fn signed_shl(self, n: u32) -> Self {

    }
    fn signed_shr(self, n: u32) -> Self {
        
    }
    fn unsigned_shl(self, n: u32) -> Self {

    }
    fn unsigned_shr(self, n: u32) -> Self {
        
    }
    fn swap_bytes(self) -> Self {
        Self::swap_bytes(self)
    }
    fn from_be(x: Self) -> Self {
        Self::from_be(x)
    }
    fn from_le(x: Self) -> Self {
        Self::from_le(x)
    }
    fn to_be(self) -> Self {
        Self::to_be(self)
    }
    fn to_le(self) -> Self {
        Self::to_le(self)
    }
    fn pow(self, exp: u32) -> Self {

    }
}*/

impl<const N: usize> Saturating for BUint<N> {
    #[inline]
    fn saturating_add(self, rhs: Self) -> Self {
        Self::saturating_add(self, rhs)
    }

    #[inline]
    fn saturating_sub(self, rhs: Self) -> Self {
        Self::saturating_sub(self, rhs)
    }
}

macro_rules! check_zero_or_one {
    ($self: ident) => {
        if N == 0 {
            return *$self;
        }
        if $self.last_digit_index() == 0 {
            let d = $self.digits[0];
            if d == 0 || d == 1 {
                return *$self;
            }
        }
    }
}

impl<const N: usize> BUint<N> {
    #[inline]
    fn fixpoint<F>(mut self, max_bits: ExpType, f: F) -> Self
    where F: Fn(Self) -> Self {
        let mut xn = f(self);
        while self < xn {
            self = if xn.bits() > max_bits {
                Self::power_of_two(max_bits)
            } else {
                xn
            };
            xn = f(self);
        }
        while self > xn {
            self = xn;
            xn = f(self);
        }
        self
    }
}

impl<const N: usize> Roots for BUint<N> {
    #[inline]
    fn sqrt(&self) -> Self {
        check_zero_or_one!(self);

        if let Some(n) = self.to_u128() {
            return n.sqrt().into();
        }
        let bits = self.bits();
        let max_bits = bits / 2 + 1;

        let guess = Self::power_of_two(max_bits);
        guess.fixpoint(max_bits, |s| {
            let q = self / s;
            let t = s + q;
            t >> 1
        })
    }

    #[inline]
    fn cbrt(&self) -> Self {
        check_zero_or_one!(self);

        if let Some(n) = self.to_u128() {
            return n.cbrt().into();
        }
        let bits = self.bits();
        let max_bits = bits / 3 + 1;

        let guess = Self::power_of_two(max_bits);
        guess.fixpoint(max_bits, |s| {
            let q = self / (s * s);
            let t: Self = (s << 1) + q;
            t.div_rem_digit(3).0
        })
    }

    #[inline]
    fn nth_root(&self, n: u32) -> Self {
        match n {
            0 => panic!("attempt to calculate zeroth root"),
            1 => *self,
            2 => self.sqrt(),
            3 => self.cbrt(),
            _ => {
                check_zero_or_one!(self);

                if let Some(x) = self.to_u128() {
                    return x.nth_root(n).into();
                }
                let bits = self.bits();
                let n = n as ExpType;
                if bits <= n {
                    return Self::ONE;
                }

                let max_bits = bits / n + 1;
        
                let guess = Self::power_of_two(max_bits);
                let n_minus_1 = n - 1;

                guess.fixpoint(max_bits, |s| {
                    let q = self / s.pow(n_minus_1);
                    let mul: Self = n_minus_1.into();
                    let t: Self = s * mul + q;
                    t.div_rem_unchecked(n.into()).0
                })
            }
        }
    }
}

macro_rules! to_int {
	($int: ty, $method: ident) => {
		#[inline]
		fn $method(&self) -> Option<$int> {
			<$int>::try_from(*self).ok()
		}
	}
}

impl<const N: usize> ToPrimitive for BUint<N> {
    to_int!(i8, to_i8);
    to_int!(i16, to_i16);
    to_int!(i32, to_i32);
    to_int!(i64, to_i64);
    to_int!(i128, to_i128);
    to_int!(isize, to_isize);
    to_int!(u8, to_u8);
    to_int!(u16, to_u16);
    to_int!(u32, to_u32);
    to_int!(u64, to_u64);
    to_int!(u128, to_u128);
    to_int!(usize, to_usize);

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some(self.as_())
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some(self.as_())
    }
}

impl<const N: usize> Unsigned for BUint<N> {}

impl<const N: usize> Zero for BUint<N> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        Self::is_zero(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Roots;

	test_unsigned! {
		function: sqrt(a: ref &u128)
	}
	test_unsigned! {
		function: cbrt(a: ref &u128)
	}
	test_unsigned! {
		function: nth_root(u: ref &u128, n: u32),
		quickcheck_skip: n == 0
	}
}