use crate::digit::{Digit, SignedDigit, self};
use crate::uint::BUint;
#[allow(unused_imports)]
use crate::I128;
use crate::macros::option_expect;
use crate::ExpType;
use crate::doc;

#[allow(unused)]
macro_rules! test_signed {
    {
        function: $name: ident ($($param: ident : $ty: ty), *)
        $(,cases: [
            $(($($arg: expr), *)), *
        ])?
        $(,quickcheck_skip: $skip: expr)?
    } => {
        crate::test::test_big_num! {
            big: crate::I128,
            primitive: i128,
            function: $name,
            $(cases: [
                $(($($arg), *)), *
            ],)?
            quickcheck: ($($param : $ty), *),
            $(quickcheck_skip: $skip,)?
            converter: Into::into
        }
    };
    {
        function: $name: ident ($($param: ident : $ty: ty), *)
        $(,cases: [
            $(($($arg: expr), *)), *
        ])?
        $(,quickcheck_skip: $skip: expr)?,
        converter: $converter: expr
    } => {
        crate::test::test_big_num! {
            big: crate::I128,
            primitive: i128,
            function: $name,
            $(cases: [
                $(($($arg), *)), *
            ],)?
            quickcheck: ($($param : $ty), *),
            $(quickcheck_skip: $skip,)?
            converter: $converter
        }
    };
}

mod cast;
mod checked;
mod cmp;
mod convert;
mod endian;
mod fmt;
#[cfg(feature = "numtraits")]
mod numtraits;
mod ops;
mod overflowing;
mod radix;
mod saturating;
mod unchecked;
mod wrapping;

#[cfg(feature = "serde_all")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Hash, /*Debug, */)]
#[cfg_attr(feature = "serde_all", derive(Serialize, Deserialize))]
pub struct Bint<const N: usize> {
    bits: BUint<N>,
}

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

impl<const N: usize> Bint<N> {
    #[doc=doc::min_const!(Bint::<2>)]
    pub const MIN: Self = {
        let mut digits = [0; N];
        digits[N - 1] = 1 << (digit::BITS - 1);
        Self::from_bits(BUint::from_digits(digits))
    };

    #[doc=doc::max_const!(Bint::<2>)]
    pub const MAX: Self = {
        let mut digits = [Digit::MAX; N];
        digits[N - 1] >>= 1;
        Self::from_bits(BUint::from_digits(digits))
    };

    #[doc=doc::zero_const!(Bint::<2>)]
    pub const ZERO: Self = {
        Self::from_bits(BUint::ZERO)
    };

    #[doc=doc::one_const!(Bint::<2>)]
    pub const ONE: Self = {
        Self::from_bits(BUint::ONE)
    };

    pos_const!(TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

    neg_const!(NEG_ONE ONE 1, NEG_TWO TWO 2, NEG_THREE THREE 3, NEG_FOUR FOUR 4, NEG_FIVE FIVE 5, NEG_SIX SIX 6, NEG_SEVEN SEVEN 7, NEG_EIGHT EIGHT 8, NEG_NINE NINE 9, NEG_TEN TEN 10);

    #[doc=doc::bits_const!(Bint::<2>, 64)]
    pub const BITS: ExpType = BUint::<N>::BITS;

    #[doc=doc::bytes_const!(Bint::<2>, 8)]
    pub const BYTES: ExpType = BUint::<N>::BYTES;

    const N_MINUS_1: usize = N - 1;
}

macro_rules! log {
    ($method: ident $(, $base: ident : $ty: ty)?) => {
        #[inline]
        pub const fn $method(self, $($base : $ty),*) -> ExpType {
            if self.is_negative() {
                #[cfg(debug_assertions)]
                panic!("attempt to calculate log of negative number");
                #[cfg(not(debug_assertions))]
                0
            } else {
                self.bits.$method($($base.bits)?)
            }
        }
    }
}

impl<const N: usize> Bint<N> {
    #[doc=doc::count_ones!(Bint::<4>)]
    #[inline]
    pub const fn count_ones(self) -> ExpType {
        self.bits.count_ones()
    }

    #[doc=doc::count_zeros!(Bint::<5>)]
    #[inline]
    pub const fn count_zeros(self) -> ExpType {
        self.bits.count_zeros()
    }

    #[doc=doc::leading_zeros!(Bint::<3>)]
    #[inline]
    pub const fn leading_zeros(self) -> ExpType {
        self.bits.leading_zeros()
    }

    #[doc=doc::trailing_zeros!(Bint::<3>)]
    #[inline]
    pub const fn trailing_zeros(self) -> ExpType {
        self.bits.trailing_zeros()
    }

    #[doc=doc::leading_ones!(Bint::<4>, NEG_ONE)]
    #[inline]
    pub const fn leading_ones(self) -> ExpType {
        self.bits.leading_ones()
    }

    #[doc=doc::trailing_ones!(Bint::<6>)]
    #[inline]
    pub const fn trailing_ones(self) -> ExpType {
        self.bits.trailing_ones()
    }

    #[doc=doc::rotate_left!(Bint::<2>, "i")]
    #[inline]
    pub const fn rotate_left(self, n: ExpType) -> Self {
        Self::from_bits(self.bits.rotate_left(n))
    }

    #[doc=doc::rotate_right!(Bint::<2>, "i")]
    #[inline]
    pub const fn rotate_right(self, n: ExpType) -> Self {
        Self::from_bits(self.bits.rotate_right(n))
    }

    #[doc=doc::swap_bytes!(Bint::<2>, "i")]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
        Self::from_bits(self.bits.swap_bytes())
    }

    #[doc=doc::reverse_bits!(Bint::<6>, "i")]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
        Self::from_bits(self.bits.reverse_bits())
    }

    /// Computes the absolute value of `self` without any wrapping or panicking.
    #[doc=doc::example_header!(Bint)]
    /// assert_eq!(Bint::<3>::from(100).unsigned_abs(), Bint::from(100));
    /// assert_eq!(Bint::<3>::from(-100).unsigned_abs(), Bint::from(100));
    /// assert_eq!(Bint::<3>::MAX.unsigned_abs(), Bint::MAX.to_bits());
    /// ```
    #[inline]
    pub const fn unsigned_abs(self) -> BUint<N> {
        if self.is_negative() {
            self.wrapping_neg().bits
        } else {
            self.bits
        }
    }

    #[doc=doc::pow!(Bint::<4>)]
    #[inline]
    pub const fn pow(self, exp: ExpType) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_pow(exp), "attempt to calculate power with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_pow(exp)
    }

    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        assert!(self != Self::MIN || rhs != Self::NEG_ONE, "attempt to divide with overflow");
        self.wrapping_div_euclid(rhs)
    }

    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        assert!(self != Self::MIN || rhs != Self::NEG_ONE, "attempt to calculate remainder with overflow");
        self.wrapping_rem_euclid(rhs)
    }

    #[inline]
    pub const fn abs(self) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_abs(), "attempt to negate with; overflow");

        #[cfg(not(debug_assertions))]
        match self.checked_abs() {
            Some(int) => int,
            None => Self::MIN,
        }
    }

    #[inline]
    pub const fn signum(self) -> Self {
        if self.is_negative() {
            Self::NEG_ONE
        } else if self.is_zero() {
            Self::ZERO
        } else {
            Self::ONE
        }
    }

    #[inline]
    pub const fn is_positive(self) -> bool {
        let signed_digit = self.signed_digit();
        signed_digit.is_positive() ||
        (signed_digit == 0 && !self.bits.is_zero())
    }
    
    #[inline]
    pub const fn is_negative(self) -> bool {
        self.signed_digit().is_negative()
    }
    
    #[doc=doc::doc_comment! {
        Bint::<2>,
        "Returns `true` if and only if `self == 2^k` for some integer `k`.",
        
        "let n = " doc::int_str!(Bint::<2>) "::from(1i16 << 12);\n"
        "assert!(n.is_power_of_two());\n"
        "let m = " doc::int_str!(Bint::<2>) "::from(90i8);\n"
        "assert!(!m.is_power_of_two());"
        "assert!(!((-n).is_power_of_two()));"
    }]
    #[inline]
    pub const fn is_power_of_two(self) -> bool {
        if self.is_negative() {
            false
        } else {
            self.bits.is_power_of_two()
        }
    }

    #[doc=doc::next_power_of_two!(Bint::<2>, "`Self::MIN`", "NEG_ONE")]
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        #[cfg(debug_assertions)]
        return option_expect!(self.checked_next_power_of_two(), "attempt to calculate next power of two with overflow");

        #[cfg(not(debug_assertions))]
        self.wrapping_next_power_of_two()
    }

    #[doc=doc::checked_next_power_of_two!(Bint::<2>)]
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        if self.is_negative() {
            Some(Self::ONE)
        } else {
            match self.bits.checked_next_power_of_two() {
                Some(uint) => {
                    let out = Self::from_bits(uint);
                    if out.is_negative() {
                        None
                    } else {
                        Some(out)
                    }
                },
                None => None,
            }
        }
    }

    #[doc=doc::wrapping_next_power_of_two!(Bint::<2>, "`Self::MIN`")]
    #[inline]
    pub const fn wrapping_next_power_of_two(self) -> Self {
        match self.checked_next_power_of_two() {
            Some(int) => int,
            None => Self::MIN,
        }
    }

    log!(log, base: Self);
    log!(log2);
    log!(log10);

    #[inline]
    pub const fn abs_diff(self, other: Self) -> BUint<N> {
        if self < other {
            other.wrapping_sub(self).to_bits()
        } else {
            self.wrapping_sub(other).to_bits()
        }
    }

    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        // TODO: credit rust source code
        if rhs == Self::NEG_ONE {
            return Some(self);
        }
        let rem = match self.checked_rem(rhs) {
            Some(rem) => rem,
            None => return None,
        };
        let m = if (rem.is_positive() && rhs.is_negative()) || (rem.is_negative() && rhs.is_positive()) {
            match rem.checked_add(rhs) {
                Some(rem) => rem,
                None => return None,
            }
        } else {
            rem
        };

        if m.is_zero() {
            Some(self)
        } else {
            let sub = match rhs.checked_sub(m) {
                Some(sub) => sub,
                None => return None,
            };
            self.checked_add(sub)
        }
    }

    #[inline]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
        if rhs == Self::NEG_ONE {
            return self;
        }
        let rem = self % rhs;
        let m = if (rem.is_positive() && rhs.is_negative()) || (rem.is_negative() && rhs.is_positive()) {
            rem + rhs
        } else {
            rem
        };

        if m.is_zero() {
            self
        } else {
            self + (rhs - m)
        }
    }

    #[inline]
    pub const fn div_floor(self, rhs: Self) -> Self {
        let d = self / rhs;
        let r = self % rhs;
        if (r.is_positive() && rhs.is_negative()) || (r.is_negative() && rhs.is_positive()) {
            d - Self::ONE
        } else {
            d
        }
    }

    #[inline]
    pub const fn div_ceil(self, rhs: Self) -> Self {
        let d = self / rhs;
        let r = self % rhs;
        if (r.is_positive() && rhs.is_positive()) || (r.is_negative() && rhs.is_negative()) {
            d + Self::ONE
        } else {
            d
        }
    }
}

impl<const N: usize> Bint<N> {
    #[doc=doc::bits!(Bint::<2>)]
    #[inline]
    pub const fn bits(&self) -> ExpType {
        self.bits.bits()
    }

    #[doc=doc::bit!(Bint::<4>)]
    #[inline]
    pub const fn bit(&self, b: ExpType) -> bool {
        self.bits.bit(b)
    }

    #[inline(always)]
    const fn signed_digit(&self) -> SignedDigit {
        self.bits.digits()[N - 1] as SignedDigit
    }

    #[doc=doc::is_zero!(Bint::<2>)]
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.bits.is_zero()
    }

    #[doc=doc::is_one!(Bint::<2>)]
    #[inline]
    pub const fn is_one(self) -> bool {
        self.bits.is_one()
    }

    #[inline(always)]
    pub const fn digits(&self) -> &[Digit; N] {
        &self.bits.digits()
    }

    #[inline(always)]
    pub const fn from_digits(digits: [Digit; N]) -> Self {
        Self::from_bits(BUint::from_digits(digits))
    }

    #[inline(always)]
    pub const fn from_bits(bits: BUint<N>) -> Self {
        Self {
            bits,
        }
    }
    
    #[inline(always)]
    pub const fn to_bits(self) -> BUint<N> {
        self.bits
    }

    #[inline]
    pub const fn to_exp_type(self) -> Option<ExpType> {
        if self.is_negative() {
            None
        } else {
            self.bits.to_exp_type()
        }
    }
}

use core::default::Default;

impl<const N: usize> Default for Bint<N> {
    #[doc=doc::default!()]
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

use core::iter::{Iterator, Product, Sum};

impl<const N: usize> Product<Self> for Bint<N> {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, const N: usize> Product<&'a Self> for Bint<N> {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<const N: usize> Sum<Self> for Bint<N> {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, const N: usize> Sum<&'a Self> for Bint<N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    test_signed! {
        function: count_ones(a: i128),
        cases: [
            (34579834758459769875878374593749837548i128),
            (-720496794375698745967489576984655i128)
        ],
        converter: test::u32_to_exp
    }
    test_signed! {
        function: count_zeros(a: i128),
        cases: [
            (97894576897934857979834753847877889734i128),
            (-302984759749756756756756756756756i128)
        ],
        converter: test::u32_to_exp
    }
    test_signed! {
        function: leading_zeros(a: i128),
        cases: [
            (1234897937459789793445634456858978937i128),
            (-30979347598678947567567567i128),
            (0i128)
        ],
        converter: test::u32_to_exp
    }
    test_signed! {
        function: trailing_zeros(a: i128),
        cases: [
            (8003849534758937495734957034534073957i128),
            (-972079507984789567894375674857645i128),
            (0i128)
        ],
        converter: test::u32_to_exp
    }
    test_signed! {
        function: leading_ones(a: i128),
        cases: [
            (1),
            (290758976947569734598679898445i128),
            (-1)
        ],
        converter: test::u32_to_exp
    }
    test_signed! {
        function: trailing_ones(a: i128),
        cases: [
            (i128::MAX),
            (72984756897458906798456456456i128),
            (-1)
        ],
        converter: test::u32_to_exp
    }
    test_signed! {
        function: rotate_left(a: i128, b: u16),
        cases: [
            (3457894375984563459457i128, 61845 as u16),
            (-34598792345789i128, 4 as u16)
        ]
    }
    test_signed! {
        function: rotate_right(a: i128, b: u16),
        cases: [
            (109375495687201345976994587i128, 354 as u16),
            (-4598674589769i128, 75 as u16)
        ]
    }
    test_signed! {
        function: swap_bytes(a: i128),
        cases: [
            (98934757983792102304988394759834587i128),
            (-234i128)
        ]
    }
    test_signed! {
        function: reverse_bits(a: i128),
        cases: [
            (349579348509348374589749083490580i128),
            (-22003495898345i128)
        ]
    }
    test_signed! {
        function: unsigned_abs(a: i128),
        cases: [
            (i128::MIN),
            (0),
            (45645634534564264564534i128)
        ]
    }
    test_signed! {
        function: pow(a: i128, b: u16),
        cases: [
            (-564i128, 6 as u16),
            (6957i128, 8 as u16),
            (-67i128, 19 as u16)
        ],
        quickcheck_skip: a.checked_pow(b as u32).is_none()
    }
    test_signed! {
        function: div_euclid(a: i128, b: i128),
        cases: [
            (-29475698745698i128 * 4685684568, 29475698745698i128),
            (4294567897594568765i128, 249798748956i128),
            (27456979757i128, 45i128)
        ],
        quickcheck_skip: a.checked_div(b).is_none()
    }
    test_signed! {
        function: rem_euclid(a: i128, b: i128),
        cases: [
            (-7902709857689456456i128 * 947659873456, 7902709857689456456i128),
            (-46945656i128, 896794576985645i128),
            (-45679i128, -8i128)
        ],
        quickcheck_skip: a.checked_rem(b).is_none()
    }
    test_signed! {
        function: abs(a: i128),
        cases: [
            (0i128),
            (i128::MAX),
            (-249576984756i128)
        ],
        quickcheck_skip: a == i128::MIN
    }
    test_signed! {
        function: signum(a: i128),
        cases: [
            (0i128),
            (275966456345645635473947569i128),
            (-972945769613987589476598745i128)
        ]
    }
    test_signed! {
        function: is_positive(a: i128),
        cases: [
            (304950490384054358903845i128),
            (-34958789i128),
            (0i128)
        ]
    }
    test_signed! {
        function: is_negative(a: i128),
        cases: [
            (19847690479i128),
            (-1019487692i128),
            (0i128)
        ]
    }

    #[test]
    fn is_power_of_two() {
        assert!(!I128::from(-94956729465i128).is_power_of_two());
        assert!(!I128::from(79458945i128).is_power_of_two());
        assert!(I128::from(1i128 << 17).is_power_of_two());
    }

    #[test]
    fn next_power_of_two() {
        assert_eq!(I128::from(-372979834534345587i128).next_power_of_two(), 1i128.into());
        assert_eq!(I128::from((1i128 << 88) - 5).next_power_of_two(), (1i128 << 88).into());
        assert_eq!(I128::from(1i128 << 56).next_power_of_two(), (1i128 << 56).into());
    }

    #[test]
    fn checked_next_power_of_two() {
        assert_eq!(I128::from(-979457698).checked_next_power_of_two(), Some(1i128.into()));
        assert_eq!(I128::from(5).checked_next_power_of_two(), Some(8i32.into()));
        assert_eq!(I128::from(i128::MAX - 5).checked_next_power_of_two(), None);
    }

    #[test]
    fn wrapping_next_power_of_two() {
        assert_eq!(I128::from(-89i128).wrapping_next_power_of_two(), 1i128.into());
        assert_eq!(I128::from((1i128 << 75) + 4).wrapping_next_power_of_two(), (1i128 << 76).into());
        assert_eq!(I128::from(i128::MAX / 2 + 4).wrapping_next_power_of_two(), I128::MIN);
    }
    test_signed! {
        function: log(a: i128, base: i128),
        quickcheck_skip: a <= 0 || base <= 1,
        converter: test::u32_to_exp
    }
    test_signed! {
        function: log2(a: i128),
        quickcheck_skip: a <= 0,
        converter: test::u32_to_exp
    }
    test_signed! {
        function: log10(a: i128),
        quickcheck_skip: a <= 0,
        converter: test::u32_to_exp
    }
    test_signed! {
        function: abs_diff(a: i128, b: i128)
    }
    test_signed! {
        function: checked_next_multiple_of(a: i128, b: i128),
        converter: crate::test::converters::option_converter
    }
    test_signed! {
        function: next_multiple_of(a: i128, b: i128),
        quickcheck_skip: a.checked_next_multiple_of(b).is_none()
    }
    test_signed! {
        function: div_floor(a: i128, b: i128),
        quickcheck_skip: a.checked_div(b).is_none()
    }
    test_signed! {
        function: div_ceil(a: i128, b: i128),
        quickcheck_skip: a.checked_div(b).is_none()
    }
    
    #[test]
    fn bit() {
        let i = I128::from(0b1001010100101010101i128);
        assert!(i.bit(2));
        assert!(!i.bit(3));
        assert!(i.bit(8));
        assert!(!i.bit(9));
        assert!(i.bit(i.bits() - 1));
    }

    #[test]
    fn is_zero() {
        assert!(I128::ZERO.is_zero());
        assert!(!I128::MAX.is_zero());
        assert!(!I128::ONE.is_zero());
    }

    #[test]
    fn is_one() {
        assert!(I128::ONE.is_one());
        assert!(!I128::MAX.is_one());
        assert!(!I128::ZERO.is_one());
    }

    #[test]
    fn bits() {
        let u = I128::from(0b11101001010100101010101i128);
        assert_eq!(u.bits(), 23);
    }

    #[test]
    fn default() {
        assert_eq!(I128::default(), i128::default().into());
    }
}