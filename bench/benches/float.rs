use bnum_old::cast::CastFrom as CastFromOld;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use bnum::cast::CastFrom;

mod unzip;

const SAMPLE_SIZE: usize = 500;

use bnum::{Int, Uint};
use bnum::Float;
use bnum::cast::As;

use bnum::types::{F32, F64, F128, F256, U256, U1024, I1024};


fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.gen::<(u64, u64)>())
        .map(|(a, b)| (
            (f64::from_bits(a >> 1), f64::from_bits(b >> 1)),
            (F64::from_bits((a >> 1).as_()), F64::from_bits((b >> 1).as_()))
        ));
    let (prim_inputs, big_inputs) = unzip::unzip2(inputs);

//     // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
//     //     for a in inputs.iter().cloned() {
//     //         let _ = black_box(a).floor();
//     //     }
//     // }));
//     group.bench_with_input(BenchmarkId::new("Iterative", "old"), &big_inputs, |b, inputs| b.iter(|| {
//         inputs.iter().cloned().for_each(|(a, b)| {
//             let _ = black_box(a) + black_box(b);
//         })
//     }));
//     group.bench_with_input(BenchmarkId::new("Iterative", "prim"), &prim_inputs, |b, inputs| b.iter(|| {
//         inputs.iter().cloned().for_each(|(a, b)| {
//             let _ = black_box(a) + black_box(b);
//         })
//     }));
//     group.finish();
}

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
                    let digits = *b.digits();
                    let old_d8 = bnum_old::BUintD8::from_digits(digits);
                    let out = <Self as bnum_old::cast::CastFrom<bnum_old::BUintD8<{$N * 8}>>>::cast_from(old_d8);
                    // assert_eq!(out.to_str_radix(16), b.to_str_radix(16));
                    out
                }
            }

            impl BFrom<Int<{$N * 8}>> for bnum_old::BInt<$N> {
                fn bfrom(b: Int<{$N * 8}>) -> Self {
                    Self::from_bits(bnum_old::BUint::bfrom(b.to_bits()))
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

new_to_old!(62);

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    const N: usize = 62;
    let big_inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.gen::<(Int<{N*8}>, i128)>())
        .map(|(a, b)| (
            ((a, b), (bnum_old::BInt::bfrom(a), b))
        ));
    let (inputs1, inputs2) = unzip::unzip2(big_inputs);

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d8"), &inputs1, |b, inputs| b.iter(|| {
        inputs.iter().cloned().map(|(a, b)| {
            i128::try_from(a)
        }).collect::<Vec<_>>()
    }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d64"), &inputs2, |b, inputs| b.iter(|| {
        inputs.iter().cloned().map(|(a, b)| {
            i128::try_from(a)
        }).collect::<Vec<_>>()
    }));
    group.finish();
}

criterion_group!(benches, bench_add);
criterion_main!(benches);