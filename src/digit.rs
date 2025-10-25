pub type Digit = u8;
pub type DoubleDigit = u16;

#[inline]
pub const fn to_double_digit(low: Digit, high: Digit) -> DoubleDigit {
    ((high as DoubleDigit) << Digit::BITS) | low as DoubleDigit
}

// TODO: these will no longer be necessary once const_bigint_helper_methods is stabilised: https://github.com/rust-lang/rust/issues/85532

#[inline]
pub const fn carrying_add_u128(a: u128, b: u128, carry: bool) -> (u128, bool) {
    let (s1, o1) = a.overflowing_add(b);
    if carry {
        let (s2, o2) = s1.overflowing_add(1);
        (s2, o1 || o2)
    } else {
        (s1, o1)
    }
}

#[inline]
pub const fn carrying_add(a: Digit, b: Digit, carry: bool) -> (Digit, bool) {
    let (s1, o1) = a.overflowing_add(b);
    if carry {
        let (s2, o2) = s1.overflowing_add(1);
        (s2, o1 || o2)
    } else {
        (s1, o1)
    }
}

#[inline]
pub const fn borrowing_sub(a: Digit, b: Digit, borrow: bool) -> (Digit, bool) {
    let (s1, o1) = a.overflowing_sub(b);
    if borrow {
        let (s2, o2) = s1.overflowing_sub(1);
        (s2, o1 || o2)
    } else {
        (s1, o1)
    }
}

#[inline]
pub const fn borrowing_sub_u128(a: u128, b: u128, borrow: bool) -> (u128, bool) {
    let (s1, o1) = a.overflowing_sub(b);
    if borrow {
        let (s2, o2) = s1.overflowing_sub(1);
        (s2, o1 || o2)
    } else {
        (s1, o1)
    }
}

#[inline]
pub const fn widening_mul(a: Digit, b: Digit) -> (Digit, Digit) {
    let prod = a as DoubleDigit * b as DoubleDigit;
    (prod as Digit, (prod >> Digit::BITS) as Digit)
}

#[inline]
pub const fn carrying_mul(a: Digit, b: Digit, carry: Digit, current: Digit) -> (Digit, Digit) {
    let prod =
        carry as DoubleDigit + current as DoubleDigit + (a as DoubleDigit) * (b as DoubleDigit);
    (prod as Digit, (prod >> Digit::BITS) as Digit)
}

#[inline]
pub const fn carrying_mul_u128(a: u128, b: u128, carry: u128, current: u128) -> (u128, u128) {
    let (a_lo, a_hi) = (a as u64, (a >> 64) as u64);
    let (b_lo, b_hi) = (b as u64, (b >> 64) as u64);
    let (c_lo, c_hi) = (carry as u64, (carry >> 64) as u64);
    let (d_lo, d_hi) = (current as u64, (current >> 64) as u64);
    let x = (a_lo as u128) * (b_lo as u128) + (c_lo as u128) + (d_lo as u128);
    let y = (a_lo as u128) * (b_hi as u128) + (c_hi as u128) + (d_hi as u128);
    let (y, carry_y) = y.overflowing_add((a_hi as u128) * (b_lo as u128));
    let (x, carry_x) = x.overflowing_add(y << 64);
    let carry2 = if carry_y { 1 << 64 } else { 0 };
    let carry3 = if carry_x { 1 } else { 0 };
    let z = (a_hi as u128) * (b_hi as u128) + carry2 + carry3 + (y >> 64);

    (x, z)
}

#[inline]
pub const fn div_rem_wide(low: Digit, high: Digit, rhs: Digit) -> (Digit, Digit) {
    debug_assert!(high < rhs);

    let a = to_double_digit(low, high);
    (
        (a / rhs as DoubleDigit) as Digit,
        (a % rhs as DoubleDigit) as Digit,
    )
}

#[inline]
pub const fn div_rem_wide_u64(low: u64, high: u64, rhs: u64) -> (u64, u64) {
    debug_assert!(high < rhs);

    let a = ((high as u128) << 64) | (low as u128);
    (
        (a / rhs as u128) as u64,
        (a % rhs as u128) as u64,
    )
}

// #[inline]
// pub const fn carrying_add_u64(a: u64, b: u64, carry: bool) -> (u64, bool) {
//     let (s1, o1) = a.overflowing_add(b);
//     if carry {
//         let (s2, o2) = s1.overflowing_add(1);
//         (s2, o1 || o2)
//     } else {
//         (s1, o1)
//     }
// }

// #[inline]
// pub const fn borrowing_sub_u64(a: u64, b: u64, borrow: bool) -> (u64, bool) {
//     let (s1, o1) = a.overflowing_sub(b);
//     if borrow {
//         let (s2, o2) = s1.overflowing_sub(1);
//         (s2, o1 || o2)
//     } else {
//         (s1, o1)
//     }
// }
