use super::BIint;
use crate::sign::Sign;
use crate::uint::BUint;
use core::convert::TryFrom;
use num_traits::{Signed, ToPrimitive};
use crate::TryFromIntError;

impl<I, const N: usize> From<I> for BIint<N> where I: Into<BUint<N>> {
    fn from(i: I) -> Self {
        let uint = i.into();
        Self {
            uint,
            sign: uint.sign_or_zero(Sign::Plus),
        }
    }
}

impl<const N: usize> TryFrom<BIint<N>> for BUint<N> {
    type Error = TryFromIntError;

    fn try_from(int: BIint<N>) -> Result<Self, Self::Error> {
        if int.is_negative() {
            Err("Can't convert negative BIint to BUint")
        } else {
            Ok(int.uint)
        }
    }
}

macro_rules! from_iint {
    {
        from: $int: tt,
        as: $as_type: tt
    } => {
        impl<const N: usize> From<$int> for BIint<N> {
            fn from(int: $int) -> Self {
                let uint = if int == $int::MIN {
                    $int::MAX as $as_type + 1
                } else {
                    int.abs() as $as_type
                };
                Self {
                    uint: uint.into(),
                    sign: match int {
                        0 => Sign::Zero,
                        _ => {
                            if int > 0 {
                                Sign::Plus
                            } else {
                                Sign::Minus
                            }
                        }
                    },
                }
            }
        }
    }
}

from_iint! {
    from: i8,
    as: u8
}
from_iint! {
    from: i16,
    as: u16
}
from_iint! {
    from: i32,
    as: u32
}
from_iint! {
    from: isize,
    as: usize
}
from_iint! {
    from: i64,
    as: u64
}
from_iint! {
    from: i128,
    as: u128
}

impl<const N: usize> TryFrom<BIint<N>> for u128 {
    type Error = TryFromIntError;

    fn try_from(iint: BIint<N>) -> Result<Self, Self::Error> {
        if iint.is_negative() {
            return Err("Can't convert negative bigint to u128");
        }
        iint.uint.to_u128().ok_or("bigint is too large to convert to u128")
    }
}

impl<const N: usize> TryFrom<BIint<N>> for u64 {
    type Error = TryFromIntError;

    fn try_from(iint: BIint<N>) -> Result<Self, Self::Error> {
        if iint.is_negative() {
            return Err("Can't convert negative bigint to u64");
        }
        iint.uint.to_u64().ok_or("bigint is too large to convert to u64")
    }
}

macro_rules! to_int {
    ($to: tt, $method: ident, $err: expr) => {
        impl<const N: usize> TryFrom<BIint<N>> for $to {
            type Error = TryFromIntError;

            fn try_from(iint: BIint<N>) -> Result<Self, Self::Error> {
                iint.$method().ok_or($err)
            }
        }
    }
}

to_int!(usize, to_usize, "BIint is too large to cast to usize");
to_int!(u32, to_u32, "BIint is too large to cast to u32");
to_int!(u16, to_u16, "BIint is too large to cast to u16");
to_int!(u8, to_u8, "BIint is too large to cast to u8");

to_int!(i128, to_i128, "BIint is too large to cast to i128");
to_int!(i64, to_i64, "BIint is too large to cast to i64");
to_int!(isize, to_isize, "BIint is too large to cast to isize");
to_int!(i32, to_i32, "BIint is too large to cast to i32");
to_int!(i16, to_i16, "BIint is too large to cast to i16");
to_int!(i8, to_i8, "BIint is too large to cast to i8");

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;
    use num_traits::One;

    #[test]
    fn it_converts_u8() {
        let u = 23u8;
        let a = BIint::<2>::from(u);
        let into: u8 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(u8::try_from(-BIint::<2>::one()).is_err());
    }

    #[test]
    fn it_converts_u16() {
        let u = 23463u16;
        let a = BIint::<2>::from(u);
        let into: u16 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(u16::try_from(-BIint::<2>::one()).is_err());
    }

    #[test]
    fn it_converts_u32() {
        let u = 47839744u32;
        let a = BIint::<2>::from(u);
        let into: u32 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(u32::try_from(-BIint::<2>::one()).is_err());
    }

    #[test]
    fn it_converts_usize() {
        let u = 34534054usize;
        let a = BIint::<2>::from(u);
        let into: usize = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(usize::try_from(-BIint::<2>::one()).is_err());
    }

    #[test]
    fn it_converts_u64() {
        let u = 3453455986584534435u64;
        let a = BIint::<2>::from(u);
        let into: u64 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(u64::try_from(-BIint::<2>::one()).is_err());
    }

    #[test]
    fn it_converts_u128() {
        let u = 3845345083490573945798643453455478697u128;
        let a = BIint::<2>::from(u);
        let into: u128 = a.try_into().unwrap();
        assert_eq!(into, u);
        assert!(u128::try_from(-BIint::<2>::one()).is_err());
    }

    #[test]
    fn it_converts_i8() {
        let u = -122i8;
        let a = BIint::<2>::try_from(u).unwrap();
        let into: i8 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_i16() {
        let u = -23423i16;
        let a = BIint::<2>::try_from(u).unwrap();
        let into: i16 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_i32() {
        let u = -3495734i32;
        let a = BIint::<2>::try_from(u).unwrap();
        let into: i32 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_isize() {
        let u = -9874934isize;
        let a = BIint::<2>::try_from(u).unwrap();
        let into: isize = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_i64() {
        let u = -3495793490095843i64;
        let a = BIint::<2>::try_from(u).unwrap();
        let into: i64 = a.try_into().unwrap();
        assert_eq!(into, u);
    }

    #[test]
    fn it_converts_i128() {
        let u = -9758947634598734853948720108238435i128;
        let a = BIint::<2>::try_from(u).unwrap();
        let into: i128 = a.try_into().unwrap();
        assert_eq!(into, u);
    }
}