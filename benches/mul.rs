use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bnum::BUint;

pub fn criterion_benchmark1(c: &mut Criterion) {
	let a1 = u128::MAX;
	//let a2 = rand::random::<BUint::<10000>>();
    c.bench_function(
		"to_be_bytes_prim",
		|b| b.iter(|| {
			black_box(a1).to_be_bytes()
		})
	);
}

pub fn criterion_benchmark2(c: &mut Criterion) {
	let a1 = bnum::U128::MAX;
	//let a2 = rand::random::<BUint::<10000>>();
    c.bench_function(
		"to_be_bytes_big",
		|b| b.iter(|| {
			black_box(a1).to_be_bytes()
		})
	);
}

criterion_group!(benches, criterion_benchmark1, criterion_benchmark2);
criterion_main!(benches);