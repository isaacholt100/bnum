# bnum

Arbitrary precision, fixed-size signed and unsigned integer types for Rust: `$BInt` and `$BUint`.

## Overview

The aim of this crate is to provide integer types of arbitrary fixed size which behave exactly like Rust's primitive integer types: `u8`, `i8`, `u16`, `i16`, etc. Nearly all methods defined on Rust's signed and unsigned primitive integers are defined on `$BInt` and `$BUint` respectively. Additionally, some other useful methods are provided, mostly inspired by the [`BigInt`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html) and [`BigUint`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html) types from the [`num_bigint`](https://docs.rs/num-bigint/latest/num_bigint/index.html) crate.

This crate uses Rust's const generics to allow creation of integers of arbitrary size that can be determined at compile time. `$BUint<N>` is stored as an array of digits (primitive unsigned integers) of length `N`. This means all `$BUint`s (and `$BInt`s) can be stored on the stack, as they are fixed size. `$BInt` is simply stored as a `$BUint` in two's complement.

The default digit that is used to store `$BUint` and `$BInt` is [`u64`], so the number of bits in `$BUint<N>` or `$BInt<N>` is `N * u64::BITS` (i.e. `N * 64`);

`bnum` can be used in `no_std` environments, provided a global default allocator is configured.

**NB: the examples in the documentation use specific types (e.g. `U256`, `U512`,  or `I256`, `I512`) to give examples of correct usage for most methods. There is nothing special about these types: all methods that are shown with these are implemented for `$BUint<N>` (or `$BInt<N>`) for any value of `N`.**

## Example Usage

```rust
// Calculate the `n`th Fibonacci number, using the type alias `U512`.
// Note that this is just an example, it is not the most efficient algorithm to calculate Fibonacci numbers!

use bnum::types::U512;

fn fibonacci(n: usize) -> U512 {
    let mut f_n: U512 = U512::ZERO; // or `U512::from(0u8)`
    let mut f_n_next: U512 = U512::ONE; // or `U512::from(1u8)`

    for _ in 0..n {
        let temp = f_n_next;
        f_n_next += f_n;
        f_n = temp;
    }

    f_n
}

let n = 100;
let f_n = fibonacci(n);

println!("The {}th Fibonacci number is {}", n, f_n);
// Prints "The 100th Fibonacci number is 354224848179261915075"

assert_eq!(f_n, U512::from_str_radix("354224848179261915075", 10).unwrap());
```

## Features

### Random Number Generation

The `rand` feature allows creation of random `$BInt`s and `$BUint`s via the [`rand`](https://docs.rs/rand/latest/rand/) crate.

### Serialization and Deserialization

The `serde` feature enables serialization and deserialization of `$BInt` and `$BUint` via the [`serde`](https://docs.rs/serde/latest/serde/) and [`serde_big_array`](https://docs.rs/serde-big-array/latest/serde_big_array/) crates.

### `num_traits` and `num_integer`

The `numtraits` feature includes implementations of traits from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/) and [`num_integer`](https://docs.rs/num-integer/latest/num_integer/) crates, e.g. [`AsPrimitive`](https://docs.rs/num-traits/latest/num_traits/cast/trait.AsPrimitive.html), [`Signed`](https://docs.rs/num-traits/latest/num_traits/sign/trait.Signed.html), [`Integer`](https://docs.rs/num-integer/latest/num_integer/trait.Integer.html) and [`Roots`](https://docs.rs/num-integer/latest/num_integer/trait.Roots.html).

### `u8` Digit

By default, each "digit" which is stored in a `$BUint` (or a `$BInt`) is a `u64`. This means that the integer is stored as an array of base `2^64` digits. This gives the best performance as having a larger number of bits in each digit means less digits need to be stored for a given bit width, so less operations need to be performed. The drawback of this is that the number of bits in a `$BUint` or a `$BInt` must be a multiple of 64. For situations where memory is limited or a more precise size is required, the `u8_digit` feature can be enabled. This means that each digit is now stored as a `u8` instead of a `u64`, so the number of bits can be any multiple of 8 instead of 64. The integer is therefore stored as an array of base `2^8` digits, meaning the number of bits in `$BUint<N>` or `$BInt<N>` is `N * u8::BITS` (i.e. `N * 8`) instead of `N * u64::BITS` (i.e. `N * 64`).

### Nightly features

Some functionality in this crate currently only works with the Nightly Rust compiler. The `nightly` feature enables this functionality, at the cost of only being able to compile on nightly. The nightly features that this crate uses are [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560), [`const_mut_refs`](https://github.com/rust-lang/rust/issues/57349), [`const_maybe_uninit_as_mut_ptr`](https://github.com/rust-lang/rust/issues/75251), [`const_trait_impl`](https://github.com/rust-lang/rust/issues/67792), [`const_num_from_num`](https://github.com/rust-lang/rust/issues/87852), [`const_swap`](https://github.com/rust-lang/rust/issues/83163).

Activating the `nightly` feature will make nearly every method defined in the library `const`, and will enable the `from_be_bytes`, `from_le_bytes`, `from_ne_bytes`, `to_be_bytes`, `to_le_bytes` and `to_ne_bytes` methods on `$BUint` and `$BInt`.

## Testing

This crate is tested with the [`quickcheck`](https://docs.rs/quickcheck/latest/quickcheck/) crate as well as with specific edge cases. The outputs of methods are compared to the outputs of the equivalent methods of primitive integers to ensure that the behaviour is identical.

## Documentation

Documentation for this project is currently incomplete and will be completed at a later date. In the meantime, since the API for all undocumented methods is the same as for the equivalent signed or unsigned primitive integer, the documentation for these can be referred to instead, e.g. [`u64`](https://doc.rust-lang.org/std/primitive.u64.html) or [`i64`](https://doc.rust-lang.org/std/primitive.i64.html).

**NB: `bnum`s version is currently pre-`1.0.0`. As per the [Semantic Versioning guidelines](https://semver.org/#spec-item-4), the public API may contain breaking changes while it is in this stage. However, as the API is designed to be as similar as possible to the API of Rust's primitive integers, it is unlikely that there will be a large number of breaking changes.**

## Known Issues

At the moment, the [`From`](https://doc.rust-lang.org/core/convert/trait.From.html) trait is implemented for `$BUint` and `$BInt`, from all the Rust primitive integers. However, this behaviour is not quite correct. For example, if a 24-bit wide integer were created (with the `u8_digit` feature enabled), this should not implement `From<u32>`, etc. and should implement `TryFrom<u32>` instead. To ensure correct behaviour, the [`FromPrimitive`](https://docs.rs/num-traits/latest/num_traits/cast/trait.FromPrimitive.html) trait from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/index.html) crate can be used instead.

The [`num_traits::NumCast`](https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html) trait is implemented for `$BUint` and `$BInt` but will panic if its method [`from`](https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html#tymethod.from) is called, as it is not possible to guarantee a correct conversion, due to trait bounds enforced by [`NumCast`](https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html). This trait should therefore never be used on `$BUint` or `$BInt`. The implementation exists only to allow implementation of the [`num_traits::PrimInt`](https://docs.rs/num-traits/latest/num_traits/int/trait.PrimInt.html) trait for `$BUint` and `$BInt`.

## Future Work

This library aims to provide arbitrary precision equivalents of Rust's 3 built-in number types: signed integers (`$BInt`), unsigned integers (`$BUint`) and floats. Signed and unsigned integers have been implemented and nearly fully tested, and will aim to keep up to date with Rust's integer interface (e.g. when a new method is implemented on a Rust primitive integer, this library will attempt to keep in step to include that method as well).

Currently, arbitrary precision fixed size floats are being worked on but are incomplete. Most of the basic methods have been implemented but are not fully tested, and at the moment there is no implementation of the transcendental floating point methods such as `sin`, `exp`, `log`, etc.

Obviously, the documentation needs to be completed, and benchmarks need to be written as well. This will take priority over the implementation of floats.

Additionally, a proc macro for parsing numeric values is being developed, which will allow easier creation of large constant values for `$BInt` and `$BUint`.

## Licensing

bnum is licensed under either the MIT license or the Apache License 2.0.