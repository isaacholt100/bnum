use super::{BUint, ExpType};
use crate::Bint;
use crate::macros::{expect, wrapping_pow};

impl<const N: usize> BUint<N> {
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
    pub const fn wrapping_add_signed(self, rhs: Bint<N>) -> Self {
        self.overflowing_add_signed(rhs).0
    }
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        expect!(self.checked_div(rhs), "attempt to divide by zero")
    }
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        expect!(self.checked_rem(rhs), "attempt to calculate the remainder with a divisor of zero")
    }
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }
    pub const fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }
    pub const fn wrapping_shl(self, rhs: ExpType) -> Self {
        self.overflowing_shl(rhs).0
    }
    pub const fn wrapping_shr(self, rhs: ExpType) -> Self {
        self.overflowing_shr(rhs).0
    }
    wrapping_pow!();
}

#[cfg(test)]
mod tests {
    test_unsigned! {
        function: wrapping_add(a: u128, b: u128),
        cases: [
            (u128::MAX - 394857938475u128, 3947587348957384975893475983744567797u128),
            (984756897982709347597234977937u128, 4957698475906748597694574094567944u128)
        ]
    }
    test_unsigned! {
        function: wrapping_sub(a: u128, b: u128),
        cases: [
            (34593475897340985709493475u128, 3947587348957384975893475983744567797u128),
            (1030495898347598730975979834759739457u128, 4957698475906748597694574094567944u128)
        ]
    }
    test_unsigned! {
        function: wrapping_mul(a: u128, b: u128),
        cases: [
            (3495739457839457394759794056809u128, 2u128),
            (294576809458698734905649865789746u128, 387544865834759837495354u128)
        ]
    }
    test_unsigned! {
        function: wrapping_div(a: u128, b: u128),
        cases: [
            (908940869048689045869048680405869009u128, 9347539457839475893475959u128),
            (9476485690845684567394573544345543u128, 349587458697u128)
        ],
        quickcheck_skip: b == 0
    }
    test_unsigned! {
        function: wrapping_div_euclid(a: u128, b: u128),
        cases: [
            (495769576475698737689374598674899857u128, 856894756457986456u128),
            (13495893475u128, 349583453457458697u128)
        ],
        quickcheck_skip: b == 0
    }
    test_unsigned! {
        function: wrapping_rem(a: u128, b: u128),
        cases: [
            (3749867984576984576u128, 32948754985690845697459867u128),
            (9713957246732875468937458973495u128, 498674957697458967498576u128)
        ],
        quickcheck_skip: b == 0
    }
    test_unsigned! {
        function: wrapping_rem_euclid(a: u128, b: u128),
        cases: [
            (5934758937458945864956899u128, 2438578934756345734875839745889u128),
            (4598674995476u128, 3943894579u128)
        ],
        quickcheck_skip: b == 0
    }
    test_unsigned! {
        function: wrapping_neg(a: u128),
        cases: [
            (0u128),
            (9476456827497389475983745979345u128)
        ]
    }
    test_unsigned! {
        function: wrapping_shl(a: u128, b: u16),
        cases: [
            (932769476979405769495675499u128, 35879 as u16),
            (1093457349754735987349857453u128, 77 as u16),
            (149247380901725085354480586487102439424u128, 13 as u16)
        ]
    }
    test_unsigned! {
        function: wrapping_shr(a: u128, b: u16),
        cases: [
            (37462745603947597349857345u128, 1973 as u16),
            (13945603458763847597349573945u128, 14 as u16),
            (13945603458763847597349573945u128, 4 as u16)
        ]
    }
    test_unsigned! {
        function: wrapping_pow(a: u128, b: u16),
        cases: [
            (345973457475345345564u128, 45613 as u16),
            (234u128, 6 as u16)
        ]
    }
}