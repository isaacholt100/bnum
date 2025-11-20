use crate::{Integer, Uint, Int};
use crate::Byte;

use crate::Exponent;

/// Associated constants.
impl<const S: bool, const N: usize, const OM: u8> Integer<S, N, OM> {
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
    pub const BITS: Exponent = N as Exponent * Byte::BITS;

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
    pub const BYTES: Exponent = Self::BITS.div_ceil(8);

    // / The value `0`.
    // / 
    // / # Examples
    // / 
    // / ```
    // / use bnum::types::{U2048, I256};
    // / 
    // / assert_eq!(U2048::ZERO.count_zeros(), 2048);
    // / assert_eq!(n!().count_ones(), 0);
    // / ```
    pub(crate) const ZERO: Self = Self::from_bytes([0; N]);


    // / The value `1`.
    // / 
    // / # Examples
    // / 
    // / ```
    // / use bnum::types::{U1024, I1024};
    // / 
    // / assert_eq!(U1024::ONE.trailing_ones(), 1);
    // / assert_eq!(I1024::ONE.leading_zeros(), 1023);
    // / ```
    pub(crate) const ONE: Self = Self::from_byte(1);

    pub(crate) const ALL_ONES: Self = Self::from_bytes([0xFF; N]);

    /// The minimum value that this type can represent. For unsigned integers, this is `0`. For signed integers, this is `-2^(Self::BITS - 1)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::types::{U512, I512};
    /// 
    /// assert_eq!(U512::MIN, n!(0));
    /// assert_eq!(I512::MIN.trailing_zeros(), 511); // memory representation is 100...0
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
    /// use bnum::types::{U256, I256};
    /// 
    /// assert_eq!(U256::MAX.not(), U256::MIN); // memory representation is 111...1
    /// assert_eq!(I256::MAX.not(), I256::MIN); // memory representation is 011...1
    /// ```
    pub const MAX: Self = Self::MIN.not();
}

/// (Signed integers only.) Associated constants.
impl<const N: usize, const OM: u8> Int<N, OM> {
    // / The value `-1`.
    // / 
    // / # Examples
    // / 
    // / ```
    // / use bnum::types::I256;
    // / 
    // / assert_eq!(I256::NEG_ONE.count_ones(), 256); // memory representation is 111...1
    // / ```
    pub(crate) const NEG_ONE: Self = Uint::MAX.cast_signed();
}