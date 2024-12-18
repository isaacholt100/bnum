use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

mod unzip;

const SAMPLE_SIZE: usize = 10000;

use bnum::Float;

type F64 = Float<8, 52>;
type F32 = Float<4, 23>;


fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.gen::<f64>())
        .map(|a| a.to_bits())
        .map(|a| (f64::from_bits(a), F64::from_bits(a.into())));
    let (prim_inputs, big_inputs) = unzip::unzip2(inputs);

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    group.bench_with_input(BenchmarkId::new("Iterative", "old"), &big_inputs, |b, inputs| b.iter(|| {
        inputs.iter().cloned().for_each(|a| {
            let _ = black_box(a).trunc();
        })
    }));
    group.bench_with_input(BenchmarkId::new("Iterative", "prim"), &prim_inputs, |b, inputs| b.iter(|| {
        inputs.iter().cloned().for_each(|a| {
            let _ = black_box(a).trunc();
        })
    }));
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);