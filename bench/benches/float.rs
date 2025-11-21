use bnum_old::cast::CastFrom as CastFromOld;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use bnum::cast::CastFrom;

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

const N: usize = 22;
new_to_old!(N);

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("round");
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let big_inputs = (0..SAMPLE_SIZE)
        .map(|_| rng.random::<(Uint<{N*8}>, Uint<{N*8}>)>())
        .map(|(a, b)| (
            ((a, b), (bnum_old::BUint::bfrom(a), bnum_old::BUint::bfrom(b)))
        ));
    let (inputs1, inputs2) = unzip::unzip2(big_inputs);

    let s = Uint::<{N*8}>::MAX.to_str_radix(10);

    // group.bench_with_input(BenchmarkId::new("Recursive", "new"), &big_inputs, |b, inputs| b.iter(|| {
    //     for a in inputs.iter().cloned() {
    //         let _ = black_box(a).floor();
    //     }
    // }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d8"), &inputs1, |b, inputs| b.iter(|| {
        inputs.iter().cloned().map(|(a, b)| {
            a.leading_zeros()
        }).collect::<Vec<_>>()
    }));
    group.bench_with_input(BenchmarkId::new("Iterative", "d64"), &inputs2, |b, inputs| b.iter(|| {
        inputs.iter().cloned().map(|(a, b)| {
            a.leading_ones()
        }).collect::<Vec<_>>()
    }));
    group.finish();
}

criterion_group!(benches, bench_add);
criterion_main!(benches);