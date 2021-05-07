fn main() {

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
}