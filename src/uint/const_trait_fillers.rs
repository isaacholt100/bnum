use super::Integer;
use crate::ExpType;
use core::cmp::Ordering;

/// Provides `const` function alternatives to methods of common traits, such as `Add` and `BitOr`. These functions will be removed once `const` traits are stabilized.
impl<const S: bool, const N: usize> Integer<S, N> {
    #[inline]
    pub const fn bitand(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                let d = self.as_wide_digits().get(i) & rhs.as_wide_digits().get(i);
                out.as_wide_digits_mut().set(i, d);

                i += 1;
            }
        }
        out
    }

    #[inline]
    pub const fn bitor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                let d = self.as_wide_digits().get(i) | rhs.as_wide_digits().get(i);
                out.as_wide_digits_mut().set(i, d);

                i += 1;
            }
        }
        out
    }

    #[inline]
    pub const fn bitxor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                let d = self.as_wide_digits().get(i) ^ rhs.as_wide_digits().get(i);
                out.as_wide_digits_mut().set(i, d);

                i += 1;
            }
        }
        out
    }

    #[inline]
    pub const fn not(self) -> Self {
        let mut out = Self::ZERO;
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                let d = self.as_wide_digits().get(i);
                out.as_wide_digits_mut().set(i, !d);

                i += 1;
            }
        }
        out
    }

    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        unsafe {
            while i < Self::U128_DIGITS {
                if self.as_wide_digits().get(i) != other.as_wide_digits().get(i) {
                    return false;
                }
                i += 1;
            }
        }
        true
    }

    #[inline]
    pub const fn ne(&self, other: &Self) -> bool {
        !Self::eq(self, other)
    }

    #[inline]
    pub const fn cmp(&self, other: &Self) -> Ordering {
        if S {
            let s1 = self.force_sign::<true>().signed_digit();
            let s2 = other.force_sign::<true>().signed_digit();
            if s1 > s2 {
                return Ordering::Greater;
            } else if s1 < s2 {
                return Ordering::Less;
            }
        }
        let mut i = Self::U128_DIGITS;
        unsafe {
            while i > 0 {
                i -= 1;
                let a = self.as_wide_digits().get(i);
                let b = other.as_wide_digits().get(i);

                // Clippy: don't use match here as `cmp` is not yet const for primitive integers
                #[allow(clippy::comparison_chain)]
                if a > b {
                    return Ordering::Greater;
                } else if a < b {
                    return Ordering::Less;
                }
            }
        }
        Ordering::Equal
    }

    #[inline]
    pub const fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            _ => self,
        }
    }

    #[inline]
    pub const fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            _ => other,
        }
    }

    #[inline]
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.le(&max));
        if let Ordering::Less = self.cmp(&min) {
            min
        } else if let Ordering::Greater = self.cmp(&max) {
            max
        } else {
            self
        }
    }

    #[inline]
    pub const fn lt(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Less => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn le(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn gt(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Greater => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn ge(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Ordering::Greater | Ordering::Equal => true,
            _ => false,
        }
    }

    #[inline]
    pub const fn add(self, rhs: Self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_add(rhs)
        } else {
            self.wrapping_add(rhs)
        }
    }

    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_mul(rhs)
        } else {
            self.wrapping_mul(rhs)
        }
    }

    // NOTE: need to keep this due to the way the macros in ops.rs are defined
    #[inline]
    pub const fn shl(self, rhs: ExpType) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_shl(rhs)
        } else {
            self.wrapping_shl(rhs)
        }
    }

    // NOTE: need to keep this due to the way the macros in ops.rs are defined
    #[inline]
    pub const fn shr(self, rhs: ExpType) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_shr(rhs)
        } else {
            self.wrapping_shr(rhs)
        }
    }

    #[inline]
    pub const fn sub(self, rhs: Self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_sub(rhs)
        } else {
            self.wrapping_sub(rhs)
        }
    }

    #[inline]
    pub const fn div(self, rhs: Self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_div(rhs)
        } else {
            self.wrapping_div(rhs)
        }
    }

    #[inline]
    pub const fn rem(self, rhs: Self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_rem(rhs)
        } else {
            self.wrapping_rem(rhs)
        }
    }
}

impl<const N: usize> crate::Int<N> {
    #[inline]
    pub const fn neg(self) -> Self {
        if crate::OVERFLOW_CHECKS {
            self.strict_neg()
        } else {
            self.wrapping_neg()
        }
    }
}
