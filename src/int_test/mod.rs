use crate::digit::{Digit, SignedDigit, self};
use crate::uint::{BUint, self};
use crate::ParseIntError;
#[allow(unused_imports)]
use crate::I128Test;

#[allow(unused)]
macro_rules! test_signed {
    {
        test_name: $test_name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        }
    } => {
        test! {
            big: I128Test,
            primitive: i128,
            test_name: $test_name,
            method: {
                $($method ($($arg), *) ;) *
            }
        }
    };
    {
        test_name: $test_name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        },
        converter: $converter: expr
    } => {
        test! {
            big: I128Test,
            primitive: i128,
            test_name: $test_name,
            method: {
                $($method ($($arg), *) ;) *
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

use serde::{Serialize, Deserialize};

/// Stored similarly to two's complement
#[derive(Clone, Copy, Hash, Debug, Serialize, Deserialize)]
pub struct BintTest<const N: usize> {
    uint: BUint<N>,
}
/// impl containing constants

impl<const N: usize> BintTest<N> {
    pub const MIN: Self = {
        let mut digits = [0; N];
        digits[N - 1] = 1;
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
    pub const MINUS_ONE: Self = {
        Self {
            uint: BUint::MAX,
        }
    };
    pub const BYTES: usize = Self::BITS / 8;
    pub const BITS: usize = N * digit::BITS;
}

impl<const N: usize> BintTest<N> {
    uint_method! {
        fn count_ones(self) -> u32,
        fn count_zeros(self) -> u32,
        fn leading_zeros(self) -> u32,
        fn trailing_zeros(self) -> u32,
        fn leading_ones(self) -> u32,
        fn trailing_ones(self) -> u32
    }

    pub const fn rotate_left(self, n: u32) -> Self {
        Self {
            uint: self.uint.rotate_left(n),
        }
    }
    pub const fn rotate_right(self, n: u32) -> Self {
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
            if self.eq(&Self::MIN) {
                let mut digits = [0; N];
                digits[N - 1] = 1;
                BUint::from_digits(digits)
            } else {
                self.wrapping_neg().uint
            }
        } else {
            self.uint
        }
    }
    pub const fn pow(self, exp: u32) -> Self {
        todo!()
    }
    pub const fn div_euclid(self, rhs: Self) -> Self {
        todo!()
    }
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        todo!()
    }
    #[cfg(debug_assertions)]
    pub const fn abs(self) -> Self {
        match self.checked_abs() {
            Some(int) => int,
            None => panic!("attempt to negate with overflow"),
        }
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
            Self::MINUS_ONE
        } else if self.is_zero() {
            Self::ZERO
        } else {
            Self::ONE
        }
    }
    pub const fn is_positive(self) -> bool {
        self.signed_digit().is_positive() ||
        (self.signed_digit() == 0 && !self.uint.is_zero())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    test_signed! {
        test_name: test_count_ones,
        method: {
            count_ones(34579834758459769875878374593749837548i128);
            count_ones(-34579834758945986784957689473749837548i128);
        }
    }
    test_signed! {
        test_name: test_count_zeros,
        method: {
            count_zeros(97894576897934857979834753847877889734i128);
            count_zeros(-97894576897934857979834753847877889734i128);
        }
    }

    #[test]
    fn test_is_positive() {
        assert!(I128Test::from(304950490384054358903845i128).is_positive());
        assert!(!I128Test::from(-304950490384054358903845i128).is_positive());
        assert!(!I128Test::from(0).is_positive());
    }
    #[test]
    fn test_is_negative() {
        assert!(!I128Test::from(30890890894345345343453434i128).is_negative());
        assert!(I128Test::from(-8783947895897346938745873443i128).is_negative());
        assert!(!I128Test::from(0).is_negative());
    }

    test_signed! {
        test_name: test_leading_zeros,
        method: {
            leading_zeros(1234897937459789793445634456858978937i128);
            leading_zeros(-1234897937459789793445634456858978937i128);
        }
    }
    test_signed! {
        test_name: test_trailing_zeros,
        method: {
            trailing_zeros(8003849534758937495734957034534073957i128);
            trailing_zeros(-8003849534758937495734957034534073957i128);
        }
    }
    test_signed! {
        test_name: test_leading_ones,
        method: {
            leading_ones(1);
            leading_ones(-1);
        }
    }
    test_signed! {
        test_name: test_trailing_ones,
        method: {
            trailing_ones(1);
            trailing_ones(-1);
        }
    }
    test_signed! {
        test_name: test_rotate_left,
        method: {
            rotate_left(3457894375984563459457i128, 69845u32);
            rotate_left(-34598792345789i128, 4u32);
        }
    }
    test_signed! {
        test_name: test_rotate_right,
        method: {
            rotate_right(109375495687201345976994587i128, 354u32);
            rotate_right(-4598674589769i128, 75u32);
        }
    }
    test_signed! {
        test_name: test_swap_bytes,
        method: {
            swap_bytes(98934757983792102304988394759834587i128);
            swap_bytes(-234i128);
        }
    }
    test_signed! {
        test_name: test_reverse_bits,
        method: {
            reverse_bits(349579348509348374589749083490580i128);
            reverse_bits(-22003495898345i128);
        }
    }
    test_signed! {
        test_name: test_unsigned_abs,
        method: {
            unsigned_abs(i128::MIN);
            unsigned_abs(45645634534534i128);
            unsigned_abs(-456456345334534i128);
        }
    }
}

impl<const N: usize> BintTest<N> {
    uint_method! {
        fn bit(&self, index: usize) -> bool,
        fn bits(&self) -> usize,
        fn digits(&self) -> [Digit; N]
    }
    const fn signed_digit(&self) -> SignedDigit {
        self.uint.digits()[N - 1] as SignedDigit
    }
    pub const fn is_zero(self) -> bool {
        self.uint.is_zero()
    }
}

use core::default::Default;

impl<const N: usize> Default for BintTest<N> {
    fn default() -> Self {
        Self::ZERO
    }
}

use core::iter::{Iterator, Product, Sum};

impl<const N: usize> Product<Self> for BintTest<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for BintTest<N> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for BintTest<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for BintTest<N> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}