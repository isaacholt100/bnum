use super::Float;
use crate::BUint;
use crate::digit;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline(always)]
    pub const fn to_bits(self) -> BUint<W> {
        self.bits
    }

    #[inline(always)]
    pub const fn from_bits(v: BUint<W>) -> Self {
        Self {
            bits: v,
        }
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; W * digit::BYTES as usize] {
        self.to_bits().to_be_bytes()
    }

    #[inline]
    pub const fn to_le_bytes(self) -> [u8; W * digit::BYTES as usize] {
        self.to_bits().to_le_bytes()
    }

    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; W * digit::BYTES as usize] {
        self.to_bits().to_ne_bytes()
    }

    #[inline]
    pub const fn from_be_bytes(bytes: [u8; W * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_be_bytes(bytes))
    }

    #[inline]
    pub const fn from_le_bytes(bytes: [u8; W * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_le_bytes(bytes))
    }

    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; W * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_ne_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::test::U8ArrayWrapper;

    fn to_u64_bits(f: crate::F64) -> u64 {
        f.to_bits().as_u64()
    }

    test_float! {
        function: to_bits(a: f64)
    }
    test_float! {
        function: from_bits(a: u64),
        big_converter: to_u64_bits,
        converter: f64::to_bits
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
        function: from_be_bytes(a: U8ArrayWrapper<8>),
        big_converter: to_u64_bits,
        converter: f64::to_bits
    }
    test_float! {
        function: from_le_bytes(a: U8ArrayWrapper<8>),
        big_converter: to_u64_bits,
        converter: f64::to_bits
    }
    test_float! {
        function: from_ne_bytes(a: U8ArrayWrapper<8>),
        big_converter: to_u64_bits,
        converter: f64::to_bits
    }
}