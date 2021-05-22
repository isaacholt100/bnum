extern crate test;

use test::Bencher;
use crate::uint::BUint;

/*#[bench]
fn bench_const_eq(b: &mut Bencher) {
    let mut arr = [39485734; 10000];
    let u1 = BUint::from(arr);
    arr[445] = 0;
    let u2 = BUint::from(arr);
    b.iter(|| {
        test::black_box(u2.eq(&u1));
    });
}

#[bench]
fn bench_eq(b: &mut Bencher) {
    let mut arr = [39485734; 10000];
    let u1 = BUint::from(arr);
    arr[445] = 0;
    let u2 = BUint::from(arr);
    b.iter(|| {
        test::black_box(u2 == u1);
    });
}*/

#[bench]
fn bench_test(b: &mut Bencher) {
    struct S {
        digits: [u64; 1000],
    }
    let s = S {
        digits: [0; 1000],
    };
    b.iter(|| {
        for i in 0..100 {
            test::black_box((&s.digits)[i]);
        }
    });
}

#[bench]
fn bench_test_own(b: &mut Bencher) {
    struct S {
        digits: [u64; 1000],
    }
    let s = S {
        digits: [0; 1000],
    };
    b.iter(|| {
        for i in 0..100 {
            test::black_box(s.digits[i]);
        }
    });
}