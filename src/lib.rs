#![allow(incomplete_features)]
#![feature(
    const_generics,
    const_evaluatable_checked,
    const_panic,
    const_maybe_uninit_assume_init,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_ptr,
    const_ptr_offset,
    test
)]
//#![no_std]

#[macro_use]
extern crate alloc;

#[allow(unused)]
fn test_into_converter<U, T: Into<U>>(x: T) -> U {
    x.into()
}

#[allow(unused)]
macro_rules! test {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        test_name: $test_name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        }
    } => {
        test! {
            big: $big_type,
            primitive: $primitive,
            test_name: $test_name,
            method: {
                $($method ($($arg), *) ;) *
            },
            converter: crate::test_into_converter
        }
    };
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        test_name: $test_name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        },
        converter: $converter: expr
    } => {
        #[test]
        fn $test_name() {
            $(
                let prim_result = <$primitive>::$method(
                    $($arg), *
                );
                let big_result = <$big_type>::$method(
                    $($arg.into()), *
                );
                assert_eq!(big_result, $converter(prim_result));
            )*
        }
    }
}

mod uint;
mod arithmetic;
mod digit;
mod iint;
//mod benchmarks;
mod error;
mod float;
#[macro_use]
mod macros;

//pub use float::{Float, Rounding};

pub use uint::BUint;
pub use iint::BIint;
pub use error::*;

#[allow(unused)]
type I128 = iint::BIint::<2>;

#[allow(unused)]
pub type U128 = BUint::<{128 / digit::BITS}>;

pub type U256 = BUint::<{256 / digit::BITS}>;
pub type U512 = BUint::<{512 / digit::BITS}>;
pub type U1024 = BUint::<{1024 / digit::BITS}>;
pub type U2048 = BUint::<{2048 / digit::BITS}>;
pub type U4096 = BUint::<{4096 / digit::BITS}>;
pub type U8192 = BUint::<{8192 / digit::BITS}>;