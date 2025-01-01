use crate::doc;
use crate::BUintD8;
use super::Float;
use core::cmp::Ordering;

#[doc = doc::const_trait_fillers::impl_desc!()]
impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        handle_nan!(false; self, other);
        (self.is_zero() && other.is_zero()) || BUintD8::eq(&self.to_bits(), &other.to_bits())
    }

    #[inline]
    pub const fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
    }

    #[inline]
    pub const fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        handle_nan!(None; self, other);
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }
        Some(self.total_cmp(other))
    }

    #[inline]
    pub const fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(&other), Some(Ordering::Less))
    }

    #[inline]
    pub const fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(&other), Some(Ordering::Less | Ordering::Equal))
    }

    #[inline]
    pub const fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(&other), Some(Ordering::Greater))
    }

    #[inline]
    pub const fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(&other), Some(Ordering::Greater | Ordering::Equal))
    }

    #[inline]
    pub(crate) const fn neg(mut self) -> Self {
        type Digit = u8;

        self.bits.digits[W - 1] ^= 1 << (Digit::BITS - 1);
        self
    }
}