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

use num_bigint::BigUint;

#[bench]
fn bench_biguint_add(b: &mut Bencher) {
    let u1 = 394857209495782456444589679u128;
    let u2 = 30249568710094856749560704u128;
    b.iter(|| {
        test::black_box(BigUint::from_bytes_be(&u1.to_be_bytes()) + BigUint::from_bytes_be(&u2.to_be_bytes()));
    });
}

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