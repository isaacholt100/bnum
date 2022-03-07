#[allow(unused)]
macro_rules! test_big_num {
    /*{
        big: $big_type: ty,
        primitive: $primitive: ty,
        function: $name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        },
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
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        },
        $(quickcheck: $method2: ident ($($param: ident : $ty: ty), *),
        $(
        quickcheck_skip: $skip: expr,)?)?
        converter: $converter: expr
    } => {
        #[test]
        fn $name() {
            $(
                let big_result = <$big_type>::$method(
                    $($arg.into()), *
                );
                let prim_result = <$primitive>::$method(
                    $($arg.into()), *
                );
                assert_eq!(big_result, $converter(prim_result));
            )*
        }
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
        $(quickcheck: $method2: ident ($($param: ident : $ty: ty), *),
        $(
        quickcheck_skip: $skip: expr,)?)?
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
                quickcheck: $name(a: $from_type),
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
                quickcheck: $name(a: $primitive),
                converter: $converter
            }
        })*
    }
}

pub(crate) use test_into;
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