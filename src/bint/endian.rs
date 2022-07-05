use super::BInt;
use crate::buint::BUint;
use crate::digit::{self, Digit, SignedDigit};
use core::mem::MaybeUninit;
use crate::doc;

macro_rules! set_digit {
    ($out_digits: ident, $i: expr, $digit: expr, $is_negative: expr, $sign_bits: expr) => {
        if $i == Self::N_MINUS_1 {
            if ($digit as SignedDigit).is_negative() == $is_negative {
                $out_digits[$i] = $digit;
            } else {
                return None;
            }
        } else if $i < N {
            $out_digits[$i] = $digit;
        } else if $digit != $sign_bits {
            return None;
        };
    }
}

#[doc=doc::endian::impl_desc!(BInt)]
impl<const N: usize> BInt<N> {
    #[doc=doc::endian::from_be!(I 256)]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
        Self::from_bits(BUint::from_be(x.bits))
    }

    #[doc=doc::endian::from_le!(I 256)]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
        Self::from_bits(BUint::from_le(x.bits))
    }

    #[doc=doc::endian::to_be!(I 256)]
    #[inline]
    pub const fn to_be(self) -> Self {
        Self::from_be(self)
    }

    #[doc=doc::endian::to_le!(I 256)]
    #[inline]
    pub const fn to_le(self) -> Self {
        Self::from_le(self)
    }

	/// Create an integer value from a slice of bytes in big endian. The value is wrapped in an `Option` as the integer represented by the slice of bytes may represent an integer large to be represented by the type.
    /// 
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros or ones at the start so that it's length equals `Self::BYTES`. It is padded with ones if the bytes represent a negative integer, otherwise it is padded with zeros.
    /// 
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless the bytes represent a non-negative integer and leading zeros from the slice can be removed until the length of the slice equals `Self::BYTES`, or if the bytes represent a negative integer and leading ones from the slice can be removed until the length of the slice equals `Self::BYTES`.
    pub const fn from_be_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        if len == 0 {
            return Some(Self::ZERO);
        }
        let is_negative = (slice[0] as i8).is_negative();
        let sign_bits = if is_negative {
            Digit::MAX
        } else {
            Digit::MIN
        };
        let mut out_digits = if is_negative {
            [Digit::MAX; N]
        } else {
            [0; N]
        };
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(len - digit::BYTES as usize - (i << digit::BYTE_SHIFT)).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
                uninit.assume_init()
            };
            let digit = Digit::from_be_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        let rem = len & (digit::BYTES as usize - 1);
        if rem == 0 {
            Some(Self::from_bits(BUint::from_digits(out_digits)))
        } else {
			let pad_byte = if is_negative {
				u8::MAX
			} else {
				0
			};
            let mut last_digit_bytes = [pad_byte; digit::BYTES as usize];
            let mut j = 0;
            while j < rem {
                last_digit_bytes[digit::BYTES as usize - rem + j] = slice[j];
                j += 1;
            }
            let digit = Digit::from_be_bytes(last_digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
			Some(Self::from_bits(BUint::from_digits(out_digits)))
        }
    }
    
	/// Creates an integer value from a slice of bytes in little endian. The value is wrapped in an `Option` as the bytes may represent an integer too large to be represented by the type.
    /// 
    /// If the length of the slice is shorter than `Self::BYTES`, the slice is padded with zeros or ones at the end so that it's length equals `Self::BYTES`. It is padded with ones if the bytes represent a negative integer, otherwise it is padded with zeros.
    /// 
    /// If the length of the slice is longer than `Self::BYTES`, `None` will be returned, unless the bytes represent a non-negative integer and trailing zeros from the slice can be removed until the length of the slice equals `Self::BYTES`, or if the bytes represent a negative integer and trailing ones from the slice can be removed until the length of the slice equals `Self::BYTES`.
	/// 
	/// For examples, see the `from_le_slice` method documentation for `BUint`.
    pub const fn from_le_slice(slice: &[u8]) -> Option<Self> {
        let len = slice.len();
        if len == 0 {
            return Some(Self::ZERO);
        }
        let is_negative = (slice[len - 1] as i8).is_negative();
        let sign_bits = if is_negative {
            Digit::MAX
        } else {
            Digit::MIN
        };
        let mut out_digits = [sign_bits; N];
        let slice_ptr = slice.as_ptr();
        let mut i = 0;
        let exact = len >> digit::BYTE_SHIFT;
        while i < exact {
            let mut uninit = MaybeUninit::<[u8; digit::BYTES as usize]>::uninit();
            let ptr = uninit.as_mut_ptr() as *mut u8;
            let digit_bytes = unsafe {
                slice_ptr.add(i << digit::BYTE_SHIFT).copy_to_nonoverlapping(ptr, digit::BYTES as usize);
                uninit.assume_init()
            };
            let digit = Digit::from_le_bytes(digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            i += 1;
        }
        if len & (digit::BYTES as usize - 1) == 0 {
            Some(Self::from_bits(BUint::from_digits(out_digits)))
        } else {
			let pad_byte = if is_negative {
				u8::MAX
			} else {
				0
			};
            let mut last_digit_bytes = [pad_byte; digit::BYTES as usize];
            let addition = exact << digit::BYTE_SHIFT;
            let mut j = 0;
            while j + addition < len {
                last_digit_bytes[j] = slice[j + addition];
                j += 1;
            }
            let digit = Digit::from_le_bytes(last_digit_bytes);
            set_digit!(out_digits, i, digit, is_negative, sign_bits);
            Some(Self::from_bits(BUint::from_digits(out_digits)))
        }
    }
	
    #[doc=doc::endian::to_be_bytes!(I 256)]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N * digit::BYTES as usize] {
        self.bits.to_be_bytes()
    }

    #[doc=doc::endian::to_le_bytes!(I 256)]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N * digit::BYTES as usize] {
        self.bits.to_le_bytes()
    }

    #[doc=doc::endian::to_ne_bytes!(I 256)]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N * digit::BYTES as usize] {
        self.bits.to_ne_bytes()
    }

    #[doc=doc::endian::from_be_bytes!(I 256)]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_be_bytes(bytes))
    }

    #[doc=doc::endian::from_le_bytes!(I 256)]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_le_bytes(bytes))
    }

    #[doc=doc::endian::from_ne_bytes!(I 256)]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N * digit::BYTES as usize]) -> Self {
        Self::from_bits(BUint::from_ne_bytes(bytes))
    }
}

#[cfg(test)]
mod tests {
	use crate::test::types::{I128, itest, utest};
    use crate::test::{U8ArrayWrapper, TestConvert, test_bignum};

	crate::int::endian::tests!(itest);
	macro_rules! test_from_endian_slice {
		($int: ty, $endian: ident) => {
			paste::paste! {
				quickcheck::quickcheck! {
					fn [<quickcheck_ $int _from_ $endian _slice>](int: $int, pad_length: u8) -> quickcheck::TestResult {
						type Big = crate::test::types::[<$int:upper>];
						type Small = crate::test::types::$int;

						if pad_length >= Small::BITS as u8 / 8 {
							return quickcheck::TestResult::discard();
						}
						let pad_length = pad_length as usize;
			
						#[allow(unused_comparisons)]
						let mut pad_bits = if int < 0 {
							u8::MAX
						} else {
							u8::MIN
						};
			
						let mut bytes = int.[<to_ $endian _bytes>]();
						let mut passed = TestConvert::into(Big::[<from_ $endian _slice>](&bytes[..])) == Some(int);
			
						let bytes_vec = [<$endian _bytes_vec>](&bytes[..], pad_bits, pad_length);
						passed &= TestConvert::into(Big::[<from_ $endian _slice>](&bytes_vec[..])) == Some(int);
			
						let (msb, pad_range, slice_range) = [<$endian _pad>](pad_length, Small::BITS);
			
						pad_bits = {
							#[allow(unused_comparisons)]
							if Small::MIN < 0 && (bytes[msb] as i8).is_negative() {
								u8::MAX
							} else {
								u8::MIN
							}
						};
			
						for item in &mut bytes[pad_range] {
							*item = pad_bits;
						}
						let correct = Some(Big::[<from_ $endian _bytes>](bytes));
						passed &= Big::[<from_ $endian _slice>](&bytes[slice_range]) == correct;
			
						let bytes_vec = [<$endian _bytes_vec>](&bytes[..], pad_bits, pad_length);
						passed &= Big::[<from_ $endian _slice>](&bytes_vec[..]) == correct;
			
						quickcheck::TestResult::from_bool(passed)
					}
				}
			}
		};
	}

	test_from_endian_slice!(itest, be);
	test_from_endian_slice!(utest, be);
	test_from_endian_slice!(itest, le);
	test_from_endian_slice!(utest, le);


	use alloc::vec::Vec;
	use core::ops::{Range, RangeFrom};

	fn be_bytes_vec(bytes: &[u8], pad_bits: u8, pad_length: usize) -> Vec<u8> {
		let mut bytes_vec = vec![pad_bits; pad_length];
		bytes_vec.append(&mut bytes.to_vec());
		bytes_vec
	}

	fn be_pad(pad_length: usize, _bits: u32) -> (usize, Range<usize>, RangeFrom<usize>) {
		(pad_length, 0..pad_length, pad_length..)
	}

	pub fn le_bytes_vec(bytes: &[u8], pad_bits: u8, pad_length: usize) -> Vec<u8> {
		let mut bytes_vec = bytes.to_vec();
		bytes_vec.append(&mut vec![pad_bits; pad_length]);
		bytes_vec
	}
	
	pub fn le_pad(pad_length: usize, bits: u32) -> (usize, Range<usize>, Range<usize>) {
		let bytes = bits as usize / 8;
		(bytes - 1 - pad_length, (bytes - pad_length)..bytes, 0..(bytes - pad_length))
	}

	#[test]
	fn from_be_slice() {
        let arr = [73, 80, 2, 24, 160, 188, 204, 45, 33, 88, 4, 68, 230, 180, 145, 32];
        assert_eq!(I128::from_be_bytes(arr), I128::from_be_slice(&arr[..]).unwrap());
        let mut arr2 = arr;
        arr2[0] = 0;
        arr2[1] = 0;
        arr2[2] = 0;
        assert_eq!(I128::from_be_bytes(arr2), I128::from_be_slice(&arr2[2..]).unwrap());
        let mut v = arr.to_vec();
        v.insert(0, 0);
        v.insert(0, 0);
        v.insert(0, 0);
        assert_eq!(I128::from_be_bytes(arr), I128::from_be_slice(&v).unwrap());
        v.push(4);
        assert_eq!(I128::from_be_slice(&v), None);

        assert_eq!(I128::from_be_slice(&[]), Some(I128::ZERO));
    }

    #[test]
    fn from_le_slice() {
        let arr = [73, 80, 2, 24, 160, 188, 204, 45, 33, 88, 4, 68, 230, 180, 145, 32];
        //assert_eq!(I128::from_le_bytes(arr), I128::from_le_slice(&arr[..]).unwrap());
        let mut arr2 = arr;
        arr2[15] = u8::MAX;
        arr2[14] = u8::MAX;
        arr2[13] = u8::MAX;
        assert_eq!(I128::from_le_bytes(arr2), I128::from_le_slice(&arr2[..13]).unwrap());
        let mut v = arr.to_vec();
        v.extend(vec![0, 0, 0, 0, 0].into_iter());
        assert_eq!(I128::from_le_bytes(arr), I128::from_le_slice(&v).unwrap());
        v.insert(0, 4);
        assert_eq!(I128::from_le_slice(&v), None);

		let mut v = arr2.to_vec();
        v.append(&mut vec![u8::MAX; 10]);
        assert_eq!(I128::from_le_bytes(arr2), I128::from_le_slice(&v).unwrap());
        v.push(100);
        assert_eq!(I128::from_le_slice(&v), None);

        assert_eq!(I128::from_le_slice(&[]), Some(I128::ZERO));
    }
}