use crate::Integer;
use crate::cast::CastFrom;
use core::fmt::{self, Debug, Formatter};
use super::TestConvert;
use quickcheck::Arbitrary;

// This is a simple integer type which we use to test against methods on `Integer` that access the underlying bytes, or use wide digits.
// the idea is that the correctnesss of any methods not accessing the underlying bytes or using wide digits will depend on the correctness of the methods (down the call stack) that do access the underlying bytes or use wide digits
// so enough to test these methods just against Rust's primitives
// we test the methods on Integer that do access the underlying bytes or use wide digits against this type
// BitInt has simple, slow but correct implementations of all the methods we need to test against
// and we can verify that BitInt's methods are correct by testing Integer against BitInt for bit widths 16, 32, 64, 128. Since Integer is also tested against Rust's primitives for these bit widths, we are effectively transitively testing BitInt against Rust's primitives as well
// and importantly BitInt's implementation does not depend on the number of bits B, whereas Integer's implementation does. so by transitively testing BitInt against Rust's primtives as above, we are effectively guaranteeing that BitInt's implementation is correct for all bit widths B
// and then we use this guarantee to test Integer against BitInt for custom bit widths that we can't test using Rust's primitives directly
#[derive(Clone, Copy)]
pub struct BitInt<const S: bool, const B: usize> {
    bits: [bool; B],
}

impl<const S: bool, const B: usize> BitInt<S, B> {
    const ZERO: Self = Self { bits: [false; B] };

    pub const MIN: Self = if S {
        let mut bits = [false; B];
        bits[B - 1] = true; // sign bit
        Self { bits }
    } else {
        Self::ZERO
    };

    pub const MAX: Self = if S {
        let mut bits = [true; B];
        bits[B - 1] = false; // sign bit
        Self { bits }
    } else {
        Self { bits: [true; B] }
    };

    pub fn set_bit(&mut self, index: usize, value: bool) {
        self.bits[index] = value;
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.bits.iter().zip(other.bits.iter()).all(|(a, b)| a == b)
    }

    fn is_negative(&self) -> bool {
        if S { self.bits[B - 1] } else { false }
    }
    pub fn overflowing_shr(self, n: usize) -> (Self, bool) {
        let (n, overflow) = (n % B, n >= B);
        let mut out = if self.is_negative() {
            [true; B]
        } else {
            [false; B]
        };
        for i in n..B {
            out[i - n] = self.bits[i];
        }
        (Self { bits: out }, overflow)
    }

    pub fn bit(&self, index: usize) -> bool {
        self.bits[index]
    }
    pub fn rotate_left(mut self, n: usize) -> Self {
        self.bits.rotate_right(n % B); // because we use little-endian bit ordering
        self
    }

    pub fn rotate_right(mut self, n: usize) -> Self {
        self.bits.rotate_left(n % B); // because we use little-endian bit ordering
        self
    }

    pub fn overflowing_shl(self, n: usize) -> (Self, bool) {
        let (n, overflow) = (n % B, n >= B);
        let mut out = [false; B];
        for i in 0..(B - n) {
            out[i + n] = self.bits[i];
        }
        (Self { bits: out }, overflow)
    }

    pub fn trailing_ones(self) -> u32 {
        let mut count = 0;
        for &bit in self.bits.iter() {
            if bit {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn leading_ones(self) -> u32 {
        let mut count = 0;
        for &bit in self.bits.iter().rev() {
            if bit {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn leading_zeros(self) -> u32 {
        self.not().leading_ones()
    }

    pub fn trailing_zeros(self) -> u32 {
        self.not().trailing_ones()
    }

    pub fn count_ones(self) -> u32 {
        self.bits.iter().filter(|&&b| b).count() as u32
    }

    pub fn count_zeros(self) -> u32 {
        self.bits.iter().filter(|&&b| !b).count() as u32
    }

    pub fn reverse_bits(mut self) -> Self {
        self.bits.reverse();
        self
    }
}

impl<const B: usize> BitInt<false, B> {
    pub fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
        for i in (0..B).rev() {
            if self.bits[i] != rhs.bits[i] {
                return if self.bits[i] {
                    core::cmp::Ordering::Greater
                } else {
                    core::cmp::Ordering::Less
                };
            }
        }
        core::cmp::Ordering::Equal
    }

    pub fn power_of_two(n: usize) -> Self {
        let mut out = Self { bits: [false; B] };
        out.set_bit(n, true);
        out
    }

    pub fn is_power_of_two(self) -> bool {
        self.count_ones() == 1
    }

    pub fn is_zero(&self) -> bool {
        self.eq(&Self::ZERO)
    }

    pub fn is_one(&self) -> bool {
        self.bit(0) && self.is_power_of_two()
    }

    pub fn widening_mul(self, rhs: Self) -> (Self, Self) {
        let mut low = Self::ZERO;
        let mut high = Self::ZERO;
        for i in 0..B {
            let mut carry = false;
            for j in 0..B {
                let index = i + j;
                let c = if index < B {
                    low.bit(index)
                } else {
                    high.bit(index - B)
                };
                let prod = (self.bits[i] & rhs.bits[j]) ^ carry ^ c; // self.bits[i] * rhs.bits[j] + carry + c
                carry = ((self.bits[i] & rhs.bits[j]) as u8 + carry as u8 + c as u8) >= 2;

                if index < B {
                    low.set_bit(index, prod);
                } else {
                    high.set_bit(index - B, prod);
                }
            }
            high.set_bit(i, carry);
        }
        (low, high)
    }

    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (low, high) = self.widening_mul(rhs);
        let overflow = !high.is_zero();
        (low, overflow)
    }

    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut carry = false;
        for i in 0..B {
            let sum = (self.bits[i] as u8) + (rhs.bits[i] as u8) + (carry as u8);
            out.bits[i] = (sum % 2) != 0;
            carry = sum >= 2;
        }
        (out, carry)
    }

    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut borrow = false;
        for i in 0..B {
            out.bits[i] = self.bits[i] ^ rhs.bits[i] ^ borrow;
            let sub = (self.bits[i] as i8) - (rhs.bits[i] as i8) - (borrow as i8);
            borrow = sub < 0;
        }
        (out, borrow)
    }

    fn div_rem(self, rhs: Self) -> (Self, Self) {
        // restoring division algorithm
        // https://www.geeksforgeeks.org/computer-organization-architecture/restoring-division-algorithm-unsigned-integer/
        let mut quotient = self;
        let mut remainder = Self::ZERO;
        for _ in 0..B {
            let carry_bit = quotient.bit(B - 1);
            remainder = remainder.overflowing_shl(1).0;
            remainder.set_bit(0, carry_bit);
            quotient = quotient.overflowing_shl(1).0;

            // perform a left shift by 1 on the quotient and remainder, viewed as a single bit string

            let (new_rem, borrow) = remainder.overflowing_sub(rhs);
            if !borrow {
                remainder = new_rem;
                quotient.set_bit(0, true); // set LSB to 1
            } else {
                quotient.set_bit(0, false);
            }
        }
        (quotient, remainder)
    }

    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem(rhs).0)
        }
    }

    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem(rhs).1)
        }
    }
}

impl<const S: bool, const B: usize, const R: bool, const A: usize> CastFrom<BitInt<R, A>> for BitInt<S, B> {
    fn cast_from(value: BitInt<R, A>) -> Self {
        let mut out = if value.is_negative() {
            Self { bits: [true; B] }
        } else {
            Self { bits: [false; B] }
        };
        let min_bits = core::cmp::min(B, A);
        for i in 0..min_bits {
            out.set_bit(i, value.bit(i));
        }
        out
    }
}

impl<const S: bool, const B: usize, const R: bool, const A: usize> TryFrom<&BitInt<R, A>> for BitInt<S, B> {
    type Error = crate::errors::TryFromIntError;

    fn try_from(value: &BitInt<R, A>) -> Result<Self, Self::Error> {
        let value = *value;
        match (R, S) {
            (false, false) => {
                if A - (value.leading_zeros() as usize) > B {
                    Err(crate::errors::TryFromIntError(()))
                } else {
                    Ok(Self::cast_from(value))
                }
            },
            (false, true) => {
                if A - (value.leading_zeros() as usize) > B - 1 {
                    Err(crate::errors::TryFromIntError(()))
                } else {
                    Ok(Self::cast_from(value))
                }
            },
            (true, false) => {
                if value.is_negative() || (A - (value.leading_zeros() as usize) > B) {
                    Err(crate::errors::TryFromIntError(()))
                } else {
                    Ok(Self::cast_from(value))
                }
            },
            (true, true) => {
                if value.is_negative() {
                    if A - (value.leading_ones() as usize) > B - 1 {
                        Err(crate::errors::TryFromIntError(()))
                    } else {
                        Ok(Self::cast_from(value))
                    }
                } else {
                    if A - (value.leading_zeros() as usize) > B - 1 {
                        Err(crate::errors::TryFromIntError(()))
                    } else {
                        Ok(Self::cast_from(value))
                    }
                }
            }
        }
    }
}

macro_rules! bitint_from_to_primitive_int {
    ($($ty:ty), *) => {
        $(
            #[allow(unused_comparisons)]
            impl From<$ty> for BitInt<{<$ty>::MIN < 0}, {core::mem::size_of::<$ty>() * 8}> {
                fn from(value: $ty) -> Self {
                    let mut out = Self { bits: [false; core::mem::size_of::<$ty>() * 8] };
                    for i in 0..(core::mem::size_of::<$ty>() * 8) {
                        out.set_bit(i, (value >> i) & 1 != 0);
                    }
                    out
                }
            }

            #[allow(unused_comparisons)]
            impl From<BitInt<{<$ty>::MIN < 0}, {core::mem::size_of::<$ty>() * 8}>> for $ty {
                fn from(value: BitInt<{<$ty>::MIN < 0}, {core::mem::size_of::<$ty>() * 8}>) -> Self {
                    let mut out: $ty = 0;
                    for i in 0..(core::mem::size_of::<$ty>() * 8) {
                        if value.bit(i) {
                            out |= 1 << i;
                        }
                    }
                    out
                }
            }

            impl<const S: bool, const B: usize> CastFrom<$ty> for BitInt<S, B> {
                fn cast_from(value: $ty) -> Self {
                    Self::cast_from(BitInt::from(value))
                }
            }

            impl<const S: bool, const B: usize> CastFrom<BitInt<S, B>> for $ty {
                fn cast_from(value: BitInt<S, B>) -> Self {
                    Self::from(BitInt::cast_from(value))
                }
            }
        )*
    };
}

bitint_from_to_primitive_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

use core::ops::{BitAnd, BitOr, BitXor, Not};

impl<const S: bool, const B: usize> Not for BitInt<S, B> {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            bits: self.bits.map(|b| !b),
        }
    }
}

impl<const S: bool, const B: usize> BitAnd for BitInt<S, B> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        for i in 0..B {
            out.bits[i] = self.bits[i] & rhs.bits[i];
        }
        out
    }
}

impl<const S: bool, const B: usize> BitOr for BitInt<S, B> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        for i in 0..B {
            out.bits[i] = self.bits[i] | rhs.bits[i];
        }
        out
    }
}

impl<const S: bool, const B: usize> BitXor for BitInt<S, B> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        for i in 0..B {
            out.bits[i] = self.bits[i] ^ rhs.bits[i];
        }
        out
    }
}

impl<const S: bool, const B: usize> Arbitrary for BitInt<S, B> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut bits = [false; B];
        for bit in bits.iter_mut() {
            *bit = bool::arbitrary(g);
        }
        Self { bits }
    }
}

impl<const S: bool, const B: usize> Debug for BitInt<S, B> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for &bit in self.bits.iter().rev() {
            write!(f, "{}", if bit { '1' } else { '0' })?;
        }
        Ok(())
    }
}

use crate::literal_parse::get_size_params_from_bits;

macro_rules! integer_from_bit_int {
    ($($bits: literal), *) => {
        $(
            impl<const S: bool> From<BitInt<S, $bits>> for Integer<S, {get_size_params_from_bits($bits).0}, {get_size_params_from_bits($bits).1}> {
                fn from(b: BitInt<S, $bits>) -> Self {
                    let mut out = if b.is_negative() {
                        Self::ALL_ONES
                    } else {
                        Self::ZERO
                    };
                    for i in 0..$bits {
                        out.set_bit(i as u32, b.bit(i));
                    }
                    out
                }
            }

            impl<const S: bool> From<Integer<S, {get_size_params_from_bits($bits).0}, {get_size_params_from_bits($bits).1}>> for BitInt<S, $bits> {
                fn from(b: Integer<S, {get_size_params_from_bits($bits).0}, {get_size_params_from_bits($bits).1}>) -> Self {
                    let mut out = Self::ZERO;
                    for i in 0..$bits {
                        out.set_bit(i, b.bit(i as u32));
                    }
                    out
                }
            }

            impl<const S: bool> TestConvert for BitInt<S, $bits> {
                type Output = Integer<S, {get_size_params_from_bits($bits).0}, {get_size_params_from_bits($bits).1}>;

                #[inline]
                fn into(self) -> Self::Output {
                    Integer::from(self)
                }
            }
        )*
    }
}

integer_from_bit_int!(
    8, 16, 32, 64, 128, 256, 512, // powers of two
    56, 144, 160, 488, // non-powers of two, multiples of 8
    2, 3, 4, 5, 7, 9, 11, 15, 23, 31, 43, 59, 61, 73, 89, 97, 101, 113, 127, 129, 173, 255, 289, 366,
    402, 422 // non-multiples of 8
);
