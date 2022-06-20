macro_rules! test_bignum {
	{
		function: <$primitive: ty $(as $Trait: ident $(<$($gen: ty), *>)?)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
		$(, skip: $skip: expr)?
	} => {
		paste::paste! {
			quickcheck::quickcheck! {
				#[allow(non_snake_case)]
				fn [<quickcheck_ $primitive _ $($Trait _ $($($gen _) *)?)? $function>]($($param : $ty), *) -> quickcheck::TestResult {
					$(if $skip {
						return quickcheck::TestResult::discard();
					})?
	
					let (big, primitive) = crate::test::results!(<$primitive $(as $Trait $(<$($gen), *>)?)?>::$function ($($($re)? Into::into($param)), *));
	
					quickcheck::TestResult::from_bool(big == primitive)
				}
			}
		}
	};
	{
		function: <$primitive: ty $(as $Trait: ty)?> :: $function: ident,
		cases: [
            $(($($arg: expr), *)), *
        ]
	} => {
		paste::paste! {
			#[test]
			fn [<cases_ $primitive _ $function>]() {
				$(
					let (big, primitive) = crate::test::results!(<$primitive> :: $function ($(Into::into($arg)), *));
					assert_eq!(big, primitive);
				)*
			}
		}
	};
	{
		function: <$primitive: ty $(as $Trait: ty)?> :: $function: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *)
		$(, skip: $skip: expr)?
		, cases: [
            $(($($arg: expr), *)), *
        ]
	} => {
		crate::test::test_bignum! {
			function: <$primitive $(as $Trait)?> :: $function,
			cases: [
				$(($($arg), *)), *
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
				let big_result = <crate::[<$primitive:upper>] $(as $Trait)?>::$function(
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

macro_rules! quickcheck_from_to_radix {
    ($primitive: ty, $name: ident, $max: expr) => {
        paste::paste! {
            quickcheck::quickcheck! {
                fn [<quickcheck_from_to_ $name>](u: $primitive, radix: u8) -> quickcheck::TestResult {
                    #[allow(unused_comparisons)]
                    if !((2..=$max).contains(&radix)) {
                        return quickcheck::TestResult::discard();
                    }
                    let u = <crate::[<$primitive:upper>]>::from(u);
                    let v = u.[<to_ $name>](radix as u32);
                    let u1 = <crate::[<$primitive:upper>]>::[<from_ $name>](&v, radix as u32).unwrap();
                    quickcheck::TestResult::from_bool(u == u1)
                }
            }
        }
    }
}

pub(crate) use quickcheck_from_to_radix;

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

pub(crate) use test_fmt;

macro_rules! debug_skip {
	($skip: expr) => {
		{
			#[cfg(debug_assertions)]
			let skip = $skip;
			#[cfg(not(debug_assertions))]
			let skip = false;

			skip
		}
	};
}

pub(crate) use debug_skip;