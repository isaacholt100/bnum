use crate::digit::{Digit, SignedDigit, self};
use crate::uint::BUint;
#[allow(unused_imports)]
use crate::I128;
use crate::macros::expect;
use crate::ExpType;

#[allow(unused)]
macro_rules! test_signed {
    {
        name: $name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        }
    } => {
        test! {
            big: I128,
            primitive: i128,
            name: $name,
            method: {
                $($name ($($arg), *) ;) *
            }
        }
    };
    {
        name: $name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        },
        converter: $converter: expr
    } => {
        test! {
            big: I128,
            primitive: i128,
            name: $name,
            method: {
                $($name ($($arg), *) ;) *
            },
            converter: $converter
        }
    }
}

macro_rules! uint_method {
    { $(fn $name: ident ($self: ident $(,$param: ident : $Type: ty)*) -> $ret: ty), * } => {
        $(pub const fn $name($self $(,$param: $Type)*) -> $ret {
            $self.uint.$name($($param), *)
        })*
    };
    { $(fn $name: ident (&$self: ident $(,$param: ident : $Type: ty)*) -> $ret: ty), * } => {
        $(pub const fn $name($self $(,$param: $Type)*) -> $ret {
            $self.uint.$name($($param), *)
        })*
    };
}

mod cast;
mod cmp;
mod convert;
mod ops;
mod numtraits;
mod overflow;
mod checked;
mod saturating;
mod wrapping;
mod fmt;
mod endian;
mod radix;

#[cfg(feature = "serde_all")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde_all")]
#[derive(Clone, Copy, Hash, Debug, Serialize, Deserialize)]
pub struct BIint<const N: usize> {
    uint: BUint<N>,
}

#[cfg(not(feature = "serde_all"))]
#[derive(Clone, Copy, Hash, Debug)]
pub struct BIint<const N: usize> {
    uint: BUint<N>,
}

/// impl containing constants
impl<const N: usize> BIint<N> {
    pub const MIN: Self = {
        let mut digits = [0; N];
        digits[N - 1] = 1 << (digit::BITS - 1);
        Self {
            uint: BUint::from_digits(digits),
        }
    };
    pub const MAX: Self = {
        let mut digits = [Digit::MAX; N];
        digits[N - 1] >>= 1;
        Self {
            uint: BUint::from_digits(digits),
        }
    };
    pub const ZERO: Self = {
        Self {
            uint: BUint::ZERO,
        }
    };
    pub const ONE: Self = {
        Self {
            uint: BUint::ONE,
        }
    };
    pub const NEG_ONE: Self = {
        Self {
            uint: BUint::MAX,
        }
    };
    pub const BYTES: usize = BUint::<N>::BYTES;
    pub const BITS: usize = BUint::<N>::BITS;
    const N_MINUS_1: usize = N - 1;
}

macro_rules! log {
    ($method: ident $(, $base: ident : $ty: ty)?) => {
        pub const fn $method(self, $($base : $ty),*) -> ExpType {
            if self.is_negative() {
                #[cfg(debug_assertions)]
                panic!("attempt to calculate log of negative number");
                #[cfg(not(debug_assertions))]
                0
            } else {
                self.uint.$method($($base.uint)?)
            }
        }
    }
}

impl<const N: usize> BIint<N> {
    uint_method! {
        fn count_ones(self) -> ExpType,
        fn count_zeros(self) -> ExpType,
        fn leading_zeros(self) -> ExpType,
        fn trailing_zeros(self) -> ExpType,
        fn leading_ones(self) -> ExpType,
        fn trailing_ones(self) -> ExpType
    }

    pub const fn rotate_left(self, n: ExpType) -> Self {
        Self {
            uint: self.uint.rotate_left(n),
        }
    }
    pub const fn rotate_right(self, n: ExpType) -> Self {
        Self {
            uint: self.uint.rotate_right(n),
        }
    }
    pub const fn swap_bytes(self) -> Self {
        Self {
            uint: self.uint.swap_bytes(),
        }
    }
    pub const fn reverse_bits(self) -> Self {
        Self {
            uint: self.uint.reverse_bits(),
        }
    }
    pub const fn unsigned_abs(self) -> BUint<N> {
        if self.is_negative() {
            self.wrapping_neg().uint
        } else {
            self.uint
        }
    }

    #[cfg(debug_assertions)]
    pub const fn pow(self, exp: ExpType) -> Self {
        expect!(self.checked_pow(exp), "attempt to calculate power with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn pow(self, exp: ExpType) -> Self {
        self.wrapping_pow(exp)
    }

    pub const fn div_euclid(self, rhs: Self) -> Self {
        assert!(!self.eq(&Self::MIN) && !rhs.eq(&Self::NEG_ONE), "attempt to divide with overflow");
        self.wrapping_div_euclid(rhs)
    }
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        assert!(!self.eq(&Self::MIN) && !rhs.eq(&Self::NEG_ONE), "attempt to calculate remainder with overflow");
        self.wrapping_rem_euclid(rhs)
    }

    #[cfg(debug_assertions)]
    pub const fn abs(self) -> Self {
        expect!(self.checked_abs(), "attempt to negate with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn abs(self) -> Self {
        match self.checked_abs() {
            Some(int) => int,
            None => Self::MIN,
        }
    }

    pub const fn signum(self) -> Self {
        if self.is_negative() {
            Self::NEG_ONE
        } else if self.is_zero() {
            Self::ZERO
        } else {
            Self::ONE
        }
    }
    pub const fn is_positive(self) -> bool {
        let signed_digit = self.signed_digit();
        signed_digit.is_positive() ||
        (signed_digit == 0 && !self.uint.is_zero())
    }
    pub const fn is_negative(self) -> bool {
        self.signed_digit().is_negative()
    }
    pub const fn is_power_of_two(self) -> bool {
        if self.is_negative() {
            false
        } else {
            self.uint.is_power_of_two()
        }
    }

    #[cfg(debug_assertions)]
    pub const fn next_power_of_two(self) -> Self {
        expect!(self.checked_next_power_of_two(), "attempt to calculate next power of two with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn next_power_of_two(self) -> Self {
        self.wrapping_next_power_of_two()
    }

    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_negative() {
            Some(Self::ONE)
        } else {
            match self.uint.checked_next_power_of_two() {
                Some(uint) => {
                    let out = Self {
                        uint,
                    };
                    if out.is_negative() {
                        None
                    } else {
                        Some(out)
                    }
                },
                None => None,
            }
        }
    }
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::MIN,
        }
    }
    log!(log, base: Self);
    log!(log2);
    log!(log10);
}

impl<const N: usize> BIint<N> {
    uint_method! {
        fn bit(&self, index: usize) -> bool,
        fn bits(&self) -> usize
    }
    const fn signed_digit(&self) -> SignedDigit {
        self.uint.digits()[N - 1] as SignedDigit
    }
    pub const fn is_zero(self) -> bool {
        self.uint.is_zero()
    }
    #[inline(always)]
    pub const fn digits(&self) -> &[Digit; N] {
        &self.uint.digits()
    }
    #[inline(always)]
    pub const fn from_digits(digits: [Digit; N]) -> Self {
        Self {
            uint: BUint::from_digits(digits),
        }
    }
    #[inline(always)]
    pub const fn from_bits(uint: BUint<N>) -> Self {
        Self {
            uint,
        }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> BUint<N> {
        self.uint
    }
}

use core::default::Default;

impl<const N: usize> Default for BIint<N> {
    fn default() -> Self {
        Self::ZERO
    }
}

use core::iter::{Iterator, Product, Sum};

impl<const N: usize> Product<Self> for BIint<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for BIint<N> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for BIint<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for BIint<N> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_signed! {
        name: count_ones,
        method: {
            count_ones(34579834758459769875878374593749837548i128);
            count_ones(-720496794375698745967489576984655i128);
        },
        converter: crate::u32_to_exp
    }
    test_signed! {
        name: count_zeros,
        method: {
            count_zeros(97894576897934857979834753847877889734i128);
            count_zeros(-302984759749756756756756756756756i128);
        },
        converter: crate::u32_to_exp
    }
    test_signed! {
        name: leading_zeros,
        method: {
            leading_zeros(1234897937459789793445634456858978937i128);
            leading_zeros(-30979347598678947567567567i128);
            leading_zeros(0i128);
        },
        converter: crate::u32_to_exp
    }
    test_signed! {
        name: trailing_zeros,
        method: {
            trailing_zeros(8003849534758937495734957034534073957i128);
            trailing_zeros(-972079507984789567894375674857645i128);
            trailing_zeros(0i128);
        },
        converter: crate::u32_to_exp
    }
    test_signed! {
        name: leading_ones,
        method: {
            leading_ones(1);
            leading_ones(290758976947569734598679898445i128);
            leading_ones(-1);
            leading_ones(-923759647398567894358976894546i128);
        },
        converter: crate::u32_to_exp
    }
    test_signed! {
        name: trailing_ones,
        method: {
            trailing_ones(i128::MAX);
            trailing_ones(72984756897458906798456456456i128);
            trailing_ones(-1);
            trailing_ones(-293745027465978924758697459867i128);
        },
        converter: crate::u32_to_exp
    }
    test_signed! {
        name: rotate_left,
        method: {
            rotate_left(3457894375984563459457i128, 61845 as u16);
            rotate_left(-34598792345789i128, 4 as u16);
        }
    }
    test_signed! {
        name: rotate_right,
        method: {
            rotate_right(109375495687201345976994587i128, 354 as u16);
            rotate_right(-4598674589769i128, 75 as u16);
        }
    }
    test_signed! {
        name: swap_bytes,
        method: {
            swap_bytes(98934757983792102304988394759834587i128);
            swap_bytes(-234i128);
        }
    }
    test_signed! {
        name: reverse_bits,
        method: {
            reverse_bits(349579348509348374589749083490580i128);
            reverse_bits(-22003495898345i128);
        }
    }
    test_signed! {
        name: unsigned_abs,
        method: {
            unsigned_abs(i128::MIN);
            unsigned_abs(0);
            unsigned_abs(45645634534564264564534i128);
            unsigned_abs(-9729307972495769745965545i128);
        }
    }
    test_signed! {
        name: pow,
        method: {
            pow(-564i128, 6 as u16);
            pow(6957i128, 8 as u16);
            pow(-67i128, 19 as u16);
        }
    }
    test_signed! {
        name: div_euclid,
        method: {
            div_euclid(-29475698745698i128 * 4685684568, 29475698745698i128);
            div_euclid(4294567897594568765i128, 249798748956i128);
            div_euclid(27456979757i128, 45i128);
        }
    }
    test_signed! {
        name: rem_euclid,
        method: {
            rem_euclid(-7902709857689456456i128 * 947659873456, 7902709857689456456i128);
            rem_euclid(-46945656i128, 896794576985645i128);
            rem_euclid(-45679i128, -8i128);
        }
    }
    test_signed! {
        name: abs,
        method: {
            abs(0i128);
            abs(i128::MAX);
            abs(-249576984756i128);
        }
    }
    test_signed! {
        name: signum,
        method: {
            signum(0i128);
            signum(275966456345645635473947569i128);
            signum(-972945769613987589476598745i128);
        }
    }

    #[test]
    fn is_positive() {
        assert!(I128::from(304950490384054358903845i128).is_positive());
        assert!(!I128::from(-304950490384054358903845i128).is_positive());
        assert!(!I128::from(0).is_positive());
    }
    #[test]
    fn is_negative() {
        assert!(!I128::from(30890890894345345343453434i128).is_negative());
        assert!(I128::from(-8783947895897346938745873443i128).is_negative());
        assert!(!I128::from(0).is_negative());
    }

    #[test]
    fn is_power_of_two() {
        assert!(!I128::from(-94956729465i128).is_power_of_two());
        assert!(!I128::from(79458945i128).is_power_of_two());
        assert!(I128::from(1i128 << 17).is_power_of_two());
    }

    #[test]
    fn next_power_of_two() {
        assert_eq!(I128::from(-372979834534345587i128).next_power_of_two(), 1i128.into());
        assert_eq!(I128::from((1i128 << 88) - 5).next_power_of_two(), (1i128 << 88).into());
        assert_eq!(I128::from(1i128 << 56).next_power_of_two(), (1i128 << 56).into());
    }

    #[test]
    fn checked_next_power_of_two() {
        assert_eq!(I128::from(-979457698).checked_next_power_of_two(), Some(1i128.into()));
        assert_eq!(I128::from(5).checked_next_power_of_two(), Some(8i32.into()));
        assert_eq!(I128::from(i128::MAX - 5).checked_next_power_of_two(), None);
    }

    #[test]
    fn wrapping_next_power_of_two() {
        assert_eq!(I128::from(-89i128).wrapping_next_power_of_two(), 1i128.into());
        assert_eq!(I128::from((1i128 << 75) + 4).wrapping_next_power_of_two(), (1i128 << 76).into());
        assert_eq!(I128::from(i128::MAX / 2 + 4).wrapping_next_power_of_two(), I128::MIN);
    }
    // TODO: test other methods
}