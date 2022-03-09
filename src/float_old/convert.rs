use super::Float;
use crate::BUint;
use crate::digit;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline(always)]
    pub const fn to_bits(self) -> BUint<W> {
        self.uint
    }

    #[inline(always)]
    pub const fn from_bits(v: BUint<W>) -> Self {
        Self {
            uint: v,
        }
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const fn to_be_bytes(self) -> [u8; W * digit::BYTES] {
        self.to_bits().to_be_bytes()
    }
    pub const fn to_le_bytes(self) -> [u8; W * digit::BYTES] {
        self.to_bits().to_le_bytes()
    }
    pub const fn to_ne_bytes(self) -> [u8; W * digit::BYTES] {
        self.to_bits().to_ne_bytes()
    }
    pub const fn from_be_bytes(bytes: [u8; W * digit::BYTES]) -> Self {
        Self::from_bits(BUint::from_be_bytes(bytes))
    }
    pub const fn from_le_bytes(bytes: [u8; W * digit::BYTES]) -> Self {
        Self::from_bits(BUint::from_le_bytes(bytes))
    }
    pub const fn from_ne_bytes(bytes: [u8; W * digit::BYTES]) -> Self {
        Self::from_bits(BUint::from_ne_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::U8ArrayWrapper;

    test_float! {
        function: to_bits(a: f64)
    }
    test_float! {
        function: from_bits(a: u64)
    }
    test_float! {
        function: to_be_bytes(a: f64),
        converter: U8ArrayWrapper::converter
    }
    test_float! {
        function: to_le_bytes(a: f64),
        converter: U8ArrayWrapper::converter
    }
    test_float! {
        function: to_ne_bytes(a: f64),
        converter: U8ArrayWrapper::converter
    }
    test_float! {
        function: from_be_bytes(a: U8ArrayWrapper<8>)
    }
    test_float! {
        function: from_le_bytes(a: U8ArrayWrapper<8>)
    }
    test_float! {
        function: from_ne_bytes(a: U8ArrayWrapper<8>)
    }
}