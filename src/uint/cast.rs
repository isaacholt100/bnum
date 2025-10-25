use super::{Integer, Uint};
use crate::cast;

const fn bytes_cast<const N: usize, const M: usize, const SIGNED: bool>(
    from: [u8; N],
) -> [u8; M] {
    // We don't need to handle the case N = M, as the compiler optimises it away.
    let pad = if SIGNED && M > N && (from[N - 1] as i8).is_negative() {
        u8::MAX
    } else {
        0
    };
    let mut bytes = [pad; M];
    let mut i = 0;
    while i < if N < M { N } else { M } {
        bytes[i] = from[i];
        i += 1;
    }
    bytes
}

macro_rules! cast_uint_from_to_prim_int {
    ($($int: ty), *) => {
        $(
            impl<const S: bool, const N: usize> CastFrom<Integer<S, N>> for $int {
                #[inline]
                fn cast_from(value: Integer<S, N>) -> Self {
                    const BYTES: usize = (<$int>::BITS as usize) / 8;

                    let bytes = bytes_cast::<N, BYTES, S>(value.to_le_bytes());
                    Self::from_le_bytes(bytes)
                }
            }

            impl<const S: bool, const N: usize> CastFrom<$int> for Integer<S, N> {
                #[inline]
                fn cast_from(value: $int) -> Self {
                    #[allow(unused_comparisons)]
                    const SIGNED: bool = <$int>::MIN < 0;
                    const BYTES: usize = (<$int>::BITS as usize) / 8;

                    let bytes = bytes_cast::<BYTES, N, SIGNED>(value.to_le_bytes());
                    Self::from_le_bytes(bytes)
                }
            }
        )*
    };
}

macro_rules! cast_int_from_float {
    ($value: ident) => {
        if $value.is_sign_negative() {
            let u = Uint::cast_from(-$value);
            if u >= crate::Int::MIN.cast_unsigned() {
                Self::MIN
            } else {
                (-(u.force_sign::<true>())).force_sign()
            }
        } else {
            let u = Uint::cast_from($value);
            let i = u.cast_signed();
            if i.is_negative() {
                Self::MAX
            } else {
                i.force_sign()
            }
        }
    };
}

pub(crate) use cast_int_from_float;

macro_rules! cast_uint_from_to_prim_float {
    ($($f: ty), *) => {
        $(
            impl<const S: bool, const N: usize> CastFrom<Integer<S, N>> for $f {
                #[inline]
                fn cast_from(value: Integer<S, N>) -> Self {
                    if !S {
                        cast::float::cast_float_from_uint(value.force_sign::<false>())
                    } else {
                        let f = <$f>::cast_from(value.unsigned_abs_internal());
                        if value.is_negative_internal() { -f } else { f }
                    }
                }
            }

            impl<const S: bool, const N: usize> CastFrom<$f> for Integer<S, N> {
                #[inline]
                fn cast_from(value: $f) -> Self {
                    if !S {
                        cast::float::cast_uint_from_float::<$f, Uint<N>>(value).force_sign::<S>()
                    } else {
                        crate::uint::cast::cast_int_from_float!(value)
                    }
                }
            }
        )*
    };
}

use crate::ExpType;
use crate::cast::CastFrom;
#[allow(unused_imports)]
use crate::cast::float::{CastFloatFromUintHelper, CastUintFromFloatHelper, FloatMantissa};

#[cfg(feature = "float")]
impl<const N: usize> FloatMantissa for Uint<N> {
    const MAX: Self = Self::MAX;

    #[inline]
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl<const N: usize> CastUintFromFloatHelper for Uint<N> {
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}

impl<const N: usize> CastFloatFromUintHelper for Uint<N> {
    fn trailing_zeros(self) -> ExpType {
        Self::trailing_zeros(self)
    }
}

cast_uint_from_to_prim_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

cast_uint_from_to_prim_float!(f32, f64);

impl<const S: bool, const N: usize> CastFrom<bool> for Integer<S, N> {
    #[inline]
    fn cast_from(value: bool) -> Self {
        if value { Self::ONE } else { Self::ZERO }
    }
}

impl<const S: bool, const N: usize> CastFrom<char> for Integer<S, N> {
    #[inline]
    fn cast_from(value: char) -> Self {
        Self::cast_from(value as u32)
    }
}

impl<const S: bool, const N: usize, const R: bool, const M: usize> CastFrom<Integer<R, M>>
    for Integer<S, N>
{
    #[inline]
    fn cast_from(value: Integer<R, M>) -> Self {
        let bytes = bytes_cast::<M, N, R>(value.to_le_bytes());
        Self::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::cast::{CastFrom, CastTo};
    use crate::test;
    use crate::test::cast_types::*;

    crate::test::test_all! {
        testing integers;
        
        test::test_from! {
            function: <stest as CastFrom>::cast_from,
            from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
        }
        
        test::test_into! {
            function: <stest as CastTo>::cast_to,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
        }
    }
}
