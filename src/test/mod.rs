pub mod convert;
pub use convert::TestConvert;

mod macros;

#[allow(unused_imports)]
pub use macros::*;

pub mod types;

#[derive(Clone, Copy)]
pub struct U8ArrayWrapper<const N: usize>(pub [u8; N]);

impl<const N: usize> From<U8ArrayWrapper<N>> for [u8; N] {
    fn from(a: U8ArrayWrapper<N>) -> Self {
        a.0
    }
}

use quickcheck::{Arbitrary, Gen};

impl<const N: usize> Arbitrary for U8ArrayWrapper<N> {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut arr = [0u8; N];
        for x in arr.iter_mut() {
            *x = u8::arbitrary(g);
        }
        Self(arr)
    }
}

use core::fmt::{self, Debug, Formatter};

impl<const N: usize> Debug for U8ArrayWrapper<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub mod cast_types {
    use crate::{BIntD8, BUintD8};

    pub type TestUint1 = BUintD8<10>;
    pub type TestUint2 = BUintD8<8>;
    pub type TestUint3 = BUintD8<6>;
    pub type TestUint4 = BUintD8<11>;
    pub type TestUint5 = BUintD8<5>;
    pub type TestUint6 = BUintD8<7>;
    pub type TestUint7 = BUintD8<3>;
    pub type TestUint8 = BUintD8<1>;
    pub type TestUint9 = BUintD8<15>;
    pub type TestUint10 = BUintD8<17>;

    pub type TestInt1 = BIntD8<10>;
    pub type TestInt2 = BIntD8<8>;
    pub type TestInt3 = BIntD8<6>;
    pub type TestInt4 = BIntD8<11>;
    pub type TestInt5 = BIntD8<5>;
    pub type TestInt6 = BIntD8<7>;
    pub type TestInt7 = BIntD8<3>;
    pub type TestInt8 = BIntD8<1>;
    pub type TestInt9 = BIntD8<15>;
    pub type TestInt10 = BIntD8<17>;
}
