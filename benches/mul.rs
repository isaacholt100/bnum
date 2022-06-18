use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bnum::BUint;

pub fn criterion_benchmark1(c: &mut Criterion) {
	let a1 = BUint::<100>::MAX;
	let a2 = BUint::<100>::from(1000000u32);
	//let a2 = rand::random::<BUint::<10000>>();
    c.bench_function(
		"int_log",
		|b| b.iter(|| {
			black_box(a1).checked_log(black_box(a2))
		})
	);
}

pub fn criterion_benchmark2(c: &mut Criterion) {
	let a1 = BUint::<100>::MAX;
	let a2 = BUint::<100>::from(1000000u32);
	//let a2 = rand::random::<BUint::<10000>>();
    c.bench_function(
		"log",
		|b| b.iter(|| {
			black_box(a1).log(black_box(a2))
		})
	);
}

criterion_group!(benches, criterion_benchmark1, criterion_benchmark2);
criterion_main!(benches);