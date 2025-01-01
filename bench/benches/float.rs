use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

mod unzip;

const SAMPLE_SIZE: usize = 10000;

use bnum::Float;

type F256 = Float<32, 236>;
type F128 = Float<32, 236>;
type F64 = Float<8, 48>;
type F32 = Float<4, 23>;

type U256 = bnum::BUintD8<32>;
type U1024 = bnum::BUintD8<128>;


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


fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let big_inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.gen::<(U1024, u32)>());
        // .map(|(a, b)| (
        //     (F64::from_bits((a >> 1)), F64::from_bits((b >> 1)))
        // ));
    let big_inputs: Vec<_> = big_inputs.collect();

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    group.bench_with_input(BenchmarkId::new("Iterative", "old"), &big_inputs, |b, inputs| b.iter(|| {
        inputs.iter().cloned().for_each(|(a, b)| {
            let _ = black_box(black_box(a).overflowing_shl(black_box(b)));
        })
    }));
    group.finish();
}

criterion_group!(benches, bench_add);
criterion_main!(benches);