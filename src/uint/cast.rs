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
    ($ty: ty, $unsigned: ty) => {
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
                    let masked = from as $unsigned as Digit & Digit::MAX;
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
    };
    ($ty: ty) => {
        as_buint!($ty, $ty);
    };
}

as_buint!(u8);
as_buint!(u16);
as_buint!(u32);
as_buint!(u64);
as_buint!(u128);
as_buint!(usize);

as_buint!(i8, u8);
as_buint!(i16, u16);
as_buint!(i32, u32);
as_buint!(i64, u64);
as_buint!(i128, u128);
as_buint!(isize, usize);

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

#[cfg(test)]
mod tests {
    use crate::{U128, I128, U64, test};
    use crate::cast::As;

    test::test_cast_to!([u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char] as U128);

    test::test_cast_from!(U128 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32/*, f64*/]); // TODO: f64 test is failing, fix

    #[test]
    fn test_f64() {
        let i = 26971620319773403137u128;
        let big = U128::from(i);
        assert_eq!(i as f64, big.as_());
    }

    // TODO: quickcheck test as buint, as bint

    #[test]
    fn as_buint() {
        let u = 93457394573495790u64;
        let uint = U64::from(u);
        assert_eq!(U128::from(u), uint.as_());
    }

    #[test]
    fn as_bint() {
        let u = 204958679794567895u64;
        let uint = U64::from(u);
        assert_eq!(I128::from(u), uint.as_());
    }
}