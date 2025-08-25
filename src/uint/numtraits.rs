use super::Uint;

use crate::ExpType;
use num_integer::{Integer, Roots};

use crate::cast::CastFrom;
use crate::cast::float::ConvertFloatParts;
use crate::helpers::Bits;

crate::ints::numtraits::impls!(Uint);

macro_rules! from_float {
    // adapted from crate::cast::float::cast_uint_from_float
    ($method: ident, $float: ty) => {
        #[inline]
        fn $method(f: $float) -> Option<Self> {
            if !f.is_finite() {
                return None;
            }
            if f == 0.0 {
                return Some(Self::ZERO);
            }
            let (sign, exp, mant) = f.into_normalised_signed_parts();
            if sign {
                return None;
            }
            if exp < -1 {
                // in this case, the value is at most a half, so we round (ties to even) to zero
                return Some(Self::ZERO);
            }
            if exp == -1 {
                // exponent is -1, so value is in range [1/2, 1)
                if mant.is_power_of_two() {
                    // in this case, the value is exactly 1/2, so we round (ties to even) to zero
                    return Some(Self::ZERO);
                }
                return Some(Self::ONE);
            }

            let exp = exp as ExpType;
            if exp >= Self::BITS {
                return None;
            }
            let mant_bit_width = mant.bits();
            if exp <= mant_bit_width - 1 {
                // in this case, we have a fractional part to truncate
                Some(Self::cast_from(mant >> (mant_bit_width - 1 - exp))) // the right shift means the mantissa now has exp + 1 bits, and as we must have exp < U::BITS, the shifted mantissa is no wider than U
            } else {
                Some(Self::cast_from(mant) << (exp - (mant_bit_width - 1)))
            }
        }
    };
}

macro_rules! from_primitive {
    ($primitive: ty, $method: ident) => {
        #[inline]
        fn $method(n: $primitive) -> Option<Self> {
            Self::try_from(n).ok()   
        }
    };
}

impl<const N: usize> FromPrimitive for Uint<N> {
    from_primitive!(u8, from_u8);
    from_primitive!(u16, from_u16);
    from_primitive!(u32, from_u32);
    from_primitive!(u64, from_u64);
    from_primitive!(u128, from_u128);
    from_primitive!(usize, from_usize);
    from_primitive!(i8, from_i8);
    from_primitive!(i16, from_i16);
    from_primitive!(i32, from_i32);
    from_primitive!(i64, from_i64);
    from_primitive!(i128, from_i128);
    from_primitive!(isize, from_isize);

    // TODO: replace this with code from the cast/float module
    from_float!(from_f32, f32);
    from_float!(from_f64, f64);
}

impl<const N: usize> Integer for Uint<N> {
    #[inline]
    fn div_floor(&self, other: &Self) -> Self {
        Self::div_floor(*self, *other)
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
            a = Self::unchecked_shr_internal(a, a_tz);
            b = Self::unchecked_shr_internal(b, b_tz);
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
                return unsafe { Self::unchecked_shl_internal(b, b_tz) };
            }
            unsafe {
                a = Self::unchecked_shr_internal(a, a.trailing_zeros());
            }
        }
    }

    #[inline]
    fn lcm(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            Self::ZERO
        } else {
            (self / self.gcd(other)) * other
        }
    }

    #[inline]
    fn divides(&self, other: &Self) -> bool {
        self.is_multiple_of(other)
    }

    #[inline]
    fn is_multiple_of(&self, other: &Self) -> bool {
        if other.is_zero() {
            return self.is_zero();
        }
        (self % other).is_zero()
    }

    #[inline]
    fn is_even(&self) -> bool {
        self.digits[0] % 2 == 0
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.digits[0] % 2 == 1
    }

    #[inline]
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        Self::div_rem(*self, *rhs)
    }
}

impl<const N: usize> PrimInt for Uint<N> {
    crate::ints::numtraits::prim_int_methods!();

    #[inline]
    fn signed_shl(self, n: u32) -> Self {
        self << n
    }

    #[inline]
    fn signed_shr(self, n: u32) -> Self {
        let (u, overflow) = self.overflowing_shr_signed(n);

        if crate::OVERFLOW_CHECKS && overflow {
            panic!(crate::errors::err_msg!(
                "attempt to shift right with overflow"
            ))
        }

        u
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

/*
The `fixpoint` function and the implementation of `Roots` below are adapted from the Rust `num_bigint` library, https://docs.rs/num-bigint/latest/num_bigint/, modified under the MIT license. The changes are released under either the MIT license or the Apache License 2.0, as described in the README. See LICENSE-MIT or LICENSE-APACHE at the project root.

The appropriate copyright notice for the `num_bigint` code is given below:
Copyright (c) 2014 The Rust Project Developers

The original license file and copyright notice for `num_bigint` can be found in this project's root at licenses/LICENSE-num-bigint.
*/

impl<const N: usize> Uint<N> {
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

impl<const N: usize> Roots for Uint<N> {
    #[inline]
    fn sqrt(&self) -> Self {
        if self.is_zero() || self.is_one() {
            return *self;
        }

        #[cfg(not(test))]
        // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
        if let Some(n) = self.to_u128() {
            return Self::cast_from(n.sqrt());
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
        if self.is_zero() || self.is_one() {
            return *self;
        }

        #[cfg(not(test))]
        // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
        if let Some(n) = self.to_u128() {
            return Self::cast_from(n.cbrt());
        }
        let bits = self.bits();
        let max_bits = bits / 3 + 1;

        let guess = Self::power_of_two(max_bits);
        guess.fixpoint(max_bits, |s| {
            let q = self / (s * s);
            let t: Self = (s << 1) + q;
            t.div_rem_u64(3).0
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
                if self.is_zero() || self.is_one() {
                    return *self;
                }

                #[cfg(not(test))]
                // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
                if let Some(x) = self.to_u128() {
                    return Self::cast_from(x.nth_root(n));
                }
                let bits = self.bits();
                if bits <= n {
                    return Self::ONE;
                }

                let max_bits = bits / n + 1;

                let guess = Self::power_of_two(max_bits);
                let n_minus_1 = n - 1;

                guess.fixpoint(max_bits, |s| {
                    let q = self / s.pow(n_minus_1);
                    let mul = Self::cast_from(n_minus_1);
                    let t: Self = s * mul + q;
                    t.div_rem_unchecked(Self::cast_from(n)).0
                })
            }
        }
    }
}

impl<const N: usize> num_traits::Unsigned for Uint<N> {}

#[cfg(test)]
crate::test::test_all_widths! {
    crate::ints::numtraits::tests!(utest);
}
