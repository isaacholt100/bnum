/*fn main() {

#[cfg(all(use_addcarry, target_arch = "x86_64"))]
use core::arch::x86_64 as arch;

#[cfg(all(use_addcarry, target_arch = "x86"))]
use core::arch::x86 as arch;
    println!("{}", 255u8.wrapping_add(255));
    let mut a = 0;
    println!("{}, {}", unsafe { core::arch::x86_64::_addcarry_u64(0, u64::MAX, u64::MAX, &mut a) }, a);
    println!("{}", 12u16.swap_bytes());
    let arr1 = [0u32; 100000];
    let arr2 = [0u32; 100000];
    let mut sum = 0;
    let now = std::time::Instant::now();
    // 2.7-2.8ms
    /**/
    // ~0.4ms
    for i in 0..100 {
        /*let mut i = 0;
        while i < 100000 {
            sum += arr1[i] + arr2[i];
            i += 1;
        }*/
        for (a, b) in arr1.iter().zip(arr2.iter()) {
            sum += a + b;
        }
    }
    println!("{:?}", now.elapsed());
    println!("{:?}", sum);
    println!("{:E}", 99999999999999999999999999999999999999u128);
    println!("{}", format!("{:o}", u64::MAX).len());
    #[allow(arithmetic_overflow)]
    let a = 25u8 - 26u8;
}
// Add with carry:
#[cfg(use_addcarry)]
#[inline]
fn adc(carry: u8, a: u8, b: u8, out: &mut u64) -> u8 {
    unsafe {
        arch::_addcarry_u8(carry, a, b, out)
    }
}

#[cfg(not(use_addcarry))]
#[inline]
fn adc(carry: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    let sum = a as u128 + b as u128 + carry as u128;
    *out = sum as u64;
    (sum >> 64) as u8
}*/
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;

#[inline]
fn adc(carry: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    unsafe {
        arch::_addcarry_u64(carry, a, b, out)
    }
}

#[inline]
fn add_carry(carry: u8, a: u64, b: u64) -> (u64, u8) {
    let sum = a as u128 + b as u128 + carry as u128;
    //*out = sum as u64;
    (sum as u64, (sum >> 64) as u8)
}

#[inline]
const fn add_carry_test(carry: bool, a: u64, b: u64) -> (u64, bool) {
    let sum = if carry {
        a as u128 + b as u128 + 1
    } else {
        a as u128 + b as u128
    };
    //*out = sum as u64;
    (sum as u64, (sum >> 64) != 0)
}

fn main() {
    /*println!("{:?}", add_carry_test(false, u64::MAX - 1, 1));
    #[allow(unused, dead_code)]
    let mut out3 = 0;
    let now = std::time::Instant::now();
    for i in 0..1000000000 {
        if i % 2 == 0 {
            out3 = add_carry_test(true, 345435048, 309485039845938).0;
        }
    }
    println!("add_carry_test: {:?}", now.elapsed());
    #[allow(unused, dead_code)]
    let mut out = 0;
    let now = std::time::Instant::now();
    for i in 0..1000000000 {
        if i % 2 == 0 {
            out = add_carry(1, 345435048, 309485039845938).0;
        }
    }
    println!("add_carry: {:?}", now.elapsed());

    #[allow(unused, dead_code)]
    let mut out2 = 0;
    let now = std::time::Instant::now();
    for i in 0..1000000000 {
        if i % 2 == 0 {
            adc(1, 345435048, 309485039845938, &mut out2);
        }
    }
    println!("adc: {:?}", now.elapsed());
    println!("{}", out);
    println!("{}", out2);
    println!("{}", out3);*/
    #[allow(unused, dead_code)]
    let mut sum = 0;
    let now = std::time::Instant::now();
    let arr = [0; 100000];
    for i in 0..100000 - 8 {
        let a = unsafe { *((&arr[i..(i + 8)]).as_ptr() as *const [i32; 8]) };
        //let a = [arr[i]];
        //let a = [arr[i], arr[i + 1], arr[i + 2], arr[i + 3], arr[i + 4], arr[i + 5], arr[i + 6], arr[i + 7]];
        sum += &a.len();
        //let a = [arr[i], arr[i + 1], arr[i + 2], arr[i + 3], arr[i + 4], arr[i + 5], arr[i + 6], arr[i + 7]];
    }
    //println!("{:?}", now.elapsed());
    //println!("{}", sum);
    /*let mut v2: Vec<[i32; 8]> = vec![];
    let now = std::time::Instant::now();
    let arr = [0; 100000];
    for i in 0..100000 - 8 {
        v2.push([arr[i], arr[i + 1], arr[i + 2], arr[i + 3], arr[i + 4], arr[i + 5], arr[i + 6], arr[i + 7]]);
        //let a = [arr[i], arr[i + 1], arr[i + 2], arr[i + 3], arr[i + 4], arr[i + 5], arr[i + 6], arr[i + 7]];
    }
    println!("{:?}", now.elapsed());
    println!("{:?}", v.len());
    println!("{:?}", v2.len());*/
    const LENGTH: usize = 1000;
    let arr1 = [39457348957893475983745897344u128; LENGTH];
    let arr2 = [12354987348957389475983475934u128; LENGTH];
    //let arr1 = [120u8; LENGTH];
    //let arr2 = [90u8; LENGTH];
    //let arr2 = [120u8; 1000 * 8];
    let mut s = 0;
    let now = std::time::Instant::now();
    for j in 0..10000 {
        let mut arr3 = [0; LENGTH];
        let mut i = 0;
        while i < LENGTH {
            arr3[i] = arr1[i] + arr2[i];
            if i % 3 == 0 {
            s += &arr3.len();
            }
            i += 1;
        }
    }
    //println!("{:?}", now.elapsed());
    //println!("{:?}", s);
    //println!("{:b}", 128i8);
    println!("{:?}", 254u8.overflowing_shl(443));
    fn last_set_bit(n: u32) -> u8 {
        ((std::mem::size_of_val(&n) as u8) << 3) - n.leading_zeros() as u8
    }
    fn ilog2(n: u32) -> u8 {
        last_set_bit(n) - 1
    }
    println!("{:b} {:b}", -45i8 as u8, -23i8 as u8);
    println!("{:?}", (-45i8).overflowing_mul(-23i8));
    println!("{:?}", (-45i8 as u8).overflowing_mul(-23i8 as u8));
}