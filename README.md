# bnum

[![GitHub](https://img.shields.io/badge/GitHub-isaacholt100/bnum-default?logo=github)](https://github.com/isaacholt100/bnum)
[![doc.rs](https://img.shields.io/docsrs/bnum)](https://docs.rs/bnum/latest/bnum)
[![Crates.io](https://img.shields.io/crates/d/bnum?logo=rust)
](https://crates.io/crates/bnum)
[![dependency status](https://deps.rs/repo/github/isaacholt100/bnum/status.svg)](https://deps.rs/repo/github/isaacholt100/bnum)
[![codecov](https://codecov.io/gh/isaacholt100/bnum/branch/master/graph/badge.svg)](https://codecov.io/gh/isaacholt100/bnum)
[![license](https://img.shields.io/crates/l/bnum)](https://github.com/isaacholt100/bnum)

Arbitrary precision, fixed-size signed and unsigned integer types for Rust.

## Overview

The aim of this crate is to provide integer types of arbitrary fixed size which behave exactly like Rust's primitive integer types: `u8`, `i8`, `u16`, `i16`, etc. Nearly all methods defined on Rust's signed and unsigned primitive integers are defined `bnum`'s signed and unsigned integers. Additionally, some other useful methods are provided, mostly inspired by the [`BigInt`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html) and [`BigUint`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html) types from the [`num_bigint`](https://docs.rs/num-bigint/latest/num_bigint/index.html) crate.

This crate uses Rust's const generics to allow creation of integers of arbitrary size that can be determined at compile time. Unsigned integers are stored as an array of digits (primitive unsigned integers) of length `N`. This means all `bnum` integers can be stored on the stack, as they are fixed size. Signed integers are simply stored as an unsigned integer in two's complement.

`bnum` defines 4 unsigned integer types: each uses a different primitive integer as its digit type. `BUint` uses `u64` as its digit, `BUintD32` uses `u32`, `BUintD16` uses `u16` and `BUintD8` uses `u8`. The signed integer types `BInt`, `BIntD32`, `BIntD16` and `BIntD8` are represented by these unsigned integers respectively.

`BUint` and `BInt` are the fastest as they store (and so operate on) the least number of digits for a given bit size. However, the drawback is that the bit size must be a multiple of `64` (`bitsize = N * 64`). This is why other integer types are provided as well, as they allow the bit size to be a multiple of `32`, `16` or `8` instead. When choosing which of these types to use, determine which of `64, 32, 16, 8` is the largest divisor of the desired bit size, and use the corresponding type. For example, if you wanted a 96-bit unsigned integer, 32 is the largest divisor of 96 out of these, so use `BUintD32<3>`. A 40-bit signed integer would be `BIntD8<5>`.

## Why bnum?

- **Zero dependencies by default**: `bnum` does not depend on any other crates by default. Support for crates such as [`rand`](https://docs.rs/rand/latest/rand/) and [`serde`](https://docs.rs/serde/latest/serde/) can be enabled with crate [features](#features).
- **`no-std` compatible**: `bnum` can be used in `no_std` environments, provided that the [`arbitrary`](#fuzzing) and [`quickcheck`](#quickcheck) features are not enabled.
- **Compile-time integer parsing**: the `from_str_radix` and `parse_str_radix` methods on `bnum` integers are `const`, which allows parsing of integers from string slices at compile time. Note that this is more powerful than compile-time parsing of integer literals. This is because it allows parsing of strings in all radices from `2` to `36` inclusive instead of just `2`, `8`, `10` and `16`. Additionally, the string to be parsed does not have to be a literal: it could, for example, be obtained via [`include_str!`](https://doc.rust-lang.org/core/macro.include_str.html), or [`env!`](https://doc.rust-lang.org/core/macro.env.html)`.
- **`const` evaluation**: nearly all methods defined on `bnum` integers are `const`, which allows complex compile-time calculations.

## Installation

To install and use `bnum`, simply add the following line to your `Cargo.toml` file in the `[dependencies]` section:

```toml
bnum = "0.10.0"
```

Or, to enable various `bnum` features as well, add for example this line instead:

```toml
bnum = { version = "0.10.0", features = ["rand"] } # enables the "rand" feature
```

## Example Usage

**NB: the examples in the documentation use specific type aliases (e.g. `U256`, `U512`,  or `I256`, `I512`) to give examples of correct usage for most methods. There is nothing special about these types in particular: all methods that are shown with these are implemented for all unsigned/signed `bnum` integers for any value of `N`.**

```rust
// As of version 0.6.0, you can parse integers from string slices at compile time with the const methods `from_str_radix` or `parse_str_radix`:
use bnum::types::{U256, I256};
use bnum::errors::ParseIntError;

// `parse_str_radix` returns an integer, and panics if the string fails to parse
const UINT_FROM_DECIMAL_STR: U256 = U256::parse_str_radix("12345678901234567890", 10);

// If you are not sure that the string will successfully parse, you can use `from_str_radix` which returns a `Result`
const RESULT_INT_FROM_HEXA_STR: Result<I256, ParseIntError> = I256::from_str_radix("-1234567890abcdef", 16);

assert_eq!(format!("{}", UINT_FROM_DECIMAL_STR), "12345678901234567890");
assert_eq!(format!("{:x}", RESULT_INT_FROM_HEXA_STR.unwrap().abs()), "1234567890abcdef");
```

```rust
// Calculate the `n`th Fibonacci number, using the type alias `U512`.

use bnum::types::U512; // `U512` is a type alias for a `BUint` which contains 8 `u64` digits

// Calculate the nth Fibonacci number
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

```rust
// Construct an 80-bit signed integer
// Out of [64, 32, 16, 8], 16 is the largest divisor of 80, so use `BIntD16`
use bnum::BIntD16;

type I80 = BIntD16<5>; // 80 / 16 = 5

let neg_one = I80::NEG_ONE; // -1
assert_eq!(neg_one.count_ones(), 80); // signed integers are stored in two's complement so `-1` is represented as `111111...`
```

## Features

### Fuzzing

The `arbitrary` feature derives the [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait from the [`arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/) crate. **Note: currently, this feature cannot be used with `no_std` (see <https://github.com/rust-fuzz/arbitrary/issues/38>).**

### Random Number Generation

The `rand` feature allows creation of random `bnum` integers via the [`rand`](https://docs.rs/rand/latest/rand/) crate.

### Serialization and Deserialization

The `serde` feature enables serialization and deserialization of `bnum` integers via the [`serde`](https://docs.rs/serde/latest/serde/) and [`serde_big_array`](https://docs.rs/serde-big-array/latest/serde_big_array/) crates.

### `num_traits` and `num_integer` trait implementations

The `numtraits` feature includes implementations of traits from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/) and [`num_integer`](https://docs.rs/num-integer/latest/num_integer/) crates, e.g. [`AsPrimitive`](https://docs.rs/num-traits/latest/num_traits/cast/trait.AsPrimitive.html), [`Signed`](https://docs.rs/num-traits/latest/num_traits/sign/trait.Signed.html), [`Integer`](https://docs.rs/num-integer/latest/num_integer/trait.Integer.html) and [`Roots`](https://docs.rs/num-integer/latest/num_integer/trait.Roots.html).

### Quickcheck

The `quickcheck` feature enables the [`Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) trait from the [`quickcheck`](https://docs.rs/quickcheck/latest/quickcheck/) crate. **Note: currently, this feature cannot be used with `no_std`.**

### Zeroize

The `zeroize` feature enables the [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) trait from the [`zeroize`](https://docs.rs/zeroize/latest/zeroize/) crate.

### Valuable

The `valuable` feature enables the [`Valuable`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html) trait from the [`valuable`](https://docs.rs/valuable/latest/valuable/) crate.

### Nightly features

Activating the `nightly` feature will enable the `from_be_bytes`, `from_le_bytes`, `from_ne_bytes`, `to_be_bytes`, `to_le_bytes` and `to_ne_bytes` methods on `bnum`'s unsigned and signed integers and will make the `unchecked_...` methods `const`. This comes at the cost of only being able to compile on nightly. The nightly features that this uses are [`generic_const_exprs`](https://github.com/rust-lang/rust/issues/76560), [`const_trait_impl`](https://github.com/rust-lang/rust/issues/67792) and [`const_option_ext`](https://github.com/rust-lang/rust/issues/91930).

## Testing

This crate is tested with the [`quickcheck`](https://docs.rs/quickcheck/latest/quickcheck/) crate as well as with specific edge cases. The outputs of methods are compared to the outputs of the equivalent methods of primitive integers to ensure that the behaviour is identical.

## Minimum Supported Rust Version

The current Minimum Supported Rust Version (MSRV) is `1.65.0`. <!-- TODO: check that this is inline with msrv specified in Cargo.toml-->

## Documentation

If a method is not documented explicitly, it will have a link to the equivalent method defined on primitive Rust integers (since the methods have the same functionality).

**NB: `bnum` is currently pre-`1.0.0`. As per the [Semantic Versioning guidelines](https://semver.org/#spec-item-4), the public API may contain breaking changes while it is in this stage. However, as the API is designed to be as similar as possible to the API of Rust's primitive integers, it is unlikely that there will be a large number of breaking changes.**

## Known Issues

At the moment, the [`From`](https://doc.rust-lang.org/core/convert/trait.From.html) trait is implemented for `bnum` integers, from all the Rust primitive integers. However, this behaviour is not quite correct. For example, if a 24-bit wide unsigned integer (`BUintD8<3>`) were created, this should not implement `From<u32>`, etc. and should implement `TryFrom<u32>` instead. To ensure correct behaviour, the [`FromPrimitive`](https://docs.rs/num-traits/latest/num_traits/cast/trait.FromPrimitive.html) trait from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/index.html) crate can be used instead, as this will always return an [`Option`](https://doc.rust-lang.org/core/option/enum.Option.html) rather than the integer itself.

The [`num_traits::NumCast`](https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html) trait is implemented for `bnum` integers but will intentionally panic if its method [`from`](https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html#tymethod.from) is called, as it is not possible to guarantee a correct conversion, due to trait bounds enforced by [`NumCast`](https://docs.rs/num-traits/latest/num_traits/cast/trait.NumCast.html). This trait should therefore never be used on `bnum` integers. The implementation exists only to allow implementation of the [`num_traits::PrimInt`](https://docs.rs/num-traits/latest/num_traits/int/trait.PrimInt.html) trait.

## Prior bugs

The short list of bugs in previous versions can be found at [`changes/prior-bugs.md`](https://github.com/isaacholt100/bnum/blob/master/changes/prior-bugs.md).

## Future Work

This library aims to provide arbitrary, fixed precision equivalents of Rust's 3 built-in number types: signed integers, unsigned integers and floats. Signed and unsigned integers have been implemented and fully tested, and will aim to keep up to date with Rust's integer interface. (e.g. when a new method is implemented on a Rust primitive integer, this library will attempt to keep in step to include that method as well. This includes nightly-only methods.)

Currently, arbitrary precision fixed size floats are being worked on but are incomplete. Most of the basic methods, such as arithmetic and classification, have been implemented, but at the moment there is no implementation of the transcendental floating point methods such as `sin`, `exp`, `log`, etc.

Additionally, a proc macro for parsing numeric values will be developed at some point.

## Licensing

`bnum` is licensed under either the MIT license or the Apache License 2.0.