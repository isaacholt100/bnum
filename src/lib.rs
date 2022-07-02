#![allow(incomplete_features)]
#![feature(
    generic_const_exprs,
    const_mut_refs,
    const_maybe_uninit_as_mut_ptr,
    const_trait_impl,
)]
#![cfg_attr(test, feature(
	bigint_helper_methods,
    int_log,
	int_roundings,
    float_minimum_maximum,
    wrapping_next_power_of_two,
	mixed_integer_ops,
))]
#![doc = include_str!("../README.md")]
#![no_std]

#[macro_use]
extern crate alloc;

mod cast;
mod digit;
mod doc;
pub mod errors;
mod bint;
mod int;
pub mod prelude;

#[cfg(feature = "rand")]
mod random;

mod buint;
mod radix_bases;
mod types;

#[cfg(test)]
mod test;

#[cfg(test)]
use test::types::*;

pub use cast::As;

#[cfg(feature = "rand")]
pub use random::RandomUniformInt;

#[cfg(feature = "usize_exptype")]
type ExpType = usize;
#[cfg(not(feature = "usize_exptype"))]
type ExpType = u32;

pub use buint::BUint;
pub use bint::BInt;
pub use digit::Digit;

pub use types::*;