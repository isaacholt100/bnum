use super::Int;
use crate::Uint;
use crate::cast;

macro_rules! bint_as_primitive {
    ($($int: ty), *) => {
        $(
            impl<const N: usize> CastFrom<Int<N>> for $int {
                #[inline]
                fn cast_from(from: Int<N>) -> Self {
                    const BYTES: usize = (<$int>::BITS as usize) / 8;

                    let bytes = cast::bytes_cast::<N, BYTES, true>(from.to_le_bytes());
                    Self::from_le_bytes(bytes)
                }
            }
        )*
    };
}

macro_rules! primitive_as_bint {
    ($($ty: ty), *) => {
        $(
            impl<const N: usize> CastFrom<$ty> for Int<N> {
                #[inline]
                fn cast_from(from: $ty) -> Self {
                    Self::from_bits(Uint::cast_from(from))
                }
            }
        )*
    }
}

macro_rules! bint_cast_from_float {
    ($f: ty) => {
        #[inline]
        fn cast_from(from: $f) -> Self {
            if from.is_sign_negative() {
                let u = Uint::<N>::cast_from(-from);
                if u >= Self::MIN.to_bits() {
                    Self::MIN
                } else {
                    -Self::from_bits(u)
                }
            } else {
                let u = Uint::<N>::cast_from(from);
                let i = Self::from_bits(u);
                if i.is_negative() { Self::MAX } else { i }
            }
        }
    };
}

pub(crate) use bint_cast_from_float;

use crate::cast::CastFrom;

bint_as_primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

impl<const N: usize> CastFrom<Int<N>> for f32 {
    #[inline]
    fn cast_from(from: Int<N>) -> Self {
        let f = f32::cast_from(from.unsigned_abs());
        if from.is_negative() { -f } else { f }
    }
}

impl<const N: usize> CastFrom<Int<N>> for f64 {
    #[inline]
    fn cast_from(from: Int<N>) -> Self {
        let f = f64::cast_from(from.unsigned_abs());
        if from.is_negative() { -f } else { f }
    }
}

primitive_as_bint!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char
);

impl<const N: usize, const M: usize> CastFrom<Uint<M>> for Int<N> {
    #[inline]
    fn cast_from(from: Uint<M>) -> Self {
        Self::from_bits(Uint::cast_from(from))
    }
}

impl<const N: usize, const M: usize> CastFrom<Int<M>> for Int<N> {
    #[inline]
    fn cast_from(from: Int<M>) -> Self {
        Self::from_bits(Uint::cast_from(from))
    }
}

impl<const N: usize> CastFrom<f32> for Int<N> {
    crate::bint::cast::bint_cast_from_float!(f32);
}

impl<const N: usize> CastFrom<f64> for Int<N> {
    crate::bint::cast::bint_cast_from_float!(f64);
}

#[cfg(test)]
mod tests {
    crate::int::cast::tests!(itest);
}
