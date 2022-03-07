extern crate test;


use crate::BUint;

#[inline]
pub const fn add_carry_unsigned(carry: u8, a: u64, b: u64) -> (u64, u8) {
    let sum = a as u128 + b as u128 + carry as u128;
    (sum as u64, (sum >> 64) as u8)
}

#[inline]
pub const fn carrying_add(a: u64, rhs: u64, carry: bool) -> (u64, bool) {
    // note: longer-term this should be done via an intrinsic, but this has been shown
    //   to generate optimal code for now, and LLVM doesn't have an equivalent intrinsic
    let (a, b) = a.overflowing_add(rhs);
    let (c, d) = a.overflowing_add(carry as u64);
    (c, b | d)
}

#[bench]
fn bench_1(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10000 {
            test::black_box(carrying_add(i, i, true));
        }
        for i in u64::MAX-10000..u64::MAX {
            test::black_box(carrying_add(i, i, true));
        }
    })
}

#[bench]
fn bench_2(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10000 {
            test::black_box(add_carry_unsigned(1, i, i));
        }
        for i in u64::MAX-10000..u64::MAX {
            test::black_box(add_carry_unsigned(1, i, i));
        }
    })
}

#[bench]
fn bench_shift_add(b: &mut Bencher) {
    let a = 3534433434645650u64;
    b.iter(|| {
        for _ in 0..10000 {
            test::black_box((a >> 6) + (a >> 2) + (a >> 1));
        }
    });
}

#[bench]
fn bench_mul(b: &mut Bencher) {
    let a = 3534433434645650u64;
    b.iter(|| {
        for _ in 0..10000 {
            test::black_box(a * 72);
        }
    });
}

