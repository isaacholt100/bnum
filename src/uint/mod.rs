use crate::digit::{Digit, self};
use crate::macros::expect;
use crate::ExpType;
use core::cmp::Ordering;
use core::mem::MaybeUninit;

#[allow(unused)]
macro_rules! test_unsigned {
    {
        name: $name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        }
    } => {
        test! {
            big: U128,
            primitive: u128,
            name: $name,
            method: {
                $($method ($($arg), *) ;) *
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
            big: U128,
            primitive: u128,
            name: $name,
            method: {
                $($method ($($arg), *) ;) *
            },
            converter: $converter
        }
    }
}

/*#[allow(unused)]
macro_rules! possible_const_fn {
    ($self: ident, fn $name: ident (self $(,$param: ident : $Type: ty)*) -> $ret: ty $body: block) => {
        #[cfg(feature = "intrinsics")]
        pub fn $name($self $(,$param: $Type)*) -> $ret $body
        #[cfg(not(feature = "intrinsics"))]
        pub const fn $name($self $(,$param: $Type)*) -> $ret $body
    };
}*/

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
mod cast;

#[cfg(feature = "nightly")]
pub use cast::{cast_up, cast_down};

//pub use checked::div_float;

pub const fn unchecked_shl<const N: usize>(u: BUint<N>, rhs: ExpType) -> BUint<N> {
    // This is to make sure that the number of bits in `u` doesn't overflow a usize, which would cause unexpected behaviour for shifting
    assert!(BUint::<N>::BITS <= usize::MAX);
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
            core::mem::forget(u);
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
                if new_digit != 0 {
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

pub const fn unchecked_shr<const N: usize>(u: BUint<N>, rhs: ExpType) -> BUint<N> {
    // This is to make sure that the number of bits in `u` doesn't overflow a usize, which would cause unexpected behaviour for shifting
    assert!(BUint::<N>::BITS <= usize::MAX);
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
#[cfg(feature = "serde_all")]
#[derive(Clone, Copy, Hash, /*Debug, */Serialize, Deserialize)]
pub struct BUint<const N: usize> {
    #[serde(with = "BigArray")]
    digits: [Digit; N],
}

/// Big unsigned integer type. Digits are stored as little endian (least significant bit first);
#[cfg(not(feature = "serde_all"))]
#[derive(Clone, Copy, Hash, /*Debug*/)]
pub struct BUint<const N: usize> {
    digits: [Digit; N],
}

/// Associated constants for this type.
impl<const N: usize> BUint<N> {
    /// The smallest value that can be represented by this integer type (i.e. 0).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let u128_min = u128::MIN;
    /// assert_eq!(BUint::<2>::from(u128_min), BUint::<2>::MIN);
    /// ```
    pub const MIN: Self = {
        Self {
            digits: [Digit::MIN; N],
        }
    };

    /// The largest value that can be represented by this integer type (i.e. 2^(64N) - 1).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let u128_max = u128::MAX;
    /// assert_eq!(BUint::<2>::from(u128_max), BUint::<2>::MAX);
    /// ```
    pub const MAX: Self = {
        Self {
            digits: [Digit::MAX; N],
        }
    };

    /// The value of zero represented by this type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// assert_eq!(BUint::<2>::MIN, BUint::<2>::ZERO);
    /// ```
    pub const ZERO: Self = Self::MIN;
    
    /// The value of one represented by this type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let u128_one = 1u128;
    /// assert_eq!(BUint::<2>::from(u128_one), BUint::<2>::ONE);
    /// ```
    pub const ONE: Self = Self::from_digit(1);

    /// The size of this type in bits.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// assert_eq!(BUint::<4>::BITS, 4 * 64);
    /// ```
    pub const BITS: usize = digit::BITS * N;

    /// The size of this type in bytes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// assert_eq!(BUint::<6>::BYTES, 6 * 8);
    /// ```
    pub const BYTES: usize = Self::BITS / 8;
}

impl<const N: usize> BUint<N> {
    /// Returns the number of ones in the binary representation of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<4>::from(0b010111101010000u64);
    /// assert_eq!(n.count_ones(), 7);
    /// ```
    pub const fn count_ones(self) -> ExpType {
        let mut ones = 0;
        let mut i = 0;
        while i < N {
            ones += self.digits[i].count_ones() as ExpType;
            i += 1;
        }
        ones
    }

    /// Returns the number of zeros in the binary representation of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// assert_eq!(BUint::<5>::MAX.count_zeros(), 0);
    /// ```
    pub const fn count_zeros(self) -> ExpType {
        let mut zeros = 0;
        let mut i = 0;
        while i < N {
            zeros += self.digits[i].count_zeros() as ExpType;
            i += 1;
        }
        zeros
    }

    /// Returns the number of leading zeros in the binary representation of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n: BUint::<3> = BUint::<3>::MAX >> 4;
    /// assert_eq!(n.leading_zeros(), 4);
    /// ```
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

    /// Returns the number of trailing zeros in the binary representation of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0b01111010100000u128);
    /// assert_eq!(n.trailing_zeros(), 5);
    /// ```
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

    /// Returns the number of leading ones in the binary representation of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n: BUint::<3> = BUint::<3>::MAX >> 6;
    /// assert_eq!((!n).leading_ones(), 6);
    /// ```
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

    /// Returns the number of trailing ones in the binary representation of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0b1000101011011u128);
    /// assert_eq!(n.trailing_ones(), 2);
    /// ```
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

    /// Shifts the bits to the left by a specified amount, `n`, wrapping the truncated bits to the end of the resulting integer.
    /// 
    /// Please note this isn't the same operation as the `<<` shifting operator!
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x13f40000000000000000000000004f76u128);
    /// let m = BUint::<2>::from(0x4f7613f4u128);
    ///
    /// assert_eq!(n.rotate_left(16), m);
    /// ```
    pub const fn rotate_left(self, n: ExpType) -> Self {
        self.unchecked_rotate_left(n & Self::BITS_MINUS_1)
    }

    /// Shifts the bits to the right by a specified amount, `n`, wrapping the truncated bits to the beginning of the resulting integer.
    /// 
    /// Please note this isn't the same operation as the `>>` shifting operator!
    /// 
    /// `rotate_right(n)` is equivalent to `rotate_left(Self::BITS - n)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x4f7613f4u128);
    /// let m = BUint::<2>::from(0x13f40000000000000000000000004f76u128);
    ///
    /// assert_eq!(n.rotate_right(16), m);
    /// ```
    pub const fn rotate_right(self, n: ExpType) -> Self {
        let n = n & Self::BITS_MINUS_1;
        self.unchecked_rotate_left(Self::BITS as ExpType - n)
    }
    const N_MINUS_1: usize = N - 1;

    /// Reverses the byte order of the integer.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x12345678901234567890123456789012u128);
    /// let m = BUint::<2>::from(0x12907856341290785634129078563412u128);
    /// assert_eq!(n.swap_bytes(), m);
    /// ```
    pub const fn swap_bytes(self) -> Self {
        let mut uint = Self::ZERO;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[Self::N_MINUS_1 - i].swap_bytes();
            i += 1;
        }
        uint
    }

    const fn uninit() -> [MaybeUninit<Digit>; N] {
        unsafe { MaybeUninit::<[MaybeUninit<Digit>; N]>::uninit().assume_init() }
    }

    fn init(arr: [MaybeUninit<Digit>; N]) -> Self {
        let digits = unsafe {*(&arr as *const _ as *const [Digit; N])};
        Self::from_digits(digits)
    }

    pub fn swap_bytes_test(self) -> Self {
        let mut digits = Self::uninit();
        //let mut uint = Self::ZERO;
        let mut i = 0;
        while i < N {
            digits[i] = MaybeUninit::new(self.digits[Self::N_MINUS_1 - i].swap_bytes());
            i += 1;
        }
        Self::init(digits)
    }

    /// Reverses the order of bits in the integer. The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0x12345678901234567890123456789012u128);
    /// let m = BUint::<2>::from(0x48091e6a2c48091e6a2c48091e6a2c48u128);
    ///
    /// assert_eq!(n.reverse_bits(), m);
    /// assert_eq!(BUint::<6>::ZERO, BUint::<6>::ZERO.reverse_bits());
    /// ```
    pub const fn reverse_bits(self) -> Self {
        let mut uint = Self::ZERO;
        let mut i = 0;
        while i < N {
            uint.digits[i] = self.digits[Self::N_MINUS_1 - i].reverse_bits();
            i += 1;
        }
        uint
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(3u128);
    /// let m = BUint::<2>::from(3u128.pow(5));
    /// assert_eq!(n.pow(5), m);
    /// ```
    #[cfg(debug_assertions)]
    pub const fn pow(self, exp: ExpType) -> Self {
        expect!(self.checked_pow(exp), "attempt to calculate power with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn pow(self, exp: ExpType) -> Self {
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
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem_euclid(rhs)
    }

    /// Returns `true` if and only if `self == 2^k` for some integer `k`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<3>::from(u128::MAX);
    /// assert!((n + BUint::ONE).is_power_of_two());
    /// 
    /// let m = BUint::<3>::from(100u128);
    /// assert!(!m.is_power_of_two());
    /// ```
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

    /// Returns the smallest power of two greater than or equal to `self`.
    /// 
    /// When return value overflows (i.e. `self > (1 << (64N-1)`), it panics in debug mode and the return value is wrapped to 0 in release mode (the only situation in which method can return 0).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<5>::from(2u128);
    /// assert_eq!(n.next_power_of_two(), n);
    /// 
    /// let m = BUint::<5>::from(3u128);
    /// assert_eq!(m.next_power_of_two(), BUint::from(4u128));
    /// ```
    #[cfg(debug_assertions)]
    pub const fn next_power_of_two(self) -> Self {
        expect!(self.checked_next_power_of_two(), "attempt to calculate next power of two with overflow")
    }
    #[cfg(not(debug_assertions))]
    pub const fn next_power_of_two(self) -> Self {
        self.wrapping_next_power_of_two()
    }

    /// Returns the smallest power of two greater than or equal to `self`. If the next power of two is greater than `Self::MAX`, `None` is returned, otherwise the power of two is wrapped in `Some`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<5>::from(2u128);
    /// assert_eq!(n.checked_next_power_of_two(), Some(n));
    /// 
    /// let m = BUint::<5>::from(3u128);
    /// assert_eq!(m.checked_next_power_of_two(), Some(BUint::from(4u128)));
    /// 
    /// assert_eq!(BUint::<5>::MAX.checked_next_power_of_two(), None);
    /// ```
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

    /// Returns the smallest power of two greater than or equal to `self`. If the next power of two is greater than `Self::MAX`, the return value is wrapped to 0.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<4>::from(39457897u128);
    /// assert_eq!(n.wrapping_next_power_of_two(), n.next_power_of_two());
    /// 
    /// assert_eq!(BUint::<4>::MAX.wrapping_next_power_of_two(), BUint::ZERO);
    /// ```
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::ZERO,
        }
    }

    #[cfg(debug_assertions)]
    pub const fn log2(self) -> ExpType {
        expect!(self.checked_log2(), "attempt to calculate log2 of zero")
    }
    #[cfg(not(debug_assertions))]
    pub const fn log2(self) -> ExpType {
        match self.checked_log2() {
            Some(n) => n,
            None => 0,
        }
    }

    #[cfg(debug_assertions)]
    pub const fn log10(self) -> ExpType {
        expect!(self.checked_log10(), "attempt to calculate log10 of zero")
    }
    #[cfg(not(debug_assertions))]
    pub const fn log10(self) -> ExpType {
        match self.checked_log10() {
            Some(n) => n,
            None => 0,
        }
    }

    #[cfg(debug_assertions)]
    pub const fn log(self, base: Self) -> ExpType {
        expect!(self.checked_log(base), "attempt to calculate log of zero")
    }
    #[cfg(not(debug_assertions))]
    pub const fn log(self, base: Self) -> ExpType {
        match self.checked_log(base) {
            Some(n) => n,
            None => 0,
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
            out |= d0 as u64;
            out_bits += bits_want;
            bits -= bits_want;

            if out_bits == 64 {
                break;
            }
        }
        out
    }

    /// Returns the fewest bits necessary to represent `self`.
    /// 
    /// This is equal to the size of the type in bits minus the leading zeros of `self`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0b01110001u128);
    /// assert_eq!(n.bits(), 7);
    /// 
    /// let m = BUint::<2>::from(99384759374394579457u128);
    /// assert_eq!(m.bits(), BUint::<2>::BITS - m.leading_zeros() as usize);
    /// ```
    pub const fn bits(&self) -> ExpType {
        Self::BITS as ExpType - self.leading_zeros()
    }

    /// Returns a boolean of the bit in the given position (`true` if the bit is set).
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bint::BUint;
    /// 
    /// let n = BUint::<2>::from(0b001010100101010101u128);
    /// assert!(n.bit(0));
    /// assert!(!n.bit(1));
    /// assert!(!n.bit(BUint::<2>::BITS - 1));
    /// ```
    pub const fn bit(&self, index: usize) -> bool {
        let digit = self.digits[index >> digit::BIT_SHIFT];
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
    /// use bint::BUint;
    /// 
    /// let power = 11;
    /// assert_eq!(BUint::<2>::power_of_two(11), (1u128 << 11).into());
    /// ```
    pub const fn power_of_two(power: usize) -> Self {
        let mut out = Self::ZERO;
        out.digits[power >> digit::BIT_SHIFT] = 1 << (power & (digit::BITS - 1));
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

    /// Returns whether `self` is zero.
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

    /// Returns whether `self` is one.
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
    /*fn pollard_rho(&self) -> Vec<Self> {

    }
    pub fn factors(&self) -> Vec<Self> {
        use num_traits::ToPrimitive;
        if let Some(n) = self.to_u16() {
            crate::factors::FACTORS[n as usize].iter().map(|n| {
                Self::from(*n)
            }).collect()
        } else {
            todo!()
        }
    }*/
}

use core::default::Default;

impl<const N: usize> Default for BUint<N> {
    /// Returns the default value of `Self::ZERO`.
    fn default() -> Self {
        Self::ZERO
    }
}

use core::iter::{Iterator, Product, Sum};

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

#[cfg(test)]
mod tests {
    use crate::{U128};

    test_unsigned! {
        name: count_ones,
        method: {
            count_ones(203583443659837459073490583937485738404u128);
            count_ones(3947594755489u128);
        },
        converter: crate::u32_to_exp
    }
    test_unsigned! {
        name: count_zeros,
        method: {
            count_zeros(7435098345734853045348057390485934908u128);
            count_zeros(3985789475546u128);
        },
        converter: crate::u32_to_exp
    }
    test_unsigned! {
        name: leading_ones,
        method: {
            leading_ones(3948590439409853946593894579834793459u128);
            leading_ones(u128::MAX - 0b111);
        },
        converter: crate::u32_to_exp
    }
    test_unsigned! {
        name: leading_zeros,
        method: {
            leading_zeros(49859830845963457783945789734895834754u128);
            leading_zeros(40545768945769u128);
        },
        converter: crate::u32_to_exp
    }
    test_unsigned! {
        name: trailing_ones,
        method: {
            trailing_ones(45678345973495637458973488509345903458u128);
            trailing_ones(u128::MAX);
        },
        converter: crate::u32_to_exp
    }
    test_unsigned! {
        name: trailing_zeros,
        method: {
            trailing_zeros(23488903477439859084534857349857034599u128);
            trailing_zeros(343453454565u128);
        },
        converter: crate::u32_to_exp
    }
    test_unsigned! {
        name: rotate_left,
        method: {
            rotate_left(394857348975983475983745983798579483u128, 5555 as u16);
            rotate_left(4056890546059u128, 12 as u16);
        }
    }
    test_unsigned! {
        name: rotate_right,
        method: {
            rotate_right(90845674987957297107197973489575938457u128, 10934 as u16);
            rotate_right(1345978945679u128, 33 as u16);
        }
    }
    test_unsigned! {
        name: swap_bytes,
        method: {
            swap_bytes(3749589304858934758390485937458349058u128);
            swap_bytes(3405567798345u128);
        }
    }
    test_unsigned! {
        name: reverse_bits,
        method: {
            reverse_bits(3345565093489578938485934957893745984u128);
            reverse_bits(608670986790835u128);
        }
    }
    test_unsigned! {
        name: pow,
        method: {
            pow(59345u128, 4 as u16);
            pow(54u128, 9 as u16);
        }
    }
    test_unsigned! {
        name: div_euclid,
        method: {
            div_euclid(345987945738945789347u128, 345987945738945789347u128);
            div_euclid(139475893475987093754099u128, 3459837453479u128);
        }
    }
    test_unsigned! {
        name: rem_euclid,
        method: {
            rem_euclid(8094589656797897987u128, 8094589656797897987u128);
            rem_euclid(3734597349574397598374594598u128, 3495634895793845783745897u128);
        }
    }
    #[test]
    fn is_power_of_two() {
        let power = U128::from(1u128 << 88);
        let non_power = U128::from((1u128 << 88) - 5);
        assert!(power.is_power_of_two());
        assert!(!non_power.is_power_of_two());
    }
    test_unsigned! {
        name: checked_next_power_of_two,
        method: {
            checked_next_power_of_two(1340539475937597893475987u128);
            checked_next_power_of_two(u128::MAX);
        },
        converter: |option: Option<u128>| -> Option<U128> {
            match option {
                None => None,
                Some(u) => Some(u.into()),
            }
        }
    }
    test_unsigned! {
        name: next_power_of_two,
        method: {
            next_power_of_two(394857834758937458973489573894759879u128);
            next_power_of_two(800345894358459u128);
        }
    }
    /*test_unsigned! {
        name: wrapping_next_power_of_two,
        method: {
            wrapping_next_power_of_two(97495768945869084687890u128);
            wrapping_next_power_of_two(u128::MAX);
        }
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
}