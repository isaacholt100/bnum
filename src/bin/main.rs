fn main() {
    use bint::{U128, BUint};
    /*let b = U128::from(347589374593749543749857349857993u128);
    let mut arr = [U128::MIN; 100000];
    let now = std::time::Instant::now();
    for i in 0..100000 {
        arr[i] = b.rotate_left(234343u32);
    }
    println!("{:?}", now.elapsed());
    println!("{:?}", &arr[1..2]);
    let (a, b): (i8, i8) = (-16, -7);
    println!("{:b} {:b}", a, b);
    println!("{}", !(!a * !b));
    println!("{:?}", i8::overflowing_mul(a, b));
    println!("{:b}", i8::wrapping_mul(a, b));
    println!("{:?}", u8::overflowing_mul(a as u8, b as u8));
    println!("{:b} {:b}", -8i8, -16i8);*/
    let arr = [345345345; 1000];
    let u1 = BUint::from(arr);
    let u2 = BUint::from(arr);
    let mut eq = false;
    let now = std::time::Instant::now();
    for i in 0..1 {
        if i & 1 == 0 {
            //eq = u1.eq(&u2);
            eq = u1 == u2;
        }
    }
    println!("{:?}", now.elapsed());
    let a: i8 = -100;
    let b: i8 = -5;
    println!("{}", -5i8);
    println!("{:b}", (a as i8).div_euclid(b as i8));
    use num_bigint::{BigInt, ToBigInt};

    println!("{:?}", i8::from_str_radix("-11", 2))
    //println!("{:?}", 3i32 >> -1);
    //println!("{:?} {:?} {:b}", b.bits(), b.bits_test(), b);
}