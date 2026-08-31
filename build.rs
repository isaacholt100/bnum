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

    // `Digits<u128>` needs a 64-by-64-to-128-bit partial product. On these
    // default targets LLVM lowers that operation to a `__multi3` libcall rather
    // than inline instructions, so overflowing multiplication accumulates in
    // u32 digits instead. Keep this exception list narrow: it is based on the
    // generated code, not on pointer width or language-level u128 support.
    // wasm keeps i64 halves but has no i64 high-multiply; default sparc64 lacks
    // the required high-multiply instruction.
    println!("cargo::rustc-check-cfg=cfg(no_widening_u128_mul)");
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")
        .expect("Cargo must set CARGO_CFG_TARGET_ARCH for build scripts");
    let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY")
        .expect("Cargo must set CARGO_CFG_TARGET_FAMILY for build scripts");
    if target_family.split(',').any(|family| family == "wasm")
        || target_arch == "sparc64"
    {
        println!("cargo::rustc-cfg=no_widening_u128_mul");
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
