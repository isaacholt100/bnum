use super::BUint;
use crate::arch;
use crate::digit::{Digit, DoubleDigit, DIGIT_BIT_SHIFT, DIGIT_BITS_U32};

const LONG_MUL_THRESHOLD: usize = 32;
const KARATSUBA_THRESHOLD: usize = 256;

impl<const N: usize> BUint<N> {
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut carry = 0u8;
        self.digits.iter().zip(rhs.digits.iter()).for_each(|(a, b)| {
            let result = arch::add_carry(carry, a, b);
            out.digits[i] = result.0;
            carry = result.1;
        });
        (out, carry != 0)
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut carry = 0u8;
        let mut i = 0;
        while i < N {
            let result = arch::add_carry_unsigned(carry, self.digits[i], rhs.digits[i]);
            out.digits[i] = result.0;
            carry = result.1;
            i += 1;
        }
        (out, carry != 0)
    }
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut borrow = 0u8;
        self.digits.iter().zip(rhs.digits.iter()).for_each(|(a, b)| {
            let result = arch::sub_borrow(borrow, a, b);
            out.digits[i] = result.0;
            borrow = result.1;
        })
        (out, borrow != 0)
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut out = Self::ZERO;
        let mut borrow = 0u8;
        let mut i = 0;
        while i < N {
            let result = arch::sub_borrow_unsigned(borrow, self.digits[i], rhs.digits[i]);
            out.digits[i] = result.0;
            borrow = result.1;
            i += 1;
        }
        (out, borrow != 0)
    }
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        // TODO: implement
        (Self::ZERO, false)
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    const fn long_mul(self, rhs: Self) -> (Self, bool) {
        let mut overflow = false;
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            let mut j = 0;
            let mut carry = 0;
            while j < N {
                let index = i + j;
                if index < N {
                    let (prod, c) = arch::mul_carry_unsigned(carry, out.digits[index], self.digits[i], rhs.digits[j]);
                    out.digits[index] = prod;
                    carry = c;
                } else if self.digits[i] != 0 || rhs.digits[j] != 0 || carry != 0 {
                    overflow = true;
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        (out, overflow)
    }
    /*const fn split<const M: usize>(self) -> (BUint<M>, BUint<{N - M}>) {
        use std::mem::MaybeUninit;

        let mut left = MaybeUninit::<[Digit; M]>::uninit();
        let mut right = MaybeUninit::<[Digit; N - M]>::uninit();
        let digits_ptr = self.digits.as_ptr();
        let left_ptr = left.as_mut_ptr() as *mut Digit;
        let right_ptr = right.as_mut_ptr() as *mut Digit;
        unsafe {
            digits_ptr.copy_to_nonoverlapping(left_ptr, M);
            digits_ptr.add(M).copy_to_nonoverlapping(right_ptr, N - M);
            std::mem::forget(self);
            (BUint::from_digits(left.assume_init()), BUint::from_digits(right.assume_init()))
        }
    }*/
    const fn karatsuba(self, rhs: Self) -> (Self, bool)/* where [u8; {N >> 1}]: Sized, [u8; {N - (N >> 1)}]: Sized*/ {
        /*if self.last_digit_index() == 0 && rhs.last_digit_index() == 0 {
            let prod = self.digits[0] as DoubleDigit * rhs.digits[0] as DoubleDigit;
            let mut out = Self::ZERO;
            out.digits[0] = prod as Digit;
            out.digits[1] = (prod >> 64) as Digit;
            (out, false)
        } else {
            let M = N >> 1;
            let (x_H, x_L) = self.split::<{N >> 1}>();
            let (y_H, y_L) = self.split::<{N >> 1}>();
            
            unimplemented!()
        }*/
        unimplemented!()
    }
    const fn toom3(self, rhs: Self) -> (Self, bool) {
        unimplemented!()
    }
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        /*if N <= LONG_MUL_THRESHOLD {
            self.long_mul(rhs)
        } else if N <= KARATSUBA_THRESHOLD {
            self.karatsuba(rhs)
        } else {
            self.toom3(rhs)
        }*/
        self.long_mul(rhs)
    }
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        // TODO: implement
        (Self::ZERO, false)
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_div(rhs), false)
    }
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        // TODO: implement
        (Self::ZERO, false)
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_rem(rhs), false)
    }
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }
    /*#[cfg(feature = "intrinsics")]
    pub fn overflowing_neg(self) -> (Self, bool) {
        if self == Self::ZERO {
            (Self::ZERO, false)
        } else {
            (!self + Self::ONE, true)
        }
    }*/
    //#[cfg(not(feature = "intrinsics"))]
    pub const fn overflowing_neg(self) -> (Self, bool) {
        if self.is_zero() {
            (Self::ZERO, false)
        } else {
            (self.not().add(Self::ONE), true)
        }
    }
    pub const fn unchecked_shl(self, rhs: u32) -> Self {
        if rhs == 0 {
            self
        } else {
            let digit_shift = (rhs >> DIGIT_BIT_SHIFT) as usize;
            let shift = (rhs % DIGIT_BITS_U32) as u8;
            
            let mut out = Self::ZERO;
            let mut carry = 0;
            let carry_shift = DIGIT_BITS_U32 as u8 - shift;
            let mut last_index = digit_shift;
            let mut i = digit_shift;

            while i < N {
                let digit = self.digits[i - digit_shift];
                let new_carry = digit >> carry_shift;
                let new_digit = (digit << shift) | carry;
                if new_digit != 0 {
                    last_index = i;
                    out.digits[i] = new_digit;
                }
                carry = new_carry;
                i += 1;
            }

            if carry != 0 {
                last_index += 1;
                if last_index < N {
                    out.digits[last_index] = carry;
                }
            }

            out
        }
    }
    pub const fn unchecked_shr(self, rhs: u32) -> Self {
        if rhs == 0 {
            self
        } else {
            let digit_shift = (rhs >> DIGIT_BIT_SHIFT) as usize;
            let shift = (rhs % DIGIT_BITS_U32) as u8;
            
            let mut out = Self::ZERO;
            let mut borrow = 0;
            let borrow_shift = DIGIT_BITS_U32 as u8 - shift;
            let mut i = digit_shift;

            while i < N {
                let digit = self.digits[Self::N_MINUS_1 + digit_shift - i];
                let new_borrow = digit << borrow_shift;
                let new_digit = (digit >> shift) | borrow;
                out.digits[Self::N_MINUS_1 - i] = new_digit;
                borrow = new_borrow;
                i += 1;
            }

            out
        }
    }
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        if rhs as usize >= Self::BITS {
            (self.unchecked_shl(rhs & (Self::BITS - 1) as u32), true)
        } else {
            (self.unchecked_shl(rhs), false)
        }
    }
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        if rhs as usize >= Self::BITS {
            (self.unchecked_shr(rhs & (Self::BITS - 1) as u32), true)
        } else {
            (self.unchecked_shr(rhs), false)
        }
    }
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    fn converter(tuple: (u128, bool)) -> (U128, bool) {
        (tuple.0.into(), tuple.1)
    }

    test_unsigned! {
        test_name: test_overflowing_add_with_overflow,
        method: overflowing_add(u128::MAX - 35348957, 34059304859034578959083490834850937458u128),
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_add,
        method: overflowing_add(34987358947598374835u128, 340593453454564568u128),
        converter: converter
    }

    test_unsigned! {
        test_name: test_overflowing_sub_with_overflow,
        method: overflowing_sub(34053457987u128, 34059304859034578959083490834850937458u128),
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_sub,
        method: overflowing_sub(34987358947598374835345345345454645645u128, 9856946974958764564564508456849058u128),
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_mul,
        method: overflowing_mul(93875893745946675675675675745687345u128, 394857456456456456434534355645384975u128),
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_shl,
        method: overflowing_shl(u128::MAX - 3453475, 5u32),
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_shr,
        method: overflowing_shr(349573947593475973453348759u128, 10u32),
        converter: converter
    }
}