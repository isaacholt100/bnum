use super::{BUint, ExpType};
use crate::macros::expect;

impl<const N: usize> BUint<N> {
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
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
    pub const fn wrapping_pow(self, exp: ExpType) -> Self {
        self.overflowing_pow(exp).0
    }
}

#[cfg(test)]
mod tests {
    use crate::{U128};
    
    test_unsigned! {
        name: wrapping_add,
        method: {
            wrapping_add(u128::MAX - 394857938475u128, 3947587348957384975893475983744567797u128);
            wrapping_add(984756897982709347597234977937u128, 4957698475906748597694574094567944u128);
        }
    }
    test_unsigned! {
        name: wrapping_sub,
        method: {
            wrapping_sub(34593475897340985709493475u128, 3947587348957384975893475983744567797u128);
            wrapping_sub(1030495898347598730975979834759739457u128, 4957698475906748597694574094567944u128);
        }
    }
    test_unsigned! {
        name: wrapping_mul,
        method: {
            wrapping_mul(3495739457839457394759794056809u128, 2u128);
            wrapping_mul(294576809458698734905649865789746u128, 387544865834759837495354u128);
        }
    }
    test_unsigned! {
        name: wrapping_div,
        method: {
            wrapping_div(908940869048689045869048680405869009u128, 9347539457839475893475959u128);
            wrapping_div(9476485690845684567394573544345543u128, 349587458697u128);
        }
    }
    test_unsigned! {
        name: wrapping_div_euclid,
        method: {
            wrapping_div_euclid(495769576475698737689374598674899857u128, 856894756457986456u128);
            wrapping_div_euclid(13495893475u128, 349583453457458697u128);
        }
    }
    test_unsigned! {
        name: wrapping_rem,
        method: {
            wrapping_rem(3749867984576984576u128, 32948754985690845697459867u128);
            wrapping_rem(9713957246732875468937458973495u128, 498674957697458967498576u128);
        }
    }
    test_unsigned! {
        name: wrapping_rem_euclid,
        method: {
            wrapping_rem_euclid(5934758937458945864956899u128, 2438578934756345734875839745889u128);
            wrapping_rem_euclid(4598674995476u128, 3943894579u128);
        }
    }
    test_unsigned! {
        name: wrapping_neg,
        method: {
            wrapping_neg(0u128);
            wrapping_neg(9476456827497389475983745979345u128);
        }
    }
    test_unsigned! {
        name: wrapping_shl,
        method: {
            wrapping_shl(932769476979405769495675499u128, 35879 as u16);
            wrapping_shl(1093457349754735987349857453u128, 77 as u16);
        }
    }
    test_unsigned! {
        name: wrapping_shr,
        method: {
            wrapping_shr(37462745603947597349857345u128, 1973 as u16);
            wrapping_shr(13945603458763847597349573945u128, 14 as u16);
        }
    }
    test_unsigned! {
        name: wrapping_pow,
        method: {
            wrapping_pow(345973457475345345564u128, 45613 as u16);
            wrapping_pow(234u128, 6 as u16);
        }
    }
}