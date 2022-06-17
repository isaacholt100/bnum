use super::{BUint, ExpType};
use num_traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, SaturatingAdd, SaturatingMul, SaturatingSub, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, ToPrimitive, Unsigned, Zero, Pow, Saturating, AsPrimitive};
use num_integer::{Integer, Roots};
use crate::digit::{self, Digit};

crate::int::numtraits::impls!(BUint);

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
		// Paul E. Black, "binary GCD", in Dictionary of Algorithms and Data Structures [online], Paul E. Black, ed. 2 November 2020. (accessed 15th June 2022) Available from: https://www.nist.gov/dads/HTML/binaryGCD.html
		// https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Implementation

		let (mut a, mut b) = (*self, *other);
		if a.is_zero() {
			return b;
		}
		if b.is_zero() {
			return a;
		}
		let mut a_tz = a.trailing_zeros();
		let mut b_tz = b.trailing_zeros();
		// Normalise `a` and `b` so that both of them has no leading zeros, so both must be odd.
		a = super::unchecked_shr(a, a_tz);
		b = super::unchecked_shr(b, b_tz);

		if b_tz > a_tz {
			// Ensure `a_tz >= b_tz`
			core::mem::swap(&mut a_tz, &mut b_tz);
		}
		loop {
			if a < b {
				// Ensure `a >= b`
				core::mem::swap(&mut a, &mut b);
			}
			a -= b;
			if a.is_zero() {
				return super::unchecked_shl(b, b_tz);
			}
			a = super::unchecked_shr(a, a.trailing_zeros());
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
		// credit num_bigint source code
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
		/*if self.is_zero() {
			return Self::ZERO;
		}
		let m = *self;
		let mut u = m;
		let mut s: Self;
		let mut t: Self;

		loop {
			s = u;
			let d = m / s;
			u = super::unchecked_shr(s, 1) + super::unchecked_shr(d, 1);
			if s.is_odd() && d.is_odd() {
				u += Self::ONE;
			}
			if u >= s {
				return s;
			}
		}*/
		// credit num_bigint source code
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
		// credit num_bigint source code
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
		// credit num_bigint source code
        match n {
            0 => panic!(crate::error::err_msg!("attempt to calculate zeroth root")),
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

crate::int::numtraits::tests!(u128);