use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bnum::BUint;
use num_integer::Roots;

pub fn criterion_benchmark(c: &mut Criterion) {
	let a1 = BUint::<100>::MAX;
	//let a2 = rand::random::<BUint::<10000>>();
    c.bench_function(
		"mul",
		|b| b.iter(|| {
			black_box(a1).sqrt()
		})
	);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);