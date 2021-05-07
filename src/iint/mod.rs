use crate::uint::BUint;
use crate::sign::Sign;
use crate::tryops::TryOps;
use num_traits::{One, Zero};

mod cmp;
mod convert;
mod ops;
mod tryops;
mod numtraits;

impl<const N: usize> BUint<N> {
    fn sign_or_zero(&self, sign: Sign) -> Sign {
        if self.is_zero() {
            Sign::Zero
        } else {
            sign
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct BIint<const N: usize> {
    uint: BUint<N>,
    sign: Sign,
}

impl<const N: usize> BIint<N> {
    fn bit_op<C, G>(self, rhs: Self, closure: C, get_sign: G) -> Self where
        C: Fn(BUint<N>, BUint<N>) -> BUint<N>,
        G: Fn(Sign, Sign) -> Sign
    {
        match (self.sign, rhs.sign) {
            (Sign::Zero, _) => Self::zero(),
            (_, Sign::Zero) => Self::zero(),
            _ => {
                let sign = get_sign(self.sign, rhs.sign);
                let a = if self.sign == Sign::Plus {
                    self.uint
                } else {
                    !self.uint + BUint::<N>::one()
                };
                let b = if rhs.sign == Sign::Plus {
                    rhs.uint
                } else {
                    !rhs.uint + BUint::<N>::one()
                };
                let uint = if sign == Sign::Plus {
                    closure(a, b)
                } else {
                    !closure(a, b) + BUint::<N>::one()
                };
                Self {
                    uint,
                    sign: uint.sign_or_zero(sign),
                }
            }
        }
    }
}

use std::cmp::Ordering;

impl<const N: usize> BIint<N> {
    fn add_inner_or(self, rhs: Self) -> Result<Self, &'static str> {
        Ok(if rhs.sign == self.sign {
            Self {
                uint: self.uint.try_add(rhs.uint)?,
                sign: self.sign,
            }
        } else {
            match self.uint.cmp(&rhs.uint) {
                Ordering::Greater => {
                    Self {
                        uint: self.uint.try_sub(rhs.uint)?,
                        sign: Sign::Plus,
                    }
                },
                Ordering::Less => {
                    Self {
                        uint: rhs.uint.try_sub(self.uint)?,
                        sign: Sign::Minus,
                    }
                },
                Ordering::Equal => {
                    Self::zero()
                }
            }
        })
    }
}

use std::fmt::{Debug, Formatter, self};

impl<const N: usize> Debug for BIint<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", "bigiint")
    }
}