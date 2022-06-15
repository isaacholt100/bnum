use rand::distributions::uniform::{SampleUniform, UniformSampler, SampleBorrow};
use rand::distributions::{Distribution, Standard};
use rand::{Rng, Fill, Error};
use crate::{BUint, BInt};

// credit all below code to rand source code

impl<const N: usize> Distribution<BUint<N>> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BUint<N> {
        BUint::from_digits(rng.gen())
    }
}

impl<const N: usize> Distribution<BInt<N>> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BInt<N> {
        BInt::from_bits(rng.gen())
    }
}

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RandomUniformInt<X> {
    low: X,
    range: X,
    z: X,
}

pub struct Slice<T>(pub [T]);

macro_rules! fill_impl {
    ($ty: ty) => {
        impl<const N: usize> Fill for Slice<$ty> {
            fn try_fill<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), Error> {
                if self.0.len() > 0 {
                    rng.try_fill_bytes(unsafe {
                        core::slice::from_raw_parts_mut(self.0.as_mut_ptr()
                            as *mut u8,
                            self.0.len() * core::mem::size_of::<$ty>()
                        )
                    })?;
                    for x in &mut self.0 {
                        *x = x.to_le();
                    }
                }
                Ok(())
            }
        }
    };
}

fill_impl!(BUint<N>);
fill_impl!(BInt<N>);

macro_rules! uniform_int_impl {
    ($ty:ty, $u_large:ty $(, $as_unsigned: ident, $as_signed: ident)?) => {
        impl<const N: usize> SampleUniform for $ty {
            type Sampler = RandomUniformInt<$ty>;
        }

        impl<const N: usize> UniformSampler for RandomUniformInt<$ty> {
            type X = $ty;

            #[inline]
            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(low < high, "Uniform::new called with `low >= high`");
                UniformSampler::new_inclusive(low, high - <$ty>::ONE)
            }

            #[inline]
            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(
                    low <= high,
                    "Uniform::new_inclusive called with `low > high`"
                );

                let range = high.wrapping_sub(low).wrapping_add(<$ty>::ONE)$(.$as_unsigned())?;
                let ints_to_reject = if !range.is_zero() {
                    (<$u_large>::MAX - range + 1) % range
                } else {
                    <$u_large>::ZERO
                };

                RandomUniformInt {
                    low,
                    range: $(BInt::$as_signed)?(range),
                    z: $(BInt::$as_signed)?(ints_to_reject),
                }
            }

            #[inline]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                let range = self.range$(.$as_unsigned())?;
                if !range.is_zero() {
                    let zone = <$u_large>::MAX - (self.z)$(.$as_unsigned())?;
                    loop {
                        let v: $u_large = rng.gen();
                        let (lo, hi) = v.widening_mul(range);
                        if lo <= zone {
                            return self.low.wrapping_add($(BInt::$as_signed)?(hi));
                        }
                    }
                } else {
                    rng.gen()
                }
            }

            #[inline]
            fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(low < high, "UniformSampler::sample_single: low >= high");
                Self::sample_single_inclusive(low, high - <$ty>::ONE, rng)
            }

            #[inline]
            fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Self::X
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = *low_b.borrow();
                let high = *high_b.borrow();
                assert!(low <= high, "UniformSampler::sample_single_inclusive: low > high");
                let range = high.wrapping_sub(low).wrapping_add(<$ty>::ONE)$(.$as_unsigned())?;
                if range.is_zero() {
                    return rng.gen();
                }

                let zone = if <$u_large>::MAX.bits() <= 16 {
                    let ints_to_reject = (<$u_large>::MAX - range + 1) % range;
                    <$u_large>::MAX - ints_to_reject
                } else {
                    (range << range.leading_zeros()).wrapping_sub(<$u_large>::ONE)
                };

                loop {
                    let v: $u_large = rng.gen();
                    let (lo, hi) = v.widening_mul(range);
                    if lo <= zone {
                        return low.wrapping_add($(BInt::$as_signed)?(hi));
                    }
                }
            }
        }
    };
}

uniform_int_impl!(BUint<N>, BUint<N>);

uniform_int_impl!(BInt<N>, BUint<N>, to_bits, from_bits);