use super::Float;
use core::ops::{Add, Sub, Mul, Neg};
use crate::arithmetic;
use super::{Exponent, Sign, digit::{self, Digit, DoubleDigit, SignedDoubleDigit}};
use core::mem::MaybeUninit;

const fn add_base_10(carry: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let sum = a as DoubleDigit + b as DoubleDigit + carry as DoubleDigit;
    ((sum % digit::BASE as DoubleDigit) as Digit, (sum / digit::BASE as DoubleDigit) as u8)
}

const fn sub_base_10(borrow: u8, a: Digit, b: Digit) -> (Digit, u8) {
    let diff = a as SignedDoubleDigit - b as SignedDoubleDigit - borrow as SignedDoubleDigit;
    if diff < 0 {
        ((diff + digit::BASE as SignedDoubleDigit) as Digit, 1)
    } else {
        (diff as Digit, 0)
    }
}

const fn mul_base_10(carry: Digit, current: Digit, a: Digit, b: Digit) -> (Digit, Digit) {
    let prod = carry as DoubleDigit + current as DoubleDigit + (a as DoubleDigit) * (b as DoubleDigit);
    ((prod % digit::BASE as DoubleDigit) as Digit, (prod / digit::BASE as DoubleDigit) as Digit)
}

impl<const N: usize> Float<N> {
    const fn add_internal(self, rhs: Self, sign: Sign) -> Self {
        let mut digits = [0; N];
        let mut carry = 0;
        let (x, y) = Self::order_by_exponent(self, rhs);
        let exp_diff = (y.exponent - x.exponent) as usize;
        let mut i = N;
        while i > exp_diff {
            i -= 1;
            let (sum, c) = add_base_10(carry, x.digits[i - exp_diff], y.digits[i]);
            carry = c;
            digits[i] = sum;
        }
        while i > 0 {
            i -= 1;
            // Can optimise later as second argument will always be zero
            let (sum, c) = add_base_10(carry, 0, y.digits[i]);
            carry = c;
            digits[i] = sum;
        }
        if carry == 0 {
            Self {
                sign,
                exponent: y.exponent,
                digits,
            }
        } else {
            let mut uninit = MaybeUninit::<[Digit; N]>::uninit();
            let uninit_ptr = uninit.as_mut_ptr() as *mut Digit;
            let digits_ptr = digits.as_ptr();
        
            let carry_arr = [carry as Digit];
            let carry_ptr = carry_arr.as_ptr();
        
            unsafe {
                digits_ptr.copy_to_nonoverlapping(uninit_ptr.offset(1), N - 1);
                carry_ptr.copy_to_nonoverlapping(uninit_ptr, 1);
                Self {
                    digits: uninit.assume_init(),
                    exponent: y.exponent + 1,
                    sign,
                }
            }
        }
    }
    /// (smaller, greater)
    const fn order_by_magnitude(self, other: Self) -> (Self, Self, Sign) {
        if let core::cmp::Ordering::Less = self.compare_same_signs(&other) {
            (self, other, Sign::Negative)
        } else {
            (other, self, Sign::Positive)
        }
    }
    const fn sub_internal(self, rhs: Self) -> Self {
        let (x, y, sign) = Self::order_by_magnitude(self, rhs);
        let mut digits = [0; N];
        let exp_diff = (y.exponent - x.exponent) as usize;
        let mut borrow = 0;
        let mut exponent = y.exponent;
        let mut i = N;
        while i > exp_diff {
            i -= 1;
            let (sub, b) = arithmetic::sub_borrow_unsigned(borrow, x.digits[i - exp_diff], y.digits[i]);
            borrow = b;
            digits[i] = sub;
            if sub == 0 || borrow != 0 {
                exponent -= 1;
            } else {
                exponent = y.exponent;
            }
        }
        while i > 0 {
            i -= 1;
            // Can optimise later as third argument will always be zero
            let (sub, b) = arithmetic::sub_borrow_unsigned(borrow, y.digits[i], 0);
            borrow = b;
            digits[i] = sub;
            if sub == 0 || borrow != 0 {
                exponent -= 1;
            } else {
                exponent = y.exponent;
            }
        }
        Self::finalise(Self {
            sign,
            digits,
            exponent
        })
    }
}

impl<const N: usize> Add for Float<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (&self.sign, &rhs.sign) {
            (Sign::NaN, _) => Self::NAN,
            (_, Sign::NaN) => Self::NAN,
            (Sign::PositiveInfinity, Sign::NegativeInfinity) => Self::NAN,
            (Sign::NegativeInfinity, Sign::PositiveInfinity) => Self::NAN,
            (Sign::PositiveInfinity, _) => Self::INFINITY,
            (_, Sign::PositiveInfinity) => Self::INFINITY,
            (Sign::NegativeInfinity, _) => Self::NEG_INFINITY,
            (_, Sign::NegativeInfinity) => Self::NEG_INFINITY,
            (Sign::Positive, Sign::Positive) => {
                self.add_internal(rhs, Sign::Positive)
            },
            (Sign::Negative, Sign::Negative) => {
                self.add_internal(rhs, Sign::Negative)
            },
            (Sign::Positive, Sign::Negative) => {
                self.sub_internal(rhs)
            },
            (Sign::Negative, Sign::Positive) => {
                self.sub_internal(rhs)
            }
        }
    }
}

impl<const N: usize> Sub for Float<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match (&self.sign, &rhs.sign) {
            (Sign::NaN, _) => Self::NAN,
            (_, Sign::NaN) => Self::NAN,
            (Sign::PositiveInfinity, Sign::PositiveInfinity) => Self::NAN,
            (Sign::NegativeInfinity, Sign::NegativeInfinity) => Self::NAN,
            (Sign::PositiveInfinity, _) => Self::INFINITY,
            (_, Sign::PositiveInfinity) => Self::NEG_INFINITY,
            (Sign::NegativeInfinity, _) => Self::NEG_INFINITY,
            (_, Sign::NegativeInfinity) => Self::INFINITY,
            (Sign::Positive, Sign::Positive) => {
                self.sub_internal(rhs)
            },
            (Sign::Negative, Sign::Negative) => {
                self.sub_internal(rhs)
            },
            (Sign::Positive, Sign::Negative) => {
                self.add_internal(rhs, Sign::Positive)
            },
            (Sign::Negative, Sign::Positive) => {
                self.add_internal(rhs, Sign::Negative)
            }
        }
    }
}

impl<const N: usize> Mul for Float<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        debug_assert!(self.leading_zeros() == 0 && rhs.leading_zeros() == 0);
        let sign = match (&self.sign, &rhs.sign) {
            (Sign::NaN, _) => return Self::NAN,
            (_, Sign::NaN) => return Self::NAN,
            (Sign::PositiveInfinity, Sign::PositiveInfinity) => return Self::INFINITY,
            (Sign::NegativeInfinity, Sign::NegativeInfinity) => return Self::INFINITY,
            (Sign::PositiveInfinity, Sign::NegativeInfinity) => return Self::NEG_INFINITY,
            (Sign::NegativeInfinity, Sign::PositiveInfinity) => return Self::NEG_INFINITY,
            (Sign::PositiveInfinity, Sign::Positive) => return Self::INFINITY,
            (Sign::NegativeInfinity, Sign::Negative) => return Self::INFINITY,
            (Sign::PositiveInfinity, Sign::Negative) => return Self::NEG_INFINITY,
            (Sign::NegativeInfinity, Sign::Positive) => return Self::NEG_INFINITY,
            (Sign::Positive, Sign::PositiveInfinity) => return Self::INFINITY,
            (Sign::Negative, Sign::NegativeInfinity) => return Self::INFINITY,
            (Sign::Negative, Sign::PositiveInfinity) => return Self::NEG_INFINITY,
            (Sign::Positive, Sign::NegativeInfinity) => return Self::NEG_INFINITY,
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative,
        };
        let mut temp_digits = [0; N];
        let mut out_digits = [0; N];
        let mut i = N;
        while i > 0 {
            i -= 1;
            let mut carry = 0;
            let mut j = N;
            while j > 0 {
                let index = i + j;
                j -= 1;
                if index < N {
                    let (prod, c) = mul_base_10(carry, out_digits[index], self.digits[i], rhs.digits[j]);
                    carry = c;
                    out_digits[index] = prod;
                } else {
                    let (prod, c) = mul_base_10(carry, temp_digits[index - N], self.digits[i], rhs.digits[j]);
                    carry = c;
                    temp_digits[index - N] = prod;
                }
            }
            if i == 0 && j == 0 && carry != 0 {
                out_digits[0] = carry;
                return Self {
                    exponent: self.exponent + rhs.exponent + 1,
                    sign,
                    digits: out_digits,
                };
            }
        }
        let mut uninit = MaybeUninit::<[Digit; N]>::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut Digit;
        let digits_ptr = out_digits.as_ptr();
    
        let end_arr = [temp_digits[0]];
        let end_ptr = end_arr.as_ptr();
    
        unsafe {
            digits_ptr.copy_to_nonoverlapping(uninit_ptr, N - 1);
            end_ptr.copy_to_nonoverlapping(uninit_ptr.offset(N as isize - 1), 1);
            Self {
                digits: uninit.assume_init(),
                exponent: self.exponent + rhs.exponent,
                sign,
            }
        }
    }
}

impl<const N: usize> Neg for Float<N> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}