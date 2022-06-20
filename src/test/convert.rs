use crate::types::{U128, I128, U64, I64/*, F64*/};

pub trait TestConvert {
    type Output;

    fn into(self) -> Self::Output;
}

impl TestConvert for u128 {
    type Output = u128;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_le()
    }
}

impl TestConvert for U128 {
    type Output = u128;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl TestConvert for u64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_le()
    }
}

impl TestConvert for U64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl TestConvert for I64 {
    type Output = i64;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
    }
}

impl TestConvert for i128 {
    type Output = i128;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_le()
    }
}

impl TestConvert for I128 {
    type Output = i128;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self)
        }
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
        self.to_bits().to_le()
    }
}

impl TestConvert for f32 {
    type Output = u32;

    #[inline]
    fn into(self) -> Self::Output {
        self.to_bits().to_le()
    }
}

/*impl TestConvert for F64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self.to_bits())
        }
    }
}*/

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

impl TestConvert for u32 {
    type Output = u32;
    
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
        match self {
            Ok(val) => Ok(TestConvert::into(val)),
            Err(err) => Err(TestConvert::into(err)),
        }
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

test_convert_to_self!(core::num::FpCategory, bool, core::cmp::Ordering, u8, u16, usize, i8, i16, i32, i64, isize);