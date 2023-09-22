
fn main() {
    // let a: u128 = x324534;
    // let a = lit_parser::n!(x234987234x24);
    // println!("{}", a);
    for i in 126..=126 {
        println!("{}, {}", log10_2e(i), 2.0f64.powi(i as i32).log10().floor());
    }
}

fn log10_2e(e: u128) -> u128 {
    let mut pow = 0;
    let mut current_pow = 0;
    let mut mod10 = 1;
    for i in 0..e {
        mod10 *= 2;
        if mod10 >= 10 {
            current_pow += 1;
            mod10 %= 10;
            if current_pow > pow {
                pow = current_pow;
                current_pow = 0;
            }
        }
        // println!("{}", mod10);
        // println!("{}", current_pow);
        // println!("{}\n", pow);
    }
    pow
}