#[allow(unused)]
macro_rules! test_from_endian_slice {
	($int: ty, $endian: ident) => {
		paste::paste! {
			quickcheck::quickcheck! {
				fn [<quickcheck_ $int _from_ $endian _slice>](int: $int, pad_length: u8) -> quickcheck::TestResult {
					type Big = crate::test::types::[<$int:upper>];
					type Small = crate::test::types::$int;
					use crate::test::TestConvert;
					use crate::int::endian;

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
		
					let bytes_vec = endian::[<$endian _bytes_vec>](&bytes[..], pad_bits, pad_length);
					passed &= TestConvert::into(Big::[<from_ $endian _slice>](&bytes_vec[..])) == Some(int);
		
					let (msb, pad_range, slice_range) = endian::[<$endian _pad>](pad_length, Small::BITS);
		
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
		
					let bytes_vec = endian::[<$endian _bytes_vec>](&bytes[..], pad_bits, pad_length);
					passed &= Big::[<from_ $endian _slice>](&bytes_vec[..]) == correct;
		
					quickcheck::TestResult::from_bool(passed)
				}
			}
		}
	};
}

#[allow(unused)]
pub(crate) use test_from_endian_slice;


use alloc::vec::Vec;
use core::ops::{Range, RangeFrom};

#[allow(unused)]
pub fn be_bytes_vec(bytes: &[u8], pad_bits: u8, pad_length: usize) -> Vec<u8> {
	let mut bytes_vec = vec![pad_bits; pad_length];
	bytes_vec.append(&mut bytes.to_vec());
	bytes_vec
}

#[allow(unused)]
pub fn be_pad(pad_length: usize, _bits: u32) -> (usize, Range<usize>, RangeFrom<usize>) {
	(pad_length, 0..pad_length, pad_length..)
}

#[allow(unused)]
pub fn le_bytes_vec(bytes: &[u8], pad_bits: u8, pad_length: usize) -> Vec<u8> {
	let mut bytes_vec = bytes.to_vec();
	bytes_vec.append(&mut vec![pad_bits; pad_length]);
	bytes_vec
}

#[allow(unused)]
pub fn le_pad(pad_length: usize, bits: u32) -> (usize, Range<usize>, Range<usize>) {
	let bytes = bits as usize / 8;
	(bytes - 1 - pad_length, (bytes - pad_length)..bytes, 0..(bytes - pad_length))
}

#[allow(unused)]
macro_rules! tests {
	($int: ty) => {
		test_bignum! {
			function: <$int>::from_be(a: $int)
		}
		test_bignum! {
			function: <$int>::from_le(a: $int)
		}
		test_bignum! {
			function: <$int>::to_be(a: $int)
		}
		test_bignum! {
			function: <$int>::to_le(a: $int)
		}
	
		test_bignum! {
			function: <$int>::to_be_bytes(a: $int)
		}
	
		test_bignum! {
			function: <$int>::to_le_bytes(a: $int)
		}
	
		test_bignum! {
			function: <$int>::to_ne_bytes(a: $int)
		}
	
		test_bignum! {
			function: <$int>::from_be_bytes(a: U8ArrayWrapper<{<$int>::BITS as usize / 8}>)
		}
	
		test_bignum! {
			function: <$int>::from_le_bytes(a: U8ArrayWrapper<{<$int>::BITS as usize / 8}>)
		}
		
		test_bignum! {
			function: <$int>::from_ne_bytes(a: U8ArrayWrapper<{<$int>::BITS as usize / 8}>)
		}

		//crate::int::endian::test_from_endian_slice!($int, be);
		//crate::int::endian::test_from_endian_slice!($int, le);
	};
}

#[allow(unused)]
pub(crate) use tests;