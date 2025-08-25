fn main() {
    let value = if std::panic::catch_unwind(|| {
        #[allow(arithmetic_overflow)]
        let _ = 255u8 + 3; // checks if overflow checks are enabled
    }).is_err() {
        "true"
    } else {
        "false"
    };
    println!("cargo::rustc-env=BNUM_OVERFLOW_CHECKS={}", value);
}