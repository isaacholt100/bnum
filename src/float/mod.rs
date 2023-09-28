use crate::bint::BIntD8;
use crate::cast::{As, CastFrom};
use crate::digit::u8 as digit;
use crate::{BUintD8, ExpType};

type Digit = u8;

#[cfg(test)]
pub type F64 = Float<8, 52>;

#[cfg(test)]
pub type F32 = Float<4, 23>;

macro_rules! handle_nan {
    ($ret: expr; $($n: expr), +) => {
        if $($n.is_nan()) || + {
            return $ret;
        }
    };
}

mod cast;
mod classify;
mod cmp;
mod consts;
mod convert;
mod endian;
mod math;
mod ops;
mod to_str;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct Float<const W: usize, const MB: usize> {
    bits: BUintD8<W>,
}

// TODO: implement rand traits

impl<const W: usize, const MB: usize> Float<W, MB> {
    const MB: ExpType = MB as _;
    const BITS: ExpType = BUintD8::<W>::BITS;

    const EXPONENT_BITS: ExpType = Self::BITS - Self::MB - 1;

    /*const MANTISSA_WORDS: (usize, usize) = (MB / digit::BITS as usize, MB % digit::BITS as usize);

    const EXPONENT_MASK: BUintD8<W> = BUintD8::MAX.wrapping_shl(Self::MB) ^ BIntD8::MIN.to_bits();*/

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

    let digits = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8];
    let parsed = F32::parse(&digits, 0);
    let mut s = unsafe { String::from_utf8_unchecked(digits.into_iter().map(|d| d + 48).collect()) };
    s.insert(1, '.');
    s.push_str("");
    println!("{}", s);
    println!("{:032b}", parsed.to_bits());
    println!("{:032b}", f32::from_str(&s).unwrap().to_bits());
    println!("F: {}", parsed.to_f32());
    println!("f: {}", f32::from_str(&s).unwrap());
    println!("n: {}", <f32 as num_traits::Num>::from_str_radix(&s, 10).unwrap());
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline(always)]
    const fn from_words(words: [Digit; W]) -> Self {
        Self::from_bits(BUintD8::from_digits(words))
    }

    #[inline(always)]
    const fn words(&self) -> &[Digit; W] {
        &self.bits.digits
    }

    #[inline]
    const fn exponent(self) -> BIntD8<W> {
        BIntD8::from_bits(self.exp_mant().0)
    }

    /*const fn actual_exponent(self) -> BIntD8<W> {
        self.exponent() - Self::EXP_BIAS
    }
    const fn unshifted_exponent(self) -> BIntD8<W> {
        BIntD8::from_bits(self.to_bits() & Self::EXPONENT_MASK)
    }*/
    const MANTISSA_MASK: BUintD8<W> = BUintD8::MAX.wrapping_shr(Self::EXPONENT_BITS + 1);
    /*const fn mantissa(self) -> BUintD8<W> {
        self.to_bits() & Self::MANTISSA_MASK
    }
    const fn actual_mantissa(self) -> BUintD8<W> {
        if self.is_subnormal() {
            self.mantissa()
        } else {
            self.mantissa() | (BUintD8::ONE.wrapping_shl(MB))
        }
    }*/
    #[inline(always)]
    const fn to_int(self) -> BIntD8<W> {
        BIntD8::from_bits(self.to_bits())
    }

    #[inline]
    pub const fn copysign(self, sign: Self) -> Self {
        let mut self_words = *self.words();
        if sign.is_sign_negative() {
            self_words[W - 1] |= 1 << (digit::BITS - 1);
        } else {
            self_words[W - 1] &= (!0) >> 1;
        }
        Self::from_bits(BUintD8::from_digits(self_words))
    }

    #[inline]
    pub const fn signum(self) -> Self {
        handle_nan!(Self::NAN; self);
        Self::ONE.copysign(self)
    }

    #[inline]
    pub const fn next_up(self) -> Self {
        use core::num::FpCategory;

        match self.classify() {
            FpCategory::Nan => self,
            FpCategory::Infinite => {
                if self.is_sign_negative() {
                    Self::MIN
                } else {
                    self
                }
            }
            FpCategory::Zero => Self::MIN_POSITIVE_SUBNORMAL,
            _ => {
                if self.is_sign_negative() {
                    Self::from_bits(self.to_bits().sub(BUintD8::ONE))
                } else {
                    Self::from_bits(self.to_bits().add(BUintD8::ONE))
                }
            }
        }
    }

    #[inline]
    pub const fn next_down(self) -> Self {
        use core::num::FpCategory;

        match self.classify() {
            FpCategory::Nan => self,
            FpCategory::Infinite => {
                if self.is_sign_negative() {
                    self
                } else {
                    Self::MAX
                }
            }
            FpCategory::Zero => Self::MAX_NEGATIVE_SUBNORMAL,
            _ => {
                if self.is_sign_negative() {
                    Self::from_bits(self.to_bits().add(BUintD8::ONE))
                } else {
                    Self::from_bits(self.to_bits().sub(BUintD8::ONE))
                }
            }
        }
    }
}

impl<const W: usize, const MB: usize> Default for Float<W, MB> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    // split into sign, exponent and mantissa
    #[inline]
    const fn to_raw_parts(self) -> (bool, BUintD8<W>, BUintD8<W>) {
        let sign = self.is_sign_negative();
        let exp = self.bits.bitand(BIntD8::<W>::MAX.to_bits()).shr(Self::MB);
        let mant = self.bits.bitand(Self::MANTISSA_MASK);

        (sign, exp, mant)
    }

    // split into sign, exponent and mantissa and adjust to reflect actual numerical represenation, but without taking exponent bias into account
    #[inline]
    const fn to_parts_biased(self) -> (bool, BUintD8<W>, BUintD8<W>) {
        let (sign, exp, mant) = self.to_raw_parts();
        if exp.is_zero() {
            (sign, BUintD8::ONE, mant)
        } else {
            (sign, exp, mant.bitor(BUintD8::ONE.shl(Self::MB)))
        }
    }

    /*// split into sign, exponent and mantissa and adjust to reflect actual numerical represenation
    #[inline]
    const fn to_parts(self) -> (bool, BIntD8<W>, BUintD8<W>) {
        let (sign, exp, mant) = self.to_parts_biased();
        (sign, BIntD8::from_bits(exp).sub(Self::EXP_BIAS), mant)
    }*/

    // construct float from sign, exponent and mantissa
    #[inline]
    const fn from_raw_parts(sign: bool, exp: BUintD8<W>, mant: BUintD8<W>) -> Self {
        debug_assert!(
            exp.shl(Self::MB).bitand(mant).is_zero(),
            "mantissa and exponent overlap"
        );
        let mut bits = exp.shl(Self::MB).bitor(mant);
        if sign {
            bits.digits[W - 1] |= 1 << (digit::BITS - 1);
        }
        Self::from_bits(bits)
    }

    /*// construct float from sign, exponent and mantissa, adjusted to reflect actual numerical representation
    #[inline]
    const fn from_parts(sign: bool, exp: BIntD8<W>, mant: BUintD8<W>) -> Self {
        let exp = exp.add(Self::EXP_BIAS);
        todo!()
    }*/

    #[inline]
    const fn exp_mant(&self) -> (BUintD8<W>, BUintD8<W>) {
        let bits = self.bits;
        let exp = (bits.shl(1)).shr(Self::MB + 1);
        let mant = bits.bitand(Self::MANTISSA_MASK);

        if exp.is_zero() {
            (BUintD8::ONE, mant)
        } else {
            (exp, mant.bitor(BUintD8::ONE.shl(Self::MB)))
        }
    }

    /*#[inline]
    pub(super) const fn decode(self) -> (BUintD8<W>, BIntD8<W>) {
        let bits = self.bits;
        let exp = (bits.shl(1)).shr(Self::MB + 1);
        let mant = if exp.is_zero() {
            (bits.bitand(Self::MANTISSA_MASK)).shl(1)
        } else {
            (bits.bitand(Self::MANTISSA_MASK)).bitor((BUintD8::power_of_two(Self::MB)))
        };
        let exp = BIntD8::from_bits(exp)
            .sub(Self::EXP_BIAS)
            .add(MB.as_::<BIntD8<W>>());
        (mant, exp)
    }*/

    #[inline]
    const fn from_exp_mant(negative: bool, exp: BUintD8<W>, mant: BUintD8<W>) -> Self {
        let mut bits = (exp.shl(Self::MB)).bitor(mant);
        if negative {
            bits = bits.bitor(BIntD8::MIN.to_bits());
        }
        let f = Self::from_bits(bits);
        f
    }
}

#[cfg(test)]
impl From<f64> for F64 {
    #[inline]
    fn from(f: f64) -> Self {
        Self::from_bits(f.to_bits().into())
    }
}

#[cfg(test)]
impl From<f32> for F32 {
    #[inline]
    fn from(f: f32) -> Self {
        Self::from_bits(f.to_bits().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::test_bignum;
    use crate::test::types::{ftest, FTEST};

    test_bignum! {
        function: <ftest>::copysign(f1: ftest, f2: ftest)
    }

    test_bignum! {
        function: <ftest>::signum(f: ftest)
    }

    test_bignum! {
        function: <ftest>::next_up(f: ftest)
    }

    test_bignum! {
        function: <ftest>::next_down(f: ftest)
    }
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