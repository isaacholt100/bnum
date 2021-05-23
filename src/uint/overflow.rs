use super::BUint;
use crate::arch;
use crate::digit;

//const LONG_MUL_THRESHOLD: usize = 32;
//const KARATSUBA_THRESHOLD: usize = 256;

impl<const N: usize> BUint<N> {
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
    const fn long_mul(self, rhs: Self) -> (Self, bool) {
        let mut overflow = false;
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            let mut carry = 0;
            let mut j = 0;
            while j < N {
                let index = i + j;
                if index < N {
                    let (prod, c) = arch::mul_carry_unsigned(carry, out.digits[index], self.digits[i], rhs.digits[j]);
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
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
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
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_div(rhs), false)
    }
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_rem(rhs), false)
    }
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }
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
            const BITS_MINUS_1: u32 = digit::BITS_U32 - 1;
            let digit_shift = (rhs >> digit::BIT_SHIFT) as usize;
            let shift = (rhs & BITS_MINUS_1) as u8;
            
            let mut out = Self::ZERO;
            let mut i = digit_shift;

            if shift == 0 {
                while i < N {
                    let digit = self.digits[i - digit_shift];
                    out.digits[i] = digit;
                    i += 1;
                }
            } else {
                let mut carry = 0;
                let carry_shift = digit::BITS_U32 as u8 - shift;
                let mut last_index = digit_shift;
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
            }

            out
        }
    }
    pub const fn unchecked_shr(self, rhs: u32) -> Self {
        if rhs == 0 {
            self
        } else {
            const BITS_MINUS_1: u32 = digit::BITS_U32 - 1;
            let digit_shift = (rhs >> digit::BIT_SHIFT) as usize;
            let shift = (rhs & BITS_MINUS_1) as u8;
            
            let mut out = Self::ZERO;
            let mut i = digit_shift;

            if shift == 0 {
                while i < N {
                    let digit = self.digits[Self::N_MINUS_1 + digit_shift - i];
                    out.digits[Self::N_MINUS_1 - i] = digit;
                    i += 1;
                }
            } else {
                let mut borrow = 0;
                let borrow_shift = digit::BITS_U32 as u8 - shift;
                while i < N {
                    let digit = self.digits[Self::N_MINUS_1 + digit_shift - i];
                    let new_borrow = digit << borrow_shift;
                    let new_digit = (digit >> shift) | borrow;
                    out.digits[Self::N_MINUS_1 - i] = new_digit;
                    borrow = new_borrow;
                    i += 1;
                }
            }

            out
        }
    }
    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        if rhs as usize >= Self::BITS {
            (self.unchecked_shl(rhs & Self::BITS_MINUS_1), true)
        } else {
            (self.unchecked_shl(rhs), false)
        }
    }
    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        if rhs as usize >= Self::BITS {
            (self.unchecked_shr(rhs & Self::BITS_MINUS_1), true)
        } else {
            (self.unchecked_shr(rhs), false)
        }
    }
    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        if exp == 0 {
            return (Self::ONE, false);
        }
        if self.is_zero() {
            return (Self::ZERO, false);
        }
        let mut y = Self::ONE;
        let mut n = exp;
        let mut x = self;
        let mut overflow = false;

        macro_rules! overflowing_mul {
            ($var: ident) => {
                let (prod, o) = x.overflowing_mul($var);
                $var = prod;
                if o {
                    overflow = o;
                }
            }
        }

        while n > 1 {
            if n & 1 == 0 {
                overflowing_mul!(x);
                n >>= 1;
            } else {
                overflowing_mul!(y);
                overflowing_mul!(x);
                n -= 1;
                n >>= 1;
            }
        }
        let (prod, o) = x.overflowing_mul(y);
        if o {
            overflow = o;
        }
        (prod, overflow)
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    fn converter(tuple: (u128, bool)) -> (U128, bool) {
        (tuple.0.into(), tuple.1)
    }

    test_unsigned! {
        test_name: test_overflowing_add,
        method: {
            overflowing_add(u128::MAX - 35348957, 34059304859034578959083490834850937458u128);
            overflowing_add(34987358947598374835u128, 340593453454564568u128);
        },
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_sub,
        method: {
            overflowing_sub(34053457987u128, 34059304859034578959083490834850937458u128);
            overflowing_sub(34987358947598374835345345345454645645u128, 9856946974958764564564508456849058u128);
        },
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_mul,
        method: {
            overflowing_mul(93875893745946675675675675745687345u128, 394857456456456456434534355645384975u128);
            overflowing_mul(103453534455674958789u128, 509u128);
        },
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_shl,
        method: {
            overflowing_shl(u128::MAX - 3453475, 5u32);
            overflowing_shl(934987774987u128, 55645u32);
        },
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_shr,
        method: {
            overflowing_shr(349573947593475973453348759u128, 10u32);
            overflowing_shr(972456948567894576895749857u128, 5897659u32);
        },
        converter: converter
    }
    test_unsigned! {
        test_name: test_overflowing_pow,
        method: {
            overflowing_pow(3444334u128, 3345334345u32);
            overflowing_pow(23u128, 31u32);
        },
        converter: converter
    }
}