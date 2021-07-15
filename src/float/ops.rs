use super::Float;
use core::ops::{Add, Sub, Mul, Div, Neg};
use crate::arithmetic;
use super::{Exponent, Sign};
use crate::digit::{self, Digit, DoubleDigit, SignedDoubleDigit};
use core::mem::MaybeUninit;
use crate::BUint;

macro_rules! handle_exp_overflow {
    ($option: expr, $ret: expr) => {
        match $option {
            Some(a) => a,
            None => return $ret,
        }
    };
}
macro_rules! mul_div_handler {
    ($self: ident, $sign: expr) => {
        if $self.exponent.is_negative() {
            Self::zero($sign)
        } else {
            Self::inf($sign)
        }
    }
}
/*const fn add_base_10(carry: u8, a: Digit, b: Digit) -> (Digit, u8) {
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
}*/

impl<const N: usize> Float<N> {
    const fn add_internal(self, rhs: Self, sign: Sign) -> Self {
        let mut digits = [0; N];
        let mut carry = 0;
        let (x, y) = Self::order_by_exponent(self, rhs);
        let exp_diff = (y.exponent - x.exponent) as usize;
        let mut i = 0;
        while i < N - exp_diff {
            let (sum, c) = arithmetic::add_carry_unsigned(carry, x.digits[i + exp_diff], y.digits[i]);
            carry = c;
            digits[i] = sum;
            i += 1;
        }
        while i < N {
            // Can optimise later as second argument will always be zero
            let (sum, c) = arithmetic::add_carry_unsigned(carry, 0, y.digits[i]);
            carry = c;
            digits[i] = sum;
            i += 1;
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
                digits_ptr.copy_to_nonoverlapping(uninit_ptr, N - 1);
                carry_ptr.copy_to_nonoverlapping(uninit_ptr.offset(N as isize - 1), 1);
                Self {
                    digits: uninit.assume_init(),
                    exponent: handle_exp_overflow!(y.exponent.checked_add(1), Self::inf(sign)),
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
        let mut i = 0;
        while i < N - exp_diff {
            let (sub, b) = arithmetic::sub_borrow_unsigned(borrow, y.digits[i], x.digits[i + exp_diff]);
            borrow = b;
            digits[i] = sub;
            i += 1;
        }
        while i < N {
            // Can optimise later as third argument will always be zero
            let (sub, b) = arithmetic::sub_borrow_unsigned(borrow, y.digits[i], 0);
            borrow = b;
            digits[i] = sub;
            i += 1;
        }
        Self::finalise(Self {
            sign,
            digits,
            exponent
        })
    }
    pub const fn to_buint(&self) -> BUint<N> {
        BUint::from_digits(self.digits)
    }
    pub const fn div(self, rhs: Self) -> Self {
        let sign = match (&self.sign, &rhs.sign) {
            (Sign::NaN, _) => return Self::NAN,
            (_, Sign::NaN) => return Self::NAN,
            (Sign::PositiveInfinity, Sign::PositiveInfinity) => return Self::NAN,
            (Sign::NegativeInfinity, Sign::NegativeInfinity) => return Self::NAN,
            (Sign::PositiveInfinity, Sign::NegativeInfinity) => return Self::NAN,
            (Sign::NegativeInfinity, Sign::PositiveInfinity) => return Self::NAN,
            (Sign::PositiveInfinity, Sign::Positive) => return Self::INFINITY,
            (Sign::NegativeInfinity, Sign::Negative) => return Self::INFINITY,
            (Sign::PositiveInfinity, Sign::Negative) => return Self::NEG_INFINITY,
            (Sign::NegativeInfinity, Sign::Positive) => return Self::NEG_INFINITY,
            (Sign::Positive, Sign::PositiveInfinity) => return Self::ZERO,
            (Sign::Negative, Sign::NegativeInfinity) => return Self::ZERO,
            (Sign::Negative, Sign::PositiveInfinity) => return Self::NEG_ZERO,
            (Sign::Positive, Sign::NegativeInfinity) => return Self::NEG_ZERO,
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Negative, Sign::Negative) => Sign::Positive,
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative,
        };
        let (uint, lt) = crate::uint::div_float(self.to_buint(), rhs.to_buint());
        let exponent = if lt {
            let a = handle_exp_overflow!(self.exponent.checked_sub(rhs.exponent), mul_div_handler!(self, sign));
            handle_exp_overflow!(a.checked_sub(1), mul_div_handler!(self, sign))
        } else {
            handle_exp_overflow!(self.exponent.checked_sub(rhs.exponent), mul_div_handler!(self, sign))
        };
        Self::finalise(Self {
            digits: uint.digits(),
            exponent,
            sign,
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

impl<const N: usize> Mul for Float<N> where [(); 2 * N]: Sized {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        //assert_eq!(self.leading_zeros(), 0);
        //assert_eq!(rhs.leading_zeros(), 0);
        //debug_assert!(self.leading_zeros() == 0 && rhs.leading_zeros() == 0);
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
        let mut digits = [0; 2 * N];
        let mut carry = 0;
        let mut i = 0;
        while i < N {
            carry = 0;
            let mut j = 0;
            while j < N {
                let index = i + j;
                let (prod, c) = arithmetic::mul_carry_unsigned(carry, digits[index], self.digits[i], rhs.digits[j]);
                digits[index] = prod;
                carry = c;
                j += 1;
                if j == N {
                    digits[index + 1] = carry;
                }
            }
            i += 1;
        }
        let mut uninit = MaybeUninit::<[Digit; N]>::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut Digit;
        let digits_ptr = digits.as_ptr();
        return if digits[2 * N - 1] == 0 {
            unsafe {
                digits_ptr.add(N - 1).copy_to_nonoverlapping(uninit_ptr, N);
                Self {
                    digits: uninit.assume_init(),
                    exponent: handle_exp_overflow!(self.exponent.checked_add(rhs.exponent), mul_div_handler!(self, sign)),
                    sign,
                }
            }
        } else {
            unsafe {
                digits_ptr.add(N).copy_to_nonoverlapping(uninit_ptr, N);
                Self {
                    digits: uninit.assume_init(),
                    exponent: {
                        let a = handle_exp_overflow!(self.exponent.checked_add(rhs.exponent), mul_div_handler!(self, sign));
                        handle_exp_overflow!(a.checked_add(1), mul_div_handler!(self, sign))
                    },
                    sign,
                }
            }
        };
        /*let mut temp_digits = [0; N];
        let mut out_digits = [0; N];
        let mut i = 0;
        let mut carry = 0;
        while i < N {
            carry = 0;
            let mut j = 0;
            while j < N {
                let index = i + j;
                if index < N {
                    let (prod, c) = arithmetic::mul_carry_unsigned(carry, temp_digits[index], self.digits[i], rhs.digits[j]);
                    carry = c;
                    temp_digits[index] = prod;
                } else {
                    let (prod, c) = arithmetic::mul_carry_unsigned(carry, out_digits[index - N], self.digits[i], rhs.digits[j]);
                    carry = c;
                    out_digits[index - N] = prod;
                }
                j += 1;
            }
            i += 1;
        }
        if carry != 0 {
            out_digits[N - 1] = carry;
            return Self {
                exponent: self.exponent + rhs.exponent + 1,
                sign,
                digits: out_digits,
            };
        }
        let mut uninit = MaybeUninit::<[Digit; N]>::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut Digit;
        let digits_ptr = out_digits.as_ptr();
    
        let end_arr = [temp_digits[N - 1]];
        let end_ptr = end_arr.as_ptr();
    
        unsafe {
            digits_ptr.copy_to_nonoverlapping(uninit_ptr.offset(1), N - 1);
            end_ptr.copy_to_nonoverlapping(uninit_ptr, 1);
            Self {
                digits: uninit.assume_init(),
                exponent: self.exponent + rhs.exponent,
                sign,
            }
        }*/
    }
}

impl<const N: usize> Div for Float<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::div(self, rhs)
    }
}

impl<const N: usize> Neg for Float<N> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(self)
    }
}