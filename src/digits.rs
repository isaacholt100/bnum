use core::marker::PhantomData;

use crate::{Byte, Exponent, Integer};

// since we don't have generic_const_exprs, this is used for a u8 array of length N * M + E
#[derive(Clone, Copy)]
#[repr(C)] // very important this is repr(C) to ensure no padding between arrays
pub struct Digits<D, const N: usize, const M: usize = 1, const E: usize = 0>(
    [[u8; N]; M],
    [u8; E],
    PhantomData<D>,
);

impl<D, const N: usize> Digits<D, N, 1, 0> {
    #[inline(always)]
    pub const fn from_integer<const S: bool, const B: usize, const OM: u8>(
        int: Integer<S, N, B, OM>,
    ) -> Self {
        debug_assert!(
            Self::DIGIT_BYTES == 1
                || Self::DIGIT_BYTES == 2
                || Self::DIGIT_BYTES == 4
                || Self::DIGIT_BYTES == 8
                || Self::DIGIT_BYTES == 16
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
    pub const fn from_integer_ref<const S: bool, const B: usize, const OM: u8>(
        int: &Integer<S, N, B, OM>,
    ) -> &Self {
        debug_assert!(
            Self::DIGIT_BYTES == 1
                || Self::DIGIT_BYTES == 2
                || Self::DIGIT_BYTES == 4
                || Self::DIGIT_BYTES == 8
                || Self::DIGIT_BYTES == 16
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

    #[inline(always)]
    pub const fn grow<const M: usize, const E: usize>(self) -> Digits<D, N, M, E> {
        let mut head = [[0u8; N]; M];
        head[0] = self.0[0];
        let tail = [0u8; E];
        Digits(head, tail, PhantomData)
    }
}

impl<D, const N: usize, const M: usize, const E: usize> Digits<D, N, M, E> {
    #[inline(always)]
    const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr() as _
    }

    #[inline(always)]
    const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr() as _
    }

    #[inline(always)]
    const fn force_digit<C>(self) -> Digits<C, N, M, E> {
        Digits(self.0, self.1, PhantomData)
    }

    pub const ALL_ZEROS: Self = Self([[0; N]; M], [0; E], PhantomData);
    pub const ONE: Self = {
        let mut out = Self::ALL_ZEROS.force_digit::<u8>();
        out.set(0, 1);
        out.force_digit()
    };
    const ALL_ONES: Self = Self([[u8::MAX; N]; M], [u8::MAX; E], PhantomData);

    const BYTE_LEN: usize = M * N + E;

    const BITS: u32 = Self::BYTE_LEN as u32 * 8;

    const DIGIT_BYTES: usize = core::mem::size_of::<D>();

    const DIGIT_LEN: usize = Self::BYTE_LEN.div_ceil(Self::DIGIT_BYTES);

    const LAST_DIGIT_BYTES: usize = {
        if Self::BYTE_LEN % Self::DIGIT_BYTES == 0 {
            Self::DIGIT_BYTES
        } else {
            Self::BYTE_LEN % Self::DIGIT_BYTES
        }
    };

    const LAST_DIGIT_BITS: u32 = Self::LAST_DIGIT_BYTES as u32 * 8;

    const LAST_DIGIT_PAD_BYTES: usize = Self::DIGIT_BYTES - Self::LAST_DIGIT_BYTES;

    const LAST_DIGIT_OFFSET: usize = Self::BYTE_LEN - Self::LAST_DIGIT_BYTES;

    #[inline]
    pub const fn remove_tail(self) -> Digits<D, N, M, 0> {
        Digits(self.0, [0; 0], PhantomData)
    }
}

macro_rules! digits_impl {
    { $name: ident; type $digit_name: ident = $($digit_type: ident), *; $impl_block: item } => {
        mod $name {
            $(
                mod $digit_type {
                    use super::super::*;

                    type $digit_name = $digit_type;

                    $impl_block
                }
            )*
        }
    }
}

digits_impl! {
    indices;
    type Digit = u8, u16, u32, u64, u128;

    #[allow(dead_code)]
    impl<const N: usize, const M: usize, const E: usize> Digits<Digit, N, M, E> {
    /// checks if index is within bounds only in debug mode
        #[inline]
        pub const fn get(&self, index: usize) -> Digit {
            debug_assert!(index < Self::DIGIT_LEN);
            if Self::DIGIT_BYTES == 1 {
                return unsafe { *(self.as_ptr().add(index) as *const _) };
            }
            if index == Self::DIGIT_LEN - 1 {
                return self.last();
            }
            let mut bytes = [0; { core::mem::size_of::<Digit>() }];
            unsafe {
                self.as_ptr()
                    .add(index * Self::DIGIT_BYTES)
                    .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::DIGIT_BYTES);
            }
            Digit::from_le_bytes(bytes)
        }

        #[inline]
        pub const fn last_padded<const ONES: bool>(&self) -> Digit {
            let mut bytes = if ONES {
                [u8::MAX; { core::mem::size_of::<Digit>() }]
            } else {
                [0; { core::mem::size_of::<Digit>() }]
            };
            unsafe {
                self.as_ptr()
                    .add(Self::LAST_DIGIT_OFFSET)
                    .copy_to_nonoverlapping(bytes.as_mut_ptr(), Self::LAST_DIGIT_BYTES);
            }
            Digit::from_le_bytes(bytes)
        }

        #[inline]
        pub const fn last(&self) -> Digit {
            self.last_padded::<false>()
        }

        #[inline]
        pub const fn set(&mut self, index: usize, value: Digit) {
            debug_assert!(index < Self::DIGIT_LEN);
            let bytes = value.to_le_bytes();
            if Self::DIGIT_BYTES == 1 {
                unsafe { *(self.as_mut_ptr().add(index) as *mut _) = value };
                return;
            }
            if index == Self::DIGIT_LEN - 1 {
                return self.set_last(value);
            }
            unsafe {
                self.as_mut_ptr()
                    .add(index * Self::DIGIT_BYTES)
                    .copy_from_nonoverlapping(bytes.as_ptr(), Self::DIGIT_BYTES);
            }
        }

        #[inline]
        pub const fn set_last(&mut self, value: Digit) {
            let bytes = value.to_le_bytes();
            unsafe {
                self.as_mut_ptr()
                    .add(Self::LAST_DIGIT_OFFSET)
                    .copy_from_nonoverlapping(bytes.as_ptr(), Self::LAST_DIGIT_BYTES);
            }
        }

        #[inline]
        pub const fn set_be(&mut self, index: usize, value: Digit) {
            debug_assert!(index < Self::DIGIT_LEN);
            if Self::DIGIT_BYTES == 1 {
                unsafe { *(self.as_mut_ptr().add(Self::BYTE_LEN - 1 - index) as *mut _) = value };
                return;
            }
            if index == Self::DIGIT_LEN - 1 {
                return self.set_last_be(value);
            }
            let bytes = value.to_le_bytes();
            unsafe {
                self.as_mut_ptr()
                    .add(Self::BYTE_LEN - (index + 1) * Self::DIGIT_BYTES)
                    .copy_from_nonoverlapping(bytes.as_ptr(), Self::DIGIT_BYTES);
            }
        }

        #[inline]
        pub const fn set_last_be(&mut self, value: Digit) {
            let bytes = value.to_le_bytes();
            unsafe {
                self.as_mut_ptr().copy_from_nonoverlapping(
                    bytes.as_ptr().add(Self::DIGIT_BYTES - Self::LAST_DIGIT_BYTES),
                    Self::LAST_DIGIT_BYTES,
                );
            }
        }
    }
}

digits_impl! {
    methods;
    type Digit = u8, u16, u32, u64, u128;

    #[allow(dead_code)] // because we don't use all these methods for each digit, just usually a single digit for a given method
    impl<const N: usize, const M: usize, const E: usize> Digits<Digit, N, M, E> {
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
                if tz != Digit::BITS {
                    return zeros;
                }
                i += 1;
            }
            zeros
        }

        // #[inline]
        // pub const fn leading_ones(self) -> Exponent {
        //     let mut ones = 0;
        //     let mut i = Self::DIGIT_LEN;
        //     while i > 0 {
        //         i -= 1;
        //         let digit = self.get(i);
        //         ones += digit.leading_ones();
        //         if digit != Digit::MAX {
        //             break;
        //         }
        //     }
        //     ones
        // }

        #[inline]
        pub const fn leading_zeros(self) -> Exponent {
            let mut zeros = 0;
            let mut i = Self::DIGIT_LEN;
            while i > 0 {
                i -= 1;
                let digit = self.get(i);
                let lz = digit.leading_zeros();
                zeros += lz;
                if lz != Digit::BITS {
                    return zeros - (Self::LAST_DIGIT_PAD_BYTES as u32 * 8);
                }
            }
            zeros - (Self::LAST_DIGIT_PAD_BYTES as u32 * 8)
        }

        #[inline]
        pub const fn bit_width(self) -> Exponent {
            Self::BITS - self.leading_zeros()
        }

        #[inline]
        pub const fn bit(&self, index: Exponent) -> bool {
            let byte = self.get(index as usize / Byte::BITS as usize);
            byte & (1 << (index % Byte::BITS)) != 0
        }

        #[inline]
        pub const fn swap_bytes(self) -> Self {
            let mut out = Self::ALL_ZEROS;
            let mut i = 0;
            while i < Self::DIGIT_LEN {
                let d = self.get(i);
                out.set_be(i, d.swap_bytes());

                i += 1;
            }
            out
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
                if self.get(i) != other.get(i) {
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
                if a > b {
                    return core::cmp::Ordering::Greater;
                } else if a < b {
                    return core::cmp::Ordering::Less;
                }
            }
            core::cmp::Ordering::Equal
        }

        #[inline]
        pub const fn bitand(self, rhs: Self) -> Self {
            let mut out = Self::ALL_ZEROS;
            let mut i = 0;
            while i < Self::DIGIT_LEN {
                let d = self.get(i) & rhs.get(i);
                out.set(i, d);

                i += 1;
            }
            out
        }

        #[inline]
        pub const fn bitor(self, rhs: Self) -> Self {
            let mut out = Self::ALL_ZEROS;
            let mut i = 0;
            while i < Self::DIGIT_LEN {
                let d = self.get(i) | rhs.get(i);
                out.set(i, d);

                i += 1;
            }
            out
        }

        #[inline]
        pub const fn bitxor(self, rhs: Self) -> Self {
            let mut out = Self::ALL_ZEROS;
            let mut i = 0;
            while i < Self::DIGIT_LEN {
                let d = self.get(i) ^ rhs.get(i);
                out.set(i, d);

                i += 1;
            }
            out
        }

        #[inline]
        pub const fn not(self) -> Self {
            let mut out = Self::ALL_ZEROS;
            let mut i = 0;
            while i < Self::DIGIT_LEN {
                let d = !self.get(i);
                out.set(i, d);

                i += 1;
            }
            out
        }

        #[inline]
        pub const fn is_power_of_two(self) -> bool {
            let mut i = 0;
            let mut ones = 0;
            while i < Self::DIGIT_LEN {
                ones += self.get(i).count_ones();
                if ones > 1 {
                    return false;
                }
                i += 1;
            }
            ones == 1
        }

        #[inline]
        pub const fn digit_carrying_add(a: Digit, b: Digit, carry: bool) -> (Digit, bool) {
            let (s1, o1) = a.overflowing_add(b);
            let (s2, o2) = s1.overflowing_add(carry as Digit); // this is faster as avoids a branch
            (s2, o1 || o2)
        }

        #[inline]
        pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
            let mut out = Self::ALL_ZEROS;
            let mut carry = false;
            let mut i = 0;
            let mut result = (0, false);

            while i < Self::DIGIT_LEN {
                result = Self::digit_carrying_add(self.get(i), rhs.get(i), carry);
                out.set(i, result.0);
                carry = result.1;
                i += 1;
            }

            if Self::LAST_DIGIT_BITS != Digit::BITS {
                carry = (Digit::BITS - result.0.leading_zeros()) > Self::LAST_DIGIT_BITS;
                if carry {
                    debug_assert!(result.0.leading_zeros() == Digit::BITS - Self::LAST_DIGIT_BITS - 1);
                }
            }

            (out, carry)
        }

        #[inline]
        pub const fn digit_borrowing_sub(a: Digit, b: Digit, borrow: bool) -> (Digit, bool) {
            let (s1, o1) = a.overflowing_sub(b);
            let (s2, o2) = s1.overflowing_sub(borrow as Digit); // this is faster as avoid a branch
            (s2, o1 || o2)
        }

        #[inline]
        pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
            let mut out = Self::ALL_ZEROS;
            let mut borrow = false;
            let mut i = 0;
            while i < Self::DIGIT_LEN {
                let result = Self::digit_borrowing_sub(self.get(i), rhs.get(i), borrow);
                out.set(i, result.0);
                borrow = result.1;
                i += 1;
            }
            (out, borrow)
        }

        // naive O(N^2) "digit by digit" multiplication
        #[inline]
        pub const fn long_mul<const WIDEN: bool>(self, rhs: Self) -> (Self, bool) {
            let mut overflow = false;
            let mut out = Self::ALL_ZEROS;
            let mut prod: Digit = 0;

            let mut i = 0;
            loop {
                if (WIDEN && i >= Self::DIGIT_LEN.div_ceil(2)) || (!WIDEN && i >= Self::DIGIT_LEN) {
                    break;
                }
                let mut carry = 0;
                let mut j = 0;
                loop {
                    if (WIDEN && j >= Self::DIGIT_LEN.div_ceil(2)) || (!WIDEN && i + j >= Self::DIGIT_LEN) {
                        break;
                    }
                    let index = i + j;
                    (prod, carry) = Self::digit_carrying_mul_add(
                        self.get(i),
                        rhs.get(j),
                        carry,
                        out.get(index),
                    );
                    out.set(index, prod);
                    j += 1;
                }
                if WIDEN {
                    if Self::DIGIT_LEN.div_ceil(2) * 2 > Self::DIGIT_LEN && i == Self::DIGIT_LEN.div_ceil(2) - 1 {
                        // index is too large, ...
                        // but should be enough leading zeros that carry is zero
                        debug_assert!(carry == 0);
                    } else {
                        out.set(i + Self::DIGIT_LEN.div_ceil(2), carry);
                    }
                }
                // compiler optimises away this code if not using the returned overflow
                if Self::LAST_DIGIT_BITS != Digit::BITS && Digit::BITS - Self::LAST_DIGIT_BITS > prod.leading_zeros() {
                    overflow = true;
                } else if carry != 0 {
                    overflow = true;
                } else if self.get(i) != 0 {
                    // j += 1;
                    while j < Self::DIGIT_LEN {
                        if rhs.get(j) != 0 {
                            overflow = true;
                            break;
                        }

                        j += 1;
                    }
                }

                i += 1;
            }
            (out, overflow)
        }

        #[inline]
        pub(crate) const fn mul_digit(self, rhs: Digit) -> (Self, bool, ) {
            let mut out = Self::ALL_ZEROS;
            let (mut prod, mut carry) = (0, 0);
            let mut i = 0;
            while i < Self::DIGIT_LEN {
                let d = self.get(i);
                (prod, carry) = Self::digit_carrying_mul_add(d, rhs, carry, 0);
                out.set(i, prod);
                i += 1;
            }
            let overflow = carry != 0
                || (Self::LAST_DIGIT_BITS != Digit::BITS
                    && Digit::BITS - prod.leading_zeros() > Self::LAST_DIGIT_BITS);

            (out, overflow)
        }

        #[inline]
        pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
            // if rhs.bit_width() < Digit::BITS {
            //     self.mul_digit(rhs.get(0))
            // } else {
                self.long_mul::<false>(rhs)
            // }
        }

        #[inline]
        pub const unsafe fn unchecked_shl(self, rhs: Exponent) -> Self {
            let byte_shift = (rhs / 8) as usize;
            let bit_shift = rhs % 8;
            let mut out = Self::ALL_ZEROS;

            unsafe {
                out
                    .as_mut_ptr()
                    .add(byte_shift)
                    .copy_from_nonoverlapping(self.as_ptr(), Self::BYTE_LEN - byte_shift);
            }

            if bit_shift != 0 {
                let carry_shift = Digit::BITS - bit_shift;
                let mut carry = 0;

                let mut i = (rhs / Digit::BITS) as usize;
                while i < Self::DIGIT_LEN {
                    let d = out.get(i);
                    out.set(i, (d << bit_shift) | carry);
                    carry = d >> carry_shift;

                    i += 1;
                }
            }

            out
        }

        #[inline]
        pub(crate) const unsafe fn unchecked_shr(self, rhs: Exponent, negative_sign_extend: bool) -> Self {
            let mut out = if negative_sign_extend {
                Self::ALL_ONES
            } else {
                Self::ALL_ZEROS
            };
            let byte_shift = (rhs / 8) as usize;
            let bit_shift = rhs % 8;

            unsafe {
                out
                    .as_mut_ptr()
                    .copy_from_nonoverlapping(self.as_ptr().add(byte_shift), Self::BYTE_LEN - byte_shift);
            }

            if bit_shift != 0 {
                let carry_shift = Digit::BITS - bit_shift;

                let mut i = (Self::BITS - rhs).div_ceil(Digit::BITS) as usize; // we could just start at DIGIT_LEN, but then we would just be shifting all ones/all zeros for the unaffected digits (at higher indices) which would do nothing

                let mut carry = if negative_sign_extend {
                    let shift_back = if Self::LAST_DIGIT_BITS == Digit::BITS || i != Self::DIGIT_LEN {
                        // if i is not the last digit index, then we have a full digit
                        carry_shift
                    } else {
                        // last digit (the incomplete one) has Self::LAST_DIGIT_BITS bits, so shift back by this minus bit_shift
                        Self::LAST_DIGIT_BITS - bit_shift
                    };
                    Digit::MAX << shift_back // if negative, then we initialise the carry to have the correct number of sign bits (we can get this expression by looking at the expression for carry shift in the while loop. the previous digit will have been all ones (unless this was the last digit, but then we can view it as having infinite leading ones, as this still represents the same value))
                } else {
                    0
                };
                while i > 0 {
                    i -= 1;

                    let current_digit = out.get(i);
                    out.set(i, (current_digit >> bit_shift) | carry);
                    carry = current_digit << carry_shift;
                }
            }

            out
        }

        #[inline]
        pub const unsafe fn unchecked_rotate_left(self, rhs: Exponent) -> Self {
            let digit_shift = (rhs / 8) as usize;
            let bit_shift = rhs % 8;

            let mut out = self;
            unsafe {
                out.as_mut_ptr().add(digit_shift).copy_from_nonoverlapping(self.as_ptr(), Self::BYTE_LEN - digit_shift);
                out.as_mut_ptr().copy_from_nonoverlapping(self.as_ptr().add(Self::BYTE_LEN - digit_shift), digit_shift);
            }
            // let slice = unsafe { core::slice::from_raw_parts_mut(out.as_mut_ptr(), Self::BYTE_LEN) };
            // slice.rotate_left(digit_shift);

            if bit_shift != 0 {
                let carry_shift = Digit::BITS - bit_shift;
                let mut carry =
                    out.last() >> (Self::LAST_DIGIT_BITS - bit_shift);

                let mut i = 0;
                while i < Self::DIGIT_LEN {
                    let current_digit = out.get(i);
                    out
                        .set(i, (current_digit << bit_shift) | carry);
                    carry = current_digit >> carry_shift;
                    i += 1;
                }
            }

            out
        }
    }
}

#[cfg(any(test, feature = "quickcheck"))]
digits_impl! {
    quickcheck_arbitrary;
    type Digit = u8, u16, u32, u64, u128;

    impl<const N: usize, const M: usize, const E: usize> quickcheck::Arbitrary for Digits<Digit, N, M, E> {
        #[inline]
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut out = Self::ALL_ZEROS;
            for i in 0..Self::DIGIT_LEN {
                out.set(i, <Digit as quickcheck::Arbitrary>::arbitrary(g));
            }
            out
        }
    }
}

trait DoubleDigit: Sized {
    type Double;
}

impl DoubleDigit for u8 {
    type Double = u16;
}

impl DoubleDigit for u16 {
    type Double = u32;
}

impl DoubleDigit for u32 {
    type Double = u64;
}

impl DoubleDigit for u64 {
    type Double = u128;
}

impl<const N: usize, const M: usize, const E: usize> Digits<u128, N, M, E> {
    #[inline]
    pub const fn digit_carrying_mul_add(
        a: u128,
        b: u128,
        carry: u128,
        current: u128,
    ) -> (u128, u128) {
        let (a_lo, a_hi) = (a as u64, (a >> 64) as u64);
        let (b_lo, b_hi) = (b as u64, (b >> 64) as u64);
        let (c_lo, c_hi) = (carry as u64, (carry >> 64) as u64);
        let (d_lo, d_hi) = (current as u64, (current >> 64) as u64);
        let x = (a_lo as u128) * (b_lo as u128) + (c_lo as u128) + (d_lo as u128);
        let y = (a_lo as u128) * (b_hi as u128) + (c_hi as u128) + (d_hi as u128);
        let (y, carry_y) = y.overflowing_add((a_hi as u128) * (b_lo as u128));
        let (x, carry_x) = x.overflowing_add(y << 64);
        let carry2 = if carry_y { 1 << 64 } else { 0 };
        let carry3 = if carry_x { 1 } else { 0 };
        let z = (a_hi as u128) * (b_hi as u128) + carry2 + carry3 + (y >> 64);

        (x, z)
    }
}

digits_impl! {
    knuth_division;
    type Digit = u8, u16, u32, u64;

    #[allow(dead_code)]
    impl<const N: usize, const M: usize, const E: usize> Digits<Digit, N, M, E> {
        #[inline]
        pub const fn digit_carrying_mul_add(a: Digit, b: Digit, c: Digit, carry: Digit) -> (Digit, Digit) {
            type Double = <Digit as DoubleDigit>::Double;

            let prod = (a as Double) * (b as Double) + (c as Double) + (carry as Double);
            (prod as Digit, (prod >> Digit::BITS) as Digit)
        }

        #[inline]
        const fn digit_div_rem_wide(low: Digit, high: Digit, rhs: Digit) -> (Digit, Digit) {
            type Double = <Digit as DoubleDigit>::Double;

            debug_assert!(high < rhs);

            let a = ((high as Double) << Digit::BITS) | (low as Double);
            (
                (a / rhs as Double) as Digit,
                (a % rhs as Double) as Digit,
            )
        }

        #[inline]
        pub(crate) const fn div_rem_digit(self, rhs: Digit) -> (Self, Digit) {
            let mut out = Self::ALL_ZEROS;
            let mut rem: Digit = 0;

            let mut i = Self::DIGIT_LEN;
            while i > 0 {
                i -= 1;

                let d = self.get(i);
                let (q, r) = Self::digit_div_rem_wide(d, rem, rhs);
                rem = r;
                out.set(i, q);
            }
            (out, rem)
        }

        #[inline(always)]
        const fn digit_widening_mul(a: Digit, b: Digit) -> (Digit, Digit) {
            Self::digit_carrying_mul_add(a, b, 0, 0)
        }

        const fn div_rem_knuth_wide(mut self, rhs: Self, n: usize) -> (Self, Self) {
            // The Art of Computer Programming Volume 2 by Donald Knuth, Section 4.3.1, Algorithm D
            // using the modification from exercise 37
            debug_assert!(n >= 2); // if n = 1, then we should have used the division by digit method instead

            let m = self.bit_width().div_ceil(Digit::BITS) as usize - n;
            let e = rhs.get(n - 1).leading_zeros();

            let mut q = Self::ALL_ZEROS;

            // exercise 37
            // 2^e (v_(n - 1) v_(n - 2) v_(n - 3))_b is exactly 3 digits, and has MSB set to 1, due to choice of e
            let v_dash = (rhs.get(n - 1) << e)
                | (rhs.get(n - 2).unbounded_shr(Digit::BITS - e));
            let v_dash_dash = {
                let mut out = rhs.get(n - 2) << e;
                if n >= 3 {
                    out |= rhs.get(n - 3).unbounded_shr(Digit::BITS - e);
                }
                out
            };

            let mut j = m + 1; // D2
            while j > 0 {
                j -= 1; // D7

                let u_dash = if j + n == Self::DIGIT_LEN {
                    self.get(j + n - 1).unbounded_shr(Digit::BITS - e)
                } else {
                    (self.get(j + n) << e) | (self.get(j + n - 1).unbounded_shr(Digit::BITS - e))
                };
                let u_dash_dash = (self.get(j + n - 1) << e)
                    | (self.get(j + n - 2)
                        .unbounded_shr(Digit::BITS - e)); // have that n >= 2 from the assertion, so these indices are valid

                let u_dash_dash_dash = if j + n >= 3 {
                    (self.get(j + n - 2) << e) | (self.get(j + n - 3).unbounded_shr(Digit::BITS - e))
                } else {
                    self.get(j + n - 2) << e
                };

                // D3
                let mut q_hat = if u_dash < v_dash {
                    let (mut q, r) = Self::digit_div_rem_wide(u_dash_dash, u_dash, v_dash);

                    #[inline(always)]
                    const fn digit_tuple_gt(a: (Digit, Digit), b: (Digit, Digit)) -> bool {
                        a.1 > b.1 || a.1 == b.1 && a.0 > b.0
                    }

                    if digit_tuple_gt(
                        Self::digit_widening_mul(q, v_dash_dash),
                        (u_dash_dash_dash, r),
                    ) {
                        q -= 1;

                        if let Some(r) = r.checked_add(v_dash) {
                            if digit_tuple_gt(
                                Self::digit_widening_mul(q, v_dash_dash),
                                (u_dash_dash_dash, r),
                            ) {
                                q -= 1;
                            }
                        }
                    }
                    q
                } else {
                    Digit::MAX
                };

                // D4
                let borrow = {
                    let (m, overflow) = rhs.force_digit::<u128>().mul_digit(q_hat as u128);
                    let m = m.force_digit::<Digit>();
                    let borrow = self.sub_partial(m, j, n);
                    if overflow {
                        debug_assert!(n == Self::BITS.div_ceil(Digit::BITS) as usize);
                    }
                    overflow || borrow
                };

                if borrow {
                    // D6
                    q_hat -= 1;
                    self.add_partial(rhs, j, n);
                }

                // D5
                q.set(j, q_hat);
            }

            (q, self)
        }
        #[inline]
        const fn sub_partial(&mut self, rhs: Self, start: usize, range: usize) -> bool {
            let mut borrow = false;
            let mut i = 0;
            while i <= range {
                if i + start == Self::DIGIT_LEN {
                    if i < Self::DIGIT_LEN && rhs.get(i) != 0 {
                        borrow = true;
                    }
                } else {
                    let (sub, overflow) = Self::digit_borrowing_sub(
                        self.get(i + start),
                        rhs.get(i),
                        borrow,
                    );
                    self.set(i + start, sub);
                    borrow = overflow;
                }
                i += 1;
            }
            borrow
        }

        #[inline]
        const fn add_partial(&mut self, rhs: Self, start: usize, range: usize) {
            let mut carry = false;
            let mut i = 0;
            while i < range {
                let (sum, overflow) = Self::digit_carrying_add(
                    self.get(i + start),
                    rhs.get(i),
                    carry,
                );
                self.set(i + start, sum);
                carry = overflow;
                i += 1;
            }
            if carry {
                if range + start != Self::DIGIT_LEN {
                    self.set(
                        range + start,
                        self.get(range + start).wrapping_add(1),
                    );
                }
            }
            // debug_assert!(carry);
        }

        #[inline]
        pub(crate) const fn div_rem_unchecked(self, rhs: Self) -> (Self, Self) {
            debug_assert!(!rhs.eq(&Self::ALL_ZEROS));
            debug_assert!(self.cmp(&rhs).is_gt());

            let bit_width = rhs.bit_width();
            if bit_width <= Digit::BITS {
                let d = rhs.get(0);
                let (div, rem) = self.div_rem_digit(d);
                let mut out = Self::ALL_ZEROS;
                out.set(0, rem);
                (div, out)
            } else {
                self.div_rem_knuth_wide(rhs, bit_width.div_ceil(Digit::BITS) as usize)
            }
        }
    }
}
