use super::BInt;
use crate::{BUint, ExpType};
use crate::doc;
use crate::digit::{self, Digit};

macro_rules! pos_const {
    ($($name: ident $num: literal), *) => {
        $(
            #[doc=concat!("The value of ", $num, " represented by this type.")]
            pub const $name: Self = Self::from_bits(BUint::$name);
        )*
    }
}

macro_rules! neg_const {
    ($($name: ident $pos: ident $num: literal), *) => {
        $(
            #[doc=concat!("The value of -", $num, " represented by this type.")]
            pub const $name: Self = Self::$pos.wrapping_neg();
        )*
    }
}

#[doc=doc::consts::impl_desc!()]
impl<const N: usize> BInt<N> {
    #[doc=doc::consts::min!(I 512)]
    pub const MIN: Self = {
        let mut digits = [0; N];
        digits[N - 1] = 1 << (digit::BITS - 1);
        Self::from_bits(BUint::from_digits(digits))
    };

    #[doc=doc::consts::max!(I 512)]
    pub const MAX: Self = {
        let mut digits = [Digit::MAX; N];
        digits[N - 1] >>= 1;
        Self::from_bits(BUint::from_digits(digits))
    };

    #[doc=doc::consts::bits!(I 512, 512)]
    pub const BITS: ExpType = BUint::<N>::BITS;

    #[doc=doc::consts::bytes!(I 512, 512)]
    pub const BYTES: ExpType = BUint::<N>::BYTES;

    #[doc=doc::consts::zero!(I 512)]
    pub const ZERO: Self = Self::from_bits(BUint::ZERO);

    #[doc=doc::consts::one!(I 512)]
    pub const ONE: Self = Self::from_bits(BUint::ONE);

    pos_const!(TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

    neg_const!(NEG_ONE ONE 1, NEG_TWO TWO 2, NEG_THREE THREE 3, NEG_FOUR FOUR 4, NEG_FIVE FIVE 5, NEG_SIX SIX 6, NEG_SEVEN SEVEN 7, NEG_EIGHT EIGHT 8, NEG_NINE NINE 9, NEG_TEN TEN 10);

    pub(super) const N_MINUS_1: usize = N - 1;
}