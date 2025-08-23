use crate::Uint;
use crate::ExpType;
use crate::cast::{As, CastFrom};
use super::Float;

// TODO: look at algorithm 1 in https://arxiv.org/pdf/2101.11408 (can use const fn and associated constants for lookup table for powers of 5)

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub fn parse(digits: &[u8], exp: i32) -> Self where [u8; W * 2]: {
        let one = Self::ONE;
        let two = Self::TWO;
        let three: Self = 3u8.as_();
        let four: Self = 4u8.as_();
        let five: Self = 5u8.as_();
        let six: Self = 6u8.as_();
        let seven: Self = 7u8.as_();
        let eight: Self = 8u8.as_();
        let nine: Self = 9u8.as_();
        let ten: Self = 10u8.as_();

        let mut out = Self::ZERO;
        let mut pow = Self::ONE;
        for d in digits {
            let a = Self::cast_from(*d) * pow;

            out = out + a;
            pow = pow / ten;
        }
        out.powi(exp)
    }
}

#[test]
fn test_parse() {
    // use core::str::FromStr;
    // use alloc::string::String;

    // let digits = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8];
    // let parsed = super::F32::parse(&digits, 0);
    // let mut s = unsafe { String::from_utf8_unchecked(digits.into_iter().map(|d| d + 48).collect()) };
    // s.insert(1, '.');
    // s.push_str("");
    // println!("{}", s);
    // println!("{:032b}", parsed.to_bits());
    // println!("{:032b}", f32::from_str(&s).unwrap().to_bits());
    // println!("F: {}", parsed.to_f32());
    // println!("f: {}", f32::from_str(&s).unwrap());
    // println!("n: {}", <f32 as num_traits::Num>::from_str_radix(&s, 10).unwrap());
}

// TODO: create round-to-nearest ties-to-even function, it could take a uint and a target bit width, and return the correctly rounded result in the target precision, as well as the overflow, and whether a round up occurred

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