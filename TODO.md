## Floats

- FromStr trait: REMEMBER: the num_traits crate has a general from_str_radix method for floats, could use if stuck
- Display, debug, upper exp, lower exp traits
- Transcendental functions:
	- exp
	- exp2
	- exp_m1
	- ln
	- ln_1p
	- log
	- log2
	- log10
	- cbrt
	- hypot
	- sin
	- cos
	- tan
	- asin
	- acos
	- atan
	- atan2
	- sin_cos
	- sinh
	- cosh
	- tanh
	- asinh
	- acosh
	- atanh
	- to_degrees
	- to_radians
    - powf
    - gamma
    - ln_gamma
- Other functions:
    - mul_add
    - midpoint
    - recip
- Optimised division algorithm depending on size of mantissa
- Optimised multiplication algorithm depending on size of mantissa
- Constants:
	- DIGITS. For this, we can divide MB by f64::LOG2_10 and take floor (roughly speaking).
	- MIN_10_EXP. For this, we can divide MIN_EXP by f64::LOG2_10, and take floor (roughly speaking)
	- MAX_10_EXP. For this, we can divide MAX_EXP by f64::LOG2_10, and take floor (roughly speaking)
- Maths constants:
	- E
	- FRAC_1_PI
	- FRAC_1_SQRT_2
	- FRAC_2_PI
	- FRAC_PI_2
	- FRAC_PI_3
	- FRAC_PI_4
	- FRAC_PI_6
	- FRAC_PI_8
	- LN_2
	- LN_10
	- LOG2_10
	- LOG2_E
	- LOG10_2
	- LOG10_E
	- PI
	- SQRT_2
	- TAU
- FloatToInt trait
- From/TryFrom trait for ints, other floats
- Float type aliases from IEEE standard: f16, f32, f64, f80, f128. (Include f32 and f64 as allows const methods which aren't available on the primitives)
- Rand:
    - gen_range stuff
- num_traits::{Bounded, Float, FloatConst, FloatCore, AsPrimitive, FromPrimitive, ToPrimitive, FromBytes, ToBytes, Inv, MulAdd, MulAddAssign, Pow, Signed, Euclid, Num}
- Division algorithm which doesn't need where clause

## Ints

- unsigned_signed_diff methods
- unchecked_neg for int
- isqrt methods
- Faster mulitplication algorithm for larger integers
- Faster division algorithms for larger integers
- Update serde to use decimal string instead of struct debug - but CHECK that all serde options serialise primitive ints as decimal strings
- do we need the from_be_slice and from_le_slice methods? (think we can just call from_radix_{b, l}e with radix 256)
- create more efficient implementation of ilog10 (see e.g. Hacker's Delight book)
- modpow
- isolate_most_least_significant_one (but wait til the name is stabilised)

## Crates to support

- Proptest


## Other things

- Replace bitors, bitands, shifts, masks etc. with more efficient implementations (e.g. using set_bit, flip_bit, one-less-than-power-of-two methods, methods for efficiently generating masks/getting certain range of bits of integer)
- Add 16 bit and 32 bit width types to the test widths, so test u16, u32, f16, f32 as well (just make the digit sizes that are too wide not do anything for those tests)
- Consider removing Div<Digit> impl
- Rewrite README
- consider raising issue in num_traits crate about PrimInt dependency on NumCast
- consider splitting off allow-based methods into gated "alloc" feature
- work out and add assertions about sizes of e.g. int widths (should be <= u32::MAX), and float mantissa and exponent widths, etc.
- include list of difference with primitives in README, e.g. overflow_checks not detected yet, serde implementation different, memory layout different (always little endian - although maybe this could be changed? probably not a good idea though)
- test using stable, only use nightly when need to test be_bytes methods
- check you're happy with the layout of the random crate-level module