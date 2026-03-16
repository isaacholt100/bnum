fn is_nightly() -> bool {
    match std::process::Command::new("rustc")
        .arg("--version")
        .output()
    {
        Ok(output) => {
            String::from_utf8_lossy(&output.stdout).contains("nightly")
        },
        Err(_) => {
            false
        },
    }
}


fn main() {
    println!("cargo::rustc-check-cfg=cfg(nightly)");
    if is_nightly() {
        println!("cargo::rustc-cfg=nightly");
    }


    const {
        assert!(255u8.checked_add(1).is_none());
    };
    let value = if std::panic::catch_unwind(|| {
        #[allow(arithmetic_overflow)]
        let _ = 255u8 + 1; // checks if overflow checks are enabled
    })
    .is_err()
    {
        "true"
    } else {
        "false"
    };
    println!("cargo::rustc-env=BNUM_OVERFLOW_CHECKS={}", value);
    
}
