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
	};
}

#[allow(unused)]
pub(crate) use tests;