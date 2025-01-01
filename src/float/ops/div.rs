use core::num::FpCategory;
use crate::float::FloatExponent;
use crate::float::UnsignedFloatExponent;
use crate::ExpType;
use crate::BUintD8;
use super::Float;
use crate::cast::As;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(crate) fn div_internal(self, rhs: Self, negative: bool) -> Self
    where
        [(); W * 2]:,
    {
        let (a, b) = (self, rhs);
        let (_, e1, s1) = a.into_biased_parts();
        let (_, e2, s2) = b.into_biased_parts();

        let b1 = s1.bits();
        let b2 = s2.bits();

        let mut e =
            (e1 as FloatExponent) - (e2 as FloatExponent) + Self::EXP_BIAS + (b1 as FloatExponent)
                - (b2 as FloatExponent)
                - 1;

        let mut extra_shift = 0;
        if !e.is_positive() {
            extra_shift = (1 - e) as UnsignedFloatExponent;
            e = 1;
        }

        let total_shift = (MB as FloatExponent + 1 + b2 as FloatExponent - b1 as FloatExponent) - (extra_shift as FloatExponent);

        let large = if !total_shift.is_negative() {
            (s1.as_::<BUintD8<{ W * 2 }>>()) << total_shift
        } else {
            (s1.as_::<BUintD8<{ W * 2 }>>()) >> (-total_shift)
        };
        let mut division = (large / (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();

        let rem = if division.bits() != Self::MB + 2 {
            let rem = (large % (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();
            rem
        } else {
            e += 1;
            division =
                ((large >> 1 as ExpType) / (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();
            let rem =
                ((large >> 1 as ExpType) % (s2.as_::<BUintD8<{ W * 2 }>>())).as_::<BUintD8<W>>();
            rem
        };
        if rem * BUintD8::TWO > s2 {
            division += BUintD8::ONE;
        } else if rem * BUintD8::TWO == s2 {
            if (division & BUintD8::ONE) == BUintD8::ONE {
                division += BUintD8::ONE;
            }
        }
        if division.bits() == Self::MB + 2 {
            e += 1;
            division >>= 1 as ExpType;
        }

        if e > Self::MAX_EXP + Self::EXP_BIAS - 1 {
            return Self::INFINITY;
        }

        if e == 1 && division.bits() < Self::MB + 1 {
            return Self::from_raw_parts(negative, 0, division);
        }

        if division >> Self::MB != BUintD8::ZERO {
            division ^= BUintD8::ONE << Self::MB;
        }
        Self::from_raw_parts(negative, e as UnsignedFloatExponent, division)
    }

    #[inline]
    pub(super) fn div(self, rhs: Self) -> Self
    where
        [(); W * 2]:,
    {
        let negative = self.is_sign_negative() ^ rhs.is_sign_negative();
        match (self.classify(), rhs.classify()) {
            (FpCategory::Nan, _) | (_, FpCategory::Nan) => Self::NAN,
            (FpCategory::Infinite, FpCategory::Infinite) => Self::NAN,
            (FpCategory::Zero, FpCategory::Zero) => Self::NAN,
            (FpCategory::Infinite, _) | (_, FpCategory::Zero) => {
                if negative {
                    Self::NEG_INFINITY
                } else {
                    Self::INFINITY
                }
            }
            (FpCategory::Zero, _) | (_, FpCategory::Infinite) => {
                if negative {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                }
            }
            (_, _) => self.div_internal(rhs, negative),
        }
    }
}

/*/// Returns tuple of division and whether u is less than v
pub const fn div_float<const N: usize>(u: BUintD8<N>, v: BUintD8<N>) -> (BUintD8<N>, bool) {
    let gt = if let core::cmp::Ordering::Less = u.cmp(&v) {
        0
    } else {
        1
    };
    // `self` is padded with N trailing zeros (less significant digits).
    // `v` is padded with N leading zeros (more significant digits).
    let shift = v.digits[N - 1].leading_zeros();
    // `shift` is between 0 and 64 inclusive.
    let v = super::unchecked_shl(v, shift);
    // `v` is still padded with N leading zeros.

    struct Remainder<const M: usize> {
        first: Digit,
        second: Digit,
        rest: [Digit; M],
    }
    impl<const M: usize> Remainder<M> {
        const fn new(uint: BUintD8<M>, shift: ExpType) -> Self {
            // This shift can be anything from 0 to 64 inclusive.
            // Scenarios:
            // * shift by 0 -> nothing happens, still N trailing zeros.
            // * shift by 64 -> all digits shift by one to the right, there are now (N - 1) trailing zeros and 1 leading zero.
            // * shift by amount between 0 and 64 -> there may be 0 or 1 leading zeros and (N - 1) or N trailing zeros.
            // So indexing between 2N - 1 and N - 1 will get any non-zero digits.
            // Instead of a logical right shift, we will perform a rotate right on the uint - this is the same except the part of the number which may have been removed from the right shift is instead brought to the most significant digit of the number.
            // Then do fancy bit shifts and logic to separate the first and last non zero digits.
            let shift = Digit::BITS - shift;
            let mut rest = uint.rotate_right(shift);
            let last_digit = rest.digits[M - 1];
            let last = (last_digit << shift) >> shift;
            let second = last_digit ^ last;
            rest.digits[M - 1] = last;
            Self {
                first: 0,
                second,
                rest: rest.digits,
            }
        }
        const fn index(&self, index: usize) -> Digit {
            if index == M - 1 {
                self.first
            } else if index == M {
                self.second
            } else if index > M {
                self.rest[index - M - 1]
            } else {
                // There are M - 1 trailing zeros so we can return zero here.
                0
            }
        }
        const fn set_digit(&mut self, index: usize, digit: Digit) {
            if index == M - 1 {
                self.first = digit;
            } else if index == M {
                self.second = digit;
            } else if index > M {
                self.rest[index - M - 1] = digit;
            }
        }
        /*const fn to_uint(self, shift: ExpType) -> BUintD8<M> {
            let mut out = BUintD8::ZERO;
            let mut i = 0;
            while i < M {
                out.digits[i] = self.index(i) >> shift;
                i += 1;
            }
            if shift > 0 {
                let mut i = 0;
                while i < M {
                    out.digits[i] |= self.rest[i] << (Digit::BITS - shift);
                    i += 1;
                }
            }
            out
        }*/
        const fn sub(&mut self, start: usize, rhs: Mul<M>, end: usize) -> bool {
            let mut carry = false;
            let mut i = 0;
            while i < end {
                let (sum, overflow1) = rhs.index(i).overflowing_add(carry as Digit);
                let (sub, overflow2) = self.index(i + start).overflowing_sub(sum);
                self.set_digit(i + start, sub);
                carry = overflow1 || overflow2;
                i += 1;
            }
            carry
        }
        const fn add(&mut self, start: usize, rhs: [Digit; M], end: usize) -> bool {
            let mut carry = false;
            let mut i = 0;
            while i < end {
                let (sum, overflow1) = rhs[i].overflowing_add(carry as Digit);
                let (sum, overflow2) = self.index(i + start).overflowing_sub(sum);
                self.set_digit(i + start, sum);
                carry = overflow1 || overflow2;
                i += 1;
            }
            carry
        }
    }

    // The whole implementation of `Mul` doesn't need to change as it is already padded with N leading zeros.
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
                let (prod, c) = crate::arithmetic::mul_carry_unsigned(carry, 0, uint.digits[i], rhs);
                carry = c;
                rest[i] = prod;
                i += 1;
            }
            Self {
                last: carry,
                rest,
            }
        }
        const fn index(&self, index: usize) -> Digit {
            if index == M {
                self.last
            } else {
                self.rest[index]
            }
        }
    }

    let mut u = Remainder::new(u, shift);
    let mut q = BUintD8::ZERO;
    let v_n_1 = v.digits[N - 1];
    let v_n_2 = v.digits[N - 2];
    let gt_half = v_n_1 > digit::HALF;

    let mut j = N + gt;
    while j > gt {
        j -= 1;
        let u_jn = u.index(j + N);
        let mut q_hat = if u_jn < v_n_1 {
            let (mut q_hat, mut r_hat) = if gt_half {
                BUintD8::<N>::div_wide(u_jn, u.index(j + N - 1), v_n_1)
            } else {
                BUintD8::<N>::div_half(u_jn, u.index(j + N - 1), v_n_1)
            };
            loop {
                let a = ((r_hat as DoubleDigit) << digit::BITS) | u.index(j + N - 2) as DoubleDigit;
                let b = q_hat as DoubleDigit * v_n_2 as DoubleDigit;
                if b <= a {
                    break;
                }
                /*let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);
                if hi < r_hat {
                    break;
                } else if hi == r_hat && lo <= u.index(j + n - 2) {
                    break;
                }*/
                q_hat -= 1;
                let (new_r_hat, overflow) = r_hat.overflowing_add(v_n_1);
                r_hat = new_r_hat;
                if overflow {
                    break;
                }
            }
            q_hat
        } else {
            Digit::MAX
        };

        let q_hat_v = Mul::new(v, q_hat);
        let carry = u.sub(j, q_hat_v, N + 1);
        if carry {
            q_hat -= 1;
            let carry = u.add(j, v.digits, N);
            u.set_digit(j + N, u.index(j + N).wrapping_add(carry as Digit));
        }
        // if self is less than other, q_hat is 0
        q.digits[j - gt] = q_hat;
    }

    (q, gt == 0)
    //super::unchecked_shl(self.as_buint::<{N * 2}>(), N as u16 * 64).div_rem(v.as_buint::<{N * 2}>()).0
}*/