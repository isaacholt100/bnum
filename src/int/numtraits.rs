macro_rules! as_primitive_impl {
	($Int: ident; $($ty: ty), *) => {
		$(
			impl<const N: usize> AsPrimitive<$ty> for $Int<N> {
				#[inline]
				fn as_(self) -> $ty {
					crate::As::as_(self)
				}
			}
		)*
	}
}

pub(crate) use as_primitive_impl;
		
macro_rules! num_trait_impl {
	($Int: ident, $tr: ident, $method: ident, $ret: ty) => {
		impl<const N: usize> $tr for $Int<N> {
			#[inline]
			fn $method(&self, rhs: &Self) -> $ret {
				Self::$method(*self, *rhs)
			}
		}
	}
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

macro_rules! impls {
	($Int: ident) => {
		use crate::cast::CastFrom;

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

		use crate::int::numtraits::num_trait_impl;
		
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
		
		impl<const N: usize> CheckedNeg for $Int<N> {
			#[inline]
			fn checked_neg(&self) -> Option<Self> {
				Self::checked_neg(*self)
			}
		}
		
		impl<const N: usize> CheckedShl for $Int<N> {
			#[inline]
			fn checked_shl(&self, rhs: u32) -> Option<Self> {
				Self::checked_shl(*self, rhs)
			}
		}
		
		impl<const N: usize> CheckedShr for $Int<N> {
			#[inline]
			fn checked_shr(&self, rhs: u32) -> Option<Self> {
				Self::checked_shr(*self, rhs)
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
		
		crate::int::numtraits::as_primitive_impl!($Int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
		
		crate::int::numtraits::as_bigint_impl!([u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128, char, bool] as $Int);
		
		impl<const N: usize> AsPrimitive<$Int<N>> for f32 {
			#[inline]
			fn as_(self) -> $Int<N> {
				$Int::cast_from(self)
			}
		}
		
		impl<const N: usize> AsPrimitive<$Int<N>> for f64 {
			#[inline]
			fn as_(self) -> $Int<N> {
				$Int::cast_from(self)
			}
		}
		
		impl<const N: usize, const M: usize> AsPrimitive<crate::BUint<M>> for $Int<N> {
			#[inline]
			fn as_(self) -> crate::BUint<M> {
				crate::BUint::<M>::cast_from(self)
			}
		}
		
		impl<const N: usize, const M: usize> AsPrimitive<crate::BInt<M>> for $Int<N> {
			#[inline]
			fn as_(self) -> crate::BInt<M> {
				crate::BInt::<M>::cast_from(self)
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
		
		use crate::errors::ParseIntError;
		
		impl<const N: usize> Num for $Int<N> {
			type FromStrRadixErr = ParseIntError;
		
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
				self == &Self::ONE
			}
		}
		
		impl<const N: usize> Zero for $Int<N> {
			#[inline]
			fn zero() -> Self {
				Self::ZERO
			}
		
			#[inline]
			fn is_zero(&self) -> bool {
				Self::is_zero(self)
			}
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
		crate::int::numtraits::prim_int_method! {
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
		}

		#[cfg(has_leading_trailing_ones)]
		#[inline]
		fn leading_ones(self) -> u32 {
			Self::leading_ones(self)
		}
	
		#[cfg(has_leading_trailing_ones)]
		#[inline]
		fn trailing_ones(self) -> u32 {
			Self::trailing_ones(self)
		}
	
		#[cfg(has_reverse_bits)]
		#[inline]
		fn reverse_bits(self) -> Self {
			Self::reverse_bits(self)
		}
	};
}

pub(crate) use prim_int_methods;

#[allow(unused)]
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

#[allow(unused)]
pub(crate) use test_to_primitive;

#[allow(unused)]
macro_rules! test_from_primitive {
	($int: ty; $($prim: ty), *) => {
		paste::paste! {
			$(
				test_bignum! {
					function: <$int>::[<from_ $prim>](u: $prim)
				}
			)*
		}
	};
}

#[allow(unused)]
pub(crate) use test_from_primitive;

macro_rules! tests {
	($int: ty) => {
		#[cfg(test)]
		mod tests {
			use super::*;
			use num_traits::PrimInt;
			use crate::test::{test_bignum, types::*};

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

			use crate::int::numtraits::{test_to_primitive, test_from_primitive};

			test_to_primitive!($int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

			test_from_primitive!($int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

			test_bignum! {
				function: <$int as Integer>::gcd(a: ref &$int, b: ref &$int),
				skip: {
					#[allow(unused_comparisons)]
					let cond = <$int>::MIN < 0 && (a == <$int>::MIN && (b == <$int>::MIN || b == 0)) || (b == <$int>::MIN && (a == <$int>::MIN || a == 0));
					cond
				}
			}
			test_bignum! {
				function: <$int as Integer>::is_multiple_of(a: ref &$int, b: ref &$int),
				skip: b == 0
			}
			test_bignum! {
				function: <$int as Integer>::is_even(a: ref &$int)
			}
			test_bignum! {
				function: <$int as Integer>::is_odd(a: ref &$int)
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
		}
	};
}

pub(crate) use tests;