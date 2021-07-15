
#[macro_use]
mod ops;
mod numtraits;

use crate::digit::{Digit, self};

use core::str::FromStr;
use core::num::FpCategory;

type Exponent = i128;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Sign {
    NaN,
    NegativeInfinity,
    Negative,
    Positive,
    PositiveInfinity,
}

impl Sign {
    const fn abs(&self) -> Self {
        match self {
            Sign::Negative => Sign::Positive,
            Sign::NegativeInfinity => Sign::PositiveInfinity,
            Sign::Positive => Sign::Positive,
            Sign::PositiveInfinity => Sign::PositiveInfinity,
            Sign::NaN => Sign::NaN,
        }
    }
    const fn neg(&self) -> Self {
        match self {
            Sign::NaN => Sign::NaN,
            Sign::Negative => Sign::Positive,
            Sign::Positive => Sign::Negative,
            Sign::NegativeInfinity => Sign::PositiveInfinity,
            Sign::PositiveInfinity => Sign::NegativeInfinity,
        }
    }
}

/// Digits stored in little endian
#[derive(Clone, Copy, Debug, Hash)]
pub struct Float<const N: usize> {
    exponent: Exponent,
    sign: Sign,
    digits: [Digit; N],
}

impl<const N: usize> Float<N> {
    pub const MIN_POSITIVE: Self = {
        let mut out = Self::ZERO;
        out.digits[N - 1] = 1;
        out.exponent = Exponent::MIN;
        out
    };
    pub const MAX_NEGATIVE: Self = {
        let mut out = Self::MIN_POSITIVE;
        out.sign = Sign::Negative;
        out
    };
    pub const MAX: Self = Self {
        digits: [Digit::MAX; N],
        sign: Sign::Positive,
        exponent: Exponent::MAX,
    };
    pub const MIN: Self = Self {
        digits: [Digit::MAX; N],
        sign: Sign::Negative,
        exponent: Exponent::MAX,
    };
    pub const ZERO: Self = Self {
        exponent: 0,
        sign: Sign::Positive,
        digits: [0; N],
    };
    pub const NEG_ZERO: Self = Self {
        exponent: 0,
        sign: Sign::Negative,
        digits: [0; N],
    };
    pub const ONE: Self = {
        let mut out = Self::ZERO;
        out.digits[N - 1] = 1;
        out
    };
    pub const NEG_ONE: Self = {
        let mut out = Self::ONE;
        out.sign = Sign::Negative;
        out
    };
    pub const INFINITY: Self = Self {
        exponent: 0,
        sign: Sign::PositiveInfinity,
        digits: [0; N],
    };
    pub const NEG_INFINITY: Self = Self {
        exponent: 0,
        sign: Sign::NegativeInfinity,
        digits: [0; N],
    };
    pub const NAN: Self = Self {
        exponent: 0,
        sign: Sign::NaN,
        digits: [0; N],
    };
    /*pub fn finalise(&self, sd: Option<f64>, truncated: bool, external: bool) -> Self {
        let mut out = *self;
        macro_rules! handle_external {
            () => {
                if external {
                    if self.exponent > Self::MAX_E {
                        if self.is_sign_positive() {
                            out = Self::POS_INFINITY;
                        } else {
                            out = Self::NEG_INFINITY;
                        }
                    } else if self.exponent < Self::MIN_E {
                        if self.is_sign_positive() {
                            out = Self::POS_ZERO;
                        } else {
                            out = Self::NEG_ZERO;
                        }
                    }
                }
            };
        }
        if let Some(mut sd) = sd {
            let mut j = 0;
            let mut rd = 0;
            let mut round_up = false;
            let mut w = 0;
            let mut xdi = 0;

            if !self.is_finite() {
                return out;
            }
            let mut digits = 1;
            let mut k = self.digits[0];
            while k >= 10 {
                digits += 1;
                k /= 10;
            }
            let mut i = sd - digits;
            println!("{}", i);
            if i < 0 {
                i += LOG_BASE as f64;
                j = sd;
                xdi = 0;
                w = self.digits[0];
                rd = w as u128 / u128::pow(10, (digits - j) as u32 - 1) % 10;
            } else {
                xdi = ((i + 1) / LOG_BASE as f64).ceil();
                k = N as f64;
                if xdi >= k {
                    if truncated {
                        digits = 1;
                        rd = 0;
                        i %= LOG_BASE as f64;
                        j = i - LOG_BASE as f64 + 1;
                    } else {
                        handle_external!();
                        println!("{}", xdi);
                        println!("{:?}", self);
                        return out;
                    }
                } else {
                    w = self.digits[xdi as usize];
                    k = w;
                    digits = 1;
                    while k >= 10 {
                        k /= 10;
                        digits += 1;
                    }
                    i %= LOG_BASE as f64;
                    j = i - LOG_BASE as f64 + digits;
                    rd = if j < 0 {
                        0
                    } else {
                        w as u128 / u128::pow(10, (digits - j) as u32 - 1) % 10
                    };
                }
            }
            let truncated = truncated || sd < 0 || xdi as usize + 1 < N || (if j < 0 {
                w as u128
            } else {
                w as u128 % u128::pow(10, (digits - j) as u32 - 1)
            }) != 0;

            if sd < 1 || out.digits[0] == DIGIT_ZERO {
                out.digits = [DIGIT_ZERO; N];
                if round_up {
                    sd -= out.exponent as f64 + 1;
                    out.digits[0] = f64::powf(10, (LOG_BASE as f64 - sd % LOG_BASE as f64) % LOG_BASE as f64);
                    out.exponent = -sd.trunc() as Exponent;
                } else {
                    out.exponent = 0;
                }
                return out;
            }
            if i == 0 {
                for d in out.digits[xdi as usize..].iter_mut() {
                    *d = DIGIT_ZERO;
                }
                k = 1;
                xdi -= 1;
            } else {
                for d in out.digits[xdi as usize + 1..].iter_mut() {
                    *d = DIGIT_ZERO;
                }
                k = f64::powf(10, LOG_BASE as f64 - i);
                out.digits[xdi as usize] = if j > 0 {
                    (w / f64::powf(10, digits - j) % f64::powf(10, j)).trunc() * k
                } else {
                    0
                };
            }
        }
        handle_external!();
        out
    }*/
    const fn usize_to_exponent(u: usize) -> Exponent {
        u as Exponent
    }
    /// Returns (smaller, larger)
    const fn order_by_exponent(a: Self, b: Self) -> (Self, Self) {
        if b.exponent < a.exponent {
            (b, a)
        } else {
            (a, b)
        }
    }
    const fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.sign, &other.sign) {
            (Sign::NaN, _) => None,
            (_, Sign::NaN) => None,
            (Sign::PositiveInfinity, Sign::PositiveInfinity) => Some(Ordering::Equal),
            (Sign::NegativeInfinity, Sign::NegativeInfinity) => Some(Ordering::Equal),
            (Sign::PositiveInfinity, _) => Some(Ordering::Greater),
            (Sign::NegativeInfinity, _) => Some(Ordering::Less),
            (_, Sign::PositiveInfinity) => Some(Ordering::Less),
            (_, Sign::NegativeInfinity) => Some(Ordering::Greater),
            (Sign::Positive, Sign::Negative) => {
                if self.is_zero() && other.is_zero() {
                    Some(Ordering::Equal)    
                } else {
                    Some(Ordering::Greater)
                }
            },
            (Sign::Negative, Sign::Positive) => {
                if self.is_zero() && other.is_zero() {
                    Some(Ordering::Equal)    
                } else {
                    Some(Ordering::Less)
                }
            },
            (Sign::Positive, Sign::Positive) => {
                Some(self.compare_same_signs(other))
            },
            (Sign::Negative, Sign::Negative) => {
                Some(other.compare_same_signs(self))
            },
        }
    }
    const fn zero(sign: Sign) -> Self {
        if let Sign::Negative = sign {
            Self::NEG_ZERO
        } else {
            Self::ZERO
        }
    }
    const fn inf(sign: Sign) -> Self {
        if let Sign::Negative = sign {
            Self::NEG_INFINITY
        } else {
            Self::INFINITY
        }
    }
    pub const fn is_zero(&self) -> bool {
        match &self.sign {
            Sign::NaN | Sign::NegativeInfinity | Sign::PositiveInfinity => return false,
            _ => {},
        };
        let mut i = 0;
        while i < N {
            if self.digits[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }
    pub const fn is_nan(&self) -> bool {
        match &self.sign {
            Sign::NaN => true,
            _ => false,
        }
    }
    pub const fn is_infinite(&self) -> bool {
        match &self.sign {
            Sign::PositiveInfinity | Sign::NegativeInfinity => true,
            _ => false,
        }
    }
    pub const fn is_finite(&self) -> bool {
        match &self.sign {
            Sign::Positive | Sign::Negative => true,
            _ => false,
        }
    }
    pub const fn is_sign_positive(&self) -> bool {
        match &self.sign {
            Sign::Positive | Sign::PositiveInfinity => true,
            _ => false,
        }
    }
    pub const fn is_sign_negative(&self) -> bool {
        match &self.sign {
            Sign::Negative | Sign::NegativeInfinity => true,
            _ => false,
        }
    }
    const fn sf(&self) -> usize {
        N - self.trailing_zeros()
    }
    const fn trailing_zeros(&self) -> usize {
        let mut zeros = 0;
        let mut i = 0;
        while i < N {
            if self.digits[i] == 0 {
                zeros += 1;
            } else {
                break;
            }
            i += 1;
        }
        zeros
    }
    pub const fn is_integer(&self) -> bool {
        if self.is_zero() {
            return true;
        }
        if self.exponent.is_negative() {
            return false;
        }
        if self.exponent < N as Exponent - 1 {
            let mut i = 0;
            while i < N - self.exponent as usize - 1 {
                if self.digits[i] != 0 {
                    return false;
                }
                i += 1;
            }
            return true;
        } else {
            return true;
        }
        self.is_finite() && self.exponent >= Self::usize_to_exponent(N) - Self::usize_to_exponent(self.trailing_zeros()) - 1
    }
    const fn compare_same_signs(&self, other: &Self) -> Ordering {
        if self.exponent < other.exponent {
            Ordering::Less
        } else if self.exponent > other.exponent {
            Ordering::Greater
        } else {
            let mut i = N;
            while i > 0 {
                i -= 1;
                let a = self.digits[i];
                let b = other.digits[i];
                if a > b {
                    return Ordering::Greater;
                } else if a < b {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
    const fn leading_zeros(&self) -> usize {
        let mut zeros = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            if self.digits[i] == 0 {
                zeros += 1;
            } else {
                break;
            }
        }
        zeros
    }
    const fn rotate_digits_right(digits: [Digit; N], n: usize) -> [Digit; N] {
        let mut uninit = MaybeUninit::<[Digit; N]>::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut Digit;
        let digits_ptr = digits.as_ptr();
        
        unsafe {
            digits_ptr.copy_to_nonoverlapping(uninit_ptr.offset(n as isize), N - n);
            digits_ptr.offset((N - n) as isize).copy_to_nonoverlapping(uninit_ptr, n);
            core::mem::forget(digits);
            uninit.assume_init()
        }
    }
    const fn finalise(self) -> Self {
        let leading_zeros = self.leading_zeros();
        if leading_zeros == 0 || leading_zeros == N {
            return self;
        }
        assert!(leading_zeros as isize >= 0);
        let exponent = self.exponent - leading_zeros as i128;
        let digits = Self::rotate_digits_right(self.digits, leading_zeros);
        Self {
            sign: self.sign,
            digits,
            exponent,
        }
    }
    pub const fn abs(self) -> Self {
        Self {
            exponent: self.exponent,
            digits: self.digits,
            sign: self.sign.abs(),
        }
    }
    pub fn abs_sub(self, other: Self) -> Self {
        if let Some(Ordering::Greater) = self.partial_cmp(&other) {
            self - other
        } else {
            Self::ZERO
        }
    }
    pub const fn classify(&self) -> FpCategory {
        match self.sign {
            Sign::NaN => FpCategory::Nan,
            Sign::NegativeInfinity | Sign::PositiveInfinity => FpCategory::Infinite,
            _ => {
                if self.is_zero() {
                    FpCategory::Zero
                } else {
                    FpCategory::Normal
                }
            }
        }
    }
    pub const fn signum(&self) -> Self {
        match &self.sign {
            Sign::NaN => Self::NAN,
            Sign::NegativeInfinity | Sign::Negative => Self::NEG_ONE,
            _ => Self::ONE,
        }
    }
    pub const fn neg(self) -> Self {
        Self {
            digits: self.digits,
            exponent: self.exponent,
            sign: self.sign.neg(),
        }
    }
    pub const fn copysign(&self, sign: &Self) -> Self {
        match (&self.sign, &sign.sign) {
            (Sign::NaN, _) => Self::NAN,
            (_, Sign::NaN) => Self::NAN,
            (Sign::Positive, Sign::Negative) => self.neg(),
            (Sign::Positive, Sign::NegativeInfinity) => self.neg(),
            (Sign::PositiveInfinity, Sign::Negative) => self.neg(),
            (Sign::PositiveInfinity, Sign::NegativeInfinity) => self.neg(),
            (Sign::Negative, Sign::Positive) => self.neg(),
            (Sign::Negative, Sign::PositiveInfinity) => self.neg(),
            (Sign::NegativeInfinity, Sign::Positive) => self.neg(),
            (Sign::NegativeInfinity, Sign::PositiveInfinity) => self.neg(),
            _ => *self,
        }
    }
    pub const fn max(&self, other: &Self) -> Self {
        match self.partial_cmp(other) {
            Some(o) => match o {
                Ordering::Less => *other,
                _ => *self,
            },
            None => if self.is_nan() {
                *other
            } else {
                *self
            }
        }
    }
    pub const fn min(&self, other: &Self) -> Self {
        match self.partial_cmp(other) {
            Some(o) => match o {
                Ordering::Greater => *other,
                _ => *self,
            },
            None => if self.is_nan() {
                *other
            } else {
                *self
            }
        }
    }
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        if crate::macros::expect!(min.partial_cmp(max), "assertion failed: min <= max") == Ordering::Greater {
            panic!("assertion failed: min <= max");
        }
        match self.partial_cmp(min) {
            None => Self::NAN,
            Some(o) => match o {
                Ordering::Greater => {
                    match self.partial_cmp(max).unwrap() {
                        Ordering::Greater => *max,
                        _ => *self,
                    }
                },
                _ => *min
            }
        }
    }
    pub const fn from_digit(digit: Digit) -> Self {
        let mut digits = [0; N];
        digits[N - 1] = digit;
        Self {
            digits,
            sign: Sign::Positive,
            exponent: 0,
        }
    }
    pub fn to_hex_string(self) -> String {
        let to_str = |sign_str| {
            if self.is_zero() {
                return "0".into();
            }
            if self.exponent.is_negative() {
                format!(
                    "{sign}0.{:0>width$}{:x}",
                    "",
                    self.to_buint(),
                    width = (-(self.exponent + 1) as usize) * digit::BITS / 4,
                    sign = sign_str,
                )
            } else if self.is_integer() {
                let int_str = self.to_buint().to_str_radix(16);
                let int_str = int_str.trim_end_matches('0');
                let diff = self.exponent - Self::usize_to_exponent(N - self.trailing_zeros()) + 1;
                println!("diff: {} {:?}", diff, (self.exponent, N));
                format!("{sign}{int}{:0>width$}", "", sign = sign_str, int = int_str, width = diff as usize * digit::BITS / 4)
            } else {
                let int_str = self.to_buint().to_str_radix(16);
                let diff = Self::usize_to_exponent(N - self.trailing_zeros()) - self.exponent - 1;
                let (int, frac) = int_str.split_at(int_str.len() - diff as usize * digit::BITS / 4);
                format!("{}{}.{}", sign_str, int, frac)
            }
        };
        match &self.sign {
            Sign::PositiveInfinity => "inf".into(),
            Sign::NegativeInfinity => "-inf".into(),
            Sign::NaN => "NaN".into(),
            Sign::Positive => to_str(""),
            Sign::Negative => to_str("-"),
        }
    }
    fn powu(self, exp: u32) -> Self where [(); 2 * N]: Sized {
        if exp == 0 {
            return Self::ONE;
        }
        if self.is_zero() {
            return Self::ZERO;
        }
        let mut y = Self::ONE;
        let mut n = exp;
        let mut x = self;

        while n > 1 {
            if n & 1 == 0 {
                x = x * x;
                n >>= 1;
            } else {
                y = x * y;
                x = x * x;
                n -= 1;
                n >>= 1;
            }
        }
        x * y
    }
    pub fn powi(self, exp: i32) -> Self where [(); 2 * N]: Sized {
        if exp.is_negative() {
            Self::ONE / self.powu(-exp as u32)
        } else {
            self.powu(exp as u32)
        }
    }
    pub const fn trunc(self) -> Self {
        if self.is_integer() {
            self
        } else if self.exponent.is_negative() {
            Self::ZERO
        } else {
            let mut out = self;
            let e = self.exponent as usize + 1;
            let mut i = 0;
            while i < N - e {
                out.digits[i] = 0;
                i += 1;
            }
            out
        }
    }
    pub fn floor(self) -> Self {
        if self.is_sign_positive() {
            self.trunc()
        } else {
            if self.is_integer() {
                self
            } else {
                self.trunc() - Self::ONE
            }
        }
    }
    pub fn ceil(self) -> Self {
        if self.is_sign_negative() {
            self.trunc()
        } else {
            if self.is_integer() {
                self
            } else {
                self.trunc() + Self::ONE
            }
        }
    }
    pub fn round(self) -> Self {
        if self.is_integer() {
            self
        } else {
            todo!()
        }
    }
}

use core::mem::MaybeUninit;

use core::cmp::{PartialEq, PartialOrd, Ordering};

impl<const N: usize> PartialEq for Float<N> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.sign, &other.sign) {
            (Sign::PositiveInfinity, Sign::PositiveInfinity) => {
                true
            },
            (Sign::NegativeInfinity, Sign::NegativeInfinity) => {
                true
            },
            (Sign::Positive, Sign::Positive) => {
                &self.exponent == &other.exponent && &self.digits == &other.digits
            },
            (Sign::Negative, Sign::Negative) => {
                &self.exponent == &other.exponent && &self.digits == &other.digits
            },
            (Sign::Positive, Sign::Negative) => {
                self.is_zero() && other.is_zero()
            },
            (Sign::Negative, Sign::Positive) => {
                self.is_zero() && other.is_zero()
            },
            _ => false,
        }
    }
}

impl<const N: usize> PartialOrd for Float<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Self::partial_cmp(self, other)
    }
}

impl<const N: usize> FromStr for Float<N> {
    type Err = &'static str;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "inf" => Ok(Self::INFINITY),
            "-inf" => Ok(Self::NEG_INFINITY),
            "NaN" => Ok(Self::NAN),
            "-NaN" => Ok(Self::NAN),
            mut src => {
                let sign = match &src[0..1] {
                    "-" => {
                        src = &src[1..];
                        Sign::Negative
                    },
                    "+" => {
                        src = &src[1..];
                        Sign::Positive
                    },
                    _ => Sign::Positive,
                };
                todo!()
            }
        }
    }
}

use core::fmt::{Debug, Display, Formatter, self};

impl<const N: usize> Display for Float<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        /*let mut slice = self.digits;
        let slice = &mut slice;
        slice.reverse();
        use core::convert::TryInto;
        let digits: &mut [u64; N] = slice.try_into().unwrap();*/
        /*let u = crate::BUint::from_digits(self.digits);
        let mut out = format!("{}", u);
        let diff = self.exponent + 1 - N as i128;
        if diff == 0 {
            write!(f, "{}", out)
        } else if -diff == (N - 1) as i128 {
            write!(f, "{}", out)
        } else if diff < 0 {
            write!(f, "{}.{}", &out[0..(-diff) as usize], &out[(-diff) as usize..])
        } else {
            write!(f, "{}{:width$}", out, "", width = diff as usize)
        }*/
        let mut out = String::new();
        for d in self.digits.iter().rev() {
            out.push_str(&format!("{d:.19}", d = d));
        }
        write!(f, "{}", out)
    }
}

/*impl<const N: usize> Debug for Float<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_integer() {
            write!(f, "{}.0", self)
        } else {
            write!(f, "{}", self)
        }
    }
}*/

pub trait Array {
    fn rotate_left(self, n: usize) -> Self;
    fn rotate_right(self, n: usize) -> Self;
    fn rotate(self, n: isize) -> Self;
}

const MAX_N: usize = isize::MAX as usize;

impl<T, const N: usize> Array for [T; N] {
    fn rotate_left(self, n: usize) -> Self {
        let n = n % N;
        if n > MAX_N {
            rotate_array_right(self, N - n)
        } else {
            rotate_array_left(self, n)
        }
    }
    fn rotate_right(self, n: usize) -> Self {
        let n = n % N;
        if n > MAX_N {
            rotate_array_left(self, N - n)
        } else {
            rotate_array_right(self, n)
        }
    }
    fn rotate(self, n: isize) -> Self {
        if n.is_negative() {
            self.rotate_left(-n as usize)
        } else {
            self.rotate_right(n as usize)
        }
    }
}

fn rotate_array_left<const N: usize, T>(arr: [T; N], n: usize) -> [T; N] {
    debug_assert!(n <= isize::MAX as usize);
    let diff = N - n;
    let mut uninit = MaybeUninit::<[T; N]>::uninit();
    let uninit_ptr = uninit.as_mut_ptr() as *mut T;
    let arr_ptr = arr.as_ptr();
    unsafe {
        arr_ptr.add(n).copy_to_nonoverlapping(uninit_ptr, diff);
        arr_ptr.copy_to_nonoverlapping(uninit_ptr.add(diff), n);
        core::mem::forget(arr);
        uninit.assume_init()
    }
}

fn rotate_array_right<const N: usize, T>(arr: [T; N], n: usize) -> [T; N] {
    debug_assert!(n <= isize::MAX as usize);
    let diff = N - n;
    let mut uninit = MaybeUninit::<[T; N]>::uninit();
    let uninit_ptr = uninit.as_mut_ptr() as *mut T;
    let arr_ptr = arr.as_ptr();
    unsafe {
        arr_ptr.add(diff).copy_to_nonoverlapping(uninit_ptr, n);
        arr_ptr.copy_to_nonoverlapping(uninit_ptr.add(n), diff);
        core::mem::forget(arr);
        uninit.assume_init()
    }
}

impl<const N: usize> fmt::LowerHex for Float<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate() {
        let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let arr = arr.rotate_left(13);
        assert_eq!(arr, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let arr = arr.rotate_right(4);
        assert_eq!(arr, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
        assert_eq!(arr.rotate_right(usize::MAX), arr.rotate_right(usize::MAX % arr.len()));
    }

    #[test]
    fn display() {
        let one = Float::<2>::ONE;
        let two = Float::from_digit(2);
        println!("{}", (one / two).to_hex_string());
        let a = Float::<2>::from_digit(0x12);
        let b = Float::<2>::from_digit(0x56);
        println!("{:x}", a * b + a + b);
        println!("{:?}", Float::<16>::from_digit(5).powi(100));
        println!("{:x}", (Float::<64>::from_digit(0x56) * Float::from_digit(0x18) / Float::from_digit(17)).trunc());
        let a = Float::<2>::from_digit(0xf3);
        println!("{:x}", a.powi(800));
        panic!("")
    }

    #[test]
    fn test() {
        let a = Float::<3> {
            sign: Sign::Negative,
            digits: [2, 3, 4],
            exponent: 35,
        };
        assert_eq!(a / a, Float::ONE);
        let t = Float::from_digit(2);
        println!("t: {:?}", t);
        let mut r = Float::ONE;
        for i in 0..64 {
            r = r * t;
        }
        println!("{:?}", r);
        for _ in 0..64 {
            r = r / t;
        }
        assert_eq!(r, Float::<2>::ONE);
        println!("{:?}", Float::<2>::ONE / Float::from_digit(2));
        println!("{:x}", (Float::<2>::ONE * Float::from_digit(16)));
        println!("{}", Float::<2> {
            digits: [0, 1],
            exponent: 1,
            sign: Sign::Negative,
        }.to_hex_string());
        panic!("");
    }

    /*#[test]
    fn finalise() {
        let mut x = Float::<3> {
            digits: [1, 0, 1],
            exponent: 2,
            sign: Sign::Positive,
        };
        //x.finalise(Some(14), false, true);
        assert_eq!(x.digits, [1, 0, 0]);
        assert_eq!(x.exponent, 2);
        assert_eq!(x.sign, Sign::Positive);
        panic!("poo")
    }*/
}