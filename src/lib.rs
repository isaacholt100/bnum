#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(feature = "nightly", feature(generic_const_exprs))]
#![cfg_attr(
    all(test, feature = "nightly"),
    feature(
        bigint_helper_methods,
        int_roundings,
        float_minimum_maximum,
        wrapping_next_power_of_two,
        unchecked_shifts,
        strict_overflow_ops,
        unbounded_shifts
    )
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "arbitrary", feature = "quickcheck")), no_std)]
// TODO: MAKE SURE NO_STD IS ENABLED WHEN PUBLISHING NEW VERSION

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

#[cfg(feature = "signed")]
mod bint;
mod buint;

pub mod cast;
mod digit;
mod doc;
pub mod errors;
mod helpers;
mod int;
mod nightly;
pub mod prelude;

#[cfg(feature = "rand")]
pub mod random;

pub mod types;

#[cfg(feature = "float")]
mod float;

#[cfg(feature = "float")]
pub use float::Float;

#[cfg(test)]
mod test;

#[cfg(test)]
use test::types::*;

type ExpType = u32;

#[cfg(feature = "signed")]
pub use bint::Int;
pub use buint::Uint;

type Digit = u8;

/// Trait for fallible conversions between `bnum` integer types.
///
/// Unfortunately, [`TryFrom`] cannot currently be used for conversions between `bnum` integers, since [`TryFrom<T> for T`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html#impl-TryFrom%3CU%3E-for-T) is already implemented by the standard library (and so it is not possible to implement `TryFrom<Uint<M>> for Uint<N>`). When the [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560) feature becomes stabilised, it may be possible to use [`TryFrom`] instead of `BTryFrom`. `BTryFrom` is designed to have the same behaviour as [`TryFrom`] for conversions between two primitive types, and conversions between a primitive type and a bnum type. `BTryFrom` is a workaround for the issue described above, and so you should not implement it yourself. It should only be used for conversions between `bnum` integers.
pub trait BTryFrom<T>: Sized {
    type Error;

    fn try_from(from: T) -> Result<Self, Self::Error>;
}
