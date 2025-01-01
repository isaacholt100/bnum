

    /*pub fn remquof(mut self, mut y: Self) -> /*(Self, BIntD8<W>)*/(Self, Self) {
        handle_nan!(self; self);
        handle_nan!(y; y);
        if self.is_infinite() || y.is_infinite() {
            return (Self::NAN, Self::NAN);
        }

        if y.is_zero() {
            return (Self::QNAN, Self::QNAN);
        }
        if self.is_zero() {
            return (self, self);
        }
        let ux = self.to_bits();
        let mut uy = y.to_bits();
        let mut ex = self.exponent();
        let mut ey = y.exponent();
        let sx = self.is_sign_negative();
        let sy = y.is_sign_negative();
        let mut uxi = ux;

        /* normalize x and y */
        let mut i;
        if ex.is_zero() {
            i = uxi << (Self::BITS - Self::MB);
            while !BIntD8::from_bits(i).is_negative() {
                ex -= BIntD8::ONE;
                i <<= 1u8;
            }
            uxi <<= -ex + BIntD8::ONE;
        } else {
            uxi &= BUintD8::MAX >> (Self::BITS - Self::MB);
            uxi |= BUintD8::ONE << Self::MB;
        }
        if ey.is_zero() {
            i = uy << (Self::BITS - Self::MB);
            while !BIntD8::from_bits(i).is_negative() {
                ey -= BIntD8::ONE;
                i <<= 1u8;
            }
            uy <<= -ey + BIntD8::ONE;
        } else {
            uy &= BUintD8::MAX >> (Self::BITS - Self::MB);
            uy |= BUintD8::ONE << Self::MB;
        }

        let mut q = BUintD8::<W>::ZERO;
        if ex + BIntD8::ONE != ey {
            if ex < ey {
                return (self, 0);
            }
            /* x mod y */
            while ex > ey {
                i = uxi.wrapping_sub(uy);
                if !BIntD8::from_bits(i).is_negative() {
                    uxi = i;
                    q += BUintD8::ONE;
                }
                uxi <<= 1u8;
                q <<= 1u8;
                ex -= BIntD8::ONE;
            }
            i = uxi.wrapping_sub(uy);
            if !BIntD8::from_bits(i).is_negative() {
                uxi = i;
                q += BUintD8::ONE;
            }
            if uxi.is_zero() {
                //ex = BIntD8::TWO - BIntD8::from(Self::BITS);
                ex = BIntD8::from(-60i8);
            } else {
                while (uxi >> Self::MB).is_zero() {
                    uxi <<= 1u8;
                    ex -= BIntD8::ONE;
                }
            }
        }

        /* scale result and decide between |x| and |x|-|y| */
        if ex.is_positive() {
            uxi -= BUintD8::ONE << Self::MB;
            uxi |= ex.to_bits() << Self::MB;
        } else {
            uxi >>= -ex + BIntD8::ONE;
        }
        self = Self::from_bits(uxi);
        if sy {
            y = -y;
        }
        if ex == ey || (ex + BIntD8::ONE == ey && (Self::TWO * self > y || (Self::TWO * self == y && !(q % BUintD8::TWO).is_zero()))) {
            self = self - y;
            q += BUintD8::ONE;
        }
        q &= BUintD8::MAX >> 1u8;
        let quo = if sx ^ sy { -BIntD8::from_bits(q) } else { BIntD8::from_bits(q) };
        if sx {
            (-self, quo)
        } else {
            (self, quo)
        }
    }*/