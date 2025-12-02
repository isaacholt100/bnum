use super::{Integer, Uint};
use crate::cast;

#[inline]
const fn bytes_cast<const N: usize, const M: usize, const SIGNED: bool>(from: [u8; N]) -> [u8; M] {
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
            impl<const S: bool, const N: usize, const B: usize, const OM: u8> CastFrom<Integer<S, N, B, OM>> for $int {
                #[inline]
                fn cast_from(value: Integer<S, N, B, OM>) -> Self {
                    const BYTES: usize = (<$int>::BITS as usize) / 8;

                    let bytes = bytes_cast::<N, BYTES, S>(value.to_bytes());
                    Self::from_le_bytes(bytes)
                }
            }

            impl<const S: bool, const N: usize, const B: usize, const OM: u8> CastFrom<$int> for Integer<S, N, B, OM> {
                #[inline]
                fn cast_from(value: $int) -> Self {
                    #[allow(unused_comparisons)]
                    const SIGNED: bool = <$int>::MIN < 0;
                    const BYTES: usize = (<$int>::BITS as usize) / 8;

                    let bytes = bytes_cast::<BYTES, N, SIGNED>(value.to_le_bytes());
                    let mut out = Self::from_bytes(bytes);
                    out.set_sign_bits();
                    out
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
            impl<const S: bool, const N: usize, const B: usize, const OM: u8> CastFrom<Integer<S, N, B, OM>> for $f {
                #[inline]
                fn cast_from(value: Integer<S, N, B, OM>) -> Self {
                    if !S {
                        cast::float::cast_float_from_uint(value.force_sign::<false>())
                    } else {
                        let f = <$f>::cast_from(value.unsigned_abs_internal());
                        if value.is_negative_internal() { -f } else { f }
                    }
                }
            }

            impl<const S: bool, const N: usize, const B: usize, const OM: u8> CastFrom<$f> for Integer<S, N, B, OM> {
                #[inline]
                fn cast_from(value: $f) -> Self {
                    if !S {
                        cast::float::cast_uint_from_float::<$f, Uint<N, B, OM>>(value).force_sign::<S>()
                    } else {
                        crate::integer::cast::cast_int_from_float!(value)
                    }
                }
            }
        )*
    };
}

use crate::Exponent;
use crate::cast::CastFrom;
#[allow(unused_imports)]
use crate::cast::float::{CastFloatFromUintHelper, CastUintFromFloatHelper, FloatMantissa};

#[cfg(feature = "float")]
impl<const N: usize, const B: usize> FloatMantissa for Uint<N, B> {
    const MAX: Self = Self::MAX;

    #[inline]
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl<const N: usize, const B: usize, const OM: u8> CastUintFromFloatHelper for Uint<N, B, OM> {
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
}

impl<const N: usize, const B: usize, const OM: u8> CastFloatFromUintHelper for Uint<N, B, OM> {
    fn trailing_zeros(self) -> Exponent {
        Self::trailing_zeros(self)
    }
}

cast_uint_from_to_prim_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

cast_uint_from_to_prim_float!(f32, f64);

impl<const S: bool, const N: usize, const B: usize, const OM: u8> CastFrom<bool>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn cast_from(value: bool) -> Self {
        if value { Self::ONE } else { Self::ZERO }
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> CastFrom<char>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn cast_from(value: char) -> Self {
        Self::cast_from(value as u32)
    }
}

impl<
    const S: bool,
    const N: usize,
    const B: usize,
    const R: bool,
    const M: usize,
    const A: usize,
    const OM: u8,
> CastFrom<Integer<R, M, A, OM>> for Integer<S, N, B, OM>
{
    #[inline]
    fn cast_from(value: Integer<R, M, A, OM>) -> Self {
        let bytes = bytes_cast::<M, N, R>(value.to_bytes());
        let mut out = Self::from_bytes(bytes);
        out.set_sign_bits();
        out
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

#[cfg(test)]
crate::test::test_all_custom_bit_widths! {
    use crate::cast::{CastFrom, CastTo};
    use crate::test;

    test::test_from! {
        function: <utest as CastFrom>::cast_from,
        from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }

    test::test_into! {
        function: <utest as CastTo>::cast_to,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }


    test::test_from! {
        function: <itest as CastFrom>::cast_from,
        from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }

    test::test_into! {
        function: <itest as CastTo>::cast_to,
        into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)
    }
}

#[cfg(test)]
mod double_custom_bit_width_cast_tests {
    use crate::test::BitInt;
    use crate::cast::CastFrom;
    use crate::Integer;
    use crate::literal_parse::get_size_params_from_bits;

    macro_rules! test_double_custom_bit_width_cast {
        ($($from: literal => $to: literal), *) => {
            paste::paste! {
                $(
                    quickcheck::quickcheck! {
                        fn [<quickcheck_cast_from_u_ $from _to $to _bits>](v: BitInt<false, $from>) -> bool {
                            let w = Integer::from(v);
                            let a = BitInt::<false, $to>::cast_from(v);
                            let b = Integer::<false, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::cast_from(w);

                            let w = Integer::from(v);
                            let c = BitInt::<true, $to>::cast_from(v);
                            let d = Integer::<true, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::cast_from(w);

                            crate::test::test_eq(a, b) && crate::test::test_eq(c, d)
                        }

                        fn [<quickcheck_cast_from_i_ $from _to $to _bits>](v: BitInt<true, $from>) -> bool {
                            let w = Integer::from(v);
                            let a = BitInt::<false, $to>::cast_from(v);
                            let b = Integer::<false, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::cast_from(w);

                            let w = Integer::from(v);
                            let c = BitInt::<true, $to>::cast_from(v);
                            let d = Integer::<true, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::cast_from(w);
                            crate::test::test_eq(a, b) && crate::test::test_eq(c, d)
                        }
                    }
                )*
            }
        };
    }

    test_double_custom_bit_width_cast!(
        8 => 16,
        16 => 32,
        32 => 64,
        64 => 128,
        16 => 8,
        32 => 16,
        64 => 32,
        128 => 64,
        16 => 128,
        8 => 64,
        128 => 32,
        64 => 16,
        127 => 128,
        129 => 128,
        173 => 256,
        256 => 173,
        5 => 11,
        4 => 23,
        23 => 4,
        289 => 160,
        160 => 289
    );
}
