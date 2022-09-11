use super::Float;
use crate::uint::BUint;
use crate::int::Bint;
use crate::As;

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub const RADIX: u32 = 2;

    pub const MANTISSA_DIGITS: u32 = MB as u32 + 1;

    pub const DIGITS: u32 = BUint::<W>::ONE.wrapping_shl(Self::MB).ilog10() as u32;

    pub const EPSILON: Self = {
        let u = Self::EXP_BIAS.to_bits() - MB.as_::<BUint<W>>();
        Self::from_bits(u << Self::MB)
    };

    pub const EXP_BIAS: Bint<W> = Bint::MAX.wrapping_shr(Self::MB + 1);

    pub const MIN: Self = {
        let mut e = BUint::MAX;
        e = e.wrapping_shr(Self::MB + 1);
        e = e.wrapping_shl(Self::MB + 1);
        let mut m = BUint::MAX;
        m = m.wrapping_shr(Self::EXPONENT_BITS + 1);
        Self::from_bits(e | m)
    };

    pub const MIN_POSITIVE: Self = {
        Self::from_bits(BUint::ONE.wrapping_shl(Self::MB))
    };
    pub const MAX_NEGATIVE: Self = -Self::MIN_POSITIVE;
    pub const MAX: Self = Self::MIN.abs();

    pub const MIN_EXP: Bint<W> = (-Self::EXP_BIAS).wrapping_add(Bint::ONE.wrapping_shl(1));
    pub const MAX_EXP: Bint<W> = Self::EXP_BIAS.wrapping_add(Bint::ONE);
    pub const MAX_UNBIASED_EXP: BUint<W> = Self::EXP_BIAS.to_bits() * BUint::TWO;
    pub const MIN_10_EXP: Self = todo!();
    pub const MAX_10_EXP: Self = todo!();

    pub const MAX_SUBNORMAL: Self = Self::from_bits(BUint::MAX.wrapping_shr(Self::EXPONENT_BITS + 1));
    pub const MIN_SUBNORMAL: Self = -Self::MAX_SUBNORMAL;
    pub const MIN_POSITIVE_SUBNORMAL: Self = Self::from_bits(BUint::ONE);
    pub const MAX_NEGATIVE_SUBNORMAL: Self = -Self::MIN_POSITIVE_SUBNORMAL;

    pub const NAN: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB - 1);
        Self::from_bits(u)
    };

    pub const QNAN: Self = {
        let bits = Self::NAN.to_bits();
        Self::from_bits(bits | (BUint::ONE << (Self::MB - 1)))
    };

    pub const NEG_NAN: Self = -Self::NAN;

    pub const NEG_QNAN: Self = -Self::QNAN;

    pub const INFINITY: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(1);
        u = u.wrapping_shr(1 + Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const NEG_INFINITY: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shr(Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const ZERO: Self = Self::from_bits(BUint::ZERO);

    pub const NEG_ZERO: Self = Self::from_words(*Bint::<W>::MIN.digits());

    pub const ONE: Self = {
        let mut u = BUint::MAX;
        u = u.wrapping_shl(2);
        u = u.wrapping_shr(2 + Self::MB);
        u = u.wrapping_shl(Self::MB);
        Self::from_bits(u)
    };

    pub const TWO: Self = {
        let (exp, _) = Self::ONE.exp_mant();
        Self::from_exp_mant(false, exp + BUint::ONE, BUint::ZERO)
    };

    pub const HALF: Self = {
        let (exp, _) = Self::ONE.exp_mant();
        Self::from_exp_mant(false, exp - BUint::ONE, BUint::ZERO)
    };

    pub const QUARTER: Self = {
        let (exp, _) = Self::ONE.exp_mant();
        Self::from_exp_mant(false, exp - BUint::TWO, BUint::ZERO)
    };
    
    pub const NEG_ONE: Self = Self::from_bits(Self::ONE.bits | Self::NEG_ZERO.bits);
}

#[cfg(test)]
mod tests {
    use crate::F64;
    type F32 = crate::Float::<4, 23>;

    macro_rules! test_constant {
        ($($constant: ident), *) => {
            $(
                assert_eq!(F64::$constant.to_bits(), f64::$constant.to_bits().into(), "constant `{}` not equal to the primitive equivalent", stringify!($constant));
            )*
        }
    }

    #[test]
    fn test_constants() {
        test_constant!(NAN, INFINITY, NEG_INFINITY, MAX, MIN, MIN_POSITIVE, EPSILON);

        assert_eq!(F64::ZERO.to_bits(), 0.0f64.to_bits().into());
        assert_eq!(F64::NEG_ZERO.to_bits(), (-0.0f64).to_bits().into());
        assert_eq!(F64::ONE.to_bits(), 1.0f64.to_bits().into());
        assert_eq!(F64::QUARTER.to_bits(), 0.25f64.to_bits().into());

        assert_eq!(F64::HALF.to_bits(), 0.5f64.to_bits().into());
        assert_eq!(F64::NEG_ONE.to_bits(), (-1.0f64).to_bits().into());

        assert_eq!(F64::MAX_NEGATIVE.to_bits(), (-f64::MIN_POSITIVE).to_bits().into());

        assert_eq!(F64::MIN_EXP, f64::MIN_EXP.into());
        assert_eq!(F64::MAX_EXP, f64::MAX_EXP.into());

        assert_eq!(F64::RADIX, f64::RADIX);
        assert_eq!(F64::MANTISSA_DIGITS, f64::MANTISSA_DIGITS);
        assert_eq!(F64::DIGITS, f64::DIGITS);

        assert_eq!(F64::BITS, 64);
        assert_eq!(F64::EXPONENT_BITS, 11);
        assert_eq!(F64::EXP_BIAS, 1023i32.into());
        assert_eq!(F32::MAX_UNBIASED_EXP, 254u32.into());
    }
}