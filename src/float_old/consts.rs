use crate::ExpType;
use super::Float;
use crate::uint::BUint;
use crate::int::Bint;

impl<const W: usize, const MANTISSA_BITS: usize> Float<W, MANTISSA_BITS> {

    pub const RADIX: u32 = 2;

    pub const MANTISSA_DIGITS: u32 = MANTISSA_BITS as u32 + 1;

    pub const DIGITS: u32 = BUint::<W>::ONE.wrapping_shl(MANTISSA_BITS as ExpType).log10() as u32;

    pub const EPSILON: Self = todo!();

    pub const EXP_BIAS: Bint<W> = Bint::MAX.wrapping_shr(MANTISSA_BITS + 1);

    pub const MIN: Self = {
        let mut e = BUint::MAX;
        e = e.wrapping_shr(MANTISSA_BITS as ExpType + 1);
        e = e.wrapping_shl(MANTISSA_BITS as ExpType + 1);
        let mut m = BUint::MAX;
        m = m.wrapping_shr(Self::EXPONENT_BITS as ExpType + 1);
        Self {
            uint: e | m,
        }
    };

    pub const MIN_POSITIVE: Self = {
        Self {
            uint: BUint::ONE.wrapping_shl(MANTISSA_BITS as ExpType),
        }
    };
    pub const MAX_NEGATIVE: Self = Self::MIN_POSITIVE.neg();
    pub const MAX: Self = Self::MIN.abs();

    pub const MIN_EXP: Bint<W> = Self::EXP_BIAS.neg().wrapping_add(Bint::ONE.wrapping_shl(1));
    pub const MAX_EXP: Bint<W> = Self::EXP_BIAS.wrapping_add(Bint::ONE);
    pub const MAX_UNBIASED_EXP: BUint<W> = Self::EXP_BIAS.to_bits() * BUint::TWO;
    pub const MIN_10_EXP: Self = todo!();
    pub const MAX_10_EXP: Self = todo!();

    pub const MAX_SUBNORMAL: Self = Self {
        uint: BUint::MAX.wrapping_shr(Self::EXPONENT_BITS as ExpType + 1),
    };
    pub const MIN_SUBNORMAL: Self = Self::MAX_SUBNORMAL.neg();
    pub const MIN_POSITIVE_SUBNORMAL: Self = Self {
        uint: BUint::ONE,
    };
    pub const MAX_NEGATIVE_SUBNORMAL: Self = Self::MIN_POSITIVE_SUBNORMAL.neg();

    pub const NAN: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType - 1);
        Self {
            uint: u,
        }
    };

    pub const NEG_NAN: Self = Self::NAN.neg();

    pub const INFINITY: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(1 + MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType);
        Self {
            uint: u,
        }
    };

    pub const NEG_INFINITY: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shr(MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType);
        Self {
            uint: u,
        }
    };

    pub const ZERO: Self = Self::from_bits(BUint::ZERO);

    pub const NEG_ZERO: Self = Self::from_words(*Bint::<W>::MIN.digits());

    pub const ONE: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(2);
        u = u.wrapping_shr(2 + MANTISSA_BITS as ExpType);
        u = u.wrapping_shl(MANTISSA_BITS as ExpType);
        Self::from_bits(u)
    };
    
    pub const NEG_ONE: Self = Self::from_bits(Self::ONE.uint | Self::NEG_ZERO.uint);
}