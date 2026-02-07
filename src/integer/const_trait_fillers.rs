use crate::Exponent;
use crate::OverflowMode;
use crate::{Int, Integer};
use core::cmp::Ordering;

/// Provides `const` function alternatives to methods of common traits, such as `Add` and `BitOr`. These functions will be removed once `const` traits are stabilized.
impl<const S: bool, const N: usize, const B: usize, const OM: u8> Integer<S, N, B, OM> {
    #[inline]
    pub const fn bitand(self, rhs: Self) -> Self {
        self.to_digits::<u8>().bitand(rhs.to_digits()).to_integer() // u8 is the fastest
    }

    #[inline]
    pub const fn bitor(self, rhs: Self) -> Self {
        self.to_digits::<u8>().bitor(rhs.to_digits()).to_integer() // u8 is the fastest
    }

    #[inline]
    pub const fn bitxor(self, rhs: Self) -> Self {
        self.to_digits::<u8>().bitxor(rhs.to_digits()).to_integer() // u8 is the fastest
    }

    #[inline]
    pub const fn not(self) -> Self {
        let mut out = self.to_digits::<u8>().not().to_integer(); // u8 is the fastest
        out.set_sign_bits(); // as if Self::LAST_BYTE_BITS is not 8, the padding bits will have been flipped, so need to reset
        out
    }

    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        self.as_digits::<u128>().eq(other.as_digits())
    }

    #[inline]
    pub const fn ne(&self, other: &Self) -> bool {
        !Self::eq(self, other)
    }

    #[inline]
    pub const fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_negative_internal(), other.is_negative_internal()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => self.as_digits::<u32>().cmp(other.as_digits()),
        }
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
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_add(rhs),
            OverflowMode::Panicking => self.strict_add(rhs),
            OverflowMode::Saturating => self.saturating_add(rhs),
        }
    }

    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_mul(rhs),
            OverflowMode::Panicking => self.strict_mul(rhs),
            OverflowMode::Saturating => self.saturating_mul(rhs),
        }
    }

    // NOTE: don't get rid of these, they make defining the shift traits easier
    #[inline]
    pub const fn shl(self, rhs: Exponent) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_shl(rhs),
            OverflowMode::Panicking => self.strict_shl(rhs),
            OverflowMode::Saturating => self.unbounded_shl(rhs),
        }
    }

    // NOTE: don't get rid of these, they make defining the shift traits easier
    #[inline]
    pub const fn shr(self, rhs: Exponent) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_shr(rhs),
            OverflowMode::Panicking => self.strict_shr(rhs),
            OverflowMode::Saturating => self.unbounded_shr(rhs),
        }
    }

    #[inline]
    pub const fn sub(self, rhs: Self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_sub(rhs),
            OverflowMode::Panicking => self.strict_sub(rhs),
            OverflowMode::Saturating => self.saturating_sub(rhs),
        }
    }

    #[inline]
    pub const fn div(self, rhs: Self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_div(rhs),
            OverflowMode::Panicking => self.strict_div(rhs),
            OverflowMode::Saturating => self.saturating_div(rhs),
        }
    }

    #[inline]
    pub const fn rem(self, rhs: Self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_rem(rhs),
            OverflowMode::Panicking => self.strict_rem(rhs),
            OverflowMode::Saturating => self.saturating_rem(rhs),
        }
    }
}

impl<const N: usize, const B: usize, const OM: u8> Int<N, B, OM> {
    #[inline]
    pub const fn neg(self) -> Self {
        match Self::OVERFLOW_MODE {
            OverflowMode::Wrapping => self.wrapping_neg(),
            OverflowMode::Panicking => self.strict_neg(),
            OverflowMode::Saturating => self.saturating_neg(),
        }
    }
}
