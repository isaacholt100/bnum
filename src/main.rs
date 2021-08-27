fn main() {
    /*println!("{}", bint::Float::<8, 52>::DIGITS);
    println!("{:b}", f32::EPSILON.to_bits());*/
    println!("{:b}", (f64::from_bits(0xFFFFFFFFFFF) + f64::from_bits(0x7FFFFFFFFFFFFFF0)).to_bits());
    println!("{}", f64::MAX as f32);
    println!("{:b}", (f32::from_bits(1) as f64).to_bits());
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