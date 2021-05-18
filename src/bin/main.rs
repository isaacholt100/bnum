fn main() {
    use bint::U128;
    let b = U128::from(347589374593749543749857349857993u128);
    let mut arr = [U128::MIN; 100000];
    let now = std::time::Instant::now();
    for i in 0..100000 {
        arr[i] = b.rotate_left(234343u32);
    }
    println!("{:?}", now.elapsed());
    println!("{:?}", &arr[1..2]);
    //println!("{:?} {:?} {:b}", b.bits(), b.bits_test(), b);
}