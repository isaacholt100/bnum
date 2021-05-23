use super::BUint;
use crate::digit;

macro_rules! as_int {
    ($method: ident, $int: ty) => {
        pub const fn $method(&self) -> $int {
            (&self.digits)[0] as $int
        }
    };
}

impl<const N: usize> BUint<N> {
    as_int!(as_u8, u8);
    as_int!(as_u16, u16);
    as_int!(as_u32, u32);
    pub const fn as_u64(&self) -> u64 {
        (&self.digits)[0]
    }
    pub const fn as_u128(&self) -> u128 {
        digit::to_double_digit((&self.digits)[1], (&self.digits)[0])
    }
    pub const fn as_usize(&self) -> usize {
        self.as_u128() as usize
    }

    as_int!(as_i8, i8);
    as_int!(as_i16, i16);
    as_int!(as_i32, i32);
    as_int!(as_i64, i64);
    pub const fn as_i128(&self) -> i128 {
        self.as_u128() as i128
    }
    pub const fn as_isize(&self) -> isize {
        self.as_u128() as isize
    }
    pub fn as_f32(&self) -> f32 {
        let mantissa = self.to_mantissa();
        let exp = self.bits() - last_set_bit(mantissa) as usize;

        if exp > f32::MAX_EXP as usize {
            f32::INFINITY
        } else {
            (mantissa as f32) * 2f32.powi(exp as i32)
        }
    }
    pub fn as_f64(&self) -> f64 {
        let mantissa = self.to_mantissa();
        let exp = self.bits() - last_set_bit(mantissa) as usize;

        if exp > f64::MAX_EXP as usize {
            f64::INFINITY
        } else {
            (mantissa as f64) * 2f64.powi(exp as i32)
        }
    }
}

const fn last_set_bit(n: u64) -> u8 {
    64 - n.leading_zeros() as u8
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn test_as_u8() {
        let u = 458937495794835975u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u8(), u as u8);
    }
    #[test]
    fn test_as_u16() {
        let u = 45679457045646u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u16(), u as u16);
    }
    #[test]
    fn test_as_u32() {
        let u = 9475697398457690379876u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u32(), u as u32);
    }
    #[test]
    fn test_as_u64() {
        let u = 987927348957345930475972439857u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u64(), u as u64);
    }
    #[test]
    fn test_as_u128() {
        let u = 49576947589673498576905868576485690u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_u128(), u as u128);
    }
    #[test]
    fn test_as_usize() {
        let u = 309485608560934564564568456u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_usize(), u as usize);
    }

    #[test]
    fn test_as_i8() {
        let u = 456759876u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i8(), u as i8);
    }
    #[test]
    fn test_as_i16() {
        let u = 9458769456904856u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i16(), u as i16);
    }
    #[test]
    fn test_as_i32() {
        let u = 95792684579875345u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i32(), u as i32);
    }
    #[test]
    fn test_as_i64() {
        let u = 4586745698783453459756456u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i64(), u as i64);
    }
    #[test]
    fn test_as_i128() {
        let u = 232030679846578450968409568098465u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_i128(), u as i128);
    }
    #[test]
    fn test_as_isize() {
        let u = 4568094586492858767245068445987u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_isize(), u as isize);
    }

    #[test]
    fn test_as_f32() {
        let u = 35478975973468456798569u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_f32(), u as f32);
    }
    #[test]
    fn test_as_f64() {
        let u = 896286490745687459674865u128;
        let uint = U128::from(u);
        assert_eq!(uint.as_f64(), u as f64);
    }
}