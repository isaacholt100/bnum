use crate::arch;
use crate::{ParseIntError, TryFromIntError};

#[macro_use]
macro_rules! test_with_u128 {
    ($a: expr, $test_name: ident, $method: ident $(,$args: expr)*) => {
        #[test]
        fn $test_name() {
            let uint = BUint::<2>::from($a);
            assert_eq!(BUint::<2>::$method(uint, $($args),*), u128::$method($a, $($args),*));
        }
    };
    ((convert)$a: expr, $test_name: ident, $method: ident $(,$args: expr)*) => {
        #[test]
        fn $test_name() {
            let uint = BUint::<2>::from($a);
            assert_eq!(BUint::<2>::$method(uint, $($args),*), BUint::from(u128::$method($a, $($args),*)));
        }
    };
    ($test_name: ident, $method: ident $(,$args: expr)*) => {
        #[test]
        fn $test_name() {
            assert_eq!(BUint::<2>::$method($($args),*), u128::$method($($args),*));
        }
    }
}

mod cmp;
mod convert;
mod ops;
mod tryops;
mod numtraits;
mod overflow;
mod checked;
mod saturating;
mod wrapping;
mod fmt;
mod endian;

/// A big unsigned integer type. Digits are stored as little endian

#[derive(Clone, Copy, Hash)]
pub struct BUint<const N: usize> {
    digits: [u64; N],
}

// Standard Rust uint impl
impl<const N: usize> BUint<N> {
    pub const MIN: Self = {
        Self {
            digits: [u64::MIN; N],
        }
    };
    pub const MAX: Self = {
        Self {
            digits: [u64::MAX; N],
        }
    };
    pub const BITS: usize = 64 * N;
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
        // TODO: implement
        panic!("")
    }
    pub const fn count_ones(self) -> u32 {
        let mut i = 0;
        let mut ones = 0;
        while i < N {
            ones += self.digits[i].count_ones();
            i += 1;
        }
        ones
    }
    pub const fn count_zeros(self) -> u32 {
        let mut i = 0;
        let mut zeros = 0;
        while i < N {
            zeros += self.digits[i].count_zeros();
            i += 1;
        }
        zeros
    }
    pub const fn leading_zeros(self) -> u32 {
        let mut i = 0;
        let mut zeros = 0;
        while i < N {
            let digit = self.digits[N - 1 - i];
            zeros += digit.leading_zeros();
            if digit != u64::MIN {
                break;
            }
            i += 1;
        }
        zeros
    }
    pub const fn trailing_zeros(self) -> u32 {
        let mut i = 0;
        let mut zeros = 0;
        while i < N {
            let digit = self.digits[i];
            zeros += digit.trailing_zeros();
            if digit != u64::MIN {
                break;
            }
            i += 1;
        }
        zeros
    }
    pub const fn leading_ones(self) -> u32 {
        let mut i = 0;
        let mut ones = 0;
        while i < N {
            let digit = self.digits[N - 1 - i];
            ones += digit.leading_ones();
            if digit != u64::MAX {
                break;
            }
            i += 1;
        }
        ones
    }
    pub const fn trailing_ones(self) -> u32 {
        let mut i = 0;
        let mut ones = 0;
        while i < N {
            let digit = self.digits[i];
            ones += digit.trailing_ones();
            if digit != u64::MAX {
                break;
            }
            i += 1;
        }
        ones
    }
    pub const fn rotate_left(self, n: u32) -> Self {
        // TODO: implement
        Self::MIN
    }
    pub const fn rotate_right(self, n: u32) -> Self {
        // TODO: implement
        Self::MIN
    }
    pub const fn swap_bytes(self) -> Self {
        let mut uint = Self::MIN;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[N - 1 - i].swap_bytes();
            i += 1;
        }
        uint
    }
    pub const fn reverse_bits(self) -> Self {
        let mut uint = Self::MIN;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[N - 1 - i].reverse_bits();
            i += 1;
        }
        uint
    }
    pub fn pow(self, exp: u32) -> Self {
        self.checked_pow(exp).unwrap()
    }
    pub fn div_euclid(self, rhs: Self) -> Self {
        self.checked_div_euclid(rhs).unwrap()
    }
    pub fn rem_euclid(self, rhs: Self) -> Self {
        self.checked_rem_euclid(rhs).unwrap()
    }
    pub const fn is_power_of_two(self) -> bool {
        let mut i = 0;
        let mut ones = 0;
        while i < N {
            ones += self.digits[i].count_ones();
            if ones > 1 {
                return false;
            }
            i += 1;
        }
        ones == 1
    }
    pub fn next_power_of_two(self) -> Self {
        // TODO: implement
        if cfg!(debug_assertions) {
            // Should panic if overflow
            self.checked_next_power_of_two().unwrap()
        } else {
            // Should return 0 if overflow
            self.checked_next_power_of_two().unwrap_or(Self::MIN)
        }
    }
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        // TODO: implement
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test from_str_radix
    test_with_u128!(203583443659837459073490583937485738404u128, test_count_ones, count_ones);
    test_with_u128!(7435098345734853045348057390485934908u128, test_count_zeros, count_zeros);
    test_with_u128!(3948590439409853946593894579834793459u128, test_leading_ones, leading_ones);
    test_with_u128!(49859830845963457783945789734895834754u128, test_leading_zeros, leading_zeros);
    test_with_u128!(45678345973495637458973488509345903458u128, test_trailing_ones, trailing_ones);
    test_with_u128!(23488903477439859084534857349857034599u128, test_trailing_zeros, trailing_zeros);
    // Test rotate_left
    // Test rotate right
    test_with_u128!((convert)3749589304858934758390485937458349058u128, test_swap_bytes, swap_bytes);
    test_with_u128!((convert)3345565093489578938485934957893745984u128, test_reverse_bits, reverse_bits);
    // Test pow
    // Test div_euclid
    // Test rem_euclid
    #[test]
    fn test_is_power_of_two() {
        let power = BUint::<2>::from(1u128 << 88);
        let non_power = BUint::<2>::from((1u128 << 88) - 5);
        assert!(power.is_power_of_two());
        assert!(!non_power.is_power_of_two());
    }
    // Test next_power_of_two
    // Test checked_next_power_of_two
}

impl<const N: usize> BUint<N> {
    pub const ONE: Self = {
        let mut zero = Self::MIN;
        zero.digits[0] = 1;
        zero
    };
    const fn is_zero(self) -> bool {
        let mut i = 0;
        while i < N {
            if self.digits[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }
    const fn zeros() -> [u64; N] {
        [0; N]
    }
    const fn last_digit_index(&self) -> usize {
        let mut index = 0;
        let mut i = 0;
        while i < N {
            if self.digits[i] != 0 {
                index = i;
            }
            i += 1;
        }
        /*for (i, digit) in self.digits.iter().enumerate() {
            if digit != &0 {
                index = i;
            }
        }*/
        index
    }
    fn from_uninit<C>(mut closure: C) -> Self where C: FnMut(usize) -> u64 {
        // This is an unsafe but faster version, would be implemented but can't transmute for const generic array yet
        /*use std::mem::{self, MaybeUninit};
        let mut digits: [MaybeUninit<u64>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for i in 0..N {
            digits[i] = MaybeUninit::new(closure(i));
        }
        Self::from(unsafe {
            mem::transmute::<_, [u64; N]>(digits)
        });*/
        let mut digits = Self::zeros();
        for i in 0..N {
            digits[i] = closure(i);
        }
        Self::from(digits)
    }
    fn op<C>(&self, rhs: &Self, mut closure: C) -> Self where C: FnMut(u64, u64) -> u64 {
        Self::from_uninit(|i| {
            closure(self.digits[i], rhs.digits[i])
        })
    }
    fn shift_left(&self, by: usize) -> Self {
        let slice = &self.digits[by..];
        let mut digits = Self::zeros();
        for (i, digit) in slice.iter().enumerate() {
            digits[i] = *digit;
        }
        Self {
            digits,
        }
    }
    pub fn try_from_buint<const M: usize>(uint: BUint<M>) -> Result<Self, TryFromIntError> {
        let last_digit_index = uint.last_digit_index();
        if last_digit_index >= N {
            return Err("BUint<T, M> too large to convert to BUint<N>");
        }
        let mut digits = Self::zeros();
        for i in 0..last_digit_index {
            digits[i] = uint.digits[i];
        }
        Ok(Self::from(digits))
    }
    fn add_mut(self, rhs: &Self, out: &mut Self) -> Result<(), &'static str> {
        let mut carry = 0u8;
        for i in 0..N {
            carry = arch::adc(carry, self.digits[i], rhs.digits[i], &mut out.digits[i]);
        }
        if carry != 0 {
            Err("Overflow")
        } else {
            Ok(())
        }
    }
    fn sub_mut(self, rhs: &Self, out: &mut Self) -> Result<(), &'static str> {
        let mut borrow = 0u8;
        for i in 0..N {
            borrow = arch::sbb(borrow, self.digits[i], rhs.digits[i], &mut out.digits[i]);
        }
        if borrow != 0 {
            Err("Underflow")
        } else {
            Ok(())
        }
    }
}

use std::default::Default;

impl<const N: usize> Default for BUint<N> {
    fn default() -> Self {
        Self::MIN
    }
}

use std::iter::{Iterator, Product, Sum};

impl<const N: usize> Product<Self> for BUint<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for BUint<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::MIN, |a, b| a + b)
    }
}