use super::BIntD8;
use crate::{BUintD8, Digit};

macro_rules! pos_const {
    ($($name: ident $num: literal), *) => {
        $(
            #[doc = doc::consts::value_desc!($num)]
            pub const $name: Self = Self::from_bits(BUintD8::$name);
        )*
    }
}

macro_rules! neg_const {
    ($($name: ident $num: literal), *) => {
        $(
            #[doc = doc::consts::value_desc!("-" $num)]
            pub const $name: Self = {
                let mut u = BUintD8::MAX;
                u.digits[0] -= ($num - 1);
                Self::from_bits(u)
            };
        )*
    }
}

use crate::doc;
use crate::ExpType;

#[doc = doc::consts::impl_desc!()]
impl<const N: usize> BIntD8<N> {
    #[doc = doc::consts::min!(I 512)]
    pub const MIN: Self = {
        let mut digits = [0; N];
        digits[N - 1] = 1 << (Digit::BITS - 1);
        Self::from_bits(BUintD8::from_digits(digits))
    };

    #[doc = doc::consts::max!(I 512)]
    pub const MAX: Self = {
        let mut digits = [Digit::MAX; N];
        digits[N - 1] >>= 1;
        Self::from_bits(BUintD8::from_digits(digits))
    };

    #[doc = doc::consts::bits!(I 512, 512)]
    pub const BITS: ExpType = BUintD8::<N>::BITS;

    #[doc = doc::consts::bytes!(I 512, 512)]
    pub const BYTES: ExpType = BUintD8::<N>::BYTES;

    #[doc = doc::consts::zero!(I 512)]
    pub const ZERO: Self = Self::from_bits(BUintD8::ZERO);

    #[doc = doc::consts::one!(I 512)]
    pub const ONE: Self = Self::from_bits(BUintD8::ONE);

    pos_const!(TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

    neg_const!(NEG_ONE 1, NEG_TWO 2, NEG_THREE 3, NEG_FOUR 4, NEG_FIVE 5, NEG_SIX 6, NEG_SEVEN 7, NEG_EIGHT 8, NEG_NINE 9, NEG_TEN 10);

    pub(crate) const N_MINUS_1: usize = N - 1;
}
