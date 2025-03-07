use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use bnum::cast::CastFrom;

mod unzip;

const SAMPLE_SIZE: usize = 10000;

use bnum::{BIntD8, BUintD8};
use bnum::Float;

type F256 = Float<32, 236>;
type F128 = Float<32, 236>;
type F64 = Float<8, 48>;
type F32 = Float<4, 23>;

type U256 = bnum::BUintD8<32>;
type U1024 = bnum::BUintD8<128>;
type I1024 = bnum::BIntD8<128>;


fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.gen::<(u64, u64)>())
        .map(|(a, b)| (
            (f64::from_bits(a >> 1), f64::from_bits(b >> 1)),
            (F64::from_bits((a >> 1).into()), F64::from_bits((b >> 1).into()))
        ));
    let (prim_inputs, big_inputs) = unzip::unzip2(inputs);

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    group.bench_with_input(BenchmarkId::new("Iterative", "old"), &big_inputs, |b, inputs| b.iter(|| {
        inputs.iter().cloned().for_each(|(a, b)| {
            let _ = black_box(a) + black_box(b);
        })
    }));
    group.bench_with_input(BenchmarkId::new("Iterative", "prim"), &prim_inputs, |b, inputs| b.iter(|| {
        inputs.iter().cloned().for_each(|(a, b)| {
            let _ = black_box(a) + black_box(b);
        })
    }));
    group.finish();
}

// benchmark for time taken to call BUintD8::u128_digit, using one random fixed input, no benchmark groups, just a standalone benchmark
// fn bench_u128_digit(c: &mut Criterion) {
//     let mut group = c.benchmark_group("digit");
//     let mut rng = rand::rngs::StdRng::seed_from_u64(0);
//     let input = rng.gen::<U1024>();
//     group.bench_with_input(BenchmarkId::new("u128", "d8"), &input, |b, input| b.iter(|| input.digit(0)));
//     group.bench_with_input(BenchmarkId::new("u128", "u128"), &input, |b, input| b.iter(|| input.digit(0)));
// }

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

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    const N: usize = 17;
    let big_inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.gen::<(BUintD8<{N*8}>, BUintD8<{N*8}>)>())
        .map(|(a, b)| (
            ((a, b), (unsafe { core::mem::transmute::<_, bnum_old::BUint<N>>(a) }, unsafe { core::mem::transmute::<_, bnum_old::BUint<N>>(b) }))
        ));
    let (inputs1, inputs2) = unzip::unzip2(big_inputs);

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d8"), &inputs1, |b, inputs| b.iter(|| {
        inputs.iter().cloned().map(|(a, b)| {
            a >> 47
        }).collect::<Vec<_>>()
    }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d64"), &inputs2, |b, inputs| b.iter(|| {
        inputs.iter().cloned().map(|(a, b)| {
            a >> 47
        }).collect::<Vec<_>>()
    }));
    group.finish();
}

criterion_group!(benches, bench_add);
criterion_main!(benches);