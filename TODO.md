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
- Rand:
    - gen_range stuff
- num_traits::{Bounded, Float, FloatConst, FloatCore, AsPrimitive, FromPrimitive, ToPrimitive, FromBytes, ToBytes, Inv, MulAdd, MulAddAssign, Pow, Signed, Euclid, Num}
- Division algorithm which doesn't need where clause

## Ints

- Faster mulitplication algorithm for larger integers
- Faster division algorithms for larger integers
- isqrt methods
- Update serde to use decimal string instead of struct debug - but CHECK that all serde options serialise primitive ints as decimal strings
- create more efficient implementation of ilog10 (see e.g. Hacker's Delight book)
- modpow
- isolate_most_least_significant_one for uints, ints (but wait til the name is stabilised)
- faster algorithms for parsing and printing larger integers
- think about whether you could make to_str_radix and the functions it uses into generic functions which take an argument which "pushes" the next character to the existing string (so either pushing to a vector or calling write!(f, ...))
- unchecked_disjoint_bitor for uint (can do by iterating unchecked_disjoint bitor on u8s/u128 digits, can only add this once it is stablised for primitives though, not much point adding on nightly only)

## Crates to support

- Proptest


## Other things

- Replace bitors, bitands, shifts, masks etc. with more efficient implementations (e.g. using set_bit, flip_bit, one-less-than-power-of-two methods, methods for efficiently generating masks/getting certain range of bits of integer)
- consider raising issue in num_traits crate about PrimInt dependency on NumCast
- work out and add assertions about sizes of float mantissa and exponent widths, etc.
- maybe mention that serde impl is different from primitives
- check you're happy with the layout of the random crate-level module
- maybe add additional optimal type parameter (default u128) WideDigit, which specifies the wide digit to be iterated over (e.g. if memory usage really is a concern, or could somehow find a way of making this internal based on the size of the integer). this would also make testing of non standard bit widhts easier (just compare against use u8 wide digits)
- maybe rewrite code using while let Some(x) = iter.next() (using const iterator like methods), this will mean easier to migrate to iterators when they are const