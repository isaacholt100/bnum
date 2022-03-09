#[allow(unused)]
macro_rules! test_big_num {
    /*{
        big: $big_type: ty,
        primitive: $primitive: ty,
        function: $name: ident,
        cases: [
            $($method: ident ($($arg: expr), *) ;) *
        ],
        quickcheck: $method2: ident ($($param: ident : $ty: ty), *)
    } => {
        test_big_num! {
            big: $big_type,
            primitive: $primitive,
            function: $name,
            method: {
                $($method ($($arg), *) ;) *
            },
            quickcheck: $method2 ($($param : $ty), *),
            converter: Into::into
        }
    };*/
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        function: $name: ident,
        $(cases: [
            $(($($arg: expr), *)), *
        ],)?
        $(quickcheck: ($($param: ident : $ty: ty), *),
        $(
        quickcheck_skip: $skip: expr,)?)?
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
                assert_eq!(big_result, $converter(prim_result));
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

                    quickcheck::TestResult::from_bool(big_result == $converter(prim_result))
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
        function: <$Trait: ty> :: $name: ident ($($param: ident : $ty: ty), *)
        $(,quickcheck_skip: $skip: expr)?
    } => {
        paste::paste! {
            fn [<$name _converter>]<T: Into<$big_type>>(f: T) -> $big_type {
                f.into()
            }
            crate::test::test_trait! {
                big: $big_type,
                primitive: $primitive,
                test_name: $name,
                function: <$Trait>::$name,
                quickcheck: ($($param : $ty), *)
                $(,quickcheck_skip: $skip)?
                ,converter: [<$name _converter>]
            }
        }
    }
}

pub(crate) use test_op;

/*
pub trait As<T> {
    fn as_(a: T) -> Self;
}

impl As<u128> for f64 {
    fn as_(a: u128) -> Self {
        a as Self
    }
}

impl As<u128> for f32 {
    fn as_(a: u128) -> Self {
        a as Self
    }
}

impl As<f64> for u128 {
    fn as_(a: f64) -> Self {
        a as Self
    }
}

impl As<f32> for u128 {
    fn as_(a: f32) -> Self {
        a as Self
    }
}

impl As<u128> for crate::U128 {
    fn as_(a: u128) -> Self {
        Self::from(a)
    }
}

#[allow(unused)]
macro_rules! test_float_conv {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        test_name: $test_name: ident,
        function: <$Trait: ty>::$name: ident,
        from: $from: ty,
        converter: $converter: expr
    } => {
        paste::paste! {
            quickcheck::quickcheck! {
                fn [<quickcheck_ $test_name>](a: $from) -> quickcheck::TestResult {
                    let big_result = <$big_type as $Trait>::$name(crate::test::As::as_(a));
                    let prim_result = <$primitive as $Trait>::$name(crate::test::As::as_(a));

                    quickcheck::TestResult::from_bool($converter(big_result) == $converter(prim_result))
                }
            }
        }
    }
}

pub(crate) use test_float_conv;*/

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