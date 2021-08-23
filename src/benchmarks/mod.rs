extern crate test;

use test::Bencher;
use crate::uint::BUint;
use alloc::string::String;
use crate::U128;

/*#[bench]
fn bench_const_eq(b: &mut Bencher) {
    let mut arr = [39485734; 10000];
    let u1 = BUint::from(arr);
    arr[445] = 0;
    let u2 = BUint::from(arr);
    b.iter(|| {
        test::black_box(u2.eq(&u1));
    });
}

#[bench]
fn bench_eq(b: &mut Bencher) {
    let mut arr = [39485734; 10000];
    let u1 = BUint::from(arr);
    arr[445] = 0;
    let u2 = BUint::from(arr);
    b.iter(|| {
        test::black_box(u2 == u1);
    });
}*/

#[bench]
fn bench_recursion(b: &mut Bencher) {
    fn sum(v: &mut Vec<u128>) -> u128 {
        if v.len() == 0 {
            0
        } else {
            v.pop().unwrap().wrapping_add(sum(v))
        }
    }
    let mut v = (0..100000000).collect();
    b.iter(|| {
        for i in 0..10000 {
            test::black_box({
                sum(&mut v)
            });
        }
    });
}

#[bench]
fn bench_iter(b: &mut Bencher) {
    let v: Vec<u128> = (0..100000000).collect();
    b.iter(|| {
        for _ in 0..1000000 {
            test::black_box({
                let mut i = v.iter();
                while let Some(_) = i.next() {

                }
            });
        }
    })
}

#[bench]
fn bench_while(b: &mut Bencher) {
    let v: Vec<u128> = (0..100000000).collect();
    b.iter(|| {
        for _ in 0..1000000 {
            test::black_box({
                let mut i = 0;
                //let mut sum = 0;
                let len = v.len();
                while i < len {
                    //sum += v[i];
                    i += 1;
                }
                //sum;
            });
        }
    })
}

#[bench]
fn bench_buint_add(b: &mut Bencher) {
    let u1 = 394857209495782456444589679u128;
    let u1 = U128::from(u1);
    let u2 = 30249568710094856749560704u128;
    let u2 = U128::from(u2);
    b.iter(|| {
        test::black_box(u2 + u1);
    });
}

#[bench]
fn bench_test(b: &mut Bencher) {
    let u1 = 2456799254794579u128;
    let u = U128::from(u1);
    b.iter(|| {
        for i in 0..1000000 {
            test::black_box({
                let d = u.digits();
                let u = U128::from(*d);
                //u + u;
            });
        }
    })
}

use std::collections::HashSet;

const ITEMS: &[&str] = &["a", "b", "c", "d", "e", "f", "g", "h", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

#[bench]
fn bench_set(b: &mut Bencher) {
    use std::iter::FromIterator;
    let mut set = HashSet::new();
    for i in 0..1000 {
        set.insert(i);
    }
    b.iter(|| {
        for i in 0..1000 {
            test::black_box(set.contains(&i));
        }
    })
}

#[bench]
fn bench_vec(b: &mut Bencher) {
    use std::iter::FromIterator;
    let mut set = Vec::new();
    for i in 0..1000 {
        set.push(i);
    }
    b.iter(|| {
        for i in 0..1000 {
            test::black_box(set.contains(&i));
        }
    })
}

trait Overload {
    type Ret;

    fn call(self) -> Self::Ret;
}

fn overloadable<Args: Overload>(args: Args) -> <Args as Overload>::Ret {
    args.call()
}

mod a {

}
fn a() {}

macro_rules! overload {
    {
        $vis: vis fn $name: ident;
        $(fn $dummy: ident ($($arg: ident: $ty: tt), *) -> $ret: ty $body: block)*
    } => {
        mod $name {
            pub trait Overload {
                type Ret;
            
                fn call(self) -> Self::Ret;
            }
            pub fn $name<Args: Overload>(args: Args) -> <Args as Overload>::Ret {
                args.call()
            }
            $(
                #[allow(unused_parens)]
                impl Overload for ($($ty), *) {
                    type Ret = $ret;

                    fn call(self) -> Self::Ret {
                        #[allow(unused_parents)]
                        let ($($arg), *) = self;
                        $body
                    }
                }
            )*
        }
        $vis use $name::$name;
    };
}

overload! {
    pub fn test_o;
    fn test_o(a: u8) -> u8 {
        a
    }
    fn test_o(a: u16, b: u16) -> u16 {
        a as u16
    }
}

fn t() {
    test_o((2, 3));
}

trait Add {
    fn add1(self) -> Self;
}

impl Add for u8 {
    fn add1(self) -> Self {
        self + 1
    }
}

impl Add for u16 {
    fn add1(self) -> Self {
        self + 1
    }
}

fn add1<T: Add>(t: T) -> T {
    t.add1()
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
                    pub const fn num_variants() -> usize {
                        NUM_VARIANTS
                    }
                    pub const fn as_u8(&self) -> u8 {
                        self.tag as u8
                    }
                    pub const fn as_u16(&self) -> u16 {
                        self.tag as u16
                    }
                    $(
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

                    fn deref(&self) -> &Self::Target {
                        &self.variant
                    }
                }
                impl Drop for $name {
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
            }
            pub use my_mod::$name;
        )*
    }
}

tagun! {
    enum TestEnum {
        a: i32,
        b: u32,
        pub c: String,
        d: Vec<char>
    }
}

fn ttt() {
    let a = TestEnum::a(-62);
    let c = unsafe { &a.c };
}