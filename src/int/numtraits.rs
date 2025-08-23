use super::Int;
use crate::Uint;

macro_rules! from_float {
    ($method: ident, $float: ty) => {
        #[inline]
        fn $method(f: $float) -> Option<Self> {
            if f.is_sign_negative() {
                let i = Self::from_bits(Uint::$method(-f)?);
                if i == Self::MIN {
                    Some(Self::MIN)
                } else if i.is_negative() {
                    None
                } else {
                    Some(-i)
                }
            } else {
                let i = Self::from_bits(Uint::$method(f)?);
                if i.is_negative() { None } else { Some(i) }
            }
        }
    };
}

use crate::ExpType;
use crate::errors;
use num_integer::{Integer, Roots};
use num_traits::Signed;

use crate::cast::CastFrom;

crate::ints::numtraits::impls!(Int);

macro_rules! from_primitive {
    ($primitive: ty, $method: ident) => {
        #[inline]
        fn $method(n: $primitive) -> Option<Self> {
            Self::try_from(n).ok()
        }
    };
}

impl<const N: usize> FromPrimitive for Int<N> {
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

    from_float!(from_f32, f32);
    from_float!(from_f64, f64);
}

impl<const N: usize> Integer for Int<N> {
    #[inline]
    fn div_floor(&self, other: &Self) -> Self {
        Self::div_floor(*self, *other)
    }

    #[inline]
    fn mod_floor(&self, other: &Self) -> Self {
        let rem = self % other;
        if rem.is_zero() {
            return rem;
        }
        if rem.is_negative() != other.is_negative() {
            rem + other
        } else {
            rem
        }
    }

    #[inline]
    fn gcd(&self, other: &Self) -> Self {
        let gcd = self.unsigned_abs().gcd(&other.unsigned_abs());
        let out = Self::from_bits(gcd);
        out.abs()
    }

    #[inline]
    fn lcm(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            Self::ZERO
        } else {
            (self.div_floor(&self.gcd(other)) * *other).abs()
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
        self.bits.is_even()
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.bits.is_odd()
    }

    #[inline]
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        (self / other, self % other)
    }
}

impl<const N: usize> PrimInt for Int<N> {
    crate::ints::numtraits::prim_int_methods!();

    #[inline]
    fn signed_shl(self, n: u32) -> Self {
        self << n
    }

    #[inline]
    fn signed_shr(self, n: u32) -> Self {
        self >> n
    }

    #[inline]
    fn unsigned_shl(self, n: u32) -> Self {
        self << n
    }

    #[inline]
    fn unsigned_shr(self, n: u32) -> Self {
        Self::from_bits(self.to_bits() >> n)
    }
}

impl<const N: usize> Roots for Int<N> {
    #[inline]
    fn sqrt(&self) -> Self {
        if self.is_negative() {
            panic!(crate::errors::err_msg!("imaginary square root"))
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
                panic!(crate::errors::err_msg!("attempt to calculate zeroth root"));
            }
            if n == 1 {
                return *self;
            }
            if n.is_even() {
                panic!("{} imaginary root degree of {}", errors::err_prefix!(), n)
            } else {
                let out = Self::from_bits(self.unsigned_abs().nth_root(n));
                out.wrapping_neg()
            }
        } else {
            Self::from_bits(self.bits.nth_root(n))
        }
    }
}

impl<const N: usize> Signed for Int<N> {
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

#[cfg(test)]
crate::test::test_all_widths! {
    crate::ints::numtraits::tests!(itest);
}
