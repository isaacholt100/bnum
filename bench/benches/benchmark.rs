#![feature(wrapping_next_power_of_two, int_roundings)]

use bnum::types::{U128, U512};
// use bnum::prelude::*;
use core::iter::Iterator;
use criterion::black_box;
use rand::prelude::*;

mod unzip;
use unzip::unzip2;

// use super::unzip2;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const SAMPLE_SIZE: usize = 10000;

macro_rules! bench_against_primitive {
    { $primitive: ty; $group_name: ident; $($method: ident ($($param: ident : $(ref $re: tt)? $ty: ty), *);) * } => {
        paste::paste! {
            $(
                fn [<bench_ $primitive _ $method>](c: &mut Criterion) {
                    let mut group = c.benchmark_group(stringify!($method));
                    let mut rng = rand::rngs::StdRng::seed_from_u64(0); // use same seed so can compare between different benchmarks more accurately
                    #[allow(unused_parens)]
                    let inputs = unzip2((0..SAMPLE_SIZE)
                        .map(|_| rng.gen::<($($ty), *)>())
                        .map(|($($param), *)| (
                            ($(Into::into($param)), *, ()),
                            ($(Into::into($param)), *, ()), // TODO: report this as bug in Rust compiler, shouldn't need extra ()
                            // ($(TryInto::try_into($param).unwrap()), *, ())
                        )));
                    let big_inputs = inputs.0;
                    let prim_inputs = inputs.1;
                    // let ruint_inputs = inputs.2;

                    const SIZE_ID: &'static str = stringify!([<$primitive:upper>]);

                    group.bench_with_input(BenchmarkId::new("bnum", SIZE_ID), &big_inputs, |b, inputs| {
                        b.iter(|| {
                            for ($($param), *, ()) in inputs.iter().cloned() {
                                let _ = [<$primitive:upper>]::$method($($($re)? black_box($param)), *);
                            }
                        })
                    });
                    group.bench_with_input(BenchmarkId::new("core", SIZE_ID), &prim_inputs, |b, inputs| {
                        b.iter(|| {
                            for ($($param), *, ()) in inputs.iter().cloned() {
                                let _ = [<$primitive>]::$method($($($re)? black_box($param)), *);
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
            criterion_group!($group_name, $([<bench_ $primitive _ $method>]), *);
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
// use core::ops::{BitAnd, BitOr, BitXor, Not};

bench_against_primitive! {
    u128; u128_benches;

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

    // bitand(a: u128, b: u128);
    // bitor(a: u128, b: u128);
    // bitxor(a: u128, b: u128);
    // not(a: u128);

    eq(a: ref &u128, b: ref &u128);
    partial_cmp(a: ref &u128, b: ref &u128);
}

criterion_main!(u128_benches);
