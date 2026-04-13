use crate::cast::CastFrom;
use crate::errors::{ParseIntError, TryFromCharError, TryFromIntError};
use crate::{Exponent, Int, Integer, Uint};
use core::str::FromStr;

pub trait IntegerConvertHelper {
    const BITS: Exponent;
    const SIGNED: bool;

    fn leading_zeros_at_least_threshold(&self, threshold: Exponent) -> bool;
    fn is_negative(&self) -> bool;
    fn leading_ones_at_least_threshold(&self, threshold: Exponent) -> bool;
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> IntegerConvertHelper
    for Integer<S, N, B, OM>
{
    const BITS: Exponent = Self::BITS;
    const SIGNED: bool = S;

    #[inline]
    fn leading_zeros_at_least_threshold(&self, threshold: Exponent) -> bool {
        Self::leading_zeros_at_least_threshold(self, threshold)
    }

    #[inline]
    fn leading_ones_at_least_threshold(&self, threshold: Exponent) -> bool {
        Self::leading_ones_at_least_threshold(self, threshold)
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.is_negative_internal()
    }
}

macro_rules! impl_int_convert_helper_for_primitive_int {
    ($($int: ty), *) => {
        $(
            impl IntegerConvertHelper for $int {
                const BITS: Exponent = Self::BITS as Exponent;
                #[allow(unused_comparisons)]
                const SIGNED: bool = <$int>::MIN < 0;

                #[inline]
                fn leading_zeros_at_least_threshold(&self, threshold: Exponent) -> bool {
                    self.leading_zeros() >= threshold
                }

                #[inline]
                fn leading_ones_at_least_threshold(&self, threshold: Exponent) -> bool {
                    self.leading_ones() >= threshold
                }

                #[inline]
                fn is_negative(&self) -> bool {
                    #[allow(unused_comparisons)]
                    { self < &0 }
                }
            }
        )*
    };
}

impl_int_convert_helper_for_primitive_int!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

// Conversion functions

#[inline]
fn uint_try_from_uint<U, V>(uint: U) -> Result<V, TryFromIntError>
where
    V: CastFrom<U>,
    U: IntegerConvertHelper,
    V: IntegerConvertHelper,
{
    if U::BITS <= V::BITS || uint.leading_zeros_at_least_threshold(U::BITS - V::BITS) {
        Ok(V::cast_from(uint))
    } else {
        Err(TryFromIntError(()))
    }
}

#[inline]
fn uint_try_from_int<I, U>(int: I) -> Result<U, TryFromIntError>
where
    U: CastFrom<I>,
    I: IntegerConvertHelper,
    U: IntegerConvertHelper,
{
    if int.is_negative() {
        return Err(TryFromIntError(()));
    }
    if I::BITS - 1 <= U::BITS || int.leading_zeros_at_least_threshold(I::BITS - U::BITS) {
        Ok(U::cast_from(int))
    } else {
        Err(TryFromIntError(()))
    }
}

#[inline]
fn int_try_from_uint<U, I>(uint: U) -> Result<I, TryFromIntError>
where
    I: CastFrom<U>,
    U: IntegerConvertHelper,
    I: IntegerConvertHelper,
{
    if U::BITS <= I::BITS - 1 || uint.leading_zeros_at_least_threshold(U::BITS - I::BITS + 1) {
        Ok(I::cast_from(uint))
    } else {
        Err(TryFromIntError(()))
    }
}

#[inline]
fn int_try_from_int<I, J>(int: I) -> Result<J, TryFromIntError>
where
    J: CastFrom<I>,
    I: IntegerConvertHelper,
    J: IntegerConvertHelper,
{
    if I::BITS <= J::BITS {
        return Ok(J::cast_from(int));
    }
    if int.is_negative() {
        if int.leading_ones_at_least_threshold(I::BITS - J::BITS + 1) {
            Ok(J::cast_from(int))
        } else {
            Err(TryFromIntError(()))
        }
    } else {
        if int.leading_zeros_at_least_threshold(I::BITS - J::BITS + 1) {
            Ok(J::cast_from(int))
        } else {
            Err(TryFromIntError(()))
        }
    }
}

#[inline]
fn integer_try_from_integer<I, J>(int: I) -> Result<J, TryFromIntError>
where
    J: CastFrom<I>,
    I: IntegerConvertHelper,
    J: IntegerConvertHelper,
{
    match (J::SIGNED, I::SIGNED) {
        (false, false) => uint_try_from_uint(int),
        (false, true) => uint_try_from_int(int),
        (true, false) => int_try_from_uint(int),
        (true, true) => int_try_from_int(int),
    }
}

macro_rules! integer_try_from_into_primitive_integer {
    ($($uint: ty), *) => {
        $(
            impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<Integer<S, N, B, OM>> for $uint {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(value: Integer<S, N, B, OM>) -> Result<Self, Self::Error> {
                    integer_try_from_integer(value)
                }
            }

            impl<const S: bool, const N: usize, const B: usize, const OM: u8> TryFrom<$uint> for Integer<S, N, B, OM> {
                type Error = TryFromIntError;

                #[inline]
                fn try_from(value: $uint) -> Result<Self, Self::Error> {
                    integer_try_from_integer(value)
                }
            }
        )*
    }
}

integer_try_from_into_primitive_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

// just for testing, we want to compare Integer's with wrapping behaviour and Wrapping(prim_int), and with saturating behaviour and Saturating(prim_int), so need to be able to convert
#[cfg(test)]
impl<const S: bool, const N: usize, const B: usize, const OM: u8, T> TryFrom<Integer<S, N, B, OM>> for core::num::Wrapping<T> where T: TryFrom<Integer<S, N, B, OM>> {
    type Error = <T as TryFrom<Integer<S, N, B, OM>>>::Error;

    #[inline]
    fn try_from(value: Integer<S, N, B, OM>) -> Result<Self, Self::Error> {
        Ok(core::num::Wrapping(value.try_into()?))
    }
}

#[cfg(test)]
impl<const S: bool, const N: usize, const B: usize, const OM: u8, T> TryFrom<Integer<S, N, B, OM>> for core::num::Saturating<T> where T: TryFrom<Integer<S, N, B, OM>> {
    type Error = <T as TryFrom<Integer<S, N, B, OM>>>::Error;

    #[inline]
    fn try_from(value: Integer<S, N, B, OM>) -> Result<Self, Self::Error> {
        Ok(core::num::Saturating(value.try_into()?))
    }
}

// impl<
//     const S: bool,
//     const N: usize,
//     const B: usize,
//     const R: bool,
//     const M: usize,
//     const A: usize,
//     const OM: u8,
// > BTryFrom<Integer<R, M, A, OM>> for Integer<S, N, B, OM>
// {
//     type Error = TryFromIntError;

//     fn try_from(value: Integer<R, M, A, OM>) -> Result<Self, Self::Error> {
//         integer_try_from_integer(value)
//     }
// }

impl<
    const S: bool,
    const N: usize,
    const B: usize,
    const R: bool,
    const M: usize,
    const A: usize,
    const OM: u8,
> TryFrom<&Integer<R, M, A, OM>> for Integer<S, N, B, OM>
{
    type Error = TryFromIntError;

    fn try_from(value: &Integer<R, M, A, OM>) -> Result<Self, Self::Error> {
        integer_try_from_integer(*value)
    }
}

impl<const N: usize, const B: usize, const M: usize, const A: usize, const OM: u8>
    TryFrom<Int<N, B, OM>> for Uint<M, A, OM>
{
    type Error = TryFromIntError;

    fn try_from(value: Int<N, B, OM>) -> Result<Self, Self::Error> {
        uint_try_from_int(value)
    }
}

impl<const N: usize, const B: usize, const M: usize, const A: usize, const OM: u8>
    TryFrom<Uint<N, B, OM>> for Int<M, A, OM>
{
    type Error = TryFromIntError;

    fn try_from(value: Uint<N, B, OM>) -> Result<Self, Self::Error> {
        int_try_from_uint(value)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> From<bool>
    for Integer<S, N, B, OM>
{
    #[inline]
    fn from(small: bool) -> Self {
        Self::cast_from(small)
    }
}

impl<const N: usize, const B: usize, const OM: u8> TryFrom<char> for Uint<N, B, OM> {
    type Error = TryFromCharError;

    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        <Self as TryFrom<u32>>::try_from(u32::from(c)).map_err(|_| TryFromCharError(()))
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> FromStr for Integer<S, N, B, OM> {
    type Err = ParseIntError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(src, 10)
    }
}

#[cfg(test)]
mod tests {
    use crate::test;
    use crate::test::cast_types::*;
    use alloc::string::ToString;

    crate::test::test_all! {
        testing unsigned;

        test::test_tryfrom_same_sign!(UTest; TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10);

        test::test_from! {
            function: <UTest as TryFrom>::try_from,
            from_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10) // TODO: when we can use TryFrom for conversions between bnum ints, we can just add the list of test types here, same as in the casting tests
        }
        test::test_into! {
            function: <UTest as TryInto>::try_into,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10)
        }

        #[test]
        fn out_of_range_conversion_error() {
            let invalid = -1i32;

            let test_msg = UTest::try_from(invalid).unwrap_err().to_string();

            assert!(test_msg.contains("out of range integral type conversion attempted"));
        }
    }

    #[test]
    fn invalid_char_conversion_error() {
        type U13s = crate::t!(U13s);

        let invalid_char = char::from_u32(0x2764).unwrap();

        let test_msg = U13s::try_from(invalid_char).unwrap_err().to_string();

        assert!(test_msg.contains("unicode code point out of range"));
    }

    crate::test::test_all! {
        testing signed;

        test::test_tryfrom_same_sign!(ITest; TestInt1, TestInt2, TestInt3, TestInt4, TestInt5, TestInt6, TestInt7, TestInt8, TestInt9, TestInt10);

        test::test_from! {
            function: <ITest as TryFrom>::try_from,
            from_types: (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, bool, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10)
        }
        test::test_into! {
            function: <ITest as TryInto>::try_into,
            into_types: (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, TestUint1, TestUint2, TestUint3, TestUint4, TestUint5, TestUint6, TestUint7, TestUint8, TestUint9, TestUint10)
        }
    }
}

#[cfg(test)]
mod double_custom_bit_width_convert_tests {
    use crate::Integer;
    use crate::test::BitInt;
    use crate::macros::get_size_params_from_bits;

    macro_rules! test_double_custom_bit_width_convert {
        ($($from: literal => $to: literal), *) => {
            paste::paste! {
                $(
                    quickcheck::quickcheck! {
                        fn [<quickcheck_cast_from_u_ $from _to $to _bits>](v: BitInt<false, $from>) -> bool {
                            let w = Integer::from(v);
                            let a = BitInt::<false, $to>::try_from(&v);
                            let b = Integer::<false, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::try_from(&w);

                            let w = Integer::from(v);
                            let c = BitInt::<true, $to>::try_from(&v);
                            let d = Integer::<true, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::try_from(&w);
                            crate::test::test_eq(a, b) && crate::test::test_eq(c, d)
                        }

                        fn [<quickcheck_cast_from_i_ $from _to $to _bits>](v: BitInt<true, $from>) -> bool {
                            let w = Integer::from(v);
                            let a = BitInt::<false, $to>::try_from(&v);
                            let b = Integer::<false, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::try_from(&w);

                            let w = Integer::from(v);
                            let c = BitInt::<true, $to>::try_from(&v);
                            let d = Integer::<true, {get_size_params_from_bits($to).0}, {get_size_params_from_bits($to).1}>::try_from(&w);
                            crate::test::test_eq(a, b) && crate::test::test_eq(c, d)
                        }
                    }
                )*
            }
        };
    }

    test_double_custom_bit_width_convert!(
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
        128 => 127,
        128 => 129,
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
