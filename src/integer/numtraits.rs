use super::{Integer, Uint};

use crate::Exponent;
use crate::Int;
use num_integer::{Roots, Integer as IntegerTrait};

use crate::cast::CastFrom;

use num_traits::ops::overflowing::{OverflowingAdd, OverflowingMul, OverflowingSub};
use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedEuclid, CheckedMul, CheckedNeg,
    CheckedRem, CheckedShl, CheckedShr, CheckedSub, ConstOne, ConstZero, Euclid, FromBytes,
    FromPrimitive, MulAdd, MulAddAssign, Num, One, Pow, PrimInt, Saturating, SaturatingAdd,
    SaturatingMul, SaturatingSub, Signed, ToBytes, ToPrimitive, Unsigned, WrappingAdd, WrappingMul,
    WrappingNeg, WrappingShl, WrappingShr, WrappingSub, Zero,
};

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Bounded for Integer<S, N, B, OM> {
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
        impl<const S: bool, const N: usize, const B: usize, const OM: u8> $tr for Integer<S, N, B, OM> {
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

impl<const S: bool, const N: usize, const B: usize, const OM: u8> CheckedNeg for Integer<S, N, B, OM> {
    #[inline]
    fn checked_neg(&self) -> Option<Self> {
        Self::checked_neg(*self)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> CheckedShl for Integer<S, N, B, OM> {
    #[inline]
    fn checked_shl(&self, rhs: Exponent) -> Option<Self> {
        Self::checked_shl(*self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> CheckedShr for Integer<S, N, B, OM> {
    #[inline]
    fn checked_shr(&self, rhs: Exponent) -> Option<Self> {
        Self::checked_shr(*self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> CheckedEuclid for Integer<S, N, B, OM> {
    #[inline]
    fn checked_div_euclid(&self, rhs: &Self) -> Option<Self> {
        Self::checked_div_euclid(*self, *rhs)
    }

    #[inline]
    fn checked_rem_euclid(&self, rhs: &Self) -> Option<Self> {
        Self::checked_rem_euclid(*self, *rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Euclid for Integer<S, N, B, OM> {
    #[inline]
    fn div_euclid(&self, rhs: &Self) -> Self {
        Self::div_euclid(*self, *rhs)
    }

    #[inline]
    fn rem_euclid(&self, rhs: &Self) -> Self {
        Self::rem_euclid(*self, *rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> WrappingNeg for Integer<S, N, B, OM> {
    #[inline]
    fn wrapping_neg(&self) -> Self {
        Self::wrapping_neg(*self)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> WrappingShl for Integer<S, N, B, OM> {
    #[inline]
    fn wrapping_shl(&self, rhs: Exponent) -> Self {
        Self::wrapping_shl(*self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> WrappingShr for Integer<S, N, B, OM> {
    #[inline]
    fn wrapping_shr(&self, rhs: Exponent) -> Self {
        Self::wrapping_shr(*self, rhs)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Pow<Exponent> for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn pow(self, exp: Exponent) -> Self {
        Self::pow(self, exp)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Saturating for Integer<S, N, B, OM> {
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

impl<const S: bool, const N: usize, const B: usize, const OM: u8> ToPrimitive for Integer<S, N, B, OM> {
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
            impl<const S: bool, const N: usize, const B: usize, const OM: u8> AsPrimitive<$ty> for Integer<S, N, B, OM> {
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
            impl<const S: bool, const N: usize, const B: usize, const OM: u8> AsPrimitive<Integer<S, N, B, OM>> for $ty {
                #[inline]
                fn as_(self) -> Integer<S, N, B, OM> {
                    Integer::cast_from(self)
                }
            }
        )*
    }
}

impl_as_primitive_integer_for_primitive!(
    u8, u16, u32, usize, u64, u128, i8, i16, i32, isize, i64, i128, f32, f64, char, bool
);

impl<const S: bool, const N: usize, const B: usize, const R: bool, const M: usize, const A: usize, const OM: u8> AsPrimitive<Integer<R, M, A, OM>>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn as_(self) -> Integer<R, M, A, OM> {
        Integer::cast_from(self)
    }
}

impl<const S: bool, const N: usize, const OM: u8> FromBytes for Integer<S, N, 0, OM> {
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

impl<const S: bool, const N: usize, const OM: u8> ToBytes for Integer<S, N, 0, OM> {
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

impl<const S: bool, const N: usize, const B: usize, const OM: u8> MulAdd for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        (self * a) + b
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> MulAddAssign for Integer<S, N, B, OM> {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Num for Integer<S, N, B, OM> {
    type FromStrRadixErr = crate::errors::ParseIntError;

    #[inline]
    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Self::from_str_radix(string, radix)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> num_traits::NumCast for Integer<S, N, B, OM> {
    fn from<T: ToPrimitive>(_n: T) -> Option<Self> {
        panic!(concat!(
            crate::errors::err_prefix!(),
            "`num_traits::NumCast` trait is not supported for ",
            stringify!($Int)
        ))
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> One for Integer<S, N, B, OM> {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn is_one(&self) -> bool {
        Self::is_one(&self)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> ConstOne for Integer<S, N, B, OM> {
    const ONE: Self = Self::ONE;
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Zero for Integer<S, N, B, OM> {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        Self::is_zero(&self)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> ConstZero for Integer<S, N, B, OM> {
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
            match crate::cast::float::uint_try_from_float::<$float, Uint<N, B, OM>>(f) {
                Ok(u) => Some(u.force_sign::<S>()),
                Err(_) => None,
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

impl<const S: bool, const N: usize, const B: usize, const OM: u8> FromPrimitive for Integer<S, N, B, OM> {
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

    from_primitive_float!(from_f32, f32, S);
    from_primitive_float!(from_f64, f64, S);
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> IntegerTrait for Integer<S, N, B, OM> {
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
                return Integer::<true, N, B, OM>::MIN.abs().force_sign();
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
            a = a.unchecked_shr_internal(a_tz);
            b = b.unchecked_shr_internal(b_tz);
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
                a = a.unchecked_shr_internal(a.trailing_zeros());
            }
        }
    }

    #[inline]
    fn lcm(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            Self::ZERO
        } else {
            let gcd = (self / self.gcd(other)) * other;
            if S {
                gcd.force_sign::<true>().abs().force_sign()
            } else {
                gcd
            }
        }
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

impl<const S: bool, const N: usize, const OM: u8> PrimInt for Integer<S, N, 0, OM> {
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

impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    #[inline]
    const fn nth_root_internal(self, n: Exponent) -> Self {
        if self.is_zero() {
            return self;
        }
        let bit_width = self.bit_width();
        if n > bit_width {
            // in this case, output should be < (2^bit_width)^(1/n) < 2^1 = 2, and output must be at least 1, so output is 1
            return Self::ONE;
        }
        let e = if bit_width % n == 0 {
            bit_width / n
        } else {
            bit_width / n + 1
        };
        let mut x = Self::power_of_two(e);
        loop {
            let y = (x.mul_u128_digit(n as u128 - 1).0.add(self.div(x.pow(n - 1))))
                .div_rem_u64(n as u64)
                .0;
            if y.ge(&x) {
                return x;
            }
            x = y;
        }
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Roots for Integer<S, N, B, OM> {
    #[inline]
    fn sqrt(&self) -> Self {
        self.isqrt()
    }

    #[inline]
    fn cbrt(&self) -> Self {
        self.nth_root(3)
    }

    #[inline]
    fn nth_root(&self, n: u32) -> Self {
        match n {
            0 => panic!(crate::errors::err_msg!("attempt to calculate zeroth root")),
            1 => *self,
            2 => self.sqrt(),
            _ => {
                if self.is_negative_internal() {
                    let out = self.unsigned_abs_internal().nth_root(n);
                    return out.wrapping_neg().force_sign();
                }

                self.force_sign().nth_root_internal(n).force_sign()
            }
        }
    }
}

impl<const N: usize, const B: usize, const OM: u8> Unsigned for Uint<N, B, OM> {}

impl<const N: usize, const B: usize, const OM: u8> Signed for Int<N, B, OM> {
    #[inline]
    fn abs(&self) -> Self {
        Self::abs(*self)
    }

    #[inline]
    fn abs_sub(&self, other: &Self) -> Self {
        if self <= other {
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
        ($($prim: ty), *) => {
            paste::paste! {
                $(
                    test_bignum! {
                        function: <STest>::[<to_ $prim>](u: ref &STest)
                    }
                )*
            }
        };
    }

    macro_rules! test_from_primitive {
        ($($prim: ty), *) => {
            paste::paste! {
                $(
                    test_bignum! {
                        function: <STest>::[<from_ $prim>](u: $prim),
                        cases: [
                            (STestBase::MIN as $prim)
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

        use crate::test::cast_types::*;

        crate::test::test_into! {
            function: <STest as AsPrimitive>::as_,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
        }

        test_bignum! {
            function: <STest as CheckedAdd>::checked_add(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as CheckedSub>::checked_sub(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as CheckedMul>::checked_mul(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as CheckedDiv>::checked_div(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as CheckedRem>::checked_rem(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as CheckedNeg>::checked_neg(a: ref &STest)
        }
        test_bignum! {
            function: <STest as CheckedShl>::checked_shl(a: ref &STest, b: u8)
        }
        test_bignum! {
            function: <STest as CheckedShr>::checked_shr(a: ref &STest, b: u8)
        }
        test_bignum! {
            function: <STest as CheckedEuclid>::checked_div_euclid(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as CheckedEuclid>::checked_rem_euclid(a: ref &STest, b: ref &STest)
        }

        test_bignum! {
            function: <STest as Euclid>::div_euclid(a: ref &STest, b: ref &STest),
            skip: a.checked_div_euclid(b).is_none()
        }
        test_bignum! {
            function: <STest as Euclid>::rem_euclid(a: ref &STest, b: ref &STest),
            skip: a.checked_rem_euclid(b).is_none()
        }

        test_bignum! {
            function: <STest as SaturatingAdd>::saturating_add(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as SaturatingSub>::saturating_sub(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as SaturatingMul>::saturating_mul(a: ref &STest, b: ref &STest)
        }

        test_bignum! {
            function: <STest as Saturating>::saturating_add(a: STest, b: STest)
        }
        test_bignum! {
            function: <STest as Saturating>::saturating_sub(a: STest, b: STest)
        }

        test_bignum! {
            function: <STest as WrappingAdd>::wrapping_add(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as OverflowingAdd>::overflowing_add(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as WrappingSub>::wrapping_sub(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as OverflowingSub>::overflowing_sub(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as WrappingMul>::wrapping_mul(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as OverflowingMul>::overflowing_mul(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as WrappingNeg>::wrapping_neg(a: ref &STest)
        }
        test_bignum! {
            function: <STest as WrappingShl>::wrapping_shl(a: ref &STest, b: u16)
        }
        test_bignum! {
            function: <STest as WrappingShr>::wrapping_shr(a: ref &STest, b: u16)
        }

        test_bignum! {
            function: <STest as One>::is_one(a: ref &STest)
        }
        test_bignum! {
            function: <STest as Zero>::is_zero(a: ref &STest)
        }

        #[test]
        fn one() {
            assert_eq!(STest::one(), TestConvert::into(STestBase::one()));
        }

        #[test]
        fn zero() {
            assert_eq!(STest::zero(), TestConvert::into(STestBase::zero()));
        }

        #[test]
        fn min_value() {
            assert_eq!(STest::min_value(), TestConvert::into(STestBase::min_value()));
        }

        #[test]
        fn max_value() {
            assert_eq!(STest::max_value(), TestConvert::into(STestBase::max_value()));
        }

        test_bignum! {
            function: <STest as MulAdd>::mul_add(a: STest, b: STest, c: STest),
            skip: a.checked_mul(b).map(|d| d.checked_add(c)).flatten().is_none()
        }

        test_bignum! {
            function: <STest as Roots>::sqrt(a: ref &STest),
            skip: a.is_negative_internal()
        }
        test_bignum! {
            function: <STest as Roots>::cbrt(a: ref &STest)
        }
        test_bignum! {
            function: <STest as Roots>::nth_root(a: ref &STest, n: u32),
            skip: n == 0 || {
                let cond = a.is_negative_internal();

                n % 2 == 0 && cond || (n == 1 && cond && a == <STest>::MIN) // second condition is due to an error in the num_integer crate, which incorrectly panics when calculating the first root of i128::MIN
            },
            cases: [(ref &STest::MAX, 2)]
        }

        test_to_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        test_from_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

        test_bignum! {
            function: <STest as IntegerTrait>::div_floor(a: ref &STest, b: ref &STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest as IntegerTrait>::mod_floor(a: ref &STest, b: ref &STest),
            skip: b.is_zero()
        }
        test_bignum! {
            function: <STest as IntegerTrait>::lcm(a: ref &STest, b: ref &STest),
            skip: {
                let cond = a.checked_mul(b).is_none() || (a.is_negative_internal() && a == <STest>::MIN) || (b.is_negative_internal() && b == <STest>::MIN); // lcm(a, b) <= a * b
                cond
            },
            cases: [(ref &(1i32 as STestBase), ref &(-1i32 as STestBase))]
        }

        use crate::n;

        test_bignum! {
            function: <STest as IntegerTrait>::gcd(a: ref &STest, b: ref &STest),
            skip: {
                #[allow(unused_comparisons)]
                let cond = STestBase::MIN < 0 && crate::overflow::GLOBAL_OVERFLOW_CHECKS && (a == <STest>::MIN && (b == <STest>::MIN || b == n!(0))) || (b == <STest>::MIN && (a == <STest>::MIN || a == n!(0)));
                cond
            }
        }
        test_bignum! {
            function: <STest as IntegerTrait>::is_multiple_of(a: ref &STest, b: ref &STest)
        }
        test_bignum! {
            function: <STest as IntegerTrait>::is_even(a: ref &STest)
        }
        test_bignum! {
            function: <STest as IntegerTrait>::is_odd(a: ref &STest)
        }
        test_bignum! {
            function: <STest as IntegerTrait>::div_rem(a: ref &STest, b: ref &STest),
            skip: b.is_zero()
        }

        test_bignum! {
            function: <STest as PrimInt>::from_be(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::from_le(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::to_be(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::to_le(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::unsigned_shl(a: STest, n: u8),
            skip: n >= <STest>::BITS as u8
        }
        test_bignum! {
            function: <STest as PrimInt>::unsigned_shr(a: STest, n: u8),
            skip: n >= <STest>::BITS as u8
        }
        test_bignum! {
            function: <STest as PrimInt>::signed_shl(a: STest, n: u8),
            skip: n >= <STest>::BITS as u8
        }
        test_bignum! {
            function: <STest as PrimInt>::signed_shr(a: STest, n: u8),
            skip: n >= <STest>::BITS as u8
        }
        test_bignum! {
            function: <STest as PrimInt>::count_ones(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::count_zeros(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::leading_zeros(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::trailing_zeros(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::leading_ones(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::trailing_ones(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::rotate_left(a: STest, n: u32)
        }
        test_bignum! {
            function: <STest as PrimInt>::rotate_right(a: STest, n: u32)
        }
        test_bignum! {
            function: <STest as PrimInt>::swap_bytes(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::reverse_bits(a: STest)
        }
        test_bignum! {
            function: <STest as PrimInt>::pow(a: STest, n: u32),
            skip: a.checked_pow(n).is_none()
        }
        
        test_bignum! {
            function: <STest as FromBytes>::from_be_bytes(bytes: ref &[u8; STest::BYTES as usize])
        }
        test_bignum! {
            function: <STest as FromBytes>::from_le_bytes(bytes: ref &[u8; STest::BYTES as usize])
        }
        test_bignum! {
            function: <STest as ToBytes>::to_be_bytes(a: ref &STest)
        }
        test_bignum! {
            function: <STest as ToBytes>::to_le_bytes(a: ref &STest)
        }
    }

    crate::test::test_all! {
        testing signed;

        test_bignum! {
            function: <STest as Signed>::abs(a: ref &STest),
            skip: crate::overflow::GLOBAL_OVERFLOW_CHECKS && a == <STest>::MIN
        }
        test_bignum! {
            function: <STest as Signed>::abs_sub(a: ref &STest, b: ref &STest),
            skip: crate::overflow::GLOBAL_OVERFLOW_CHECKS && (a > b && a.checked_sub(b).is_none())
        }
        test_bignum! {
            function: <STest as Signed>::signum(a: ref &STest)
        }
        test_bignum! {
            function: <STest as Signed>::is_positive(a: ref &STest)
        }
        test_bignum! {
            function: <STest as Signed>::is_negative(a: ref &STest)
        }
    }
}
