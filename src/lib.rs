#![allow(incomplete_features)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(
    all(test, nightly),
    feature(
        bigint_helper_methods,
        int_roundings,
        // float_minimum_maximum,
        wrapping_next_power_of_two,
        unchecked_shifts,
        unchecked_neg,
        f16,
        f128,
        int_from_ascii
    )
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "arbitrary", feature = "quickcheck")), no_std)]
// TODO: MAKE SURE NO_STD IS ENABLED WHEN PUBLISHING NEW VERSION

// TODO: create issue on gh about v1.0 release. problem is that crates like rand aren't in 1.x yet

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

mod integer;

pub mod cast;
mod doc;
pub mod errors;
mod helpers;
#[doc(hidden)]
pub mod literal_parse;
pub mod prelude;
mod digits;
mod overflow;

// #[cfg(feature = "float")]
// mod float;

#[cfg(feature = "rand")]
pub mod random;

pub mod types;

#[cfg(test)]
mod test;

type Exponent = u32;
type Byte = u8;

pub use integer::{Int, Integer, Uint};
pub use overflow::OverflowMode;

// #[cfg(feature = "float")]
// pub use float::Float;