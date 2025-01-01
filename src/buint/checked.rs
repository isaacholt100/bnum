use crate::digit;
use crate::doc;
use crate::errors::div_zero;
use crate::helpers::tuple_to_option;
use crate::ExpType;

macro_rules! checked {
    ($BUint: ident, $BInt: ident, $Digit: ident) => {
        #[doc = doc::checked::impl_desc!()]
        impl<const N: usize> $BUint<N> {
            #[doc = doc::checked::checked_add!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_add(rhs))
            }

            #[doc = doc::checked::checked_add_signed!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_add_signed(self, rhs: $BInt<N>) -> Option<Self> {
                tuple_to_option(self.overflowing_add_signed(rhs))
            }

            #[doc = doc::checked::checked_sub!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_sub(rhs))
            }

            #[doc = doc::checked::checked_mul!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
                tuple_to_option(self.overflowing_mul(rhs))
            }

            pub(crate) const fn div_rem_digit(self, rhs: $Digit) -> (Self, $Digit) {
                let mut out = Self::ZERO;
                let mut rem: $Digit = 0;
                let mut i = N;
                while i > 0 {
                    i -= 1;
                    let (q, r) = digit::$Digit::div_rem_wide(self.digits[i], rem, rhs);
                    rem = r;
                    out.digits[i] = q;
                }
                (out, rem)
            }
            
            
            #[inline]
            pub(crate) const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
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
                            self.basecase_div_rem(rhs, ldi + 1)
                        }
                    }
                }
            }
            
            #[inline]
            pub(crate) const fn div_rem(self, rhs: Self) -> (Self, Self) {
                if rhs.is_zero() {
                    div_zero!()
                } else {
                    self.div_rem_unchecked(rhs)
                }
            }
            
            #[doc = doc::checked::checked_div!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                if rhs.is_zero() {
                    None
                } else {
                    Some(self.div_rem_unchecked(rhs).0)
                }
            }
            
            #[doc = doc::checked::checked_div_euclid!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                self.checked_div(rhs)
            }
            
            #[doc = doc::checked::checked_rem!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                if rhs.is_zero() {
                    None
                } else {
                    Some(self.div_rem_unchecked(rhs).1)
                }
            }
            
            #[doc = doc::checked::checked_rem_euclid!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                self.checked_rem(rhs)
            }
        
            #[doc = doc::checked::checked_neg!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_neg(self) -> Option<Self> {
                if self.is_zero() {
                    Some(self)
                } else {
                    None
                }
            }

            #[doc = doc::checked::checked_shl!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
                if rhs >= Self::BITS {
                    None
                } else {
                    unsafe {
                        Some(Self::unchecked_shl_internal(self, rhs))
                    }
                }
            }

            #[doc = doc::checked::checked_shr!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
                if rhs >= Self::BITS {
                    None
                } else {
                    unsafe {
                        Some(Self::unchecked_shr_internal(self, rhs))
                    }
                }
            }

            #[doc = doc::checked::checked_pow!(U)]
            #[must_use = doc::must_use_op!()]
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

            #[doc = doc::checked::checked_next_multiple_of!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
                match self.checked_rem(rhs) {
                    Some(rem) => {
                        if rem.is_zero() {
                            // `rhs` divides `self` exactly so just return `self`
                            Some(self)
                        } else {
                            // `next_multiple = floor(self / rhs) * rhs + rhs = (self - rem) + rhs`
                            self.checked_add(rhs.sub(rem))
                        }
                    },
                    None => None,
                }
            }

            #[doc = doc::checked::checked_ilog2!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_ilog2(self) -> Option<ExpType> {
                self.bits().checked_sub(1)
            }

            #[inline]
            const fn iilog(m: ExpType, b: Self, k: Self) -> (ExpType, Self) {
                // https://people.csail.mit.edu/jaffer/III/iilog.pdf
                if b.gt(&k) {
                    (m, k)
                } else {
                    let (new, q) = Self::iilog(m << 1, b.mul(b), k.div_rem_unchecked(b).0);
                    if b.gt(&q) {
                        (new, q)
                    } else {
                        (new + m, q.div(b))
                    }
                }
            }

            #[doc = doc::checked::checked_ilog10!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_ilog10(self) -> Option<ExpType> {
                if self.is_zero() {
                    return None;
                }
                if Self::TEN.gt(&self) {
                    return Some(0);
                }
                Some(Self::iilog(1, Self::TEN, self.div_rem_digit(10).0).0)
            }

            #[doc = doc::checked::checked_ilog!(U)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_ilog(self, base: Self) -> Option<ExpType> {
                use core::cmp::Ordering;
                match base.cmp(&Self::TWO) {
                    Ordering::Less => None,
                    Ordering::Equal => self.checked_ilog2(),
                    Ordering::Greater => {
                        if self.is_zero() {
                            return None;
                        }
                        if base.gt(&self) {
                            return Some(0);
                        }
                        Some(Self::iilog(1, base, self.div(base)).0)
                    }
                }
            }

            #[doc = doc::checked::checked_next_power_of_two!(U 256)]
            #[must_use = doc::must_use_op!()]
            #[inline]
            pub const fn checked_next_power_of_two(self) -> Option<Self> {
                if self.is_power_of_two() {
                    return Some(self);
                }
                let bits = self.bits();
                if bits == Self::BITS {
                    return None;
                }
                Some(Self::power_of_two(bits))
            }
        }
    };
}

#[cfg(test)]
crate::test::all_digit_tests! {
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
            (328622u32 as utest, 10000u32 as utest), // tests the unlikely condition in the division algorithm at step D5
            (2074086u32 as utest, 76819u32 as utest) // tests the unlikely condition in the division algorithm at step D5
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
        function: <utest>::checked_ilog(a: utest, b: utest),
        cases: [
            (2u8, 60u8),
            (utest::MAX, 2u8)
        ]
    }
    test_bignum! {
        function: <utest>::checked_ilog2(a: utest)
    }
    test_bignum! {
        function: <utest>::checked_ilog10(a: utest)
    }
    test_bignum! {
        function: <utest>::checked_next_power_of_two(a: utest),
        cases: [
            (utest::MAX)
        ]
    }
}

crate::macro_impl!(checked);
