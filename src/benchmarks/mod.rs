extern crate test;

use test::Bencher;
use crate::uint::BUint;
use alloc::string::String;
use crate::U128;

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
fn bench_buint_add(b: &mut Bencher) {
    let u1 = 394857209495782456444589679u128;
    let u1 = U128::from(u1);
    let u2 = 30249568710094856749560704u128;
    let u2 = U128::from(u2);
    b.iter(|| {
        test::black_box(u2 + u1);
    });
}

#[bench]
fn bench_test(b: &mut Bencher) {
    let u1 = 2456799254794579u128;
    let u = U128::from(u1);
    b.iter(|| {
        for i in 0..1000000 {
            test::black_box({
                let d = u.digits();
                let u = U128::from(d);
                //u + u;
            });
        }
    })
}