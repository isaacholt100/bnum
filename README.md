# Bint

Arbitrary precision, fixed-size signed and unsigned integer types for Rust: `Bint` and `BUint`.

## Overview

The aim of this crate is to provide integer types of any fixed size which behave exactly like Rust's primitive integer types: `u8`, `i8`, `u16`, `i16`, etc. Nearly all methods defined on Rust's signed and unsigned primitive integers are defined on `Bint` and `BUint` respectively. Additionally, some other useful methods are provided, mostly inspired by the `BigInt` and `BigUint` types from the `num` crate.

This crate uses Rust's const generics to creation allow integers of any size that can be determined at compile time. `BUint<N>` is stored as an array of digits (primitive unsigned integers) of length `N`. `Bint` is simply stored as a `BUint` in two's complement.

**NB: this library relies on a few features that are only available on the nightly Rust compiler, and so currently it can only run on nightly. These features are `generic_const_exprs`, `const_intrinsic_copy`, `const_mut_refs`, `const_maybe_uninit_as_mut_ptr`, `const_trait_impl`. This allows nearly all methods defined on `BUint` and `Bint` to be `const`, just as the ones on Rust's primitive integers are.** 

## Features

### Random Number Generation

Random `Bint`s and `BUint`s can be created via the popular `rand` crate when the `rand` feature is enabled.

### Serialization and Deserialization

The `serde` feature enables serialization and deserialization of `Bint` and `BUint` via the `serde` and `serde-big-array` crates.

### `u8` Digit

By default, each "digit" which is stored in a `BUint` (or a `Bint`) is a `u64`. This gives the best performance as having a larger number of bits in each digit means less digits need to be stored for a given type, so less operations need to be performed. The drawback of this is that the number of bits in a `BUint` or a `Bint` is a multiple of 64. For situations where memory is limited or a more precise size is required, the `u8_digit` feature can be enabled. This means that each digit is now stored as a `u8` instead of a `u64`, so the number of bits can be any multiple of 8 instead of 64.

## Testing

This crate is tested with the `quickcheck` crate as well as with specific edge cases.

## Future Work

This library aims to provide arbitrary precision equivalents of Rust's 3 built-in number types: signed integers (`Bint`), unsigned integers (`BUint`) and floats. Signed and unsigned integers have been implemented and nearly fully tested, and will keep up to date with Rust's integer interface (e.g. when a new method is implemented on a Rust primitive integer, this library will be updated to include that method as well).

Currently, arbitrary precision fixed size floats are being worked on but are incomplete. Most of the basic methods have been implemented but are not fully tested, and there is no implementation of the transcendental floating point methods such as `sin`, `exp`, `log`, etc.

Obviously, the documentation needs to be completed, and benchmarks need to be written as well. This will take priority over the implementation of floats.