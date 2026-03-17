use bnum_old::cast::CastFrom as CastFromOld;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use bnum::cast::CastFrom;

use core::hint::black_box;

mod unzip;

const SAMPLE_SIZE: usize = 1000;

use bnum::{Int, Uint};
// use bnum::Float;
use bnum::cast::As;

// use bnum::types::{F32, F64, F128, F256, U256, U1024, I1024};


// benchmark for time taken to call Uint::u128_digit, using one random fixed input, no benchmark groups, just a standalone benchmark
// fn bench_u128_digit(c: &mut Criterion) {
//     let mut group = c.benchmark_group("digit");
//     let mut rng = rand::rngs::StdRng::seed_from_u64(0);
//     let input = rng.gen::<U1024>();
//     group.bench_with_input(BenchmarkId::new("u128", "d8"), &input, |b, input| b.iter(|| input.digit(0)));
//     group.bench_with_input(BenchmarkId::new("u128", "u128"), &input, |b, input| b.iter(|| input.digit(0)));
// }

trait BFrom<T> {
    fn bfrom(t: T) -> Self;
}

macro_rules! new_to_old {
    ($($N: expr), *) => {
        $(
            impl BFrom<Uint<{$N * 8}>> for bnum_old::BUint<$N> {
                fn bfrom(b: Uint<{$N * 8}>) -> Self {
                    assert!(core::mem::size_of::<Self>() == core::mem::size_of::<Uint<{$N * 8}>>());
                    let digits = *b.as_bytes();
                    let old_d8 = bnum_old::BUintD8::from_digits(digits);
                    let out = <Self as bnum_old::cast::CastFrom<bnum_old::BUintD8<{$N * 8}>>>::cast_from(old_d8);
                    // assert_eq!(out.to_str_radix(16), b.to_str_radix(16));
                    out
                }
            }

            impl BFrom<Int<{$N * 8}>> for bnum_old::BInt<$N> {
                fn bfrom(b: Int<{$N * 8}>) -> Self {
                    Self::from_bits(bnum_old::BUint::bfrom(b.cast_unsigned()))
                }
            }

            impl BFrom<Uint<{$N * 4}>> for bnum_old::BUintD32<$N> {
                fn bfrom(b: Uint<{$N * 4}>) -> Self {
                    assert!(core::mem::size_of::<Self>() == core::mem::size_of::<Uint<{$N * 4}>>());
                    let digits = *b.as_bytes();
                    let old_d8 = bnum_old::BUintD8::from_digits(digits);
                    let out = <Self as bnum_old::cast::CastFrom<bnum_old::BUintD8<{$N * 4}>>>::cast_from(old_d8);
                    // assert_eq!(out.to_str_radix(16), b.to_str_radix(16));
                    out
                }
            }

            impl BFrom<Int<{$N * 4}>> for bnum_old::BIntD32<$N> {
                fn bfrom(b: Int<{$N * 4}>) -> Self {
                    Self::from_bits(bnum_old::BUintD32::bfrom(b.cast_unsigned()))
                }
            }
        )*
    }
}

const fn last_digit_index<const N: usize>(b: &bnum_old::BUint<N>) -> usize {
    let digits = b.digits();
    let mut index = 0;
    let mut i = 1;
    while i < N {
        if digits[i] != 0 {
            index = i;
        }
        i += 1;
    }
    index
}

const N: usize = 10;
const M: usize = N * 2;
new_to_old!(N);
new_to_old!(M);

use num_integer::Roots;

fn from_be_bytes2(mut bytes: [u8; 16]) -> u128 {
    bytes.reverse();
    u128::from_le_bytes(bytes)
}

fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let big_inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.random::<(Uint<4>, Uint<4>, u32)>())
        .map(|(a, b, c)| (
            (
                (a, b, c),
                (a.as_::<u32>(), b.as_::<u32>(), c)
            )
            // ((a.to_str_radix(16), b), (a.to_str_radix(16), b))
        ));
    let (inputs1, inputs2) = unzip::unzip2(big_inputs);

    let mut i = 0;

    use core::ops::BitAnd;

    group.bench_with_input(BenchmarkId::new("Iterative", "bnum"), "bnum", |b, _| b.iter_batched(|| {
        i += 1;
        inputs1[i % SAMPLE_SIZE]
    }, |(a, b, c)| {
        black_box(black_box(a) & black_box(b))
    }, criterion::BatchSize::SmallInput));

    i = 0;
    group.bench_with_input(BenchmarkId::new("Iterative", "u32"), "u32", |b, _| b.iter_batched(|| {
        i += 1;
        inputs2[i % SAMPLE_SIZE]
    }, |(a, b, c)| {
        black_box(black_box(a) & black_box(b))
    }, criterion::BatchSize::SmallInput));

    group.finish();
}

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let big_inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.random::<(Uint<{N*8}>, Uint<{N * 8}>, u32)>())
        .map(|(a, b, c)| (
            (
                (a, b, c),
                (bnum_old::BUint::bfrom(a), bnum_old::BUint::bfrom(b), c),
                (bnum_old::BUintD32::bfrom(a), bnum_old::BUintD32::bfrom(b), c),
            )
            // ((a.to_str_radix(16), b), (a.to_str_radix(16), b))
        ));
    let (inputs1, inputs2, inputs3) = unzip::unzip3(big_inputs);

    let s = Uint::<{N*8}>::MAX.to_str_radix(10);

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    let mut i = 0;

    group.bench_with_input(BenchmarkId::new("Iterative", "d8"), "d8", |b, _| b.iter_batched(|| {
        i += 1;
        inputs1[i % SAMPLE_SIZE]
    }, |(a, b, c)| {
        a.leading_zeros()
    }, criterion::BatchSize::SmallInput));

    i = 0;
    group.bench_with_input(BenchmarkId::new("Iterative", "d64"), "d64", |b, _| b.iter_batched(|| {
        i += 1;
        inputs2[i % SAMPLE_SIZE]
    }, |(a, b, c)| {
        a.leading_zeros()
    }, criterion::BatchSize::SmallInput));
    // TODO: comopare with rug, ruint, num_bigint, etc, primitives

    i = 0;
    group.bench_with_input(BenchmarkId::new("Iterative", "d32"), "d32", |b, _| b.iter_batched(|| {
        i += 1;
        inputs3[i % SAMPLE_SIZE]
    }, |(a, b, c)| {
        a.leading_zeros()
    }, criterion::BatchSize::SmallInput));

    // group.bench_with_input(BenchmarkId::new("Iterative", "d8"), &inputs1, |b, inputs| b.iter(|| {
    //     i += 1;

    //     inputs[i % SAMPLE_SIZE].0 .0.swap_bytes();
    //     inputs.iter().cloned().map(|(a, b, c)| {
    //         black_box(a).swap_bytes()
    //         // Uint::<{N*8}>::from_str_radix(&a, 16)
    //     }).collect::<Vec<_>>()
    // }));
    // group.bench_with_input(BenchmarkId::new("Iterative", "d64"), &inputs2, |b, inputs| b.iter(|| {
    //     inputs.iter().cloned().map(|(a, b, c)| {
    //         black_box(a).swap_bytes()
    //         // bnum_old::BUint::<N>::from_str_radix(&a, 16)
    //     }).collect::<Vec<_>>()
    // }));
    // group.bench_with_input(BenchmarkId::new("Iterative", "d32"), &inputs3, |b, inputs| b.iter(|| {
    //     inputs.iter().cloned().map(|(a, b, c)| {
    //         a * b
    //         // bnum_old::BUintD32::<N>::from_str_radix(&a, 16)
    //     }).collect::<Vec<_>>()
    // }));
    group.finish();
}

criterion_group!(benches, bench_mul);
criterion_main!(benches);