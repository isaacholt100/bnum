/*use bnum::types::{U1024, U512};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rug::Integer;

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

bench_with_primitive!(
    u128,
    checked_next_power_of_two(a),
    [0u128, 1, 4, 43, 49, 1, 3, 6, 103409825304503945].into_iter()
);
//criterion_group!(benches, bench_checked_next_power_of_two_with_primitive);

fn bench_from_str_radix(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_str_radix");

    for src in [
        "2304972034958712347519203945723045928374590234579802345790987",
        "92374952734059",
    ] {
        group.bench_with_input(BenchmarkId::new("rug", src), &src, |b, &s| {
            b.iter(|| Integer::from_str_radix(s, 16))
        });
        group.bench_with_input(BenchmarkId::new("bnum", src), &src, |b, &s| {
            b.iter(|| U512::from_str_radix(s, 16))
        });
        group.bench_with_input(BenchmarkId::new("primitive", src), &src, |b, &s| {
            b.iter(|| u128::from_str_radix(s, 16))
        });
    }
}

fn bench_to_str_radix(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_str_radix");

    for src in [
        "230947523049587103204586710392345793045607203975927348",
        "2283456978970345",
    ] {
        let r = Integer::from_str_radix(src, 10).unwrap();
        let big = U512::from_str_radix(src, 10).unwrap();

        group.bench_with_input(BenchmarkId::new("rug", src), &r, |b, i| {
            b.iter(|| format!("{}", i))
        });
        group.bench_with_input(BenchmarkId::new("bnum", src), &big, |b, i| {
            b.iter(|| format!("{}", i))
        });
    }
}

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add");

    for src in [
        "230947523049587103204586710392345793045607203975927348",
        "2283456978970345",
    ] {
        let r = Integer::from_str_radix(src, 10).unwrap();
        let big = U512::from_str_radix(src, 10).unwrap();

        group.bench_with_input(BenchmarkId::new("rug", src), &r, |b, i| {
            b.iter(|| Integer::from(i + i))
        });
        group.bench_with_input(BenchmarkId::new("bnum", src), &big, |b, i| b.iter(|| i + i));
    }
}

fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul");

    for src in [
        "230947523049587103204586710392345793045607203975927348",
        "2283456978970345",
    ] {
        let r = Integer::from_str_radix(src, 10).unwrap();
        let big = U1024::from_str_radix(src, 10).unwrap();

        group.bench_with_input(BenchmarkId::new("rug", src), &r, |b, i| {
            b.iter(|| Integer::from(i * i))
        });
        group.bench_with_input(BenchmarkId::new("bnum", src), &big, |b, i| b.iter(|| i * i));
    }
}

fn bench_count_ones(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_ones");

    for src in [
        "230947523049587103204586710392345793045607203975927348",
        "2283456978970345",
    ] {
        let r = Integer::from_str_radix(src, 10).unwrap();
        let big = bnum::types::U2048::from_str_radix(src, 10).unwrap();

        group.bench_with_input(BenchmarkId::new("rug", src), &r, |b, i| {
            b.iter(|| i.count_ones())
        });
        group.bench_with_input(BenchmarkId::new("bnum", src), &big, |b, i| {
            b.iter(|| i.count_ones())
        });
    }
}

fn bench_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("ops");

    let a = u128::MAX;
    let b = u128::MAX >> 24;

    let c = u64::MAX;
    let d = u64::MAX >> 24;

    let e = u8::MAX;
    let f = u8::MAX;

    group.bench_with_input(BenchmarkId::new("128", "add"), &(a, b), |b, &(a, c)| {
        b.iter(|| for _ in 0..100000 { black_box(a.wrapping_add(c)); })
    });
    group.bench_with_input(BenchmarkId::new("64", "add"), &(c, d), |b, &(a, c)| {
        b.iter(|| for _ in 0..100000 { black_box(a.wrapping_add(c)); })
    });
    group.bench_with_input(BenchmarkId::new("8", "add"), &(e, f), |b, &(a, c)| {
        b.iter(|| for _ in 0..100000 { black_box(a.wrapping_add(c)); })
    });
}

fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("div");

    for src in [
        "230947523049587103204586710392345793045607203975927348",
        "2283456978970345",
    ] {
        let r = Integer::from_str_radix(src, 10).unwrap();
        let big = U1024::from_str_radix(src, 10).unwrap();

        group.bench_with_input(BenchmarkId::new("rug", src), &r, |b, i| {
            b.iter(|| Integer::from(i / 10928))
        });
        group.bench_with_input(BenchmarkId::new("bnum", src), &big, |b, i| b.iter(|| *i));
    }
}

criterion_group!(benches, bench_ops);

criterion_main!(benches);
*/
#![feature(wrapping_next_power_of_two, int_roundings)]

use bnum::types::U128;
use core::iter::Iterator;
use criterion::black_box;
use rand::prelude::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const SAMPLE_SIZE: usize = 10000;

trait Into2<T> {
    fn into(self) -> T;
}

impl<T, U: Into<T>> Into2<T> for U {
    fn into(self) -> T {
        Into::into(self)
    }
}

fn unzip3<T, U, V, I>(iterator: I) -> (Vec<T>, Vec<U>, Vec<V>)
where I: Iterator<Item = (T, U, V)> {
    let (mut v1, mut v2, mut v3) = match iterator.size_hint().1 {
        Some(cap) => (Vec::with_capacity(cap), Vec::with_capacity(cap), Vec::with_capacity(cap)),
        None => (Vec::new(), Vec::new(), Vec::new()),
    };
    iterator.for_each(|(t, u, v)| {
        v1.push(t);
        v2.push(u);
        v3.push(v);
    });

    (v1, v2, v3)
}

macro_rules! bench_against_primitive {
    { $primitive: ty; $($method: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *);) * } => {
        paste::paste! {
            $(
                fn [<bench_ $primitive _ $method>](c: &mut Criterion) {
                    let mut group = c.benchmark_group(stringify!($method));
                    let mut rng = rand::rngs::StdRng::seed_from_u64(0); // use same seed so can compare between different benchmarks more accurately
                    #[allow(unused_parens)]
                    let inputs: (Vec<_>, Vec<_>) = (0..SAMPLE_SIZE)
                        .map(|_| rng.gen::<($($ty), *)>())
                        .map(|($($param), *)| (
                            ($(Into::into($param)), *, ()),
                            ($($param), *, ()),
                            // ($(TryInto::try_into($param).unwrap()), *, ())
                        ))
                        .unzip();
                    let big_inputs = inputs.0;
                    let prim_inputs = inputs.1;
                    // let ruint_inputs = inputs.2;
                    group.bench_with_input(BenchmarkId::new("bnum", "rand"), &big_inputs, |b, inputs| {
                        b.iter(|| {
                            for ($($param), *, ()) in inputs.iter().cloned() {
                                let _ = [<$primitive:upper>]::$method($($($re)? black_box($param)), *);
                            }
                        })
                    });
                    group.bench_with_input(BenchmarkId::new("core", "rand"), &prim_inputs, |b, inputs| {
                        b.iter(|| {
                            for ($($param), *, ()) in inputs.iter().cloned() {
                                let _ = <$primitive>::$method($($($re)? black_box($param)), *);
                            }
                        })
                    });
                    // group.bench_with_input(BenchmarkId::new("ruint", "rand"), &ruint_inputs, |b, inputs| {
                    //     b.iter(|| {
                    //         #[allow(unused_parens)]
                    //         for ($($param), *, ()) in inputs.iter().cloned() {
                    //             let _ = [<R $primitive:upper>]::$method($($($re)? black_box($param)), *);
                    //         }
                    //     })
                    // });
                    group.finish();
                }
            )*
            criterion_group!([<$primitive _benches>], $([<bench_ $primitive _ $method>]), *);
        }
    };
}

trait Format {
    fn display(self) -> String;
    fn debug(self) -> String;
    fn binary(self) -> String;
    fn upper_hex(self) -> String;
    fn lower_hex(self) -> String;
    fn octal(self) -> String;
    fn upper_exp(self) -> String;
    fn lower_exp(self) -> String;
}

macro_rules! impl_format {
    ($($ty: ty), *) => {
        $(
            impl Format for $ty {
                fn display(self) -> String {
                    format!("{}", self)
                }
                fn debug(self) -> String {
                    format!("{:?}", self)
                }
                fn binary(self) -> String {
                    format!("{:b}", self)
                }
                fn upper_hex(self) -> String {
                    format!("{:X}", self)
                }
                fn lower_hex(self) -> String {
                    format!("{:x}", self)
                }
                fn octal(self) -> String {
                    format!("{:o}", self)
                }
                fn upper_exp(self) -> String {
                    format!("{:E}", self)
                }
                fn lower_exp(self) -> String {
                    format!("{:e}", self)
                }
            }
        )*
    };
}

impl_format!(u128, U128);

use core::cmp::{PartialEq, PartialOrd};
use core::ops::{BitAnd, BitOr, BitXor, Not};

bench_against_primitive! {
    u128;
    checked_add(a: u128, b: u128);
    checked_add_signed(a: u128, b: i128);
    checked_sub(a: u128, b: u128);
    checked_mul(a: u128, b: u128);
    checked_div(a: u128, b: u128);
    checked_div_euclid(a: u128, b: u128);
    checked_rem(a: u128, b: u128);
    checked_rem_euclid(a: u128, b: u128);
    checked_neg(a: u128);
    checked_shl(a: u128, b: u32);
    checked_shr(a: u128, b: u32);
    checked_pow(a: u128, b: u32);
    checked_next_multiple_of(a: u128, b: u128);
    checked_ilog2(a: u128);
    checked_ilog10(a: u128);
    checked_ilog(a: u128, b: u128);
    checked_next_power_of_two(a: u128);

    from_be(a: u128);
    from_le(a: u128);
    to_be(a: u128);
    to_le(a: u128);
    to_be_bytes(a: u128);
    to_le_bytes(a: u128);
    to_ne_bytes(a: u128);
    from_be_bytes(a: [u8; 128 / 8]);
    from_le_bytes(a: [u8; 128 / 8]);
    from_ne_bytes(a: [u8; 128 / 8]);

    overflowing_add(a: u128, b: u128);
    overflowing_add_signed(a: u128, b: i128);
    overflowing_sub(a: u128, b: u128);
    overflowing_mul(a: u128, b: u128);
    overflowing_neg(a: u128);
    overflowing_shl(a: u128, b: u32);
    overflowing_shr(a: u128, b: u32);
    overflowing_pow(a: u128, b: u32);

    display(a: u128);
    debug(a: u128);
    binary(a: u128);
    upper_hex(a: u128);
    lower_hex(a: u128);
    octal(a: u128);
    upper_exp(a: u128);
    lower_exp(a: u128);

    saturating_add(a: u128, b: u128);
    saturating_add_signed(a: u128, b: i128);
    saturating_sub(a: u128, b: u128);
    saturating_mul(a: u128, b: u128);
    saturating_pow(a: u128, exp: u32);

    wrapping_add(a: u128, b: u128);
    wrapping_add_signed(a: u128, b: i128);
    wrapping_sub(a: u128, b: u128);
    wrapping_mul(a: u128, b: u128);
    wrapping_neg(a: u128);
    wrapping_shl(a: u128, rhs: u32);
    wrapping_shr(a: u128, rhs: u32);
    wrapping_pow(a: u128, exp: u32);
    wrapping_next_power_of_two(a: u128);

    count_ones(a: u128);
    count_zeros(a: u128);
    leading_zeros(a: u128);
    trailing_zeros(a: u128);
    leading_ones(a: u128);
    trailing_ones(a: u128);
    rotate_left(a: u128, b: u32);
    rotate_right(a: u128, b: u32);
    swap_bytes(a: u128);
    reverse_bits(a: u128);
    is_power_of_two(a: u128);

    bitand(a: u128, b: u128);
    bitor(a: u128, b: u128);
    bitxor(a: u128, b: u128);
    not(a: u128);

    eq(a: ref &u128, b: ref &u128);
    partial_cmp(a: ref &u128, b: ref &u128);
}

type U64 = bnum::BUint::<1>;

fn bench_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("asdfasdf");
    let prim_input = (u64::MAX, u64::MAX);
    let big_input = (U64::MAX, U64::MAX);
    group.bench_with_input(BenchmarkId::new("bnum", "bnum"), &big_input, |b, input| {
        b.iter(|| black_box(input.0).wrapping_add(black_box(input.1)))
    });
    group.bench_with_input(BenchmarkId::new("core", "core"), &prim_input, |b, input| {
        b.iter(|| black_box(input.0).wrapping_add(black_box(input.1)))
    });
}

criterion_group!(bench2, bench_test);

criterion_main!(bench2, u128_benches);
