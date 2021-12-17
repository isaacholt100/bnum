#![allow(incomplete_features)]
#![cfg_attr(feature = "nightly", feature(
    generic_const_exprs,
    const_maybe_uninit_assume_init,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_ptr,
    const_ptr_offset,
    test,
    unchecked_math,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    inline_const,
    //bigint_helper_methods,
))]
//#![no_std]

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
mod float_old;
#[macro_use]
mod macros;
mod fraction;
mod radix_bases;
mod factors;
#[macro_use]
mod doc;
mod vector;
mod matrix;

#[cfg(feature = "nightly")]
mod benchmarks;

type ExpType = usize;

pub use vector::Vector;

pub use matrix::Matrix;

pub use float_old::Float;

pub use uint::BUint;
pub use iint::BIint;
pub use error::*;
pub use digit::Digit;

pub use fraction::Fraction;

pub type U64 = BUint::<{64 / digit::BITS}>;
pub type U128 = BUint::<{128 / digit::BITS}>;
pub type U256 = BUint::<{256 / digit::BITS}>;
pub type U512 = BUint::<{512 / digit::BITS}>;
pub type U1024 = BUint::<{1024 / digit::BITS}>;
pub type U2048 = BUint::<{2048 / digit::BITS}>;
pub type U4096 = BUint::<{4096 / digit::BITS}>;
pub type U8192 = BUint::<{8192 / digit::BITS}>;

pub type I128 = iint::BIint::<{128 / digit::BITS}>;

pub type F64 = float_old::Float::<{64 / digit::BITS}, 52>;
