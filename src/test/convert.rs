use crate::{Int, Uint, Integer};
use core::num::{Wrapping, Saturating};
use crate::t;

pub trait TestConvert {
    type Output;

    fn into(self) -> Self::Output;
}

#[allow(unused)] // since this is only used with certain crate feature but these are likely to change often
pub fn test_eq<T, U>(t: T, u: U) -> bool
where
    T: TestConvert,
    U: TestConvert,
    <T as TestConvert>::Output: PartialEq<<U as TestConvert>::Output>,
{
    t.into() == u.into()
}

macro_rules! test_convert_bigints {
    ($($bits: literal), *) => {
        paste::paste! {
            $(
                impl TestConvert for [<u $bits>] {
                    type Output = t!([<U $bits>]);

                    #[inline]
                    fn into(self) -> Self::Output {
                        Uint::from_le_bytes(self.to_le_bytes())
                    }
                }

                impl TestConvert for [<i $bits>] {
                    type Output = t!([<I $bits>]);

                    #[inline]
                    fn into(self) -> Self::Output {
                        Int::from_le_bytes(self.to_le_bytes())
                    }
                }

                impl TestConvert for Wrapping<[<u $bits>]> {
                    type Output = t!([<U $bits w>]);

                    #[inline]
                    fn into(self) -> Self::Output {
                        TestConvert::into(self.0).force()
                    }
                }

                impl TestConvert for Wrapping<[<i $bits>]> {
                    type Output = t!([<I $bits w>]);

                    #[inline]
                    fn into(self) -> Self::Output {
                        TestConvert::into(self.0).force()
                    }
                }

                impl TestConvert for Saturating<[<u $bits>]> {
                    type Output = t!([<U $bits s>]);

                    #[inline]
                    fn into(self) -> Self::Output {
                        TestConvert::into(self.0).force()
                    }
                }

                impl TestConvert for Saturating<[<i $bits>]> {
                    type Output = t!([<I $bits s>]);

                    #[inline]
                    fn into(self) -> Self::Output {
                        TestConvert::into(self.0).force()
                    }
                }
            )*
        }
    };
}

test_convert_bigints!(128, 64, 32, 16, 8);

impl TestConvert for usize {
    type Output = Uint<{usize::BITS as usize / 8}>;

    #[inline]
    fn into(self) -> Self::Output {
        Uint::from_le_bytes(self.to_le_bytes())
    }
}

impl TestConvert for isize {
    type Output = Int<{isize::BITS as usize / 8}>;

    #[inline]
    fn into(self) -> Self::Output {
        Int::from_le_bytes(self.to_le_bytes())
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> TestConvert for Integer<S, N, B, OM> {
    type Output = Self;

    #[inline]
    fn into(self) -> Self::Output {
        self
    }
}

impl<T: TestConvert> TestConvert for Option<T> {
    type Output = Option<<T as TestConvert>::Output>;

    #[inline]
    fn into(self) -> Self::Output {
        self.map(TestConvert::into)
    }
}

// IEEE 754 and Rust do not specify the sign bit or payload of NaN results produced by
// arithmetic or math operations across architectures: for instance, on x86_64 SSE,
// `0.0 / 0.0`, `0.0 % 0.0`, `(-1.0).sqrt()`, and `inf - inf` produce the negative
// quiet NaN indefinite (`0xfff8_0000_0000_0000` / `0xffc0_0000`), whereas on aarch64
// they produce positive quiet NaN (`0x7ff8_0000_0000_0000` / `0x7fc0_0000`).
// Canonicalise all NaNs in `TestConvert` so any two NaNs compare equal while every
// non-NaN value (including `+0.0` vs `-0.0`) is still checked bit-for-bit.
const CANONICAL_F64_NAN_BITS: u64 = 0x7ff8_0000_0000_0000;
const CANONICAL_F32_NAN_BITS: u32 = 0x7fc0_0000;

impl TestConvert for f64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        if self.is_nan() {
            CANONICAL_F64_NAN_BITS
        } else {
            self.to_bits()
        }
    }
}

impl TestConvert for f32 {
    type Output = u32;

    #[inline]
    fn into(self) -> Self::Output {
        if self.is_nan() {
            CANONICAL_F32_NAN_BITS
        } else {
            self.to_bits()
        }
    }
}

#[cfg(feature = "float")]
impl TestConvert for crate::types::F64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        use crate::cast::As;

        if self.is_nan() {
            CANONICAL_F64_NAN_BITS
        } else {
            self.to_bits().as_()
        }
    }
}

#[cfg(feature = "float")]
impl TestConvert for crate::types::F32 {
    type Output = u32;

    #[inline]
    fn into(self) -> Self::Output {
        use crate::cast::As;

        if self.is_nan() {
            CANONICAL_F32_NAN_BITS
        } else {
            self.to_bits().as_()
        }
    }
}

impl<T: TestConvert, U: TestConvert> TestConvert for (T, U) {
    type Output = (<T as TestConvert>::Output, <U as TestConvert>::Output);

    #[inline]
    fn into(self) -> Self::Output {
        (TestConvert::into(self.0), TestConvert::into(self.1))
    }
}

impl<T, const N: usize> TestConvert for [T; N] {
    type Output = Self;

    #[inline]
    fn into(self) -> Self::Output {
        self
    }
}

impl TestConvert for crate::errors::ParseIntError {
    type Output = core::num::IntErrorKind;

    #[inline]
    fn into(self) -> Self::Output {
        self.kind().clone()
    }
}

impl TestConvert for core::num::ParseIntError {
    type Output = core::num::IntErrorKind;

    #[inline]
    fn into(self) -> Self::Output {
        self.kind().clone()
    }
}

#[cfg(feature = "float")]
impl TestConvert for core::num::ParseFloatError {
    type Output = bool; // true if Empty, false if Invalid

    #[inline]
    fn into(self) -> Self::Output {
        self == "".parse::<f32>().unwrap_err()
    }
}

#[cfg(feature = "float")]
impl TestConvert for crate::errors::ParseFloatError {
    type Output = bool; // true if Empty, false if Invalid

    #[inline]
    fn into(self) -> Self::Output {
        self == crate::errors::ParseFloatError::EMPTY
    }
}

impl<T: TestConvert, E: TestConvert> TestConvert for Result<T, E> {
    type Output = Result<<T as TestConvert>::Output, <E as TestConvert>::Output>;

    #[inline]
    fn into(self) -> Self::Output {
        self.map(TestConvert::into).map_err(TestConvert::into)
    }
}

impl TestConvert for core::num::TryFromIntError {
    type Output = ();

    #[inline]
    fn into(self) -> Self::Output {
        ()
    }
}

impl TestConvert for crate::errors::TryFromIntError {
    type Output = ();

    #[inline]
    fn into(self) -> Self::Output {
        ()
    }
}

// impl<T> TestConvert for Vec<T> {
//     type Output = Self;

//     #[inline]
//     fn into(self) -> Self::Output {
//         self
//     }
// }

impl TestConvert for core::char::TryFromCharError {
    type Output = ();

    #[inline]
    fn into(self) -> Self::Output {
        ()
    }
}

impl TestConvert for crate::errors::TryFromCharError {
    type Output = ();

    #[inline]
    fn into(self) -> Self::Output {
        ()
    }
}

impl TestConvert for core::convert::Infallible {
    type Output = ();

    #[inline]
    fn into(self) -> Self::Output {
        ()
    }
}

macro_rules! test_convert_to_self {
    ($($ty: ty), *) => {
        $(
            impl TestConvert for $ty {
                type Output = Self;

                #[inline]
                fn into(self) -> Self::Output {
                    self
                }
            }
        )*
    };
}

test_convert_to_self!(
    core::num::FpCategory,
    core::cmp::Ordering,
    bool
);

#[cfg(feature = "alloc")]
test_convert_to_self!(alloc::string::String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_canonicalises_float_nans() {
        // Simulate x86_64 negative QNaN indefinite, aarch64 positive QNaN, payload NaNs, and sNaNs.
        for nan64_bits in [
            0x7ff8_0000_0000_0000u64,
            0xfff8_0000_0000_0000u64,
            0x7ff8_0000_0000_0001u64,
            0xfff8_1234_5678_9abcu64,
            0x7ff0_0000_0000_0001u64,
            0xfff0_0000_0000_0001u64,
        ] {
            let f = f64::from_bits(nan64_bits);
            assert_eq!(TestConvert::into(f), CANONICAL_F64_NAN_BITS);
            #[cfg(feature = "float")]
            {
                let bf = crate::types::F64::from(f);
                assert_eq!(TestConvert::into(bf), CANONICAL_F64_NAN_BITS);
                assert!(test_eq(bf, f));
                assert!(test_eq(crate::types::F64::NAN, f));
            }
        }

        for nan32_bits in [
            0x7fc0_0000u32,
            0xffc0_0000u32,
            0x7fc0_0001u32,
            0xffc0_1234u32,
            0x7f80_0001u32,
            0xff80_0001u32,
        ] {
            let f = f32::from_bits(nan32_bits);
            assert_eq!(TestConvert::into(f), CANONICAL_F32_NAN_BITS);
            #[cfg(feature = "float")]
            {
                let bf = crate::types::F32::from(f);
                assert_eq!(TestConvert::into(bf), CANONICAL_F32_NAN_BITS);
                assert!(test_eq(bf, f));
                assert!(test_eq(crate::types::F32::NAN, f));
            }
        }

        // Non-NaN bit patterns (including +0.0 vs -0.0 and +inf vs -inf) must remain distinct.
        assert_ne!(TestConvert::into(0.0_f64), TestConvert::into(-0.0_f64));
        assert_ne!(TestConvert::into(0.0_f32), TestConvert::into(-0.0_f32));
        assert_ne!(TestConvert::into(f64::INFINITY), TestConvert::into(f64::NAN));
        assert_ne!(TestConvert::into(f32::INFINITY), TestConvert::into(f32::NAN));
    }
}
