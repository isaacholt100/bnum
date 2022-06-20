mod convert;
pub use convert::TestConvert;

mod macros;
#[allow(unused_imports)]
pub use macros::*;

macro_rules! test_type {
	($prefix: ident, $suffix: ident, $bits: literal) => {
		paste::paste! {
			#[allow(non_camel_case_types, unused)]
			pub type [<$prefix $suffix>] = [<$prefix $bits>];
	
			#[allow(unused)]
			pub type [<$prefix:upper $suffix:upper>] = crate::types::[<$prefix:upper $bits>];
		}
	};
}

macro_rules! test_types {
	($suffix: ident, $bits: literal) => {
		pub mod types {
			paste::paste! {
				test_type!(u, $suffix, $bits);
				test_type!(i, $suffix, $bits);
			}
		}
	};
}

test_types!(test, 128);

#[derive(Clone, Copy)]
pub struct U8ArrayWrapper<const N: usize>([u8; N]);

impl<const N: usize> From<U8ArrayWrapper<N>> for [u8; N] {
    fn from(a: U8ArrayWrapper<N>) -> Self {
        a.0
    }
}

use quickcheck::{Arbitrary, Gen};

impl Arbitrary for U8ArrayWrapper<16> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u128::arbitrary(g).to_be_bytes())
    }
}

impl Arbitrary for U8ArrayWrapper<8> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u64::arbitrary(g).to_be_bytes())
    }
}

impl Arbitrary for U8ArrayWrapper<4> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u32::arbitrary(g).to_be_bytes())
    }
}

impl Arbitrary for U8ArrayWrapper<2> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u16::arbitrary(g).to_be_bytes())
    }
}

impl Arbitrary for U8ArrayWrapper<1> {
    fn arbitrary(g: &mut Gen) -> Self {
        Self(u8::arbitrary(g).to_be_bytes())
    }
}

use core::fmt::{Formatter, self, Debug};

impl<const N: usize> Debug for U8ArrayWrapper<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}