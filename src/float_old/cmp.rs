use super::Float;
use crate::{Bint, BUint};
use core::cmp::{PartialOrd, PartialEq, Ordering};

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const fn max(self, other: Self) -> Self {
        handle_nan!(other; self);
        handle_nan!(self; other);
        match self.total_cmp(&other) {
            Ordering::Less => other,
            _ => self,
        }
    }
    pub const fn min(self, other: Self) -> Self {
        handle_nan!(other; self);
        handle_nan!(self; other);
        match self.total_cmp(&other) {
            Ordering::Greater => other,
            _ => self,
        }
    }
    pub const fn maximum(self, other: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(other; other);
        match self.total_cmp(&other) {
            Ordering::Less => other,
            _ => self,
        }
    }
    pub const fn minimum(self, other: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(other; other);
        match self.total_cmp(&other) {
            Ordering::Greater => other,
            _ => self,
        }
    }
    pub const fn clamp(self, min: Self, max: Self) -> Self {
        match Self::partial_cmp(&min, &max) {
            None | Some(Ordering::Greater) => panic!("assertion failed: min <= max"),
            _ => {
                handle_nan!(self; self);
                let is_zero = self.is_zero();
                if is_zero && min.is_zero() {
                    return self;
                }
                if let Ordering::Less = Self::total_cmp(&self, &min) {
                    return min;
                }
                if is_zero && max.is_zero() {
                    return self;
                }
                if let Ordering::Greater = Self::total_cmp(&self, &max) {
                    return max;
                }
                self
            }
        }
    }
    pub const fn total_cmp(&self, other: &Self) -> Ordering {
        let left = self.to_int();
        let right = other.to_int();
        if left.is_negative() && right.is_negative() {
            Bint::cmp(&left, &right).reverse()
        } else {
            Bint::cmp(&left, &right)
        }
    }
}

impl<const W: usize, const MB: usize> const PartialEq for Float<W, MB> {
    fn eq(&self, other: &Self) -> bool {
        handle_nan!(false; self, other);
        (self.is_zero() && other.is_zero()) || BUint::eq(&self.to_bits(), &other.to_bits())
    }
}
impl<const W: usize, const MB: usize> const PartialOrd for Float<W, MB> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        handle_nan!(None; self, other);
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }
        Some(self.total_cmp(other))
    }
}

#[cfg(test)]
mod tests {
    test_float! {
        function: max(a: f64, b: f64)
    }
    test_float! {
        function: min(a: f64, b: f64)
    }
    test_float! {
        function: clamp(a: f64, b: f64, c: f64)
    }
    // TODO: test total_cmp, partial_cmp, eq
}