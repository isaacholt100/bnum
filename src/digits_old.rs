use core::cmp::PartialEq;
use core::mem::transmute;

use crate::{Exponent, Integer};

// since we don't have generic_const_exprs, this is used for a u8 array of length N * M + E

// bytes are stored in native endian ordering to allow for transmutation
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Digit<const BYTES: usize> {
    bytes: [u8; BYTES],
}

const fn force_len<const N: usize, const M: usize>(bytes: [u8; N]) -> [u8; M] {
    debug_assert!(N == M);
    unsafe { core::mem::transmute_copy(&bytes) }
    // let ptr = bytes.as_ptr() as *const u8;
    // unsafe { *(ptr as *const [u8; M]) }
}

macro_rules! digit_method {
    { $method: ident (self $(, $arg: ident : $ty: ty) *) -> $ret: ty; } => {
        #[inline]
        pub const fn $method(self $(, $arg: $ty) *) -> $ret {
            // TODO: replace these with transmute_copy's, will mean less code, and will also work for other methods which take extra args of type Self
            // match BYTES {
            //     1 => Self::from_ne_bytes(force_len(u8::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ).to_ne_bytes())),
            //     2 => Self::from_ne_bytes(force_len(u16::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ).to_ne_bytes())),
            //     4 => Self::from_ne_bytes(force_len(u32::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ).to_ne_bytes())),
            //     8 => Self::from_ne_bytes(force_len(u64::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ).to_ne_bytes())),
            //     16 => Self::from_ne_bytes(force_len(u128::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ).to_ne_bytes())),
            //     _ => unsafe { core::hint::unreachable_unchecked() },
            // }
            use core::mem::transmute_copy;
            unsafe {
                match BYTES {
                    1 => transmute_copy(&u8::$method(transmute_copy(&self), $( &$arg ),* )),
                    2 => transmute_copy(&u16::$method(transmute_copy(&self), $( &$arg ),* )),
                    4 => transmute_copy(&u32::$method(transmute_copy(&self), $( &$arg ),* )),
                    8 => transmute_copy(&u64::$method(transmute_copy(&self), $( &$arg ),* )),
                    16 => transmute_copy(&u128::$method(transmute_copy(&self), $( &$arg ),* )),
                    _ => unsafe { core::hint::unreachable_unchecked() },
                }
            }
        }
    };
    // { $method: ident (self $(, $arg: ident : $ty: ty) *) -> $ret: ty; } => {
    //     #[inline]
    //     pub const fn $method(self $(, $arg: $ty) *) -> $ret {
    //         unsafe {
    //             match BYTES {
    //                 1 => u8::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ),
    //                 2 => u16::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ),
    //                 4 => u32::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ),
    //                 8 => u64::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ),
    //                 16 => u128::from_ne_bytes(force_len(self.bytes)).$method($( $arg ),* ),
    //                 _ => core::hint::unreachable_unchecked(),
    //             }
    //         }
    //     }
    // };
}

macro_rules! digit_methods {
    { $($method: ident (self $(, $arg: ident : $ty: ty) *) -> $ret: ty;)* } => {
        $(
            digit_method! { $method (self $(, $arg : $ty) *) -> $ret; }
        )*
    };
}

macro_rules! digit_ops {
    { $($op: tt : $method: ident (self, $arg: ident : $ty: ty) -> $ret: ty; )* } => {
        $(
            #[inline]
            pub const fn $method(self, $arg: $ty) -> $ret {
                use core::mem::transmute_copy;
                unsafe {
                    match BYTES {
                        1 => transmute_copy(&(transmute_copy::<_, u8>(&self) $op transmute_copy::<_, _>(&$arg))),
                        2 => transmute_copy(&(transmute_copy::<_, u16>(&self) $op transmute_copy::<_, _>(&$arg))),
                        4 => transmute_copy(&(transmute_copy::<_, u32>(&self) $op transmute_copy::<_, _>(&$arg))),
                        8 => transmute_copy(&(transmute_copy::<_, u64>(&self) $op transmute_copy::<_, _>(&$arg))),
                        16 => transmute_copy(&(transmute_copy::<_, u128>(&self) $op transmute_copy::<_, _>(&$arg))),
                        _ => unsafe { core::hint::unreachable_unchecked() },
                    }
                }
            }
        )*
    }
}

impl<const BYTES: usize> Digit<BYTES> {
    const BITS: u32 = BYTES as u32 * 8;
    const ALL_ONES: Self = Self {
        bytes: [u8::MAX; BYTES],
    };

    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; BYTES]) -> Self {
        Self { bytes }
    }

    #[inline]
    const fn from_le_bytes(mut bytes: [u8; BYTES]) -> Self {
        if cfg!(target_endian = "big") {
            bytes.reverse();
        }
        Self { bytes }
    }

    #[inline]
    const fn to_le_bytes(self) -> [u8; BYTES] {
        let mut bytes = self.bytes;
        if cfg!(target_endian = "big") {
            bytes.reverse();
        }
        bytes
    }

    #[inline]
    const fn eq(&self, other: &Self) -> bool {
        use core::mem::transmute_copy;
        unsafe {
            match BYTES {
                1 => u8::from_ne_bytes(force_len(self.bytes)) == u8::from_ne_bytes(force_len(other.bytes)),
                2 => u16::from_ne_bytes(force_len(self.bytes)) == u16::from_ne_bytes(force_len(other.bytes)),
                4 => u32::from_ne_bytes(force_len(self.bytes)) == u32::from_ne_bytes(force_len(other.bytes)),
                8 => u64::from_ne_bytes(force_len(self.bytes)) == u64::from_ne_bytes(force_len(other.bytes)),
                16 => u128::from_ne_bytes(force_len(self.bytes)) == u128::from_ne_bytes(force_len(other.bytes)),
                _ => core::hint::unreachable_unchecked(),
            }
        }
    }

    #[inline]
    const fn gt(&self, other: &Self) -> bool {
        use core::mem::transmute_copy;
        unsafe {
            match BYTES {
                1 => u8::from_ne_bytes(force_len(self.bytes)) > u8::from_ne_bytes(force_len(other.bytes)),
                2 => u16::from_ne_bytes(force_len(self.bytes)) > u16::from_ne_bytes(force_len(other.bytes)),
                4 => u32::from_ne_bytes(force_len(self.bytes)) > u32::from_ne_bytes(force_len(other.bytes)),
                8 => u64::from_ne_bytes(force_len(self.bytes)) > u64::from_ne_bytes(force_len(other.bytes)),
                16 => u128::from_ne_bytes(force_len(self.bytes)) > u128::from_ne_bytes(force_len(other.bytes)),
                _ => core::hint::unreachable_unchecked(),
            }
        }
    }

    #[inline]
    const fn lt(&self, other: &Self) -> bool {
        use core::mem::transmute_copy;
        unsafe {
            match BYTES {
                1 => u8::from_ne_bytes(force_len(self.bytes)) < u8::from_ne_bytes(force_len(other.bytes)),
                2 => u16::from_ne_bytes(force_len(self.bytes)) < u16::from_ne_bytes(force_len(other.bytes)),
                4 => u32::from_ne_bytes(force_len(self.bytes)) < u32::from_ne_bytes(force_len(other.bytes)),
                8 => u64::from_ne_bytes(force_len(self.bytes)) < u64::from_ne_bytes(force_len(other.bytes)),
                16 => u128::from_ne_bytes(force_len(self.bytes)) < u128::from_ne_bytes(force_len(other.bytes)),
                _ => core::hint::unreachable_unchecked(),
            }
        }
    }

    #[inline]
    const fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    digit_methods! {
        count_ones(self) -> Exponent;
        trailing_zeros(self) -> Exponent;
        leading_ones(self) -> Exponent;
        leading_zeros(self) -> Exponent;
        reverse_bits(self) -> Self;
    }

    // digit_ops! {
    //     & : bitand(self, rhs: Self) -> Self;
    //     | : bitor(self, rhs: Self) -> Self;
    //     ^ : bitxor(self, rhs: Self) -> Self;
    // }
    // digit_method! { reverse_bits(self) -> Self; }
    // digit_method! { shl(self, rhs: Exponent) -> Self; }
}

#[repr(C)] // very important this is repr(C) to ensure no padding between arrays
pub struct Digits<const DIGIT_BYTES: usize, const N: usize, const M: usize = 1, const E: usize = 0>(
    [[u8; N]; M],
    [u8; E],
);

impl<const DIGIT_BYTES: usize, const N: usize> Digits<DIGIT_BYTES, N, 1, 0> {
    #[inline(always)]
    pub const fn from_integer<const S: bool, const B: usize, const OM: u8>(
        int: Integer<S, N, B, OM>,
    ) -> Self {
        debug_assert!(
            DIGIT_BYTES == 1
                || DIGIT_BYTES == 2
                || DIGIT_BYTES == 4
                || DIGIT_BYTES == 8
                || DIGIT_BYTES == 16
        );
        const {
            assert!(core::mem::size_of::<Self>() == core::mem::size_of::<Integer<S, N, B, OM>>());
            assert!(core::mem::align_of::<Self>() == core::mem::align_of::<Integer<S, N, B, OM>>()); // Integer is repr(transparent) over [u8; N], so alignment is 1. 
            // alignment of a repr(C) struct is max alignment of its fields, so Digits also has alignment 1
        }
        // SAFETY: transmuting between arrays of same size
        unsafe { core::mem::transmute_copy(&int) }
    }

    #[inline(always)]
    pub const fn from_integer_ref<'a, const S: bool, const B: usize, const OM: u8>(
        int: &'a Integer<S, N, B, OM>,
    ) -> &'a Self {
        debug_assert!(
            DIGIT_BYTES == 1
                || DIGIT_BYTES == 2
                || DIGIT_BYTES == 4
                || DIGIT_BYTES == 8
                || DIGIT_BYTES == 16
        );
        unsafe { &*(int as *const Integer<S, N, B, OM> as *const Self) }
    }

    #[inline(always)]
    pub const fn to_integer<const S: bool, const B: usize, const OM: u8>(
        self,
    ) -> Integer<S, N, B, OM> {
        const {
            assert!(core::mem::size_of::<Self>() == core::mem::size_of::<Integer<S, N, B, OM>>());
            assert!(core::mem::align_of::<Self>() == core::mem::align_of::<Integer<S, N, B, OM>>()); // Integer is repr(transparent) over [u8; N], so alignment is 1. 
            // alignment of a repr(C) struct is max alignment of its fields, so Digits also has alignment 1
        }
        // SAFETY: transmuting between arrays of same size
        unsafe { core::mem::transmute_copy(&self) }
    }
}

impl<const DIGIT_BYTES: usize, const N: usize, const M: usize, const E: usize>
    Digits<DIGIT_BYTES, N, M, E>
{
    #[inline(always)]
    const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr() as _
    }

    #[inline(always)]
    const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr() as _
    }

    const ALL_ZEROS: Self = Self([[0; N]; M], [0; E]);

    const BYTE_LEN: usize = M * N + E;

    const DIGIT_LEN: usize = Self::BYTE_LEN.div_ceil(DIGIT_BYTES);

    const LAST_DIGIT_BYTES: usize = {
        if Self::BYTE_LEN % DIGIT_BYTES == 0 {
            DIGIT_BYTES
        } else {
            Self::BYTE_LEN % DIGIT_BYTES
        }
    };

    const LAST_DIGIT_PAD_BYTES: usize = DIGIT_BYTES - Self::LAST_DIGIT_BYTES;

    const LAST_DIGIT_OFFSET: usize = Self::BYTE_LEN - Self::LAST_DIGIT_BYTES;

    /// checks if index is within bounds only in debug mode
    #[inline]
    pub const fn get(&self, index: usize) -> Digit<DIGIT_BYTES> {
        debug_assert!(index < Self::DIGIT_LEN);
        if DIGIT_BYTES == 1 {
            return unsafe { *(self.as_ptr().add(index) as *const _) };
        }
        if index == Self::DIGIT_LEN - 1 {
            return self.last();
        }
        let mut bytes = [0; DIGIT_BYTES];
        unsafe {
            self.as_ptr()
                .add(index * DIGIT_BYTES)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), DIGIT_BYTES);
        }
        Digit::from_le_bytes(bytes)
    }

    #[inline]
    pub const fn last_padded<const ONES: bool>(&self) -> Digit<DIGIT_BYTES> {
        let mut bytes = if ONES {
            [u8::MAX; DIGIT_BYTES]
        } else {
            [0; DIGIT_BYTES]
        };
        unsafe {
            self.as_ptr()
                .add(Self::LAST_DIGIT_OFFSET)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::LAST_DIGIT_BYTES);
        }
        Digit::from_le_bytes(bytes)
    }

    #[inline]
    pub const fn last(&self) -> Digit<DIGIT_BYTES> {
        self.last_padded::<false>()
    }

    #[inline]
    pub const fn set(&mut self, index: usize, value: Digit<DIGIT_BYTES>) {
        debug_assert!(index < Self::DIGIT_LEN);
        let bytes = value.to_le_bytes();
        if DIGIT_BYTES == 1 {
            unsafe { *(self.as_mut_ptr().add(index) as *mut _) = value };
            return;
        }
        if index == Self::DIGIT_LEN - 1 {
            return self.set_last(value);
        }
        unsafe {
            self.as_mut_ptr()
                .add(index * DIGIT_BYTES)
                .copy_from_nonoverlapping(bytes.as_ptr(), DIGIT_BYTES);
        }
    }

    #[inline]
    pub const fn set_last(&mut self, value: Digit<DIGIT_BYTES>) {
        let bytes = value.to_le_bytes();
        unsafe {
            self.as_mut_ptr()
                .add(Self::LAST_DIGIT_OFFSET)
                .copy_from_nonoverlapping(bytes.as_ptr(), Self::LAST_DIGIT_BYTES);
        }
    }

    #[inline]
    pub const fn set_be(&mut self, index: usize, value: Digit<DIGIT_BYTES>) {
        debug_assert!(index < Self::DIGIT_LEN);
        if DIGIT_BYTES == 1 {
            unsafe { *(self.as_mut_ptr().add(Self::BYTE_LEN - 1 - index) as *mut _) = value };
            return;
        }
        if index == Self::DIGIT_LEN - 1 {
            return self.set_last_be(value);
        }
        let bytes = value.to_le_bytes();
        unsafe {
            self.as_mut_ptr()
                .add(Self::BYTE_LEN - (index + 1) * DIGIT_BYTES)
                .copy_from_nonoverlapping(bytes.as_ptr(), DIGIT_BYTES);
        }
    }

    #[inline]
    pub const fn set_last_be(&mut self, value: Digit<DIGIT_BYTES>) {
        let bytes = value.to_le_bytes();
        unsafe {
            self.as_mut_ptr().copy_from_nonoverlapping(
                bytes.as_ptr().add(DIGIT_BYTES - Self::LAST_DIGIT_BYTES),
                Self::LAST_DIGIT_BYTES,
            );
        }
    }
}

impl<const DIGIT_BYTES: usize, const N: usize, const M: usize, const E: usize>
    Digits<DIGIT_BYTES, N, M, E>
{
    #[inline]
    pub const fn count_ones(self) -> Exponent {
        let mut ones = 0;
        let mut i = 0;
        while i < Self::DIGIT_LEN {
            let digit = self.get(i);
            ones += digit.count_ones();
            i += 1;
        }
        ones
    }

    #[inline]
    pub const fn trailing_zeros(self) -> Exponent {
        let mut zeros = 0;
        let mut i = 0;
        while i < Self::DIGIT_LEN {
            let digit = self.get(i);
            let tz = digit.trailing_zeros();
            zeros += tz;
            if tz != Digit::<DIGIT_BYTES>::BITS {
                return zeros;
            }
            i += 1;
        }
        zeros
    }

    #[inline]
    pub const fn leading_ones(self) -> Exponent {
        let mut ones = 0;
        let mut i = Self::DIGIT_LEN;
        while i > 0 {
            i -= 1;
            let digit = self.get(i);
            ones += digit.leading_ones();
            if digit.ne(&Digit::ALL_ONES) {
                break;
            }
        }
        ones
    }

    #[inline]
    pub const fn leading_zeros(self) -> Exponent {
        let mut zeros = 0;
        let mut i = Self::DIGIT_LEN;
        while i > 0 {
            i -= 1;
            let digit = self.get(i);
            let lz = digit.leading_zeros();
            zeros += lz;
            if lz != Digit::<DIGIT_BYTES>::BITS {
                return zeros - (Self::LAST_DIGIT_PAD_BYTES as u32 * 8);
            }
        }
        zeros - (Self::LAST_DIGIT_PAD_BYTES as u32 * 8)
    }

    #[inline]
    pub const fn reverse_bits(self) -> Self {
        let mut out = Self::ALL_ZEROS;
        let mut i = 0;
        while i < Self::DIGIT_LEN {
            let d = self.get(i);
            out.set_be(i, d.reverse_bits());

            i += 1;
        }
        out
    }

    #[inline]
    pub const fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < Self::DIGIT_LEN {
            if self.get(i).ne(&other.get(i)) {
                return false;
            }
            i += 1;
        }
        true
    }

    #[inline]
    pub const fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let mut i = Self::DIGIT_LEN;
        while i > 0 {
            i -= 1;

            let a = self.get(i);
            let b = other.get(i);
            if a.gt(&b) {
                return core::cmp::Ordering::Greater;
            } else if a.lt(&b) {
                return core::cmp::Ordering::Less;
            }
        }
        core::cmp::Ordering::Equal
    }

    // #[inline]
    // pub const fn bitand(self, rhs: Self) -> Self {
    //     let mut out = Self::ALL_ZEROS;
    //     let mut i = 0;
    //     while i < Self::DIGIT_LEN {
    //         let d = self.get(i) & rhs.get(i);
    //         out.set(i, d);

    //         i += 1;
    //     }
    //     out
    // }

    // #[inline]
    // pub const fn unchecked_shl(self, rhs: Exponent) -> Self {
    //     let byte_shift = (rhs / Digit::<DIGIT_BYTES>::BITS) as usize;
    //     let bit_shift = (rhs % Digit::<DIGIT_BYTES>::BITS);
    //     let mut out = Self::ALL_ZEROS;

    //     if bit_shift == 0 {
    //         let start_index = (rhs / Digit::<DIGIT_BYTES>::BITS) as usize;
    //         let mut i = start_index;
    //         while i < Self::DIGIT_LEN {
    //             let d = self.get(i - start_index);
    //             out.set(i, d);
    //             i += 1;
    //         }
    //         out
    //     } else {
    //         let carry_shift = Digit::<DIGIT_BYTES>::BITS - bit_shift;
    //         let mut carry = 0;

    //         let start_index = (rhs / Digit::<DIGIT_BYTES>::BITS) as usize;
    //         let mut i = start_index;
    //         while i < Self::DIGIT_LEN {
    //             let d = self.get(i - start_index);
    //             out.set(i, (d << bit_shift) | carry);
    //             carry = d >> carry_shift;
    //             i += 1;
    //         }

    //         out
    //     }
    // }
}
