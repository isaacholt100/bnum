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
        int_roundings,
        float_minimum_maximum,
        wrapping_next_power_of_two,
    )
)]
#![doc = include_str!("../README.md")]
//#![no_std]

#[macro_use]
extern crate alloc;

mod bint;
mod buint;

pub mod cast;
mod digit;
mod doc;
pub mod errors;
mod int;
mod nightly;
pub mod prelude;

#[cfg(feature = "rand")]
pub mod random;

pub mod types;

/*#[cfg(feature = "nightly")]
mod float;

#[cfg(feature = "nightly")]
pub use float::Float;*/

#[cfg(test)]
mod test;

#[cfg(test)]
use test::types::*;

#[cfg(feature = "usize_exptype")]
type ExpType = usize;
#[cfg(not(feature = "usize_exptype"))]
type ExpType = u32;

/*pub const fn widening_mul_u128(a: u128, b: u128) -> (u128, u128) {
    let (a_low, a_high) = (a as u64 as u128, a >> 64);
    let (b_low, b_high) = (b as u64 as u128, b >> 64);

    let high = a_high * b_high;
    let low = a_low * b_low;

    let (mid, carry) = (a_low * b_high).overflowing_add(b_low * a_high);
    let (mid_low, mut mid_high) = (mid as u64 as u128, mid >> 64);
    if carry {
        mid_high |= 1 << 64;
    }

    let (low, carry) = low.overflowing_add(mid_low << 64);

    (low, high + mid_high + carry as u128)
}*/

macro_rules! macro_impl {
    ($name: ident) => {
        use crate::bigints::*;

        crate::main_impl!($name);
    };
}

pub(crate) use macro_impl;

macro_rules! main_impl {
    ($name: ident) => {
        $name!(BUint, BInt, u64);
        $name!(BUintD32, BIntD32, u32);
        $name!(BUintD16, BIntD16, u16);
        $name!(BUintD8, BIntD8, u8);
    };
}

pub(crate) use main_impl;

mod bigints {
	pub use crate::bint::{BInt, BIntD16, BIntD32, BIntD8};
	pub use crate::buint::{BUint, BUintD16, BUintD32, BUintD8};
}

pub use bigints::*;