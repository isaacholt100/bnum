use crate::digit::{Digit, DoubleDigit, self};
use crate::errors::option_expect;
use crate::ExpType;
use core::mem::MaybeUninit;
use crate::{doc, errors};

#[inline]
pub const fn carrying_mul(a: Digit, b: Digit, carry: Digit, current: Digit) -> (Digit, Digit) {
	// credit num_bigint source code
    let prod = carry as DoubleDigit + current as DoubleDigit + (a as DoubleDigit) * (b as DoubleDigit);
    (prod as Digit, (prod >> Digit::BITS) as Digit)
}

#[inline]
pub const fn unchecked_shl<const N: usize>(u: BUint<N>, rhs: ExpType) -> BUint<N> {
	// credit num_bigint source code
    debug_assert!(BUint::<N>::BITS <= usize::MAX as ExpType);
    if rhs == 0 {
        u
    } else {
        let digit_shift = (rhs >> digit::BIT_SHIFT) as usize;
        let shift = (rhs & digit::BITS_MINUS_1) as u8;
        
        let mut out = BUint::ZERO;
        let digits_ptr = u.digits.as_ptr();
        let out_ptr = out.digits.as_mut_ptr() as *mut Digit;
        unsafe {
            digits_ptr.copy_to_nonoverlapping(out_ptr.add(digit_shift), N - digit_shift);
        }

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

#[inline]
pub const fn unchecked_shr<const N: usize>(u: BUint<N>, rhs: ExpType) -> BUint<N> {
	// credit num_bigint source code
    // This is to make sure that the number of bits in `u` doesn't overflow a usize, which would cause unexpected behaviour for shifting
    debug_assert!(BUint::<N>::BITS <= usize::MAX as ExpType);
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
        }

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

#[cfg(feature = "serde")]
use ::{serde_big_array::BigArray, serde::{Serialize, Deserialize}};

/// Big unsigned integer type, of fixed size which must be known at compile time. Digits are stored in little endian (least significant digit first). `BUint<N>` aims to exactly replicate the behaviours of Rust's built-in unsigned integer types: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`. The const generic parameter `N` is the number of digits that are stored.

// Clippy: we can allow derivation of `Hash` and manual implementation of `PartialEq` as the derived `PartialEq` would be the same except we make our implementation const.
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BUint<const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub(crate) digits: [Digit; N],
}

macro_rules! pos_const {
    ($($name: ident $num: literal), *) => {
        $(
            #[doc=concat!("The value of ", $num, " represented by this type.")]
            pub const $name: Self = Self::from_digit($num);
        )*
    }
}

#[doc=doc::assoc_consts!()]
impl<const N: usize> BUint<N> {
    #[doc=doc::min_const!(U512)]
    pub const MIN: Self = Self::from_digits([Digit::MIN; N]);

    #[doc=doc::max_const!(U512)]
    pub const MAX: Self = Self::from_digits([Digit::MAX; N]);

    #[doc=doc::bits_const!(U512, 512)]
    pub const BITS: ExpType = digit::BITS * N as ExpType;

    #[doc=doc::bytes_const!(U512, 512)]
    pub const BYTES: ExpType = Self::BITS / 8;

	#[doc=doc::zero_const!(U512)]
	pub const ZERO: Self = Self::MIN;

    pos_const!(ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);
}

impl<const N: usize> BUint<N> {
    #[doc=doc::count_ones!(U1024)]
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

    #[doc=doc::count_zeros!(U1024)]
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

    #[doc=doc::leading_zeros!(U1024)]
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

    #[doc=doc::trailing_zeros!(U1024)]
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

    #[doc=doc::leading_ones!(U1024, MAX)]
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

    #[doc=doc::trailing_ones!(U1024)]
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
            Self::from_digits(uninit.assume_init())
        }
    }

    #[inline]
    const fn unchecked_rotate_left(self, n: ExpType) -> Self {
		// credit num_bigint source code
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

    #[doc=doc::rotate_left!(U256, "u")]
    #[inline]
    pub const fn rotate_left(self, n: ExpType) -> Self {
        self.unchecked_rotate_left(n & Self::BITS_MINUS_1)
    }

    #[doc=doc::rotate_right!(U256, "u")]
    #[inline]
    pub const fn rotate_right(self, n: ExpType) -> Self {
        let n = n & Self::BITS_MINUS_1;
        self.unchecked_rotate_left(Self::BITS as ExpType - n)
    }

    const N_MINUS_1: usize = N - 1;

    #[doc=doc::swap_bytes!(U256, "u")]
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

    #[doc=doc::reverse_bits!(U256, "u")]
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

    #[doc=doc::pow!(U256)]
    #[inline]
    pub const fn pow(self, exp: ExpType) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_pow(exp), errors::err_msg!("attempt to calculate power with overflow"));
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
    /// use bnum::BUint;
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
    /// use bnum::BUint;
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
        U256,
        "Returns `true` if and only if `self == 2^k` for some integer `k`.",
        
        "let n = " stringify!(U256) "::from(1u16 << 14);\n"
        "assert!(n.is_power_of_two());\n"
        "let m = " stringify!(U256) "::from(100u8);\n"
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

    #[doc=doc::next_power_of_two!(U256, "0", "ZERO")]
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_next_power_of_two(), errors::err_msg!("attempt to calculate next power of two with overflow"));
        #[cfg(not(debug_assertions))]
        self.wrapping_next_power_of_two()
    }

    #[doc=doc::checked_next_power_of_two!(U256)]
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_power_of_two() {
            return Some(self);
        }
		let bits = self.bits();
		if bits == Self::BITS {
			return None;
		}
		Some(Self::power_of_two(bits))
    }

    #[doc=doc::wrapping_next_power_of_two!(U256, "0")]
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
        return option_expect!(self.checked_log2(), errors::err_msg!("attempt to calculate log2 of zero"));
        #[cfg(not(debug_assertions))]
        match self.checked_log2() {
            Some(n) => n,
            None => 0,
        }
    }

    #[inline]
    pub fn log10(self) -> ExpType {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_log10(), errors::err_msg!("attempt to calculate log10 of zero"));
        #[cfg(not(debug_assertions))]
        match self.checked_log10() {
            Some(n) => n,
            None => 0,
        }
    }

    #[inline]
    pub fn log(self, base: Self) -> ExpType {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_log(base), errors::err_msg!("attempt to calculate log of zero or log with base < 2"));
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
	pub const fn next_multiple_of(self, rhs: Self) -> Self {
		let rem = self.wrapping_rem(rhs);
		if rem.is_zero() {
			self
		} else {
			self + (rhs - rem)
		}
	}

    #[inline]
    pub const fn div_floor(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    #[inline]
    pub const fn div_ceil(self, rhs: Self) -> Self {
        let (div, rem) = self.div_rem(rhs);
        if rem.is_zero() {
            div
        } else {
            div + Self::ONE
        }
    }
}

impl<const N: usize> BUint<N> {
    #[doc=doc::bits!(U256)]
    #[inline]
    pub const fn bits(&self) -> ExpType {
        Self::BITS as ExpType - self.leading_zeros()
    }

    #[doc=doc::bit!(U256)]
    #[inline]
    pub const fn bit(&self, index: ExpType) -> bool {
        let digit = self.digits[index as usize >> digit::BIT_SHIFT];
        digit & (1 << (index & digit::BITS_MINUS_1)) != 0
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
    /// use bnum::BUint;
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

    #[doc=doc::is_zero!(U256)]
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

    #[doc=doc::is_one!(U256)]
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

    #[inline]
    fn is_even(&self) -> bool {
        N == 0 || self.digits[0] & 1 == 0
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
    pub(crate) const fn to_exp_type(self) -> Option<ExpType> {
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

	#[allow(unused)]
	#[inline]
	const fn square(self) -> Self {
		// TODO: optimise this method, this will make exponentiation by squaring faster
		self * self
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
    use crate::types::U128;
	use crate::test::{test_bignum, debug_skip};

    crate::int::tests!(u128);

    test_bignum! {
		function: <u128>::checked_next_power_of_two(a: u128),
        cases: [
            (u128::MAX)
        ]
    }
    test_bignum! {
		function: <u128>::next_power_of_two(a: u128),
        skip: debug_skip!(a.checked_next_power_of_two().is_none())
    }
    test_bignum! {
		function: <u128>::wrapping_next_power_of_two(a: u128),
        cases: [(u128::MAX)]
    }

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
    fn is_one() {
        assert!(U128::ONE.is_one());
        assert!(!U128::MAX.is_one());
        assert!(!U128::ZERO.is_one());
    }

    #[test]
    fn bits() {
        let u = U128::from(0b1001010100101010101u128);
        assert_eq!(u.bits(), 19);

        let u = U128::power_of_two(78);
        assert_eq!(u.bits(), 79);
    }

    #[test]
    fn default() {
        assert_eq!(U128::default(), u128::default().into());
    }

    #[test]
    fn is_power_of_two() {
        let power = U128::from(1u128 << 88);
        let non_power = U128::from((1u128 << 88) - 5);
        assert!(power.is_power_of_two());
        assert!(!non_power.is_power_of_two());
    }
}