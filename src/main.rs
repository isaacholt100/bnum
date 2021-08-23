fn main() {
    println!("{}", bint::Float::<8, 52>::DIGITS);
    println!("{:b}", f32::EPSILON.to_bits());
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