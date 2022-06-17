use crate::digit::{Digit, SignedDigit, self};
use crate::buint::BUint;
#[allow(unused_imports)]
use crate::types::I128;
use crate::macros::option_expect;
use crate::ExpType;
use crate::{doc, error};

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

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Big signed integer type, of fixed size which must be known at compile time. Digits are stored in little endian (least significant digit first). `BInt<N>` aims to exactly replicate the behaviours of Rust's built-in signed integer types: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`. The const generic parameter `N` is the number of digits that are stored in the underlying `BUint`.

// Clippy: we can allow derivation of `Hash` and manual implementation of `PartialEq` as the derived `PartialEq` would be the same except we make our implementation const.
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BInt<const N: usize> {
    bits: BUint<N>,
}

macro_rules! pos_const {
    ($($name: ident $num: literal), *) => {
        $(
            #[doc=concat!("The value of ", $num, " represented by this type.")]
            pub const $name: Self = Self::from_bits(BUint::$name);
        )*
    }
}

macro_rules! neg_const {
    ($($name: ident $pos: ident $num: literal), *) => {
        $(
            #[doc=concat!("The value of -", $num, " represented by this type.")]
            pub const $name: Self = Self::$pos.wrapping_neg();
        )*
    }
}

#[doc=doc::assoc_consts!()]
impl<const N: usize> BInt<N> {
    #[doc=doc::min_const!(I512)]
    pub const MIN: Self = {
        let mut digits = [0; N];
        digits[N - 1] = 1 << (digit::BITS - 1);
        Self::from_bits(BUint::from_digits(digits))
    };

    #[doc=doc::max_const!(I512)]
    pub const MAX: Self = {
        let mut digits = [Digit::MAX; N];
        digits[N - 1] >>= 1;
        Self::from_bits(BUint::from_digits(digits))
    };

    #[doc=doc::zero_const!(I512)]
    pub const ZERO: Self = {
        Self::from_bits(BUint::ZERO)
    };

    #[doc=doc::one_const!(I512)]
    pub const ONE: Self = {
        Self::from_bits(BUint::ONE)
    };

    pos_const!(TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

    neg_const!(NEG_ONE ONE 1, NEG_TWO TWO 2, NEG_THREE THREE 3, NEG_FOUR FOUR 4, NEG_FIVE FIVE 5, NEG_SIX SIX 6, NEG_SEVEN SEVEN 7, NEG_EIGHT EIGHT 8, NEG_NINE NINE 9, NEG_TEN TEN 10);

    #[doc=doc::bits_const!(I512, 512)]
    pub const BITS: ExpType = BUint::<N>::BITS;

    #[doc=doc::bytes_const!(I512, 512)]
    pub const BYTES: ExpType = BUint::<N>::BYTES;

    const N_MINUS_1: usize = N - 1;
}

macro_rules! log {
    ($method: ident $(, $base: ident : $ty: ty)?) => {
        #[inline]
        pub const fn $method(self, $($base : $ty),*) -> ExpType {
            if self.is_negative() {
                #[cfg(debug_assertions)]
                panic!(crate::error::err_msg!("attempt to calculate log of negative number"));
                #[cfg(not(debug_assertions))]
                0
            } else {
                self.bits.$method($($base.bits)?)
            }
        }
    }
}

impl<const N: usize> BInt<N> {
    #[doc=doc::count_ones!(I256)]
    #[inline]
    pub const fn count_ones(self) -> ExpType {
        self.bits.count_ones()
    }

    #[doc=doc::count_zeros!(I256)]
    #[inline]
    pub const fn count_zeros(self) -> ExpType {
        self.bits.count_zeros()
    }

    #[doc=doc::leading_zeros!(I256)]
    #[inline]
    pub const fn leading_zeros(self) -> ExpType {
        self.bits.leading_zeros()
    }

    #[doc=doc::trailing_zeros!(I256)]
    #[inline]
    pub const fn trailing_zeros(self) -> ExpType {
        self.bits.trailing_zeros()
    }

    #[doc=doc::leading_ones!(I256, NEG_ONE)]
    #[inline]
    pub const fn leading_ones(self) -> ExpType {
        self.bits.leading_ones()
    }

    #[doc=doc::trailing_ones!(I256)]
    #[inline]
    pub const fn trailing_ones(self) -> ExpType {
        self.bits.trailing_ones()
    }

    #[doc=doc::rotate_left!(I256, "i")]
    #[inline]
    pub const fn rotate_left(self, n: ExpType) -> Self {
        Self::from_bits(self.bits.rotate_left(n))
    }

    #[doc=doc::rotate_right!(I256, "i")]
    #[inline]
    pub const fn rotate_right(self, n: ExpType) -> Self {
        Self::from_bits(self.bits.rotate_right(n))
    }

    #[doc=doc::swap_bytes!(I256, "i")]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
        Self::from_bits(self.bits.swap_bytes())
    }

    #[doc=doc::reverse_bits!(I256, "i")]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
        Self::from_bits(self.bits.reverse_bits())
    }

    /// Computes the absolute value of `self` without any wrapping or panicking.
    #[doc=doc::example_header!(BInt)]
    /// assert_eq!(BInt::<3>::from(100).unsigned_abs(), BInt::from(100));
    /// assert_eq!(BInt::<3>::from(-100).unsigned_abs(), BInt::from(100));
    /// assert_eq!(BInt::<3>::MAX.unsigned_abs(), BInt::MAX.to_bits());
    /// ```
    #[inline]
    pub const fn unsigned_abs(self) -> BUint<N> {
        if self.is_negative() {
            self.wrapping_neg().bits
        } else {
            self.bits
        }
    }

    #[doc=doc::pow!(I256)]
    #[inline]
    pub const fn pow(self, exp: ExpType) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_pow(exp), error::err_msg!("attempt to calculate power with overflow"));

        #[cfg(not(debug_assertions))]
        self.wrapping_pow(exp)
    }

    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        assert!(self != Self::MIN || rhs != Self::NEG_ONE, error::err_msg!("attempt to divide with overflow"));
        self.wrapping_div_euclid(rhs)
    }

    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        assert!(self != Self::MIN || rhs != Self::NEG_ONE, error::err_msg!("attempt to calculate remainder with overflow"));
        self.wrapping_rem_euclid(rhs)
    }

    #[inline]
    pub const fn abs(self) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_abs(), error::err_msg!("attempt to negate with overflow"));

        #[cfg(not(debug_assertions))]
        match self.checked_abs() {
            Some(int) => int,
            None => Self::MIN,
        }
    }

    #[inline]
    pub const fn signum(self) -> Self {
        if self.is_negative() {
            Self::NEG_ONE
        } else if self.is_zero() {
            Self::ZERO
        } else {
            Self::ONE
        }
    }

    #[inline]
    pub const fn is_positive(self) -> bool {
        let signed_digit = self.signed_digit();
        signed_digit.is_positive() ||
        (signed_digit == 0 && !self.bits.is_zero())
    }
    
    #[inline]
    pub const fn is_negative(self) -> bool {
        self.signed_digit().is_negative()
    }
    
    #[doc=doc::doc_comment! {
        I256,
        "Returns `true` if and only if `self == 2^k` for some integer `k`.",
        
        "let n = " stringify!(I256) "::from(1i16 << 12);\n"
        "assert!(n.is_power_of_two());\n"
        "let m = " stringify!(I256) "::from(90i8);\n"
        "assert!(!m.is_power_of_two());"
        "assert!(!((-n).is_power_of_two()));"
    }]
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
        if self.is_negative() {
            false
        } else {
            self.bits.is_power_of_two()
        }
    }

    #[doc=doc::next_power_of_two!(I256, "`Self::MIN`", "NEG_ONE")]
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_next_power_of_two(), error::err_msg!("attempt to calculate next power of two with overflow"));

        #[cfg(not(debug_assertions))]
        self.wrapping_next_power_of_two()
    }

    #[doc=doc::checked_next_power_of_two!(I256)]
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_negative() {
            Some(Self::ONE)
        } else {
            match self.bits.checked_next_power_of_two() {
                Some(uint) => {
                    let out = Self::from_bits(uint);
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

    #[doc=doc::wrapping_next_power_of_two!(I256, "`Self::MIN`")]
    #[inline]
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::MIN,
        }
    }

    log!(log, base: Self);
    log!(log2);
    log!(log10);

    #[inline]
    pub const fn abs_diff(self, other: Self) -> BUint<N> {
        if self < other {
            other.wrapping_sub(self).to_bits()
        } else {
            self.wrapping_sub(other).to_bits()
        }
    }

    #[inline]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
        let rem = self.rem_euclid(rhs);
		if rhs.is_negative() {
			self - rem
		} else {
			if rem.is_zero() {
				self
			} else {
				self + (rhs - rem)
			}
		}
    }

    #[inline]
	pub const fn div_floor(self, rhs: Self) -> Self {
		if rhs.is_zero() {
			crate::macros::div_zero!();
		}
		let (div, rem) = self.div_rem_unchecked(rhs);
		if rem.is_zero() || self.is_negative() == rhs.is_negative() {
			div
		} else {
			div - Self::ONE
		}
	}

    #[inline]
	pub const fn div_ceil(self, rhs: Self) -> Self {
		if rhs.is_zero() {
			crate::macros::div_zero!();
		}
		let (div, rem) = self.div_rem_unchecked(rhs);
		if rem.is_zero() || self.is_negative() != rhs.is_negative() {
			div
		} else {
			div + Self::ONE
		}
	}
}

impl<const N: usize> BInt<N> {
    #[doc=doc::bits!(I256)]
    #[inline]
    pub const fn bits(&self) -> ExpType {
        self.bits.bits()
    }

    #[doc=doc::bit!(I256)]
    #[inline]
    pub const fn bit(&self, b: ExpType) -> bool {
        self.bits.bit(b)
    }

    #[inline(always)]
    const fn signed_digit(&self) -> SignedDigit {
        self.bits.digits[N - 1] as SignedDigit
    }

    #[doc=doc::is_zero!(I256)]
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.bits.is_zero()
    }

    #[doc=doc::is_one!(I256)]
    #[inline]
    pub const fn is_one(&self) -> bool {
        self.bits.is_one()
    }

    #[inline(always)]
    pub const fn digits(&self) -> &[Digit; N] {
        &self.bits.digits
    }

    #[inline(always)]
    pub const fn from_digits(digits: [Digit; N]) -> Self {
        Self::from_bits(BUint::from_digits(digits))
    }

    #[inline(always)]
    pub const fn from_bits(bits: BUint<N>) -> Self {
        Self {
            bits,
        }
    }
    
    #[inline(always)]
    pub const fn to_bits(self) -> BUint<N> {
        self.bits
    }

    #[inline]
    pub(crate) const fn to_exp_type(self) -> Option<ExpType> {
        if self.is_negative() {
            None
        } else {
            self.bits.to_exp_type()
        }
    }
}

use core::default::Default;

impl<const N: usize> Default for BInt<N> {
    #[doc=doc::default!()]
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

use core::iter::{Iterator, Product, Sum};

impl<const N: usize> Product<Self> for BInt<N> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for BInt<N> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for BInt<N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for BInt<N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	use crate::test::{test_bignum, debug_skip};

    crate::int::tests!(i128);

    test_bignum! {
        function: <i128>::unsigned_abs(a: i128),
        cases: [
            (i128::MIN),
            (0)
        ]
    }
    test_bignum! {
        function: <i128>::abs(a: i128),
        skip: debug_skip!(a == i128::MIN)
    }
    test_bignum! {
        function: <i128>::signum(a: i128)
    }
    test_bignum! {
        function: <i128>::is_positive(a: i128)
    }
    test_bignum! {
        function: <i128>::is_negative(a: i128)
    }
    
    #[test]
    fn bit() {
        let i = I128::from(0b1001010100101010101i128);
        assert!(i.bit(2));
        assert!(!i.bit(3));
        assert!(i.bit(8));
        assert!(!i.bit(9));
        assert!(i.bit(i.bits() - 1));
    }

    #[test]
    fn is_zero() {
        assert!(I128::ZERO.is_zero());
        assert!(!I128::MAX.is_zero());
        assert!(!I128::ONE.is_zero());
    }

    #[test]
    fn is_one() {
        assert!(I128::ONE.is_one());
        assert!(!I128::MAX.is_one());
        assert!(!I128::ZERO.is_one());
    }

    #[test]
    fn bits() {
        let u = I128::from(0b11101001010100101010101i128);
        assert_eq!(u.bits(), 23);
    }

    #[test]
    fn default() {
        assert_eq!(I128::default(), i128::default().into());
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
}