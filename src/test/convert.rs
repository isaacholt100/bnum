use crate::{Int, Uint, Integer};

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
                    type Output = Uint<{usize::div_ceil($bits, 8)}, $bits>;

                    #[inline]
                    fn into(self) -> Self::Output {
                        Uint::from_le_bytes(self.to_le_bytes())
                    }
                }

                impl TestConvert for [<i $bits>] {
                    type Output = Int<{usize::div_ceil($bits, 8)}, $bits>;

                    #[inline]
                    fn into(self) -> Self::Output {
                        Int::from_le_bytes(self.to_le_bytes())
                    }
                }
            )*
        }
    };
}

test_convert_bigints!(128, 64, 32, 16, 8);

impl TestConvert for usize {
    type Output = Uint<{usize::BITS as usize / 8}, {usize::BITS as usize}>;

    #[inline]
    fn into(self) -> Self::Output {
        Uint::from_le_bytes(self.to_le_bytes())
    }
}

impl TestConvert for isize {
    type Output = Int<{isize::BITS as usize / 8}, {isize::BITS as usize}>;

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

impl TestConvert for f64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_bits()
    }
}

impl TestConvert for f32 {
    type Output = u32;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_bits()
    }
}

// #[cfg(feature = "float")]
// impl TestConvert for crate::types::F64 {
//     type Output = u64;

//     #[inline]
//     fn into(self) -> Self::Output {
//         use crate::cast::As;

//         self.to_bits().as_()
//     }
// }

// #[cfg(feature = "float")]
// impl TestConvert for crate::types::F32 {
//     type Output = u32;

//     #[inline]
//     fn into(self) -> Self::Output {
//         use crate::cast::As;

//         self.to_bits().as_()
//     }
// }

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
