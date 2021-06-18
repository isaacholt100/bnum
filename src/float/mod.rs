mod ops;
mod numtraits;
mod digit;

use digit::Digit;

use core::str::FromStr;
use core::num::FpCategory;

type Exponent = i128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy)]
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
        let mut out = Self::NEG_ZERO;
        out.digits[N - 1] = 1;
        out.exponent = Exponent::MIN;
        out
    };
    pub const MAX: Self = Self {
        digits: [digit::MAX; N],
        sign: Sign::Positive,
        exponent: Exponent::MAX,
    };
    pub const MIN: Self = Self {
        digits: [digit::MAX; N],
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
        out.digits[0] = 1;
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
    pub const fn is_zero(&self) -> bool {
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
    const fn trailing_zeros(&self) -> usize {
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
    pub const fn is_integer(&self) -> bool {
        self.is_finite() && self.trailing_zeros() as i128 + self.exponent >= N as i128 - 1
    }
    const fn compare_same_signs(&self, other: &Self) -> Ordering {
        if self.exponent < other.exponent {
            Ordering::Less
        } else if self.exponent > other.exponent {
            Ordering::Greater
        } else {
            let mut i = 0;
            while i < N {
                let a = self.digits[i];
                let b = other.digits[i];
                if a > b {
                    return Ordering::Greater;
                } else if a < b {
                    return Ordering::Less;
                }
                i += 1;
            }
            Ordering::Equal
        }
    }
    const fn leading_zeros(&self) -> usize {
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
    const fn rotate_digits_left(digits: [Digit; N], n: isize) -> [Digit; N] {
        let mut uninit = MaybeUninit::<[Digit; N]>::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut Digit;
        let digits_ptr = digits.as_ptr();
        
        unsafe {
            digits_ptr.offset(n).copy_to_nonoverlapping(uninit_ptr, N - n as usize);
            digits_ptr.copy_to_nonoverlapping(uninit_ptr.offset(N as isize - n), n as usize);
            core::mem::forget(digits);
            uninit.assume_init()
        }
    }
    const fn finalise(self) -> Self {
        let leading_zeros = self.leading_zeros();
        if leading_zeros == 0 {
            return self;
        }
        assert!(leading_zeros as isize >= 0);
        let exponent = self.exponent - leading_zeros as i128;
        let digits = Self::rotate_digits_left(self.digits, leading_zeros as isize);
        Self {
            sign: self.sign,
            digits,
            exponent,
        }
    }
    pub const fn abs(&self) -> Self {
        Self {
            exponent: self.exponent,
            digits: self.digits,
            sign: self.sign.abs(),
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
    pub fn from_digit(digit: Digit) -> Self {
        let mut digits = [0; N];
        digits[0] = digit;
        Self {
            digits,
            sign: Sign::Positive,
            exponent: 0,
        }
    }
}

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
        for d in &self.digits {
            out.push_str(&d.to_string());
        }
        write!(f, "{}", out)
    }
}

impl<const N: usize> Debug for Float<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_integer() {
            write!(f, "{}.0", self)
        } else {
            write!(f, "{}", self)
        }
    }
}

use core::mem::MaybeUninit;

fn rotate_array_left<const N: usize, T>(arr: [T; N], n: isize) -> [T; N] {
    let mut uninit = MaybeUninit::<[T; N]>::uninit();
    let uninit_ptr = uninit.as_mut_ptr() as *mut T;
    let arr_ptr = arr.as_ptr();
    unsafe {
        arr_ptr.offset(n).copy_to_nonoverlapping(uninit_ptr, N - n as usize);
        arr_ptr.copy_to_nonoverlapping(uninit_ptr.offset(N as isize - n), n as usize);
        core::mem::forget(arr);
        uninit.assume_init()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let arr = rotate_array_left(arr, 3);
        assert_eq!(arr, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_display() {
        let s = "304534895";
        let a = Float::<2>::NEG_ONE;
        let mut big = a;
        big.exponent = 2;
        let mut bigger = a;
        bigger.exponent = 3;
        println!("{}", bigger);
        println!("{}", a);
        println!("{}", u64::MAX);
        panic!("")
    }

    #[test]
    fn test() {
        let a = Float::<2>::from_digit(839457928347995u64);
        let b = Float::<2>::from_digit(13460129348701u64);
        let c = a * b;
        let d = Float::<2>::from_digit(198794092724950679u64);
        assert_eq!(format!("{}", c - d), "11299212298157794571504253816");
    }

    /*#[test]
    fn test_finalise() {
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