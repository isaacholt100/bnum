use super::{BUint, ExpType};
use crate::digit::{self, Digit};
use crate::nightly::impl_const;
use crate::BInt;
use num_integer::{Integer, Roots};
use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl,
    CheckedShr, CheckedSub, FromPrimitive, MulAdd, MulAddAssign, Num, One, Pow, PrimInt,
    Saturating, SaturatingAdd, SaturatingMul, SaturatingSub, ToPrimitive, Unsigned, WrappingAdd,
    WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, Zero,
};

crate::int::numtraits::impls!(BUint);

//impl_const! {
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
//}

impl_const! {
    impl<const N: usize> const Integer for BUint<N> {
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
            unsafe {
                a = super::unchecked_shr(a, a_tz);
                b = super::unchecked_shr(b, b_tz);
            }

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
                    return unsafe {
                        super::unchecked_shl(b, b_tz)
                    };
                }
                unsafe {
                    a = super::unchecked_shr(a, a.trailing_zeros());
                }
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
}

impl_const! {
    impl<const N: usize> const PrimInt for BUint<N> {
        crate::int::numtraits::prim_int_methods!();

        #[inline]
        fn signed_shl(self, n: u32) -> Self {
            self << n
        }

        #[inline]
        fn signed_shr(self, n: u32) -> Self {
            (BInt::from_bits(self) >> n).to_bits()
        }

        #[inline]
        fn unsigned_shl(self, n: u32) -> Self {
            self << n
        }

        #[inline]
        fn unsigned_shr(self, n: u32) -> Self {
            self >> n
        }
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
    };
}

/*
The `fixpoint` function and the implementation of `Roots` below are adapted from the Rust `num_bigint` library: https://docs.rs/num-bigint/latest/num_bigint/ used under the MIT license.
The original license file and copyright notice for this project can be found in this project's root at licenses/LICENSE-num-bigint.
*/

impl<const N: usize> BUint<N> {
    #[inline]
    fn fixpoint<F>(mut self, max_bits: ExpType, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
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

        #[cfg(not(test))]
        // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
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

        #[cfg(not(test))]
        // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
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
            0 => panic!(crate::errors::err_msg!("attempt to calculate zeroth root")),
            1 => *self,
            2 => self.sqrt(),
            3 => self.cbrt(),
            _ => {
                check_zero_or_one!(self);

                #[cfg(not(test))]
                // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
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
	{ $($name: ident -> $int: ty), * }  => {
		$(
			#[inline]
			fn $name(&self) -> Option<$int> {
				let mut out = 0;
				let mut i = 0;
				if digit::BITS > <$int>::BITS {
					let small = self.digits[i] as $int;
					let trunc = small as digit::Digit;
					if self.digits[i] != trunc {
						return None;
					}
					out = small;
					i = 1;
				} else {
					loop {
						let shift = i << digit::BIT_SHIFT;
						if i >= N || shift >= <$int>::BITS as usize {
							break;
						}
						out |= self.digits[i] as $int << shift;
						i += 1;
					}
				}

				#[allow(unused_comparisons)]
				if out < 0 {
					return None;
				}

				while i < N {
					if self.digits[i] != 0 {
						return None;
					}
					i += 1;
				}

				Some(out)
			}
		)*
	};
}

//impl_const! {
impl<const N: usize> ToPrimitive for BUint<N> {
    to_int! {
        to_u8 -> u8,
        to_u16 -> u16,
        to_u32 -> u32,
        to_u64 -> u64,
        to_u128 -> u128,
        to_usize -> usize,

        to_i8 -> i8,
        to_i16 -> i16,
        to_i32 -> i32,
        to_i64 -> i64,
        to_i128 -> i128,
        to_isize -> isize
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
//}

impl<const N: usize> Unsigned for BUint<N> {}

#[cfg(test)]
crate::int::numtraits::tests!(utest);
