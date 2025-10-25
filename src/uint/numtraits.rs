use super::{Integer, Uint};

use crate::ExpType;
use crate::Int;
use num_integer::Roots;

use crate::cast::CastFrom;
use crate::cast::float::ConvertFloatParts;
use crate::helpers::Bits;

use num_traits::ops::overflowing::{OverflowingAdd, OverflowingMul, OverflowingSub};
use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedEuclid, CheckedMul, CheckedNeg,
    CheckedRem, CheckedShl, CheckedShr, CheckedSub, ConstOne, ConstZero, Euclid, FromBytes,
    FromPrimitive, MulAdd, MulAddAssign, Num, One, Pow, PrimInt, Saturating, SaturatingAdd,
    SaturatingMul, SaturatingSub, Signed, ToBytes, ToPrimitive, Unsigned, WrappingAdd, WrappingMul,
    WrappingNeg, WrappingShl, WrappingShr, WrappingSub, Zero,
};

impl<const S: bool, const N: usize> Bounded for Integer<S, N> {
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
        impl<const S: bool, const N: usize> $tr for Integer<S, N> {
            #[inline]
            fn $method(&self, rhs: &Self) -> $ret {
                Self::$method(*self, *rhs)
            }
        }
    };
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

num_trait_impl!(OverflowingAdd, overflowing_add, (Self, bool));
num_trait_impl!(OverflowingSub, overflowing_sub, (Self, bool));
num_trait_impl!(OverflowingMul, overflowing_mul, (Self, bool));

impl<const S: bool, const N: usize> CheckedNeg for Integer<S, N> {
    #[inline]
    fn checked_neg(&self) -> Option<Self> {
        Self::checked_neg(*self)
    }
}

impl<const S: bool, const N: usize> CheckedShl for Integer<S, N> {
    #[inline]
    fn checked_shl(&self, rhs: ExpType) -> Option<Self> {
        Self::checked_shl(*self, rhs)
    }
}

impl<const S: bool, const N: usize> CheckedShr for Integer<S, N> {
    #[inline]
    fn checked_shr(&self, rhs: ExpType) -> Option<Self> {
        Self::checked_shr(*self, rhs)
    }
}

impl<const S: bool, const N: usize> CheckedEuclid for Integer<S, N> {
    #[inline]
    fn checked_div_euclid(&self, rhs: &Self) -> Option<Self> {
        Self::checked_div_euclid(*self, *rhs)
    }

    #[inline]
    fn checked_rem_euclid(&self, rhs: &Self) -> Option<Self> {
        Self::checked_rem_euclid(*self, *rhs)
    }
}

impl<const S: bool, const N: usize> Euclid for Integer<S, N> {
    #[inline]
    fn div_euclid(&self, rhs: &Self) -> Self {
        Self::div_euclid(*self, *rhs)
    }

    #[inline]
    fn rem_euclid(&self, rhs: &Self) -> Self {
        Self::rem_euclid(*self, *rhs)
    }
}

impl<const S: bool, const N: usize> WrappingNeg for Integer<S, N> {
    #[inline]
    fn wrapping_neg(&self) -> Self {
        Self::wrapping_neg(*self)
    }
}

impl<const S: bool, const N: usize> WrappingShl for Integer<S, N> {
    #[inline]
    fn wrapping_shl(&self, rhs: ExpType) -> Self {
        Self::wrapping_shl(*self, rhs)
    }
}

impl<const S: bool, const N: usize> WrappingShr for Integer<S, N> {
    #[inline]
    fn wrapping_shr(&self, rhs: ExpType) -> Self {
        Self::wrapping_shr(*self, rhs)
    }
}

impl<const S: bool, const N: usize> Pow<ExpType> for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn pow(self, exp: ExpType) -> Self {
        Self::pow(self, exp)
    }
}

impl<const S: bool, const N: usize> Saturating for Integer<S, N> {
    #[inline]
    fn saturating_add(self, rhs: Self) -> Self {
        Self::saturating_add(self, rhs)
    }

    #[inline]
    fn saturating_sub(self, rhs: Self) -> Self {
        Self::saturating_sub(self, rhs)
    }
}

macro_rules! to_primitive_int {
    ($primitive: ty, $method: ident) => {
        #[inline]
        fn $method(&self) -> Option<$primitive> {
            (*self).try_into().ok()
        }
    };
}

impl<const S: bool, const N: usize> ToPrimitive for Integer<S, N> {
    to_primitive_int!(u8, to_u8);
    to_primitive_int!(u16, to_u16);
    to_primitive_int!(u32, to_u32);
    to_primitive_int!(u64, to_u64);
    to_primitive_int!(u128, to_u128);
    to_primitive_int!(usize, to_usize);
    to_primitive_int!(i8, to_i8);
    to_primitive_int!(i16, to_i16);
    to_primitive_int!(i32, to_i32);
    to_primitive_int!(i64, to_i64);
    to_primitive_int!(i128, to_i128);
    to_primitive_int!(isize, to_isize);

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some(self.as_())
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some(self.as_())
    }
}

macro_rules! impl_as_primitive_for_integer {
    ($($ty: ty), *) => {
        $(
            impl<const S: bool, const N: usize> AsPrimitive<$ty> for Integer<S, N> {
                #[inline]
                fn as_(self) -> $ty {
                    <$ty>::cast_from(self)
                }
            }
        )*
    }
}

impl_as_primitive_for_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

macro_rules! impl_as_primitive_integer_for_primitive {
    ($($ty: ty), *) => {
        $(
            impl<const S: bool, const N: usize> AsPrimitive<Integer<S, N>> for $ty {
                #[inline]
                fn as_(self) -> Integer<S, N> {
                    Integer::cast_from(self)
                }
            }
        )*
    }
}

impl_as_primitive_integer_for_primitive!(
    u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128, f32, f64, char, bool
);

impl<const S: bool, const N: usize, const R: bool, const M: usize> AsPrimitive<Integer<R, M>>
    for Integer<S, N>
{
    #[inline]
    fn as_(self) -> Integer<R, M> {
        Integer::cast_from(self)
    }
}

impl<const S: bool, const N: usize> FromBytes for Integer<S, N> {
    type Bytes = [u8; N];

    #[inline]
    fn from_be_bytes(bytes: &[u8; N]) -> Self {
        Self::from_be_bytes(*bytes)
    }

    #[inline]
    fn from_le_bytes(bytes: &[u8; N]) -> Self {
        Self::from_le_bytes(*bytes)
    }
}

impl<const S: bool, const N: usize> ToBytes for Integer<S, N> {
    type Bytes = [u8; N];

    #[inline]
    fn to_be_bytes(&self) -> [u8; N] {
        Self::to_be_bytes(*self)
    }

    #[inline]
    fn to_le_bytes(&self) -> [u8; N] {
        Self::to_le_bytes(*self)
    }
}

impl<const S: bool, const N: usize> MulAdd for Integer<S, N> {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl<const S: bool, const N: usize> MulAddAssign for Integer<S, N> {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

impl<const S: bool, const N: usize> Num for Integer<S, N> {
    type FromStrRadixErr = crate::errors::ParseIntError;

    #[inline]
    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Self::from_str_radix(string, radix)
    }
}

impl<const S: bool, const N: usize> num_traits::NumCast for Integer<S, N> {
    fn from<T: ToPrimitive>(_n: T) -> Option<Self> {
        panic!(concat!(
            crate::errors::err_prefix!(),
            "`num_traits::NumCast` trait is not supported for ",
            stringify!($Int)
        ))
    }
}

impl<const S: bool, const N: usize> One for Integer<S, N> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn is_one(&self) -> bool {
        Self::is_one(&self)
    }
}

impl<const S: bool, const N: usize> ConstOne for Integer<S, N> {
    const ONE: Self = Self::ONE;
}

impl<const S: bool, const N: usize> Zero for Integer<S, N> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        Self::is_zero(&self)
    }
}

impl<const S: bool, const N: usize> ConstZero for Integer<S, N> {
    const ZERO: Self = Self::ZERO;
}

macro_rules! from_primitive_float {
    // adapted from crate::cast::float::cast_uint_from_float
    ($method: ident, $float: ty, $signed: ident) => {
        #[inline]
        fn $method(f: $float) -> Option<Self> {
            if $signed {
                if f.is_sign_negative() {
                    let i = Uint::$method(-f)?.force_sign();
                    return if i == Self::MIN {
                        Some(Self::MIN)
                    } else if i.is_negative_internal() {
                        None
                    } else {
                        Some(i.wrapping_neg())
                    };
                } else {
                    let i = Uint::$method(f)?.force_sign();
                    return if i.is_negative_internal() {
                        None
                    } else {
                        Some(i)
                    };
                }
            }
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

macro_rules! from_primitive_int {
    ($primitive: ty, $method: ident) => {
        #[inline]
        fn $method(n: $primitive) -> Option<Self> {
            Self::try_from(n).ok()
        }
    };
}

impl<const S: bool, const N: usize> FromPrimitive for Integer<S, N> {
    from_primitive_int!(u8, from_u8);
    from_primitive_int!(u16, from_u16);
    from_primitive_int!(u32, from_u32);
    from_primitive_int!(u64, from_u64);
    from_primitive_int!(u128, from_u128);
    from_primitive_int!(usize, from_usize);
    from_primitive_int!(i8, from_i8);
    from_primitive_int!(i16, from_i16);
    from_primitive_int!(i32, from_i32);
    from_primitive_int!(i64, from_i64);
    from_primitive_int!(i128, from_i128);
    from_primitive_int!(isize, from_isize);

    // TODO: replace this with code from the cast/float module
    from_primitive_float!(from_f32, f32, S);
    from_primitive_float!(from_f64, f64, S);
}

impl<const S: bool, const N: usize> num_integer::Integer for Integer<S, N> {
    #[inline]
    fn div_floor(&self, other: &Self) -> Self {
        Self::div_floor(*self, *other)
    }

    #[inline]
    fn mod_floor(&self, other: &Self) -> Self {
        let rem = self % other;
        if S && rem.is_zero() {
            return rem;
        }
        if rem.is_negative_internal() != other.is_negative_internal() {
            rem + other
        } else {
            rem
        }
    }

    #[inline]
    fn gcd(&self, other: &Self) -> Self {
        // Paul E. Black, "binary GCD", in Dictionary of Algorithms and Data Structures [online], Paul E. Black, ed. 2 November 2020. (accessed 15th June 2022) Available from: https://www.nist.gov/dads/HTML/binaryGCD.html
        // https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Implementation
        if S {
            let gcd = self
                .unsigned_abs_internal()
                .gcd(&other.unsigned_abs_internal())
                .force_sign();
            if gcd == Self::MIN {
                return crate::Int::MIN.force_sign();
            }
            return gcd;
        }
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
            a = a.unchecked_shr_pad_internal::<false>(a_tz);
            b = b.unchecked_shr_pad_internal::<false>(b_tz);
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
                a = a.unchecked_shr_pad_internal::<false>(a.trailing_zeros());
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
        self.bytes[0] % 2 == 0
    }

    #[inline]
    fn is_odd(&self) -> bool {
        self.bytes[0] % 2 == 1
    }

    #[inline]
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        if self.is_division_overflow(rhs) {
            panic!(crate::errors::err_msg!("attempt to divide with overflow"));
        }
        if rhs.is_zero() {
            panic!(crate::errors::err_msg!("attempt to divide by zero"));
        }
        self.div_rem_unchecked(*rhs)
    }
}

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

impl<const S: bool, const N: usize> PrimInt for Integer<S, N> {
    #[inline]
    fn from_be(x: Self) -> Self {
        if cfg!(target_endian = "big") {
            x
        } else {
            x.swap_bytes()
        }
    }

    #[inline]
    fn from_le(x: Self) -> Self {
        if cfg!(target_endian = "little") {
            x
        } else {
            x.swap_bytes()
        }
    }

    #[inline]
    fn to_be(self) -> Self {
        if cfg!(target_endian = "big") {
            self
        } else {
            self.swap_bytes()
        }
    }

    #[inline]
    fn to_le(self) -> Self {
        if cfg!(target_endian = "little") {
            self
        } else {
            self.swap_bytes()
        }
    }

    prim_int_method! {
        fn count_ones(self) -> u32;
        fn count_zeros(self) -> u32;
        fn leading_zeros(self) -> u32;
        fn trailing_zeros(self) -> u32;
        fn rotate_left(self, n: u32) -> Self;
        fn rotate_right(self, n: u32) -> Self;
        fn swap_bytes(self) -> Self;
        fn pow(self, exp: u32) -> Self;
        fn leading_ones(self) -> u32;
        fn trailing_ones(self) -> u32;
        fn reverse_bits(self) -> Self;
    }

    #[inline]
    fn signed_shl(self, n: u32) -> Self {
        self << n
    }

    #[inline]
    fn signed_shr(self, n: u32) -> Self {
        (self.force_sign::<true>() >> n).force_sign()
    }

    #[inline]
    fn unsigned_shl(self, n: u32) -> Self {
        self << n
    }

    #[inline]
    fn unsigned_shr(self, n: u32) -> Self {
        (self.force_sign::<false>() >> n).force_sign()
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

impl<const S: bool, const N: usize> Roots for Integer<S, N> {
    #[inline]
    fn sqrt(&self) -> Self {
        if self.is_negative_internal() {
            panic!(crate::errors::err_msg!("imaginary square root"))
        }
        if self.is_zero() || self.is_one() {
            return *self;
        }

        #[cfg(not(test))]
        // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
        if let Some(n) = self.to_u128() {
            return Self::cast_from(n.sqrt());
        }
        let n = self.force_sign::<false>();
        let bits = n.bits();
        let max_bits = bits / 2 + 1;

        let guess = Uint::power_of_two(max_bits);
        guess
            .fixpoint(max_bits, |s| {
                let q = n / s;
                let t = s + q;
                t >> 1
            })
            .force_sign()
    }

    #[inline]
    fn cbrt(&self) -> Self {
        if self.is_negative_internal() {
            let out = self.unsigned_abs_internal().cbrt();
            return out.wrapping_neg().force_sign();
        }
        if self.is_zero() || self.is_one() {
            return *self;
        }

        #[cfg(not(test))]
        // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
        if let Some(n) = self.to_u128() {
            return Self::cast_from(n.cbrt());
        }
        let n = self.force_sign::<false>();
        let bits = n.bits();
        let max_bits = bits / 3 + 1;

        let guess = Uint::power_of_two(max_bits);
        guess
            .fixpoint(max_bits, |s| {
                let q = n / (s * s);
                let t: Uint<N> = (s << 1) + q;
                t.div_rem_u64(3).0
            })
            .force_sign()
    }

    #[inline]
    fn nth_root(&self, n: u32) -> Self {
        match n {
            0 => panic!(crate::errors::err_msg!("attempt to calculate zeroth root")),
            1 => *self,
            2 => self.sqrt(),
            3 => self.cbrt(),
            _ => {
                if self.is_negative_internal() {
                    let out = self.unsigned_abs_internal().nth_root(n);
                    return out.wrapping_neg().force_sign();
                }
                if self.is_zero() || self.is_one() {
                    return *self;
                }

                #[cfg(not(test))]
                // disable this when testing as this condition will always be true when testing against primitives, so the rest of the algorithm wouldn't be tested
                if let Some(x) = self.to_u128() {
                    return Self::cast_from(x.nth_root(n));
                }
                let num = self.force_sign::<false>();
                let bits = num.bits();
                if bits <= n {
                    return Self::ONE;
                }

                let max_bits = bits / n + 1;

                let guess = Uint::power_of_two(max_bits);
                let n_minus_1 = n - 1;

                guess
                    .fixpoint(max_bits, |s| {
                        let q = num / s.pow(n_minus_1);
                        let mul = Uint::cast_from(n_minus_1);
                        let t = s * mul + q;
                        t.div_rem_unchecked(Uint::cast_from(n)).0
                    })
                    .force_sign()
            }
        }
    }
}

impl<const N: usize> Unsigned for Uint<N> {}

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
        self.is_negative_internal()
    }
}

#[cfg(test)]
mod tests {
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

    macro_rules! test_from_primitive {
        ($int: ty; $($prim: ty), *) => {
            paste::paste! {
                $(
                    test_bignum! {
                        function: <$int>::[<from_ $prim>](u: $prim),
                        cases: [
                            (<$int>::MIN as $prim)
                        ]
                    }
                )*
            }
        };
    }

    use super::*;
    use crate::test::{TestConvert, test_bignum};

    crate::test::test_all! {
        testing integers;

        crate::test::test_into! {
            function: <stest as AsPrimitive>::as_,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64)
        }

        test_bignum! {
            function: <stest as CheckedAdd>::checked_add(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as CheckedSub>::checked_sub(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as CheckedMul>::checked_mul(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as CheckedDiv>::checked_div(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as CheckedRem>::checked_rem(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as CheckedNeg>::checked_neg(a: ref &stest)
        }
        test_bignum! {
            function: <stest as CheckedShl>::checked_shl(a: ref &stest, b: u8)
        }
        test_bignum! {
            function: <stest as CheckedShr>::checked_shr(a: ref &stest, b: u8)
        }
        test_bignum! {
            function: <stest as CheckedEuclid>::checked_div_euclid(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as CheckedEuclid>::checked_rem_euclid(a: ref &stest, b: ref &stest)
        }

        test_bignum! {
            function: <stest as Euclid>::div_euclid(a: ref &stest, b: ref &stest),
            skip: a.checked_div_euclid(b).is_none()
        }
        test_bignum! {
            function: <stest as Euclid>::rem_euclid(a: ref &stest, b: ref &stest),
            skip: a.checked_rem_euclid(b).is_none()
        }

        test_bignum! {
            function: <stest as SaturatingAdd>::saturating_add(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as SaturatingSub>::saturating_sub(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as SaturatingMul>::saturating_mul(a: ref &stest, b: ref &stest)
        }

        test_bignum! {
            function: <stest as Saturating>::saturating_add(a: stest, b: stest)
        }
        test_bignum! {
            function: <stest as Saturating>::saturating_sub(a: stest, b: stest)
        }

        test_bignum! {
            function: <stest as WrappingAdd>::wrapping_add(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as OverflowingAdd>::overflowing_add(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as WrappingSub>::wrapping_sub(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as OverflowingSub>::overflowing_sub(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as WrappingMul>::wrapping_mul(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as OverflowingMul>::overflowing_mul(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as WrappingNeg>::wrapping_neg(a: ref &stest)
        }
        test_bignum! {
            function: <stest as WrappingShl>::wrapping_shl(a: ref &stest, b: u16)
        }
        test_bignum! {
            function: <stest as WrappingShr>::wrapping_shr(a: ref &stest, b: u16)
        }

        test_bignum! {
            function: <stest as One>::is_one(a: ref &stest)
        }
        test_bignum! {
            function: <stest as Zero>::is_zero(a: ref &stest)
        }

        #[test]
        fn one() {
            assert_eq!(stest::one(), TestConvert::into(STEST::one()));
        }

        #[test]
        fn zero() {
            assert_eq!(stest::zero(), TestConvert::into(STEST::zero()));
        }

        #[test]
        fn min_value() {
            assert_eq!(stest::min_value(), TestConvert::into(STEST::min_value()));
        }

        #[test]
        fn max_value() {
            assert_eq!(stest::max_value(), TestConvert::into(STest::max_value()));
        }

        test_bignum! {
            function: <stest as MulAdd>::mul_add(a: stest, b: stest, c: stest),
            skip: a.checked_mul(b).map(|d| d.checked_add(c)).flatten().is_none()
        }

        test_bignum! {
            function: <stest>::sqrt(a: ref &stest),
            skip: {
                #[allow(unused_comparisons)]
                let cond = a < 0;

                cond
            }
        }
        test_bignum! {
            function: <stest>::cbrt(a: ref &stest)
        }
        test_bignum! {
            function: <stest>::nth_root(a: ref &stest, n: u32),
            skip: n == 0 || {
                #[allow(unused_comparisons)]
                let cond = a < 0;

                n % 2 == 0 && cond || (n == 1 && cond && a == <stest>::MIN) // second condition is due to an error in the num_integer crate, which incorrectly panics when calculating the first root of i128::MIN
            }
        }

        test_to_primitive!($int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        test_from_primitive!($int; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        test_bignum! {
            function: <$int as Integer>::div_floor(a: ref &$int, b: ref &$int),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <$int as Integer>::mod_floor(a: ref &$int, b: ref &$int),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <$int as Integer>::lcm(a: ref &$int, b: ref &$int),
            skip: {
                #[allow(unused_comparisons)]
                let cond = a.checked_mul(b).is_none() || (a < 0 && a == <$int>::MIN) || (b < 0 && b == <$int>::MIN); // lcm(a, b) <= a * b
                cond
            },
            cases: [(ref &(1 as $int), ref &(-1i8 as $int))]
        }
        test_bignum! {
            function: <$int as Integer>::gcd(a: ref &$int, b: ref &$int),
            skip: {
                #[allow(unused_comparisons)]
                let cond = <$int>::MIN < 0 && (a == <$int>::MIN && (b == <$int>::MIN || b == 0)) || (b == <$int>::MIN && (a == <$int>::MIN || a == 0));
                cond
            }
        }
        test_bignum! {
            function: <$int as Integer>::is_multiple_of(a: ref &$int, b: ref &$int)
        }
        test_bignum! {
            function: <$int as Integer>::is_even(a: ref &$int)
        }
        test_bignum! {
            function: <$int as Integer>::is_odd(a: ref &$int)
        }
        test_bignum! {
            function: <$int as Integer>::div_rem(a: ref &$int, b: ref &$int),
            skip: b.is_zero()
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

    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <stest as Signed>::abs(a: ref &stest)
        }
        test_bignum! {
            function: <stest as Signed>::abs_sub(a: ref &stest, b: ref &stest)
        }
        test_bignum! {
            function: <stest as Signed>::signum(a: ref &stest)
        }
        test_bignum! {
            function: <stest as Signed>::is_positive(a: ref &stest)
        }
        test_bignum! {
            function: <stest as Signed>::is_negative(a: ref &stest)
        }
    }
}
