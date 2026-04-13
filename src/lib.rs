#![allow(incomplete_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    all(test, nightly),
    feature(
        widening_mul,
        signed_bigint_helpers,
        int_roundings,
        wrapping_int_impl,
        float_minimum_maximum,
        uint_bit_width,
        wrapping_next_power_of_two,
        f16,
        f128,
        int_from_ascii
    )
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "arbitrary", feature = "quickcheck")), no_std)]
// TODO: MAKE SURE NO_STD IS ENABLED WHEN PUBLISHING NEW VERSION

// TODO: create issue on gh about v1.0 release. problem is that crates like rand aren't in 1.x yet

#[cfg(any(feature = "alloc", test))]
#[macro_use]
extern crate alloc;

mod integer;

pub mod cast;
mod doc;
pub(crate) mod macros;
pub mod errors;
mod helpers;
pub mod prelude;
mod digits;
mod overflow;

#[doc(hidden)]
pub mod __internal {
    pub use super::macros::*;
}

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
