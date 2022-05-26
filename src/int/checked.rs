use super::Bint;
use crate::macros::checked_pow;
use crate::{ExpType, BUint};
use crate::doc;

#[inline]
const fn tuple_to_option<const N: usize>((int, overflow): (Bint<N>, bool)) -> Option<Bint<N>> {
    if overflow {
        None
    } else {
        Some(int)
    }
}

macro_rules! checked_log {
    ($method: ident $(, $base: ident: $ty: ty)?) => {
        #[inline]
        pub const fn $method(self $(, $base: $ty)?) -> Option<ExpType> {
            if self.is_negative() {
                None
            } else {
                self.bits.$method($($base)?)
            }
        }
    }
}

impl<const N: usize> Bint<N> {
    #[inline]
    #[doc=doc::checked_add!(Bint::<2>)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_add(rhs))
    }

    #[inline]
    pub const fn checked_add_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_unsigned(rhs))
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }

    #[inline]
    pub const fn checked_sub_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_unsigned(rhs))
    }

    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }

    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_div(rhs))
        }
    }

    #[inline]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_div_euclid(rhs))
        }
    }

    #[inline]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_rem(rhs))
        }
    }

    #[inline]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_rem_euclid(rhs))
        }
    }

    #[inline]
    pub const fn checked_neg(self) -> Option<Self> {
        tuple_to_option(self.overflowing_neg())
    }

    #[inline]
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
        tuple_to_option(self.overflowing_shl(rhs))
    }

    #[inline]
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        tuple_to_option(self.overflowing_shr(rhs))
    }

    #[inline]
    pub const fn checked_abs(self) -> Option<Self> {
        tuple_to_option(self.overflowing_abs())
    }
    
    checked_pow!();
    checked_log!(checked_log2);
    checked_log!(checked_log10);
}

#[cfg(test)]
mod tests {
    test_signed! {
        function: checked_add(a: i128, b: i128),
        cases: [
            (-23967907456549865i128, 20459867945345546i128),
            (i128::MAX, 1i128)
        ]
    }
    test_signed! {
        function: checked_sub(a: i128, b: i128),
        cases: [
            (20974950679475645345i128, -92347569026164856487654i128),
            (-23947604957694857656i128, -202092349587049567495675i128)
        ]
    }
    test_signed! {
        function: checked_mul(a: i128, b: i128),
        cases: [
            (i128::MIN, -1i128)
        ]
    }
    test_signed! {
        function: checked_div(a: i128, b: i128),
        cases: [
            (2249495769845768947598254i128, -497495769i128),
            (-20907564975789647596748956456i128, -4096579405794756974586i128),
            (-34564564564i128, -33219654565456456453434545697i128)
        ]
    }
    test_signed! {
        function: checked_div_euclid(a: i128, b: i128),
        cases: [
            (203967405967394576984756897i128, 20495876945762097956546i128),
            (-203597649576948756456453345i128, 820459674957689i128)
        ]
    }
    test_signed! {
        function: checked_rem(a: i128, b: i128),
        cases: [
            (20459671029456874957698457698i128, 819475697456465656i128),
            (-2045967240596724598645645i128, -3479475689457i128),
            (-45679029357694576987245896765i128, -309768972045967498576i128)
        ]
    }
    test_signed! {
        function: checked_rem_euclid(a: i128, b: i128),
        cases: [
            (10349724596745674589647567456i128, 4697230968746597i128),
            (-10349724596745674589647567456i128, -4697230968746597i128),
            (-409725978957694794454865i128, 2045967495769859645i128)
        ]
    }
    test_signed! {
        function: checked_neg(a: i128),
        cases: [
            (-239794576947569847566236i128),
            (-872340961370495749576895i128),
            (i128::MIN)
        ]
    }
    test_signed! {
        function: checked_shl(a: i128, b: u16),
        cases: [
            (1907304597249567965987i128, 21 as u16),
            (-2023973209458764967589i128, 15 as u16),
            (2845197495679875698546i128, 8457 as u16)
        ]
    }
    test_signed! {
        function: checked_shr(a: i128, b: u16),
        cases: [
            (61354072459679717429576097i128, 120 as u16),
            (-23045692977456978956795i128, 18 as u16),
            (203967947569745986748956i128, 128 as u16)
        ]
    }
    test_signed! {
        function: checked_pow(a: i128, b: u16),
        cases: [
            (-13i128, 22 as u16),
            (7i128, 29 as u16)
        ]
    }
}