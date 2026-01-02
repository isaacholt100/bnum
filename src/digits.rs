use core::mem::transmute;

#[repr(C)]
pub struct Digits<const DIGIT_BYTES: usize, const N: usize, const M: usize = 1, const E: usize = 0>([[u8; N]; M], [u8; E]);

// since we don't have generic_const_exprs, this is used for a u8 array of length N * M + E

// bytes are stored in native endian ordering to allow for transmutation
pub struct Digit<const BYTES: usize> {
    bytes: [u8; BYTES],
}

// macro_rules! digit_method {
//     { $($method: ident (self $(, $arg: ident : $ty: ty) *) -> $ret: ty;) * } => {
//         $(
//             #[inline]
//             const fn $method(self $(, $arg: $ty) *) -> $ret {
//                 unsafe {
//                     match BYTES {
//                         1 => transmute(transmute::<_, u8>(self.bytes).$method($( $arg ),* )),
//                         2 => transmute(transmute::<_, u16>(self.bytes).$method($( $arg ),* )),
//                         4 => transmute(transmute::<_, u32>(self.bytes).$method($( $arg ),* )),
//                         8 => transmute(transmute::<_, u64>(self.bytes).$method($( $arg ),* )),
//                         16 => transmute(transmute::<_, u128>(self.bytes).$method($( $arg ),* )),
//                         _ => unsafe { core::hint::unreachable_unchecked() },
//                     }
//                 }
//             }
//         )*
//     }
// }

// impl<const BYTES: usize> Digit<BYTES> {
//     const BITS: u32 = BYTES as u32 * 8;
//     const ALL_ONES: Self = Self { bytes: [u8::MAX; BYTES] };

//     #[inline]
//     const fn from_le_bytes(mut bytes: [u8; BYTES]) -> Self {
//         if cfg!(target_endian = "big") {
//             bytes.reverse();
//         }
//         Self { bytes }
//     }

//     #[inline]
//     const fn to_le_bytes(self) -> [u8; BYTES] {
//         let mut bytes = self.bytes;
//         if cfg!(target_endian = "big") {
//             bytes.reverse();
//         }
//         bytes
//     }

//     digit_method! {
//         count_ones(self) -> Exponent;
//         trailing_zeros(self) -> Exponent;
//         leading_ones(self) -> Exponent;
//         reverse_bits(self) -> Self;
//     }
// }

// impl<const DIGIT_BYTES: usize, const N: usize, const M: usize, const E: usize> Digits<DIGIT_BYTES, N, M, E> {
//     #[inline(always)]
//     const fn as_ptr(&self) -> *const u8 {
//         self.0.as_ptr() as _
//     }

//     const BYTE_LEN: usize = M * N + E;
    
//     const DIGIT_LEN: usize = Self::BYTE_LEN.div_ceil(DIGIT_BYTES);
    
//     const LAST_DIGIT_BYTES: usize = {
//         if Self::BYTE_LEN % DIGIT_BYTES == 0 {
//             DIGIT_BYTES
//         } else {
//             Self::BYTE_LEN % DIGIT_BYTES
//         }
//     };


//     const LAST_DIGIT_OFFSET: usize = Self::BYTE_LEN - Self::LAST_DIGIT_BYTES;
    
//     /// checks if index is within bounds only in debug mode
//     #[inline]
//     pub const fn get(&self, index: usize) -> Digit<DIGIT_BYTES> {
//         debug_assert!(index < Self::DIGIT_LEN);
//         if index == Self::DIGIT_LEN - 1 {
//             return self.last();
//         }
//         let mut bytes = [0; DIGIT_BYTES];
//         unsafe {
//             self.as_ptr()
//                 .add(index * DIGIT_BYTES)
//                 .copy_to_nonoverlapping(bytes.as_mut_ptr(), DIGIT_BYTES);
//         }
//         Digit::from_le_bytes(bytes)
//     }

//     #[inline]
//     pub const fn last_padded<const ONES: bool>(&self) -> Digit<DIGIT_BYTES> {
//         let mut bytes = if ONES { [u8::MAX; DIGIT_BYTES] } else { [0; DIGIT_BYTES] };
//         unsafe {
//             self.as_ptr()
//                 .add(Self::LAST_DIGIT_OFFSET)
//                 .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::LAST_DIGIT_BYTES);
//         }
//         Digit::from_le_bytes(bytes)
//     }

//     #[inline]
//     pub const fn last(&self) -> Digit<DIGIT_BYTES> {
//         self.last_padded::<false>()
//     }
// }

// impl<const DIGIT_BYTES: usize, const N: usize, const M: usize, const E: usize> Digits<DIGIT_BYTES, N, M, E> {
//     #[inline]
//     pub const fn count_ones(&self) -> Exponent {
//         let mut ones = 0;
//         let mut i = 0;
//         while i < Self::DIGIT_LEN {
//             let digit = self.get(i);
//             ones += digit.count_ones();
//             i += 1;
//         }
//         ones
//     }

//     #[inline]
//     pub const fn trailing_zeros(&self) -> Exponent {
//         let mut zeros = 0;
//         let mut i = 0;
//         while i < Self::DIGIT_LEN {
//             let digit = self.get(i);
//             let tz = digit.trailing_zeros();
//             zeros += tz;
//             if tz != Digit::<DIGIT_BYTES>::BITS {
//                 return zeros;
//             }
//             i += 1;
//         }
//         zeros
//     }

//     #[inline]
//     pub const fn leading_ones(mut self) -> Exponent {
//         let mut ones = 0;
//         let mut i = N;
//         while i > 0 {
//             i -= 1;
//             let digit = self.get(i);
//             ones += digit.leading_ones();
//             if digit != Digit::<DIGIT_BYTES>::ALL_ONES {
//                 break;
//             }
//         }
//         ones
//     }

//     // #[inline]
//     // pub const fn reverse_bits(self, out: DigitsMut<'a, T, N, M>) -> Self {
//     //     let mut out = Self::ZERO;
//     //     let mut i = 0;
//     //     while i < Self::DIGIT_LEN {
//     //         unsafe {
//     //             let d = self.as_wide_digits().get(i);
//     //             out.as_wide_digits_mut().set_be(i, d.reverse_bits());
//     //         }

//     //         i += 1;
//     //     }
//     //     out
//     // }
// }

// pub struct DigitsMut<const DIGIT_BYTES: usize, const N: usize, const M: usize> {
//     refr: &'a mut T,
// }

// impl<const DIGIT_BYTES: usize, const N: usize, const M: usize, const E: usize> DigitsMut<DIGIT_BYTES, N, M, E> {
//     const DIGIT_LEN: usize = Digits::<'a, T, DIGIT_BYTES, N, M>::DIGIT_LEN;
//     const LAST_DIGIT_BYTES: usize = Digits::<'a, T, DIGIT_BYTES, N, M>::LAST_DIGIT_BYTES;

//     #[inline(always)] 
//     pub const fn new(r: &'a mut T) -> Self {
//         Self { refr: r }
//     }

//     #[inline(always)]
//     const fn as_mut_ptr(&mut self) -> *mut u8 {
//         self.refr as *mut T as _
//     }

//     #[inline]
//     pub const unsafe fn set(&mut self, index: usize, value: Digit<DIGIT_BYTES>) {
//         debug_assert!(index < Self::DIGIT_LEN);
//         let bytes = value.to_le_bytes();
//         if index == Self::DIGIT_LEN - 1 {
//             return self.set_last(value);
//         }
//         unsafe {
//             self.as_mut_ptr()
//                 .add(index * DIGIT_BYTES)
//                 .copy_from_nonoverlapping(bytes.as_ptr(), DIGIT_BYTES);
//         }
//     }

//     #[inline]
//     pub const fn set_last(&mut self, value: Digit<DIGIT_BYTES>) {
//         let bytes = value.to_le_bytes();
//         unsafe {
//             self.as_mut_ptr()
//                 .add(Self::LAST_DIGIT_OFFSET)
//                 .copy_from_nonoverlapping(bytes.as_ptr(), Self::LAST_DIGIT_BYTES);
//         }
//     }

//     #[inline]
//     pub const unsafe fn set_be(&mut self, index: usize, value: Digit<DIGIT_BYTES>) {
//         debug_assert!(index < Self::DIGIT_LEN);
//         if index == Self::DIGIT_LEN - 1 {
//             return self.set_last_be(value);
//         }
//         let bytes = value.to_le_bytes();
//         unsafe {
//             self.as_mut_ptr()
//                 .add(Self::BYTE_LEN - (index + 1) * 16)
//                 .copy_from_nonoverlapping(bytes.as_ptr(), 16);
//         }
//     }

//     #[inline]
//     pub const fn set_last_be(&mut self, value: Digit<DIGIT_BYTES>) {
//         let bytes = value.to_le_bytes();
//         unsafe {
//             self.as_mut_ptr()
//                 .copy_from_nonoverlapping(bytes.as_ptr().add(16 - Self::LAST_DIGIT_BYTES), Self::LAST_DIGIT_BYTES);
//         }
//     }
// }