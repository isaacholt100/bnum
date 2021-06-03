use super::BintTest;
use crate::uint::BUint;
use crate::uint;
use crate::digit::Digit;

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

    pub fn as_f32(&self) -> f32 {
        let f = self.unsigned_abs().as_f32();
        if self.is_negative() {
            -f
        } else {
            f
        }
    }

    pub fn as_f64(&self) -> f64 {
        let f = self.unsigned_abs().as_f64();
        if self.is_negative() {
            -f
        } else {
            f
        }
    }

    pub const fn as_buint<const M: usize>(&self) -> BUint<M> where [Digit; M - N]: Sized {
        if M > N {
            let padding_digit = if self.is_negative() {
                1
            } else {
                0
            };
            uint::cast_up::<N, M>(&self.uint, padding_digit)
        } else {
            uint::cast_down::<N, M>(&self.uint)
        }
    }
    pub const fn as_biint<const M: usize>(&self) -> BintTest<M> where [Digit; M - N]: Sized {
        BintTest {
            uint: self.as_buint()
        }
    }
}