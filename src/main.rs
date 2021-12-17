use bint::{Matrix, Vector};

fn main() {
    /*println!("{}", bint::Float::<8, 52>::DIGITS);
    println!("{:b}", f32::EPSILON.to_bits());*/
    let a = f32::from_bits(i32::MAX as u32 - 11);
    let b = f32::from_bits(i32::MAX as u32 - 1);
    println!("{:032b}", (a - b).to_bits());
    println!("a: {:032b}", a.to_bits());
    println!("b: {:032b}", b.to_bits());
    /*println!("{:b}", f64::NAN.to_bits());
    println!("{:b}", (f32::from_bits(1) as f64).to_bits());*/
    let i = 0b01111111101111111111110010001010u32;
    println!("{:032b}", i);
    println!("{:032b}", i.wrapping_add(885));
    let (f1, f2) = (f32::from_bits(i), f32::from_bits(i.wrapping_add(885)));
    assert!(f1.is_nan() && f2.is_nan());
    println!("{:032b}", (f1 - f2).to_bits());

    println!("{}", (f64::from_bits(f64::MIN_POSITIVE.to_bits() + 1) - f64::MIN_POSITIVE).is_subnormal());

    //assert_eq!(5, int_parser::test_proc!(0x80000038479827896789347569873459867349506708937458967395476937458967389054763907456973589476893475689734598673894576893745896738947596789374568));
    let m1: Matrix<i32, 4, 2> = [Vector::from([0, 2, 4, 6]), Vector::from([1, 3, 5, 7])].into();
    let m2: Matrix<i32, 2, 3> = [Vector::from([1, 4]), Vector::from([2, 5]), [3,6].into()].into();
    use core::ops::Mul;
    let m3 = m1.mul(m2);
    println!("{:?}", m3);

    let m1: Matrix<i32, 3, 3> = [Vector::from([1, 0, 0]), Vector::from([4, 0, 0]), Vector::from([0, 1, 0])].into();
    assert!(m1.is_rref());

    let m2: Matrix<i32, 3, 5> = [Vector::from([0, 0, 0]), Vector::from([1, 0, 0]), Vector::from([0, 1, 0]), Vector::from([0, 0, 1]), Vector::from([2, 3, 4])].into();
    assert!(m2.is_rref());

    let m3: Matrix<i32, 4, 4> = [Vector::from([1, 0, 0, 0]), Vector::from([0, 1, 0, 0]), Vector::from([2, 3, 0, 0]), Vector::from([0, 0, 1, 0])].into();
    assert!(m3.is_rref());

    let m4: Matrix<i32, 4, 3> = [Vector::from([0, 1, 0, 0]), Vector::from([0, 4, 0, 0]), Vector::from([0, 0, 1, 0])].into();
    assert!(!m4.is_rref());
}