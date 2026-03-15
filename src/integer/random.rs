/*
Most of the code in this file is adapted from code from the Rust `rand` library, https://docs.rs/rand/latest/rand/, modified under the MIT license. The changes are released under either the MIT license or the Apache License 2.0, as described in the README. See LICENSE-MIT or LICENSE-APACHE at the project root.
The original license file for `rand` can be found in this project's root at licenses/LICENSE-rand.

The appropriate copyright notices for this are given below:
Copyright 2018 Developers of the Rand project.
Copyright 2013-2017 The Rust Project Developers.
Copyright 2018-2020 Developers of the Rand project.
Copyright 2017 The Rust Project Developers.
*/

use crate::Integer;
use crate::Uint;
use rand::distr::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distr::{Distribution, StandardUniform};
use rand::{Fill, Rng, RngExt};
use crate::random::UniformInt;
use crate::cast::As;

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Distribution<Integer<S, N, B, OM>> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Integer<S, N, B, OM> {
        if Integer::<S, N, B, OM>::BITS <= 32 { // because rand uses u32 to generate <= 32-bit integers
            return rng.next_u32().as_::<Uint<N, B, OM>>().force_sign();
        }
        let mut out = Integer::ZERO;
        rng.fill_bytes(out.as_bytes_mut());
        out
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Fill for Integer<S, N, B, OM> {
    #[inline]
    fn fill_slice<R: Rng + ?Sized>(this: &mut [Self], rng: &mut R) {
        if this.len() > 0 {
            rng.fill_bytes(unsafe {
                core::slice::from_raw_parts_mut(
                    this.as_mut_ptr() as *mut u8,
                    this.len() * core::mem::size_of::<Integer<S, N, B, OM>>(),
                )
            });
        }
    }
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> SampleUniform for Integer<S, N, B, OM> {
    type Sampler = UniformInt<Self>;
}

#[inline]
const fn widening_mul_u32(a: u32, b: u32) -> (u32, u32) {
    let m = a as u64 * b as u64;
    (m as u32, (m >> 32) as u32)
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> UniformSampler for UniformInt<Integer<S, N, B, OM>> {
    type X = Integer<S, N, B, OM>;

    #[inline]
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if high <= low {
            return Err(rand::distr::uniform::Error::EmptyRange);
        }
        UniformSampler::new_inclusive(low, high - Integer::ONE)
    }

    #[inline]
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, rand::distr::uniform::Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if high < low {
            return Err(rand::distr::uniform::Error::EmptyRange);
        }

        let range = high.wrapping_sub(low).wrapping_add(Integer::ONE).force_sign::<false>();
        let thresh = if !range.is_zero() {
            if Integer::<S, N, B, OM>::BITS <= 32 { // because rand uses u32 to generate <= 32-bit integers
                use crate::cast::As;

                let range = range.as_::<u32>();
                let t = range.wrapping_neg() % range;
                t.as_::<Uint<N, B, OM>>().force_sign()
            } else {
                (range.wrapping_neg() % range).force_sign()
            }
        } else {
            Integer::ZERO
        };

        Ok(UniformInt {
            low,
            range: range.force_sign(),
            thresh: thresh,
        })
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        if Integer::<S, N, B, OM>::BITS <= 32 { // because rand uses u32 to generate <= 32-bit integers
            use crate::cast::As;

            let range = self.range.force_sign::<false>().as_::<u32>();
            if range == 0 {
                rng.random()
            } else {
                let thresh = self.thresh.force_sign::<false>().as_::<u32>();

                loop {
                    let v: u32 = rng.random();
                    let (lo, hi) = widening_mul_u32(v, range);
                    if lo >= thresh {
                        return self.low.wrapping_add(hi.as_());
                    }
                }
            }
        }
        let range = self.range.force_sign::<false>();
        if range.is_zero() {
            rng.random()
        } else {
            let thresh = self.thresh.force_sign::<false>();

            loop {
                let v: Uint<N, B, OM> = rng.random();
                let (lo, hi) = v.widening_mul(range);
                if lo >= thresh {
                    return self.low.wrapping_add(hi.force_sign());
                }
            }
        }
    }

    #[inline]
    fn sample_single<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Result<Self::X, rand::distr::uniform::Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if high <= low {
            return Err(rand::distr::uniform::Error::EmptyRange);
        }
        Self::sample_single_inclusive(low, high - Integer::ONE, rng)
    }

    #[inline]
    fn sample_single_inclusive<R: Rng + ?Sized, B1, B2>(low_b: B1, high_b: B2, rng: &mut R) -> Result<Self::X, rand::distr::uniform::Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if high < low {
            return Err(rand::distr::uniform::Error::EmptyRange);
        }
        let range = high.wrapping_sub(low).wrapping_add(Integer::ONE).force_sign::<false>();

        if Integer::<S, N, B, OM>::BITS <= 32 { // because rand uses u32 to generate <= 32-bit integers
            use crate::cast::As;

            let range = range.as_::<u32>();
            if range == 0 {
                return Ok(rng.random());
            }
            let (mut lo, mut result) = widening_mul_u32(rng.random::<u32>(), range);
            while lo > range.wrapping_neg() {
                let (new_lo, new_hi) = widening_mul_u32(rng.random::<u32>(), range);
                match lo.checked_add(new_hi) {
                    Some(x) => {
                        if x == u32::MAX {
                            lo = new_lo;
                        } else {
                            break;
                        }
                    },
                    None => {
                        result += 1;
                        break;
                    }
                }
            }

            return Ok(low.wrapping_add(result.as_::<Uint<N, B, OM>>().force_sign()));
        }
        if range.is_zero() {
            return Ok(rng.random());
        }

        let (mut lo, mut result) = rng.random::<Uint<N, B, OM>>().widening_mul(range);
        while lo > range.wrapping_neg() {
            let (new_lo, new_hi) = rng.random::<Uint<N, B, OM>>().widening_mul(range);
            match lo.checked_add(new_hi) {
                Some(x) => {
                    if x == Uint::<N, B, OM>::MAX {
                        lo = new_lo;
                    } else {
                        break;
                    }
                },
                None => {
                    result += Uint::<N, B, OM>::ONE;
                    break;
                }
            }
        }

        Ok(low.wrapping_add(result.force_sign()))
    }
}

#[cfg(test)]
crate::test::test_all! {
    testing integers;
    
    use rand::{Fill, SeedableRng};
    use rand::rngs::SmallRng; // use SmallRng as doesn't require an extra crate feature

    fn seeded_rngs<R: SeedableRng + Clone>(seed: u64) -> (R, R) {
        let rng = R::seed_from_u64(seed);
        let rng2 = rng.clone(); // need to clone `rng` to produce the same results before it is used as `gen` mutates it
        (rng, rng2)
    }

    quickcheck::quickcheck! {
        #[allow(non_snake_case)]
        fn quickcheck_SmallRng_gen_stest(seed: u64) -> bool {
            use crate::test::convert;

            let (mut rng, mut rng2) = seeded_rngs::<SmallRng>(seed);

            let big = rng.random::<STEST>();
            let primitive = rng2.random::<stest>();

            convert::test_eq(big, primitive)
        }

        #[allow(non_snake_case)]
        fn quickcheck_SmallRng_fill_stest_slice(seed: u64) -> bool {
            use crate::test::convert;

            const SLICE_LENGTH: usize = 20;

            let (mut rng, mut rng2) = seeded_rngs::<SmallRng>(seed);

            let mut big_array = [STEST::MIN; SLICE_LENGTH];
            let mut primitive_array = [stest::MIN; SLICE_LENGTH];

            Fill::fill_slice(&mut big_array, &mut rng);

            Fill::fill_slice(&mut primitive_array, &mut rng2);

            big_array
                .into_iter()
                .zip(primitive_array.into_iter())
                .all(|(big, primitive)| convert::test_eq(big, primitive))
        }

        #[allow(non_snake_case)]
        fn quickcheck_SmallRng_gen_range_stest(seed: u64, min: stest, max: stest) -> quickcheck::TestResult {
            if min >= max {
                return quickcheck::TestResult::discard();
            }
            use crate::test::convert;
            use crate::cast::CastFrom;

            let mut result = true;
            
            let (mut rng, mut rng2) = seeded_rngs::<SmallRng>(seed);

            let min_big = STEST::cast_from(min);
            let max_big = STEST::cast_from(max);

            let big = rng.random_range(min_big..max_big);
            let primitive = rng2.random_range(min..max);  // calls sample_single from UniformSampler


            result &= convert::test_eq(big, primitive);

            let big = rng.random_range(min_big..=max_big);
            let primitive = rng2.random_range(min..=max); // calls sample_single_inclusive from UniformSampler

            result &= convert::test_eq(big, primitive);

            use rand::distr::Uniform;

            let big = Uniform::new(min_big, max_big)
                .unwrap()
                .sample(&mut rng); // calls sample from UniformSampler

            let primitive = Uniform::new(min, max)
                .unwrap()
                .sample(&mut rng2); // calls sample from UniformSampler

            result &= convert::test_eq(big, primitive);

            quickcheck::TestResult::from_bool(result)
        }
    }
}
