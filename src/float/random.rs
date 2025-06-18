use rand::distr::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distr::{Distribution, Open01, OpenClosed01, StandardUniform};
use rand::{Fill, Rng, SeedableRng};

use super::{Float, FloatExponent};
use crate::Uint;

fn seeded_rngs<R: SeedableRng + Clone>(seed: u64) -> (R, R) {
    let rng = R::seed_from_u64(seed);
    let rng2 = rng.clone(); // need to clone `rng` to produce the same results before it is used as `gen` mutates it
    (rng, rng2)
}

impl<const W: usize, const MB: usize> Distribution<Float<W, MB>> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Float<W, MB> {
        let random_bits: Uint<W> = rng.random();
        let mantissa = random_bits.shr(Float::<W, MB>::BITS - Float::<W, MB>::MB - 1);
        if mantissa.is_zero() {
            return Float::ZERO;
        }
        if mantissa.is_one() {
            return Float::HALF_EPSILON;
        }
        let mantissa_bits = mantissa.bits();
        let abs_exponent = Float::<W, MB>::MB + 2 - mantissa_bits; // has to be in this order to prevent overflow
        Float::from_signed_parts(
            false,
            -(abs_exponent as FloatExponent),
            mantissa.shl(abs_exponent - 1),
        )
    }
}

impl<const W: usize, const MB: usize> Distribution<Float<W, MB>> for OpenClosed01 {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Float<W, MB> {
        let random_bits: Uint<W> = rng.random();
        let mantissa = random_bits.shr(Float::<W, MB>::BITS - Float::<W, MB>::MB - 1);
        let mantissa = mantissa.add(Uint::ONE); // add one so mantissa is never zero
        if mantissa.is_zero() {
            return Float::HALF_EPSILON;
        }
        let mantissa_bits = mantissa.bits();
        let abs_exponent = Float::<W, MB>::MB + 2 - mantissa_bits; // has to be in this order to prevent overflow
        Float::from_signed_parts(
            false,
            -(abs_exponent as FloatExponent),
            mantissa.shl(abs_exponent - 1),
        )
    }
}

impl<const W: usize, const MB: usize> Distribution<Float<W, MB>> for Open01 {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Float<W, MB> {
        let random_bits: Uint<W> = rng.random();
        let mantissa = random_bits.shr(Float::<W, MB>::BITS - Float::<W, MB>::MB);
        if mantissa.is_zero() {
            return Float::HALF_EPSILON;
        }
        let mantissa_bits = mantissa.bits();
        let abs_exponent = Float::<W, MB>::MB + 1 - mantissa_bits; // has to be in this order to prevent overflow
        let exponent = -(abs_exponent as FloatExponent);
        let mut mantissa = mantissa.shl(1).bitor(Uint::ONE); // = 2*mantissa + 1
        mantissa.set_bit(0, true);
        mantissa = mantissa.shl(Float::<W, MB>::MB - mantissa_bits);
        let a = Float::from_signed_parts(false, exponent, mantissa);
        a
    }
}

#[cfg(test)]
mod tests {
    use crate::{float::F32, test::convert};
    use rand::distr::OpenClosed01;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use rand::{distr::Open01, rngs::SmallRng};

    use super::seeded_rngs;

    #[test]
    fn test_random() {
        let mut r = StdRng::seed_from_u64(0);
        let seed = r.random();
        let (mut r1, mut r2) = seeded_rngs::<SmallRng>(seed);
        let big: F32 = r1.random();
        let prim: f32 = r2.random();

        assert!(convert::test_eq(big, prim));

        let big: F32 = r1.sample(OpenClosed01);
        let prim: f32 = r2.sample(OpenClosed01);

        assert!(convert::test_eq(big, prim));
    }
}
