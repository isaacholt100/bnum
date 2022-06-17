use super::BInt;
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, SaturatingAdd, SaturatingMul, SaturatingSub, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, ToPrimitive, Signed, Zero, Pow, Saturating, AsPrimitive};
use num_integer::{Integer, Roots};
use core::convert::TryFrom;
use crate::{ExpType, BUint};
use crate::digit::{self, Digit};
use crate::error;

crate::int::numtraits::impls!(BInt);

macro_rules! from_int {
	($int: ty, $name: ident) => {
		#[inline]
		fn $name(n: $int) -> Option<Self> {
			const INT_BITS: usize = <$int>::BITS as usize;
			let initial_digit = if n.is_negative() {
				Digit::MAX
			} else {
				Digit::MIN
			};
			let mut out = Self::from_bits(BUint::from_digits([initial_digit; N]));
			let mut i = 0;
			while i << digit::BIT_SHIFT < INT_BITS {
				let d = (n >> (i << digit::BIT_SHIFT)) as Digit;
				if d != initial_digit {
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
	};
}

macro_rules! from_uint {
	($uint: ty, $name: ident) => {
		#[inline]
		fn $name(n: $uint) -> Option<Self> {
			const UINT_BITS: usize = <$uint>::BITS as usize;
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
	};
}

impl<const N: usize> FromPrimitive for BInt<N> {
	from_uint!(u8, from_u8);
    from_uint!(u16, from_u16);
    from_uint!(u32, from_u32);
    from_uint!(u64, from_u64);
    from_uint!(u128, from_u128);
    from_uint!(usize, from_usize);
    from_int!(i8, from_i8);
    from_int!(i16, from_i16);
    from_int!(i32, from_i32);
    from_int!(i64, from_i64);
    from_int!(i128, from_i128);
    from_int!(isize, from_isize);

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
        let gcd = self.unsigned_abs().gcd(&other.unsigned_abs());
		let out = Self::from_bits(gcd);
		if out == Self::MIN {
			panic!("got Self::MIN {:0b} and {:0b}", &self, &other)
		}
		out.abs()
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
			if n == 0 {
				panic!(crate::error::err_msg!("attempt to calculate zeroth root"));
			}
			if n == 1 {
				return *self;
			}
            if n.is_even() {
                panic!("{} imaginary root degree of {}", error::err_prefix!(), n)
            } else {
                let out = Self::from_bits(self.unsigned_abs().nth_root(n));
                out.wrapping_neg()
            }
        } else {
            Self::from_bits(self.bits.nth_root(n))
        }
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
        self.signed_digit().is_negative()
    }
}

crate::int::numtraits::tests!(i128);