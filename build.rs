fn main() {
    const { assert!(255u8.checked_add(1).is_none()); };
    let value = if std::panic::catch_unwind(|| {
        #[allow(arithmetic_overflow)]
        let _ = 255u8 + 1; // checks if overflow checks are enabled
    }).is_err() {
        "true"
    } else {
        "false"
    };
    println!("cargo::rustc-env=BNUM_OVERFLOW_CHECKS={}", value);
}