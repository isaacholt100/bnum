use super::Uint;
use crate::digit;

impl<const N: usize,  const B: usize, const OM: u8> Uint<N, B, OM> {
    // naive O(N^2) "digit by digit" multiplication
    #[inline]
    pub(super) const fn long_mul(self, rhs: Self) -> (Self, bool) {
        let mut overflow = false;
        let mut out = Self::ZERO;
        let (mut prod, mut carry): (u128, u128);

        let mut i = 0;
        while i < Self::U128_DIGITS {
            let self_digit_i = unsafe { self.as_wide_digits().get(i) };
            carry = 0;
            let mut j = 0;
            unsafe {
                while j < Self::U128_DIGITS - 1 - i {
                    let index = i + j;
                    (prod, carry) = digit::carrying_mul_u128(
                        self_digit_i,
                        rhs.as_wide_digits().get(j),
                        carry,
                        out.as_wide_digits().get(index),
                    );
                    out.as_wide_digits_mut().set(index, prod);
                    j += 1;
                }
            }
            // unfortunately, we have to handle the last digit separately, as otherwise we need to initialise prod, which slows performance considerably
            let (prod, c) = digit::carrying_mul_u128(
                self_digit_i,
                unsafe { rhs.as_wide_digits().get(j) },
                carry,
                out.as_wide_digits().last(),
            );
            out.as_wide_digits_mut().set_last(prod);

            if Self::U128_BITS_REMAINDER != 0 {
                if 128 - Self::U128_BITS_REMAINDER > prod.leading_zeros() { // prod needs to be initialised here
                    overflow = true;
                }
            }
            if c != 0 {
                overflow = true;
            } else if self_digit_i != 0 {
                if j < Self::U128_DIGITS - 1 && rhs.as_wide_digits().last() != 0 {
                    overflow = true;
                } else {
                    j += 1;
                    unsafe {
                        while j < Self::U128_DIGITS - 1 {
                            if rhs.as_wide_digits().get(j) != 0 {
                                overflow = true;
                                break;
                            }
                            j += 1;
                        }
                    }
                }
            }
            i += 1;
        }
        out.set_sign_bits(); // in case of overflow, need to set sign bits
        (out, overflow)
    }

    #[inline]
    pub(crate) const fn mul_u128_digit(self, rhs: u128) -> (Self, bool) {
        let mut out = Self::ZERO;
        let (mut prod, mut carry) = (0, 0);
        let mut i = 0;
        while i < Self::U128_DIGITS {
            let d = unsafe { self.as_wide_digits().get(i) };
            (prod, carry) = digit::carrying_mul_u128(d, rhs, carry, 0);
            unsafe {
                out.as_wide_digits_mut().set(i, prod);
            }
            i += 1;
        }
        let overflow = carry != 0 || (Self::U128_BITS_REMAINDER != 0 && 128 - prod.leading_zeros() > Self::U128_BITS_REMAINDER);

        out.set_sign_bits();
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
            fn quickcheck_mul_u128(a: UTEST, b: u128) -> quickcheck::TestResult {
                let c = match UTEST::try_from(b) {
                    Ok(v) => v,
                    Err(_) => return quickcheck::TestResult::discard(),
                };
                quickcheck::TestResult::from_bool(a.mul_u128_digit(b) == a.overflowing_mul(c))
            }
        }

        #[test]
        fn cases_mul_u128() {
            assert_eq!(UTEST::from_byte(27).pow(3).overflowing_mul(UTEST::from_byte(8)), UTEST::from_byte(27).pow(3).mul_u128_digit(8));
        }
    }
}