use super::BIint;
use crate::uint::BUint;
use crate::uint;
use crate::digit::{Digit, self};

macro_rules! as_int {
    ($method: ident, $int: ty, $type_str: expr, $assertion: expr) => {
        /// Casts `self` to 
        #[doc=$type_str]
        /// # Examples
        /// 
        /// ```
        /// use bint::BIint;
        /// 
        /// let n = 1097937598374598734507959845u128;
        /// let u = BIint::<2>::from(n);
        #[doc=$assertion]
        /// ```
        pub const fn $method(&self) -> $int {
            const ZERO: $int = 0;
            const ONES: $int = ZERO.wrapping_sub(1);
            if self.is_negative() {
                let digits = self.digits();
                let mut out = ONES;
                let mut i = 0;
                while i << digit::BIT_SHIFT < <$int>::BITS as crate::ExpType && i < N {
                    out &= !((!digits[i]) as $int << (i << digit::BIT_SHIFT));
                    i += 1;
                }
                out
            } else {
                self.uint.$method()
            }
        }
    };
}

impl<const N: usize> BIint<N> {
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

    pub fn as_f32(&self) -> f32 {
        let f = self.unsigned_abs().as_f32();
        if self.is_negative() {
            -f
        } else {
            f
        }
    }

    pub fn as_f64(&self) -> f64 {
        let f = self.unsigned_abs().as_f64();
        if self.is_negative() {
            -f
        } else {
            f
        }
    }

    #[cfg(feature = "nightly")]
    pub const fn as_buint<const M: usize>(&self) -> BUint<M> where [Digit; M - N]: Sized {
        if M > N {
            let padding_digit = if self.is_negative() {
                Digit::MAX
            } else {
                0
            };
            uint::cast_up::<N, M>(&self.uint, padding_digit)
        } else {
            uint::cast_down::<N, M>(&self.uint)
        }
    }
    #[cfg(feature = "nightly")]
    pub const fn as_biint<const M: usize>(&self) -> BIint<M> where [Digit; M - N]: Sized {
        BIint {
            uint: self.as_buint()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{I128, U128, BIint, digit};
    macro_rules! test_cast {
        ($name: ident, $ty: ty, $($int: expr), *) => {
            #[test]
            fn $name() {
                $(
                    let i = $int;
                    let int = I128::from(i);
                    assert_eq!(int.$name(), i as $ty);
                )*
            }
        }
    }
    test_cast!(as_u8, u8, 458937495794835975i128, -29384759987497i128);
    test_cast!(as_u16, u16, 102934875345345198i128, -913702945789798i128);
    test_cast!(as_u32, u32, 10979450769725496798i128, -10249567973451i128);
    test_cast!(as_u64, u64, 10938745987983453758i128, -9374589794256667i128);
    test_cast!(as_u128, u128, 10938724769498576897445987983453758i128, -93745897942945768979834656556667i128);
    test_cast!(as_usize, usize, 908792947459670945345127i128, -109237029456794705968789i128);
    test_cast!(as_i8, i8, 2039865797i128, -402975694757i128);
    test_cast!(as_i16, i16, 20298756979746597i128, -2039479945679i128);
    test_cast!(as_i32, i32, 2405679207457979i128, -340139749576934598i128);
    test_cast!(as_i64, i64, 9874526834687545876i128, -2098679457699458765i128);
    test_cast!(as_i128, i128, 270974906739475967495876897i128, -20598794576984756897546i128);

    #[test]
    fn sign_fill_cast() {
        let i = -4004509459345i64;
        let int = BIint::<{64 / digit::BITS}>::from(i);
        assert_eq!(int.as_i128(), i as i128);
        let i = -20495870948567i64;
        let int = BIint::<{64 / digit::BITS}>::from(i);
        assert_eq!(int.as_u128(), i as u128);
    }

    test_cast!(as_isize, isize, 394769476974569745987i128, -102934794587689457i128);
    test_cast!(as_f32, f32, 239794570942799856546i128, -8498567294094756974568i128);
    test_cast!(as_f64, f64, 2094857694759475689745897i128, -72079847568974568i128);

    #[cfg(feature = "nightly")]
    #[test]
    fn as_buint() {
        let i = 39845968768945885i64;
        let int = BIint::<{64 / digit::BITS}>::from(i);
        assert_eq!(U128::from(i as u128), int.as_buint::<{128 / digit::BITS}>());
        let i = -4059684564856590i64;
        let int = BIint::<{64 / digit::BITS}>::from(i);
        assert_eq!(U128::from(i as u128), int.as_buint::<{128 / digit::BITS}>());
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn as_biint() {
        let i = 230987495678497456i64;
        let int = BIint::<{64 / digit::BITS}>::from(i);
        assert_eq!(I128::from(i), int.as_biint::<{128 / digit::BITS}>());
        let i = -2398679420567947564i64;
        let int = BIint::<{64 / digit::BITS}>::from(i);
        assert_eq!(I128::from(i), int.as_biint::<{128 / digit::BITS}>());
    }
}