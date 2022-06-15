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
    int_log,
    float_minimum_maximum,
    wrapping_next_power_of_two,
	mixed_integer_ops,
))]
#![doc = include_str!("../README.md")]
//#![no_std]

// TODO: sort out license
// TODO: credit all necessary bits of code/rewrite myself. have already commented where all bits which need crediting, just need to actually credit them properly

#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate quickcheck;

mod cast;
mod digit;
mod doc;
mod error;
mod bint;
mod int;
pub mod prelude;

#[cfg(feature = "rand")]
mod random;

mod buint;
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

pub use buint::BUint;
pub use bint::BInt;
pub use error::*;
pub use digit::Digit;

pub use types::*;