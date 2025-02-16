use super::BUintD8;
use crate::{digit, Digit};

macro_rules! pos_const {
    ($($name: ident $num: literal), *) => {
        $(
            #[doc = doc::consts::value_desc!($num)]
            pub const $name: Self = Self::from_digit($num);
        )*
    }
}

use crate::doc;
use crate::ExpType;

#[doc = doc::consts::impl_desc!()]
impl<const N: usize> BUintD8<N> {
    #[doc = doc::consts::min!(U 512)]
    pub const MIN: Self = Self::from_digits([Digit::MIN; N]);

    #[doc = doc::consts::max!(U 512)]
    pub const MAX: Self = Self::from_digits([Digit::MAX; N]);

    #[doc = doc::consts::bits!(U 512, 512)]
    pub const BITS: ExpType = digit::BITS * N as ExpType;

    #[doc = doc::consts::bytes!(U 512, 512)]
    pub const BYTES: ExpType = Self::BITS / 8;

    #[doc = doc::consts::zero!(U 512)]
    pub const ZERO: Self = Self::MIN;

    pos_const!(ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

    pub(crate) const U128_DIGITS: usize = Self::BITS as usize / 128;
    pub(crate) const U128_DIGIT_REMAINDER: usize = Self::BITS as usize % 128;
}
