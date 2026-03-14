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

pub(crate) const GLOBAL_OVERFLOW_CHECKS: bool = {
    match option_env!("BNUM_OVERFLOW_CHECKS") {
        Some(v) if str_eq(v, "true") => true,
        Some(v) if str_eq(v, "false") => false,
        _ => cfg!(debug_assertions), // if the environment variable is not set, fallback to using whether in release mode or not. this should never happen though as build.rs will always set the variable
    }
};

/// An enum that represents the different possible overflow behaviour for [`Integer`](crate::Integer).
/// * The `Wrap` variant specifies that arithmetic operations should wrap around on overflow.
/// * The `Panic` variant specifies that arithmetic operations should panic on overflow.
/// * The `Saturate` variant specifies that arithmetic operations should saturate to the integer type's maximum or minimum value on overflow.
/// For more details on overflow behaviour, see the [`Integer`](crate::Integer) documentation.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum OverflowMode {
    Wrap = 0,
    Panic = 1,
    Saturate = 2,
}

impl OverflowMode {
    /// The default overflow mode, which is the mode used for [`Integer`](crate::Integer) types when the const-generic parameter `OM` is not explicitly specified. 
    /// 
    /// The value of `DEFAULT` is controlled by the [`overflow-checks` flag](https://doc.rust-lang.org/cargo/reference/profiles.html#overflow-checks): if overflow checks are enabled, then `DEFAULT` is `Self::Panic`; if overflow checks are disabled, then `DEFAULT` is `Self::Wrap`.
    pub const DEFAULT: Self = if GLOBAL_OVERFLOW_CHECKS {
        OverflowMode::Panic
    } else {
        OverflowMode::Wrap
    };

    #[inline]
    pub(crate) const fn to_u8(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_eq() {
        assert!(str_eq("hello", "hello"));
        assert!(!str_eq("hello", "world"));
        assert!(str_eq("", ""));
        assert!(str_eq("a", "a"));
        assert!(str_eq("hi there", "hi there"));
        assert!(!str_eq("hello", "helloo"));
        assert!(!str_eq("", "a"));
    }

    #[test]
    fn test_overflow_mode_to_u8() {
        assert_eq!(OverflowMode::Wrap.to_u8(), 0);
        assert_eq!(OverflowMode::Panic.to_u8(), 1);
        assert_eq!(OverflowMode::Saturate.to_u8(), 2);
    }
}