use crate::Float;

#[inline]
const fn str_eq_case_insensitive(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut i = 0;
    while i < a.len() {
        let a_lower = a[i].to_ascii_lowercase();
        let b_lower = b[i].to_ascii_lowercase();
        if a_lower != b_lower {
            return false;
        }
        i += 1;
    }
    true
}

impl<const W: usize, const MB: usize> Float<W, MB> {
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ()> {
        if radix < 2 || radix > 36 {
            return Err(());
        }
        if !radix.is_power_of_two() {
            return Err(());
        }
        if src.is_empty() {
            return Err(());
        }
        let (is_negative, src) = match src.as_bytes()[0] {
            b'+' => (false, src.split_at(1).1),
            b'-' => (true, src.split_at(1).1),
            _ => (false, src),
        };
        if str_eq_case_insensitive(src, "inf") || str_eq_case_insensitive(src, "infinity") {
            return Ok(if is_negative { Self::NEG_INFINITY } else { Self::INFINITY });
        }
        if str_eq_case_insensitive(src, "nan") {
            return Ok(if is_negative { Self::NAN } else { -Self::NAN });
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_eq_case_insensitive() {
        assert!(str_eq_case_insensitive("inf", "INF"));
        assert!(str_eq_case_insensitive("infinity", "INFINITY"));
        assert!(str_eq_case_insensitive("nan", "NAN"));
        assert!(str_eq_case_insensitive("NaN", "NAN"));
        assert!(str_eq_case_insensitive("Infinity", "INFINITy"));
        assert!(!str_eq_case_insensitive("inf", "infinity"));
        assert!(!str_eq_case_insensitive("inf", "nan"));
        assert!(!str_eq_case_insensitive("infinity", "nan"));   
    }
}
