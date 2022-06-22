use super::BUint;
use crate::digit::Digit;
use crate::errors::div_zero;
use crate::{ExpType, BInt};
use crate::doc;
use crate::int::checked::tuple_to_option;
use crate::digit::{self, DoubleDigit};

#[doc=doc::checked::impl_desc!()]
impl<const N: usize> BUint<N> {
    #[inline]
    #[doc=doc::checked_add!(U256)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }

    #[inline]
    pub const fn checked_add_signed(self, rhs: BInt<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_signed(rhs))
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }
    
    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }
    
    #[inline]
    const fn div_wide(high: Digit, low: Digit, rhs: Digit) -> (Digit, Digit) {
		// credit uint source code
        let lhs = digit::to_double_digit(high, low);
        let rhs = rhs as DoubleDigit;

        ((lhs / rhs) as Digit, (lhs % rhs) as Digit)
    }
    
    #[inline]
    const fn div_half(rem: Digit, digit: Digit, rhs: Digit) -> (Digit, Digit) {
		// credit uint source code
        const fn div_rem(a: Digit, b: Digit) -> (Digit, Digit) {
            (a / b, a % b)
        }
        let (hi, rem) = div_rem((rem << digit::HALF_BITS) | (digit >> digit::HALF_BITS), rhs);
        let (lo, rem) = div_rem((rem << digit::HALF_BITS) | (digit & digit::HALF), rhs);

        ((hi << digit::HALF_BITS) | lo, rem)
    }

    #[inline]
    const fn div_rem_small(self, rhs: Digit) -> (Self, Self) {
		// credit uint source code
        let (div, rem) = self.div_rem_digit(rhs);
        (div, Self::from_digit(rem))
    }

    #[inline]
    pub const fn div_rem_digit(self, rhs: Digit) -> (Self, Digit) {
		// credit uint source code
        let mut rem: Digit = 0;
        let mut out = Self::ZERO;
        if rhs > digit::HALF {
            let mut i = N;
            while i > 0 {
                i -= 1;
                let (q, r) = Self::div_wide(rem, self.digits[i], rhs);
                out.digits[i] = q;
                rem = r;
            }
        } else {
            let mut i = N;
            while i > 0 {
                i -= 1;
                let (q, r) = Self::div_half(rem, self.digits[i], rhs);
                out.digits[i] = q;
                rem = r;
            }
        }
        (out, rem)
    }
    
    const fn div_rem_core(self, v: Self, n: usize, m: usize) -> (Self, Self) {
		// credit uint source code
        let shift = v.digits[n - 1].leading_zeros() as ExpType;
        let v = super::unchecked_shl(v, shift);

        //debug_assert!(v.bit(N as ExpType * digit::BITS - 1));
        debug_assert!(n + m <= N);

        struct Remainder<const M: usize> {
            first: Digit,
            rest: [Digit; M],
        }
        impl<const M: usize> Remainder<M> {
            const fn new(uint: BUint<M>, shift: ExpType) -> Self {
                let first = uint.digits[0] << shift;
                let rest = uint.wrapping_shr(digit::BITS - shift);
                Self {
                    first,
                    rest: rest.digits,
                }
            }
            const fn index(&self, index: usize) -> Digit {
                if index == 0 {
                    self.first
                } else {
                    self.rest[index - 1]
                }
            }
            const fn set_digit(&mut self, index: usize, digit: Digit) {
                if index == 0 {
                    self.first = digit;
                } else {
                    self.rest[index - 1] = digit;
                }
            }
            const fn into_uint(self, shift: ExpType) -> BUint<M> {
                let mut out = BUint::ZERO;
                let mut i = 0;
                while i < M {
                    out.digits[i] = self.index(i) >> shift;
                    i += 1;
                }
                if shift > 0 {
                    let mut i = 0;
                    while i < M {
                        out.digits[i] |= self.rest[i] << (digit::BITS as ExpType - shift);
                        i += 1;
                    }
                }
                out
            }
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
                    let (sum, overflow2) = self.index(i + start).overflowing_add(sum);
                    self.set_digit(i + start, sum);
                    carry = overflow1 || overflow2;
                    i += 1;
                }
                carry
            }
        }

        #[derive(Clone, Copy)]
        struct Mul<const M: usize> {
            last: Digit,
            rest: [Digit; M],
        }
        impl<const M: usize> Mul<M> {
            const fn new(uint: BUint<M>, rhs: Digit) -> Self {
                let mut rest = [0; M];
                let mut carry: Digit = 0;
                let mut i = 0;
                while i < M {
                    let (prod, c) = digit::carrying_mul(uint.digits[i], rhs, carry);
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
        
        let mut u = Remainder::new(self, shift);
        let mut q = Self::ZERO;
        let v_n_1 = v.digits[n - 1];
        let v_n_2 = v.digits[n - 2];
        let gt_half = v_n_1 > digit::HALF;

        let mut j = m + 1;
        while j > 0 {
            j -= 1;
            let u_jn = u.index(j + n);
            let mut q_hat = if u_jn < v_n_1 {
                let (mut q_hat, mut r_hat) = if gt_half {
                    Self::div_wide(u_jn, u.index(j + n - 1), v_n_1)
                } else {
                    Self::div_half(u_jn, u.index(j + n - 1), v_n_1)
                };
                loop {
                    //let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);
                    let a = ((r_hat as DoubleDigit) << digit::BITS) | u.index(j + n - 2) as DoubleDigit;
                    let b = q_hat as DoubleDigit * v_n_2 as DoubleDigit;
                    if b <= a {
                        break;
                    }
                    /*let (hi, lo) = digit::from_double_digit(q_hat as DoubleDigit * v_n_2 as DoubleDigit);*/
                    /*if hi < r_hat {
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
            let carry = u.sub(j, q_hat_v, n + 1);
            if carry {
                q_hat -= 1;
                let carry = u.add(j, v.digits, n);
                u.set_digit(j + n, u.index(j + n).wrapping_add(carry as Digit));
            }

            q.digits[j] = q_hat;
        }

        let remainder = u.into_uint(shift);
        (q, remainder)
    }

    #[inline]
    pub const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
		// credit uint source code
        if self.is_zero() {
            return (Self::ZERO, Self::ZERO);
        }

        use core::cmp::Ordering;

        match self.cmp(&rhs) {
            Ordering::Less => (Self::ZERO, self),
            Ordering::Equal => (Self::ONE, Self::ZERO),
            Ordering::Greater => {
                let self_last_digit_index = self.last_digit_index();
                let rhs_last_digit_index = rhs.last_digit_index();
                if rhs_last_digit_index == 0 {
                    let first_digit = rhs.digits[0];
                    if first_digit == 1 {
                        return (self, Self::ZERO);
                    }
                    return self.div_rem_small(first_digit);
                }
                self.div_rem_core(rhs, rhs_last_digit_index + 1, self_last_digit_index - rhs_last_digit_index)
            }
        }
    }

    #[inline]
    pub const fn div_rem(self, rhs: Self) -> (Self, Self) {
        if rhs.is_zero() {
            div_zero!()
        } else {
            self.div_rem_unchecked(rhs)
        }
    }

    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).0)
        }
    }

    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_div(rhs)
    }

    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.div_rem_unchecked(rhs).1)
        }
    }

    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        self.checked_rem(rhs)
    }

    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        if self.is_zero() {
            Some(self)
        } else {
            None
        }
    }

    #[inline]
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(super::unchecked_shl(self, rhs))
        }
    }

    #[inline]
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            Some(super::unchecked_shr(self, rhs))
        }
    }

	#[inline]
	pub const fn checked_pow(mut self, mut pow: ExpType) -> Option<Self> {
		// https://en.wikipedia.org/wiki/Exponentiation_by_squaring#Basic_method
		if pow == 0 {
			return Some(Self::ONE);
		}
		let mut y = Self::ONE;
		while pow > 1 {
			if pow & 1 == 1 {
				y = match self.checked_mul(y) {
					Some(m) => m,
					None => return None,
				};
			}
			self = match self.checked_mul(self) {
				Some(m) => m,
				None => return None,
			};
			pow >>= 1;
		}
		self.checked_mul(y)
	}

	#[inline]
	fn ilog(m: ExpType, b: Self, k: Self) -> (ExpType, Self) {
		// https://people.csail.mit.edu/jaffer/III/ilog.pdf
		if b > k {
			(m, k)
		} else {
			let (new, q) = Self::ilog(m << 1, b * b, k.div_rem_unchecked(b).0);
			if b > q {
				(new, q)
			} else {
				(new + m, q / b)
			}
		}
	}

	/*#[inline]
	fn ilog2(mut n: ExpType, mut m: ExpType, mut b: Self, mut k: Self) -> ExpType {
		let mut q;
		loop {
			q = k;
			k = k / b;
			if b > k {
				break;
			}
			m <<= 1;
			b = b.wrapping_mul(b);
		}
		let mut new = m;
		while m > 1 {
			m >>= 1;
			b = b.sqrt();
			if b <= q {
				q = q / b;
				new += m;
			}
		}
		new
	}*/

	#[inline]
	fn checked_log_small(self, base: Digit) -> Option<ExpType> {
		// https://people.csail.mit.edu/jaffer/III/ilog.pdf
		if base == 2 {
			return self.checked_log2();
		}
		if base < 2 {
            None
        } else {
			let base = Self::from_digit(base);
			Some(if base > self {
				0
			} else {
				Self::ilog(1, base, self / base).0
			})
		}
	}

    #[inline]
    pub const fn checked_log2(self) -> Option<ExpType> {
        self.bits().checked_sub(1)
    }

    #[inline]
    pub fn checked_log10(self) -> Option<ExpType> {
		if self.is_zero() {
			return None;
		}
        self.checked_log_small(10)
    }

    #[inline]
    pub const fn log_big(self, base: Self) -> ExpType {
		// Function adapted from Rust's core library: https://doc.rust-lang.org/core/ used under the MIT license.
		// The original license file for this project can be found in this project's root at licenses/LICENSE-rust.
		let (mut n, mut r) = if Self::BITS >= 128 {
			let b = (self.bits() - 1) / base.bits();
			let r = self.div_rem_unchecked(base.pow(b)).0;
			(b, r)
		} else {
			(0, self)
		};
		while r >= base {
			r = r / base;
			n += 1;
		}
		n
	}

	const LOG_THRESHOLD: Self = Self::from_digit(60);

	#[inline]
	pub fn checked_log(self, base: Self) -> Option<ExpType> {
		if self.is_zero() {
			return None;
		}
		if base < Self::LOG_THRESHOLD {
			self.checked_log_small(base.digits[0])
		} else {
			Some(self.log_big(base))
		}
	}

	#[inline]
	pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
		match self.checked_rem(rhs) {
			Some(rem) => {
				if rem.is_zero() {
					// `rhs` divides `self` exactly so just return `self`
					Some(self)
				} else {
					// `next_multiple = (self / rhs) * rhs + rhs = (self - rem) + rhs`
					self.checked_add(rhs - rem)
				}
			},
			None => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test::{test_bignum, types::*};

    test_bignum! {
		function: <utest>::checked_add(a: utest, b: utest),
        cases: [
            (utest::MAX, 1u8)
        ]
    }
    test_bignum! {
        function: <utest>::checked_add_signed(a: utest, b: itest)
    }
    test_bignum! {
		function: <utest>::checked_sub(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::checked_mul(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::checked_div(a: utest, b: utest),
        cases: [
			(328622u32 as utest, 10000u32 as utest), // tests the unlikely condition
			(2074086u32 as utest, 76819u32 as utest) // tests the unlikely condition
        ]
    }
    test_bignum! {
		function: <utest>::checked_div_euclid(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::checked_rem(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::checked_rem_euclid(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::checked_neg(a: utest)
    }
    test_bignum! {
		function: <utest>::checked_shl(a: utest, b: u16)
    }
    test_bignum! {
		function: <utest>::checked_shr(a: utest, b: u16)
    }
    test_bignum! {
		function: <utest>::checked_pow(a: utest, b: u16)
    }
    test_bignum! {
		function: <utest>::checked_log(a: utest, b: utest)
    }
    test_bignum! {
		function: <utest>::checked_log2(a: utest)
    }
    test_bignum! {
		function: <utest>::checked_log10(a: utest)
    }
}