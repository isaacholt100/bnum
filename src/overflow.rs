const fn str_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

pub const GLOBAL_OVERFLOW_CHECKS: bool = {
    match option_env!("BNUM_OVERFLOW_CHECKS") {
        Some(v) if str_eq(v, "true") => true,
        Some(v) if str_eq(v, "false") => false,
        _ => cfg!(debug_assertions), // if the environment variable is not set, fallback to using whether in release mode or not. this should never happen though as build.rs will always set the variable
    }
};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum OverflowMode {
    Wrapping = 0,
    Panicking = 1,
    Saturating = 2,
}

impl OverflowMode {
    pub const DEFAULT: Self = if GLOBAL_OVERFLOW_CHECKS {
        OverflowMode::Panicking
    } else {
        OverflowMode::Wrapping
    };

    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => OverflowMode::Wrapping,
            1 => OverflowMode::Panicking,
            2 => OverflowMode::Saturating,
            _ => panic!("invalid overflow mode value"),
        }
    }

    #[inline]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
