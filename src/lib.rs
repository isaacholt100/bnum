#![allow(incomplete_features)]
#![cfg_attr(feature = "nightly", feature(
    generic_const_exprs,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_mut_ptr,
    const_ptr_offset,
    unchecked_math,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    inline_const,
    const_trait_impl,
    bigint_helper_methods,
    int_roundings,
))]
#![cfg_attr(test, feature(test))]
#![doc = include_str!("../README.md")]
#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate quickcheck;

mod uint;
mod arithmetic;
mod digit;
mod int;
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
#[cfg(test)]
mod test;

mod expr;

#[cfg(all(feature = "nightly", test))]
mod benchmarks;

type ExpType = usize;

pub use vector::Vector;

pub use matrix::Matrix;

pub use float_old::Float;

pub use uint::BUint;
pub use int::Bint;
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

pub type I128 = int::Bint::<{128 / digit::BITS}>;

pub type F64 = float_old::Float::<{64 / digit::BITS}, 52>;