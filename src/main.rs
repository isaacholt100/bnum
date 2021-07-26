fn main() {
    println!("{:?}", i8::from_str_radix("-10000000", 2));
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