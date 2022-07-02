use super::BUint;
use crate::digit::Digit;
use crate::errors::div_zero;
use crate::{ExpType, BInt};
use crate::doc;
use crate::int::checked::tuple_to_option;
use crate::digit::{self, DoubleDigit};

const fn div_rem_double(a: DoubleDigit, b: DoubleDigit) -> (DoubleDigit, DoubleDigit) {
	(a / b, a % b)
}

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

	const fn basecase_div_rem(self, mut v: Self) -> (Self, Self) {
		// Division algorithm from The Art of Computer Programming Volume 2 by Donald Knuth, Section 4.3.1, Algorithm D

		let mut q = Self::ZERO;
		let n = v.last_digit_index() + 1;
		let m = self.last_digit_index() + 1 - n;
		let shift = v.digits[n - 1].leading_zeros() as ExpType;

		v = unsafe {
			super::unchecked_shl(v, shift)
		}; // D1

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
            const fn digit(&self, index: usize) -> Digit {
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
            const fn shr(self, shift: ExpType) -> BUint<M> {
                let mut out = BUint::ZERO;
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
            const fn sub(&mut self, rhs: Mul<M>, start: usize, range: usize) -> bool {
                let mut carry = false;
                let mut i = 0;
                while i < range {
                    let (sum, overflow1) = rhs.digit(i).overflowing_add(carry as Digit);
                    let (sub, overflow2) = self.digit(i + start).overflowing_sub(sum);
                    self.set_digit(i + start, sub);
                    carry = overflow1 || overflow2;
                    i += 1;
                }
                carry
            }
            const fn add(&mut self, rhs: BUint<M>, start: usize, range: usize) {
                let mut carry = false;
                let mut i = 0;
                while i < range {
                    let (sum, overflow1) = rhs.digits[i].overflowing_add(carry as Digit);
                    let (sum, overflow2) = self.digit(i + start).overflowing_add(sum);
                    self.set_digit(i + start, sum);
                    carry = overflow1 || overflow2;
                    i += 1;
                }
				if carry {
					self.set_digit(range + start, self.digit(range + start).wrapping_add(1)); // we use wrapping_add here, not regular addition as a carry will always occur to the left of self.digit(range + start)
				}
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
            const fn digit(&self, index: usize) -> Digit {
                if index == M {
                    self.last
                } else {
                    self.rest[index]
                }
            }
        }

		let mut u = Remainder::new(self, shift);

		let mut j = m + 1; // D2
		while j > 0 {
			j -= 1; // D7

			let (mut q_hat, mut r_hat) = div_rem_double(digit::to_double_digit(u.digit(j + n - 1), u.digit(j + n)), v.digits[n - 1] as DoubleDigit); // D3
			if q_hat == Digit::MAX as DoubleDigit + 1 || q_hat * v.digits[n - 2] as DoubleDigit > digit::to_double_digit(u.digit(j + n - 2), r_hat as Digit) {
				q_hat -= 1;
				r_hat += v.digits[n - 1] as DoubleDigit;
			}
			if r_hat < Digit::MAX as DoubleDigit + 1 {
				if q_hat == Digit::MAX as DoubleDigit + 1 || q_hat * v.digits[n - 2] as DoubleDigit > digit::to_double_digit(u.digit(j + n - 2), r_hat as Digit) {
					q_hat -= 1;
				}
			}
			let mut q_hat = q_hat as Digit;
			let overflow = u.sub(Mul::new(v, q_hat), j, n + 1); // D4
			
			if overflow { // D5
				q_hat -= 1;
				u.add(v, j, n);
			}
			q.digits[j] = q_hat;
		}
		(q, u.shr(shift))
	}

	pub const fn div_rem_digit(self, rhs: Digit) -> (Self, Digit) {
		let mut out = Self::ZERO;
		let mut rem: Digit = 0;
		let mut i = N;
		while i > 0 {
			i -= 1;
			let double = digit::to_double_digit(self.digits[i], rem);
			let (q, r) = div_rem_double(double, rhs as DoubleDigit);
			rem = r as Digit;
			out.digits[i] = q as Digit;
		}
		(out, rem)
	}

    #[inline]
    pub const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
		use core::cmp::Ordering;

		if self.is_zero() {
			return (Self::ZERO, Self::ZERO);
		}

		match self.cmp(&rhs) {
			Ordering::Less => (Self::ZERO, self),
			Ordering::Equal => (Self::ONE, Self::ZERO),
			Ordering::Greater => {
				let ldi = rhs.last_digit_index();
				if ldi == 0 {
					let (div, rem) = self.div_rem_digit(rhs.digits[0]);
					(div, Self::from_digit(rem))
				} else {
					self.basecase_div_rem(rhs)
				}
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
            unsafe {
				Some(super::unchecked_shl(self, rhs))
			}
        }
    }

    #[inline]
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        if rhs >= Self::BITS {
            None
        } else {
            unsafe {
				Some(super::unchecked_shr(self, rhs))
			}
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
	const fn ilog(m: ExpType, b: Self, k: Self) -> (ExpType, Self) {
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

	#[inline]
	const fn checked_log_small(self, base: Digit) -> Option<ExpType> {
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
    pub const fn checked_log10(self) -> Option<ExpType> {
		if self.is_zero() {
			return None;
		}
        self.checked_log_small(10)
    }

    #[inline]
    pub const fn log_big(self, base: Self) -> ExpType {
		// Function adapted from Rust's core library: https://doc.rust-lang.org/core/ used under the MIT license.
		// The original license file and copyright notice for this project can be found in this project's root at licenses/LICENSE-rust.
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
	pub const fn checked_log(self, base: Self) -> Option<ExpType> {
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