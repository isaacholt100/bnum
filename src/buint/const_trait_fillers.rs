use super::Uint;
use crate::ExpType;
use crate::doc;
use core::cmp::Ordering;

#[doc = doc::const_trait_fillers::impl_desc!()]
impl<const N: usize> Uint<N> {
    #[inline]
    pub const fn bitand(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut u128_digits = out.as_u128_digits_mut();
        let mut i = 0;
        unsafe {
            while i < Self::FULL_U128_DIGITS {
                let d = self.as_u128_digits().get(i) & rhs.as_u128_digits().get(i);
                u128_digits.set(i, d);

                i += 1;
            }
        }
        let d = self.as_u128_digits().last() & rhs.as_u128_digits().last();
        u128_digits.set_last(d);
        out
    }

    #[inline]
    pub const fn bitor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut u128_digits = out.as_u128_digits_mut();
        let mut i = 0;
        unsafe {
            while i < Self::FULL_U128_DIGITS {
                let d = self.as_u128_digits().get(i) | rhs.as_u128_digits().get(i);
                u128_digits.set(i, d);

                i += 1;
            }
        }
        let d = self.as_u128_digits().last() | rhs.as_u128_digits().last();
        u128_digits.set_last(d);
        out
    }

    #[inline]
    pub const fn bitxor(self, rhs: Self) -> Self {
        let mut out = Self::ZERO;
        let mut u128_digits = out.as_u128_digits_mut();
        let mut i = 0;
        unsafe {
            while i < Self::FULL_U128_DIGITS {
                let d = self.as_u128_digits().get(i) ^ rhs.as_u128_digits().get(i);
                u128_digits.set(i, d);

                i += 1;
            }
        }
        let d = self.as_u128_digits().last() ^ rhs.as_u128_digits().last();
        u128_digits.set_last(d);
        out
    }

    #[inline]
    pub const fn not(self) -> Self {
        let mut out = Self::ZERO;
        let mut u128_digits = out.as_u128_digits_mut();
        let mut i = 0;
        unsafe {
            while i < Self::FULL_U128_DIGITS {
                let d = !self.as_u128_digits().get(i);
                u128_digits.set(i, d);

                i += 1;
            }
        }
        let d = !self.as_u128_digits().last();
        u128_digits.set_last(d);
        out
    }

    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        unsafe {
            while i < Self::FULL_U128_DIGITS {
                if self.as_u128_digits().get(i) != other.as_u128_digits().get(i) {
                    return false;
                }
                i += 1;
            }
        }
        self.as_u128_digits().last() == other.as_u128_digits().last()
    }

    #[inline]
    pub const fn ne(&self, other: &Self) -> bool {
        !Self::eq(self, other)
    }

    #[inline]
    pub const fn cmp(&self, other: &Self) -> Ordering {
        let a = self.as_u128_digits().last();
        let b = other.as_u128_digits().last();
        if a > b {
            return Ordering::Greater;
        } else if a < b {
            return Ordering::Less;
        }
        let mut i = Self::U128_DIGITS - 1;
        unsafe {
            while i > 0 {
                i -= 1;
                let a = self.as_u128_digits().get(i);
                let b = other.as_u128_digits().get(i);

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
