/// benchmark CastFrom<I2024> for U1024
use bnum::cast::{As, CastFrom};
use bnum::types::{I1024, U1024};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const fn min1(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

const fn min2(a: usize, b: usize) -> usize {
    b ^ ((a ^ b) & (-((a < b) as isize) as usize))
}

fn benchmark_cast_from(c: &mut Criterion) {
    c.bench_function("CastFrom<I1024> for U1024", |b| {
        b.iter(|| {
            black_box(min2(1234, 123948))
        });
    });
}

criterion_group!(benches, benchmark_cast_from);
criterion_main!(benches);