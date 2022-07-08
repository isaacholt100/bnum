#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(
    feature = "nightly",
    feature(
        generic_const_exprs,
        const_mut_refs,
        const_maybe_uninit_as_mut_ptr,
        const_trait_impl,
        const_num_from_num,
        const_swap,
    )
)]
#![cfg_attr(
    test,
    feature(
        bigint_helper_methods,
        int_log,
        int_roundings,
        float_minimum_maximum,
        wrapping_next_power_of_two,
        mixed_integer_ops,
    )
)]
#![doc = include_str!("../README.md")]
#![no_std]

#[macro_use]
extern crate alloc;

mod bint;
pub mod cast;
mod digit;
mod doc;
// TODO: document this module's items
pub mod errors;
mod int;
mod nightly;
// TODO: document this module's items
pub mod prelude;

#[cfg(feature = "rand")]
pub mod random;

mod buint;
mod radix_bases;
pub mod types;

#[cfg(test)]
mod test;

#[cfg(test)]
use test::types::*;

#[cfg(feature = "usize_exptype")]
type ExpType = usize;
#[cfg(not(feature = "usize_exptype"))]
type ExpType = u32;

pub use bint::BInt;
pub use buint::BUint;
pub use digit::Digit;

// TODO: indicate which methods are only available on nightly (maybe not bc already mentioned in README)