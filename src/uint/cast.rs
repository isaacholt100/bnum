use super::BUint;
use crate::digit::{self, Digit};
use crate::{Bint, ExpType};
use core::mem::MaybeUninit;

macro_rules! as_int {
    ($method: ident, $int: ty, $type_str: expr, $assertion: expr) => {
        /// Casts `self` to 
        #[doc=$type_str]
        /// # Examples
        /// 
        /// ```
        /// use bint::BUint;
        /// 
        /// let n = 1097937598374598734507959845u128;
        /// let u = BUint::<2>::from(n);
        #[doc=$assertion]
        /// ```
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
    as_int!(as_u8, u8, "a `u8`.", "assert_eq!(u.as_u8(), n as u8);");
    as_int!(as_u16, u16, "a `u16`.", "assert_eq!(u.as_u16(), n as u16);");
    as_int!(as_u32, u32, "a `u32`.", "assert_eq!(u.as_u32(), n as u32);");
    as_int!(as_u64, u64, "a `u64`.", "assert_eq!(u.as_u64(), n as u64);");
    as_int!(as_u128, u128, "a `u128`.", "assert_eq!(u.as_u128(), n as u128);");
    as_int!(as_usize, usize, "a `usize`.", "assert_eq!(u.as_usize(), n as usize);");

    as_int!(as_i8, i8, "an `i8`.", "assert_eq!(u.as_i8(), n as i8);");
    as_int!(as_i16, i16, "an `i16`.", "assert_eq!(u.as_i16(), n as i16);");
    as_int!(as_i32, i32, "an `i32`.", "assert_eq!(u.as_i32(), n as i32);");
    as_int!(as_i64, i64, "an `i64`.", "assert_eq!(u.as_i64(), n as i64);");
    as_int!(as_i128, i128, "an `i128`.", "assert_eq!(u.as_i128(), n as i128);");
    as_int!(as_isize, isize, "an `isize`.", "assert_eq!(u.as_isize(), n as isize);");

    /// Converts `self` to an `f32` floating point number. 
    /// 
    /// If `self` is larger than the largest integer that can be represented by an `f32`, `f32::INFINITY` is returned.
    #[doc=crate::doc::example_header!(BUint)]
    /// let n = 1097937598374598734507959845u128;
    /// let u = BUint::<2>::from(n);
    /// assert_eq!(u.as_f32(), n as f32);
    /// ```
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
    pub fn as_f64(&self) -> f64 {
        let mantissa = self.to_mantissa();
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

    #[cfg(feature = "nightly")]
    pub const fn as_buint<const M: usize>(&self) -> BUint<M> where [Digit; M.saturating_sub(N)]: Sized {
        if M > N {
            cast_up::<N, M>(&self, 0)
        } else {
            cast_down::<N, M>(&self)
        }
    }

    #[cfg(feature = "nightly")]
    pub const fn as_biint<const M: usize>(&self) -> Bint<M> where [Digit; M.saturating_sub(N)]: Sized {
        Bint::from_bits(self.as_buint())
    }
}

#[cfg(feature = "nightly")]
pub const fn cast_up<const N: usize, const M: usize>(u: &BUint<N>, digit: Digit) -> BUint<M> where [Digit; M.saturating_sub(N)]: Sized {
    debug_assert!(M > N);
    let mut digits = MaybeUninit::<[Digit; M]>::uninit();
    let digits_ptr = digits.as_mut_ptr() as *mut Digit;
    let self_ptr = u.digits.as_ptr();
    let padding = [digit; M.saturating_sub(N)];
    let padding_ptr = padding.as_ptr();

    unsafe {
        self_ptr.copy_to_nonoverlapping(digits_ptr, N);
        padding_ptr.copy_to_nonoverlapping(digits_ptr.offset(N as isize), M.saturating_sub(N));
        BUint {
            digits: digits.assume_init(),
        }
    }
}

#[cfg(feature = "nightly")]
pub const fn cast_down<const N: usize, const M: usize>(u: &BUint<N>) -> BUint<M> {
    let mut digits = MaybeUninit::<[Digit; M]>::uninit();
    let digits_ptr = digits.as_mut_ptr() as *mut Digit;
    let self_ptr = u.digits.as_ptr();

    unsafe {
        self_ptr.copy_to_nonoverlapping(digits_ptr, M);
        BUint {
            digits: digits.assume_init(),
        }
    }
}

const fn last_set_bit(n: u64) -> u8 {
    64 - n.leading_zeros() as u8
}

#[cfg(test)]
mod tests {
    use crate::{U128, BUint, I128, digit};

    #[test]
    fn as_u8() {
        let u = 458937495794835975u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u8(), u as u8);
    }
    #[test]
    fn as_u16() {
        let u = 45679457045646u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u16(), u as u16);
    }
    #[test]
    fn as_u32() {
        let u = 9475697398457690379876u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u32(), u as u32);
    }
    #[test]
    fn as_u64() {
        let u = 987927348957345930475972439857u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u64(), u as u64);
    }
    #[test]
    fn as_u128() {
        let u = 49576947589673498576905868576485690u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u128(), u as u128);
    }
    #[test]
    fn as_usize() {
        let u = 309485608560934564564568456u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_usize(), u as usize);
    }

    #[test]
    fn as_i8() {
        let u = 456759876u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i8(), u as i8);
    }
    #[test]
    fn as_i16() {
        let u = 9458769456904856u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i16(), u as i16);
    }
    #[test]
    fn as_i32() {
        let u = 95792684579875345u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i32(), u as i32);
    }
    #[test]
    fn as_i64() {
        let u = 4586745698783453459756456u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i64(), u as i64);
    }
    #[test]
    fn as_i128() {
        let u = 232030679846578450968409568098465u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i128(), u as i128);
    }
    #[test]
    fn as_isize() {
        let u = 4568094586492858767245068445987u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_isize(), u as isize);
    }

    #[test]
    fn as_f32() {
        let u = 35478975973468456798569u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_f32(), u as f32);
    }
    #[test]
    fn as_f64() {
        let u = 896286490745687459674865u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_f64(), u as f64);
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn as_buint() {
        let u = 93457394573495790u64;
        let uint = BUint::<{64 / digit::BITS}>::from(u);
        assert_eq!(U128::from(u), uint.as_buint::<{128 / digit::BITS}>());
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn as_biint() {
        let u = 204958679794567895u64;
        let uint = BUint::<{64 / digit::BITS}>::from(u);
        assert_eq!(I128::from(u), uint.as_biint::<{128 / digit::BITS}>());
    }
}