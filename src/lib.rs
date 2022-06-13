#![allow(incomplete_features)]
#![cfg_attr(feature = "nightly", feature(
    generic_const_exprs,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_mut_ptr,
    const_trait_impl,
    bigint_helper_methods, // not necessary
    int_roundings, // not necessary
    const_bigint_helper_methods, // not necessary
))]
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
//mod float;
mod int;
pub mod prelude;

#[cfg(feature = "rand")]
mod random;

mod uint;
mod macros;
mod radix_bases;

#[cfg(test)]
mod test;

pub use cast::As;

#[cfg(feature = "rand")]
pub use random::RandomUniformInt;

#[cfg(all(feature = "nightly", test))]
mod benchmarks;

#[cfg(feature = "usize_exptype")]
type ExpType = usize;
#[cfg(not(feature = "usize_exptype"))]
type ExpType = u32;

//pub use float::Float;

pub use uint::BUint;
pub use int::Bint;
pub use error::*;
pub use digit::Digit;

pub type U64 = BUint::<{64 / digit::BITS as usize}>;
pub type U128 = BUint::<{128 / digit::BITS as usize}>;
pub type U256 = BUint::<{256 / digit::BITS as usize}>;
pub type U512 = BUint::<{512 / digit::BITS as usize}>;
pub type U1024 = BUint::<{1024 / digit::BITS as usize}>;
pub type U2048 = BUint::<{2048 / digit::BITS as usize}>;
pub type U4096 = BUint::<{4096 / digit::BITS as usize}>;
pub type U8192 = BUint::<{8192 / digit::BITS as usize}>;

pub type I128 = Bint::<{128 / digit::BITS as usize}>;
pub type I64 = Bint::<{64 / digit::BITS as usize}>;

/*pub type F64 = Float::<{64 / digit::BITS as usize}, 52>;

pub const fn u64_words(bits: usize) -> usize {
    let bytes = (bits + 7) / 8;
    bytes * 8 / u64::BITS as usize
}

pub const fn u32_words(bits: usize) -> usize {
    let rem = bits % u64::BITS as usize;
    let bytes = (rem + 7) / 8;
    debug_assert!(bytes <= 8);
    (bytes & 0b100) >> 2
}

pub const fn u16_words(bits: usize) -> usize {
    let rem = bits % u64::BITS as usize;
    let bytes = (rem + 7) / 8;
    debug_assert!(bytes <= 8);
    (bytes & 0b10) >> 1
}

pub const fn u8_words(bits: usize) -> usize {
    let rem = bits % u64::BITS as usize;
    let bytes = (rem + 7) / 8;
    debug_assert!(bytes <= 8);
    bytes & 0b1
}*/

/*pub struct U<const W64: usize, const W32: usize, const W16: usize, const W8: usize> {
    u64_words: [u64; W64],
    u32_words: [u32; W32],
    u16_words: [u16; W16],
    u8_words: [u8; W8],
}

pub type Tst<const B: usize> = U::<{u64_words(B)}, {u32_words(B)}, {u16_words(B)}, {u8_words(B)}>;

pub type T = Tst::<88>;*/