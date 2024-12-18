#[cfg(test)]
impl super::F32 {
    #[inline]
    pub fn recip(self) -> Self {
        /*let (_, e, m) = self.into_biased_parts();
        let normalised = Self::from_raw_parts(self.is_sign_negative(), Self::EXP_BIAS.to_bits() - BUintD8::ONE, m);
        println!("norm: {}", normalised.to_f64());
        let r = normalised.recip_internal();
        let e = self.exponent() - Self::EXP_BIAS;
        //println!("{}", e);
        let e = (-e) + Self::EXP_BIAS;
        Self::from_raw_parts(self.is_sign_negative(), e.to_bits() - BUintD8::ONE, r.into_biased_parts().2)*/
        self.recip_internal()
    }

    #[inline]
    pub fn recip_internal(self) -> Self {
        if self.is_zero() {
            return Self::NAN;
        }
        // solve 1/b - x = 0 so 1/x - b = 0 =: f(x)
        // x_{n + 1} = x_n - f(x_n) / f'(x_n)
        // = x_n - (1/x_n - b) / (-1/x_n^2)
        // = x_n + (x_n - b x_n^2)
        // = x_n (2 - b x_n)
        let (_, e, m) = self.into_signed_parts();
        let inv_e = (-e + Self::EXP_BIAS).to_bits() - BUintD8::ONE;
        //println!("{}", e);
        let normalised = Self::from_raw_parts(false, Self::EXP_BIAS.to_bits() - BUintD8::ONE, m);
        if normalised == Self::ONE {
            return Self::from_raw_parts(self.is_sign_negative(), inv_e + 1, BUintD8::ZERO);
        }
        //println!("norm init: {:064b}", normalised.to_bits());
        let mut x_n = Self::from_bits(
            (normalised * Self::HALF).to_bits() ^ (BUintD8::MAX >> (Self::BITS - Self::MB)),
        );

        let mut m_n = x_n.into_biased_parts().2 << 1;

        /*
        0.5 <= x_n < 1
        1 <= normalised < 2
        so 0.5 <= x_n * normalised < 2
        1 <= x_n * 2 < 2
        */

        //println!("x_n: {}", x_n.to_f32());
        let mut iters = 0;
        loop {
            let a1 = x_n * Self::TWO;
            let a2 = x_n * normalised * x_n;
            let x_n_1 = a1 - a2;
            assert!(a1.to_f32() >= 1.0 && a1.to_f32() <= 2.0);
            assert!(a2.to_f32() >= 0.5 && a2.to_f32() <= 1.0);

            let ma1 = m_n << 1;
            let xnf = x_n_1.to_f32();
            assert!(
                0.5 <= xnf && xnf < 1.0,
                "{}, norm: {}",
                xnf,
                normalised.to_f32()
            );
            // x_n * (2 - norm * x_n)
            if x_n_1 == x_n || iters == 100 {
                //println!("done: new: {}, old: {}", x_n_1.to_f64(), x_n.to_f64());
                //println!("norm: {:064b}", x_n_1.to_bits());
                let mut m = x_n_1.into_biased_parts().2;
                if m.bit(Self::MB) {
                    m ^= BUintD8::ONE << Self::MB;
                }
                let unnormalised = Self::from_raw_parts(self.is_sign_negative(), inv_e, m);
                // println!("iters: {}", iters);
                return unnormalised;
            }
            x_n = x_n_1;
            iters += 1;
        }
    }

    #[cfg(test)]
    pub(crate) fn to_f64(self) -> f64 {
        f64::from_bits(self.to_bits().as_())
    }

    #[cfg(test)]
    pub(crate) fn to_f32(self) -> f32 {
        f32::from_bits(self.to_bits().as_())
    }

    #[inline]
    pub fn recip2(self) -> Self
    where
        [(); W * 2]:,
    {
        Self::ONE / self
    }
}

#[cfg(test)]
mod tests {


    /*#[test]
    fn fmod() {
        use super::super::F64;
        let f1 = 0.0;
        let f2 = f64::INFINITY;
        //println!("{:064b}", ((-0.0f64).div_euclid(f2)).to_bits());
        let a = (F64::from(f1) * (F64::from(f2))).to_bits();
        let b = (f1 * (f2)).to_bits();
        /*println!("{:064b}", a);
        println!("{:064b}", b);*/
        assert!(a == b.into());
    }

    quickcheck::quickcheck! {
        fn qc_recip(u: u32) -> quickcheck::TestResult {
            let f = f32::from_bits(u);
            if !f.is_finite() || f >= 2.0 || f <= 1.0 {
                return quickcheck::TestResult::discard();
            }

            let b1 = f.recip().to_bits();
            let b2 = super::super::F32::from(f).recip().to_f32().to_bits();
            return quickcheck::TestResult::from_bool(b1 == b2 || b1 + 1 == b2);
        }
    }

    #[test]
    fn recip2() {
        assert!((0.0 as ftest).to_bits().count_zeros() == 32);
        use super::super::F32;

        let f1 = 1.7517333f32; //f32::from_bits(0b0_01111110_01001000000000000000000u32);
        println!("{}", f1);

        println!("{:032b}", f1.recip().to_bits());
        println!("{:032b}", F32::from(f1).recip_internal().to_f32().to_bits());
        panic!("")
    }

    test_bignum! {
        function: <ftest>::recip(f: ftest),
        skip: !f.is_finite() || f == 0.0 || f >= 2.0 || f <= 1.0
    }

    #[test]
    fn recip_u8() {
        let mut g = true;
        let mut close = true;
        for i in 1..=u8::MAX {
            let u = 0b0_01111110_00000000000000000000000u32 | ((i as u32) << 15);
            let f = f32::from_bits(u);

            let b1 = f.recip().to_bits();
            let b2 = super::super::F32::from(f)
                .recip_internal()
                .to_f32()
                .to_bits();
            let eq = b1 == b2;
            if !eq {
                println!("{:08b}", i);
                if b2 - b1 != 1 {
                    close = false;
                }
            }
            if b1 > b2 {
                if b1 > b2 {
                    g = false;
                }
            }
        }
        println!("all greater: {}", g);
        println!("all close: {}", close);
        panic!("")
    }*/
}