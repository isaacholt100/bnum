macro_rules! test_bignum {
    {
        function: $($unsafe: ident)? <$TestType: ty $(as $Trait: ident $(<$($gen: ty), +>)?)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
        $(, cmp: input.$idx: literal)?
        $(, skip: $skip: expr)?
    } => {
        paste::paste! {
            quickcheck::quickcheck! {
                #[allow(non_snake_case)]
                fn [<quickcheck_ $TestType _ $($Trait _ $($($gen _) +)?)? $function>]($($param : $ty), *) -> quickcheck::TestResult {
                    $(if $skip {
                        return quickcheck::TestResult::discard();
                    })?

                    let (actual, expected) = $($unsafe)? {
                        crate::test::results!(<$TestType $(as $Trait $(<$($gen), *>)?)?>::$function ($($($re)? $param.try_into().expect("test argument conversion failed")), *))
                    };

                    quickcheck::TestResult::from_bool(actual == expected)
                }
            }
        }
    };
    {
        function: <$TestType: ty> :: $function: ident, // need to handle the case with and without trait separately due to repetition of the cases arguments
        $(, cmp: input.$idx: literal)?
        cases: [
            $(($($(ref $re2: tt)? $arg: expr), *)), *
        ]
    } => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[test]
            fn [<cases_ $TestType _ $function>]() {
                $(
                    let (actual, expected) = crate::test::results!(<$TestType> :: $function ($($($re2)? TryInto::try_into($arg).expect("test argument conversion failed")), *));

                    assert_eq!(actual, expected, "failed cases assertion with inputs {:?}", ($($arg), *));
                )*
            }
        }
    };
    {
        function: <$TestType: ty as $Trait: ident> :: $function: ident,
        $( , cmp: input.$idx: literal )?
        cases: [
            $(($($(ref $re2: tt)? $arg: expr), *)), *
        ],
    } => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[test]
            fn [<cases_ $TestType _ $Trait _ $function>]() {
                $(
                    let (actual, expected) = crate::test::results!(<$TestType as $Trait> :: $function ($($($re2)? TryInto::try_into($arg).expect("test argument conversion failed")), *));

                    assert_eq!(actual, expected, "failed cases assertion with inputs {:?}", ($($arg), *));
                )*
            }
        }
    };
    {
        function: <$TestType: ty $(as $Trait: ident)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
        $(, skip: $skip: expr)?
        , cases: [
            $(($($(ref $re2: tt)? $arg: expr), *)), *
        ]
    } => {
        crate::test::test_bignum! {
            function: <$TestType $(as $Trait)?> :: $function,
            cases: [
                $(($($(ref $re2)? $arg), *)), *
            ]
        }
        crate::test::test_bignum! {
            function: <$TestType $(as $Trait)?> :: $function ($($param : $(ref $re)? $ty), *)
            $(, skip: $skip)?
        }
    };
}

pub(crate) use test_bignum;

macro_rules! results {
    (<$TestType: ty $(as $Trait: ty)?> :: $function: ident ($($arg: expr), *) $(, cmp: input.$idx: literal)?) => {
        paste::paste! {
            {
                let actual = <$TestType $(as $Trait)?>::$function(
                    $($arg), *
                );
                let expected = <[<$TestType Base>] $(as $Trait)?>::$function(
                    $($arg), *
                );

                use crate::test::TestConvert;
                (TestConvert::into(actual), TestConvert::into(expected))
            }
        }
    };
}

pub(crate) use results;

macro_rules! mutable_var {
    (&mut $arg: ident) => {
        mut $arg
    };
    (& $arg: ident) => {
        $arg
    };
    ($arg: ident) => {
        $arg
    };
}

pub(crate) use mutable_var;

macro_rules! results2 {
    (<$TestType: ty $(as $Trait: ty)?> :: $function: ident ($($a: tt $($b: ident $($c: ident)?)?), *) $(, cmp: input.$idx: literal)?) => {
        paste::paste! {
            {
                // use crate::test::mutable_var;

                // let (
                //     $(mutable_var!($a $($b $($c)?)?)),*
                // ) = ($($arg.try_into().expect("test argument conversion failed")), *);

                // let (
                //     $(mutable_var!([<$a $($b $($c)?)? _base>])), *
                // ) = ($($arg.try_into().expect("test argument conversion failed")), *);

                // let actual = <$TestType $(as $Trait)?>::$function(
                //     $($a $($b $($c)?)?), *
                // );
                // let expected = <[<$TestType Base>] $(as $Trait)?>::$function(
                //     $([<$a $($b $($c)?)? _base>]), *
                // );

                // use crate::test::TestConvert;
                // (TestConvert::into(actual), TestConvert::into(expected))
            }
        }
    };
}

pub(crate) use results2;

macro_rules! test_tryfrom_same_sign {
    ($TestType: ty; $($From: ty), *) => {
        paste::paste! {
            $(
                quickcheck::quickcheck! {
                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $TestType _TryFrom_ $From _try_from>](from: $From) -> bool {
                        let actual: Result<$TestType, _> = <$TestType as TryFrom<_>>::try_from(&from);
                        let expected: Result<[<$TestType Base>], _> = <[<$TestType Base>] as TryFrom<_>>::try_from(from);

                        test::convert::test_eq(actual, expected)
                    }
                }
            )*
        }
    };
}

pub(crate) use test_tryfrom_same_sign;

macro_rules! test_from {
    {
        function: <$TestType: ty as $Trait: ident>:: $name: ident,
        from_types: ($($from_type: ty), *)
    } => {
        $(
            crate::test::test_bignum! {
                function: < $TestType as $Trait<$from_type> >::$name(from: $from_type)
            }
        )*
    }
}

pub(crate) use test_from;

macro_rules! test_into {
    {
        function: <$TestType: ty as $Trait: ident>:: $name: ident,
        into_types: ($($into_type: ty), *)
    } => {
        paste::paste! {
            $(
                crate::test::test_bignum! {
                    function: < $TestType as $Trait<$into_type> >::$name(from: [<$TestType Base>])
                }
            )*
        }
    }
}

pub(crate) use test_into;

macro_rules! primitive_with_overflow_behaviour {
    ($primitive: ident) => {
        $primitive
    };
    ($primitive: ident, w) => {
        core::num::Wrapping<$primitive>
    };
    ($primitive: ident, s) => {
        core::num::Saturating<$primitive>
    };
}

pub(crate) use primitive_with_overflow_behaviour;

macro_rules! test_width_and_sign {
    ($bits: expr, $prefix: ident $($overflow_mode: ident)? $(, $float: literal)?; $($s: tt) * ) => {
        paste::paste! {
            mod [<$prefix $bits>] {
                #[allow(unused_imports)]
                use super::*;

                use crate::t;

                use crate::test::primitive_with_overflow_behaviour;

                #[allow(unused)]
                pub type UTestBase = primitive_with_overflow_behaviour!( [<u $bits>] $(, $overflow_mode)? );

                #[allow(unused)]
                pub type ITestBase = primitive_with_overflow_behaviour!( [<i $bits>] $(, $overflow_mode)? );

                #[allow(unused)]
                pub type STestBase = primitive_with_overflow_behaviour!( [< $prefix $bits>] $(,$overflow_mode)? );

                #[allow(unused)]
                pub type UTest = t!([<U $bits $($overflow_mode)?>]);

                #[allow(unused)]
                pub type ITest = t!([<I $bits $($overflow_mode)?>]);

                #[allow(unused)]
                pub type STest = t!([<$prefix:upper $bits $($overflow_mode)?>]);

                $(
                    #[cfg(feature = $float)]
                    pub type FTestBase = [<f $bits>];

                    #[cfg(feature = $float)]
                    pub type FTest = crate::float::Float<{core::mem::size_of::<FTestBase>()}, {FTestBase::MANTISSA_DIGITS as usize - 1}>;
                )?

                $($s)*
            }
        }
    };
}

pub(crate) use test_width_and_sign;

macro_rules! test_custom_bit_widths {
    ([$($bits: literal), *]; $tests: item ) => {
        paste::paste! {
            $(
                mod [<bits $bits>] {
                    #[allow(unused_imports)]
                    use super::*;

                    use crate::t;
                    
                    #[allow(unused)]
                    pub type UTestBase = crate::test::BitInt<false, $bits>;

                    #[allow(unused)]
                    pub type ITestBase = crate::test::BitInt<true, $bits>;

                    #[allow(unused)]
                    pub type UTest = t!([<U $bits>]);

                    #[allow(unused)]
                    pub type ITest = t!([<I $bits>]);

                    $tests
                }
            )*
        }
    };
}

pub(crate) use test_custom_bit_widths;

macro_rules! test_all_custom_bit_widths {
    { $($s: tt) * } => {
        mod custom_bit_width_tests {
            #[allow(unused_imports)]
            use super::*;

            crate::test::test_custom_bit_widths!(
                [
                    8, 16, 32, 64, 128, 256, 512, // powers of two
                    56, 144, 160, 488, // non-powers of two, multiples of 8
                    2, 3, 4, 5, 7, 9, 11, 15, 23, 31, 43, 59, 61, 73, 89, 97, 101, 113, 129, 173, 255, 289, 366, 402, 422 // non-multiples of 8
                ];
                mod inner { use super::*; $($s)* } // a bit of a hack, maybe we can do this more idiomatically while still being able to pass all the widths as a list
            );
        }
    }
}

pub(crate) use test_all_custom_bit_widths;

macro_rules! test_all {
    { testing unsigned $($overflow_mode: ident)?; $($s: tt) * } => {
        // for unsigned specific tests
        paste::paste! {
            mod [<$($overflow_mode _)? unsigned_only>] {
                #[allow(unused_imports)]
                use super::*;

                crate::test::test_width_and_sign!(16, u $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(32, u $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(64, u $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(128, u $($overflow_mode)?; $($s)*);
            }
        }
    };
    { testing signed $($overflow_mode: ident)?; $($s: tt) * } => {
        // for signed specific tests
        paste::paste! {
            mod [<$($overflow_mode _)? signed_only>] {
                #[allow(unused_imports)]
                use super::*;

                crate::test::test_width_and_sign!(16, i $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(32, i $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(64, i $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(128, i $($overflow_mode)?; $($s)*);
            }
        }
    };
    { testing integers $($overflow_mode: ident)?; $($s: tt) * } => {
        paste::paste! {
            mod [<$($overflow_mode _)? integers>] {
                #[allow(unused_imports)]
                use super::*;

                // for unsigned and signed tests
                crate::test::test_width_and_sign!(16, u $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(32, u $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(64, u $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(128, u $($overflow_mode)?; $($s)*);

                crate::test::test_width_and_sign!(16, i $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(32, i $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(64, i $($overflow_mode)?; $($s)*);
                crate::test::test_width_and_sign!(128, i $($overflow_mode)?; $($s)*);
            }
        }
    };
    { testing floats; $($s: tt) * } => {
        // #[cfg(nightly)]
        // crate::test::test_width_and_sign!(16, f, "float"; $($s)*);

        crate::test::test_width_and_sign!(32, i, "float"; $($s)*);
        crate::test::test_width_and_sign!(64, i, "float"; $($s)*);

        // #[cfg(nightly)]
        // crate::test::test_width_and_sign!(128, f, "float"; $($s)*);
    };
}

pub(crate) use test_all;
