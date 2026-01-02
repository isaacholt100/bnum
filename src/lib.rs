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
mod digit;
mod doc;
pub mod errors;
mod helpers;
#[doc(hidden)]
pub mod literal_parse;
pub mod prelude;
mod wide_digits;
mod digits;
mod overflow;

use wide_digits::{WideDigits, WideDigitsMut};

#[cfg(feature = "rand")]
pub mod random;

pub mod types;

#[cfg(feature = "float")]
mod float;

#[cfg(feature = "float")]
pub use float::Float;

#[cfg(test)]
mod test;

type Exponent = u32;
type Byte = u8;

pub use integer::{Int, Integer, Uint};

/// Trait for fallible conversions between `bnum` integer types.
///
/// Unfortunately, [`TryFrom`] cannot currently be used for conversions between `bnum` integers, since [`TryFrom<T> for T`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html#impl-TryFrom%3CU%3E-for-T) is already implemented by the standard library (and so it is not possible to implement `TryFrom<Uint<M, A, OM>> for Uint<N, B, OM>`). When the [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560) feature becomes stabilised, it may be possible to use [`TryFrom`] instead of `BTryFrom`. `BTryFrom` is designed to have the same behaviour as [`TryFrom`] for conversions between two primitive types, and conversions between a primitive type and a bnum type. `BTryFrom` is a workaround for the issue described above, and so you should not implement it yourself. It should only be used for conversions between `bnum` integers.
pub trait BTryFrom<T>: Sized {
    type Error;

    fn try_from(from: T) -> Result<Self, Self::Error>;
}

pub use overflow::OverflowMode;