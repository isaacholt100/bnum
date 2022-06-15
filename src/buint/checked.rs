use super::BUint;
use crate::digit::Digit;
use crate::macros::{div_zero, checked_pow};
use crate::{ExpType, BInt};
use crate::doc;
use crate::int::checked::tuple_to_option;
use crate::digit::{self, DoubleDigit};

#[doc=doc::checked::impl_desc!()]
impl<const N: usize> BUint<N> {
    #[inline]
    pub const fn checked_add_signed(self, rhs: BInt<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_signed(rhs))
    }
    #[inline]
    #[doc=doc::checked_add!(U256)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }


    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }
    
    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }
    
    #[inline]
    const fn div_wide(high: Digit, low: Digit, rhs: Digit) -> (Digit, Digit) {
		// credit uint source code
        let lhs = digit::to_double_digit(high, low);
        let rhs = rhs as DoubleDigit;

        ((lhs / rhs) as Digit, (lhs % rhs) as Digit)
    }
    
    #[inline]
    const fn div_half(rem: Digit, digit: Digit, rhs: Digit) -> (Digit, Digit) {
		// credit uint source code
        const fn div_rem(a: Digit, b: Digit) -> (Digit, Digit) {
            (a / b, a % b)
        }
        let (hi, rem) = div_rem((rem << digit::HALF_BITS) | (digit >> digit::HALF_BITS), rhs);
        let (lo, rem) = div_rem((rem << digit::HALF_BITS) | (digit & digit::HALF), rhs);

        ((hi << digit::HALF_BITS) | lo, rem)
    }

    #[inline]
    const fn div_rem_small(self, rhs: Digit) -> (Self, Self) {
		// credit uint source code
        let (div, rem) = self.div_rem_digit(rhs);
        (div, Self::from_digit(rem))
    }

    #[inline]
    pub const fn div_rem_digit(self, rhs: Digit) -> (Self, Digit) {
		// credit uint source code
        let mut rem: Digit = 0;
        let mut out = Self::ZERO;
        if rhs > digit::HALF {
            let mut i = N;
            while i > 0 {
                i -= 1;
                let (q, r) = Self::div_wide(rem, self.digits[i], rhs);
                out.digits[i] = q;
                rem = r;
            }
        } else {
            let mut i = N;
            while i > 0 {
                i -= 1;
                let (q, r) = Self::div_half(rem, self.digits[i], rhs);
                out.digits[i] = q;
                rem = r;
            }
        }
        (out, rem)
    }
    
    const fn div_rem_core(self, v: Self, n: usize, m: usize) -> (Self, Self) {
		// credit uint source code
        let shift = v.digits[n - 1].leading_zeros() as ExpType;
        let v = super::unchecked_shl(v, shift);

        //debug_assert!(v.bit(N as ExpType * digit::BITS - 1));
        debug_assert!(n + m <= N);

        struct Remainder<const M: usize> {
            first: Digit,
            rest: [Digit; M],
        }
        impl<const M: usize> Remainder<M> {
            const fn new(uint: BUint<M>, shift: ExpType) -> Self {
                let first = uint.digits[0] << shift;
                let rest = uint.wrapping_shr(digit::BITS - shift);
                Self {
                    first,
                    rest: rest.digits,
                }
            }
            const fn index(&self, index: usize) -> Digit {
                if index == 0 {
                    self.first
                } else {
                    self.rest[index - 1]
                }
            }
            const fn set_digit(&mut self, index: usize, digit: Digit) {
                if index == 0 {
                    self.first = digit;
                } else {
                    self.rest[index - 1] = digit;
                }
            }
            const fn into_uint(self, shift: ExpType) -> BUint<M> {
                let mut out = BUint::ZERO;
                let mut i = 0;
                while i < M {
                    out.digits[i] = self.index(i) >> shift;
                    i += 1;
                }
                if shift > 0 {
                    let mut i = 0;
                    while i < M {
                        out.digits[i] |= self.rest[i] << (digit::BITS as ExpType - shift);
                        i += 1;
                    }
                }
                out
            }
            const fn sub(&mut self, start: usize, rhs: Mul<M>, end: usize) -> bool {
                let mut carry = false;
                let mut i = 0;
                while i < end {
                    let (sum, overflow1) = rhs.index(i).overflowing_add(carry as Digit);
                    let (sub, overflow2) = self.index(i + start).overflowing_sub(sum);
                    self.set_digit(i + start, sub);
                    carry = overflow1 || overflow2;
                    i += 1;
                }
                carry
            }
            const fn add(&mut self, start: usize, rhs: [Digit; M], end: usize) -> bool {
                let mut carry = false;
                let mut i = 0;
                while i < end {
                    let (sum, overflow1) = rhs[i].overflowing_add(carry as Digit);
                    let (sum, overflow2) = self.index(i + start).overflowing_add(sum);
                    self.set_digit(i + start, sum);
                    carry = overflow1 || overflow2;
                    i += 1;
                }
                carry
            }
        }

        #[derive(Clone, Copy)]
        struct Mul<const M: usize> {
            last: Digit,
            rest: [Digit; M],
        }
        impl<const M: usize> Mul<M> {
            const fn new(uint: BUint<M>, rhs: Digit) -> Self {
                let mut rest = [0; M];
                let mut carry: Digit = 0;
                let mut i = 0;
                while i < M {
                    let (prod, c) = uint.digits[i].carrying_mul(rhs, carry);
                    carry = c;
                    rest[i] = prod;
                    i += 1;
                }
                Self {
                    last: carry,
                    rest,
                }
            }
            const fn index(&self, index: usize) -> Digit {
                if index == M {
                    self.last
                } else {
                    self.rest[index]
                }
            }
        }
        
        let mut u = Remainder::new(self, shift);
        let mut q = Self::ZERO;
        let v_n_1 = v.digits[n - 1];
        let v_n_2 = v.digits[n - 2];
        let gt_half = v_n_1 > digit::HALF;

        let mut j = m + 1;
        while j > 0 {
            j -= 1;
            let u_jn = u.index(j + n);
            let mut q_hat = if u_jn < v_n_1 {
                let (mut q_hat, mut r_hat) = if gt_half {
                    Self::div_wide(u_jn, u.index(j + n - 1), v_n_1)
                } else {
                    Self::div_half(u_jn, u.index(j + n - 1), v_n_1)
                };
                loop {
                    //let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);
                    let a = ((r_hat as DoubleDigit) << digit::BITS) | u.index(j + n - 2) as DoubleDigit;
                    let b = q_hat as DoubleDigit * v_n_2 as DoubleDigit;
                    if b <= a {
                        break;
                    }
                    /*let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);*/
                    /*if hi < r_hat {
                        break;
                    } else if hi == r_hat && lo <= u.index(j + n - 2) {
                        break;
                    }*/
                    q_hat -= 1;
                    let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                    r_hat = new_r_hat;
                    if overflow {
                        break;
                    }
                }
                q_hat
            } else {
                Digit::MAX
            };
            let q_hat_v = Mul::new(v, q_hat);
            let carry = u.sub(j, q_hat_v, n + 1);
            if carry {
                q_hat -= 1;
                let carry = u.add(j, v.digits, n);
                u.set_digit(j + n, u.index(j + n).wrapping_add(carry as Digit));
            }

            q.digits[j] = q_hat;
        }

        let remainder = u.into_uint(shift);
        (q, remainder)
    }

    #[inline]
    pub const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
		// credit uint source code
        if self.is_zero() {
            return (Self::ZERO, Self::ZERO);
        }

        use core::cmp::Ordering;

        match self.cmp(&rhs) {
            Ordering::Less => (Self::ZERO, self),
            Ordering::Equal => (Self::ONE, Self::ZERO),
            Ordering::Greater => {
                let self_last_digit_index = self.last_digit_index();
                let rhs_last_digit_index = rhs.last_digit_index();
                if rhs_last_digit_index == 0 {
                    let first_digit = rhs.digits[0];
                    if first_digit == 1 {
                        return (self, Self::ZERO);
                    }
                    return self.div_rem_small(first_digit);
                }
                self.div_rem_core(rhs, rhs_last_digit_index + 1, self_last_digit_index - rhs_last_digit_index)
            }
        }
    }

    #[inline]
    pub const fn div_rem(self, rhs: Self) -> (Self, Self) {
        if rhs.is_zero() {
            div_zero!()
        } else {
            self.div_rem_unchecked(rhs)
        }
    }

    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).0)
        }
    }

    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_div(rhs)
    }

    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).1)
        }
    }

    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_rem(rhs)
    }

    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        if self.is_zero() {
            Some(self)
        } else {
            None
        }
    }

    #[inline]
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(super::unchecked_shl(self, rhs))
        }
    }

    #[inline]
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(super::unchecked_shr(self, rhs))
        }
    }
    checked_pow!();

    #[inline]
    pub const fn checked_log2(self) -> Option<ExpType> {
        self.bits().checked_sub(1)
    }

    #[inline]
    pub const fn checked_log10(self) -> Option<ExpType> {
        self.checked_log(Self::TEN)
    }

    #[inline]
    pub const fn checked_log(self, base: Self) -> Option<ExpType> {
        // TODO: this is SLOW, make faster
		// credit Rust source code
        if self.is_zero() || base < Self::TWO {
            None
        } else {
            if base == Self::TWO {
                return self.checked_log2();
            }
            let (mut n, mut r) = if Self::BITS >= 128 {
                let b = (self.bits() - 1) / base.bits();
                let r = self.div_rem_unchecked(base.pow(b)).0;
                (b, r)
            } else {
                (0, self)
            };
            while r >= base {
                r = r / base;
                n += 1;
            }
            
            /*let mut b = base;
            while b <= r {
                n += 1;
                b = match b.checked_mul(base) {
                    Some(i) => i,
                    None => break,
                };
            }*/
            Some(n)
        }
    }
}

#[cfg(test)]
mod tests {
	use crate::test::test_bignum;

    test_bignum! {
		function: <u128>::checked_add(a: u128, b: u128),
        cases: [
            (238732748937u128, 23583048508u128),
            (u128::MAX, 1u128)
        ]
    }
    test_bignum! {
        function: <u128>::checked_add_signed(a: u128, b: i128)
    }
    test_bignum! {
		function: <u128>::checked_sub(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::checked_mul(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::checked_div(a: u128, b: u128),
        cases: [
            (234233453453454563453453423u128, 34534597u128),
            (95873492093487528930479456874985769879u128, 33219654565456456453434545697u128),
            (34564564564u128, 33219654565456456453434545697u128)
        ]
    }
    test_bignum! {
		function: <u128>::checked_div_euclid(a: u128, b: u128),
        cases: [
            (3058689475456456908345374598734535u128, 973457035343453453454338408u128),
            (1734857456846783458346458640586098u128, 98474869054698745u128)
        ]
    }
    test_bignum! {
		function: <u128>::checked_rem(a: u128, b: u128),
        cases: [
            (9845764759879745698u128, 948745860945769845645986745986u128),
            (3450457689456094859604589684905698u128, 34985734895793u128),
            (4987569457756984789756745677957698476u128, 49857498576947593595548u128)
        ]
    }
    test_bignum! {
		function: <u128>::checked_rem_euclid(a: u128, b: u128),
        cases: [
            (45645609485069840574594565646456u128, 984756897456799u128),
            (9827986748560745645867456456456456u128, 98474869054698456456456456456745u128)
        ]
    }
    test_bignum! {
		function: <u128>::checked_neg(a: u128),
        cases: [
            (0u128)
        ]
    }
    test_bignum! {
		function: <u128>::checked_shl(a: u128, b: u16),
        cases: [
            (45645643454354563634554698756u128, 22 as u16),
            (4598745697987927893475u128, 5873 as u16)
        ]
    }
    test_bignum! {
		function: <u128>::checked_shr(a: u128, b: u16),
        cases: [
            (8098459098745896789454976498u128, 100 as u16),
            (9719834759874986456456465u128, 128 as u16)
        ]
    }
    test_bignum! {
		function: <u128>::checked_pow(a: u128, b: u16)
    }
    test_bignum! {
		function: <u128>::checked_log(a: u128, b: u128)
    }
    test_bignum! {
		function: <u128>::checked_log2(a: u128)
    }
    test_bignum! {
		function: <u128>::checked_log10(a: u128),
        cases: [
            (10000000000000000u128),
            (10000u128)
        ]
    }
}