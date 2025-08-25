macro_rules! test_bignum {
    {
        function: $($unsafe: ident)? <$primitive: ty $(as $Trait: ident $(<$($gen: ty), *>)?)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
        $(, skip: $skip: expr)?
    } => {
        paste::paste! {
            quickcheck::quickcheck! {
                #[allow(non_snake_case)]
                fn [<quickcheck_ $primitive _ $($Trait _ $($($gen _) *)?)? $function>]($($param : $ty), *) -> quickcheck::TestResult {
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
        function: <$primitive: ty> :: $function: ident,
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
        function: <$primitive: ty as $Trait: ty> :: $function: ident,
        cases: [
            $(($($(ref $re2: tt)? $arg: expr), *)), *
        ]
    } => {
        paste::paste! {
            #[test]
            fn [<cases_ $primitive _ $function>]() {
                $(
                    let (big, primitive) = crate::test::results!(<$primitive as $Trait> :: $function ($($($re2)? TryInto::try_into($arg).expect("test argument conversion failed")), *));
                    assert_eq!(big, primitive);
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

macro_rules! test_btryfrom {
    ($primitive: ty; $($From: ty), *) => {
        paste::paste! {
            $(
                quickcheck::quickcheck! {
                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $primitive _BTryFrom_ $From _try_from>](from: $From) -> bool {
                        let big: Result<[<$primitive:upper>], _> = <[<$primitive:upper>] as BTryFrom<_>>::try_from(from);
                        let primitive: Result<$primitive, _> = <$primitive>::try_from(from);
                        test::convert::test_eq(big, primitive)
                    }
                }
            )*
        }
    };
}

pub(crate) use test_btryfrom;

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
        crate::OVERFLOW_CHECKS && $skip
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

macro_rules! test_types {
    ($bits: expr) => {
        paste::paste! {
            #[allow(non_camel_case_types, unused)]
            pub type utest = [<u $bits>];

            #[cfg(feature = "signed")]
            #[allow(non_camel_case_types, unused)]
            pub type itest = [<i $bits>];

            // #[allow(non_camel_case_types, unused)]
            // #[cfg(feature = "float")]
            // pub type ftest = [<f $bits>];

            #[allow(non_camel_case_types, unused)]
            pub type UTEST = crate::Uint<{ $bits / 8 }>;

            #[cfg(feature = "signed")]
            #[allow(non_camel_case_types, unused)]
            pub type ITEST = crate::Int<{ $bits / 8 }>;

            // #[cfg(feature = "float")]
            // #[allow(non_camel_case_types, unused)]
            // pub type FTEST = crate::float::Float<{core::mem::size_of::<ftest>()}, {ftest::MANTISSA_DIGITS as usize - 1}>;
        }
    };
}

pub(crate) use test_types;

macro_rules! old_bnum_test_types {
    ($bits: expr) => {
        paste::paste! {
            #[allow(non_camel_case_types, unused)]
            pub type utest = bnum_old::BUintD8<{ $bits / 8 }>;

            #[cfg(feature = "signed")]
            #[allow(non_camel_case_types, unused)]
            pub type itest = bnum_old::BIntD8<{ $bits / 8 }>;

            #[allow(non_camel_case_types, unused)]
            pub type UTEST = crate::Uint<{ $bits / 8 }>;

            #[cfg(feature = "signed")]
            #[allow(non_camel_case_types, unused)]
            pub type ITEST = crate::Int<{ $bits / 8 }>;
        }
    };
}

pub(crate) use old_bnum_test_types;

macro_rules! test_against_old_types {
    ($bits: literal; $($s: tt) * ) => {
        paste::paste! {
            mod [<bits $bits>] {
                #[allow(unused_imports)]
                use super::*;
                crate::test::old_bnum_test_types!($bits);

                $($s)*
            }
        }
    };
}

pub(crate) use test_against_old_types;

// since we're using u128 digit iterations for performance, testing on the primitives isn't enough, because there will only be one u128 digit. so test against previous version of bnum, which are almost certainly completely correct due to the testing of that version, and the fact they use different sized digits (u8 not u128)
// but we only need to do this for methods that use the u128 digits (anywhere where .as_wide_digits() or .as_wide_digits_mut() is used)
// test for a few different widths, with different widths of the truncated last digit (Self::BITS % 128)
macro_rules! test_all_widths_against_old_types {
    { $($s: tt) * } => {
        mod old_comparison_tests {
            #[allow(unused_imports)]
            use super::*;

            crate::test::test_against_old_types!(56; $($s)*); // 0 * 128 + 56
            crate::test::test_against_old_types!(144; $($s)*); // 1 * 128 + 16
            crate::test::test_against_old_types!(264; $($s)*); // 2 * 128 + 8
            crate::test::test_against_old_types!(488; $($s)*); // 3 * 128 + 104
            crate::test::test_against_old_types!(584; $($s)*); // 4 * 128 + 72
            crate::test::test_against_old_types!(640; $($s)*); // 5 * 128 + 0
            crate::test::test_against_old_types!(1024; $($s)*); // 8 * 128 + 0
        }
    }
}

pub(crate) use test_all_widths_against_old_types;

macro_rules! test_all_widths {
    { $($s: tt) * } => {
        mod tests {
            #[allow(unused_imports)]
            use super::*;

            mod bits16 {
                #[allow(unused_imports)]
                use super::*;

                // #[cfg(not(feature = "nightly"))]
                #[allow(non_camel_case_types, unused)]
                type f16 = f32; // this is a bit of a hack, if nightly compiler not being used then we can't use f16s, so we'll declare it to be an f32 instead
                crate::test::test_types!(16);

                $($s)*
            }

            mod bits32 {
                #[allow(unused_imports)]
                use super::*;

                crate::test::test_types!(32);

                $($s)*
            }
            mod bits64 {
                #[allow(unused_imports)]
                use super::*;

                crate::test::test_types!(64);

                $($s)*
            }
            mod bits128 {
                #[allow(unused_imports)]
                use super::*;

                // #[cfg(not(feature = "nightly"))]
                #[allow(non_camel_case_types, unused)]
                type f128 = f64; // same hack for f128 as for f16
                crate::test::test_types!(128);

                $($s)*
            }
        }
    }
}

pub(crate) use test_all_widths;
