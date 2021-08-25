fn main() {
    /*println!("{}", bint::Float::<8, 52>::DIGITS);
    println!("{:b}", f32::EPSILON.to_bits());*/
    println!("{:e}", f64::from_bits(0xFFFFFFFFFFFFF));
    println!("{:E}", (f64::from_bits(0xFFFFFFFFFFFFF) + f64::from_bits(0xFFFFFFFFFFFFF)));
    println!("{:e}", f64::MIN_POSITIVE);
    println!("{:064b}", (f64::from_bits(0xFFFFFFFFFFFFF) + f64::from_bits(0x10000000000000)).to_bits());
}

fn factors(n: u16) -> Vec<u16> {
    let mut out = Vec::new();
    for i in 1..=n {
        if n % i == 0 {
            out.push(i);
        }
    }
    out
}