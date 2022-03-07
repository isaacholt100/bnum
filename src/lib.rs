#![allow(incomplete_features)]
#![cfg_attr(feature = "nightly", feature(
    generic_const_exprs,
    const_intrinsic_copy,
    const_mut_refs,
    const_maybe_uninit_as_mut_ptr,
    const_ptr_offset,
    test,
    unchecked_math,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    inline_const,
    const_trait_impl,
    //bigint_helper_methods,
))]
//#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate quickcheck;

mod uint;
mod arithmetic;
mod digit;
mod int;
mod error;
mod bound;
mod float_old;
#[macro_use]
mod macros;
mod fraction;
mod radix_bases;
mod factors;
#[macro_use]
mod doc;
mod vector;
mod matrix;
#[cfg(test)]
mod test;

#[cfg(feature = "nightly")]
mod benchmarks;

type ExpType = usize;

pub use vector::Vector;

pub use matrix::Matrix;

pub use float_old::Float;

pub use uint::BUint;
pub use int::Bint;
pub use error::*;
pub use digit::Digit;

pub use fraction::Fraction;

pub type U64 = BUint::<{64 / digit::BITS}>;
pub type U128 = BUint::<{128 / digit::BITS}>;
pub type U256 = BUint::<{256 / digit::BITS}>;
pub type U512 = BUint::<{512 / digit::BITS}>;
pub type U1024 = BUint::<{1024 / digit::BITS}>;
pub type U2048 = BUint::<{2048 / digit::BITS}>;
pub type U4096 = BUint::<{4096 / digit::BITS}>;
pub type U8192 = BUint::<{8192 / digit::BITS}>;

pub type I128 = int::Bint::<{128 / digit::BITS}>;

pub type F64 = float_old::Float::<{64 / digit::BITS}, 52>;

#[cfg(test)]
mod array_wrapper {
    #[derive(Clone, Copy)]
    pub struct ArrayWrapper([u8; 16]);

    impl From<ArrayWrapper> for [u8; 16] {
        fn from(a: ArrayWrapper) -> Self {
            a.0
        }
    }
    
    use quickcheck::{Arbitrary, Gen};
    
    impl Arbitrary for ArrayWrapper {
        fn arbitrary(g: &mut Gen) -> Self {
            Self(u128::arbitrary(g).to_be_bytes())
        }
    }

    use core::fmt::{Formatter, self, Debug};

    impl Debug for ArrayWrapper {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }
}

#[cfg(test)]
use array_wrapper::ArrayWrapper;