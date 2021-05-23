use super::BintTest;
use crate::uint::BUint;

impl<const N: usize> BintTest<N> {
    uint_method! {
        fn as_u8(&self) -> u8,
        fn as_u16(&self) -> u16,
        fn as_u32(&self) -> u32,
        fn as_u64(&self) -> u64,
        fn as_u128(&self) -> u128,
        fn as_usize(&self) -> usize,
        fn as_i8(&self) -> i8,
        fn as_i16(&self) -> i16,
        fn as_i32(&self) -> i32,
        fn as_i64(&self) -> i64,
        fn as_i128(&self) -> i128,
        fn as_isize(&self) -> isize//,
        //fn as_f32(&self) -> f32,
        //fn as_f64(&self) -> f64
    }
    pub const fn as_uint(&self) -> BUint<N> {
        self.uint
    }
}