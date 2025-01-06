use super::BUintD8;
use crate::{digit, Digit};

macro_rules! decode_float {
    ($name: ident, $f: ty, $u: ty) => {
        pub fn $name(f: $f) -> ($u, i16) {
            const BITS: u32 = core::mem::size_of::<$f>() as u32 * 8;
            const MANT_MASK: $u = <$u>::MAX >> (BITS - (<$f>::MANTISSA_DIGITS - 1));
            const EXP_MASK: $u = <$u>::MAX >> 1;
            const BIAS: i16 = <$f>::MAX_EXP as i16 - 1;

            let bits = f.to_bits();
            let exp = ((bits & EXP_MASK) >> (<$f>::MANTISSA_DIGITS - 1)) as i16;
            let mut mant = bits & MANT_MASK;
            if exp != 0 {
                mant |= (1 << (<$f>::MANTISSA_DIGITS - 1));
            }
            (mant, exp - (BIAS + <$f>::MANTISSA_DIGITS as i16 - 1))
        }
    };
}

decode_float!(decode_f32, f32, u32);
decode_float!(decode_f64, f64, u64);

macro_rules! buint_as_int {
    ($($int: ty), *) => {
        $(
            impl<const N: usize> CastFrom<BUintD8<N>> for $int {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: BUintD8<N>) -> Self {
                    let mut out = 0;
                    let mut i = 0;
                    while i << crate::digit::BIT_SHIFT < <$int>::BITS as usize && i < N {
                        out |= from.digits[i] as $int << (i << crate::digit::BIT_SHIFT);
                        i += 1;
                    }
                    out
                }
            }
        )*
    };
}

macro_rules! buint_as_float {
    ($f: ty) => {
        impl<const N: usize> CastFrom<BUintD8<N>> for $f {
            #[must_use = doc::must_use_op!()]
            #[inline]
            fn cast_from(value: BUintD8<N>) -> Self {
                crate::cast::float::cast_float_from_uint(value)
            }
        }
    };
}

macro_rules! as_buint {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> CastFrom<$ty> for BUintD8<N> {
                #[must_use = doc::must_use_op!()]
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
                        if <$ty>::BITS <= Digit::BITS {
                            from = 0;
                        } else {
                            from = from.wrapping_shr(Digit::BITS);
                        }
                        i += 1;
                    }
                    out
                }
            }
        )*
    };
}

#[allow(unused_imports)]
use crate::cast::float::{CastFloatFromUintHelper, CastUintFromFloatHelper, FloatMantissa};
use crate::cast::CastFrom;
use crate::doc;
use crate::ExpType;

// #[cfg(feature = "float")]
// impl<const N: usize> FloatMantissa for BUintD8<N> {
//     const ZERO: Self = Self::ZERO;
//     const ONE: Self = Self::ONE;
//     const TWO: Self = Self::TWO;
//     const MAX: Self = Self::MAX;

//     #[inline]
//     fn leading_zeros(self) -> ExpType {
//         Self::leading_zeros(self)
//     }

//     #[inline]
//     fn checked_shr(self, n: ExpType) -> Option<Self> {
//         Self::checked_shr(self, n)
//     }

//     #[inline]
//     fn is_power_of_two(self) -> bool {
//         Self::is_power_of_two(self)
//     }
// }

impl<const N: usize> CastUintFromFloatHelper for BUintD8<N> {
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}

impl<const N: usize> CastFloatFromUintHelper for BUintD8<N> {
    fn trailing_zeros(self) -> ExpType {
        Self::trailing_zeros(self)
    }
}

impl<const N: usize> BUintD8<N> {
    #[inline]
    const fn cast_up<const M: usize>(self, digit: Digit) -> BUintD8<M> {
        let mut digits = [digit; M];
        let mut i = M - N;
        while i < M {
            let index = i - (M - N);
            digits[index] = self.digits[index];
            i += 1;
        }
        BUintD8::from_digits(digits)
    }

    #[inline]
    const fn cast_down<const M: usize>(self) -> BUintD8<M> {
        let mut out = BUintD8::ZERO;
        let mut i = 0;
        while i < M {
            out.digits[i] = self.digits[i];
            i += 1;
        }
        out
    }
}

buint_as_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

buint_as_float!(f32);
buint_as_float!(f64);

as_buint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<const N: usize> CastFrom<bool> for BUintD8<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(from: bool) -> Self {
        if from {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

impl<const N: usize> CastFrom<char> for BUintD8<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(from: char) -> Self {
        Self::cast_from(from as u32)
    }
}

impl<const N: usize, const M: usize> CastFrom<BUintD8<M>> for BUintD8<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(from: BUintD8<M>) -> Self {
        if M < N {
            from.cast_up(0)
        } else {
            from.cast_down()
        }
    }
}

impl<const N: usize, const M: usize> CastFrom<crate::BIntD8<M>> for BUintD8<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(from: crate::BIntD8<M>) -> Self {
        if M < N {
            let padding_digit = if from.is_negative() { Digit::MAX } else { 0 };
            from.to_bits().cast_up(padding_digit)
        } else {
            from.to_bits().cast_down()
        }
    }
}

impl<const N: usize> CastFrom<f32> for BUintD8<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(value: f32) -> Self {
        crate::cast::float::cast_uint_from_float(value)
    }
}

impl<const N: usize> CastFrom<f64> for BUintD8<N> {
    #[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(value: f64) -> Self {
        crate::cast::float::cast_uint_from_float(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::test::types::*;

    crate::int::cast::tests!(utest);
}

macro_rules! buint_as_different_digit_bigint {
    (BUintD8: ident, BIntD8: ident, Digit: ident; $(($OtherBUint: ident, $OtherDigit: ident)), *) => {
        $(
            impl<const N: usize, const M: usize> crate::cast::CastFrom<$OtherBUint<M>> for BUintD8<N> {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: $OtherBUint<M>) -> Self {
                    let mut out = Self::ZERO;
                    if Digit::BITS < $OtherDigit::BITS {
                        const DIVIDE_COUNT: usize = ($OtherDigit::BITS / Digit::BITS) as usize;
                        let stop_index: usize = if <$OtherBUint<M>>::BITS > <BUintD8<N>>::BITS {
                            N
                        } else {
                            M * DIVIDE_COUNT
                        };
                        let mut i = 0;
                        while i < stop_index {
                            let wider_digit = from.digits[i / DIVIDE_COUNT];
                            let mini_shift = i % DIVIDE_COUNT;
                            let digit = (wider_digit >> (mini_shift << digit::BIT_SHIFT)) as Digit;
                            out.digits[i] = digit;
                            i += 1;
                        }
                    } else {
                        const DIVIDE_COUNT: usize = (Digit::BITS / $OtherDigit::BITS) as usize;
                        let stop_index: usize = if <$OtherBUint<M>>::BITS > <BUintD8<N>>::BITS {
                            N * DIVIDE_COUNT
                        } else {
                            M
                        };
                        let mut current_digit: Digit = 0;
                        let mut i = 0;
                        while i < stop_index {
                            let mini_shift = i % DIVIDE_COUNT;
                            current_digit |= (from.digits[i] as Digit) << (mini_shift << digit::$OtherDigit::BIT_SHIFT);
                            if mini_shift == DIVIDE_COUNT - 1 || i == stop_index - 1 {
                                out.digits[i / DIVIDE_COUNT] = current_digit;
                                current_digit = 0;
                            }
                            i += 1;
                        }
                    }
                    out
                }
            }

            impl<const N: usize, const M: usize> crate::cast::CastFrom<$OtherBUint<M>> for BIntD8<N> {
                #[must_use = doc::must_use_op!()]
                #[inline]
                fn cast_from(from: $OtherBUint<M>) -> Self {
                    Self::from_bits(BUintD8::cast_from(from))
                }
            }
        )*
    }
}

pub(crate) use buint_as_different_digit_bigint;
