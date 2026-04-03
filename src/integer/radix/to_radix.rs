use crate::{Byte, Integer, Uint};
use super::MAX_RADIX_POWERS;
use crate::integer::radix::assert_range;
use alloc::{string::String, vec::Vec};

#[inline]
const fn digit_to_str_byte(digit: u8) -> u8 {
    if digit < 10 {
        digit + b'0'
    } else {
        digit + b'a' - 10
    }
}

macro_rules! impl_desc {
    () => {
        "Methods which convert integers to strings and lists of digits in a given radix (base)."
    };
}

// struct UintDigitsLeIter<const N: usize, const B: usize, const OM: u8> {
//     radix: u64,
//     q: Uint<N, B, OM>,
//     at_last: bool,
// }

// impl<const N: usize, const B: usize, const OM: u8> UintDigitsLeIter<N, B, OM> {
//     #[inline]
//     fn new(n: Uint<N, B, OM>, radix: u64) -> Self {
//         Self {
//             radix,
//             q: n,
//             at_last: n.is_zero(),
//         }
//     }
// }

// impl<const N: usize, const B: usize, const OM: u8> Iterator for UintDigitsLeIter<N, B, OM> {
//     type Item = u64;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.at_last {
//             None
//         } else {
//             let (q_dash, r) = self.q.div_rem_u64(self.radix);
//             self.q = q_dash;
//             self.at_last = self.q.is_zero();
//             Some(r)
//         }
//     }
// }

// struct UintDigitsBeIter<const N: usize, const B: usize, const OM: u8> {
//     radix: u64,
//     q: Uint<N, B, OM>,
//     at_last: bool,
// }

// impl<const N: usize, const B: usize, const OM: u8> UintDigitsBeIter<N, B, OM> {
//     #[inline]
//     fn new(n: Uint<N, B, OM>, radix: u64) -> Self {
//         Self {
//             radix,
//             q: n,
//             at_last: n.is_zero(),
//         }
//     }
// }

// impl<const N: usize, const B: usize, const OM: u8> Iterator for UintDigitsBeIter<N, B, OM> {
//     type Item = u64;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         let (power, exponent) = self.q.largest_power_less_equal(self.radix);
//         let (div, rem) = self.q.div_rem(power);
//     }
// }

// struct RadixDigitsIter<const N: usize, const B: usize, const OM: u8> {
//     big_iter: UintDigitsLeIter<N, B, OM>,
//     u64_digit: u64,
//     i: usize,
//     radix: u32,
// }

// impl<const N: usize, const B: usize, const OM: u8> RadixDigitsIter<N, B, OM> {
//     #[inline]
//     fn new(n: Uint<N, B, OM>, radix: u32) -> Self {
//         let mut big_iter = UintDigitsLeIter::new(n, MAX_RADIX_POWERS[radix as usize].0);
//         let u64_digit = big_iter.next().unwrap_or(0);
//         Self {
//             big_iter,
//             u64_digit,
//             radix,
//             i: 0,
//         }
//     }
// }

// impl<const N: usize, const B: usize, const OM: u8> Iterator for RadixDigitsIter<N, B, OM> {
//     type Item = u8;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.big_iter.at_last {
//             if self.u64_digit == 0 {
//                 return None;
//             }
//             let digit = (self.u64_digit % (self.radix as u64)) as u8;
//             self.u64_digit /= self.radix as u64;
//             return Some(digit);
//         }
//         if self.i < MAX_RADIX_POWERS[self.radix as usize].1 {
//             let digit = (self.u64_digit % (self.radix as u64)) as u8;
//             self.u64_digit /= self.radix as u64;
//             self.i += 1;
//             Some(digit)
//         } else {
//             self.i = 0;
//             self.u64_digit = self.big_iter.next()?;
//             self.next()
//         }
//     }

//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let upper = Uint::<N, B>::BITS.div_ceil(self.radix.ilog2()) as usize;
//         (0, Some(upper))
//     }
// }

// struct RadixDigitsIter2<const N: usize, const B: usize, const OM: u8> {
//     q: Uint<N, B, OM>,
//     r: u64,
//     i: usize,
//     radix: u32,
//     at_last: bool,
// }

// impl<const N: usize, const B: usize, const OM: u8> RadixDigitsIter2<N, B, OM> {
//     #[inline]
//     fn new(n: Uint<N, B, OM>, radix: u32) -> Self {
//         Self {
//             q: n,
//             r: 0,
//             radix,
//             i: MAX_RADIX_POWERS[radix as usize].1,
//             at_last: n.is_zero(),
//         }
//     }
// }

// impl<const N: usize, const B: usize, const OM: u8> Iterator for RadixDigitsIter2<N, B, OM> {
//     type Item = u8;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.at_last {
//             if self.r == 0 {
//                 return None;
//             }
//             let digit = (self.r % (self.radix as u64)) as u8;
//             self.r /= self.radix as u64;
//             return Some(digit);
//         }
//         if self.i == MAX_RADIX_POWERS[self.radix as usize].1 {
//             let (div, rem) = self.q.div_rem_u64(MAX_RADIX_POWERS[self.radix as usize].0);
//             self.q = div;
//             self.r = rem;
//             self.at_last = self.q.is_zero();
//             self.i = 0;
//             return self.next();
//         }
//         let digit = (self.r % (self.radix as u64)) as u8;
//         self.r /= self.radix as u64;
//         self.i += 1;
//         Some(digit)
//     }

//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let upper = Uint::<N, B>::BITS.div_ceil(self.radix.ilog2()) as usize;
//         (0, Some(upper))
//     }
// }



#[doc = concat!("(Unsigned integers only.) ", impl_desc!())]
impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    // this is faster than just using div_rem_u64 by the radix: for the naive method, we perform log_r (n) division-remainder calcs on Uints, each one takes O(M(n)) time, where M(n) is multiplication time complexity
    // for dividing by highest power h = r^e, we perform log_h (n) = log_r (n) / e division-remainder calcs on Uints, each one takes O(M(n)) time. for such calc, we perform e division-remainder calcs on u64s, each one takes O(1) time
    // so number of division-remainder calcs is same, but we save a factor of e in the complexity
    #[inline] 
    fn to_digits_le(self, radix: u32) -> Vec<u8> {
        // return RadixDigitsIter2::new(self, radix).collect();
        let mut digits = Vec::with_capacity(Self::BITS.div_ceil(radix.ilog2()) as usize); // log_r (2^B) = B log_r (2) = B/log_2 (r)
        let mut current = self;
        let radix_u64 = radix as u64;
        let (max_pow, max_pow_exponent) = MAX_RADIX_POWERS[radix as usize];
        loop {
            let (q, mut r) = current.div_rem_u64(max_pow); // if max_pow is larger than Self::MAX, this is ok, will just mean q = 0, r = current
            if q.is_zero() {
                while r != 0 {
                    digits.push((r % radix_u64) as u8);
                    r /= radix_u64;
                }
                return digits;
            }
            for _ in 0..max_pow_exponent {
                digits.push((r % radix_u64) as u8); // guaranteed to fit into u8 as radix_u64 <= 256
                r /= radix_u64;
            }
            current = q;
        }
    }

    #[inline]
    fn to_inexact_bitwise_digits_le(self, radix: u32) -> Vec<u8> {
        let bit_width = self.bit_width();
        let radix_log2 = radix.ilog2();
        let mask = u8::MAX >> (u8::BITS - radix_log2);

        let mut digits = Vec::with_capacity(Self::BITS.div_ceil(radix_log2) as usize);

        let num_non_zero_digits = bit_width.div_ceil(u128::BITS) as usize; // number of non-zero u128 digits
        let mut offset_bit_width = 0;
        let mut carry_bits = 0;

        let self_digits = self.to_digits::<u128>();

        for i in 0..num_non_zero_digits {
            let d = self_digits.get(i); // we use wide digits as there will be fewer times when the accessed bits spans two consecutive digits, rather than just within one digit
            let digit = (d << offset_bit_width) as u8 & mask; // can truncate to u8 as this is equivalent to bitand-ing with zeros
            digits.push(digit | carry_bits);

            let mut j = radix_log2 - offset_bit_width;
            while j <= u128::BITS - radix_log2 {
                let digit = (d >> j) as u8 & mask; // can truncate to u8 as this is equivalent to bitand-ing with zeros
                digits.push(digit);

                j += radix_log2;
            }

            offset_bit_width = radix_log2 - ((j + radix_log2) % u128::BITS); // this is faster than j - u128::BITS as uses bit and optimisation

            carry_bits = (d >> j) as u8; // can truncate to u8 as this is equivalent to bitand-ing with zeros
        }

        if carry_bits != 0 {
            digits.push(carry_bits);
        }

        while let Some(&0) = digits.last() {
            digits.pop();
        }

        digits
    }

    #[inline]
    fn to_exact_bitwise_digits_le(self, radix: u32) -> Vec<u8> {
        if radix == 256 {
            let last_non_zero_byte_index = self.bytes.iter().rposition(|&b| b != 0).unwrap_or(0);
            return self.bytes[..=last_non_zero_byte_index].to_vec();
        }

        let radix_log2 = radix.ilog2();
        let mask = u8::MAX >> (u8::BITS - radix_log2);
        let mut digits = Vec::with_capacity(Self::BITS.div_ceil(radix_log2) as usize);
        debug_assert!(mask.trailing_ones() == radix_log2);
        debug_assert!(mask.count_ones() == radix_log2); // mask is l low-order 1s
        let num_non_zero_digits = self.bit_width().div_ceil(Byte::BITS) as usize;
        let digits_per_big_digit = Byte::BITS / radix_log2;

        for i in 0..num_non_zero_digits - 1 {
            let mut d = self.bytes[i]; // faster to use bytes than wide digits here
            for _ in 0..digits_per_big_digit {
                let digit = d & mask; // can truncate to u32 as this is equivalent to bitand-ing with zeros
                digits.push(digit);
                d >>= radix_log2; // this would panic if radix_log2 is 8, so we have separate case for radix 256
            }
        }
        let mut d = self.bytes[num_non_zero_digits - 1];
        while d != 0 {
            let digit = d & mask; // can truncate to u32 as this is equivalent to bitand-ing with zeros
            digits.push(digit);
            d >>= radix_log2;
        }
        digits
    }

    /// Returns the integer in the given base in big-endian digit order.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let digits = &[3, 55, 60, 100, 5, 0, 5, 88];
    /// let n = U512::from_radix_be(digits, 120).unwrap();
    /// assert_eq!(n.to_radix_be(120), digits);
    /// ```
    #[inline]
    pub fn to_radix_be(&self, radix: u32) -> Vec<u8> {
        let mut v = self.to_radix_le(radix);
        v.reverse();
        v
    }

    /// Returns the integer in the given base in little-endian digit order.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 256 inclusive.
    ///
    /// ```
    /// use bnum::types::U512;
    ///
    /// let digits = &[1, 67, 88, 200, 55, 68, 87, 120, 178];
    /// let n = U512::from_radix_le(digits, 250).unwrap();
    /// assert_eq!(n.to_radix_le(250), digits);
    /// ```
    pub fn to_radix_le(&self, radix: u32) -> Vec<u8> {
        assert_range!(radix, 256);
        if self.is_zero() {
            return vec![0];
        }
        match radix {
            2 | 4 | 16 | 256 => {
                self.to_exact_bitwise_digits_le(radix)
            },
            8 | 32 | 64 | 128 => {
                self.to_inexact_bitwise_digits_le(radix)
            },
            10 => self.to_digits_le(10),
            _ => self.to_digits_le(radix),
        }
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    /// Returns the integer as a string in the given radix.
    ///
    /// # Panics
    ///
    /// This function panics if `radix` is not in the range from 2 to 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    ///
    /// let src = "abcdefghijklmnopqrstuvwxyz";
    /// let n = U512::from_str_radix(src, 36).unwrap();
    /// assert_eq!(n.to_str_radix(36), src);
    /// 
    /// let a: I512 = n!(-0o123456701234567);
    /// assert_eq!(a.to_str_radix(8), "-123456701234567");
    /// ```
    #[inline]
    pub fn to_str_radix(&self, radix: u32) -> String {
        if self.is_negative_internal() {
            return format!("-{}", self.unsigned_abs_internal().to_str_radix(radix));
        }

        assert_range!(radix, 36);

        let mut out = self.force_sign::<false>().to_radix_be(radix);

        for byte in out.iter_mut() {
            *byte = digit_to_str_byte(*byte);
        }

        unsafe { String::from_utf8_unchecked(out) }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! quickcheck_from_to_radix {
        ($primitive: ty, $name: ident, $max: expr) => {
            paste::paste! {
                quickcheck::quickcheck! {
                    fn [<quickcheck_from_to_ $name>](u: $primitive, radix: crate::test::Radix<$max>) -> quickcheck::TestResult {
                        use crate::cast::CastFrom;
                        // dbg!(u, radix);
                        // println!("{:064x}", u);
                        let radix = radix.0;
                        let u = <[<$primitive:upper>]>::cast_from(u);
                        let v = u.[<to_ $name>](radix as u32);
                        let u1 = <[<$primitive:upper>]>::[<from_ $name>](&v, radix as u32).unwrap();
                        // assert_eq!(u, u1);
                        // dbg!(u, u1);
                        // if u != u1 {
                        //     panic!("{} {}", u, u1);
                        // }
                        quickcheck::TestResult::from_bool(u == u1)
                    }
                }
            }
        }
    }

    #[test]
    fn aaa() {
        let a = crate::n!(0xd73104da53b99783U64);
        dbg!(a.to_radix_le(256));
    }

    crate::test::test_all! {
        testing integers;
        
        quickcheck_from_to_radix!(stest, str_radix, 36);
    }

    crate::test::test_all! {
        testing unsigned;

        quickcheck_from_to_radix!(utest, radix_be, 256);
        quickcheck_from_to_radix!(utest, radix_le, 256);
    }
}
