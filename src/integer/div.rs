use crate::Exponent;
use crate::{Byte, digit};
use crate::{Int, Integer, Uint};

impl<const N: usize> Uint<N, 0> {
    #[inline]
    const fn sub_partial_digits(&mut self, rhs: Self, start: usize, range: usize) -> bool {
        let mut borrow = false;
        let mut i = 0;
        while i <= range {
            if i + start == N {
                if i < N && rhs.bytes[i] != 0 {
                    borrow = true;
                }
            } else {
                let (sub, overflow) =
                    digit::borrowing_sub(self.bytes[i + start], rhs.bytes[i], borrow);
                self.bytes[i + start] = sub;
                borrow = overflow;
            }
            i += 1;
        }
        borrow
    }
    #[inline]
    const fn add_partial_digits(&mut self, rhs: Self, start: usize, range: usize) {
        let mut carry = false;
        let mut i = 0;
        while i < range {
            let (sum, overflow) = digit::carrying_add(self.bytes[i + start], rhs.bytes[i], carry);
            self.bytes[i + start] = sum;
            carry = overflow;
            i += 1;
        }
        if carry {
            if range + start != N {
                self.bytes[range + start] = self.bytes[range + start].wrapping_add(1);
            }
        }
        // debug_assert!(carry);
    }
    // pub(crate) const fn div_rem_knuth(self, rhs: Self, n: usize) -> (Self, Self) {
    //     // The Art of Computer Programming Volume 2 by Donald Knuth, Section 4.3.1, Algorithm D
    //     // using the improvement in solution to exercise 37 in section 4.3.1 (eliminates the normalisation step)
    //     // n - 1 is the index of the last non-zero wide digit in the divisor
    //     debug_assert!(n >= 2); // if n = 1, then we should have used the division by digit method instead
    //     let e = unsafe {
    //         rhs.as_wide_digits()
    //             .get_with_correct_count(n - 1)
    //             .leading_zeros()
    //     };
    //     let s = u128::BITS - e;
    //     let m = Self::U128_DIGITS - n;

    //     let (v_1dash, v_2dash) = {
    //         let a = unsafe { rhs.as_wide_digits().get(n - 1) };
    //         let b = unsafe { rhs.as_wide_digits().get(n - 2) };

    //         let v1 = (a << e) | (b >> s);
    //         let mut v2 = b << e;
    //         if n > 2 {
    //             let c = unsafe { rhs.as_wide_digits().get(n - 3) };
    //             v2 |= c >> s;
    //         }
    //         (v1, v2)
    //     };

    //     let mut u = self; // remainder
    //     let mut q = Self::ZERO; // quotient

    //     let mut j = m; // D2
    //     while j > 0 {
    //         j -= 1; // D7

    //         let (u_1dash, u_2dash, u_3dash) = {
    //             let a = unsafe { self.as_wide_digits().get(j + n) };
    //             let b = unsafe { self.as_wide_digits().get(j + n - 1) };
    //             let c = unsafe { self.as_wide_digits().get(j + n - 2) };

    //             let u1 = (a << e) | (b >> s);
    //             let u2 = (b << e) | (c >> s);
    //             let mut u3 = c << e;
    //             if j + n > 2 {
    //                 let d = unsafe { self.as_wide_digits().get(j + n - 3) };
    //                 u3 |= d >> s;
    //             }
    //             (u1, u2, u3)
    //         };

    //         #[inline]
    //         const fn tuple_gt(a: (u128, u128), b: (u128, u128)) -> bool {
    //             a.1 > b.1 || a.1 == b.1 && a.0 > b.0
    //         }

    //         // q_hat will be either `q` or `q + 1`
    //         let mut q_hat = if u_1dash < v_1dash {
    //             let (mut q_hat, r_hat) = todo!(); //u128::div_rem_wide(u_2dash, u_1dash, v_1dash); // D3

    //             if tuple_gt(
    //                 digit::carrying_mul_u128(q_hat, v_2dash, 0, 0),
    //                 (u_3dash, r_hat),
    //             ) {
    //                 q_hat -= 1;

    //                 if let Some(r_hat) = r_hat.checked_add(v_1dash) {
    //                     // this checks if `r_hat <= b`, where `b` is the digit base
    //                     if tuple_gt(
    //                         digit::carrying_mul_u128(q_hat, v_2dash, 0, 0),
    //                         (u_3dash, r_hat),
    //                     ) {
    //                         q_hat -= 1;
    //                     }
    //                 }
    //             }
    //             q_hat
    //         } else {
    //             // `u[j + n - 1] >= v[n - 1]` so we know that estimate for q_hat would be larger than `Digit::MAX`. This is either equal to `q` or `q + 1` (very unlikely to be `q + 1`).
    //             u128::MAX
    //         };

    //         // let m = rhs.wrapping_mul(); // this shouldn't overflow: if `q_hat` is larger than 1, then `q_hat * v` is at most `self`.

    //         unsafe {
    //             q.as_wide_digits_mut().set_at_offset(j * 16, q_hat);
    //         }
    //     }
    //     todo!()
    // }
    // pub(crate) const fn div_rem_knuth2(self, rhs: Self, n: usize) -> (Self, Self) {
    //     // The Art of Computer Programming Volume 2 by Donald Knuth, Section 4.3.1, Algorithm D
    //     // using the improvement in solution to exercise 37 in section 4.3.1 (eliminates the normalisation step)
    //     // n - 1 is the index of the last non-zero wide digit in the divisor
    //     debug_assert!(n >= 2); // if n = 1, then we should have used the division by digit method instead
    //     let e = rhs.digits[n - 1].leading_zeros();
    //     let s = u8::BITS - e;
    //     let m = N - n;

    //     let (v_1dash, v_2dash) = {
    //         let a = rhs.digits[n - 1];
    //         let b = rhs.digits[n - 2];

    //         // let v1 = (a << e) | b.unbounded_shr(s);
    //         let mut v2 = b << e;
    //         if n > 2 {
    //             let c = rhs.digits[n - 3];
    //             // v2 |= c.unbounded_shr(s);
    //         }
    //         (v1, v2)
    //     };

    //     let mut u = self; // remainder
    //     let mut q = Self::ZERO; // quotient

    //     let mut j = m + 1; // D2
    //     while j > 0 {
    //         j -= 1; // D7

    //         let (u_1dash, u_2dash, u_3dash) = {
    //             let a = if j == m { 0 } else { self.digits[j + n] };
    //             let b = self.digits[j + n - 1];
    //             let c = self.digits[j + n - 2];

    //             // let u1 = (a << e) | b.unbounded_shr(s);
    //             // let u2 = (b << e) | c.unbounded_shr(s);
    //             let mut u3 = c << e;
    //             if j + n > 2 {
    //                 let d = self.digits[j + n - 3];
    //                 // u3 |= d.unbounded_shr(s);
    //             }
    //             (u1, u2, u3)
    //         };

    //         #[inline]
    //         const fn tuple_gt(a: (Digit, Digit), b: (Digit, Digit)) -> bool {
    //             a.1 > b.1 || a.1 == b.1 && a.0 > b.0
    //         }

    //         // q_hat will be either `q` or `q + 1`
    //         let mut q_hat = if u_1dash < v_1dash {
    //             let (mut q_hat, r_hat) = digit::div_rem_wide(u_2dash, u_1dash, v_1dash); // D3

    //             if tuple_gt(digit::widening_mul(q_hat, v_2dash), (u_3dash, r_hat)) {
    //                 q_hat -= 1;

    //                 if let Some(r_hat) = r_hat.checked_add(v_1dash) {
    //                     // this checks if `r_hat <= b`, where `b` is the digit base
    //                     if tuple_gt(digit::widening_mul(q_hat, v_2dash), (u_3dash, r_hat)) {
    //                         q_hat -= 1;
    //                     }
    //                 }
    //             }
    //             q_hat
    //         } else {
    //             // `u[j + n - 1] >= v[n - 1]` so we know that estimate for q_hat would be larger than `Digit::MAX`. This is either equal to `q` or `q + 1` (very unlikely to be `q + 1`).
    //             Digit::MAX
    //         };

    //         let m = rhs.checked_mul(Self::from_digit(q_hat)).unwrap(); // this shouldn't overflow: if `q_hat` is larger than 1, then `q_hat * v` is at most `self`. // TODO: use single digit multiplication algorithm instead
    //         let borrow = u.sub_partial_digits(m, j, n);

    //         if borrow {
    //             q_hat -= 1;
    //             u.add_partial_digits(rhs, j, n);
    //         }

    //         q.digits[j] = q_hat;
    //     }
    //     (q, u)
    // }

    pub(crate) const fn basecase_div_rem(self, mut v: Self, n: usize) -> (Self, Self) {
        // TODO: can use u128
        // The Art of Computer Programming Volume 2 by Donald Knuth, Section 4.3.1, Algorithm D
        // TODO: use improvement in solution to exercise 37 in section 4.3.1

        let mut q = Self::ZERO;
        let m = self.bits().div_ceil(8) as usize - n;
        let shift = v.bytes[n - 1].leading_zeros() as Exponent;

        v = unsafe { Self::unchecked_shl_internal(v, shift) }; // D1

        #[repr(C)]
        struct Remainder<const M: usize> {
            first: Byte,
            rest: [Byte; M],
        }
        impl<const M: usize> Remainder<M> {
            #[inline]
            const fn digit(&self, index: usize) -> Byte {
                let ptr = self as *const Self as *const u8;
                unsafe { ptr.add(index).read() }
                // if index == 0 {
                //     self.first
                // } else {
                //     self.rest[index - 1]
                // }
            }

            #[inline]
            const fn set_digit(&mut self, index: usize, digit: Byte) {
                let ptr = self as *mut Self as *mut u8;
                unsafe { ptr.add(index).write(digit) }
            }

            const fn shr(self, shift: Exponent) -> Uint<M> {
                let mut out = Uint::ZERO;
                let mut i = 0;
                while i < M {
                    out.bytes[i] = self.digit(i) >> shift;
                    i += 1;
                }
                if shift > 0 {
                    i = 0;
                    while i < M {
                        out.bytes[i] |= self.rest[i] << (Byte::BITS as Exponent - shift);
                        i += 1;
                    }
                }
                out
            }
            const fn new(uint: Uint<M>, shift: Exponent) -> Self {
                let first = uint.bytes[0] << shift;
                let rest = uint.wrapping_shr(Byte::BITS - shift);
                Self {
                    first,
                    rest: rest.bytes,
                }
            }
            const fn sub(&mut self, rhs: Mul<M>, start: usize, range: usize) -> bool {
                let mut borrow = false;
                let mut i = 0;
                while i <= range {
                    let (sub, overflow) =
                        digit::borrowing_sub(self.digit(i + start), rhs.digit(i), borrow);
                    self.set_digit(i + start, sub);
                    // if start == 0 && i == 0 {
                    //     self.first = sub;
                    // } else {
                    //     self.rest[i + start - 1] = sub;
                    // }
                    borrow = overflow;
                    i += 1;
                }
                borrow
            }
            const fn add(&mut self, rhs: Uint<M>, start: usize, range: usize) {
                let mut carry = false;
                let mut i = 0;
                while i < range {
                    let (sum, overflow) =
                        digit::carrying_add(self.digit(i + start), rhs.bytes[i], carry);
                    self.set_digit(i + start, sum);
                    // if start == 0 && i == 0 {
                    //     self.first = sum;
                    // } else {
                    //     self.rest[i + start - 1] = sum;
                    // }
                    carry = overflow;
                    i += 1;
                }
                if carry {
                    self.set_digit(range + start, self.digit(range + start).wrapping_add(1));
                    // if start == 0 && range == 0 {
                    //     self.first = self.first.wrapping_add(1);
                    // } else {
                    //     self.rest[range + start - 1] = self.rest[range + start - 1].wrapping_add(1);
                    // }
                }
            }
        }

        #[repr(C)] // so we can use pointers to speed up indexing
        #[derive(Clone, Copy)]
        struct Mul<const M: usize> {
            rest: [Byte; M],
            last: Byte,
        }
        impl<const M: usize> Mul<M> {
            const fn new(uint: Uint<M>, rhs: Byte) -> Self {
                let mut rest = [0; M];
                let mut carry: Byte = 0;
                let mut i = 0;
                while i < M {
                    let (prod, c) = digit::carrying_mul(uint.bytes[i], rhs, carry, 0);
                    carry = c;
                    rest[i] = prod;
                    i += 1;
                }
                Self { last: carry, rest }
            }
            #[inline]
            const fn digit(&self, index: usize) -> Byte {
                let a = self as *const Self as *const u8;
                unsafe { a.add(index).read() }
                // if index == M {
                //     self.last
                // } else {
                //     self.rest[index]
                // }
            }
        }

        let v_n_m1 = v.bytes[n - 1];
        let v_n_m2 = v.bytes[n - 2];

        let mut u = Remainder::new(self.force_overflow_mode(), shift);

        let mut j = m + 1; // D2
        while j > 0 {
            j -= 1; // D7

            let u_jn = u.digit(j + n);

            #[inline]
            const fn tuple_gt(a: (Byte, Byte), b: (Byte, Byte)) -> bool {
                a.1 > b.1 || a.1 == b.1 && a.0 > b.0
            }

            // q_hat will be either `q` or `q + 1`
            let mut q_hat = if u_jn < v_n_m1 {
                let (mut q_hat, r_hat) = digit::div_rem_wide(u.digit(j + n - 1), u_jn, v_n_m1); // D3

                if tuple_gt(
                    digit::widening_mul(q_hat, v_n_m2),
                    (u.digit(j + n - 2), r_hat as Byte),
                ) {
                    q_hat -= 1;

                    if let Some(r_hat) = r_hat.checked_add(v_n_m1) {
                        // this checks if `r_hat <= b`, where `b` is the digit base
                        if tuple_gt(
                            digit::widening_mul(q_hat, v_n_m2),
                            (u.digit(j + n - 2), r_hat as Byte),
                        ) {
                            q_hat -= 1;
                        }
                    }
                }
                q_hat
            } else {
                // `u[j + n - 1] >= v[n - 1]` so we know that estimate for q_hat would be larger than `Digit::MAX`. This is either equal to `q` or `q + 1` (very unlikely to be `q + 1`).
                Byte::MAX
            };
            let overflow = u.sub(Mul::new(v, q_hat), j, n); // D4

            if overflow {
                // D5 - unlikely, probability of this being true is ~ 2 / b where b is the digit base (i.e. `Digit::MAX + 1`)
                q_hat -= 1;
                u.add(v, j, n);
            }
            q.bytes[j] = q_hat;
        }
        (q, u.shr(shift))
    }

    // pub(crate) const fn basecase_div_rem_wide(self, mut v: Self, n: usize) -> (Self, Self) {
    //     // TODO: can use u128
    //     // The Art of Computer Programming Volume 2 by Donald Knuth, Section 4.3.1, Algorithm D
    //     // TODO: use improvement in solution to exercise 37 in section 4.3.1

    //     let mut q = Self::ZERO;
    //     let m = self.bits().div_ceil(8) as usize + 1 - n;
    //     let shift = unsafe { v.as_wide_digits().u64_digit(n - 1).leading_zeros() };

    //     v = unsafe { Self::unchecked_shl_internal(v, shift) }; // D1

    //     #[repr(C)]
    //     struct Remainder<const M: usize> {
    //         first: [Digit; 8], // 8 bytes for a u64
    //         rest: [Digit; M],
    //     }
    //     impl<const M: usize> Remainder<M> {
    //         #[inline]
    //         const fn digit(&self, index: usize) -> Digit {
    //             let ptr = self as *const Self as *const u8;
    //             unsafe { ptr.add(index).read() }
    //             // if index == 0 {
    //             //     self.first
    //             // } else {
    //             //     self.rest[index - 1]
    //             // }
    //         }

    //         #[inline]
    //         const fn set_digit(&mut self, index: usize, digit: Digit) {
    //             let ptr = self as *mut Self as *mut u8;
    //             unsafe { ptr.add(index).write(digit) }
    //         }

    //         const fn shr(self, shift: Exponent) -> Uint<M> {
    //             let mut out = Uint::ZERO;
    //             let mut i = 0;
    //             while i < M {
    //                 out.digits[i] = self.digit(i) >> shift;
    //                 i += 1;
    //             }
    //             if shift > 0 {
    //                 i = 0;
    //                 while i < M {
    //                     out.digits[i] |= self.rest[i] << (Byte::BITS as Exponent - shift);
    //                     i += 1;
    //                 }
    //             }
    //             out
    //         }
    //         const fn new(uint: Uint<M>, shift: Exponent) -> Self {
    //             let first = unsafe { uint.as_wide_digits().u64_digit(0) << shift };
    //             let rest = uint.wrapping_shr(Byte::BITS - shift);
    //             Self {
    //                 first,
    //                 rest: rest.digits,
    //             }
    //         }
    //         const fn sub(&mut self, rhs: Mul<M>, start: usize, range: usize) -> bool {
    //             let mut borrow = false;
    //             let mut i = 0;
    //             while i <= range {
    //                 let (sub, overflow) =
    //                     digit::borrowing_sub(self.digit(i + start), rhs.digit(i), borrow);
    //                 self.set_digit(i + start, sub);
    //                 // if start == 0 && i == 0 {
    //                 //     self.first = sub;
    //                 // } else {
    //                 //     self.rest[i + start - 1] = sub;
    //                 // }
    //                 borrow = overflow;
    //                 i += 1;
    //             }
    //             borrow
    //         }
    //         const fn add(&mut self, rhs: Uint<M>, start: usize, range: usize) {
    //             let mut carry = false;
    //             let mut i = 0;
    //             while i < range {
    //                 let (sum, overflow) =
    //                     digit::carrying_add(self.digit(i + start), rhs.digits[i], carry);
    //                 self.set_digit(i + start, sum);
    //                 // if start == 0 && i == 0 {
    //                 //     self.first = sum;
    //                 // } else {
    //                 //     self.rest[i + start - 1] = sum;
    //                 // }
    //                 carry = overflow;
    //                 i += 1;
    //             }
    //             if carry {
    //                 self.set_digit(range + start, self.digit(range + start).wrapping_add(1));
    //                 // if start == 0 && range == 0 {
    //                 //     self.first = self.first.wrapping_add(1);
    //                 // } else {
    //                 //     self.rest[range + start - 1] = self.rest[range + start - 1].wrapping_add(1);
    //                 // }
    //             }
    //         }
    //     }

    //     #[repr(C)] // so we can use pointers to speed up indexing
    //     #[derive(Clone, Copy)]
    //     struct Mul<const M: usize> {
    //         rest: [Digit; M],
    //         last: Digit,
    //     }
    //     impl<const M: usize> Mul<M> {
    //         const fn new(uint: Uint<M>, rhs: Digit) -> Self {
    //             let mut rest = [0; M];
    //             let mut carry: Digit = 0;
    //             let mut i = 0;
    //             while i < M {
    //                 let (prod, c) = digit::carrying_mul(uint.digits[i], rhs, carry, 0);
    //                 carry = c;
    //                 rest[i] = prod;
    //                 i += 1;
    //             }
    //             Self { last: carry, rest }
    //         }
    //         #[inline]
    //         const fn digit(&self, index: usize) -> Digit {
    //             let a = self as *const Self as *const u8;
    //             unsafe { a.add(index).read() }
    //             // if index == M {
    //             //     self.last
    //             // } else {
    //             //     self.rest[index]
    //             // }
    //         }
    //     }

    //     let v_n_m1 = v.digits[n - 1];
    //     let v_n_m2 = v.digits[n - 2];

    //     let mut u = Remainder::new(self, shift);

    //     let mut j = m + 1; // D2
    //     while j > 0 {
    //         j -= 1; // D7

    //         let u_jn = u.digit(j + n);

    //         #[inline]
    //         const fn tuple_gt(a: (Digit, Digit), b: (Digit, Digit)) -> bool {
    //             a.1 > b.1 || a.1 == b.1 && a.0 > b.0
    //         }

    //         // q_hat will be either `q` or `q + 1`
    //         let mut q_hat = if u_jn < v_n_m1 {
    //             let (mut q_hat, r_hat) = digit::div_rem_wide(u.digit(j + n - 1), u_jn, v_n_m1); // D3

    //             if tuple_gt(
    //                 digit::widening_mul(q_hat, v_n_m2),
    //                 (u.digit(j + n - 2), r_hat as Digit),
    //             ) {
    //                 q_hat -= 1;

    //                 if let Some(r_hat) = r_hat.checked_add(v_n_m1) {
    //                     // this checks if `r_hat <= b`, where `b` is the digit base
    //                     if tuple_gt(
    //                         digit::widening_mul(q_hat, v_n_m2),
    //                         (u.digit(j + n - 2), r_hat as Digit),
    //                     ) {
    //                         q_hat -= 1;
    //                     }
    //                 }
    //             }
    //             q_hat
    //         } else {
    //             // `u[j + n - 1] >= v[n - 1]` so we know that estimate for q_hat would be larger than `Digit::MAX`. This is either equal to `q` or `q + 1` (very unlikely to be `q + 1`).
    //             Digit::MAX
    //         };
    //         let overflow = u.sub(Mul::new(v, q_hat), j, n); // D4

    //         if overflow {
    //             // D5 - unlikely, probability of this being true is ~ 2 / b where b is the digit base (i.e. `Digit::MAX + 1`)
    //             q_hat -= 1;
    //             u.add(v, j, n);
    //         }
    //         q.digits[j] = q_hat;
    //     }
    //     (q, u.shr(shift))
    // }

    #[inline]
    pub(crate) const fn div_rem_unchecked_unsigned(self, rhs: Self) -> (Self, Self) {
        use core::cmp::Ordering;

        match self.cmp(&rhs) {
            Ordering::Less => (Self::ZERO, self),
            Ordering::Equal => (Self::ONE, Self::ZERO),
            Ordering::Greater => {
                let bit_width = rhs.bits();
                if bit_width <= 64 {
                    let d = unsafe { rhs.as_wide_digits().u64_digit(0) };
                    let (div, rem) = self.div_rem_u64(d);
                    let mut out = Self::ZERO;
                    unsafe { out.as_wide_digits_mut().set_u64_digit(0, rem) };
                    (div, out)
                } else {
                    // if rhs.is_power_of_two() {
                    //     return (self.wrapping_shr(rhs.ilog2()), self.bitand(rhs.wrapping_sub(Self::ONE)));
                    // }
                    self.basecase_div_rem(rhs, rhs.bits().div_ceil(8) as usize)
                }
            }
        }
    }
}

impl<const N: usize> Int<N, 0> {
    #[inline]
    pub(crate) const fn div_rem_unchecked_signed(self, rhs: Self) -> (Self, Self) {
        let (div, rem) = self.unsigned_abs().div_rem_unchecked(rhs.unsigned_abs());
        let (div, rem) = (div.cast_signed(), rem.cast_signed());

        match (self.is_negative(), rhs.is_negative()) {
            (false, false) => (div, rem),
            (false, true) => (div.wrapping_neg(), rem), // use wrapping_neg for the case that self is Self::MIN and rhs is 1 or -1
            (true, false) => (div.wrapping_neg(), rem.neg()),
            (true, true) => (div, rem.neg()),
        }
    }

    #[inline]
    pub(crate) const fn div_rem_euclid_unchecked_signed(self, rhs: Self) -> (Self, Self) {
        let (div, rem) = self.unsigned_abs().div_rem_unchecked(rhs.unsigned_abs());
        let (div, rem) = (div.cast_signed(), rem.cast_signed());

        match (self.is_negative(), rhs.is_negative()) {
            (false, false) => (div, rem),
            (false, true) => (div.wrapping_neg(), rem), // use wrapping_neg for the case that self is Self::MIN and rhs is 1 or -1
            (true, false) => {
                if rem.is_zero() {
                    (div.wrapping_neg(), rem.neg())
                } else {
                    // quotient should be div.neg() - 1
                    // but div.neg() = div.not() + 1
                    // so just return div.not()
                    (div.not(), rem.neg().add(rhs))
                }
            }
            (true, true) => {
                if rem.is_zero() {
                    (div, rem.neg())
                } else {
                    (div.add(Self::ONE), rem.neg().sub(rhs))
                }
            }
        }
    }
}

impl<const N: usize, const B: usize, const OM: u8> Uint<N, B, OM> {
    #[inline]
    pub(crate) const fn div_rem_u64(self, rhs: u64) -> (Self, u64) {
        let mut out = Self::ZERO;
        let mut rem: u64 = 0;
        let mut i = N.div_ceil(8);
        while i > 0 {
            i -= 1;
            let d = unsafe { self.as_wide_digits().u64_digit(i) };
            let (q, r) = digit::div_rem_wide_u64(d, rem, rhs);
            rem = r;
            unsafe { out.as_wide_digits_mut().set_u64_digit(i, q) };
        }
        (out, rem)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    // don't check that rhs is zero or (if signed) that self is Self::MIN and RHS is -1
    #[inline]
    pub(crate) const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
        if S {
            let (d, r) = self
                .force()
                .div_rem_unchecked_signed(rhs.force());
            (d.force(), r.force())
        } else {
            let (d, r) = self
                .force()
                .div_rem_unchecked_unsigned(rhs.force());
            (d.force(), r.force())
        }
    }

    // don't check that rhs is zero or (if signed) that self is Self::MIN and RHS is -1
    #[inline]
    pub(crate) const fn div_rem_euclid_unchecked(self, rhs: Self) -> (Self, Self) {
        if S {
            let (d, r) = self
                .force()
                .div_rem_euclid_unchecked_signed(rhs.force());
            (d.force(), r.force())
        } else {
            self.div_rem_unchecked(rhs)
        }
    }

    #[inline(always)]
    pub(crate) const fn is_division_overflow(&self, rhs: &Self) -> bool {
        S && self.eq(&Self::MIN) && rhs.force_sign().eq(&Int::NEG_ONE)
    }
}
