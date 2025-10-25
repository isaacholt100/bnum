use super::{Integer, Uint};
use crate::Byte;

use crate::ExpType;

/// Associated constants.
impl<const S: bool, const N: usize> Integer<S, N> {
    /// The total number of bits that this type contains.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::types::{U512, I1024};
    /// 
    /// assert_eq!(U512::BITS, 512);
    /// assert_eq!(I1024::BITS, 1024);
    /// ```
    pub const BITS: ExpType = N as ExpType * 8;

    /// The total number of bytes that this type contains.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::types::{U256, I512};
    ///
    /// assert_eq!(U256::BYTES, 32); // 256 / 8 = 32
    /// assert_eq!(I512::BYTES, 64); // 512 / 8 = 64
    /// ```
    pub const BYTES: ExpType = Self::BITS.div_ceil(8);

    /// The value `0`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::types::{U2048, I256};
    /// 
    /// assert_eq!(U2048::ZERO.count_zeros(), 2048);
    /// assert_eq!(I256::ZERO.count_ones(), 256);
    /// ```
    pub const ZERO: Self = Self::from_bytes([0; N]);


    /// The value `1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::types::{U1024, I2048};
    /// 
    /// assert_eq!(U1024::ONE.count_ones(), 1);
    /// assert_eq!(I2048::ONE.count_zeros(), 1);
    /// ```
    pub const ONE: Self = Self::from_byte(1);

    pub(crate) const ALL_ONES: Self = Self::from_bytes([0xFF; N]);

    /// The minimum value that this type can represent. For unsigned integers, this is `0`. For signed integers, this is `-2^(Self::BITS - 1)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::types::{U512, I1024};
    /// 
    /// assert_eq!(U512::MIN, U512::ZERO);
    /// assert_eq!(I1024::MIN.trailing_zeros(), 1023); // memory representation is 100...0
    /// ```
    pub const MIN: Self = if S {
        let mut bytes = [0; N];
        bytes[N - 1] = 1 << (Byte::BITS - 1);
        Self::from_bytes(bytes)
    } else {
        Self::ZERO
    };

    /// The maximum value that this type can represent. For unsigned integers, this is `2^Self::BITS - 1`. For signed integers, this is `2^(Self::BITS - 1) - 1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::types::{U256, I512};
    /// 
    /// assert_eq!(U256::MAX.not(), U256::MIN); // memory representation is 111...1
    /// assert_eq!(I512::MAX.not(), I512::MIN); // memory representation is 011...1
    /// ```
    pub const MAX: Self = Self::MIN.not();
}

impl<const N: usize> crate::Int<N> {
    /// The value `-1`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::types::I256;
    /// 
    /// assert_eq!(I256::NEG_ONE.count_ones(), 256); // memory representation is 111...1
    /// ```
    pub const NEG_ONE: Self = Uint::MAX.cast_signed();
}