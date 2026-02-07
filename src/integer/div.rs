use crate::{Int, Integer, Uint};

impl<const N: usize> Uint<N, 0> {
    #[inline]
    pub(crate) const fn div_rem_unchecked_unsigned(self, rhs: Self) -> (Self, Self) {
        use core::cmp::Ordering;

        match self.cmp(&rhs) {
            Ordering::Less => (Self::ZERO, self),
            Ordering::Equal => (Self::ONE, Self::ZERO),
            Ordering::Greater => {
                let (q, r) = self.to_digits::<u64>().div_rem_unchecked(rhs.to_digits());
                (q.to_integer(), r.to_integer())
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
        let (q, r) = self.to_digits::<u64>().div_rem_digit(rhs);
        (q.to_integer(), r)
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    // don't check that rhs is zero or (if signed) that self is Self::MIN and RHS is -1
    #[inline]
    pub(crate) const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
        if S {
            let (d, r) = self.force().div_rem_unchecked_signed(rhs.force());
            (d.force(), r.force())
        } else {
            let (d, r) = self.force().div_rem_unchecked_unsigned(rhs.force());
            (d.force(), r.force())
        }
    }

    // don't check that rhs is zero or (if signed) that self is Self::MIN and RHS is -1
    #[inline]
    pub(crate) const fn div_rem_euclid_unchecked(self, rhs: Self) -> (Self, Self) {
        if S {
            let (d, r) = self.force().div_rem_euclid_unchecked_signed(rhs.force());
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
