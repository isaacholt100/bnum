use crate::BUintD8;
use crate::ExpType;
use crate::cast::{As, CastFrom};
use super::Float;

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
    use core::str::FromStr;
    use alloc::string::String;

    let digits = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8];
    let parsed = super::F32::parse(&digits, 0);
    let mut s = unsafe { String::from_utf8_unchecked(digits.into_iter().map(|d| d + 48).collect()) };
    s.insert(1, '.');
    s.push_str("");
    // println!("{}", s);
    // println!("{:032b}", parsed.to_bits());
    // println!("{:032b}", f32::from_str(&s).unwrap().to_bits());
    // println!("F: {}", parsed.to_f32());
    // println!("f: {}", f32::from_str(&s).unwrap());
    // println!("n: {}", <f32 as num_traits::Num>::from_str_radix(&s, 10).unwrap());
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