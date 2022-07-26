use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rug::Integer;
use bnum::types::{U512, U1024};

macro_rules! bench_with_primitive {
	($primitive: ty, $method: ident ($($param: ident), *), $iterator: expr) => {
		paste::paste! {
			fn [<bench_ $method _with_primitive>](c: &mut criterion::Criterion) {
				type Big = bnum::types::[<$primitive:upper>];
				type Primitive = $primitive;

				let mut group = c.benchmark_group(stringify!($method));
				
				#[allow(unused_parens)]	
				for (i, ($($param), *)) in $iterator.enumerate() {
					group.bench_with_input(
						BenchmarkId::new("Big Integer", i),
						&($($param.into()), *),
						|b, &($($param), *)| b.iter(|| Big::$method($($param), *))
					);
					group.bench_with_input(
						BenchmarkId::new("Primitive Integer", i),
						&($($param.into()), *),
						|b, &($($param), *)| b.iter(|| Primitive::$method($($param), *))
					);
				}
				
				group.finish();
			}
		}
	};
}

bench_with_primitive!(u128, checked_next_power_of_two(a), [0u128, 1, 4, 43, 49, 1, 3, 6, 103409825304503945].into_iter());
//criterion_group!(benches, bench_checked_next_power_of_two_with_primitive);

fn bench_from_str_radix(c: &mut Criterion) {
	let mut group = c.benchmark_group("from_str_radix");

	for src in ["2304972034958712347519203945723045928374590234579802345790987", "92374952734059"] {
		group.bench_with_input(
			BenchmarkId::new("rug", src),
			&src,
			|b, &s| b.iter(|| Integer::from_str_radix(s, 16))
		);
		group.bench_with_input(
			BenchmarkId::new("bnum", src),
			&src,
			|b, &s| b.iter(|| U512::from_str_radix(s, 16))
		);
		group.bench_with_input(
			BenchmarkId::new("primitive", src),
			&src,
			|b, &s| b.iter(|| u128::from_str_radix(s, 16))
		);
	}
}

fn bench_to_str_radix(c: &mut Criterion) {
	let mut group = c.benchmark_group("to_str_radix");

	for src in ["230947523049587103204586710392345793045607203975927348", "2283456978970345"] {
		let r = Integer::from_str_radix(src, 10).unwrap();
		let big = U512::from_str_radix(src, 10).unwrap();

		group.bench_with_input(
			BenchmarkId::new("rug", src),
			&r,
			|b, i| b.iter(|| format!("{}", i))
		);
		group.bench_with_input(
			BenchmarkId::new("bnum", src),
			&big,
			|b, i| b.iter(|| format!("{}", i))
		);
	}
}

fn bench_add(c: &mut Criterion) {
	let mut group = c.benchmark_group("add");

	for src in ["230947523049587103204586710392345793045607203975927348", "2283456978970345"] {
		let r = Integer::from_str_radix(src, 10).unwrap();
		let big = U512::from_str_radix(src, 10).unwrap();

		group.bench_with_input(
			BenchmarkId::new("rug", src),
			&r,
			|b, i| b.iter(|| Integer::from(i + i))
		);
		group.bench_with_input(
			BenchmarkId::new("bnum", src),
			&big,
			|b, i| b.iter(|| i + i)
		);
	}
}

fn bench_mul(c: &mut Criterion) {
	let mut group = c.benchmark_group("mul");

	for src in ["230947523049587103204586710392345793045607203975927348", "2283456978970345"] {
		let r = Integer::from_str_radix(src, 10).unwrap();
		let big = U1024::from_str_radix(src, 10).unwrap();

		group.bench_with_input(
			BenchmarkId::new("rug", src),
			&r,
			|b, i| b.iter(|| Integer::from(i * i))
		);
		group.bench_with_input(
			BenchmarkId::new("bnum", src),
			&big,
			|b, i| b.iter(|| i * i)
		);
	}
}

fn bench_count_ones(c: &mut Criterion) {
	let mut group = c.benchmark_group("count_ones");

	for src in ["230947523049587103204586710392345793045607203975927348", "2283456978970345"] {
		let r = Integer::from_str_radix(src, 10).unwrap();
		let big = bnum::types::U2048::from_str_radix(src, 10).unwrap();

		group.bench_with_input(
			BenchmarkId::new("rug", src),
			&r,
			|b, i| b.iter(|| i.count_ones())
		);
		group.bench_with_input(
			BenchmarkId::new("bnum", src),
			&big,
			|b, i| b.iter(|| i.count_ones())
		);
	}
}

fn bench_div(c: &mut Criterion) {
	let mut group = c.benchmark_group("div");

	for src in ["230947523049587103204586710392345793045607203975927348", "2283456978970345"] {
		let r = Integer::from_str_radix(src, 10).unwrap();
		let big = U1024::from_str_radix(src, 10).unwrap();

		group.bench_with_input(
			BenchmarkId::new("rug", src),
			&r,
			|b, i| b.iter(|| Integer::from(i / 10928))
		);
		group.bench_with_input(
			BenchmarkId::new("bnum", src),
			&big,
			|b, i| b.iter(|| *i)
		);
	}
}

criterion_group!(benches, bench_div);

criterion_main!(benches);