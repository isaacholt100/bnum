use super::BUintD8;
use crate::{digit, Digit};

impl<const N: usize> BUintD8<N> {
    #[inline]
    pub(super) const fn long_mul(self, rhs: Self) -> (Self, bool) {
        // TODO: can use u128
        let mut overflow = false;
        let mut out = Self::ZERO;
        let mut carry: Digit;

        let mut i = 0;
        while i < N {
            carry = 0;
            let mut j = 0;
            while j < N {
                let index = i + j;
                if index < N {
                    let (prod, c) = digit::carrying_mul(
                        self.digits[i],
                        rhs.digits[j],
                        carry,
                        out.digits[index],
                    );
                    out.digits[index] = prod;
                    carry = c;
                } else if self.digits[i] != 0 && rhs.digits[j] != 0 {
                    overflow = true;
                    break;
                }
                j += 1;
            }
            if carry != 0 {
                overflow = true;
            }
            i += 1;
        }
        (out, overflow)
    }
}

// fn karatsuba<const N: usize>(a: BUintD8<N>, b: BUintD8<N>, start_index: usize, end_index: usize) -> BUintD8<N> {
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
