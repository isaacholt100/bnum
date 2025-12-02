use super::Float;
use crate::Exponent;
use crate::Uint;
use crate::float::UnsignedFloatExponent;

impl<const W: usize, const MB: usize> Float<W, MB> {
    #[inline]
    pub(super) fn rem(self, y: Self) -> Self {
        handle_nan!(self; self);
        handle_nan!(y; y);

        if y.is_zero() {
            return Self::NAN;
        }
        if self.is_zero() {
            return self;
        }
        if self.is_infinite() {
            return Self::NAN;
        }
        if y.is_infinite() {
            return self;
        }

        let mut uxi = self.to_bits();
        let mut uyi = y.to_bits();
        let mut ex = self.signed_biased_exponent();
        let mut ey = y.signed_biased_exponent();
        let mut i;

        if uxi << 1 as Exponent <= uyi << 1 as Exponent {
            if uxi << 1 as Exponent == uyi << 1 as Exponent {
                return if self.is_sign_negative() {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }

            return self;
        }

        /* normalize x and y */
        if ex == 0 {
            i = uxi << (Self::BITS - Self::MB);
            while !i.cast_signed().is_negative() {
                ex -= 1;
                i <<= 1 as Exponent;
            }

            uxi <<= -ex + 1;
        } else {
            uxi &= Uint::MAX >> (Self::BITS - Self::MB);
            uxi |= Uint::ONE << Self::MB;
        }

        if ey == 0 {
            i = uyi << (Self::BITS - Self::MB);
            while !i.cast_signed().is_negative() {
                ey -= 1;
                i <<= 1 as Exponent;
            }

            uyi <<= -ey + 1;
        } else {
            uyi &= Uint::MAX >> (Self::BITS - Self::MB);
            uyi |= Uint::ONE << Self::MB;
        }
        /* x mod y */
        while ex > ey {
            i = uxi.wrapping_sub(uyi);
            if !i.cast_signed().is_negative() {
                if i.is_zero() {
                    return if self.is_sign_negative() {
                        Self::NEG_ZERO
                    } else {
                        Self::ZERO
                    };
                }
                uxi = i;
            }
            uxi <<= 1 as Exponent;

            ex -= 1;
        }

        i = uxi.wrapping_sub(uyi);
        if !i.cast_signed().is_negative() {
            if i.is_zero() {
                return if self.is_sign_negative() {
                    Self::NEG_ZERO
                } else {
                    Self::ZERO
                };
            }
            uxi = i;
        }

        while (uxi >> Self::MB).is_zero() {
            uxi <<= 1 as Exponent;
            ex -= 1;
        }

        /* scale result up */
        if ex.is_positive() {
            uxi -= Uint::ONE << Self::MB;
            uxi |= Uint::cast_from_unsigned_float_exponent(ex as UnsignedFloatExponent) << Self::MB;
        } else {
            uxi >>= -ex + 1;
        }

        let f = Self::from_bits(uxi);
        if self.is_sign_negative() { -f } else { f }
    }
}
