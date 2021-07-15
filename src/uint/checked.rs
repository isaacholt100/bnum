use super::BUint;
use crate::digit::Digit;
use crate::macros::{div_zero, checked_pow};
use crate::ExpType;

const fn tuple_to_option<const N: usize>((int, overflow): (BUint<N>, bool)) -> Option<BUint<N>> {
    if overflow {
        None
    } else {
        Some(int)
    }
}

/*/// Returns tuple of division and whether u is less than v
pub const fn div_float<const N: usize>(u: BUint<N>, v: BUint<N>) -> (BUint<N>, bool) {
    let gt = if let core::cmp::Ordering::Less = u.cmp(&v) {
        0
    } else {
        1
    };
    // `self` is padded with N trailing zeros (less significant digits).
    // `v` is padded with N leading zeros (more significant digits).
    let shift = v.digits[N - 1].leading_zeros();
    // `shift` is between 0 and 64 inclusive.
    let v = super::unchecked_shl(v, shift);
    // `v` is still padded with N leading zeros.

    struct Remainder<const M: usize> {
        first: Digit,
        second: Digit,
        rest: [Digit; M],
    }
    impl<const M: usize> Remainder<M> {
        const fn new(uint: BUint<M>, shift: ExpType) -> Self {
            // This shift can be anything from 0 to 64 inclusive.
            // Scenarios:
            // * shift by 0 -> nothing happens, still N trailing zeros.
            // * shift by 64 -> all digits shift by one to the right, there are now (N - 1) trailing zeros and 1 leading zero.
            // * shift by amount between 0 and 64 -> there may be 0 or 1 leading zeros and (N - 1) or N trailing zeros.
            // So indexing between 2N - 1 and N - 1 will get any non-zero digits.
            // Instead of a logical right shift, we will perform a rotate right on the uint - this is the same except the part of the number which may have been removed from the right shift is instead brought to the most significant digit of the number.
            // Then do fancy bit shifts and logic to separate the first and last non zero digits.
            let shift = Digit::BITS - shift;
            let mut rest = uint.rotate_right(shift);
            let last_digit = rest.digits[M - 1];
            let last = (last_digit << shift) >> shift;
            let second = last_digit ^ last;
            rest.digits[M - 1] = last;
            Self {
                first: 0,
                second,
                rest: rest.digits,
            }
        }
        const fn index(&self, index: usize) -> Digit {
            if index == M - 1 {
                self.first
            } else if index == M {
                self.second
            } else if index > M {
                self.rest[index - M - 1]
            } else {
                // There are M - 1 trailing zeros so we can return zero here.
                0
            }
        }
        const fn set_digit(&mut self, index: usize, digit: Digit) {
            if index == M - 1 {
                self.first = digit;
            } else if index == M {
                self.second = digit;
            } else if index > M {
                self.rest[index - M - 1] = digit;
            }
        }
        /*const fn to_uint(self, shift: ExpType) -> BUint<M> {
            let mut out = BUint::ZERO;
            let mut i = 0;
            while i < M {
                out.digits[i] = self.index(i) >> shift;
                i += 1;
            }
            if shift > 0 {
                let mut i = 0;
                while i < M {
                    out.digits[i] |= self.rest[i] << (Digit::BITS - shift);
                    i += 1;
                }
            }
            out
        }*/
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
                let (sum, overflow2) = self.index(i + start).overflowing_sub(sum);
                self.set_digit(i + start, sum);
                carry = overflow1 || overflow2;
                i += 1;
            }
            carry
        }
    }

    // The whole implementation of `Mul` doesn't need to change as it is already padded with N leading zeros.
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
                let (prod, c) = crate::arithmetic::mul_carry_unsigned(carry, 0, uint.digits[i], rhs);
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
    
    let mut u = Remainder::new(u, shift);
    let mut q = BUint::ZERO;
    let v_n_1 = v.digits[N - 1];
    let v_n_2 = v.digits[N - 2];
    let gt_half = v_n_1 > digit::HALF;

    let mut j = N + gt;
    while j > gt {
        j -= 1;
        let u_jn = u.index(j + N);
        let mut q_hat = if u_jn < v_n_1 {
            let (mut q_hat, mut r_hat) = if gt_half {
                BUint::<N>::div_wide(u_jn, u.index(j + N - 1), v_n_1)
            } else {
                BUint::<N>::div_half(u_jn, u.index(j + N - 1), v_n_1)
            };
            loop {
                let a = ((r_hat as DoubleDigit) << digit::BITS) | u.index(j + N - 2) as DoubleDigit;
                let b = q_hat as DoubleDigit * v_n_2 as DoubleDigit;
                if b <= a {
                    break;
                }
                /*let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);
                if hi < r_hat {
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
        let carry = u.sub(j, q_hat_v, N + 1);
        if carry {
            q_hat -= 1;
            let carry = u.add(j, v.digits, N);
            u.set_digit(j + N, u.index(j + N).wrapping_add(carry as Digit));
        }
        // if self is less than other, q_hat is 0
        q.digits[j - gt] = q_hat;
    }

    (q, gt == 0)
    //super::unchecked_shl(self.as_buint::<{N * 2}>(), N as u16 * 64).div_rem(v.as_buint::<{N * 2}>()).0
}*/

use crate::digit::{self, DoubleDigit};

impl<const N: usize> BUint<N> {
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }
    const fn div_wide(high: Digit, low: Digit, rhs: Digit) -> (Digit, Digit) {
        let lhs = digit::to_double_digit(high, low);
        let rhs = rhs as DoubleDigit;

        ((lhs / rhs) as Digit, (lhs % rhs) as Digit)
    }
    const fn div_half(rem: Digit, digit: Digit, rhs: Digit) -> (Digit, Digit) {
        const fn div_rem(a: Digit, b: Digit) -> (Digit, Digit) {
            (a / b, a % b)
        }
        let (hi, rem) = div_rem((rem << digit::HALF_BITS) | (digit >> digit::HALF_BITS), rhs);
        let (lo, rem) = div_rem((rem << digit::HALF_BITS) | (digit & digit::HALF), rhs);

        ((hi << digit::HALF_BITS) | lo, rem)
    }
    const fn div_rem_small(self, rhs: Digit) -> (Self, Self) {
        let (div, rem) = self.div_rem_digit(rhs);
        (div, Self::from_digit(rem))
    }
    pub const fn div_rem_digit(self, rhs: Digit) -> (Self, Digit) {
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
        let shift = v.digits[n - 1].leading_zeros() as ExpType;
        let v = super::unchecked_shl(v, shift);

        //debug_assert!(v.bit(N * digit::BITS - 1));
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
            const fn to_uint(self, shift: ExpType) -> BUint<M> {
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
                    let (sum, overflow2) = self.index(i + start).overflowing_sub(sum);
                    self.set_digit(i + start, sum);
                    carry = overflow1 || overflow2;
                    i += 1;
                }
                carry
            }
        }

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
                    let (prod, c) = crate::arithmetic::mul_carry_unsigned(carry, 0, uint.digits[i], rhs);
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
                    let a = ((r_hat as DoubleDigit) << digit::BITS) | u.index(j + n - 2) as DoubleDigit;
                    let b = q_hat as DoubleDigit * v_n_2 as DoubleDigit;
                    if b <= a {
                        break;
                    }
                    /*let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);
                    if hi < r_hat {
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

        let remainder = u.to_uint(shift);
        (q, remainder)
    }
    pub const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
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
    pub const fn div_rem(self, rhs: Self) -> (Self, Self) {
        if rhs.is_zero() {
            div_zero!()
        } else {
            self.div_rem_unchecked(rhs)
        }
    }
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).0)
        }
    }
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_div(rhs)
    }
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).1)
        }
    }
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_rem(rhs)
    }
    pub const fn checked_neg(self) -> Option<Self> {
        if self.is_zero() {
            Some(self)
        } else {
            None
        }
    }
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(super::unchecked_shl(self, rhs))
        }
    }
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(super::unchecked_shr(self, rhs))
        }
    }
    checked_pow!();
    pub const fn checked_log2(self) -> Option<ExpType> {
        self.bits().checked_sub(1)
    }
    pub const fn checked_log10(self) -> Option<ExpType> {
        // TODO: can optimise this better maybe
        self.checked_log(Self::from_digit(10))
    }
    const fn unchecked_log2_exptype(self) -> ExpType {
        self.bits() - 1
    }
    pub const fn checked_log(self, base: Self) -> Option<ExpType> {
        if self.is_zero() || base.is_one() || base.is_zero() {
            None
        } else {
            let b = self.unchecked_log2_exptype() / (base.unchecked_log2_exptype() + 1);
            let mut n = b;
            let mut r = self.div_rem_unchecked(base.pow(b)).0;
            loop {
                r = r.div_rem_unchecked(base).0;
                n += 1;
                if let core::cmp::Ordering::Less = r.cmp(&base) {
                    break;
                }
            }
            Some(n)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{U128};

    fn converter(prim_result: Option<u128>) -> Option<U128> {
        match prim_result {
            Some(u) => Some(U128::from(u)),
            None => None,
        }
    }

    test_unsigned! {
        name: checked_add,
        method: {
            checked_add(238732748937u128, 23583048508u128);
            checked_add(u128::MAX, 1u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_sub,
        method: {
            checked_sub(334534859834905830u128, 93745873457u128);
            checked_sub(23423423u128, 209834908234898u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_mul,
        method: {
            checked_mul(309458690987839544353455765u128, 344597u128);
            checked_mul(958734920934875289309456874985769879u128, 33219654565456456453434545697u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_div,
        method: {
            checked_div(234233453453454563453453423u128, 34534597u128);
            checked_div(95873492093487528930479456874985769879u128, 33219654565456456453434545697u128);
            checked_div(34564564564u128, 33219654565456456453434545697u128);
            checked_div(475749674596u128, 0u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_div_euclid,
        method: {
            checked_div_euclid(3058689475456456908345374598734535u128, 973457035343453453454338408u128);
            checked_div_euclid(1734857456846783458346458640586098u128, 98474869054698745u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_rem,
        method: {
            checked_rem(9845764759879745698u128, 948745860945769845645986745986u128);
            checked_rem(3450457689456094859604589684905698u128, 34985734895793u128);
            checked_rem(4987569457756984789756745677957698476u128, 49857498576947593595548u128);
            checked_rem(475749674596u128, 0u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_rem_euclid,
        method: {
            checked_rem_euclid(45645609485069840574594565646456u128, 984756897456799u128);
            checked_rem_euclid(9827986748560745645867456456456456u128, 98474869054698456456456456456745u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_neg,
        method: {
            checked_neg(456456454698756u128);
            checked_neg(0u128);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_shl,
        method: {
            checked_shl(45645643454354563634554698756u128, 22 as u16);
            checked_shl(4598745697987927893475u128, 5873 as u16);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_shr,
        method: {
            checked_shr(8098459098745896789454976498u128, 100 as u16);
            checked_shr(9719834759874986456456465u128, 128 as u16);
        },
        converter: converter
    }
    test_unsigned! {
        name: checked_pow,
        method: {
            checked_pow(4565u128, 100 as u16);
            checked_pow(43u128, 15 as u16);
            checked_pow(5u128, 34 as u16);
        },
        converter: converter
    }
}