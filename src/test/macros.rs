macro_rules! test_bignum {
    {
        function: $($unsafe: ident)? <$primitive: ty $(as $Trait: ident $(<$($gen: ty), +>)?)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
        $(, skip: $skip: expr)?
    } => {
        paste::paste! {
            quickcheck::quickcheck! {
                #[allow(non_snake_case)]
                fn [<quickcheck_ $primitive _ $($Trait _ $($($gen _) +)?)? $function>]($($param : $ty), *) -> quickcheck::TestResult {
                    $(if $skip {
                        return quickcheck::TestResult::discard();
                    })?

                    let (big, primitive) = $($unsafe)? {
                        crate::test::results!(<$primitive $(as $Trait $(<$($gen), *>)?)?>::$function ($($($re)? TryInto::try_into($param).expect("test argument conversion failed")), *))
                    };

                    quickcheck::TestResult::from_bool(big == primitive)
                }
            }
        }
    };
    {
        function: <$primitive: ty> :: $function: ident, // need to handle the case with and without trait separately due to repetition of the cases arguments
        cases: [
            $(($($(ref $re2: tt)? $arg: expr), *)), *
        ]
    } => {
        paste::paste! {
            #[test]
            fn [<cases_ $primitive _ $function>]() {
                $(
                    let (big, primitive) = crate::test::results!(<$primitive> :: $function ($($($re2)? TryInto::try_into($arg).expect("test argument conversion failed")), *));
                    assert_eq!(big, primitive, "failed cases assertion with inputs {:?}", ($($arg), *));
                )*
            }
        }
    };
    {
        function: <$primitive: ty as $Trait: ident> :: $function: ident,
        cases: [
            $(($($(ref $re2: tt)? $arg: expr), *)), *
        ]
    } => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[test]
            fn [<cases_ $primitive _ $Trait _ $function>]() {
                $(
                    let (big, primitive) = crate::test::results!(<$primitive as $Trait> :: $function ($($($re2)? TryInto::try_into($arg).expect("test argument conversion failed")), *));
                    assert_eq!(big, primitive, "failed cases assertion with inputs {:?}", ($($arg), *));
                )*
            }
        }
    };
    {
        function: <$primitive: ty $(as $Trait: ident)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
        $(, skip: $skip: expr)?
        , cases: [
            $(($($(ref $re2: tt)? $arg: expr), *)), *
        ]
    } => {
        crate::test::test_bignum! {
            function: <$primitive $(as $Trait)?> :: $function,
            cases: [
                $(($($(ref $re2)? $arg), *)), *
            ]
        }
        crate::test::test_bignum! {
            function: <$primitive $(as $Trait)?> :: $function ($($param : $(ref $re)? $ty), *)
            $(, skip: $skip)?
        }
    };
}

pub(crate) use test_bignum;

macro_rules! results {
    (<$primitive: ty $(as $Trait: ty)?> :: $function: ident ($($arg: expr), *)) => {
        paste::paste! {
            {
                let big_result = <[<$primitive:upper>] $(as $Trait)?>::$function(
                    $($arg), *
                );
                let prim_result = <$primitive $(as $Trait)?>::$function(
                    $($arg), *
                );

                use crate::test::TestConvert;
                (TestConvert::into(big_result), TestConvert::into(prim_result))
            }
        }
    };
}

pub(crate) use results;

macro_rules! test_tryfrom_same_sign {
    ($primitive: ty; $($From: ty), *) => {
        paste::paste! {
            $(
                quickcheck::quickcheck! {
                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $primitive _TryFrom_ $From _try_from>](from: $From) -> bool {
                        let big: Result<[<$primitive:upper>], _> = <[<$primitive:upper>] as TryFrom<_>>::try_from(&from);
                        let primitive: Result<$primitive, _> = <$primitive>::try_from(from);

                        test::convert::test_eq(big, primitive)
                    }
                }
            )*
        }
    };
}

pub(crate) use test_tryfrom_same_sign;

macro_rules! test_from {
    {
        function: <$primitive: ty as $Trait: ident>:: $name: ident,
        from_types: ($($from_type: ty), *)
    } => {
        $(
            crate::test::test_bignum! {
                function: < $primitive as $Trait<$from_type> >::$name(from: $from_type)
            }
        )*
    }
}

pub(crate) use test_from;

macro_rules! test_into {
    {
        function: <$primitive: ty as $Trait: ident>:: $name: ident,
        into_types: ($($into_type: ty), *)
    } => {
        paste::paste! {
            $(
                crate::test::test_bignum! {
                    function: < $primitive as $Trait<$into_type> >::$name(from: $primitive)
                }
            )*
        }
    }
}

pub(crate) use test_into;

#[cfg(feature = "alloc")]
#[derive(Clone, Copy, Debug)]
pub struct Radix<const MAX: u32>(pub u32);

#[cfg(feature = "alloc")]
impl<const MAX: u32> quickcheck::Arbitrary for Radix<MAX> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let radix = (u32::arbitrary(g) % (MAX - 2)) + 2;
        Self(radix)
    }
}

impl<const MAX: u32> From<Radix<MAX>> for u32 {
    fn from(r: Radix<MAX>) -> Self {
        r.0
    }
}

#[cfg(feature = "alloc")]
macro_rules! quickcheck_from_to_radix {
    ($primitive: ty, $name: ident, $max: expr) => {
        paste::paste! {
            quickcheck::quickcheck! {
                fn [<quickcheck_from_to_ $name>](u: $primitive, radix: crate::test::Radix<$max>) -> quickcheck::TestResult {
                    use crate::cast::CastFrom;

                    let radix = radix.0;
                    let u = <[<$primitive:upper>]>::cast_from(u);
                    let v = u.[<to_ $name>](radix as u32);
                    let u1 = <[<$primitive:upper>]>::[<from_ $name>](&v, radix as u32).unwrap_or(!u);
                    quickcheck::TestResult::from_bool(u == u1)
                }
            }
        }
    }
}

#[cfg(feature = "alloc")]
pub(crate) use quickcheck_from_to_radix;

macro_rules! debug_skip {
    ($skip: expr) => {
        crate::overflow::GLOBAL_OVERFLOW_CHECKS && $skip
    };
}

pub(crate) use debug_skip;

#[cfg(feature = "alloc")]
macro_rules! quickcheck_from_str_radix {
    { $primitive: ident, $sign1: literal | $sign2: literal } => {
        quickcheck::quickcheck! {
            fn quickcheck_from_str_radix(buf: crate::test::U8ArrayWrapper<{<$primitive>::BITS as usize / 4}>, radix: crate::test::Radix<36>, leading_sign: bool) -> quickcheck::TestResult {
                use alloc::string::String;

                let radix = radix.0;

                fn byte_to_char(b: u8) -> char {
                    if b < 10 {
                        (b + 48) as char
                    } else {
                        (b + 87) as char
                    }
                }

                let leading_sign = if leading_sign {
                    $sign1
                } else {
                    $sign2
                };

                let mut s2 = buf.0.into_iter().map(|b| byte_to_char(b % radix as u8)).collect::<String>();
                s2.insert_str(0, leading_sign);

                let (big, primitive) = crate::test::results!(<$primitive>::from_str_radix(&s2, radix as u32));

                // let parsed =

                quickcheck::TestResult::from_bool(big == primitive)
            }
        }
    }
}

#[cfg(feature = "alloc")]
pub(crate) use quickcheck_from_str_radix;

#[cfg(feature = "alloc")]
macro_rules! quickcheck_from_str {
    ($primitive: ty) => {
        quickcheck::quickcheck! {
            fn quickcheck_from_str(n: $primitive) -> bool {
                use crate::alloc::string::ToString;
                use core::str::FromStr;

                let s = n.to_string();
                let (big, primitive) = crate::test::results!(<$primitive>::from_str(&s));

                big == primitive
            }
        }
    };
}

#[cfg(feature = "alloc")]
pub(crate) use quickcheck_from_str;

macro_rules! is_prefix_signed {
    (i) => {
        true
    };
    (u) => {
        false
    };
    (f) => {
        true
    }
}

pub(crate) use is_prefix_signed;

macro_rules! test_width_and_sign {
    ($bits: expr, $prefix: ident $(, $float: literal)?; $($s: tt) * ) => {
        paste::paste! {
            mod [<$prefix $bits>] {
                #[allow(unused_imports)]
                use super::*;

                #[allow(non_camel_case_types, unused)]
                pub type utest = [<u $bits>];

                #[allow(non_camel_case_types, unused)]
                pub type itest = [<i $bits>];

                #[allow(non_camel_case_types, unused)]
                pub type UTEST = crate::Uint<{ crate::literal_parse::get_size_params_from_bits($bits).0 }, { crate::literal_parse::get_size_params_from_bits($bits).1 }>;

                #[allow(non_camel_case_types, unused)]
                pub type ITEST = crate::Int<{ crate::literal_parse::get_size_params_from_bits($bits).0 }, { crate::literal_parse::get_size_params_from_bits($bits).1 }>;

                #[allow(non_camel_case_types, unused)]
                pub type stest = [<$prefix $bits>];

                #[allow(non_camel_case_types, unused)]
                pub type STEST = crate::Integer<{ crate::test::is_prefix_signed!($prefix) }, { crate::literal_parse::get_size_params_from_bits($bits).0 }, { crate::literal_parse::get_size_params_from_bits($bits).1 }>;

                $(
                    #[allow(non_camel_case_types)]
                    #[cfg(feature = $float)]
                    pub type ftest = [<f $bits>];

                    #[cfg(feature = $float)]
                    #[allow(non_camel_case_types)]
                    pub type FTEST = crate::float::Float<{core::mem::size_of::<ftest>()}, {ftest::MANTISSA_DIGITS as usize - 1}>;
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
                    
                    #[allow(non_camel_case_types, unused)]
                    pub type utest = crate::test::BitInt<false, $bits>;

                    #[allow(non_camel_case_types, unused)]
                    pub type itest = crate::test::BitInt<true, $bits>;

                    #[allow(non_camel_case_types, unused)]
                    pub type UTEST = crate::Uint<{ crate::literal_parse::get_size_params_from_bits($bits).0 }, { crate::literal_parse::get_size_params_from_bits($bits).1 }>;

                    #[allow(non_camel_case_types, unused)]
                    pub type ITEST = crate::Int<{ crate::literal_parse::get_size_params_from_bits($bits).0 }, { crate::literal_parse::get_size_params_from_bits($bits).1 }>;

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
    { testing unsigned; $($s: tt) * } => {
        // for unsigned specific tests
        mod unsigned_only {
            #[allow(unused_imports)]
            use super::*;

            crate::test::test_width_and_sign!(16, u; $($s)*);
            crate::test::test_width_and_sign!(32, u; $($s)*);
            crate::test::test_width_and_sign!(64, u; $($s)*);
            crate::test::test_width_and_sign!(128, u; $($s)*);
        }
    };
    { testing signed; $($s: tt) * } => {
        // for signed specific tests
        mod signed_only {
            #[allow(unused_imports)]
            use super::*;

            crate::test::test_width_and_sign!(16, i; $($s)*);
            crate::test::test_width_and_sign!(32, i; $($s)*);
            crate::test::test_width_and_sign!(64, i; $($s)*);
            crate::test::test_width_and_sign!(128, i; $($s)*);
        }
    };
    { testing integers; $($s: tt) * } => {
        // for unsigned and signed tests
        crate::test::test_width_and_sign!(16, u; $($s)*);
        crate::test::test_width_and_sign!(32, u; $($s)*);
        crate::test::test_width_and_sign!(64, u; $($s)*);
        crate::test::test_width_and_sign!(128, u; $($s)*);

        crate::test::test_width_and_sign!(16, i; $($s)*);
        crate::test::test_width_and_sign!(32, i; $($s)*);
        crate::test::test_width_and_sign!(64, i; $($s)*);
        crate::test::test_width_and_sign!(128, i; $($s)*);
    };
    { testing floats; $($s: tt) * } => {
        // #[cfg(nightly)]
        // crate::test::test_width_and_sign!(16, f, "float"; $($s)*);

        crate::test::test_width_and_sign!(32, f, "float"; $($s)*);
        crate::test::test_width_and_sign!(64, f, "float"; $($s)*);

        // #[cfg(nightly)]
        // crate::test::test_width_and_sign!(128, f, "float"; $($s)*);
    };
}

pub(crate) use test_all;
