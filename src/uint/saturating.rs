use super::{BUint, ExpType};
use crate::Bint;

#[inline]
const fn saturate_up<const N: usize>((int, overflow): (BUint<N>, bool)) -> BUint<N> {
    if overflow {
        BUint::<N>::MAX
    } else {
        int
    }
}

#[inline]
const fn saturate_down<const N: usize>((int, overflow): (BUint<N>, bool)) -> BUint<N> {
    if overflow {
        BUint::<N>::MIN
    } else {
        int
    }
}

impl<const N: usize> BUint<N> {
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        saturate_up(self.overflowing_add(rhs))
    }

    #[inline]
    pub const fn saturating_add_signed(self, rhs: Bint<N>) -> Self {
        if rhs.is_negative() {
            saturate_down(self.overflowing_add_signed(rhs))
        } else {
            saturate_up(self.overflowing_add_signed(rhs))
        }
    }

    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        saturate_down(self.overflowing_sub(rhs))
    }

    #[inline]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        saturate_up(self.overflowing_mul(rhs))
    }

    #[inline]
    pub const fn saturating_pow(self, exp: ExpType) -> Self {
        saturate_up(self.overflowing_pow(exp))
    }
}

#[cfg(test)]
mod tests {
    test_unsigned! {
        function: saturating_add(a: u128, b: u128),
        cases: [
            (3945873849578934759897458u128, 304578347593745734845646957398u128),
            (u128::MAX, 345345u128)
        ]
    }
    test_unsigned! {
        function: saturating_sub(a: u128, b: u128),
        cases: [
            (43054734875u128, 304578347593745348455647398u128),
            (394587384957893459664565697458u128, 304578347593745348455647398u128)
        ]
    }
    test_unsigned! {
        function: saturating_mul(a: u128, b: u128),
        cases: [
            (u128::MAX, 1u128),
            (u128::MAX, 345u128)
        ]
    }
    test_unsigned! {
        function: saturating_pow(a: u128, b: u16),
        cases: [
            (3593745u128, 3451 as u16),
            (11u128, 34 as u16)
        ]
    }
}