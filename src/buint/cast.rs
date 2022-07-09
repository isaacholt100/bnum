use super::BUint;
use crate::cast::CastFrom;
use crate::digit::{self, Digit};
use crate::nightly::{const_fn, impl_const};
use crate::{BInt, ExpType};
use core::mem::MaybeUninit;
use crate::doc;

impl<const N: usize> BUint<N> {
    const_fn! {
        #[inline]
        const fn cast_up<const M: usize>(self, digit: Digit) -> BUint<M> {
            let mut digits = [digit; M];
            let digits_ptr = digits.as_mut_ptr() as *mut Digit;
            let self_ptr = self.digits.as_ptr();
            unsafe {
                self_ptr.copy_to_nonoverlapping(digits_ptr, N);
                BUint::from_digits(digits)
            }
        }
    }
    const_fn! {
        #[inline]
        const fn cast_down<const M: usize>(self) -> BUint<M> {
            let mut digits = MaybeUninit::<[Digit; M]>::uninit();
            let digits_ptr = digits.as_mut_ptr() as *mut Digit;
            let self_ptr = self.digits.as_ptr();

            unsafe {
                self_ptr.copy_to_nonoverlapping(digits_ptr, M);
                BUint::from_digits(digits.assume_init())
            }
        }
    }
}

macro_rules! buint_as {
    ($($int: ty), *) => {
        $(impl_const! {
            impl<const N: usize> const CastFrom<BUint<N>> for $int {
				#[must_use = doc::must_use_op!()]
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
		})*
    };
}

buint_as!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// TODO: combine these impls for f32 and f64 into a macro for both

impl<const N: usize> CastFrom<BUint<N>> for f32 {
	#[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(from: BUint<N>) -> Self {
        if from.is_zero() {
            return 0.0;
        }
        let bits = from.bits();
        let mut mant = if BUint::<N>::BITS > u32::BITS {
            if bits < 24 {
                u32::cast_from(from) << (24 - bits)
            } else {
                u32::cast_from(from >> (bits - 24))
            }
        } else if bits < 24 {
            u32::cast_from(from) << (24 - bits)
        } else {
            u32::cast_from(from) >> (bits - 24)
        };
        let mut round_up = true;
        if bits <= 24
            || !from.bit(bits - 25)
            || (mant & 1 == 0 && from.trailing_zeros() == bits - 25)
        {
            round_up = false;
        }
        let mut exp = bits as u32 + 127 - 1;
        if round_up {
            mant += 1;
            if mant.leading_zeros() == 32 - 25 {
                exp += 1;
            }
        }
        if exp > f32::MAX_EXP as u32 + 127 {
            return f32::INFINITY;
        }
        let mant = u32::cast_from(mant);
        f32::from_bits((exp << 23) | (mant & (u32::MAX >> (32 - 23))))
    }
}

impl<const N: usize> CastFrom<BUint<N>> for f64 {
	#[must_use = doc::must_use_op!()]
    #[inline]
    fn cast_from(from: BUint<N>) -> Self {
        if from.is_zero() {
            return 0.0;
        }
        let bits = from.bits();
        let mut mant = if BUint::<N>::BITS > u64::BITS {
            if bits < 53 {
                u64::cast_from(from) << (53 - bits)
            } else {
                u64::cast_from(from >> (bits - 53))
            }
        } else if bits < 53 {
            u64::cast_from(from) << (53 - bits)
        } else {
            u64::cast_from(from) >> (bits - 53)
        };
        let mut round_up = true;
        if bits <= 53
            || !from.bit(bits - 54)
            || (mant & 1 == 0 && from.trailing_zeros() == bits - 54)
        {
            round_up = false;
        }
        let mut exp = bits as u64 + 1023 - 1;
        if round_up {
            mant += 1;
            if mant.leading_zeros() == 64 - 54 {
                exp += 1;
            }
        }
        if exp > f64::MAX_EXP as u64 + 1023 {
            return f64::INFINITY;
        }
        let mant = u64::cast_from(mant);
        f64::from_bits((exp << 52) | (mant & (u64::MAX >> (64 - 52))))
    }
}

macro_rules! as_buint {
    ($($ty: ty), *) => {
        $(impl_const! {
            impl<const N: usize> const CastFrom<$ty> for BUint<N> {
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
		})*
    };
}

as_buint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl_const! {
    impl<const N: usize> const CastFrom<bool> for BUint<N> {
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
}

impl_const! {
    impl<const N: usize> const CastFrom<char> for BUint<N> {
		#[must_use = doc::must_use_op!()]
        #[inline]
        fn cast_from(from: char) -> Self {
            Self::cast_from(from as u32)
        }
    }
}

impl_const! {
    impl<const N: usize, const M: usize> const CastFrom<BUint<M>> for BUint<N> {
		#[must_use = doc::must_use_op!()]
        #[inline]
        fn cast_from(from: BUint<M>) -> Self {
            if M < N {
                from.cast_up(0)
            } else {
                from.cast_down()
            }
        }
    }
}

impl_const! {
    impl<const N: usize, const M: usize> const CastFrom<BInt<M>> for BUint<N> {
		#[must_use = doc::must_use_op!()]
        #[inline]
        fn cast_from(from: BInt<M>) -> Self {
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
}

pub const fn u32_bits(u: u32) -> ExpType {
    32 - u.leading_zeros() as ExpType
}

pub const fn u64_bits(u: u64) -> ExpType {
    64 - u.leading_zeros() as ExpType
}

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

macro_rules! cast_from_float {
    ($f: ty, $exp_type: ty, $decoder: expr, $mant_bits: expr) => {
		#[must_use = doc::must_use_op!()]
        #[inline]
        fn cast_from(from: $f) -> Self {
            if from.is_nan() {
                return Self::ZERO;
            }
            if from.is_sign_negative() {
                return Self::MIN;
            }
            if from.is_infinite() {
                return Self::MAX;
            }
            let (mut mant, exp) = $decoder(from);
            if exp.is_negative() {
                mant = mant.checked_shr((-exp) as $exp_type).unwrap_or(0);
                if $mant_bits(mant) > Self::BITS {
                    return Self::MAX;
                }
                Self::cast_from(mant)
            } else {
                if $mant_bits(mant) + exp as ExpType > Self::BITS {
                    return Self::MAX;
                }
                Self::cast_from(mant) << exp
            }
        }
    };
}

impl<const N: usize> CastFrom<f32> for BUint<N> {
    cast_from_float!(f32, u32, decode_f32, u32_bits);
}

impl<const N: usize> CastFrom<f64> for BUint<N> {
    cast_from_float!(f64, u32, decode_f64, u64_bits);
}

crate::int::cast::tests!(utest);
