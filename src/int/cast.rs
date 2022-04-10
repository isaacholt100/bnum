use super::Bint;
use crate::uint::BUint;
use crate::uint;
use crate::cast::CastFrom;
use crate::digit::{Digit, self};

macro_rules! as_int {
    ($method: ident, $int: ty, $type_str: expr, $assertion: expr) => {
        /// Casts `self` to 
        #[doc=$type_str]
        /// # Examples
        /// 
        /// ```
        /// use bint::Bint;
        /// 
        /// let n = 1097937598374598734507959845i128;
        /// let u = Bint::<2>::from(n);
        #[doc=$assertion]
        /// ```
        #[inline]
        pub const fn $method(&self) -> $int {
            const ZERO: $int = 0;
            const ONES: $int = ZERO.wrapping_sub(1);
            if self.is_negative() {
                let digits = self.digits();
                let mut out = ONES;
                let mut i = 0;
                while i << digit::BIT_SHIFT < <$int>::BITS as usize && i < N {
                    out &= !((!digits[i]) as $int << (i << digit::BIT_SHIFT));
                    i += 1;
                }
                out
            } else {
                self.bits.$method()
            }
        }
    };
}

impl<const N: usize> Bint<N> {
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
    /// If `self` is larger than the largest integer that can be represented by an `f32`, `f32::INFINITY` is returned. If `self` is smaller than the smallest integer that can be represented by an `f32`, `f32::NEG_INFINITY` is returned.
    #[doc=crate::doc::example_header!(BUint)]
    /// let n = -109793759837u32;
    /// let u = BUint::<4>::from(n);
    /// assert_eq!(u.as_f32(), n as f32);
    /// ```
    #[inline]
    pub fn as_f32(&self) -> f32 {
        let f = self.unsigned_abs().as_f32();
        if self.is_negative() {
            -f
        } else {
            f
        }
    }

    /// Converts `self` to an `f64` floating point number. 
    /// 
    /// If `self` is larger than the largest number that can be represented by an `f64`, `f64::INFINITY` is returned. If `self` is smaller than the smallest integer that can be represented by an `f64`, `f64::NEG_INFINITY` is returned.
    #[doc=crate::doc::example_header!(BUint)]
    /// let n = 8172394878u32;
    /// let u = BUint::<4>::from(n);
    /// assert_eq!(u.as_f64(), n as f64);
    /// ```
    #[inline]
    pub fn as_f64(&self) -> f64 {
        let f = self.unsigned_abs().as_f64();
        if self.is_negative() {
            -f
        } else {
            f
        }
    }

    #[inline]
    pub const fn as_buint<const M: usize>(&self) -> BUint<M> {
        if M > N {
            let padding_digit = if self.is_negative() {
                Digit::MAX
            } else {
                0
            };
            uint::cast_up::<N, M>(&self.bits, padding_digit)
        } else {
            uint::cast_down::<N, M>(&self.bits)
        }
    }
    
    #[inline]
    pub const fn as_bint<const M: usize>(&self) -> Bint<M> {
        Bint::from_bits(self.as_buint())
    }
}

macro_rules! as_bint {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> const CastFrom<$ty> for Bint<N> {
                fn cast_from(from: $ty) -> Self {
                    Self::from_bits(BUint::cast_from(from))
                }
            }
        )*
    }
}

as_bint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char);

#[cfg(test)]
mod tests {
    use crate::{I128, U128, digit, I64, test};
    use crate::cast::As;
    
    test::test_cast_to!([u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char] as U128);

    test::test_cast_from!(I128 as [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32/*, f64*/]);

    #[test]
    fn sign_fill_cast() {
        let i = -4004509459345i64;
        let int = I64::from(i);
        assert_eq!(int.as_i128(), i as i128);
        let i = -20495870948567i64;
        let int = I64::from(i);
        assert_eq!(int.as_u128(), i as u128);
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn as_buint() {
        let i = 39845968768945885i64;
        let int = I64::from(i);
        assert_eq!(U128::from(i as u128), int.as_buint::<{128 / digit::BITS as usize}>());
        let i = -4059684564856590i64;
        let int = I64::from(i);
        assert_eq!(U128::from(i as u128), int.as_buint::<{128 / digit::BITS as usize}>());
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn as_bint() {
        let i = 230987495678497456i64;
        let int = I64::from(i);
        assert_eq!(I128::from(i), int.as_bint::<{128 / digit::BITS as usize}>());
        let i = -2398679420567947564i64;
        let int = I64::from(i);
        assert_eq!(I128::from(i), int.as_bint::<{128 / digit::BITS as usize}>());
    }
}