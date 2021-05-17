use crate::digit::{Digit, SignedDigit, DIGIT_BITS, DIGIT_BITS_U32};
use crate::uint::{BUint, self};
use crate::ParseIntError;
#[allow(unused_imports)]
use crate::I128Test;

#[allow(unused)]
macro_rules! test_signed {
    {
        test_name: $test_name: ident,
        method: $method: ident ($($arg: expr), *)
    } => {
        test! {
            big: I128Test,
            primitive: i128,
            test_name: $test_name,
            method: $method ($($arg), *)
        }
    };
    {
        test_name: $test_name: ident,
        method: $method: ident ($($arg: expr), *),
        converter: $converter: expr
    } => {
        test! {
            big: I128Test,
            primitive: i128,
            test_name: $test_name,
            method: $method ($($arg), *),
            converter: $converter
        }
    }
}

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
    const UINT_LENGTH: usize = N;
    const UINT_MIN: BUint::<N> = BUint::<N>::MIN;
    const UINT_ONE: BUint::<N> = BUint::<N>::ONE;
    const UINT_MAX: BUint::<N> = BUint::<N>::MAX;
    const UINT_BITS: usize = Self::UINT_LENGTH * DIGIT_BITS;
    const UINT_BYTES: usize = Self::UINT_BITS / 8;
    pub const BYTES: usize = Self::BITS / 8;
    pub const BITS: usize = N * DIGIT_BITS;
}

impl<const N: usize> BintTest<N> {
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
        unimplemented!()
    }
    pub const fn count_ones(self) -> u32 {
        self.uint.count_ones()
    }
    pub const fn count_zeros(self) -> u32 {
        self.uint.count_zeros()
    }
    pub const fn leading_zeros(self) -> u32 {
        self.uint.leading_zeros()
    }
    pub const fn trailing_zeros(self) -> u32 {
        self.uint.trailing_zeros()
    }
    pub const fn leading_ones(self) -> u32 {
        self.uint.leading_ones()
    }
    pub const fn trailing_ones(self) -> u32 {
        self.uint.trailing_ones()
    }
    pub const fn rotate_left(self, n: u32) -> Self {
        unimplemented!()
    }
    pub const fn rotate_right(self, n: u32) -> Self {
        unimplemented!()
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
    pub const fn unsigned_abs(self) -> BUint<{Self::UINT_LENGTH + 1}> {
        unimplemented!()
    }
    pub const fn pow(self, exp: u32) -> Self {
        unimplemented!()
    }
    pub const fn div_euclid(self, rhs: Self) -> Self {
        unimplemented!()
    }
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        unimplemented!()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test from_str_radix
    test_signed! {
        test_name: test_count_ones_pos,
        method: count_ones(34579834758459769875878374593749837548i128)
    }
    test_signed! {
        test_name: test_count_ones_neg,
        method: count_ones(-34579834758945986784957689473749837548i128)
    }
    test_signed! {
        test_name: test_count_zeros_pos,
        method: count_zeros(97894576897934857979834753847877889734i128)
    }
    test_signed! {
        test_name: test_count_zeros_neg,
        method: count_zeros(-97894576897934857979834753847877889734i128)
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
        test_name: test_leading_zeros_pos,
        method: leading_zeros(1234897937459789793445634456858978937i128)
    }
    test_signed! {
        test_name: test_leading_zeros_neg,
        method: leading_zeros(-1234897937459789793445634456858978937i128)
    }
    test_signed! {
        test_name: test_trailing_zeros_pos,
        method: trailing_zeros(8003849534758937495734957034534073957i128)
    }
    test_signed! {
        test_name: test_trailing_zeros_neg,
        method: trailing_zeros(-8003849534758937495734957034534073957i128)
    }
    test_signed! {
        test_name: test_leading_ones_pos,
        method: leading_ones(1)
    }
    test_signed! {
        test_name: test_leading_ones_neg,
        method: leading_ones(-1)
    }
    test_signed! {
        test_name: test_trailing_ones_pos,
        method: trailing_ones(1)
    }
    test_signed! {
        test_name: test_trailing_ones_neg,
        method: trailing_ones(-1)
    }

    // Test rotate_left
    // Test rotate_right

    test_signed! {
        test_name: test_swap_bytes_pos,
        method: swap_bytes(98934757983792102304988394759834587i128)
    }
    test_signed! {
        test_name: test_swap_bytes_neg,
        method: swap_bytes(-234i128)
    }
    test_signed! {
        test_name: test_reverse_bits_pos,
        method: reverse_bits(349579348509348374589749083490580i128)
    }
    test_signed! {
        test_name: test_reverse_bits_neg,
        method: reverse_bits(-22003495898345i128)
    }
}

impl<const N: usize> BintTest<N> {
    const fn signed_digit(&self) -> SignedDigit {
        self.uint.digits()[N - 1] as SignedDigit
    }
    pub fn parse_bytes(buf: &[u8], radix: u32) -> Option<Self> {
        unimplemented!()
    }
    pub fn from_radix_be(buf: &[u8], radix: u32) -> Option<Self> {
        unimplemented!()
    }
    pub fn from_radix_le(buf: &[u8], radix: u32) -> Option<Self> {
        unimplemented!()
    }
    pub fn to_str_radix(&self, radix: u32) -> String {
        unimplemented!()
    }
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        unimplemented!()
    }
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        unimplemented!()
    }
    pub const fn modpow(&self, exp: &Self, modulus: &Self) -> Self {
        unimplemented!()
    }
    pub const fn sqrt(&self) -> Self {
        unimplemented!()
    }
    pub const fn cbrt(&self) -> Self {
        unimplemented!()
    }
    pub const fn nth_root(&self, n: u32) -> Self {
        unimplemented!()
    }
    pub const fn is_zero(self) -> bool {
        self.uint.is_zero()
    }
}

use std::default::Default;

impl<const N: usize> Default for BintTest<N> {
    fn default() -> Self {
        Self::ZERO
    }
}

use std::iter::{Iterator, Product, Sum};

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