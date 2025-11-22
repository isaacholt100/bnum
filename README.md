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

`bnum` provides _fully generic_ fixed-width numeric types.

## Overview

Rust provides 10 fixed-width integer types out of the box: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, and `i128`. This interface has several limitations:
- Only 5 possible bit widths: 8, 16, 32, 64, and 128 bits.
- The types are distinct rather than being generic over bit width, meaning that generic code cannot be written over arbitrary bit widths.
- Each type must have the same overflow behaviour, globally controlled by the `overflow-checks` flag: either panic on overflow if `overflow-checks` is enabled, or wrap on overflow if not. If you wanted to specify custom overflow behaviour, you would have to use the `Wrapping<T>` or `Saturating<T>` wrapper types from the standard library, which are cumbersome.

`bnum` addresses each of these limitations by providing a _single generic integer type_ `Integer`, which has const-generic parameters to specify:
- The width of the integer in bytes
- Whether the integer is signed or unsigned.
- The overflow behaviour of the integer: wrapping, saturating, or panicking.

<!-- More specifically, `Integer<S, N, OM>` specifies an integer type which:
- Is signed if `S` is `true`, and unsigned if `S` is `false`.
- Has a width of `N * 8` bits.
- Has overflow behaviour specified by `OM`:
  - `OM = 0`: arithmetic operations wrap on overflow.
  - `OM = 1`: arithmetic operations panic on overflow.
  - `OM = 2`: arithmetic operations saturate on overflow.

For example, a useful type in cryptography would be `Integer<false, 64, 0>`: an unsigned integer type with a width of `64 * 8 = 512` bits, and which has explicit wrapping arithmetic.  TODO: move to Integer docs-->

`bnum` also provides a macro `n!` for easily creating `bnum` integers from integer literals, and a macro `nt!` for specifying `Integer` types with specific parameters from type descriptors.

To illustrate the power of this generic interface, here is a simple example:

```rust
use bnum::prelude::*; // imports common use items, including the `Integer` type and the `n!` macro

// say we want to implement a polynomial which works over any unsigned or signed integer
// of any byte width and with any overflow behaviour
// for example, the polynomial could be p(x) = 2x^3 + 3x^2 + 5x + 7

fn p<const S: bool, const N: usize, const B: usize, const OM: u8>(x: Integer<S, N, B, OM>) -> Integer<S, N, B, OM> {
    n!(2)*x.pow(3) + n!(3)*x.pow(2) + n!(5)*x + n!(7)
    // type inference means we don't need to specify the width of the integers in the n! macro
}

// 2*10^3 + 3*10^2 + 5*10 + 7 = 2357
assert_eq!(p(n!(10 U256)), n!(2357));
// evaluates p(10) as a 16-bit unsigned integer

type U24w = nt!(U24w);
// 24-bit unsigned integer with wrapping arithmetic
type I40s = nt!(I40s);
// 40-bit signed integer with saturating arithmetic
type U48p = nt!(U48p);
// 48-bit unsigned integer with panicking arithmetic

let a = p(U24w::MAX); // result wraps around and doesn't panic
let b = p(I40s::MAX); // result is too large to be contained in I40, so saturates to I40::MAX
// let c = p(U48p::MAX); // this would result in panic due to overflow
```
  
For more information on the `Integer` type and the `n!` and `nt!` macros, see the crate level documentation.

## Key features

- **Maximally space- and time-efficient**: for most libraries providing fixed-width uints, there is a trade-off between flexibility of bit widths, space efficiency, and time efficiency:
  - If the integer is stored as an array of wide digits (e.g. `u64`), then this either limits the bit width to be a multiple of `64`, or means that there is redundant storage for bit widths which are not multiples of `64`.
  - If integer is stored as an array of narrow digits (e.g. `u8`), then operations are slower (as there are more digits to process).
  
  `bnum` avoid this trade-off and achieves both space and time efficiency by storing integers as arrays of `u8` digits, but "chunking" these digits together into wide `u128` digits.
- **Strict adherence to Rust integer API**: the API of `Integer` follows the API of the Rust standard library's integer types as closely as possible. Effectively every method available on the integer types from `std` is also available on `Integer`, and has the same behaviour.
- **Zero dependencies by default**: `bnum` does not depend on any other crates by default. Support for crates such as [`rand`](https://docs.rs/rand/latest/rand/) and [`serde`](https://docs.rs/serde/latest/serde/) can be enabled with crate [features](#crate-features).
- **`no-std` and `no-alloc` compatible**: `bnum` can be used in `no_std` environments, provided that the [`arbitrary`](#fuzzing) and [`quickcheck`](#quickcheck) features are not enabled. It can also be used in `no-alloc` environments, with the only methods unavailable here being formatting and conversion to strings/vectors of digits in a given radix.
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

assert_eq!(format!("{}", UINT_FROM_DECIMAL_STR), "12345678901234567890");
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

println!("The {}th Fibonacci number is {}", n, f_n);
// Prints "The 100th Fibonacci number is 354224848179261915075"

assert_eq!(f_n, n!(354224848179261915075));
```

## Crate features

| Feature name | Default? | Enables... |
|--------------|----------|------------|
| `alloc`      | Yes      | Methods which require a global allocator (i.e. formatting and radix conversion). |
| `arbitrary`  | No       | The [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) trait from the [`arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/) crate. **Note: currently, this feature cannot be used with `no_std` (see <https://github.com/rust-fuzz/arbitrary/issues/38>).** |
| `rand`       | No       | Creation of random `bnum` types via the [`rand`](https://docs.rs/rand/latest/rand/) crate. |
| `serde`      | No       | Serialization and deserialization via the [`serde`](https://docs.rs/serde/latest/serde/) and [`serde_big_array`](https://docs.rs/serde-big-array/latest/serde_big_array/) crates. |
| `borsh`      | No       | Serialization and deserialization via the [`borsh`](https://docs.rs/borsh/latest/borsh/) crate. |
| `numtraits`  | No       | Implementations of traits from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/) and [`num_integer`](https://docs.rs/num-integer/latest/num_integer/) crates, such as [`AsPrimitive`](https://docs.rs/num-traits/latest/num_traits/cast/trait.AsPrimitive.html), [`Signed`](https://docs.rs/num-traits/latest/num_traits/sign/trait.Signed.html), [`Integer`](https://docs.rs/num-integer/latest/num_integer/trait.Integer.html) and [`Roots`](https://docs.rs/num-integer/latest/num_integer/trait.Roots.html). |
| `quickcheck` | No       | The [`Arbitrary`](https://docs.rs/quickcheck/latest/quickcheck/trait.Arbitrary.html) trait from the [`quickcheck`](https://docs.rs/quickcheck/latest/quickcheck/) crate. **Note: currently, this feature cannot be used with `no_std`.** |
| `zeroize`    | No       | The [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) trait from the [`zeroize`](https://docs.rs/zeroize/latest/zeroize/) crate. |
| `valuable`   | No       | The [`Valuable`](https://docs.rs/valuable/latest/valuable/trait.Valuable.html) trait from the [`valuable`](https://docs.rs/valuable/latest/valuable/) crate. |
| `nightly`    | No       | Testing methods whose counterparts on the primitive integers are only available on nightly. |

## Testing

This crate is tested with the [`quickcheck`](https://docs.rs/quickcheck/latest/quickcheck/) crate as well as with specific edge cases. The outputs of methods are compared to the outputs of the equivalent methods of primitive integers to ensure that the behaviour is identical.

## Minimum Supported Rust Version

The current Minimum Supported Rust Version (MSRV) is `1.86.0`. <!-- TODO: check that this is inline with msrv specified in Cargo.toml-->

## Roadmap

- Faster algorithms for certain operations on large integers, such as multiplication and division.
- Implement a generic floating point type `Float<N, MB>`, where `N` is the byte width and `MB` is the number of mantissa bits. This will have the same API and behaviour as `f32` and `f64`.

## License

`bnum` is licensed under either the MIT license or the Apache License 2.0.

