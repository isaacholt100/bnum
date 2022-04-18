use super::BUint;
use crate::digit::{self, Digit};
use crate::{Bint, ExpType};
use crate::cast::CastFrom;
use core::mem::MaybeUninit;

// TODO: implement as_float (cast to big float type), implement primitive types as_buint, as_bint, as_float

impl<const N: usize> BUint<N> {
    #[inline]
    const fn cast_up<const M: usize>(&self, digit: Digit) -> BUint<M> {
        let mut digits = [digit; M];
        let digits_ptr = digits.as_mut_ptr() as *mut Digit;
        let self_ptr = self.digits.as_ptr();
        unsafe {
            self_ptr.copy_to_nonoverlapping(digits_ptr, N);
            BUint::from_digits(digits)
        }
    }

    #[inline]
    const fn cast_down<const M: usize>(&self) -> BUint<M> {
        let mut digits = MaybeUninit::<[Digit; M]>::uninit();
        let digits_ptr = digits.as_mut_ptr() as *mut Digit;
        let self_ptr = self.digits.as_ptr();

        unsafe {
            self_ptr.copy_to_nonoverlapping(digits_ptr, M);
            BUint::from_digits(digits.assume_init())
        }
    }
}

#[inline]
const fn last_set_bit(n: u64) -> u8 {
    64 - n.leading_zeros() as u8
}

macro_rules! buint_as {
    ($($int: ty), *) => {
        $(
            impl<const N: usize> const CastFrom<BUint<N>> for $int {
                #[inline]
                fn cast_from(from: BUint<N>) -> Self {
                    let mut out = 0;
                    let mut i = 0;
                    while i << digit::BIT_SHIFT < <$int>::BITS as usize && i < N {
                        out |= from.digits[i] as $int << (i << digit::BIT_SHIFT);
                        i += 1;
                    }
                    out
                }
            }
        )*
    };
}

buint_as!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const N: usize> CastFrom<BUint<N>> for f32 {
    #[inline]
    fn cast_from(from: BUint<N>) -> Self {
        let mantissa = from.to_mantissa();
        let exp = from.bits() - last_set_bit(mantissa) as ExpType;
        if exp > f32::MAX_EXP as ExpType {
            f32::INFINITY
        } else {
            let f = mantissa as f32;
            let mut u = f.to_bits();
            u += (exp as u32) << 23;
            if u >> 31 == 1 {
                f32::INFINITY
            } else {
                f32::from_bits(u)
            }
            //(mantissa as f32) * 2f32.powi(exp as i32)
        }
    }
}

impl<const N: usize> CastFrom<BUint<N>> for f64 {
    #[inline]
    fn cast_from(from: BUint<N>) -> Self {
        let mantissa = from.to_mantissa();
        let exp = from.bits() - last_set_bit(mantissa) as ExpType;

        if exp > f64::MAX_EXP as ExpType {
            f64::INFINITY
        } else {
            let f = mantissa as f64;
            let mut u = f.to_bits();
            u += (exp as u64) << 52;
            if u >> 63 == 1 {
                f64::INFINITY
            } else {
                f64::from_bits(u)
            }
            //(mantissa as f64) * 2f64.powi(exp as i32)
        }
    }
}

macro_rules! as_buint {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> const CastFrom<$ty> for BUint<N> {
                #[inline]
                fn cast_from(mut from: $ty) -> Self {
                    #[allow(unused_comparisons)]
                    let mut out = if from < 0 {
                        Self::MAX
                    } else {
                        Self::MIN
                    };
                    let mut i = 0;
                    while from != 0 && i < N {
                        let masked = from as Digit & Digit::MAX;
                        out.digits[i] = masked;
                        if <$ty>::BITS <= digit::BITS as u32 {
                            from = 0;
                        } else {
                            from = from.wrapping_shr(digit::BITS as u32);
                        }
                        i += 1;
                    }
                    out
                }
            }
        )*
    };
}

as_buint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const N: usize> const CastFrom<bool> for BUint<N> {
    #[inline]
    fn cast_from(from: bool) -> Self {
        if from {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

impl<const N: usize> const CastFrom<char> for BUint<N> {
    #[inline]
    fn cast_from(from: char) -> Self {
        Self::cast_from(from as u32)
    }
}

impl<const N: usize, const M: usize> const CastFrom<BUint<M>> for BUint<N> {
    #[inline]
    fn cast_from(from: BUint<M>) -> Self {
        if M < N {
            from.cast_up(0)
        } else {
            from.cast_down()
        }
    }
}

impl<const N: usize, const M: usize> const CastFrom<Bint<M>> for BUint<N> {
    #[inline]
    fn cast_from(from: Bint<M>) -> Self {
        if M < N {
            let padding_digit = if from.is_negative() {
                Digit::MAX
            } else {
                0
            };
            from.to_bits().cast_up(padding_digit)
        } else {
            from.to_bits().cast_down()
        }
    }
}

use core::convert::TryFrom;

// TODO: tests are failing, fix
impl<const N: usize> CastFrom<f32> for BUint<N> {
    #[inline]
    fn cast_from(from: f32) -> Self {
        match Self::try_from(from) {
            Ok(u) => u,
            Err(err) => match err.reason {
                crate::TryFromErrorReason::Negative => Self::ZERO,
                _ => Self::MAX,
            }
        }
    }
}

impl<const N: usize> CastFrom<f64> for BUint<N> {
    #[inline]
    fn cast_from(from: f64) -> Self {
        match Self::try_from(from) {
            Ok(u) => u,
            Err(err) => match err.reason {
                crate::TryFromErrorReason::Negative => Self::ZERO,
                _ => Self::MAX,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{U128, I128, U64, I64, test};
    use crate::cast::As;

    test::test_cast_to!([u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, /*f32, f64,*/ bool, char] as U128);

    test::test_cast_to!([u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char] as U64);

    test::test_cast_from!(U64 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32/*, f64*/, U64, I64, I128, U128]);

    test::test_cast_from!(U128 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32/*, f64*/, U64, I64, I128, U128]);

    //#[test]
    fn test_f64() {
        let i = 26971620319773403137u128;
        let big = U128::from(i);
        assert_eq!(i as f64, big.as_());
    }
}