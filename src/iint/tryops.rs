use super::BIint;
use crate::sign::Sign;
use crate::tryops::TryOps;
use crate::uint::BUint;
use num_traits::{One, Zero};

impl<const N: usize> TryOps for BIint<N> {
    type Error = &'static str;

    fn try_add(self, rhs: Self) -> Result<Self, Self::Error> {
        if rhs.sign == Sign::Zero {
            return Ok(self);
        }
        match self.sign {
            Sign::Zero => Ok(rhs),
            Sign::Plus => {
                self.add_inner_or(rhs)
            },
            Sign::Minus => {
                rhs.add_inner_or(self)
            }
        }
    }
    fn try_div(self, rhs: Self) -> Result<Self, Self::Error> {
        if rhs.sign == Sign::Zero {
            Err("Can't divide by zero")
        } else {
            Ok(Self {
                uint: self.uint.try_div(rhs.uint)?,
                sign: self.sign.combine(rhs.sign),
            })
        }
    }
    fn try_mul(self, rhs: Self) -> Result<Self, Self::Error> {
        Ok(Self {
            uint: self.uint.try_mul(rhs.uint)?,
            sign: self.sign.combine(rhs.sign),
        })
    }
    fn try_pow(self, exp: u32) -> Result<Self, Self::Error> {
        Err("test error")
    }
    fn try_rem(self, rhs: Self) -> Result<Self, Self::Error> {
        if rhs.sign == Sign::Zero {
            Err("can't do modulo 0")
        } else {
            Ok(match self.sign {
                Sign::Zero => Self::zero(),
                sign => {
                    let uint = self.uint.try_rem(rhs.uint)?;
                    Self {
                        uint,
                        sign: uint.sign_or_zero(sign),
                    }
                }
            })
        }
    }
    fn try_shl(self, rhs: u32) -> Result<Self, Self::Error> {
        /*match rhs.sign {
            Sign::Zero => Ok(self),
            Sign::Minus => Err("Can't left shift by negative number"),
            Sign::Plus => Ok(match self.sign {
                Sign::Zero => Self::zero(),
                sign => Self {
                    uint: self.uint.try_shl(rhs.uint)?,
                    sign,
                }
            }),
        }*/
        Ok(match self.sign {
            Sign::Zero => Self::zero(),
            sign => Self {
                uint: self.uint.try_shl(rhs)?,
                sign,
            }
        })
    }
    fn try_shr(self, rhs: u32) -> Result<Self, Self::Error> {
        /*match rhs.sign {
            Sign::Zero => Ok(self),
            Sign::Minus => Err("Can't right shift by negative number"),
            Sign::Plus => Ok(match self.sign {
                Sign::Zero => Self::zero(),
                Sign::Plus => {
                    let uint = self.uint.try_shr(rhs.uint)?;
                    Self {
                        uint,
                        sign: uint.sign_or_zero(Sign::Plus),
                    }
                },
                Sign::Minus => {
                    let uint = self.uint >> rhs.uint;
                    if uint.is_zero() {
                        Self {
                            uint: BUint::one(),
                            sign: Sign::Minus,
                        }
                    } else {
                        Self {
                            uint,
                            sign: self.sign,
                        }
                    }
                }
            }),
        }*/
        Ok(match self.sign {
            Sign::Zero => Self::zero(),
            Sign::Plus => {
                let uint = self.uint.try_shr(rhs)?;
                Self {
                    uint,
                    sign: uint.sign_or_zero(Sign::Plus),
                }
            },
            Sign::Minus => {
                let uint = self.uint >> rhs;
                if uint.is_zero() {
                    Self {
                        uint: BUint::one(),
                        sign: Sign::Minus,
                    }
                } else {
                    Self {
                        uint,
                        sign: self.sign,
                    }
                }
            }
        })
    }
    fn try_sub(self, rhs: Self) -> Result<Self, Self::Error> {
        self.try_add(-rhs)
    }
}