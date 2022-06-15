use super::BUint;
use crate::macros::overflowing_pow;
use crate::ExpType;
use crate::digit::Digit;
use crate::{BInt, doc};

#[doc=doc::overflowing::impl_desc!()]
impl<const N: usize> BUint<N> {
    #[inline]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut carry = false;
        let mut i = 0;
        while i < N {
            let result = self.digits[i].carrying_add(rhs.digits[i], carry);
            out.digits[i] = result.0;
            carry = result.1;
            i += 1;
        }
        (out, carry)
    }

    #[inline]
    pub const fn overflowing_add_signed(self, rhs: BInt<N>) -> (Self, bool) {
		// credit Rust source code
        let (out, overflow) = self.overflowing_add(rhs.to_bits());
        (out, overflow ^ rhs.is_negative())
    }

    #[inline]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut borrow = false;
        let mut i = 0;
        while i < N {
            let result = self.digits[i].borrowing_sub(rhs.digits[i], borrow);
            out.digits[i] = result.0;
            borrow = result.1;
            i += 1;
        }
        (out, borrow)
    }

    #[inline]
    const fn long_mul(self, rhs: Self) -> (Self, bool) {
        let mut overflow = false;
        let mut out = Self::ZERO;
        let mut carry: Digit;
        let mut i = 0;
        while i < N {
            carry = 0;
            let mut j = 0;
            while j < N {
                let index = i + j;
                if index < N {
                    let (prod, c) = super::carrying_mul(self.digits[i], rhs.digits[j], carry, out.digits[index]);
                    out.digits[index] = prod;
                    carry = c;
                } else if (self.digits[i] != 0 && rhs.digits[j] != 0) || carry != 0 {
                    overflow = true;
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        (out, overflow)
    }

    #[inline]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
		// TODO: implement a faster multiplication algorithm for large values of `N`
        self.long_mul(rhs)
    }
    /*const fn overflowing_mul_digit(self, rhs: Digit) -> (Self, Digit) {
        let mut out = Self::ZERO;
        let mut carry: Digit = 0;
        let mut i = 0;
        while i < N {
            let (prod, c) = arch::mul_carry_unsigned(carry, 0, self.digits[i], rhs);
            out.digits[i] = prod;
            carry = c;
            i += 1;
        }
        (out, carry)
    }*/
    #[inline]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_div(rhs), false)
    }

    #[inline]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    #[inline]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_rem(rhs), false)
    }

    #[inline]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }

    #[inline]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        let (a, b) = (!self).overflowing_add(Self::ONE);
        (a, !b)
    }

    #[inline]
    pub const fn overflowing_shl(self, rhs: ExpType) -> (Self, bool) {
        if rhs >= Self::BITS {
            (super::unchecked_shl(self, rhs & Self::BITS_MINUS_1), true)
        } else {
            (super::unchecked_shl(self, rhs), false)
        }
    }

    #[inline]
    pub const fn overflowing_shr(self, rhs: ExpType) -> (Self, bool) {
        if rhs >= Self::BITS {
            (super::unchecked_shr(self, rhs & Self::BITS_MINUS_1), true)
        } else {
            (super::unchecked_shr(self, rhs), false)
        }
    }
    
    overflowing_pow!();
}

#[cfg(test)]
mod tests {
	use crate::test::test_bignum;

    test_bignum! {
		function: <u128>::overflowing_add(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::overflowing_sub(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::overflowing_mul(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::overflowing_div(a: u128, b: u128),
        skip: b == 0,
        cases: [
            (103573984758937498573594857389345u128, 3453454545345345345987u128),
            (193679457916593485358497389457u128, 684u128)
        ]
    }
    test_bignum! {
		function: <u128>::overflowing_div_euclid(a: u128, b: u128),
        skip: b == 0,
        cases: [
            (349573947593745898375u128, 349573947593745898375u128),
            (0u128, 3459745734895734957984579u128)
        ]
    }
    test_bignum! {
		function: <u128>::overflowing_rem(a: u128, b: u128),
        skip: b == 0,
        cases: [
            (2973459793475897343495439857u128, 56u128),
            (1u128 << 64, 2u128)
        ]
    }
    test_bignum! {
		function: <u128>::overflowing_rem_euclid(a: u128, b: u128),
        skip: b == 0,
        cases: [
            (27943758345638459034898756847983745u128, 37589734758937458973459u128),
            (0u128, 93745934953894u128)
        ]
    }
    test_bignum! {
		function: <u128>::overflowing_neg(a: u128)
    }
    test_bignum! {
		function: <u128>::overflowing_shl(a: u128, b: u16)
    }
    test_bignum! {
		function: <u128>::overflowing_shr(a: u128, b: u16)
    }
    test_bignum! {
		function: <u128>::overflowing_pow(a: u128, b: u16)
    }
}