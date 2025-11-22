/*
Most of the code in this file is adapted from code from the Rust `rand` library, https://docs.rs/rand/latest/rand/, modified under the MIT license. The changes are released under either the MIT license or the Apache License 2.0, as described in the README. See LICENSE-MIT or LICENSE-APACHE at the project root.
The original license file for `rand` can be found in this project's root at licenses/LICENSE-rand.

The appropriate copyright notices for this are given below:
Copyright 2018 Developers of the Rand project.
Copyright 2013-2017 The Rust Project Developers.
Copyright 2018-2020 Developers of the Rand project.
Copyright 2017 The Rust Project Developers.
*/

//! Utilities for generating random bnum integers.
//!
//! The `rand` feature must be enabled to use items from this module.

use core::ops::{Deref, DerefMut};

/// Wrapper type designed to be filled with random big integers.
///
/// This type exists because [`rand::Fill`](https://docs.rs/rand/latest/rand/trait.Fill.html) can't be implemented for `[Integer<S, N, B, OM>]` due to Rust's orphan rules. Instead, it is implemented for `Slice<Integer<S, N, B, OM>>`. The underlying slice can then be accessed by calling [`AsRef::as_ref`](https://doc.rust-lang.org/core/convert/trait.AsRef.html#tymethod.as_ref) or [`AsMut::as_mut`](https://doc.rust-lang.org/core/convert/trait.AsMut.html#tymethod.as_mut) on the wrapper, or deferencing it. An alternative way of filling a slice with random big integers is using the [`fill_slice`] method in this crate's [`random`](crate::random) module.
#[repr(transparent)]
pub struct Slice<T>(pub [T]);

impl<T> AsRef<[T]> for Slice<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> AsMut<[T]> for Slice<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T> Deref for Slice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for Slice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

/// Fills a slice with random big integers.
///
/// An alternative way of filling a slice with random big integers is using the [`rand::Fill`](https://docs.rs/rand/latest/rand/trait.Fill.html) trait's method [`try_fill`](https://docs.rs/rand/latest/rand/trait.Fill.html#tymethod.try_fill) on the [`Slice`] struct in this crate's [`random`](crate::random) module.
///
/// # Examples
///
/// ```
/// // Fill a `Vec` of length 10 with random `I256`s
///
/// use bnum::types::I256;
/// use bnum::random;
/// use rand::rngs::StdRng;
/// use rand::SeedableRng;
///
/// let mut v = vec![I256::ZERO; 10];
/// let mut rng = StdRng::seed_from_u64(0);
///
/// random::fill_slice(&mut v, &mut rng);
/// // each initial `I256::ZERO` is replaced with a random `I256`
///
/// println!("{:?}", v);
/// ```
pub fn fill_slice<T, R: Rng + ?Sized>(slice: &mut [T], rng: &mut R)
where
    Slice<T>: Fill,
{
    let slice = unsafe { &mut *(slice as *mut _ as *mut Slice<T>) };
    Fill::fill(slice, rng)
}

use crate::Integer;

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Fill for crate::random::Slice<Integer<S, N, B, OM>> {
    #[inline(never)]
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        if self.0.len() > 0 {
            rng.fill_bytes(unsafe {
                core::slice::from_raw_parts_mut(
                    self.0.as_mut_ptr() as *mut u8,
                    self.0.len() * core::mem::size_of::<Integer<S, N, B, OM>>(),
                )
            });
        }
    }
}

#[inline]
const fn widening_mul_u32(a: u32, b: u32) -> (u32, u32) {
    let m = a as u64 * b as u64;
    (m as u32, (m >> 32) as u32)
}

impl<const S: bool, const N: usize, const B: usize, const OM: u8> SampleUniform for Integer<S, N, B, OM> {
    type Sampler = UniformInt<Self>;
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
                t.as_()
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

            return Ok(low.wrapping_add(result.as_()));
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

/// Used for generating random big integers in a given range.
///
/// Implements the [`UniformSampler`](https://docs.rs/rand/latest/rand/distributions/uniform/trait.UniformSampler.html) trait from the [`rand`](https://docs.rs/rand/latest/rand/) crate. This struct should not be used directly; instead use the [`Uniform`](https://docs.rs/rand/latest/rand/distributions/struct.Uniform.html) struct from the [`rand`](https://docs.rs/rand/latest/rand/) crate.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniformInt<X> {
    low: X,
    range: X,
    thresh: X,
}

use crate::Uint;

use rand::distr::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distr::{Distribution, StandardUniform};
use rand::{Fill, Rng};

impl<const S: bool, const N: usize, const B: usize, const OM: u8> Distribution<Integer<S, N, B, OM>> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Integer<S, N, B, OM> {
        let mut out = Integer::ZERO;
        rng.fill_bytes(out.as_bytes_mut());
        out
    }
}

#[cfg(test)]
macro_rules! test_random {
    ($int: ty; $($Rng: ty), *) => {
        paste::paste! {
            $(
                quickcheck::quickcheck! {
                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $Rng _gen_ $int>](seed: u64) -> bool {
                        use crate::test::convert;
                        use rand::Rng;
                        use rand::rngs::$Rng;

                        let (mut rng, mut rng2) = seeded_rngs::<$Rng>(seed);

                        let big = rng.random::<[<$int:upper>]>();
                        let primitive = rng2.random::<$int>();

                        convert::test_eq(big, primitive)
                    }

                    #[allow(non_snake_case)]
                    fn [<quickcheck_ $Rng _fill_ $int _slice>](seed: u64) -> bool {
                        use crate::test::convert;
                        use rand::Fill;
                        use rand::rngs::$Rng;

                        const SLICE_LENGTH: usize = 20;

                        let (mut rng, mut rng2) = seeded_rngs::<$Rng>(seed);

                        let mut big_array = [<[<$int:upper>]>::MIN; SLICE_LENGTH];
                        let mut primitive_array = [<$int>::MIN; SLICE_LENGTH];

                        crate::random::fill_slice(&mut big_array, &mut rng);

                        primitive_array.fill(&mut rng2);

                        big_array
                            .into_iter()
                            .zip(primitive_array.into_iter())
                            .all(|(big, primitive)| convert::test_eq(big, primitive))
                    }

                    #[allow(non_snake_case)]
            		fn [<quickcheck_ $Rng _gen_range_ $int>](seed: u64, min: $int, max: $int) -> quickcheck::TestResult {
						if min >= max {
							return quickcheck::TestResult::discard();
						}
						use crate::test::convert;
                        use crate::cast::CastFrom;
						use rand::Rng;
						use rand::rngs::$Rng;

						let (mut rng, mut rng2) = seeded_rngs::<$Rng>(seed);

						let min_big = [<$int:upper>]::cast_from(min);
						let max_big = [<$int:upper>]::cast_from(max);

						let big = rng.random_range(min_big..max_big);
						let primitive = rng2.random_range(min..max);

                        let mut result = convert::test_eq(big, primitive);

						let big = rng.random_range(min_big..=max_big);
						let primitive = rng2.random_range(min..=max);

                        result &= convert::test_eq(big, primitive);

                        use rand::distr::Uniform;

                        let big = Uniform::new(min_big, max_big)
                            .unwrap()
                            .sample(&mut rng);

                        let primitive = Uniform::new(min, max)
                            .unwrap()
                            .sample(&mut rng2);

                        result &= convert::test_eq(big, primitive);

						quickcheck::TestResult::from_bool(result)
					}
				}
			)*
		}
	};
}

#[cfg(test)]
crate::test::test_all_widths! {
    use rand::SeedableRng;

    fn seeded_rngs<R: SeedableRng + Clone>(seed: u64) -> (R, R) {
        let rng = R::seed_from_u64(seed);
        let rng2 = rng.clone(); // need to clone `rng` to produce the same results before it is used as `gen` mutates it
        (rng, rng2)
    }

    test_random!(utest; StdRng, SmallRng);
    test_random!(itest; StdRng, SmallRng);
}
