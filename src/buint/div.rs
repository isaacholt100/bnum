use super::BUintD8;
use crate::{digit, Digit};
use crate::ExpType;

impl<const N: usize> BUintD8<N> {
    pub(crate) const fn basecase_div_rem(self, mut v: Self, n: usize) -> (Self, Self) {
        // The Art of Computer Programming Volume 2 by Donald Knuth, Section 4.3.1, Algorithm D

        let mut q = Self::ZERO;
        let m = self.last_digit_index() + 1 - n;
        let shift = v.digits[n - 1].leading_zeros() as ExpType;

        v = unsafe { Self::unchecked_shl_internal(v, shift) }; // D1

        struct Remainder<const M: usize> {
            first: Digit,
            rest: [Digit; M],
        }
        impl<const M: usize> Remainder<M> {
            const fn digit(&self, index: usize) -> Digit {
                if index == 0 {
                    self.first
                } else {
                    self.rest[index - 1]
                }
            }
            const fn shr(self, shift: ExpType) -> BUintD8<M> {
                let mut out = BUintD8::ZERO;
                let mut i = 0;
                while i < M {
                    out.digits[i] = self.digit(i) >> shift;
                    i += 1;
                }
                if shift > 0 {
                    i = 0;
                    while i < M {
                        out.digits[i] |= self.rest[i] << (digit::BITS as ExpType - shift);
                        i += 1;
                    }
                }
                out
            }
            const fn new(uint: BUintD8<M>, shift: ExpType) -> Self {
                let first = uint.digits[0] << shift;
                let rest = uint.wrapping_shr(digit::BITS - shift);
                Self {
                    first,
                    rest: rest.digits,
                }
            }
            /*crate::nightly::const_fns! {
                const fn set_digit(&mut self, index: usize, digit: Digit) -> () {
                    if index == 0 {
                        self.first = digit;
                    } else {
                        self.rest[index - 1] = digit;
                    }
                }
                const fn sub(&mut self, rhs: Mul<M>, start: usize, range: usize) -> bool {
                    let mut borrow = false;
                    let mut i = 0;
                    while i <= range {
                        let (sub, overflow) = digit::borrowing_sub(self.digit(i + start), rhs.digit(i), borrow);
                        self.set_digit(i + start, sub);
                        borrow = overflow;
                        i += 1;
                    }
                    borrow
                }
                const fn add(&mut self, rhs: BUintD8<M>, start: usize, range: usize) -> () {
                    let mut carry = false;
                    let mut i = 0;
                    while i < range {
                        let (sum, overflow) = digit::carrying_add(self.digit(i + start), rhs.digits[i], carry);
                        self.set_digit(i + start, sum);
                        carry = overflow;
                        i += 1;
                    }
                    if carry {
                        self.set_digit(range + start, self.digit(range + start).wrapping_add(1)); // we use wrapping_add here, not regular addition as a carry will always occur to the left of self.digit(range + start)
                    }
                }
            }*/
            const fn sub(mut self, rhs: Mul<M>, start: usize, range: usize) -> (Self, bool) {
                let mut borrow = false;
                let mut i = 0;
                while i <= range {
                    let (sub, overflow) =
                        digit::borrowing_sub(self.digit(i + start), rhs.digit(i), borrow);
                    if start == 0 && i == 0 {
                        self.first = sub;
                    } else {
                        self.rest[i + start - 1] = sub;
                    }
                    borrow = overflow;
                    i += 1;
                }
                (self, borrow)
            }
            const fn add(mut self, rhs: BUintD8<M>, start: usize, range: usize) -> Self {
                let mut carry = false;
                let mut i = 0;
                while i < range {
                    let (sum, overflow) =
                        digit::carrying_add(self.digit(i + start), rhs.digits[i], carry);
                    if start == 0 && i == 0 {
                        self.first = sum;
                    } else {
                        self.rest[i + start - 1] = sum;
                    }
                    carry = overflow;
                    i += 1;
                }
                if carry {
                    if start == 0 && range == 0 {
                        self.first = self.first.wrapping_add(1);
                    } else {
                        self.rest[range + start - 1] = self.rest[range + start - 1].wrapping_add(1);
                    }
                }
                self
            }
        }

        #[derive(Clone, Copy)]
        struct Mul<const M: usize> {
            last: Digit,
            rest: [Digit; M],
        }
        impl<const M: usize> Mul<M> {
            const fn new(uint: BUintD8<M>, rhs: Digit) -> Self {
                let mut rest = [0; M];
                let mut carry: Digit = 0;
                let mut i = 0;
                while i < M {
                    let (prod, c) = digit::carrying_mul(uint.digits[i], rhs, carry, 0);
                    carry = c;
                    rest[i] = prod;
                    i += 1;
                }
                Self { last: carry, rest }
            }
            const fn digit(&self, index: usize) -> Digit {
                if index == M {
                    self.last
                } else {
                    self.rest[index]
                }
            }
        }

        let v_n_m1 = v.digits[n - 1];
        let v_n_m2 = v.digits[n - 2];

        let mut u = Remainder::new(self, shift);

        let mut j = m + 1; // D2
        while j > 0 {
            j -= 1; // D7

            let u_jn = u.digit(j + n);

            #[inline]
            const fn tuple_gt(a: (Digit, Digit), b: (Digit, Digit)) -> bool {
                a.1 > b.1 || a.1 == b.1 && a.0 > b.0
            }

            // q_hat will be either `q` or `q + 1`
            let mut q_hat = if u_jn < v_n_m1 {
                let (mut q_hat, r_hat) =
                    digit::div_rem_wide(u.digit(j + n - 1), u_jn, v_n_m1); // D3

                if tuple_gt(
                    digit::widening_mul(q_hat, v_n_m2),
                    (u.digit(j + n - 2), r_hat as Digit),
                ) {
                    q_hat -= 1;

                    if let Some(r_hat) = r_hat.checked_add(v_n_m1) {
                        // this checks if `r_hat <= b`, where `b` is the digit base
                        if tuple_gt(
                            digit::widening_mul(q_hat, v_n_m2),
                            (u.digit(j + n - 2), r_hat as Digit),
                        ) {
                            q_hat -= 1;
                        }
                    }
                }
                q_hat
            } else {
                // `u[j + n - 1] >= v[n - 1]` so we know that estimate for q_hat would be larger than `Digit::MAX`. This is either equal to `q` or `q + 1` (very unlikely to be `q + 1`).
                Digit::MAX
            };
            let (u_new, overflow) = u.sub(Mul::new(v, q_hat), j, n); // D4
            u = u_new;

            if overflow {
                // D5 - unlikely, probability of this being true is ~ 2 / b where b is the digit base (i.e. `Digit::MAX + 1`)
                q_hat -= 1;
                u = u.add(v, j, n);
            }
            q.digits[j] = q_hat;
        }
        (q, u.shr(shift))
    }
}
