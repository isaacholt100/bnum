#![allow(incomplete_features)]
#![cfg_attr(feature = "nightly", feature(
    const_generics,
    const_evaluatable_checked,
    const_panic,
    const_maybe_uninit_assume_init,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_ptr,
    const_ptr_offset,
    test,
    unchecked_math,
))]
#![no_std]

#[allow(unused)]
fn u32_to_exp(u: u32) -> ExpType {
    u as ExpType
}

#[macro_use]
extern crate alloc;

#[allow(unused)]
macro_rules! test {
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        name: $name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        }
    } => {
        test! {
            big: $big_type,
            primitive: $primitive,
            name: $name,
            method: {
                $($method ($($arg), *) ;) *
            },
            converter: Into::into
        }
    };
    {
        big: $big_type: ty,
        primitive: $primitive: ty,
        name: $name: ident,
        method: {
            $($method: ident ($($arg: expr), *) ;) *
        },
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
    }
}

mod uint;
mod arithmetic;
mod digit;
mod iint;
mod error;
mod bound;
//mod float;
#[macro_use]
mod macros;
mod fraction;
mod radix_bases;
mod factors;

#[cfg(feature = "nightly")]
mod benchmarks;

type ExpType = usize;

//pub use float::{Float, Rounding};

pub use uint::BUint;
pub use iint::BIint;
pub use error::*;
pub use digit::Digit;

//pub use float::Float;

pub use fraction::Fraction;

pub type U128 = BUint::<{128 / digit::BITS}>;
pub type U256 = BUint::<{256 / digit::BITS}>;
pub type U512 = BUint::<{512 / digit::BITS}>;
pub type U1024 = BUint::<{1024 / digit::BITS}>;
pub type U2048 = BUint::<{2048 / digit::BITS}>;
pub type U4096 = BUint::<{4096 / digit::BITS}>;
pub type U8192 = BUint::<{8192 / digit::BITS}>;

pub type I128 = iint::BIint::<{128 / digit::BITS}>;