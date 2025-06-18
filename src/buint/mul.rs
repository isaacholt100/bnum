use super::Uint;
use crate::digit;

impl<const N: usize> Uint<N> {
    #[inline]
    pub(super) const fn long_mul(self, rhs: Self) -> (Self, bool) {
        let mut overflow = false;
        let mut out = Self::ZERO;
        let mut carry: u128;

        let mut i = 0;
        while i < Self::U128_DIGITS {
            let self_digit_i = unsafe { self.as_u128_digits().get_with_correct_count(i) }; // it would require a lot of extra code to run the loop where we don't check the correct count (i.e. separate last digit from the rest), and only a linear number of checks won't affect the performance too much
            carry = 0;
            let mut j = 0;
            unsafe {
                while j < Self::U128_DIGITS - 1 - i {
                    let index = i + j;
                    let (prod, c) = digit::carrying_mul_u128(
                        self_digit_i,
                        rhs.as_u128_digits().get(j),
                        carry,
                        out.as_u128_digits().get(index),
                    );
                    out.as_u128_digits_mut().set(index, prod);
                    carry = c;
                    j += 1;
                }
            }
            let (prod, c) = digit::carrying_mul_u128(
                self_digit_i,
                unsafe { rhs.as_u128_digits().get_with_correct_count(j) },
                carry,
                out.as_u128_digits().last(),
            );
            out.as_u128_digits_mut().set_last(prod);
            if Self::U128_DIGIT_REMAINDER != 0 {
                if 128 - Self::U128_BITS_REMAINDER > prod.leading_zeros() {
                    overflow = true;
                }
            }
            if c != 0 {
                overflow = true;
            } else if self_digit_i != 0 {
                if j < Self::U128_DIGITS - 1 && rhs.as_u128_digits().last() != 0 {
                    overflow = true;
                } else {
                    j += 1;
                    unsafe {
                        while j < Self::U128_DIGITS - 1 {
                            if rhs.as_u128_digits().get(j) != 0 {
                                overflow = true;
                                break;
                            }
                            j += 1;
                        }
                    }
                }
            }
            // assert!(!overflow);
            i += 1;
        }
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
