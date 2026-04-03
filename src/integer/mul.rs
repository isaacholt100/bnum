use super::Uint;

impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    #[inline]
    pub(crate) const fn mul_u128_digit(self, rhs: u128) -> (Self, bool) {
        let (out, mut overflow) = self.to_digits::<u128>().mul_digit(rhs);
        let mut out = out.to_integer();

        overflow |= !out.has_valid_pad_bits();
        out.set_sign_bits(); // in case of overflow, need to set sign bits

        (out, overflow)
    }
}

// fn karatsuba<const N: usize>(a: Uint<N>, b: Uint<N>, start_index: usize, end_index: usize) -> Uint<N> {
//     if a.last_digit_index() == 0 {
//         return b * a.digits[0];
//     }
//     if b.last_digit_index() == 0 {
//         return a * b.digits[0];
//     }
//     let mid_index = (start_index + end_index) / 2; // TODO: replace this with midpoint
//     let z2 = karatsuba(a, b, mid_index, end_index);
//     let z0 = karatsuba(a, b, end_index, mid_index);
//     // let d1 = abs_diff(x0, x1);
//     // let d2 = abs_diff(y0, y1);
//     //
//     // let a = karatsuba((x0 - x1)(y1 - y0))
//     // let z1 = a + z2

//     todo!()
// }

// fn karat_widening(x: &mut [u8], y: &mut [u8]) -> (&mut [u8], &mut [u8]) {
//     debug_assert_eq!(x.len(), y.len());
//     if x.len() == 1 {
//         let (a, b) = (x[0], y[0]);
//         let (c, d) = a.widening_mul(b);
//         x[0] = c;
//         y[0] = d;
//         return (x, y);
//     }
//     let m = x.len().div_ceil(2); // midpoint index
//     let (x0, x1) = x.split_at_mut(m);
//     let (y0, y1) = y.split_at_mut(m);
//     let (a1karat_widening(x1, y1);
//     karat_widening(x0, y0);
// }

#[cfg(test)]
mod tests {
    crate::test::test_all! {
        testing unsigned;

        quickcheck::quickcheck! {
            fn quickcheck_mul_u128(a: UTest, b: u128) -> quickcheck::TestResult {
                let c = match UTest::try_from(b) {
                    Ok(v) => v,
                    Err(_) => return quickcheck::TestResult::discard(),
                };
                quickcheck::TestResult::from_bool(a.mul_u128_digit(b) == a.overflowing_mul(c))
            }
        }

        #[test]
        fn cases_mul_u128() {
            assert_eq!(UTest::from_byte(27).pow(3).overflowing_mul(UTest::from_byte(8)), UTest::from_byte(27).pow(3).mul_u128_digit(8));
        }
    }
}
