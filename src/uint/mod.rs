use crate::arch;
use crate::{TryFromIntError};
use crate::digit::{Digit, self};
#[allow(unused_imports)]
pub use crate::U128;

#[allow(unused)]
macro_rules! test_unsigned {
    {
        test_name: $test_name: ident,
        method: $method: ident ($($arg: expr), *)
    } => {
        test! {
            big: U128,
            primitive: u128,
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
            big: U128,
            primitive: u128,
            test_name: $test_name,
            method: $method ($($arg), *),
            converter: $converter
        }
    }
}

#[allow(unused)]
#[macro_use]
macro_rules! possible_const_fn {
    ($self: ident, fn $name: ident (self $(,$param: ident : $Type: ty)*) -> $ret: ty $body: block) => {
        #[cfg(feature = "intrinsics")]
        pub fn $name($self $(,$param: $Type)*) -> $ret $body
        #[cfg(not(feature = "intrinsics"))]
        pub const fn $name($self $(,$param: $Type)*) -> $ret $body
    };
}
use ::serde::{Serialize, Deserialize};

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
mod radix;
mod cast;

/// A big unsigned integer type. Digits are stored as little endian

use serde_big_array::BigArray;

#[derive(Clone, Copy, Hash, Debug, Serialize, Deserialize)]

pub struct BUint<const N: usize> {
    #[serde(with = "BigArray")]
    digits: [Digit; N],
}

/// impl containing constants

impl<const N: usize> BUint<N> {
    pub const MIN: Self = {
        Self {
            digits: [Digit::MIN; N],
        }
    };
    pub const MAX: Self = {
        Self {
            digits: [Digit::MAX; N],
        }
    };
    pub const ZERO: Self = Self::MIN;
    pub const ONE: Self = {
        let mut zero = Self::ZERO;
        zero.digits[0] = 1;
        zero
    };
    pub const BITS: usize = digit::BITS * N;
    pub const BYTES: usize = Self::BITS / 8;
}

pub const fn trailing_zeros<const N: usize>(uint: BUint<N>) -> (u32, bool) {
    let mut zeros = 0;
    let mut did_break = false;
    let mut i = 0;
    while i < N {
        let digit = uint.digits[i];
        zeros += digit.trailing_zeros();
        if digit != Digit::MIN {
            did_break = true;
            break;
        }
        i += 1;
    }
    (zeros, did_break)
}

pub const fn trailing_ones<const N: usize>(uint: BUint<N>) -> (u32, bool) {
    let mut ones = 0;
    let mut did_break = false;
    let mut i = 0;
    while i < N {
        let digit = uint.digits[i];
        ones += digit.trailing_ones();
        if digit != Digit::MAX {
            did_break = true;
            break;
        }
        i += 1;
    }
    (ones, did_break)
}

// Standard Rust uint impl
impl<const N: usize> BUint<N> {
    pub const fn count_ones(self) -> u32 {
        let mut ones = 0;
        let mut i = 0;
        while i < N {
            ones += self.digits[i].count_ones();
            i += 1;
        }
        ones
    }
    pub const fn count_zeros(self) -> u32 {
        let mut zeros = 0;
        let mut i = 0;
        while i < N {
            zeros += self.digits[i].count_zeros();
            i += 1;
        }
        zeros
    }
    pub const fn leading_zeros(self) -> u32 {
        let mut zeros = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.digits[i];
            zeros += digit.leading_zeros();
            if digit != Digit::MIN {
                break;
            }
        }
        zeros
    }
    pub const fn trailing_zeros(self) -> u32 {
        trailing_zeros(self).0
    }
    pub const fn leading_ones(self) -> u32 {
        let mut ones = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.digits[i];
            ones += digit.leading_ones();
            if digit != Digit::MAX {
                break;
            }
        }
        ones
    }
    pub const fn trailing_ones(self) -> u32 {
        trailing_ones(self).0
    }
    const fn unchecked_rotate_left(self, n: u32) -> Self {
        if n == 0 {
            self
        } else {
            let digit_shift = (n >> digit::BIT_SHIFT) as usize;
            let shift = (n % digit::BITS_U32) as u8;
            
            let mut out = Self::ZERO;
            let mut carry = 0;
            let carry_shift = digit::BITS_U32 as u8 - shift;

            let mut i = 0;
            while i < N - digit_shift {
                let digit = self.digits[i];
                let new_carry = digit >> carry_shift;
                let new_digit = (digit << shift) | carry;
                carry = new_carry;
                out.digits[i + digit_shift] = new_digit;
                i += 1;
            }
            while i < N {
                let digit = self.digits[i];
                let new_carry = digit >> carry_shift;
                let new_digit = (digit << shift) | carry;
                carry = new_carry;
                out.digits[i + digit_shift - N] = new_digit;
                i += 1;
            }

            out.digits[digit_shift] |= carry;

            out
        }
    }
    const BITS_MINUS_1: u32 = (Self::BITS - 1) as u32;
    pub const fn rotate_left(self, n: u32) -> Self {
        let n = n & Self::BITS_MINUS_1;
        self.unchecked_rotate_left(n)
    }
    pub const fn rotate_right(self, n: u32) -> Self {
        let n = n & Self::BITS_MINUS_1;
        self.unchecked_rotate_left(N as u32 - n)
    }
    const N_MINUS_1: usize = N - 1;
    pub const fn swap_bytes(self) -> Self {
        let mut uint = Self::ZERO;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[Self::N_MINUS_1 - i].swap_bytes();
            i += 1;
        }
        uint
    }
    pub const fn reverse_bits(self) -> Self {
        let mut uint = Self::ZERO;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[Self::N_MINUS_1 - i].reverse_bits();
            i += 1;
        }
        uint
    }
    pub const fn pow(self, exp: u32) -> Self {
        expect!(self.checked_pow(exp), "attempt to calculate power with overflow")
    }
    pub const fn div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div_euclid(rhs)
    }
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem_euclid(rhs)
    }
    pub const fn is_power_of_two(&self) -> bool {
        let mut i = 0;
        let mut ones = 0;
        while i < N {
            ones += (&self.digits)[i].count_ones();
            if ones > 1 {
                return false;
            }
            i += 1;
        }
        ones == 1
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
        let last_set_digit_index = self.last_digit_index();
        let leading_zeros = self.digits[last_set_digit_index].leading_zeros();

        if leading_zeros == 0 {
            if last_set_digit_index == Self::N_MINUS_1 {
                None
            } else {
                let mut out = Self::ZERO;
                out.digits[last_set_digit_index + 1] = 1;
                Some(out)
            }
        } else {
            let mut out = Self::ZERO;
            out.digits[last_set_digit_index] = 1 << (digit::BITS_U32 - leading_zeros);
            Some(out)
        }
    }
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::ZERO,
        }
    } 
}

#[cfg(test)]
mod tests {
    use crate::U128;

    // Test from_str_radix

    test_unsigned! {
        test_name: test_count_ones,
        method: count_ones(203583443659837459073490583937485738404u128)
    }
    test_unsigned! {
        test_name: test_count_zeros,
        method: count_zeros(7435098345734853045348057390485934908u128)
    }
    test_unsigned! {
        test_name: test_leading_ones,
        method: leading_ones(3948590439409853946593894579834793459u128)
    }
    test_unsigned! {
        test_name: test_leading_zeros,
        method: leading_zeros(49859830845963457783945789734895834754u128)
    }
    test_unsigned! {
        test_name: test_trailing_ones,
        method: trailing_ones(45678345973495637458973488509345903458u128)
    }
    test_unsigned! {
        test_name: test_trailing_zeros,
        method: trailing_zeros(23488903477439859084534857349857034599u128)
    }
    test_unsigned! {
        test_name: test_rotate_left,
        method: rotate_left(394857348975983475983745983798579483u128, 5555u32)
    }
    test_unsigned! {
        test_name: test_rotate_right,
        method: rotate_left(90845674987957297107197973489575938457u128, 10934u32)
    }
    test_unsigned! {
        test_name: test_swap_bytes,
        method: swap_bytes(3749589304858934758390485937458349058u128)
    }
    test_unsigned! {
        test_name: test_reverse_bits,
        method: reverse_bits(3345565093489578938485934957893745984u128)
    }
    // Test pow
    // Test div_euclid
    // Test rem_euclid
    #[test]
    fn test_is_power_of_two() {
        let power = U128::from(1u128 << 88);
        let non_power = U128::from((1u128 << 88) - 5);
        assert!(power.is_power_of_two());
        assert!(!non_power.is_power_of_two());
    }
    // Test next_power_of_two
    // Test checked_next_power_of_two
    test_unsigned! {
        test_name: test_next_power_of_two,
        method: next_power_of_two(394857834758937458973489573894759879u128)
    }
}
use crate::digit::BYTE_SHIFT;

impl<const N: usize> BUint<N> {
    pub const fn bits(&self) -> usize {
        let last_digit_index = self.last_digit_index();
        ((last_digit_index + 1) << digit::BIT_SHIFT) - (&self.digits[last_digit_index]).leading_zeros() as usize
    }
    pub const fn bit(&self, index: usize) -> bool {
        const BITS_MINUS_1: usize = digit::BITS - 1;

        let digit = (&self.digits)[index >> BYTE_SHIFT];
        digit & (1 << (index & BITS_MINUS_1)) != 0
    }
    pub const fn sqrt(&self) -> Self {
        todo!()
    }
    pub const fn cbrt(&self) -> Self {
        todo!()
    }
    pub const fn nth_root(&self, n: u32) -> Self {
        todo!()
    }
    pub const fn digits(&self) -> [Digit; N] {
        self.digits
    }
    pub const fn from_digits(digits: [Digit; N]) -> Self {
        Self {
            digits,
        }
    }
    pub const fn from_digit(digit: Digit) -> Self {
        let mut out = Self::ZERO;
        out.digits[0] = digit;
        out
    }
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < N {
            if (&self.digits)[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }
    const fn last_digit_index(&self) -> usize {
        let mut index = 0;
        let mut i = 1;
        while i < N {
            if (&self.digits)[i] != 0 {
                index = i;
            }
            i += 1;
        }
        index
    }
    const fn last_digit(&self) -> Digit {
        let mut last_digit = self.digits[0];
        let mut i = 1;
        while i < N {
            let digit = self.digits[i];
            if digit != 0 {
                last_digit = digit;
            }
            i += 1;
        }
        last_digit
    }
    fn from_uninit<C>(mut closure: C) -> Self where C: FnMut(usize) -> Digit {
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
        let mut digits = [0; N];
        for i in 0..N {
            digits[i] = closure(i);
        }
        Self::from(digits)
    }
    fn op<C>(&self, rhs: &Self, mut closure: C) -> Self where C: FnMut(Digit, Digit) -> Digit {
        Self::from_uninit(|i| {
            closure(self.digits[i], rhs.digits[i])
        })
    }
    fn shift_left(&self, by: usize) -> Self {
        let slice = &self.digits[by..];
        let mut digits = [0; N];
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
        let mut digits = [0; N];
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
        Self::ZERO
    }
}

use std::iter::{Iterator, Product, Sum};

impl<const N: usize> Product<Self> for BUint<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for BUint<N> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for BUint<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for BUint<N> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}