pub mod convert;
pub use convert::TestConvert;
pub use convert::test_eq;
mod bitint;

// #[cfg(all(feature = "float", feature = "rug"))]
// mod test_float;

pub use bitint::BitInt;

mod macros;

#[allow(unused_imports)]
pub use macros::*;

#[cfg(feature = "alloc")]
#[derive(Clone, Copy, Debug)]
pub struct Radix<const MAX: u32>(pub u32);

#[cfg(feature = "alloc")]
impl<const MAX: u32> quickcheck::Arbitrary for Radix<MAX> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let radix = (u32::arbitrary(g) % (MAX - 1)) + 2;
        Self(radix)
    }
}

#[cfg(feature = "alloc")]
impl<const MAX: u32> From<Radix<MAX>> for u32 {
    fn from(r: Radix<MAX>) -> Self {
        r.0
    }
}

mod cast_signed_types {
    use crate::Int;

    pub type TestInt1 = Int<10, 0>;
    pub type TestInt2 = Int<8, 0>;
    pub type TestInt3 = Int<6, 0>;
    pub type TestInt4 = Int<11, 0>;
    pub type TestInt5 = Int<5, 0>;
    pub type TestInt6 = Int<7, 0>;
    pub type TestInt7 = Int<3, 0>;
    pub type TestInt8 = Int<1, 0>;
    pub type TestInt9 = Int<15, 0>;
    pub type TestInt10 = Int<17, 0>;
}

pub mod cast_types {
    use crate::Uint;

    pub type TestUint1 = Uint<10, 0>;
    pub type TestUint2 = Uint<8, 0>;
    pub type TestUint3 = Uint<6, 0>;
    pub type TestUint4 = Uint<11, 0>;
    pub type TestUint5 = Uint<5, 0>;
    pub type TestUint6 = Uint<7, 0>;
    pub type TestUint7 = Uint<3, 0>;
    pub type TestUint8 = Uint<1, 0>;
    pub type TestUint9 = Uint<15, 0>;
    pub type TestUint10 = Uint<17, 0>;

    pub use super::cast_signed_types::*;
}

