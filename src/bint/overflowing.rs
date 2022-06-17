use super::BInt;
use crate::digit::{SignedDigit, Digit, SignedDoubleDigit};
use crate::macros::{div_zero, rem_zero};
use crate::{ExpType, BUint, doc};
use crate::{digit, error};

#[inline]
const fn carrying_add_signed(a: SignedDigit, b: SignedDigit, carry: bool) -> (SignedDigit, bool) {
    let sum = a as SignedDoubleDigit + b as SignedDoubleDigit + carry as SignedDoubleDigit;
    (sum as SignedDigit, sum > SignedDigit::MAX as SignedDoubleDigit || sum < SignedDigit::MIN as SignedDoubleDigit)
}

#[inline]
const fn borrowing_sub_signed(a: SignedDigit, b: SignedDigit, borrow: bool) -> (SignedDigit, bool) {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    (diff as SignedDigit, diff > SignedDigit::MAX as SignedDoubleDigit || diff < SignedDigit::MIN as SignedDoubleDigit)
}

#[doc=doc::overflowing::impl_desc!()]
impl<const N: usize> BInt<N> {
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut digits = [0; N];
        let mut carry = false;

        let self_digits = self.bits.digits;
        let rhs_digits = rhs.bits.digits;

        let mut i = 0;
        while i < Self::N_MINUS_1 {
            let (sum, c) = self_digits[i].carrying_add(rhs_digits[i], carry);
            digits[i] = sum;
            carry = c;
            i += 1;
        }
        let (sum, carry) = carrying_add_signed(
            self_digits[Self::N_MINUS_1] as SignedDigit,
            rhs_digits[Self::N_MINUS_1] as SignedDigit,
            carry
        );
        digits[Self::N_MINUS_1] = sum as Digit;

        (Self::from_digits(digits), carry)
    }

    #[inline]
    pub const fn overflowing_add_unsigned(self, rhs: BUint<N>) -> (Self, bool) {
		// credit Rust source code
        let rhs = Self::from_bits(rhs);
        let (out, overflow) = self.overflowing_add(rhs);
        (out, overflow ^ rhs.is_negative())
    }

    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut digits = [0; N];
        let mut borrow = false;

        let self_digits = self.bits.digits;
        let rhs_digits = rhs.bits.digits;

        let mut i = 0;
        while i < Self::N_MINUS_1 {
            let (sub, b) = self_digits[i].borrowing_sub(rhs_digits[i], borrow);
            digits[i] = sub;
            borrow = b;
            i += 1;
        }
        let (sub, borrow) = borrowing_sub_signed(
            self_digits[Self::N_MINUS_1] as SignedDigit,
            rhs_digits[Self::N_MINUS_1] as SignedDigit,
            borrow
        );
        digits[Self::N_MINUS_1] = sub as Digit;

        (Self::from_digits(digits), borrow)
    }

    #[inline]
    pub const fn overflowing_sub_unsigned(self, rhs: BUint<N>) -> (Self, bool) {
		// credit Rust source code
        let rhs = Self::from_bits(rhs);
        let (out, overflow) = self.overflowing_sub(rhs);
        (out, overflow ^ rhs.is_negative())
    }
    
    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (uint, overflow) = self.unsigned_abs().overflowing_mul(rhs.unsigned_abs());
        let out = Self::from_bits(uint);
        if self.is_negative() == rhs.is_negative() {
            (out, overflow || out.is_negative())
        } else {
            match out.checked_neg() {
                Some(n) => (n, overflow || out.is_negative()),
                None => (out, overflow),
            }
        }
    }

	#[inline]
	pub(super) const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
		let (div, rem) = self.unsigned_abs().div_rem_unchecked(rhs.unsigned_abs());
		let (div, rem) = (Self::from_bits(div), Self::from_bits(rem));

		match (self.is_negative(), rhs.is_negative()) {
			(false, false) => (div, rem),
			(false, true) => (-div, rem),
			(true, false) => (-div, -rem),
			(true, true) => (div, -rem),
		}
	}

    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
		if rhs.is_zero() {
			div_zero!()
		}
        if self == Self::MIN && rhs == Self::NEG_ONE {
            (self, true)
        } else {
            (self.div_rem_unchecked(rhs).0, false)
        }
    }

    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
		if rhs.is_zero() {
			div_zero!()
		}
        if self == Self::MIN && rhs == Self::NEG_ONE {
            (self, true)
        } else {
			let (div, rem) = self.div_rem_unchecked(rhs);
			if self.is_negative() {
				let r_neg = rhs.is_negative();
				if !rem.is_zero() {
					if r_neg {
						return (div + Self::ONE, false)
					} else {
						return (div - Self::ONE, false)
					};
				}
			}
			(div, false)
        }
    }

    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
		if rhs.is_zero() {
			div_zero!()
		}
        if self == Self::MIN && rhs == Self::NEG_ONE {
            (Self::ZERO, true)
        } else {
            (self.div_rem_unchecked(rhs).1, false)
        }
    }

    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
		if rhs.is_zero() {
			div_zero!()
		}
        if self == Self::MIN && rhs == Self::NEG_ONE {
            (Self::ZERO, true)
        } else {
			let mut rem = self.div_rem_unchecked(rhs).1;
			if rem.is_negative() {
				if rhs.is_negative() {
					rem = rem.wrapping_sub(rhs);
				} else {
					rem = rem.wrapping_add(rhs);
				}
			}
			(rem, false)
        }
    }

    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        (!self).overflowing_add(Self::ONE)
    }

    #[inline]
    pub const fn overflowing_shl(self, rhs: ExpType) -> (Self, bool) {
        let (uint, overflow) = self.bits.overflowing_shl(rhs);
        (Self::from_bits(uint), overflow)
    }

    #[inline]
    const fn shr_internal(self, rhs: ExpType) -> Self {
		// credit num_bigint source code
        if rhs == 0 {
            self
        } else {
            let digit_shift = (rhs >> digit::BIT_SHIFT) as usize;
            let shift = (rhs & digit::BITS_MINUS_1) as u8;
    
            let mut out_digits = [Digit::MAX; N];
            let digits_ptr = self.bits.digits.as_ptr();
            let out_ptr = out_digits.as_mut_ptr() as *mut Digit;
            unsafe {
                digits_ptr.add(digit_shift).copy_to_nonoverlapping(out_ptr, N - digit_shift);
            }
    
            if shift > 0 {
                let mut borrow = 0;
                let borrow_shift = Digit::BITS as u8 - shift;

                let mut i = digit_shift;
                while i < N {
                    let digit = out_digits[Self::N_MINUS_1 - i];
                    let new_borrow = digit << borrow_shift;
                    let new_digit = (digit >> shift) | borrow;
                    out_digits[Self::N_MINUS_1 - i] = new_digit;
                    borrow = new_borrow;
                    i += 1;
                }
                out_digits[Self::N_MINUS_1 - digit_shift] |= (Digit::MAX >> (digit::BITS as u8 - shift)) << (digit::BITS as u8 - shift);
            }
    
            Self::from_digits(out_digits)
        }
    }
    const BITS_MINUS_1: ExpType = (Self::BITS - 1) as ExpType;

    #[inline]
    pub const fn overflowing_shr(self, rhs: ExpType) -> (Self, bool) {
        if self.is_negative() {
            if rhs >= Self::BITS {
                (self.shr_internal(rhs & Self::BITS_MINUS_1), true)
            } else {
                (self.shr_internal(rhs), false)
            }
        } else {
            let (uint, overflow) = self.bits.overflowing_shr(rhs);
            (Self::from_bits(uint), overflow)
        }
    }

    #[inline]
    pub const fn overflowing_abs(self) -> (Self, bool) {
        if self.is_negative() {
            self.overflowing_neg()
        } else {
            (self, false)
        }
    }

	#[inline]
	pub const fn overflowing_pow(self, pow: ExpType) -> (Self, bool) {
		let (u, mut overflow) = self.unsigned_abs().overflowing_pow(pow);
		let out_neg = self.is_negative() && pow & 1 == 1;
		let mut out = Self::from_bits(u);
		if out_neg {
			out = out.wrapping_neg();
			overflow = overflow || !out.is_negative();
		} else {
			overflow = overflow || out.is_negative();
		}
		(out, overflow)
	}
}

use core::ops::{Div, Rem};

impl<const N: usize> const Div for BInt<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            panic!(error::err_msg!("attempt to divide with overflow"))
        } else {
            if rhs.is_zero() {
                div_zero!()
            }
            self.div_rem_unchecked(rhs).0
        }
    }
}

impl<const N: usize> const Rem for BInt<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        if self.eq(&Self::MIN) && rhs.eq(&Self::NEG_ONE) {
            panic!(error::err_msg!("attempt to calculate remainder with overflow"))
        } else {
            if rhs.is_zero() {
                rem_zero!()
            }
            self.div_rem_unchecked(rhs).1
        }
    }
}

#[cfg(test)]
mod tests {
	use crate::test::test_bignum;

    test_bignum! {
        function: <i128>::overflowing_add(a: i128, b: i128),
        cases: [
            (-i128::MAX, i128::MIN),
            (i128::MAX, i128::MAX)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_sub(a: i128, b: i128),
        cases: [
            (i128::MIN, 13i128),
            (i128::MAX, -1i128),
            (i128::MAX, i128::MIN)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_mul(a: i128, b: i128),
        cases: [
            (1i128 << 64, 1i128 << 63),
            (-(1i128 << 100), 1i128 << 27)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_div(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (-1i128, 2i128),
            (i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_div_euclid(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (-1i128, 2i128),
            (i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_rem(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (-577i128 * 80456498576, 577i128),
            (i128::MIN, -1i128)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_rem_euclid(a: i128, b: i128),
        skip: b == 0,
        cases: [
            (0i128, -79872976456456i128),
            (i128::MIN, -1i128),
            (-1i128, 2i128)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_neg(a: i128),
        cases: [
            (0i128),
            (i128::MIN),
            (997340597745960395879i128)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_shl(a: i128, b: u16),
        cases: [
            (i128::MAX - 3453475, 8 as u16),
            (77948798i128, 58743 as u16),
            (-9797456456456i128, 27 as u16)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_shr(a: i128, b: u16),
        cases: [
            (i128::MIN, 11 as u16),
            (-1i128, 85 as u16)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_abs(a: i128),
        cases: [
            (0i128),
            (i128::MIN)
        ]
    }
    test_bignum! {
        function: <i128>::overflowing_pow(a: i128, b: u16),
        cases: [
            (97465984i128, 2555 as u16),
            (-19i128, 11 as u16),
            (-277i128, 14 as u16),
			(-3i128, 81u16)
        ]
    }
}