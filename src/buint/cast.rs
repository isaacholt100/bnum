use super::BUintD8;
use crate::cast;

macro_rules! cast_uint_from_to_prim_int {
    ($($int: ty), *) => {
        $(
            impl<const N: usize> CastFrom<BUintD8<N>> for $int {
                #[inline]
                fn cast_from(from: BUintD8<N>) -> Self {
                    const BYTES: usize = (<$int>::BITS as usize) / 8;

                    let bytes = cast::bytes_cast::<N, BYTES, false>(from.to_le_bytes());
                    Self::from_le_bytes(bytes)
                }
            }

            impl<const N: usize> CastFrom<$int> for BUintD8<N> {
                #[inline]
                fn cast_from(from: $int) -> Self {
                    #[allow(unused_comparisons)]
                    const SIGNED: bool = <$int>::MIN < 0;
                    const BYTES: usize = (<$int>::BITS as usize) / 8;

                    let bytes = cast::bytes_cast::<BYTES, N, SIGNED>(from.to_le_bytes());
                    Self::from_le_bytes(bytes)
                }
            }
        )*
    };
}

macro_rules! buint_as_float {
    ($f: ty) => {
        impl<const N: usize> CastFrom<BUintD8<N>> for $f {
            #[inline]
            fn cast_from(value: BUintD8<N>) -> Self {
                cast::float::cast_float_from_uint(value)
            }
        }
    };
}

#[allow(unused_imports)]
use crate::cast::float::{CastFloatFromUintHelper, CastUintFromFloatHelper, FloatMantissa};
use crate::cast::CastFrom;
use crate::ExpType;

#[cfg(feature = "float")]
impl<const N: usize> FloatMantissa for BUintD8<N> {
    const TWO: Self = Self::TWO;
    const MAX: Self = Self::MAX;

    #[inline]
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl<const N: usize> CastUintFromFloatHelper for BUintD8<N> {
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}

impl<const N: usize> CastFloatFromUintHelper for BUintD8<N> {
    fn trailing_zeros(self) -> ExpType {
        Self::trailing_zeros(self)
    }
}

cast_uint_from_to_prim_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

buint_as_float!(f32);
buint_as_float!(f64);

impl<const N: usize> CastFrom<bool> for BUintD8<N> {
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
    #[inline]
    fn cast_from(from: char) -> Self {
        Self::cast_from(from as u32)
    }
}

impl<const N: usize, const M: usize> CastFrom<BUintD8<M>> for BUintD8<N> {
    #[inline]
    fn cast_from(from: BUintD8<M>) -> Self {
        let bytes = cast::bytes_cast::<M, N, false>(from.to_le_bytes());
        Self::from_le_bytes(bytes)
    }
}

#[cfg(feature = "signed")]
impl<const N: usize, const M: usize> CastFrom<crate::BIntD8<M>> for BUintD8<N> {
    #[inline]
    fn cast_from(from: crate::BIntD8<M>) -> Self {
        let bytes = cast::bytes_cast::<M, N, true>(from.to_le_bytes());
        Self::from_le_bytes(bytes)
    }
}

impl<const N: usize> CastFrom<f32> for BUintD8<N> {
    #[inline]
    fn cast_from(value: f32) -> Self {
        cast::float::cast_uint_from_float(value)
    }
}

impl<const N: usize> CastFrom<f64> for BUintD8<N> {
    #[inline]
    fn cast_from(value: f64) -> Self {
        cast::float::cast_uint_from_float(value)
    }
}

#[cfg(test)]
mod tests {
    crate::int::cast::tests!(utest);
}
