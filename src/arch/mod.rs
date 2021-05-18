mod adc;
mod sbb;

pub use adc::*;
pub use sbb::*;

#[cfg(all(use_addcarry, target_arch = "x86_64"))]
use core::arch::x86_64 as arch;

#[cfg(all(use_addcarry, target_arch = "x86"))]
use core::arch::x86 as arch;

use crate::digit::{Digit, DoubleDigit, self};
/// Tuple of (product, carry)
pub const fn mul_carry_unsigned(carry: Digit, current: Digit, a: Digit, b: Digit) -> (Digit, Digit) {
    let prod = carry as DoubleDigit + current as DoubleDigit + (a as DoubleDigit) * (b as DoubleDigit);
    (prod as Digit, (prod >> digit::BITS) as u64)
}