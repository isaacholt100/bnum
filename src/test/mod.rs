pub mod convert;
pub use convert::TestConvert;

mod macros;

#[allow(unused_imports)]
pub use macros::*;

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

mod cast_signed_types {
    use crate::Int;

    pub type TestInt1 = Int<10>;
    pub type TestInt2 = Int<8>;
    pub type TestInt3 = Int<6>;
    pub type TestInt4 = Int<11>;
    pub type TestInt5 = Int<5>;
    pub type TestInt6 = Int<7>;
    pub type TestInt7 = Int<3>;
    pub type TestInt8 = Int<1>;
    pub type TestInt9 = Int<15>;
    pub type TestInt10 = Int<17>;
}

pub mod cast_types {
    use crate::Uint;

    pub type TestUint1 = Uint<10>;
    pub type TestUint2 = Uint<8>;
    pub type TestUint3 = Uint<6>;
    pub type TestUint4 = Uint<11>;
    pub type TestUint5 = Uint<5>;
    pub type TestUint6 = Uint<7>;
    pub type TestUint7 = Uint<3>;
    pub type TestUint8 = Uint<1>;
    pub type TestUint9 = Uint<15>;
    pub type TestUint10 = Uint<17>;

    pub use super::cast_signed_types::*;
}
