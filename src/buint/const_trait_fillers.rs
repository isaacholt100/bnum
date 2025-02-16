use super::BUintD8;
use crate::doc;
use crate::ExpType;
use crate::{digit, Digit};
use core::cmp::Ordering;

#[doc = doc::const_trait_fillers::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    #[inline]
    pub const fn bitand(self, rhs: Self) -> Self {
        // TODO: can use u128
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] & rhs.digits[i];
            i += 1;
        }
        return out;
        while i < N {
            let u128_a = super::u128_from_digits(&self.digits, i);
            let u128_b = super::u128_from_digits(&rhs.digits, i);
            let u128_out = u128_a & u128_b;
            let out_bytes = u128_out.to_le_bytes();
            let mut j = 0;
            while j < 16 {
                out.digits[i + j] = out_bytes[j];
                j += 1;
            }
            i += 16;
        }
        out
    }

    #[inline]
    pub const fn bitor(self, rhs: Self) -> Self {
        // TODO: can use u128
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] | rhs.digits[i];
            i += 1;
        }
        out
    }

    #[inline]
    pub const fn bitxor(self, rhs: Self) -> Self {
        // TODO: can use u128
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = self.digits[i] ^ rhs.digits[i];
            i += 1;
        }
        out
    }

    #[inline]
    pub const fn not(self) -> Self {
        // TODO: can use u128
        let mut out = Self::ZERO;
        let mut i = 0;
        while i < N {
            out.digits[i] = !self.digits[i];
            i += 1;
        }
        out
    }

    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        // TODO: can use u128
        let mut i = 0;
        while i < N {
            if self.digits[i] != other.digits[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    #[inline]
    pub const fn ne(&self, other: &Self) -> bool {
        !Self::eq(self, other)
    }

    #[inline]
    pub const fn cmp(&self, other: &Self) -> Ordering {
        // TODO: can use u128
        let mut i = N;
        while i > 0 {
            i -= 1;
            let a = self.digits[i];
            let b = other.digits[i];

            // Clippy: don't use match here as `cmp` is not yet const for primitive integers
            #[allow(clippy::comparison_chain)]
            if a > b {
                return Ordering::Greater;
            } else if a < b {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    crate::int::cmp::impls!();

    crate::int::ops::trait_fillers!();

    #[inline]
    pub const fn div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    #[inline]
    pub const fn rem(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }
}
