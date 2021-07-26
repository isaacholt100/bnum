use super::BIint;
use crate::ExpType;

impl<const N: usize> BIint<N> {
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self {
            uint: self.uint.wrapping_mul(rhs.uint),
        }
    }
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.overflowing_div_euclid(rhs).0
    }
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.overflowing_rem_euclid(rhs).0
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
    crate::macros::wrapping_pow!();
    pub const fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }
    // TODO: test methods
}



#[cfg(test)]
mod tests {
    use crate::I128;

    test_signed! {
        name: wrapping_add,
        method: {
            wrapping_add(i128::MAX, 1i128);
            wrapping_add(i128::MIN, i128::MIN);
            wrapping_add(i128::MIN, i128::MAX);
            wrapping_add(-2937593745679792795864564i128, -93457690295769847589679456i128);
        }
    }
    test_signed! {
        name: wrapping_sub,
        method: {
            wrapping_sub(i128::MIN, i128::MAX);
            wrapping_sub(i128::MAX, i128::MIN);
            wrapping_sub(87297694759687456797345867i128, -204958769457689745896i128);
        }
    }
    test_signed! {
        name: wrapping_mul,
        method: {
            wrapping_mul(-20459679476947i128, 2454563i128);
            wrapping_mul(-97304980546i128, -82456957489i128);
            wrapping_mul(2947056979845769i128, 456354567575675i128);
            wrapping_mul(-294769347596879457698456i128, -274596749576894564565i128);
            wrapping_mul(827549607927459867984257689456i128, -982749857697458967456456456456i128);
            wrapping_mul(-27945769874598679487564567894i128, -729307968789347658979456i128);
            wrapping_mul(i128::MIN, -1i128);
            wrapping_mul(i128::MIN, i128::MIN);
            wrapping_mul(i128::MAX, i128::MAX);
            wrapping_mul(i128::MIN, i128::MAX);
        }
    }
    test_signed! {
        name: wrapping_div,
        method: {
            wrapping_div(2459797939576437967897567567i128, -9739457689456456456456i128);
            wrapping_div(-247596802947563975967497564596456i128, -29475697294564566i128);
            wrapping_div(-274957645i128, -12074956872947569874895376456465456i128);
            wrapping_div(i128::MIN, -1i128);
        }
    }
    test_signed! {
        name: wrapping_div_euclid,
        method: {
            wrapping_div_euclid(2745345697293475693745979367987567i128, 8956274596748957689745867456456i128);
            wrapping_div_euclid(-72957698456456573567576i128, 9874i128);
        }
    }
    test_signed! {
        name: wrapping_rem,
        method: {
            wrapping_rem(793579679875698739567947568945667i128, 729569749689476589748576974i128);
            wrapping_rem(-457689375896798456456456456i128, -749257698745i128);
            wrapping_rem(-297546987794275698749857645969748576i128, 78972954767345976894757489576i128);
            wrapping_rem(-234645634564356456i128, 234645634564356456 * 247459769457645i128);
            wrapping_rem(i128::MIN, -1i128);
        }
    }
    test_signed! {
        name: wrapping_rem_euclid,
        method: {
            wrapping_rem_euclid(-823476907495675i128, 823476907495675i128 * 74859607789455i128);
            wrapping_rem_euclid(-29835706972945768979456546i128, -2495678947568979456456i128);
            wrapping_rem_euclid(-729750967395476845645645656i128, 27459806798725896798456i128);
            wrapping_rem_euclid(79819764259879827964579864565i128, -79245697297569874589678456i128);
            wrapping_rem_euclid(692405691678627596874856456i128, 79027672495756456456i128);
        }
    }
    test_signed! {
        name: wrapping_neg,
        method: {
            wrapping_neg(-27946793759876567567567i128);
            wrapping_neg(947697985762756972498576874856i128);
            wrapping_neg(i128::MIN);
            wrapping_neg(-0i128);
        }
    }
    test_signed! {
        name: wrapping_shl,
        method: {
            wrapping_shl(792470986798345769745645685645i128, 2165 as u16);
            wrapping_shl(-72459867475967498576849565i128, 1523 as u16);
            wrapping_shl(i128::MIN, 8457 as u16);
            wrapping_shl(-9726589749856456456456i128, 65 as u16);
        }
    }
    test_signed! {
        name: wrapping_shr,
        method: {
            wrapping_shr(-1i128, 101 as u16);
            wrapping_shr(-72985769795768973458967984576i128, 1658 as u16);
            wrapping_shr(1797465927984576982745896799i128, 15128 as u16);
        }
    }
    test_signed! {
        name: wrapping_pow,
        method: {
            wrapping_pow(-79576456i128, 14500 as u16);
            wrapping_pow(-14i128, 17 as u16);
            wrapping_pow(-23i128, 20 as u16);
            wrapping_pow(8i128, 29345 as u16);
            wrapping_pow(-85i128, 9 as u16);
            wrapping_pow(2i128, 127 as u16);
        }
    }
}