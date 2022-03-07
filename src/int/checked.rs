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
        pub const fn $method(self $(, $base: $ty)?) -> Option<ExpType> {
            if self.is_negative() {
                None
            } else {
                self.uint.$method($($base)?)
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
    pub const fn checked_add_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_add_unsigned(rhs))
    }
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_sub(rhs))
    }
    pub const fn checked_sub_unsigned(self, rhs: BUint<N>) -> Option<Self> {
        tuple_to_option(self.overflowing_sub_unsigned(rhs))
    }
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        tuple_to_option(self.overflowing_mul(rhs))
    }
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_div(rhs))
        }
    }
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_div_euclid(rhs))
        }
    }
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_rem(rhs))
        }
    }
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            tuple_to_option(self.overflowing_rem_euclid(rhs))
        }
    }
    pub const fn checked_neg(self) -> Option<Self> {
        tuple_to_option(self.overflowing_neg())
    }
    pub const fn checked_shl(self, rhs: ExpType) -> Option<Self> {
        tuple_to_option(self.overflowing_shl(rhs))
    }
    pub const fn checked_shr(self, rhs: ExpType) -> Option<Self> {
        tuple_to_option(self.overflowing_shr(rhs))
    }
    pub const fn checked_abs(self) -> Option<Self> {
        tuple_to_option(self.overflowing_abs())
    }
    checked_pow!();
    checked_log!(checked_log2);
    checked_log!(checked_log10);
}

#[cfg(test)]
mod tests {
    use crate::{I128};

    fn converter(prim_result: Option<i128>) -> Option<I128> {
        match prim_result {
            Some(i) => Some(I128::from(i)),
            None => None,
        }
    }

    test_signed! {
        function: checked_add(a: i128, b: i128),
        method: {
            checked_add(-23967907456549865i128, 20459867945345546i128);
            checked_add(i128::MAX, 1i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_sub(a: i128, b: i128),
        method: {
            checked_sub(20974950679475645345i128, -92347569026164856487654i128);
            checked_sub(-23947604957694857656i128, -202092349587049567495675i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_mul(a: i128, b: i128),
        method: {
            checked_mul(-209746039567459i128, 457684957i128);
            checked_mul(-20396745i128, -239486457676i128);
            checked_mul(203967459679i128, 2394864576i128);
            checked_mul(2039674596i128, -239486457i128);
            checked_mul(-204564564564539674596i128, -2394564564564564486457i128);
            checked_mul(204564564564539674596i128, -2394564564564564486457i128);
            checked_mul(i128::MIN, -1i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_div(a: i128, b: i128),
        method: {
            checked_div(2249495769845768947598254i128, -497495769i128);
            checked_div(-20907564975789647596748956456i128, -4096579405794756974586i128);
            checked_div(-34564564564i128, -33219654565456456453434545697i128);
            checked_div(-475749674596i128, 0i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_div_euclid(a: i128, b: i128),
        method: {
            checked_div_euclid(203967405967394576984756897i128, 20495876945762097956546i128);
            checked_div_euclid(-203597649576948756456453345i128, 820459674957689i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_rem(a: i128, b: i128),
        method: {
            checked_rem(20459671029456874957698457698i128, 819475697456465656i128);
            checked_rem(-2045967240596724598645645i128, -3479475689457i128);
            checked_rem(-45679029357694576987245896765i128, -309768972045967498576i128);
            checked_rem(240567945769845i128, 0i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_rem_euclid(a: i128, b: i128),
        method: {
            checked_rem_euclid(10349724596745674589647567456i128, 4697230968746597i128);
            checked_rem_euclid(-10349724596745674589647567456i128, -4697230968746597i128);
            checked_rem_euclid(-409725978957694794454865i128, 2045967495769859645i128);
            checked_rem_euclid(9847204958713460947596874i128, -200947612019376974956i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_neg(a: i128),
        method: {
            checked_neg(-239794576947569847566236i128);
            checked_neg(-872340961370495749576895i128);
            checked_neg(i128::MIN);
            checked_neg(-0i128);
        },
        converter: converter
    }
    test_signed! {
        function: checked_shl(a: i128, b: u16),
        method: {
            checked_shl(1907304597249567965987i128, 21 as u16);
            checked_shl(-2023973209458764967589i128, 15 as u16);
            checked_shl(2845197495679875698546i128, 8457 as u16);
        },
        converter: converter
    }
    test_signed! {
        function: checked_shr(a: i128, b: u16),
        method: {
            checked_shr(61354072459679717429576097i128, 120 as u16);
            checked_shr(-23045692977456978956795i128, 18 as u16);
            checked_shr(203967947569745986748956i128, 128 as u16);
        },
        converter: converter
    }
    test_signed! {
        function: checked_pow(a: i128, b: u16),
        method: {
            checked_pow(-2397456i128, 100 as u16);
            checked_pow(-13i128, 22 as u16);
            checked_pow(-26i128, 15 as u16);
            checked_pow(7i128, 29 as u16);
            checked_pow(-2i128, 127 as u16);
        },
        converter: converter
    }
}