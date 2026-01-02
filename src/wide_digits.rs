// DBL false, INCR false means working with N bytes, DBL true, INCR false means working with 2N bytes, DBL false INCR true means working with N + 1 bytes, DBL true INCR true means working with 2N + 1 bytes
// since can't use generic const exprs yet
pub struct WideDigits<const N: usize, const DBL: bool, const INCR: bool> {
    ptr: *const u8,
}

impl<const N: usize, const DBL: bool, const INCR: bool> WideDigits<N, DBL, INCR> {
    const BYTE_LEN: usize = {
        let mut len = N;
        if DBL {
            len *= 2;
        }
        if INCR {
            len += 1;
        }
        len
    };
    const DIGIT_LEN: usize = Self::BYTE_LEN.div_ceil(16);

    const U64_DIGIT_LEN: usize = Self::BYTE_LEN.div_ceil(8);

    #[inline]
    pub const fn new<T>(r: &T) -> Self {
        let ptr = r as *const T as *const u8;
        Self { ptr }
    }

    /// checks if index is within bounds only in debug mode
    #[inline]
    pub const fn get(&self, index: usize) -> u128 {
        debug_assert!(index < Self::DIGIT_LEN);
        if index == Self::DIGIT_LEN - 1 {
            return self.last();
        }
        let mut bytes = [0; 16];
        unsafe {
            // SAFETY: the debug assert above ensures index is in bounds
            self.ptr
                .add(index * 16)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), 16);
        }
        u128::from_le_bytes(bytes)
    }

    // gets big-endian digit at index. checks if index is within bounds only in debug mode. 0 is the most significant digit and is a full digit, the last digit is padded with trailing zeros
    #[inline]
    pub const fn get_be(&self, index: usize) -> u128 {
        debug_assert!(index < Self::DIGIT_LEN);
        if index == Self::DIGIT_LEN - 1 {
            return self.last_be();
        }
        let mut bytes = [0; 16];
        unsafe {
            // SAFETY: the debug assert above ensures index is in bounds
            self.ptr
                .add(Self::BYTE_LEN - (index + 1) * 16)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), 16);
        }
        u128::from_le_bytes(bytes)
    }

    #[inline]
    pub const fn u64_digit(&self, index: usize) -> u64 {
        debug_assert!(index < Self::U64_DIGIT_LEN);

        if index == Self::U64_DIGIT_LEN - 1 {
            return self.last_u64();
        }

        let mut bytes = [0; 8];
        unsafe {
            // SAFETY: the debug assert above ensures index is in bounds
            self.ptr
                .add(index * 8)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), 8);
        }
        u64::from_le_bytes(bytes)
    }

    const LAST_DIGIT_BYTES: usize = {
        if Self::BYTE_LEN % 16 == 0 {
            16
        } else {
            Self::BYTE_LEN % 16
        }
    };

    const LAST_U64_DIGIT_BYTES: usize = {
        if Self::BYTE_LEN % 8 == 0 {
            8
        } else {
            Self::BYTE_LEN % 8
        }
    };


    const LAST_DIGIT_OFFSET: usize = Self::BYTE_LEN - Self::LAST_DIGIT_BYTES;

    const LAST_U64_DIGIT_OFFSET: usize = Self::BYTE_LEN - Self::LAST_U64_DIGIT_BYTES;

    #[inline]
    pub const fn last_padded<const ONES: bool>(&self) -> u128 {
        let mut bytes = if ONES { [u8::MAX; 16] } else { [0; 16] };
        unsafe {
            self.ptr
                .add(Self::LAST_DIGIT_OFFSET)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::LAST_DIGIT_BYTES);
        }
        u128::from_le_bytes(bytes)
    }

    #[inline]
    pub const fn last_be(&self) -> u128 {
        let mut bytes = [0; 16];
        unsafe {
            self.ptr
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::LAST_DIGIT_BYTES);
        }
        u128::from_le_bytes(bytes)
    }

    #[inline]
    pub const fn last(&self) -> u128 {
        self.last_padded::<false>()
    }

    #[inline]
    pub const fn last_u64(&self) -> u64 {
        let mut bytes = [0; 8];
        unsafe {
            self.ptr
                .add(Self::LAST_U64_DIGIT_OFFSET)
                .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::LAST_U64_DIGIT_BYTES);
        }
        u64::from_le_bytes(bytes)
    }
}

// DBL false, INCR false means working with N bytes, DBL true, INCR false means working with 2N bytes, DBL false INCR true means working with N + 1 bytes, DBL true INCR true means working with 2N + 1 bytes
// since can't use generic const exprs yet
pub struct WideDigitsMut<const N: usize, const DBL: bool, const INCR: bool> {
    ptr: *mut u8,
}

impl<const N: usize, const DBL: bool, const INCR: bool> WideDigitsMut<N, DBL, INCR> {
    const LAST_DIGIT_BYTES: usize = WideDigits::<N, DBL, INCR>::LAST_DIGIT_BYTES;
    const LAST_U64_DIGIT_BYTES: usize = WideDigits::<N, DBL, INCR>::LAST_U64_DIGIT_BYTES;
    const LAST_DIGIT_OFFSET: usize = WideDigits::<N, DBL, INCR>::LAST_DIGIT_OFFSET;
    const LAST_U64_DIGIT_OFFSET: usize = WideDigits::<N, DBL, INCR>::LAST_U64_DIGIT_OFFSET;
    const DIGIT_LEN: usize = WideDigits::<N, DBL, INCR>::DIGIT_LEN;
    const U64_DIGIT_LEN: usize = WideDigits::<N, DBL, INCR>::U64_DIGIT_LEN;
    const BYTE_LEN: usize = WideDigits::<N, DBL, INCR>::BYTE_LEN;

    /// requires Self to be repr(C) so that the data is stored in contiguous memory
    #[inline]
    pub const fn new<T>(r: &mut T) -> Self {
        let ptr = r as *mut T as *mut u8;
        Self { ptr }
    }

    #[inline]
    pub const fn set(&mut self, index: usize, value: u128) {
        debug_assert!(index < Self::DIGIT_LEN);
        let bytes = value.to_le_bytes();
        if index == Self::DIGIT_LEN - 1 {
            return self.set_last(value);
        }
        unsafe {
            self.ptr
                .add(index * 16)
                .copy_from_nonoverlapping(bytes.as_ptr(), 16);
        }
    }

    #[inline]
    pub const fn set_u64_digit(&mut self, index: usize, value: u64) {
        debug_assert!(index < Self::U64_DIGIT_LEN);
        let bytes = value.to_le_bytes();
        if index == Self::U64_DIGIT_LEN - 1 {
            return self.set_last_u64(value);
        }
        unsafe {
            self.ptr
                .add(index * 8)
                .copy_from_nonoverlapping(bytes.as_ptr(), 8);
        }
    }

    #[inline]
    pub const fn set_be(&mut self, index: usize, value: u128) {
        debug_assert!(index < Self::DIGIT_LEN);
        if index == Self::DIGIT_LEN - 1 {
            return self.set_last_be(value);
        }
        let bytes = value.to_le_bytes();
        unsafe {
            // SAFETY: the debug assert above ensures index is in bounds
            self.ptr
                .add(Self::BYTE_LEN - (index + 1) * 16)
                .copy_from_nonoverlapping(bytes.as_ptr(), 16);
        }
    }

    #[inline]
    pub const fn set_last(&mut self, value: u128) {
        let bytes = value.to_le_bytes();
        unsafe {
            self.ptr
                .add(Self::LAST_DIGIT_OFFSET)
                .copy_from_nonoverlapping(bytes.as_ptr(), Self::LAST_DIGIT_BYTES);
        }
    }

    #[inline]
    pub const fn set_last_u64(&mut self, value: u64) {
        let bytes = value.to_le_bytes();
        unsafe {
            self.ptr
                .add(Self::LAST_U64_DIGIT_OFFSET)
                .copy_from_nonoverlapping(bytes.as_ptr(), Self::LAST_U64_DIGIT_BYTES);
        }
    }

    #[inline]
    pub const fn set_last_be(&mut self, value: u128) {
        let bytes = value.to_le_bytes();
        unsafe {
            self.ptr
                .copy_from_nonoverlapping(bytes.as_ptr().add(16 - Self::LAST_DIGIT_BYTES), Self::LAST_DIGIT_BYTES);
        }
    }
}
