use super::BUint;
use crate::digit;

macro_rules! as_int {
    ($method: ident, $int: ty) => {
        pub const fn $method(&self) -> $int {
            (&self.digits)[0] as $int
        }
    };
}

impl<const N: usize> BUint<N> {
    as_int!(as_u8, u8);
    as_int!(as_u16, u16);
    as_int!(as_u32, u32);
    pub const fn as_u64(&self) -> u64 {
        (&self.digits)[0]
    }
    pub const fn as_u128(&self) -> u128 {
        digit::to_double_digit((&self.digits)[1], (&self.digits)[0])
    }
    pub const fn as_usize(&self) -> usize {
        self.as_u128() as usize
    }

    as_int!(as_i8, i8);
    as_int!(as_i16, i16);
    as_int!(as_i32, i32);
    as_int!(as_i64, i64);
    pub const fn as_i128(&self) -> i128 {
        self.as_u128() as i128
    }
    pub const fn as_isize(&self) -> isize {
        self.as_u128() as isize
    }

    as_int!(as_f32, f32);
    as_int!(as_f64, f64);
}