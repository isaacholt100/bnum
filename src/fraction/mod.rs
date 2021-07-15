use crate::BUint;
use crate::macros::expect;
use core::cmp::Ordering;
use crate::{ParseIntError, ParseRationalError};
use alloc::string::String;
use crate::ExpType;

#[cfg(feature = "serde_all")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_all")]
#[derive(Clone, Copy, Debug, Hash, Serialize, Deserialize)]
pub struct Fraction<const N: usize> {
    numerator: BUint<N>,
    denominator: BUint<N>,
    positive: bool,
}

#[cfg(not(feature = "serde_all"))]
#[derive(Clone, Copy, Debug, Hash)]
pub struct Fraction<const N: usize> {
    numerator: BUint<N>,
    denominator: BUint<N>,
    positive: bool,
}

impl<const N: usize> Fraction<N> {
    pub const ZERO: Self = Self {
        positive: true,
        numerator: BUint::ZERO,
        denominator: BUint::ONE,
    };
    pub const ONE: Self = Self {
        positive: true,
        numerator: BUint::ONE,
        denominator: BUint::ONE,
    };
    pub const NEG_ONE: Self = Self {
        positive: false,
        numerator: BUint::ONE,
        denominator: BUint::ONE,
    };
    pub const MAX: Self = Self {
        positive: true,
        numerator: BUint::MAX,
        denominator: BUint::ONE,
    };
    pub const MIN: Self = Self {
        positive: false,
        numerator: BUint::MAX,
        denominator: BUint::ONE,
    };
    pub const MIN_POSITIVE: Self = Self {
        positive: true,
        numerator: BUint::ONE,
        denominator: BUint::MAX,
    };
    pub const MAX_NEGATIVE: Self = Self {
        positive: false,
        numerator: BUint::ONE,
        denominator: BUint::MAX,
    };
}

impl<const N: usize> Fraction<N> {
    pub const fn numerator(&self) -> BUint<N> {
        self.numerator
    }
    pub const fn denominator(&self) -> BUint<N> {
        self.denominator
    }
    pub const unsafe fn new_unchecked(positive: bool, numerator: BUint<N>, denominator: BUint<N>) -> Self {
        Self {
            numerator,
            denominator,
            positive,
        }
    }
    const fn reduce(mut numerator: BUint<N>, mut denominator: BUint<N>) -> (BUint<N>, BUint<N>) {
        let gcd = BUint::gcd(numerator, denominator);
        numerator = numerator.wrapping_div(gcd);
        denominator = denominator.wrapping_div(gcd);
        (numerator, denominator)
    }
    pub const fn new_checked(positive: bool, numerator: BUint<N>, denominator: BUint<N>) -> Option<Self> {
        if denominator.is_zero() {
            return None;
        }
        let (numerator, denominator) = Self::reduce(numerator, denominator);
        Some(Self {
            numerator,
            denominator,
            positive,
        })
    }
    pub const fn new(positive: bool, numerator: BUint<N>, denominator: BUint<N>) -> Self {
        expect!(Self::new_checked(positive, numerator, denominator), "Cannot create fraction with denominator of zero")
    }
    pub const fn is_one(&self) -> bool {
        self.numerator.is_one() && self.denominator.is_one()
    }
    pub const fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }
    pub const fn is_positive(&self) -> bool {
        self.positive && !self.is_zero()
    }
    pub const fn is_negative(&self) -> bool {
        !self.positive && !self.is_zero()
    }
    pub const fn abs(self) -> Self {
        Self {
            numerator: self.numerator,
            denominator: self.denominator,
            positive: true,
        }
    }
    pub const fn neg(self) -> Self {
        Self {
            numerator: self.numerator,
            denominator: self.denominator,
            positive: !self.positive,
        }
    }
    pub const fn inv(self) -> Self {
        expect!(Self::new_checked(self.positive, self.denominator, self.numerator), "Attempt to divide by zero")
    }
    pub const fn ceil(self) -> Self {
        if self.positive {
            let (div, rem) = self.numerator.div_rem(self.denominator);
            if rem.is_zero() {
                Self {
                    numerator: div,
                    denominator: BUint::ONE,
                    positive: true,
                }
            } else {
                Self {
                    numerator: div.wrapping_add(BUint::ONE),
                    denominator: BUint::ONE,
                    positive: true,
                }
            }
        } else {
            let numerator = self.numerator.wrapping_div(self.denominator);
            Self {
                numerator,
                denominator: BUint::ONE,
                positive: false,
            }
        }
    }
    pub const fn floor(self) -> Self {
        if self.positive {
            let numerator = self.numerator.wrapping_div(self.denominator);
            Self {
                numerator,
                denominator: BUint::ONE,
                positive: true,
            }
        } else {
            let (div, rem) = self.numerator.div_rem(self.denominator);
            if rem.is_zero() {
                Self {
                    numerator: div,
                    denominator: BUint::ONE,
                    positive: false,
                }
            } else {
                Self {
                    numerator: div.wrapping_add(BUint::ONE),
                    denominator: BUint::ONE,
                    positive: false,
                }
            }
        }
    }
    pub const fn trunc(self) -> Self {
        Self {
            positive: self.positive,
            numerator: self.numerator.wrapping_div(self.denominator),
            denominator: BUint::ONE,
        }
    }
    pub const fn round(self) -> Self {
        let (_, rem) = self.numerator.div_rem(self.denominator);
        let numerator = match rem.cmp(&self.denominator.wrapping_shr(1)) {
            Ordering::Greater => self.numerator.wrapping_add(BUint::ONE),
            _ => self.numerator,
        };
        Self {
            positive: self.positive,
            numerator,
            denominator: BUint::ONE,
        }
    }
    pub const fn cmp(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() {
            Ordering::Equal
        } else if self.positive > other.positive {
            Ordering::Greater
        } else if self.positive < other.positive {
            Ordering::Less
        } else {
            // Don't compare a/b and c/d by comparing ad and bc, as the multiplication of a and d or b and c may overflow.
            // Instead, compare floor(a / b) and a mod b with floor(c / d) and c mod d.
            let (div1, rem1) = self.numerator.div_rem(self.denominator);
            let (div2, rem2) = other.numerator.div_rem(other.denominator);
            match div1.cmp(&div2) {
                Ordering::Greater => {
                    if self.positive {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                },
                Ordering::Less => {
                    if self.positive {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                },
                Ordering::Equal => {
                    match rem1.cmp(&rem2) {
                        Ordering::Greater => {
                            if self.positive {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        },
                        Ordering::Less => {
                            if self.positive {
                                Ordering::Less
                            } else {
                                Ordering::Greater
                            }
                        },
                        Ordering::Equal => Ordering::Equal,
                    }
                },
            }
        }
    }
    pub const fn cmp_abs(&self, other: &Self) -> Ordering {
        let (div1, rem1) = self.numerator.div_rem(self.denominator);
        let (div2, rem2) = other.numerator.div_rem(other.denominator);
        match div1.cmp(&div2) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => rem1.cmp(&rem2),
        }
    }
    pub const fn clamp(&self, min: &Self, max: &Self) -> Self {
        if let Ordering::Greater = min.cmp(max) {
            panic!("assertion failed: min <= max");
        }
        match self.cmp(min) {
            Ordering::Greater => {
                match self.cmp(max) {
                    Ordering::Greater => *max,
                    _ => *self,
                }
            },
            _ => *min,
        }
    }
    fn map_err(result: Result<BUint<N>, ParseIntError>) -> Result<BUint<N>, ParseRationalError> {
        result.map_err(|e| ParseRationalError {
            reason: crate::ParseRationalErrorReason::ParseIntError(e)
        })
    }
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, crate::ParseRationalError> {
        let mut positive = true;
        let mut src = src;
        if src.starts_with('-') {
            src = &src[1..];
            positive = false;
        } else if src.starts_with('+') {
            src = &src[1..];
        }
        for (index, c) in src.char_indices() {
            if c == '/' {
                let (s1, s2) = src.split_at(index);
                let numerator = Self::map_err(BUint::from_str_radix(s1, radix))?;
                let denominator = Self::map_err(BUint::from_str_radix(&s2[1..], radix))?;
                return Self::new_checked(positive, numerator, denominator).ok_or(crate::ParseRationalError { 
                    reason: crate::ParseRationalErrorReason::ZeroDenominator
                });
            }
        }
        Ok(Self {
            positive,
            numerator: Self::map_err(BUint::from_str_radix(src, radix))?,
            denominator: BUint::ONE,
        })
    }
    pub const fn signum(&self) -> Self {
        if self.is_zero() {
            Self::ZERO
        } else if self.positive {
            Self::ONE
        } else {
            Self::NEG_ONE
        }
    }
    pub fn to_str_radix(self, radix: u32) -> String {
        if self.denominator.is_one() {
            self.numerator.to_str_radix(radix)
        } else {
            format!("{}/{}", self.numerator.to_str_radix(radix), self.denominator.to_str_radix(radix))
        }
    }
    pub const fn is_int(&self) -> bool {
        self.denominator.is_zero()
    }
    pub const fn frac(self) -> Self {
        let rem = self.numerator.wrapping_rem(self.denominator);
        Self {
            positive: self.positive,
            numerator: rem,
            denominator: self.denominator,
        }
    }
    pub const fn pow(self, exp: isize) -> Self {
        if exp < 0 {
            Self {
                positive: self.positive || exp & 1 == 0,
                numerator: self.denominator.pow((-exp) as ExpType),
                denominator: self.numerator.pow((-exp) as ExpType),
            }
        } else {
            Self {
                positive: self.positive || exp & 1 == 0,
                numerator: self.numerator.pow(exp as ExpType),
                denominator: self.denominator.pow(exp as ExpType),
            }
        }
    }
    pub const fn checked_pow(self, exp: ExpType) -> Option<Self> {
        Some(Self {
            positive: self.positive || exp & 1 == 0,
            numerator: match self.numerator.checked_pow(exp) {
                Some(p) => p,
                None => return None,
            },
            denominator: match self.denominator.checked_pow(exp) {
                Some(p) => p,
                None => return None,
            },
        })
    }
}

use core::fmt::{Display, LowerHex, UpperHex, Binary, Octal, Formatter, self};
use core::str::FromStr;

macro_rules! fmt_impl {
    ($Trait: ty, $int_fmt: expr, $frac_fmt: expr) => {
        impl<const N: usize> $Trait for Fraction<N> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                if self.denominator.is_one() {
                    write!(f, $int_fmt, self.numerator)
                } else {
                    write!(f, $frac_fmt, self.numerator, self.denominator)
                }
            }
        }
    }
}

fmt_impl!(Display, "{}", "{}/{}");
fmt_impl!(LowerHex, "{:x}", "{:x}/{:x}");
fmt_impl!(UpperHex, "{:X}", "{:X}/{:X}");
fmt_impl!(Binary, "{:b}", "{:b}/{:b}");
fmt_impl!(Octal, "{:o}", "{:o}/{:o}");

impl<const N: usize> Default for Fraction<N> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const N: usize> FromStr for Fraction<N> {
    type Err = ParseRationalError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

use core::cmp::{PartialEq, Eq, PartialOrd, Ord};

impl<const N: usize> PartialEq for Fraction<N> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            true
        } else {
            BUint::eq(&self.numerator, &other.numerator) && BUint::eq(&self.denominator, &other.denominator)
        }
    }
}

impl<const N: usize> Eq for Fraction<N> {}

impl<const N: usize> PartialOrd for Fraction<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl<const N: usize> Ord for Fraction<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp(self, other)
    }
}

use core::ops::{Add, Sub, Mul, Div, Rem};

impl<const N: usize> Add for Fraction<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> Sub for Fraction<N> {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> Mul for Fraction<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let (a, d) = Self::reduce(self.numerator, rhs.denominator);
        let (b, c) = Self::reduce(self.denominator, rhs.numerator);
        let numerator = a * c;
        let denominator = b * d;
        Self {
            positive: self.positive == rhs.positive,
            numerator,
            denominator,
        }
    }
}

impl<const N: usize> Div for Fraction<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

impl<const N: usize> Rem for Fraction<N> {
    type Output = Self;
    
    fn rem(self, rhs: Self) -> Self {
        todo!()
    }
}

use core::iter::{Product, Sum, Iterator};

impl<const N: usize> Sum<Self> for Fraction<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for Fraction<N> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + *b)
    }
}

impl<const N: usize> Product<Self> for Fraction<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for Fraction<N> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}