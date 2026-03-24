# bnum

[![GitHub](https://img.shields.io/badge/GitHub-isaacholt100/bnum-default?logo=github)](https://github.com/isaacholt100/bnum)
[![docs.rs](https://img.shields.io/docsrs/bnum?logo=docsdotrs)](https://docs.rs/bnum/latest/bnum)
[![Crates.io](https://img.shields.io/crates/d/bnum?logo=rust)
](https://crates.io/crates/bnum)
[![Dependents](https://img.shields.io/crates/dependents/bnum)](https://crates.io/crates/bnum/reverse_dependencies)
[![MSRV](https://img.shields.io/crates/msrv/bnum)](https://crates.io/crates/bnum)
[![dependency status](https://deps.rs/repo/github/isaacholt100/bnum/status.svg)](https://deps.rs/repo/github/isaacholt100/bnum)
[![codecov](https://codecov.io/gh/isaacholt100/bnum/branch/master/graph/badge.svg)](https://codecov.io/gh/isaacholt100/bnum)
[![license](https://img.shields.io/crates/l/bnum)](https://github.com/isaacholt100/bnum)

`bnum` provides _fully generic_ fixed-width integer types.

## Overview

Rust provides 12 fixed-width integer types out of the box: `uN` and `iN`, for `N = 8, 16, 32, 64, 128`, as well as `usize` and `isize`. This interface has several limitations:
- Only a few possible bit widths.
- The types are distinct rather than being generic over bit width and signedness, meaning that generic code cannot automatically be written over arbitrary bit widths or signedness.
- Each type has the same overflow behaviour, which is globally controlled by the `overflow-checks` flag: either panic on overflow if `overflow-checks` is enabled, or wrap on overflow if not. To specify custom overflow behaviour, the `Wrapping<T>` or `Saturating<T>` wrapper types from the standard library must be used, which are cumbersome.

`bnum` addresses each of these limitations by providing a _single generic integer type_ `Integer`, which has const-generic parameters to specify:
- The bit width of the integer: any `usize` between `2` and `2^32 - 1`.
- Whether the integer type is signed or unsigned.
- The overflow behaviour of the integer: wrap, saturate, or panic.

`bnum` also provides a macro `n!` for easily creating `bnum` integers from integer literals, and a macro `t!` for specifying `Integer` types with specific parameters from type descriptors.

To illustrate the power of this generic interface, here is a simple example:

```rust
use bnum::prelude::*;
// imports common use items
// including the `Integer` type and the `n!` macro

// say we want to write a polynomial function
// which takes any unsigned or signed integer
// of any bit width and with any overflow behaviour
// for example, the polynomial could be p(x) = 2x^3 + 3x^2 + 5x + 7

fn p<const S: bool, const N: usize, const B: usize, const OM: u8>(x: Integer<S, N, B, OM>) -> Integer<S, N, B, OM> {
    n!(2)*x.pow(3) + n!(3)*x.pow(2) + n!(5)*x + n!(7)
    // type inference means we don't need to specify the width of the integers in the n! macro
}

// 2*10^3 + 3*10^2 + 5*10 + 7 = 2357
assert_eq!(p(n!(10U256)), n!(2357));
// evaluates p(10) as a 256-bit unsigned integer

type U24w = t!(U24w);
// 24-bit unsigned integer with wrapping arithmetic
type I1044s = t!(I1044s);
// 1044-bit signed integer with saturating arithmetic
type U753p = t!(U753p);
// 753-bit unsigned integer that panics on arithmetic overflow

let a = p(U24w::MAX); // result wraps around and doesn't panic
let b = p(I1044s::MAX); // result is too large to be represented by the type, so saturates to I044s::MAX
// let c = p(U753p::MAX); // this would result in panic due to overflow
```
  
For more information on the `Integer` type and the `n!` and `t!` macros, see the item-level documentation of each.

## Key features

- **Maximally space- and time-efficient**: for most libraries providing fixed-width integers, there is a trade-off between flexibility of bit widths, space efficiency, and time efficiency:
  - If the integer is stored as an array of wider digits (e.g. `u64`), then this either limits the bit width to be a multiple of `64`, or means that there is redundant storage for bit widths which are not multiples of `64`.
  - If the integer is stored as an array of narrower digits (e.g. `u8`), then operations are generally slower (as there are more digits to process).
  
  `bnum` avoid this trade-off and achieves maximal space _and_ time efficiency by storing integers as arrays of `u8` digits; when operations are performed on the integers, the `u8` digits are "chunked" into wider digits. The width of these wider digits is determined by benchmarking the operation with each possible choice of wide digit, and choosing the fastest.
- **Highly generic and customisable**: the bit width, signedness, and overflow behaviour of the integer are specified as const-generic parameters, which allows for writing generic code without incurring any computational overhead.
- **Fast creation of values from integer literals**: the `n!` macro allows for readable construction of `Integer`s from integer literals. `n!` is a declarative rather than procedural macro, so adds minimal compile-time overhead.
- **Strict adherence to Rust integer API**: the API of `Integer` mimics the API of the Rust standard library's integer types as closely as possible. Effectively every method available on the integer types from `std` is also available on `Integer`, and has the same behaviour.
- **Zero dependencies by default**: `bnum` does not depend on any other crates by default. Support for crates such as [`rand`](https://docs.rs/rand/latest/rand/) and [`serde`](https://docs.rs/serde/latest/serde/) can be enabled with crate [features](#crate-features).
- **`no-std` and `no-alloc` compatible**: `bnum` can be used in `no_std` environments, provided that the [`arbitrary`](#fuzzing) and [`quickcheck`](#quickcheck) features are not enabled. If the `alloc` feature is disabled, it can also be used in `no-alloc` environments, with the only methods unavailable here being formatting and conversion to strings/vectors of digits in a given radix.
- **`const` evaluation**: nearly all methods defined on `bnum` integers are `const`, which allows for complex compile-time calculations. This includes parsing integers from strings via `from_str_radix`.

## Further examples

```rust
// Parsing a string in a given radix into an integer at compile time
use bnum::types::U256;
use bnum::errors::ParseIntError;

const UINT_FROM_DECIMAL_STR: U256 = match U256::from_str_radix("12345678901234567890", 10) {
    Ok(val) => val,
    Err(e) => panic!("Failed to parse integer"),
};
```

```rust
// Calculate the `n`th Fibonacci number, using the type alias `U512`.
use bnum::prelude::*;
use bnum::types::U512; // `U512` is a type alias for a `Uint` which contains 64 `u8` digits

// Calculate the nth Fibonacci number
fn fibonacci(n: usize) -> U512 {
    let mut f_n: U512 = n!(0);
    let mut f_n_next = n!(1);

    for _ in 0..n {
        let temp = f_n_next;
        f_n_next += f_n;
        f_n = temp;
    }

    f_n
}

let n = 100;
let f_n = fibonacci(n);

assert_eq!(f_n, n!(354224848179261915075));
```

## Crate features

| Feature name | Default? | Enables... |
|--------------|----------|------------|
| `alloc`      | Yes      | Methods which require a global allocator (i.e. formatting and radix conversion). |
| `arbitrary`  | No       | Implementation of the [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait from the [`arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/) crate. **Note: currently, this feature cannot be used with `no_std` (see [this issue](https://github.com/rust-fuzz/arbitrary/issues/38)).** |
| `rand`       | No       | Generate random `Integer` values via the [`rand`](https://docs.rs/rand/latest/rand/) crate. |
| `serde`      | No       | Serialization and deserialization via the [`serde`](https://docs.rs/serde/latest/serde/) and [`serde_big_array`](https://docs.rs/serde-big-array/latest/serde_big_array/) crates. |
| `borsh`      | No       | Serialization and deserialization via the [`borsh`](https://docs.rs/borsh/latest/borsh/) crate. |
| `numtraits`  | No       | Implementations of all relevant traits from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/) and [`num_integer`](https://docs.rs/num-integer/latest/num_integer/) crates, such as [`AsPrimitive`](https://docs.rs/num-traits/latest/num_traits/cast/trait.AsPrimitive.html), [`Signed`](https://docs.rs/num-traits/latest/num_traits/sign/trait.Signed.html), [`Integer`](https://docs.rs/num-integer/latest/num_integer/trait.Integer.html) and [`Roots`](https://docs.rs/num-integer/latest/num_integer/trait.Roots.html). |
| `quickcheck` | No       | Implementation of the [`Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) trait from the [`quickcheck`](https://docs.rs/quickcheck/latest/quickcheck/) crate. **Note: currently, this feature cannot be used with `no_std`.** |
| `zeroize`    | No       | Implementation of the [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) trait from the [`zeroize`](https://docs.rs/zeroize/latest/zeroize/) crate. |
| `valuable`   | No       | Implementation of the [`Valuable`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html) trait from the [`valuable`](https://docs.rs/valuable/latest/valuable/) crate. |

## Testing

This crate is tested with the [`quickcheck`](https://docs.rs/quickcheck/latest/quickcheck/) crate as well as with specific edge cases. The outputs of methods are compared to the outputs of the equivalent methods of primitive integers to ensure that the behaviour is identical.

## Minimum Supported Rust Version

The current Minimum Supported Rust Version (MSRV) is `1.87.0`. <!-- TODO: check that this is inline with msrv specified in Cargo.toml-->

## Prior bugs

The short list of bugs in previous versions can be found at [`changes/prior-bugs.md`](https://github.com/isaacholt100/bnum/blob/master/changes/prior-bugs.md) in the GitHub repository.

## Roadmap

- Faster algorithms for certain operations on large integers, such as multiplication and division.
- Implement a generic floating point type `Float`, with const-generic parameters to specify the bit width and number of mantissa bits. This will have the same API and behaviour as `f32` and `f64`.

## License

`bnum` is licensed under either the MIT license or the Apache License 2.0.

