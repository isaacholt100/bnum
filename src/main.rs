fn main() {
    use std::io::Write;
    let mut f = std::fs::File::create("radix_bases.rs").unwrap();

    for &bits in &[32, 64] {
        let max = if bits < 64 {
            (1 << bits) - 1
        } else {
            std::u64::MAX
        };

        writeln!(f, "#[deny(overflowing_literals)]").unwrap();
        writeln!(
            f,
            "pub(crate) static BASES_{bits}: [(u{bits}, usize); 257] = [",
            bits = bits
        ).unwrap();
        for radix in 0u64..257 {
            let (base, power) = if radix == 0 || radix.is_power_of_two() {
                (0, 0)
            } else {
                let mut power = 1;
                let mut base = radix;

                while let Some(b) = base.checked_mul(radix) {
                    if b > max {
                        break;
                    }
                    base = b;
                    power += 1;
                }
                (base, power)
            };
            writeln!(f, "    ({}, {}), // {}", base, power, radix).unwrap();
        }
        writeln!(f, "];").unwrap();
    }
}