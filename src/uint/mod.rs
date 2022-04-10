use crate::digit::{Digit, DoubleDigit, self};
use crate::macros::expect;
use crate::ExpType;
use core::cmp::Ordering;
use core::mem::MaybeUninit;
use crate::doc;

#[inline]
pub const fn carrying_mul(a: Digit, b: Digit, carry: Digit, current: Digit) -> (Digit, Digit) {
    let prod = carry as DoubleDigit + current as DoubleDigit + (a as DoubleDigit) * (b as DoubleDigit);
    (prod as Digit, (prod >> Digit::BITS) as Digit)
}

#[allow(unused)]
macro_rules! test_unsigned {
    {
        function: $name: ident ($($param: ident : $ty: ty), *)
        $(,cases: [
            $(($($arg: expr), *)), *
        ])?
        $(,quickcheck_skip: $skip: expr)?
    } => {
        crate::test::test_big_num! {
            big: crate::U128,
            primitive: u128,
            function: $name,
            $(cases: [
                $(($($arg), *) ), *
            ],)?
            quickcheck: ($($param : $ty), *),
            $(quickcheck_skip: $skip,)?
            converter: Into::into
        }
    };
    {
        function: $name: ident ($($param: ident : $ty: ty), *)
        $(,cases: [
            $(($($arg: expr), *)), *
        ])?
        $(,quickcheck_skip: $skip: expr)?,
        converter: $converter: expr
    } => {
        crate::test::test_big_num! {
            big: crate::U128,
            primitive: u128,
            function: $name,
            $(cases: [
                $(($($arg), *)), *
            ],)?
            quickcheck: ($($param : $ty), *),
            $(quickcheck_skip: $skip,)?
            converter: $converter
        }
    };
}

#[cfg(feature = "nightly")]
pub use cast::{cast_up, cast_down};

//pub use checked::div_float;

pub const fn unchecked_shl<const N: usize>(u: BUint<N>, rhs: ExpType) -> BUint<N> {
    // TODO: fix the commented method
    //return unchecked_shr(u.reverse_bits(), rhs).reverse_bits();
    // This is to make sure that the number of bits in `u` doesn't overflow a usize, which would cause unexpected behaviour for shifting
    assert!(BUint::<N>::BITS <= usize::MAX);
    if rhs == 0 {
        u
    } else {
        let digit_shift = (rhs >> digit::BIT_SHIFT) as usize;
        let shift = (rhs & digit::BITS_MINUS_1) as u8;
        //println!("{}", digit_shift);
        
        let mut out = BUint::ZERO;
        let digits_ptr = u.digits.as_ptr();
        let out_ptr = out.digits.as_mut_ptr() as *mut Digit;
        unsafe {
            digits_ptr.copy_to_nonoverlapping(out_ptr.add(digit_shift), N - digit_shift);
            core::mem::forget(u);
        }
        /*if rhs == 13 {
        let mut i = 0;
        while i < N - 1 {
            assert!(out.digits[i] == 0);
            i += 1;
        }
        assert!(out.digits[N - 1] == 0b01001000);
        assert!(shift == 5);
    }*/

        if shift > 0 {
            let mut carry = 0;
            let carry_shift = Digit::BITS as u8 - shift;
            let mut last_index = digit_shift;

            let mut i = digit_shift;
            while i < N {
                let digit = out.digits[i];
                let new_carry = digit >> carry_shift;
                let new_digit = (digit << shift) | carry;
                if digit != 0 {
                    last_index = i;
                }
                out.digits[i] = new_digit;
                carry = new_carry;
                i += 1;
            }
            if rhs == 13 {
                //assert!(last_index == 1);
            }

            if carry != 0 {
                last_index += 1;
                if last_index < N {
                    out.digits[last_index] = carry;
                }
            }
        }

        out
    }
}

pub const fn unchecked_shr<const N: usize>(u: BUint<N>, rhs: ExpType) -> BUint<N> {
    // This is to make sure that the number of bits in `u` doesn't overflow a usize, which would cause unexpected behaviour for shifting
    assert!(BUint::<N>::BITS as usize <= usize::MAX);
    if rhs == 0 {
        u
    } else {
        let digit_shift = (rhs >> digit::BIT_SHIFT) as usize;
        let shift = (rhs & digit::BITS_MINUS_1) as u8;

        let mut out = BUint::ZERO;
        let digits_ptr = u.digits.as_ptr();
        let out_ptr = out.digits.as_mut_ptr() as *mut Digit;
        unsafe {
            digits_ptr.add(digit_shift).copy_to_nonoverlapping(out_ptr, N - digit_shift);
            core::mem::forget(u);
        }
        /*let mut i = 0;
        while i < N - digit_shift {
            out.digits[i] = u.digits[i + digit_shift];
            i += 1;
        }*/

        if shift > 0 {
            let mut borrow = 0;
            let borrow_shift = Digit::BITS as u8 - shift;

            let mut i = digit_shift;
            while i < N {
                let digit = out.digits[BUint::<N>::N_MINUS_1 - i];
                let new_borrow = digit << borrow_shift;
                let new_digit = (digit >> shift) | borrow;
                out.digits[BUint::<N>::N_MINUS_1 - i] = new_digit;
                borrow = new_borrow;
                i += 1;
            }
        }

        out
    }
}

#[cfg(feature = "serde_all")]
use serde_big_array::BigArray;
#[cfg(feature = "serde_all")]
use serde::{Serialize, Deserialize};

/// Big unsigned integer type. Digits are stored as little endian (least significant bit first);
#[derive(Clone, Copy, Hash, /*Debug, */)]
#[cfg_attr(feature = "serde_all", derive(Serialize, Deserialize))]
pub struct BUint<const N: usize> {
    #[cfg_attr(feature = "serde_all", serde(with = "BigArray"))]
    digits: [Digit; N],
}

macro_rules! pos_const {
    ($($name: ident $num: literal), *) => {
        $(
            #[doc=concat!("The value of ", $num, " represented by this type.")]
            pub const $name: Self = Self::from_digit($num);
        )*
    }
}

mod bigint_helpers;
mod cast;
mod checked;
mod cmp;
mod convert;
mod endian;
mod fmt;
#[cfg(feature = "numtraits")]
mod numtraits;
mod ops;
mod overflowing;
mod radix;
mod saturating;
mod unchecked;
mod wrapping;

/// Associated constants for this type.
impl<const N: usize> BUint<N> {
    #[doc=doc::min_const!(BUint::<2>)]
    pub const MIN: Self = Self::from_digits([Digit::MIN; N]);

    #[doc=doc::max_const!(BUint::<2>)]
    pub const MAX: Self = Self::from_digits([Digit::MAX; N]);

    #[doc=doc::bits_const!(BUint::<2>, 64)]
    pub const BITS: ExpType = digit::BITS * N as ExpType;

    #[doc=doc::bytes_const!(BUint::<2>, 8)]
    pub const BYTES: ExpType = Self::BITS / 8;

    #[doc=doc::zero_const!(BUint::<2>)]
    pub const ZERO: Self = Self::MIN;
    
    #[doc=doc::one_const!(BUint::<2>)]
    pub const ONE: Self = Self::from_digit(1);

    pos_const!(TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);
}

impl<const N: usize> BUint<N> {
    #[doc=doc::count_ones!(BUint::<4>)]
    #[inline]
    pub const fn count_ones(self) -> ExpType {
        let mut ones = 0;
        let mut i = 0;
        while i < N {
            ones += self.digits[i].count_ones() as ExpType;
            i += 1;
        }
        ones
    }

    #[doc=doc::count_zeros!(BUint::<5>)]
    #[inline]
    pub const fn count_zeros(self) -> ExpType {
        let mut zeros = 0;
        let mut i = 0;
        while i < N {
            zeros += self.digits[i].count_zeros() as ExpType;
            i += 1;
        }
        zeros
    }

    #[doc=doc::leading_zeros!(BUint::<3>)]
    #[inline]
    pub const fn leading_zeros(self) -> ExpType {
        let mut zeros = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.digits[i];
            zeros += digit.leading_zeros() as ExpType;
            if digit != Digit::MIN {
                break;
            }
        }
        zeros
    }

    #[doc=doc::trailing_zeros!(BUint::<4>)]
    #[inline]
    pub const fn trailing_zeros(self) -> ExpType {
        let mut zeros = 0;
        let mut i = 0;
        while i < N {
            let digit = self.digits[i];
            zeros += digit.trailing_zeros() as ExpType;
            if digit != Digit::MIN {
                break;
            }
            i += 1;
        }
        zeros
    }

    #[doc=doc::leading_ones!(BUint::<4>, MAX)]
    #[inline]   
    pub const fn leading_ones(self) -> ExpType {
        let mut ones = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            let digit = self.digits[i];
            ones += digit.leading_ones() as ExpType;
            if digit != Digit::MAX {
                break;
            }
        }
        ones
    }

    #[doc=doc::trailing_ones!(BUint::<6>)]
    #[inline]
    pub const fn trailing_ones(self) -> ExpType {
        let mut ones = 0;
        let mut i = 0;
        while i < N {
            let digit = self.digits[i];
            ones += digit.trailing_ones() as ExpType;
            if digit != Digit::MAX {
                break;
            }
            i += 1;
        }
        ones
    }

    #[inline]
    const fn rotate_digits_left(self, n: usize) -> Self {
        let mut uninit = MaybeUninit::<[Digit; N]>::uninit();
        let digits_ptr = self.digits.as_ptr();
        let uninit_ptr = uninit.as_mut_ptr() as *mut Digit;
        unsafe {
            digits_ptr.copy_to_nonoverlapping(uninit_ptr.add(n), N - n);
            digits_ptr.add(N - n).copy_to_nonoverlapping(uninit_ptr, n);
            core::mem::forget(self);
            Self::from_digits(uninit.assume_init())
        }
    }

    #[inline]
    const fn unchecked_rotate_left(self, n: ExpType) -> Self {
        if n == 0 {
            self
        } else {
            let digit_shift = (n >> digit::BIT_SHIFT) as usize % N;
            let shift = (n % digit::BITS) as u8;

            let carry_shift = Digit::BITS as u8 - shift;

            let mut out = self.rotate_digits_left(digit_shift);

            if shift > 0 {
                let mut carry = 0;

                let mut i = 0;
                while i < N {
                    let digit = out.digits[i];
                    let new_carry = digit >> carry_shift;
                    out.digits[i] = (digit << shift) | carry;
                    carry = new_carry;
                    i += 1;
                }
    
                out.digits[0] |= carry;
            }

            out
        }
    }
    const BITS_MINUS_1: ExpType = (Self::BITS - 1) as ExpType;

    #[doc=doc::rotate_left!(BUint::<2>, "u")]
    #[inline]
    pub const fn rotate_left(self, n: ExpType) -> Self {
        self.unchecked_rotate_left(n & Self::BITS_MINUS_1)
    }

    #[doc=doc::rotate_right!(BUint::<2>, "u")]
    #[inline]
    pub const fn rotate_right(self, n: ExpType) -> Self {
        let n = n & Self::BITS_MINUS_1;
        self.unchecked_rotate_left(Self::BITS as ExpType - n)
    }

    const N_MINUS_1: usize = N - 1;

    #[doc=doc::swap_bytes!(BUint::<2>, "u")]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
        let mut uint = Self::ZERO;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[Self::N_MINUS_1 - i].swap_bytes();
            i += 1;
        }
        uint
    }

    #[doc=doc::reverse_bits!(BUint::<6>, "u")]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
        let mut uint = Self::ZERO;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[Self::N_MINUS_1 - i].reverse_bits();
            i += 1;
        }
        uint
    }

    #[doc=doc::pow!(BUint::<4>)]
    #[inline]
    pub const fn pow(self, exp: ExpType) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_pow(exp), "attempt to calculate power with overflow");
        #[cfg(not(debug_assertions))]
        self.wrapping_pow(exp)
    }

    /// Performs Euclidean division.
    ///
    /// Since, for the positive integers, all common definitions of division are equal, this is exactly equal to `self / rhs`.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is 0.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<4>::from(9u128);
    /// let m = BUint::<4>::from(5u128);
    /// assert_eq!(n.div_euclid(m), BUint::ONE);
    /// ```
    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div_euclid(rhs)
    }

    /// Calculates the least remainder of `self (mod rhs)`.
    ///
    /// Since, for the positive integers, all common definitions of division are equal, this is exactly equal to `self % rhs`.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `rhs` is 0.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<4>::from(11u128);
    /// let m = BUint::<4>::from(5u128);
    /// assert_eq!(n.rem_euclid(m), BUint::ONE);
    /// ```
    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem_euclid(rhs)
    }

    #[doc=doc::doc_comment! {
        BUint::<2>,
        "Returns `true` if and only if `self == 2^k` for some integer `k`.",
        
        "let n = " doc::int_str!(BUint::<2>) "::from(1u16 << 14);\n"
        "assert!(n.is_power_of_two());\n"
        "let m = " doc::int_str!(BUint::<2>) "::from(100u8);\n"
        "assert!(!m.is_power_of_two());"
    }]
    #[inline]
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

    #[doc=doc::next_power_of_two!(BUint::<2>, "0", "ZERO")]
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        #[cfg(debug_assertions)]
        return expect!(self.checked_next_power_of_two(), "attempt to calculate next power of two with overflow");
        #[cfg(not(debug_assertions))]
        self.wrapping_next_power_of_two()
    }

    #[doc=doc::checked_next_power_of_two!(BUint::<2>)]
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_power_of_two() {
            return Some(self);
        }
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
            out.digits[last_set_digit_index] = 1 << (Digit::BITS - leading_zeros);
            Some(out)
        }
    }

    #[doc=doc::wrapping_next_power_of_two!(BUint::<2>, "0")]
    #[inline]
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::ZERO,
        }
    }

    #[inline]
    pub const fn log2(self) -> ExpType {
        #[cfg(debug_assertions)]
        return expect!(self.checked_log2(), "attempt to calculate log2 of zero");
        #[cfg(not(debug_assertions))]
        match self.checked_log2() {
            Some(n) => n,
            None => 0,
        }
    }

    #[inline]
    pub const fn log10(self) -> ExpType {
        #[cfg(debug_assertions)]
        return expect!(self.checked_log10(), "attempt to calculate log10 of zero");
        #[cfg(not(debug_assertions))]
        match self.checked_log10() {
            Some(n) => n,
            None => 0,
        }
    }

    #[inline]
    pub const fn log(self, base: Self) -> ExpType {
        #[cfg(debug_assertions)]
        return expect!(self.checked_log(base), "attempt to calculate log of zero");
        #[cfg(not(debug_assertions))]
        match self.checked_log(base) {
            Some(n) => n,
            None => 0,
        }
    }

    #[inline]
    pub const fn abs_diff(self, other: Self) -> Self {
        if self < other {
            other.wrapping_sub(self)
        } else {
            self.wrapping_sub(other)
        }
    }

    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        let rem = match self.checked_rem(rhs) {
            Some(rem) => rem,
            None => return None,
        };
        if rem.is_zero() {
            Some(self)
        } else {
            self.checked_add(rhs - rem)
        }
    }

    #[inline]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
        let rem = self % rhs;
        if rem.is_zero() {
            self
        } else {
            self + (rhs - rem)
        }
    }

    #[inline]
    pub const fn div_floor(self, rhs: Self) -> Self {
        self / rhs
    }

    #[inline]
    pub const fn div_ceil(self, rhs: Self) -> Self {
        let (div, rem) = self.div_rem(rhs);
        if rem.is_zero() {
            div
        } else {
            div + BUint::ONE
        }
    }
}

impl<const N: usize> BUint<N> {
    const fn to_mantissa(&self) -> u64 {
        if N == 0 {
            return 0;
        }
        let bits = self.bits();
        if bits <= digit::BITS {
            return self.digits[0] as u64;
        }
        let mut bits = bits;
        let mut out: u64 = 0;
        let mut out_bits = 0;

        const fn min(a: ExpType, b: ExpType) -> ExpType {
            if a < b {
                a
            } else {
                b
            }
        }

        let mut i = self.last_digit_index() + 1;
        while i > 0 {
            i -= 1;
            let digit_bits = ((bits - 1) & digit::BITS_MINUS_1) + 1;
            let bits_want = min(64 - out_bits, digit_bits);
            if bits_want != 64 {
                out <<= bits_want;
            }
            let d0 = self.digits[i] as u64 >> (digit_bits - bits_want);
            out |= d0;
            out_bits += bits_want;
            bits -= bits_want;

            if out_bits == 64 {
                break;
            }
        }
        out
    }

    #[doc=doc::bits!(BUint::<2>)]
    #[inline]
    pub const fn bits(&self) -> ExpType {
        Self::BITS as ExpType - self.leading_zeros()
    }

    #[doc=doc::bit!(BUint::<4>)]
    #[inline]
    pub const fn bit(&self, index: usize) -> bool {
        let digit = self.digits[index >> digit::BIT_SHIFT];
        digit & (1 << (index & digit::BITS_MINUS_1 as usize)) != 0
    }

    /// Returns a `BUint` whose value is `2^power`.
    /// 
    /// # Panics
    /// 
    /// This function will panic if `power` is greater than or equal to `Self::BITS`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let power = 11;
    /// assert_eq!(BUint::<2>::power_of_two(11), (1u128 << 11).into());
    /// ```
    #[inline]
    pub const fn power_of_two(power: ExpType) -> Self {
        let mut out = Self::ZERO;
        out.digits[power as usize >> digit::BIT_SHIFT] = 1 << (power & (digit::BITS - 1));
        out
    }

    /// Returns the digits stored in `self` as an array. Digits are little endian (least significant digit first).
    #[inline(always)]
    pub const fn digits(&self) -> &[Digit; N] {
        &self.digits
    }

    /// Creates a new `BUint` from the given array of digits. Digits are stored as little endian (least significant digit first).
    #[inline(always)]
    pub const fn from_digits(digits: [Digit; N]) -> Self {
        Self {
            digits,
        }
    }

    /// Creates a new `BUint` from the given digit. The given digit is stored as the least significant digit.
    #[inline(always)]
    pub const fn from_digit(digit: Digit) -> Self {
        let mut out = Self::ZERO;
        out.digits[0] = digit;
        out
    }

    #[doc=doc::is_zero!(BUint::<2>)]
    #[inline]
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

    #[doc=doc::is_one!(BUint::<2>)]
    #[inline]
    pub const fn is_one(&self) -> bool {
        if N == 0 || self.digits[0] != 1 {
            return false;
        }
        let mut i = 1;
        while i < N {
            if (&self.digits)[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Returns the smallest of `self` and `other`.
    pub const fn min(&self, other: &Self) -> Self {
        match Self::cmp(&self, &other) {
            Ordering::Greater => *other,
            _ => *self,
        }
    }

    /// Returns the largest of `self` and `other`.
    pub const fn max(&self, other: &Self) -> Self {
        match Self::cmp(&self, &other) {
            Ordering::Less => *other,
            _ => *self,
        }
    }

    /// Calculates the greatest common denominator of `self` and `other`.
    #[inline]
    pub const fn gcd(self, other: Self) -> Self {
        //use std::mem;
        let (mut u, mut v) = (self, other);
        if u.is_zero() {
            return v;
        } else if v.is_zero() {
            return u;
        }
        let i = u.trailing_zeros();
        u = unchecked_shr(u, i);
        let j = v.trailing_zeros();
        v = unchecked_shr(v, j);
        let k = if i > j { j } else { i };

        loop {
            if let Ordering::Greater = u.cmp(&v)  {
                let t = (u, v);
                v = t.0;
                u = t.1;
            }
            v = v.wrapping_sub(u);
            if v.is_zero() {
                return unchecked_shl(u, k);
            }
            v = unchecked_shr(v, v.trailing_zeros());
        }
    }

    #[inline]
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

    #[inline]
    pub const fn to_exp_type(self) -> Option<ExpType> {
        let last_index = self.last_digit_index();
        if self.digits[last_index] == 0 {
            return Some(0);
        }
        if last_index >= ExpType::BITS as usize >> digit::BIT_SHIFT {
            return None;
        }
        let mut out = 0;
        let mut i = 0;
        while i <= last_index {
            out |= (self.digits[i] as ExpType) << (i << digit::BIT_SHIFT);
            i += 1;
        }
        Some(out)
    }
}

use core::default::Default;

impl<const N: usize> const Default for BUint<N> {
    #[doc=doc::default!()]
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

use core::iter::{Iterator, Product, Sum};

impl<const N: usize> Product<Self> for BUint<N> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for BUint<N> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for BUint<N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for BUint<N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use crate::ExpType;
    use crate::U128;
    use crate::test;

    test_unsigned! {
        function: count_ones(a: u128),
        cases: [
            (203583443659837459073490583937485738404u128),
            (3947594755489u128)
        ],
        converter: test::u32_to_exp
    }
    test_unsigned! {
        function: count_zeros(a: u128),
        cases: [
            (7435098345734853045348057390485934908u128),
            (3985789475546u128)
        ],
        converter: test::u32_to_exp
    }
    test_unsigned! {
        function: leading_ones(a: u128),
        cases: [
            (3948590439409853946593894579834793459u128),
            (u128::MAX - 0b111)
        ],
        converter: test::u32_to_exp
    }
    test_unsigned! {
        function: leading_zeros(a: u128),
        cases: [
            (49859830845963457783945789734895834754u128),
            (40545768945769u128)
        ],
        converter: test::u32_to_exp
    }
    test_unsigned! {
        function: trailing_ones(a: u128),
        cases: [
            (45678345973495637458973488509345903458u128),
            (u128::MAX)
        ],
        converter: test::u32_to_exp
    }
    test_unsigned! {
        function: trailing_zeros(a: u128),
        cases: [
            (23488903477439859084534857349857034599u128),
            (343453454565u128)
        ],
        converter: test::u32_to_exp
    }
    test_unsigned! {
        function: rotate_left(a: u128, b: u16),
        cases: [
            (394857348975983475983745983798579483u128, 5555 as u16),
            (4056890546059u128, 12 as u16)
        ]
    }
    test_unsigned! {
        function: rotate_right(a: u128, b: u16),
        cases: [
            (90845674987957297107197973489575938457u128, 10934 as u16),
            (1345978945679u128, 33 as u16)
        ]
    }
    test_unsigned! {
        function: swap_bytes(a: u128),
        cases: [
            (3749589304858934758390485937458349058u128),
            (3405567798345u128)
        ]
    }
    test_unsigned! {
        function: reverse_bits(a: u128),
        cases: [
            (3345565093489578938485934957893745984u128),
            (608670986790835u128)
        ]
    }
    test_unsigned! {
        function: pow(a: u128, b: u16),
        cases: [
            (59345u128, 4 as u16),
            (54u128, 9 as u16)
        ],
        quickcheck_skip: a.checked_pow(b as u32).is_none()
    }
    test_unsigned! {
        function: div_euclid(a: u128, b: u128),
        cases: [
            (345987945738945789347u128, 345987945738945789347u128),
            (139475893475987093754099u128, 3459837453479u128),
            (84949881323520u128, 9393082u128)
        ],
        quickcheck_skip: b == 0
    }
    test_unsigned! {
        function: rem_euclid(a: u128, b: u128),
        cases: [
            (8094589656797897987u128, 8094589656797897987u128),
            (3734597349574397598374594598u128, 3495634895793845783745897u128)
        ],
        quickcheck_skip: b == 0
    }
    #[test]
    fn is_power_of_two() {
        let power = U128::from(1u128 << 88);
        let non_power = U128::from((1u128 << 88) - 5);
        assert!(power.is_power_of_two());
        assert!(!non_power.is_power_of_two());
    }
    test_unsigned! {
        function: checked_next_power_of_two(a: u128),
        cases: [
            (1340539475937597893475987u128),
            (u128::MAX)
        ],
        converter: |option: Option<u128>| option.map(|u| U128::from(u))
    }
    test_unsigned! {
        function: next_power_of_two(a: u128),
        cases: [
            (394857834758937458973489573894759879u128),
            (800345894358459u128)
        ],
        quickcheck_skip: a.checked_next_power_of_two().is_none()
    }
    test_unsigned! {
        function: log(u: u128, base: u128),
        quickcheck_skip: u == 0 || base <= 1,
        converter: |u| u as ExpType
    }
    test_unsigned! {
        function: log2(u: u128),
        quickcheck_skip: u == 0,
        converter: |u| u as ExpType
    }
    test_unsigned! {
        function: log10(u: u128),
        quickcheck_skip: u == 0,
        converter: |u| u as ExpType
    }
    /*test_unsigned! {
        function: wrapping_next_power_of_two,
        cases: [
            (97495768945869084687890u128),
            (u128::MAX)
        ]
    }*/
    #[test]
    fn bit() {
        let u = U128::from(0b001010100101010101u128);
        assert!(u.bit(0));
        assert!(!u.bit(1));
        assert!(!u.bit(17));
        assert!(!u.bit(16));
        assert!(u.bit(15));
    }
    #[test]
    fn is_zero() {
        assert!(U128::MIN.is_zero());
        assert!(!U128::MAX.is_zero());
        assert!(!U128::ONE.is_zero());
    }
    #[test]
    fn bits() {
        let u = U128::from(0b1001010100101010101u128);
        assert_eq!(u.bits(), 19);

        let u = U128::power_of_two(78);
        assert_eq!(u.bits(), 79);
    }

    #[test]
    fn checked_next_multiple_of() {
        assert_eq!(U128::from(16u8).checked_next_multiple_of(8u8.into()), Some(16u8.into()));
        assert_eq!(U128::from(23u8).checked_next_multiple_of(8u8.into()), Some(24u8.into()));
        assert_eq!(U128::ONE.checked_next_multiple_of(0u8.into()), None);
        assert_eq!(U128::from(u128::MAX).checked_next_multiple_of(2u8.into()), None);
    }
}