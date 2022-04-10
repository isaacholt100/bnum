use super::BUint;
use crate::digit::{self, Digit};
use crate::{Bint, ExpType};
use crate::cast::CastFrom;
use core::mem::MaybeUninit;

macro_rules! as_int {
    ($method: ident, $int: ty, $assertion: expr) => {
        /// Casts `self` to 
        #[doc=concat!("a `", stringify!($int),"`.")]
        /// # Examples
        /// 
        /// ```
        /// use bint::BUint;
        /// 
        /// let n = 1097937598374598734507959845u128;
        /// let u = BUint::<2>::from(n);
        #[doc=$assertion]
        /// ```
        #[inline]
        pub const fn $method(&self) -> $int {
            let mut out = 0;
            let mut i = 0;
            while i << digit::BIT_SHIFT < <$int>::BITS as usize && i < N {
                out |= self.digits[i] as $int << (i << digit::BIT_SHIFT);
                i += 1;
            }
            out
        }
    };
}

/// Panic free casts to primitive numeric types.
impl<const N: usize> BUint<N> {
    as_int!(as_u8, u8, "assert_eq!(u.as_u8(), n as u8);");
    as_int!(as_u16, u16, "assert_eq!(u.as_u16(), n as u16);");
    as_int!(as_u32, u32, "assert_eq!(u.as_u32(), n as u32);");
    as_int!(as_u64, u64, "assert_eq!(u.as_u64(), n as u64);");
    as_int!(as_u128, u128, "assert_eq!(u.as_u128(), n as u128);");
    as_int!(as_usize, usize, "assert_eq!(u.as_usize(), n as usize);");

    as_int!(as_i8, i8, "assert_eq!(u.as_i8(), n as i8);");
    as_int!(as_i16, i16, "assert_eq!(u.as_i16(), n as i16);");
    as_int!(as_i32, i32, "assert_eq!(u.as_i32(), n as i32);");
    as_int!(as_i64, i64, "assert_eq!(u.as_i64(), n as i64);");
    as_int!(as_i128, i128, "assert_eq!(u.as_i128(), n as i128);");
    as_int!(as_isize, isize, "assert_eq!(u.as_isize(), n as isize);");

    /// Converts `self` to an `f32` floating point number. 
    /// 
    /// If `self` is larger than the largest integer that can be represented by an `f32`, `f32::INFINITY` is returned.
    #[doc=crate::doc::example_header!(BUint)]
    /// let n = 1097937598374598734507959845u128;
    /// let u = BUint::<2>::from(n);
    /// assert_eq!(u.as_f32(), n as f32);
    /// ```
    #[inline]
    pub fn as_f32(&self) -> f32 {
        let mantissa = self.to_mantissa();
        let exp = self.bits() - last_set_bit(mantissa) as ExpType;
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

    /// Converts `self` to an `f64` floating point number. 
    /// 
    /// If `self` is larger than the largest number that can be represented by an `f64`, `f64::INFINITY` is returned.
    #[doc=crate::doc::example_header!(BUint)]
    /// let n = 1097937598374598734507959845u128;
    /// let u = BUint::<2>::from(n);
    /// assert_eq!(u.as_f64(), n as f64);
    /// ```
    #[inline]
    pub fn as_f64(&self) -> f64 { // TODO: test for this is failing, fix
        let mantissa = self.to_mantissa();
        println!("{}", mantissa);
        let exp = self.bits() - last_set_bit(mantissa) as ExpType;

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

    #[inline]
    pub const fn as_buint<const M: usize>(&self) -> BUint<M> {
        if M > N {
            cast_up::<N, M>(&self, 0)
        } else {
            cast_down::<N, M>(&self)
        }
    }

    #[inline]
    pub const fn as_bint<const M: usize>(&self) -> Bint<M> {
        Bint::from_bits(self.as_buint())
    }
}

// TODO: implement as_float (cast to big float type), implement primitive types as_buint, as_bint, as_float
// TODO: consider switching all methods to an As<T> trait, possibly provided by another library

#[inline]
/*pub const fn cast_up<const N: usize, const M: usize>(u: &BUint<N>, digit: Digit) -> BUint<M> where [Digit; M.saturating_sub(N)]: Sized {
    debug_assert!(M > N);
    let mut digits = MaybeUninit::<[Digit; M]>::uninit();
    let digits_ptr = digits.as_mut_ptr() as *mut Digit;
    let self_ptr = u.digits.as_ptr();
    let padding = [digit; M.saturating_sub(N)];
    let padding_ptr = padding.as_ptr();

    unsafe {
        self_ptr.copy_to_nonoverlapping(digits_ptr, N);
        padding_ptr.copy_to_nonoverlapping(digits_ptr.offset(N as isize), M.saturating_sub(N));
        BUint::from_digits(digits.assume_init())
    }
}*/
pub const fn cast_up<const N: usize, const M: usize>(u: &BUint<N>, digit: Digit) -> BUint<M> {
    let mut digits = [digit; M];
    let digits_ptr = digits.as_mut_ptr() as *mut Digit;
    let u_ptr = u.digits.as_ptr();
    unsafe {
        u_ptr.copy_to_nonoverlapping(digits_ptr, N);
        BUint::from_digits(digits)
    }
}

#[inline]
pub const fn cast_down<const N: usize, const M: usize>(u: &BUint<N>) -> BUint<M> {
    let mut digits = MaybeUninit::<[Digit; M]>::uninit();
    let digits_ptr = digits.as_mut_ptr() as *mut Digit;
    let self_ptr = u.digits.as_ptr();

    unsafe {
        self_ptr.copy_to_nonoverlapping(digits_ptr, M);
        BUint::from_digits(digits.assume_init())
    }
}

#[inline]
const fn last_set_bit(n: u64) -> u8 {
    64 - n.leading_zeros() as u8
}

macro_rules! as_buint {
    ($ty: ty, $unsigned: ty) => {
        impl<const N: usize> const CastFrom<$ty> for BUint<N> {
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
    fn cast_from(from: bool) -> Self {
        if from {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

impl<const N: usize> const CastFrom<char> for BUint<N> {
    fn cast_from(from: char) -> Self {
        Self::cast_from(from as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::{U128, I128, digit, U64, test};
    use crate::cast::As;

    test::test_cast_to!([u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char] as U128);

    test::test_cast_from!(U128 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32/*, f64*/]); // TODO: f64 test is failing, fix

    #[test]
    fn test_f64() {
        let i = 26971620319773403137u128;
        let big = U128::from(i);
        assert_eq!(big.as_f64(), i as f64);
    }

    #[test]
    fn as_buint() {
        let u = 93457394573495790u64;
        let uint = U64::from(u);
        assert_eq!(U128::from(u), uint.as_buint::<{128 / digit::BITS as usize}>());
    }

    #[test]
    fn as_bint() {
        let u = 204958679794567895u64;
        let uint = U64::from(u);
        assert_eq!(I128::from(u), uint.as_bint::<{128 / digit::BITS as usize}>());
    }
}