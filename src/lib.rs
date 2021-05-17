#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(const_raw_ptr_deref)]
//#![feature(const_trait_impl)]
#![feature(const_panic)]
#![feature(const_fn)]
#![feature(const_option)]
#![feature(const_maybe_uninit_assume_init)]
#![feature(const_intrinsic_copy)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_as_ptr)]
#![feature(const_ptr_offset)]

fn test_into_converter<U, T: Into<U>>(x: T) -> U {
    x.into()
}

macro_rules! expect {
    ($option: expr, $msg: expr) => {
        match $option {
            Some(value) => value,
            None => panic!($msg),
        }
    }
}

#[allow(unused)]
macro_rules! test {
    {
        big: $big_type: tt,
        primitive: $primitive: tt,
        test_name: $test_name: ident,
        method: $method: ident ($($arg: expr), *)
    } => {
        test! {
            big: $big_type,
            primitive: $primitive,
            test_name: $test_name,
            method: $method ($($arg), *),
            converter: crate::test_into_converter
        }
    };
    {
        big: $big_type: tt,
        primitive: $primitive: tt,
        test_name: $test_name: ident,
        method: $method: ident ($($arg: expr), *),
        converter: $converter: expr
    } => {
        #[test]
        fn $test_name() {
            let prim_result = <$primitive>::$method(
                $($arg.into()),*
            );
            let big_result = <$big_type>::$method(
                $($arg.into()), *
            );
            assert_eq!(big_result, $converter(prim_result));
        }
    }
}

mod uint;
mod int;
mod iint;
mod tryops;
mod sign;
mod main;
mod arch;
mod digit;
mod bound;
mod int_test;

pub use iint::BIint;
pub use sign::Sign;
pub use uint::BUint;
pub use int::Bint;

#[allow(unused)]
type I128 = int::Bint::<{(128 / digit::DIGIT_BITS) - 1}>;

type I128Test = int_test::BintTest::<2>;

#[allow(unused)]
type U128 = BUint::<{128 / digit::DIGIT_BITS}>;

pub type U256 = BUint::<{256 / digit::DIGIT_BITS}>;
pub type U512 = BUint::<{512 / digit::DIGIT_BITS}>;
pub type U1024 = BUint::<{1024 / digit::DIGIT_BITS}>;
pub type U2048 = BUint::<{2048 / digit::DIGIT_BITS}>;
pub type U4096 = BUint::<{4096 / digit::DIGIT_BITS}>;
pub type U8192 = BUint::<{8192 / digit::DIGIT_BITS}>;

pub type ParseIntError = &'static str;
pub type TryFromIntError = &'static str;
pub type OperationError = &'static str;