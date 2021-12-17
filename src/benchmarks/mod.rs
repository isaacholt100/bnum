extern crate test;

use test::Bencher;
use crate::BUint;

#[inline]
pub const fn add_carry_unsigned(carry: u8, a: u64, b: u64) -> (u64, u8) {
    let sum = a as u128 + b as u128 + carry as u128;
    (sum as u64, (sum >> 64) as u8)
}

#[inline]
pub const fn carrying_add(a: u64, rhs: u64, carry: bool) -> (u64, bool) {
    // note: longer-term this should be done via an intrinsic, but this has been shown
    //   to generate optimal code for now, and LLVM doesn't have an equivalent intrinsic
    let (a, b) = a.overflowing_add(rhs);
    let (c, d) = a.overflowing_add(carry as u64);
    (c, b | d)
}

#[bench]
fn bench_1(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10000 {
            test::black_box(carrying_add(i, i, true));
        }
        for i in u64::MAX-10000..u64::MAX {
            test::black_box(carrying_add(i, i, true));
        }
    })
}

#[bench]
fn bench_2(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10000 {
            test::black_box(add_carry_unsigned(1, i, i));
        }
        for i in u64::MAX-10000..u64::MAX {
            test::black_box(add_carry_unsigned(1, i, i));
        }
    })
}

#[bench]
fn bench_shift_add(b: &mut Bencher) {
    let a = 3534433434645650u64;
    b.iter(|| {
        for _ in 0..10000 {
            test::black_box((a >> 6) + (a >> 2) + (a >> 1));
        }
    });
}

#[bench]
fn bench_mul(b: &mut Bencher) {
    let a = 3534433434645650u64;
    b.iter(|| {
        for _ in 0..10000 {
            test::black_box(a * 72);
        }
    });
}

macro_rules! tagun {
    {
        $(
            enum $name: ident {
                $($vis: vis $variant: ident: $ty: ty), +
            }
        )+
    } => {
        $(
            mod my_mod {
                use std::mem::ManuallyDrop;
                const _SLICE: &[&str] = &[$(stringify!($variant)), +];
                const NUM_VARIANTS: usize = _SLICE.len();
                pub union Variant {
                    $($vis $variant: ManuallyDrop<$ty>), +
                }
                #[derive(Clone, Copy)]
                pub enum Tag {
                    $($variant), +
                }
                pub struct $name {
                    tag: Tag,
                    variant: Variant,
                }
                impl $name {
                    pub const fn discriminant(&self) -> u8 {
                        self.tag as u8
                    }
                    #[inline]
                    pub const fn num_variants() -> usize {
                        NUM_VARIANTS
                    }
                    #[inline]
                    pub const fn as_u8(&self) -> u8 {
                        self.tag as u8
                    }
                    #[inline]
                    pub const fn as_u16(&self) -> u16 {
                        self.tag as u16
                    }
                    $(
                        #[inline]
                        pub const fn $variant(value: $ty) -> Self {
                            Self {
                                tag: Tag::$variant,
                                variant: Variant {
                                    $variant: ManuallyDrop::new(value),
                                },
                            }
                        }
                    )+
                }
                impl std::ops::Deref for $name {
                    type Target = Variant;

                    #[inline]
                    fn deref(&self) -> &Self::Target {
                        &self.variant
                    }
                }
                impl Drop for $name {
                    #[inline]
                    fn drop(&mut self) {
                        unsafe {
                            match self.tag {
                                $(
                                    Tag::$variant => {
                                        ManuallyDrop::drop(&mut self.variant.$variant);
                                    }
                                ), +
                            }
                        }
                    }
                }
                macro_rules! match_union {
                    (match $u: expr, {

                    }) => {
                        
                    };
                }
            }
            pub use my_mod::$name;
        )*
    }
}