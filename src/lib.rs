#![allow(incomplete_features)]
#![feature(
    generic_const_exprs,
    const_mut_refs,
    const_maybe_uninit_as_mut_ptr,
    const_trait_impl,
    bigint_helper_methods, // not necessary TODO: change code so doesn't need this, otherwise include in README, same for others
    int_roundings, // not necessary
    const_bigint_helper_methods, // not necessary
)]
#![cfg_attr(test, feature(
    test,
    int_log,
    float_minimum_maximum,
    wrapping_next_power_of_two,
))]
#![doc = include_str!("../README.md")]
//#![no_std]

// TODO: sort out license

#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate quickcheck;

mod cast;
mod digit;
mod doc;
mod error;
mod int;
pub mod prelude;

#[cfg(feature = "rand")]
mod random;

mod uint;
mod macros;
mod radix_bases;
mod types;

#[cfg(test)]
mod test;

pub use cast::As;

#[cfg(feature = "rand")]
pub use random::RandomUniformInt;

#[cfg(test)]
mod benchmarks;

#[cfg(feature = "usize_exptype")]
type ExpType = usize;
#[cfg(not(feature = "usize_exptype"))]
type ExpType = u32;

pub use uint::BUint;
pub use int::Bint;
pub use error::*;
pub use digit::Digit;

pub use types::*;