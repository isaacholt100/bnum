use bnum::cast::CastFrom;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

mod unzip;

const SAMPLE_SIZE: usize = 10000;

// use bnum::Float;

// type F256 = Float<32, 236>;
// type F128 = Float<32, 236>;
// type F64 = Float<8, 48>;
// type F32 = Float<4, 23>;

type U256 = bnum::BUintD8<32>;
type U1024 = bnum::BUintD8<{17 * 8}>;
type U1024D64 = bnum::BUint<17>;


// fn bench_fibs(c: &mut Criterion) {
//     let mut group = c.benchmark_group("round");
//     let mut rng = rand::rngs::StdRng::seed_from_u64(0);
//     let inputs = (0..SAMPLE_SIZE)
//         .map(|_| rng.gen::<(u64, u64)>())
//         .map(|(a, b)| (
//             (f64::from_bits(a >> 1), f64::from_bits(b >> 1)),
//             (F64::from_bits((a >> 1).into()), F64::from_bits((b >> 1).into()))
//         ));
//     let (prim_inputs, big_inputs) = unzip::unzip2(inputs);

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
// }


fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let big_inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.gen::<(U1024, U1024)>())
        .map(|(a, b)| ((a, b), (U1024D64::cast_from(a), U1024D64::cast_from(b))));
        // .map(|(a, b)| (
        //     (F64::from_bits((a >> 1)), F64::from_bits((b >> 1)))
        // ));
    let (inputs1, inputs2) = unzip::unzip2(big_inputs);

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d8"), &inputs1, |b, inputs| b.iter(|| {
        inputs.iter().cloned().for_each(|(a, b)| {
            let _ = black_box(!a);
        })
    }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d64"), &inputs2, |b, inputs| b.iter(|| {
        inputs.iter().cloned().for_each(|(a, b)| {
            let _ = black_box(!a);
        })
    }));
    group.finish();
}

criterion_group!(benches, bench_add);
criterion_main!(benches);