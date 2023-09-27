#![cfg_attr(feature = "nightly", allow(incomplete_features))]
#![cfg_attr(
    feature = "nightly",
    feature(
        generic_const_exprs,
        const_trait_impl,
        const_option_ext
    )
)]
#![cfg_attr(
    test,
    feature(
        bigint_helper_methods,
        int_roundings,
        float_minimum_maximum,
        wrapping_next_power_of_two,
        float_next_up_down,
        unchecked_math,
    )
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "arbitrary", feature = "quickcheck")), no_std)]
// TODO: MAKE SURE NO_STD IS ENABLED WHEN PUBLISHING NEW VERSION

#[macro_use]
extern crate alloc;

mod bint;
mod buint;

pub mod cast;
mod digit;
mod doc;
pub mod errors;
mod int;
mod nightly;
pub mod prelude;

#[cfg(feature = "rand")]
pub mod random;

pub mod types;

// #[cfg(feature = "nightly")]
// mod float;

// #[cfg(feature = "nightly")]
// pub use float::Float;

#[cfg(test)]
mod test;

#[cfg(test)]
use test::types::*;

/*#[cfg(feature = "usize_exptype")]
type ExpType = usize;
#[cfg(not(feature = "usize_exptype"))]*/
type ExpType = u32;

macro_rules! macro_impl {
    ($name: ident) => {
        use crate::bigints::*;

        crate::main_impl!($name);
    };
}

pub(crate) use macro_impl;

macro_rules! main_impl {
    ($name: ident) => {
        $name!(BUint, BInt, u64);
        $name!(BUintD32, BIntD32, u32);
        $name!(BUintD16, BIntD16, u16);
        $name!(BUintD8, BIntD8, u8);
    };
}

use crate::buint::cast::buint_as_different_digit_bigint;
use crate::bint::cast::bint_as_different_digit_bigint;

buint_as_different_digit_bigint!(BUint, BInt, u64; (BUintD32, u32), (BUintD16, u16), (BUintD8, u8));
buint_as_different_digit_bigint!(BUintD32, BIntD32, u32; (BUint, u64), (BUintD16, u16), (BUintD8, u8));
buint_as_different_digit_bigint!(BUintD16, BIntD16, u16; (BUint, u64), (BUintD32, u32), (BUintD8, u8));
buint_as_different_digit_bigint!(BUintD8, BIntD8, u8; (BUint, u64), (BUintD32, u32), (BUintD16, u16));

bint_as_different_digit_bigint!(BUint, BInt, u64; (BIntD32, u32), (BIntD16, u16), (BIntD8, u8));
bint_as_different_digit_bigint!(BUintD32, BIntD32, u32; (BInt, u64), (BIntD16, u16), (BIntD8, u8));
bint_as_different_digit_bigint!(BUintD16, BIntD16, u16; (BInt, u64), (BIntD32, u32), (BIntD8, u8));
bint_as_different_digit_bigint!(BUintD8, BIntD8, u8; (BInt, u64), (BIntD32, u32), (BIntD16, u16));

pub(crate) use main_impl;

mod bigints {
    pub use crate::bint::{BInt, BIntD16, BIntD32, BIntD8};
    pub use crate::buint::{BUint, BUintD16, BUintD32, BUintD8};
}

pub use bigints::*;

/// Trait for fallible conversions between bnum integer types.
/// 
/// Unfortunately, [`TryFrom`] cannot currently be used for conversions between bnum integers, since `TryFrom<T> for T` is already implemented by the standard library. When the `generic_const_exprs` feature becomes stabilised, it may be possible to use `TryFrom` instead of `BTryFrom`. `BTryFrom` is designed to have the same behaviour as `TryFrom` for conversions between two primitive types, and conversions between a primitive type and a bnum type. `BTryFrom` is a workaround for the issue described above, and so you should not implement it yourself.
pub trait BTryFrom<T>: Sized {
    type Error;

    fn try_from(from: T) -> Result<Self, Self::Error>;
}

// TODO: create round-to-nearest ties-to-even function, it could take a uint and a target bit width, and return the correctly rounded result in the target precision, as well as the overflow, and whether a round up occurred
// #[allow(unused)]
// fn f64_as_f32(f: f64) -> f32 {
//     if f.is_infinite() {
//         return if f.is_sign_negative() {
//             f32::NEG_INFINITY
//         } else {
//             f32::INFINITY
//         };
//     }
//     if f == 0.0 && f.is_sign_positive() {
//         return 0.0;
//     }
//     if f == 0.0 && f.is_sign_negative() {
//         return -0.0;
//     }
//     let bits = f.to_bits();
//     let mut mant = bits & 0xfffffffffffff;
//     let mut exp = ((bits & (i64::MAX as u64)) >> 52) as i32;
//     if exp != 0 {
//         mant |= 0x10000000000000;

//     } else {
//         exp = 1;
//     }
//     exp -= 1023;
//     //println!("exp: {}", exp);
//     let mut mantissa_shift = 52 - 23;
//     /*if mant.leading_zeros() != 64 - (52 + 1) {
//         exp 
//     }*/
//     if exp >= f32::MAX_EXP {
//         return if f.is_sign_negative() {
//             f32::NEG_INFINITY
//         } else {
//             f32::INFINITY
//         };
//     }
//     if exp < f32::MIN_EXP - 1 {
//         let diff = (f32::MIN_EXP - 1) - exp;
//         mantissa_shift += diff;
//         exp = -(f32::MAX_EXP - 1);
//     }
//     let new_mant = mant.checked_shr(mantissa_shift as u32).unwrap_or(0);
//     //println!("{:025b}", new_mant);

//     let shifted_back = new_mant.checked_shl(mantissa_shift as u32).unwrap_or(0);
//     let overflow = mant ^ shifted_back;
//     /*println!("overflow: {:029b}", overflow);
//     println!("mant: {:053b}", mant);
//     println!("shbk: {:053b}", shifted_back);
//     println!("lz: {}", overflow.leading_zeros());*/
//     if overflow.leading_zeros() as i32 == 64 - mantissa_shift { // this means there is a one at the overflow bit
//         if overflow.count_ones() == 1 { // this means the overflowing is 100...00 so apply ties-to-even rounding
//             if new_mant & 1 == 1 { // if the last bit is 1, then we round up
//                 mant = new_mant + 1;
//                 //println!("updated mant: {:025b}", mant);
//             } else { // otherwise we round down
//                 mant = new_mant;
//             }
//         } else {
//             mant = new_mant + 1; // round up
//         }
//     } else {
//         mant = new_mant;
//     }
//     //1111111111111111111111111
//     //111111111111111111111111
//     if mant.leading_zeros() < 64 - (23 + 1) {
//        // println!("mant overflow");
//         mant >>= 1;
//         exp += 1;
//     }
//     if exp > f32::MAX_EXP {
//         return if f.is_sign_negative() {
//             f32::NEG_INFINITY
//         } else {
//             f32::INFINITY
//         };
//     }
//     mant ^= 0x800000;
//     let sign = (bits >> 63) as u32;
//     let exp = (exp + (f32::MAX_EXP - 1)) as u32;
//     let mant = mant as u32;
//     let bits = (sign << 31) | (exp << 23) | mant;
//     f32::from_bits(bits)
// }

// #[cfg(test)]
// quickcheck::quickcheck! {
//     fn qc_test_f64_as_f32(f: f64) -> quickcheck::TestResult {
//         if !f.is_finite() {
//             return quickcheck::TestResult::discard();
//         }
//         let f2 = f64_as_f32(f);
//         let f3 = f as f32;
//         quickcheck::TestResult::from_bool(f2 == f3)
//     }
// }

// type U32 = BUintD32::<1>;
// fn parse(s: &str) -> (types::U128, U32) {
//     let mut radix = 10;
//     let mut custom_radix = false;
//     let mut src = s;
//     let bytes = s.as_bytes();
//     let len = bytes.len();
//     let mut first_char_zero = false;
//     let mut bit_width = U32::power_of_two(7);
//     let mut i = 0;
//     while i < len {
//         let byte = bytes[i];
//         if i == 0 && byte == b'0' {
//             first_char_zero = true;
//         } else if i == 1 && first_char_zero && (byte == b'b' || byte == b'o' || byte == b'x') {
//             let ptr = unsafe { src.as_ptr().add(2) };
//             let new = core::ptr::slice_from_raw_parts(ptr, src.len() - 2);
//             src = unsafe { &*(new as *const str) };
//             radix = match byte {
//                 b'b' => 2,
//                 b'o' => 8,
//                 b'x' => 16,
//                 _ => unreachable!(),
//             };
//             custom_radix = true;
//         }
//         if i != 0 && i != len - 1 && byte == b'u' {
//             let old_len = src.len();
//             let ptr = src.as_ptr();
            
//             let new_len = if custom_radix { i - 2 } else { i };
//             let bit_width_ptr = core::ptr::slice_from_raw_parts(unsafe { ptr.add(new_len + 1) }, old_len - new_len - 1);
//             let new = core::ptr::slice_from_raw_parts(ptr, new_len);
//             src = unsafe { &*(new as *const str) };
//             let bit_width_str = unsafe { &*(bit_width_ptr as *const str) };
//             bit_width = U32::parse_str_radix(bit_width_str, 10);
//             break;
//         }
//         i += 1;
//     }
//     (types::U128::parse_str_radix(src, radix), bit_width)
// }