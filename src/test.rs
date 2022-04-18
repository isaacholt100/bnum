#[allow(unused)]
macro_rules! test_big_num {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        function: $name: ident,
        $(cases: [
            $(($($arg: expr), *)), *
        ],)?
        $(
            quickcheck: ($($param: ident : $ty: ty), *),
            $(quickcheck_skip: $skip: expr,)?
            $(big_converter: $big_converter: expr,)?
        )?
        converter: $converter: expr
    } => {
        $(#[test]
        fn $name() {
            $(
                let big_result = <$big_type>::$name(
                    $($arg.into()), *
                );
                let prim_result = <$primitive>::$name(
                    $($arg.into()), *
                );
                assert_eq!(crate::test::TestConvert::into(big_result), crate::test::TestConvert::into(prim_result));
            )*
        })?
        $(paste::paste! {
            quickcheck::quickcheck! {
                fn [<quickcheck_ $name>]($($param : $ty), *) -> quickcheck::TestResult {
                    $(if $skip {
                        return quickcheck::TestResult::discard();
                    })?
                    let big_result = <$big_type>::$name($($param.into()), *);
                    let prim_result = <$primitive>::$name($($param.into()), *);

                    quickcheck::TestResult::from_bool(crate::test::TestConvert::into(big_result) == crate::test::TestConvert::into(prim_result))
                }
            }
        })?
    }
}

pub(crate) use test_big_num;

#[allow(unused)]
macro_rules! test_trait {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        test_name: $test_name: ident,
        function: <$Trait: ty>::$name: ident,
        $(quickcheck: ($($param: ident : $ty: ty), *),
        $(quickcheck_skip: $skip: expr,)?)?
        converter: $converter: expr
    } => {
        $(paste::paste! {
            quickcheck::quickcheck! {
                #[inline]
                fn [<quickcheck_ $test_name>]($($param : $ty), *) -> quickcheck::TestResult {
                    $(if $skip {
                        return quickcheck::TestResult::discard();
                    })?
                    let big_result = <$big_type as $Trait>::$name($($param.into()), *);
                    let prim_result = <$primitive as $Trait>::$name($($param.into()), *);

                    quickcheck::TestResult::from_bool($converter(big_result) == $converter(prim_result))
                }
            }
        })?
    }
}

pub(crate) use test_trait;

#[allow(unused)]
macro_rules! test_from {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        function: <$Trait: ty>:: $name: ident,
        from_types: ($($from_type: ty), *),
        converter: $converter: expr
    } => {
        $(paste::paste! {
            crate::test::test_trait! {
                big: $big_type,
                primitive: $primitive,
                test_name: [<$name _ $from_type>],
                function: <$Trait<$from_type>>::$name,
                quickcheck: (a: $from_type),
                converter: $converter
            }
        })*
    }
}

pub(crate) use test_from;

#[allow(unused)]
macro_rules! test_into {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        function: <$Trait: ty>:: $name: ident,
        from_types: ($($from_type: ty), *),
        converter: $converter: expr
    } => {
        $(paste::paste! {
            crate::test::test_trait! {
                big: $big_type,
                primitive: $primitive,
                test_name: [<$name _ $from_type>],
                function: <$Trait<$from_type>>::$name,
                quickcheck: (a: $primitive),
                converter: $converter
            }
        })*
    }
}

pub(crate) use test_into;

macro_rules! test_op {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        function: <$Trait: ty> :: $name: ident ($($param: ident : $ty: ty), *),
        converter: $converter: expr
        $(,quickcheck_skip: $skip: expr)?
    } => {
        paste::paste! {
            crate::test::test_trait! {
                big: $big_type,
                primitive: $primitive,
                test_name: $name,
                function: <$Trait>::$name,
                quickcheck: ($($param : $ty), *)
                $(,quickcheck_skip: $skip)?
                ,converter: $converter
            }
        }
    }
}

pub(crate) use test_op;

pub fn u32_to_exp(u: u32) -> crate::ExpType {
    u as crate::ExpType
}

#[derive(Clone, Copy)]
pub struct U8ArrayWrapper<const N: usize>([u8; N]);

#[cfg(feature = "nightly")]
impl<const N: usize> U8ArrayWrapper<N> {
    pub fn converter(bytes: [u8; N]) -> [u8; N] {
        bytes
    }
}

impl<const N: usize> From<U8ArrayWrapper<N>> for [u8; N] {
    fn from(a: U8ArrayWrapper<N>) -> Self {
        a.0
    }
}

use quickcheck::{Arbitrary, Gen};

impl Arbitrary for U8ArrayWrapper<16> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u128::arbitrary(g).to_be_bytes())
    }
}

impl Arbitrary for U8ArrayWrapper<8> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u64::arbitrary(g).to_be_bytes())
    }
}

use core::fmt::{Formatter, self, Debug};

impl<const N: usize> Debug for U8ArrayWrapper<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub mod converters {
    pub fn tuple_converter<T, U, V: Into<T>, W: Into<U>>((a, b): (V, W)) -> (T, U) {
        (a.into(), b.into())
    }

    pub fn option_converter<T, U: Into<T>>(o: Option<U>) -> Option<T> {
        o.map(Into::into)
    }
}

macro_rules! quickcheck_from_to_radix {
    ($big: ty, $primitive: ty, $name: ident, $max: expr) => {
        paste::paste! {
            quickcheck::quickcheck! {
                fn [<quickcheck_from_to_ $name>](u: $primitive, radix: u8) -> quickcheck::TestResult {
                    #[allow(unused_comparisons)]
                    if radix < 2 || radix > $max {
                        return quickcheck::TestResult::discard();
                    }
                    let u = $big::from(u);
                    let v = u.[<to_ $name>](radix as u32);
                    let u1 = $big::[<from_ $name>](&v, radix as u32).unwrap();
                    quickcheck::TestResult::from_bool(u == u1)
                }
            }
        }
    }
}

pub(crate) use quickcheck_from_to_radix;

macro_rules! test_cast_from {
    ($big: ty as [$($ty: ty), *]) => {
        paste::paste! {
            quickcheck::quickcheck! {
                $(
                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $big as_ $ty>](i: [<$big:lower>]) -> bool {
                        let big = $big::from(i);
                        let a1: $ty = big.as_();
                        a1 == i.as_()
                    }
                )*
            }
        }
    }
}

pub(crate) use test_cast_from;

macro_rules! test_cast_to {
    ([$($ty: ty), *] as $big: ty) => {
        paste::paste! {
            quickcheck::quickcheck! {
                $(
                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $ty _as_ $big>](i: $ty) -> bool {
                        let big: $big = i.as_();
                        let primitive: [<$big:lower>] = i.as_();
                        big == primitive.into()
                    }
                )*
            }
        }
    }
}

pub(crate) use test_cast_to;

#[allow(unused)]
macro_rules! test_fmt {
    {
        int: $int: ty,
        name: $name: ident,
        format: $format: expr,
        numbers: {
            $($number: expr), *
        }
    } => {
        paste::paste! {
            #[test]
            fn [<$name _format>]() {
                $(
                    let big = <$int>::from($number);
                    assert_eq!(format!(concat!("{:", $format, "}"), big), format!(concat!("{:", $format, "}"), $number));
                    assert_eq!(format!(concat!("{:#", $format, "}"), big), format!(concat!("{:#", $format, "}"), $number));
                )*
            }
            
            quickcheck::quickcheck! {
                fn [<quickcheck_ $name _format>](i: [<$int:lower>]) -> bool {
                    let big = <$int>::from(i);
                    format!(concat!("{:#", $format, "}"), big) == format!(concat!("{:#", $format, "}"), i)
                }
            }
        }
    }
}

#[allow(unused_imports)]
pub(crate) use test_fmt;

use crate::{U128, I128, U64, F64};

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

impl TestConvert for F64 {
    type Output = u64;

    #[inline]
    fn into(self) -> Self::Output {
        unsafe {
            core::mem::transmute(self.to_bits())
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

impl TestConvert for u32 {
    type Output = u32;
    
    fn into(self) -> Self::Output {
        self
    }
}

impl TestConvert for usize {
    type Output = u32;
    
    #[inline]
    fn into(self) -> Self::Output {
        self as u32
    }
}

impl<T: TestConvert, E: Debug> TestConvert for Result<T, E> {
    type Output = <T as TestConvert>::Output;

    #[inline]
    fn into(self) -> Self::Output {
        TestConvert::into(self.unwrap())
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

test_convert_to_self!(core::num::FpCategory, bool);