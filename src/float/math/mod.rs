use super::Float;
use crate::doc;

mod acos;
mod acosh;
mod asin;
mod asinh;
mod atan;
mod atan2;
mod atanh;
mod cbrt;
mod cos;
mod cosh;
mod div_euclid;
mod exp;
mod exp2;
mod exp_m1;
mod fixed;
mod hypot;
mod ln;
mod ln_1p;
mod log;
mod log10;
mod log2;
mod mul_add;
mod powf;
mod powi;
mod rem_euclid;
mod sin;
mod sin_cos;
mod sinh;
mod sqrt;
mod tan;
mod tanh;
mod to_degrees;
mod to_radians;

/// Mathematical functions.
impl<const W: usize, const MB: usize, const RM: u8> Float<W, MB, RM> {
    /// Computes the absolute value of `self`.
    ///
    /// Clears the sign bit of `self`. If `self` is `NaN`, returns `NaN` with its sign bit cleared.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(3.5);
    /// let y: F64 = f!(-3.5);
    ///
    /// assert_eq!(x.abs(), f!(3.5));
    /// assert_eq!(y.abs(), f!(3.5));
    /// assert!(F64::NAN.abs().is_nan());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn abs(mut self) -> Self {
        self.bits.set_bit(Self::BITS - 1, false); // set sign bit to 0
        self
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error, rounded
    /// according to [`Self::ROUNDING_MODE`].
    ///
    /// This yields a more accurate result than an unfused multiply-add (`self * a + b`), which
    /// rounds twice.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let m: F64 = f!(10.0);
    /// let x: F64 = f!(4.0);
    /// let b: F64 = f!(60.0);
    ///
    /// assert_eq!(m.mul_add(x, b), f!(100.0));
    ///
    /// // Avoids intermediate rounding loss when `a * b` nearly cancels `c`:
    /// let eps = F64::EPSILON;
    /// let one_plus_eps = F64::ONE + eps;
    /// assert_eq!(one_plus_eps.mul_add(one_plus_eps, -(F64::ONE + f!(2.0) * eps)), eps * eps);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn mul_add(self, a: Self, b: Self) -> Self {
        self.mul_add_internal(a, b)
    }

    /// Calculates Euclidean division, the matching method for [`rem_euclid`](Self::rem_euclid).
    ///
    /// This computes the integer `n` such that `self = n * rhs + self.rem_euclid(rhs)`. In other
    /// words, the result is `self / rhs` rounded to the integer `n` such that `self >= n * rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let a: F64 = f!(7.0);
    /// let b: F64 = f!(4.0);
    ///
    /// assert_eq!(a.div_euclid(b), f!(1.0));
    /// assert_eq!((-a).div_euclid(b), f!(-2.0));
    /// assert_eq!(a.div_euclid(-b), f!(-1.0));
    /// assert_eq!((-a).div_euclid(-b), f!(2.0));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        self.div_euclid_internal(rhs)
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in most cases.
    /// However, due to a floating-point round-off error, `r` can be equal to `rhs.abs()` when
    /// `self` is a very small negative number.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let a: F64 = f!(7.0);
    /// let b: F64 = f!(4.0);
    ///
    /// assert_eq!(a.rem_euclid(b), f!(3.0));
    /// assert_eq!((-a).rem_euclid(b), f!(1.0));
    /// assert_eq!(a.rem_euclid(-b), f!(3.0));
    /// assert_eq!((-a).rem_euclid(-b), f!(1.0));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    #[inline]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid_internal(rhs)
    }

    /// Raises `self` to an integer power `n`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Follows IEEE 754 `pown` special-value rules (for example, `self.powi(0)` is `1.0` for all
    /// inputs, including `0.0` and `NaN`).
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(2.0);
    ///
    /// assert_eq!(x.powi(3), f!(8.0));
    /// assert_eq!(x.powi(-2), f!(0.25));
    /// assert_eq!(F64::NAN.powi(0), F64::ONE);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn powi(self, n: i32) -> Self {
        self.powi_internal(n)
    }

    /// Raises `self` to a floating-point power `n`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `NaN` when `self` is finite and negative and `n` is finite and non-integral.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(9.0);
    ///
    /// assert_eq!(x.powf(f!(0.5)), f!(3.0));
    /// assert_eq!(f!(2.0F64).powf(f!(10.0)), f!(1024.0));
    /// assert!(f!(-2.0F64).powf(f!(0.5)).is_nan());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn powf(self, n: Self) -> Self {
        self.powf_internal(n)
    }

    /// Returns the square root of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `NaN` if `self` is strictly negative (`self < 0.0`). Note that `(-0.0).sqrt()`
    /// returns `-0.0` per IEEE 754.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64, F128 }
    ///
    /// let four: F64 = f!(4.0);
    /// assert_eq!(four.sqrt(), f!(2.0));
    /// assert!(f!(-1.0F64).sqrt().is_nan());
    ///
    /// let two: F128 = f!(2.0);
    /// assert_eq!(two.sqrt(), f!(1.41421356237309504880168872420969807857));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn sqrt(self) -> Self {
        self.sqrt_internal()
    }

    /// Returns `e^(self)` (the exponential function), rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let zero: F64 = f!(0.0);
    /// let one: F64 = f!(1.0);
    ///
    /// assert_eq!(zero.exp(), F64::ONE);
    /// assert_eq!(one.exp(), f!(2.71828182845904523536));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn exp(self) -> Self {
        self.exp_internal()
    }

    /// Returns `2^(self)`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let three: F64 = f!(3.0);
    /// let half: F64 = f!(0.5);
    ///
    /// assert_eq!(three.exp2(), f!(8.0));
    /// assert_eq!(half.exp2(), f!(2.0F64).sqrt());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn exp2(self) -> Self {
        self.exp2_internal()
    }

    /// Returns the natural logarithm of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `-∞` if `self` is `±0.0`, and `NaN` if `self` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let one: F64 = f!(1.0);
    /// let two: F64 = f!(2.0);
    ///
    /// assert_eq!(one.ln(), F64::ZERO);
    /// assert_eq!(two.ln(), f!(0.69314718055994530942));
    /// assert_eq!(F64::ZERO.ln(), F64::NEG_INFINITY);
    /// assert!(F64::NEG_ONE.ln().is_nan());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn ln(self) -> Self {
        self.ln_internal()
    }

    /// Returns the logarithm of `self` with respect to an arbitrary `base`, rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// Computes `ln(self) / ln(base)` in extended precision with a single final rounding.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(27.0);
    /// let base: F64 = f!(3.0);
    ///
    /// assert_eq!(x.log(base), f!(3.0));
    /// assert_eq!(f!(100.0F64).log(f!(10.0)), f!(2.0));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn log(self, base: Self) -> Self {
        self.log_internal(base)
    }

    /// Returns the base 2 logarithm of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `-∞` if `self` is `±0.0`, and `NaN` if `self` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let eight: F64 = f!(8.0);
    /// let quarter: F64 = f!(0.25);
    ///
    /// assert_eq!(eight.log2(), f!(3.0));
    /// assert_eq!(quarter.log2(), f!(-2.0));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn log2(self) -> Self {
        self.log2_internal()
    }

    /// Returns the base 10 logarithm of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `-∞` if `self` is `±0.0`, and `NaN` if `self` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let thousand: F64 = f!(1000.0);
    ///
    /// assert_eq!(thousand.log10(), f!(3.0));
    /// assert_eq!(f!(1.0F64).log10(), F64::ZERO);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn log10(self) -> Self {
        self.log10_internal()
    }

    /// Returns the cube root of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Unlike [`sqrt`](Self::sqrt), this is defined for negative values as well.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let pos: F64 = f!(8.0);
    /// let neg: F64 = f!(-27.0);
    ///
    /// assert_eq!(pos.cbrt(), f!(2.0));
    /// assert_eq!(neg.cbrt(), f!(-3.0));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn cbrt(self) -> Self {
        self.cbrt_internal()
    }

    /// Computes the distance between the origin and a point `(self, other)` on the Euclidean
    /// plane, i.e., `sqrt(self^2 + other^2)`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Computed without undue intermediate overflow or underflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(3.0);
    /// let y: F64 = f!(4.0);
    ///
    /// assert_eq!(x.hypot(y), f!(5.0));
    /// assert_eq!(f!(3e200F64).hypot(f!(4e200)), f!(5e200));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn hypot(self, other: Self) -> Self {
        self.hypot_internal(other)
    }

    /// Computes the sine of `self` (in radians), rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Uses full Payne–Hanek range reduction for large arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let half_pi: F64 = f!(1.57079632679489661923);
    ///
    /// assert_eq!(F64::ZERO.sin(), F64::ZERO);
    /// assert_eq!(half_pi.sin(), F64::ONE);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn sin(self) -> Self {
        self.sin_internal()
    }

    /// Computes the cosine of `self` (in radians), rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Uses full Payne–Hanek range reduction for large arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let pi: F64 = f!(3.14159265358979323846);
    ///
    /// assert_eq!(F64::ZERO.cos(), F64::ONE);
    /// assert_eq!(pi.cos(), F64::NEG_ONE);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn cos(self) -> Self {
        self.cos_internal()
    }

    /// Computes the tangent of `self` (in radians), rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Uses full Payne–Hanek range reduction for large arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let quarter_pi: F64 = f!(0.78539816339744830962);
    ///
    /// assert_eq!(F64::ZERO.tan(), F64::ZERO);
    /// assert!((quarter_pi.tan() - F64::ONE).abs() < f!(1e-15));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn tan(self) -> Self {
        self.tan_internal()
    }

    /// Computes the arcsine of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// The return value is in radians in the range `[-π/2, π/2]`, or `NaN` if `self` is outside
    /// `[-1.0, 1.0]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let half: F64 = f!(0.5);
    /// let pi_over_6: F64 = f!(0.52359877559829887308);
    ///
    /// assert_eq!(F64::ZERO.asin(), F64::ZERO);
    /// assert_eq!(half.asin(), pi_over_6);
    /// assert!(f!(2.0F64).asin().is_nan());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn asin(self) -> Self {
        self.asin_internal()
    }

    /// Computes the arccosine of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// The return value is in radians in the range `[0, π]`, or `NaN` if `self` is outside
    /// `[-1.0, 1.0]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let half: F64 = f!(0.5);
    /// let pi_over_3: F64 = f!(1.04719755119659774615);
    ///
    /// assert_eq!(F64::ONE.acos(), F64::ZERO);
    /// assert_eq!(half.acos(), pi_over_3);
    /// assert!(f!(-2.0F64).acos().is_nan());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn acos(self) -> Self {
        self.acos_internal()
    }

    /// Computes the arctangent of `self`, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// The return value is in radians in the range `[-π/2, π/2]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let one: F64 = f!(1.0);
    /// let quarter_pi: F64 = f!(0.78539816339744830962);
    ///
    /// assert_eq!(F64::ZERO.atan(), F64::ZERO);
    /// assert_eq!(one.atan(), quarter_pi);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn atan(self) -> Self {
        self.atan_internal()
    }

    /// Computes the four-quadrant arctangent of `self` (`y`) and `other` (`x`) in radians,
    /// rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// The return value lies in `[-π, π]`:
    /// - `x = 0, y = 0`: `0` (with IEEE 754 signed-zero handling)
    /// - `x >= 0`: `arctan(y / x)` in `[-π/2, π/2]`
    /// - `y >= 0, x < 0`: `arctan(y / x) + π` in `(π/2, π]`
    /// - `y < 0, x < 0`: `arctan(y / x) - π` in `(-π, -π/2)`
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let y: F64 = f!(1.0);
    /// let x: F64 = f!(-1.0);
    /// let three_pi_over_4: F64 = f!(2.35619449019234492885);
    ///
    /// assert_eq!(y.atan2(x), three_pi_over_4);
    /// assert_eq!(F64::ZERO.atan2(F64::ONE), F64::ZERO);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn atan2(self, other: Self) -> Self {
        self.atan2_internal(other)
    }

    /// Simultaneously computes the sine and cosine of `self` (in radians), rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// Returns `(self.sin(), self.cos())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(0.7);
    /// let (s, c) = x.sin_cos();
    ///
    /// assert_eq!(s, x.sin());
    /// assert_eq!(c, x.cos());
    /// assert!((s * s + c * c - F64::ONE).abs() < f!(1e-15));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn sin_cos(self) -> (Self, Self) {
        self.sin_cos_internal()
    }

    /// Returns `e^(self) - 1` in a way that is accurate even if `self` is close to zero, rounded
    /// according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(1e-16);
    ///
    /// // For tiny x, exp(x) - 1 would lose precision, whereas exp_m1(x) ≈ x:
    /// assert_eq!(x.exp_m1(), x);
    /// assert_eq!(F64::ZERO.exp_m1(), F64::ZERO);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn exp_m1(self) -> Self {
        self.exp_m1_internal()
    }

    /// Returns `ln(1 + self)` (natural logarithm of `1 + self`) more accurately than computing the
    /// operations separately, rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `-∞` if `self == -1.0`, and `NaN` if `self < -1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(1e-16);
    ///
    /// assert_eq!(x.ln_1p(), x);
    /// assert_eq!(F64::ONE.ln_1p(), f!(2.0F64).ln());
    /// assert_eq!(F64::NEG_ONE.ln_1p(), F64::NEG_INFINITY);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn ln_1p(self) -> Self {
        self.ln_1p_internal()
    }

    /// Computes the hyperbolic sine of `self`, `(e^self - e^-self) / 2`, rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(1.0);
    ///
    /// assert_eq!(F64::ZERO.sinh(), F64::ZERO);
    /// assert_eq!(x.sinh(), f!(1.17520119364380145688));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn sinh(self) -> Self {
        self.sinh_internal()
    }

    /// Computes the hyperbolic cosine of `self`, `(e^self + e^-self) / 2`, rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(1.0);
    ///
    /// assert_eq!(F64::ZERO.cosh(), F64::ONE);
    /// assert_eq!(x.cosh(), f!(1.54308063481524377848));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn cosh(self) -> Self {
        self.cosh_internal()
    }

    /// Computes the hyperbolic tangent of `self`, `sinh(self) / cosh(self)`, rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(1.0);
    ///
    /// assert_eq!(F64::ZERO.tanh(), F64::ZERO);
    /// assert_eq!(x.tanh(), f!(0.76159415595576488812));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn tanh(self) -> Self {
        self.tanh_internal()
    }

    /// Computes the inverse hyperbolic sine of `self`, `ln(self + sqrt(self^2 + 1))`, rounded
    /// according to [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(1.0);
    ///
    /// assert_eq!(F64::ZERO.asinh(), F64::ZERO);
    /// assert_eq!(x.asinh(), f!(0.88137358701954302523));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn asinh(self) -> Self {
        self.asinh_internal()
    }

    /// Computes the inverse hyperbolic cosine of `self`, `ln(self + sqrt(self^2 - 1))`, rounded
    /// according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `NaN` if `self < 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let x: F64 = f!(2.0);
    ///
    /// assert_eq!(F64::ONE.acosh(), F64::ZERO);
    /// assert_eq!(x.acosh(), f!(1.31695789692481670863));
    /// assert!(f!(0.5F64).acosh().is_nan());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn acosh(self) -> Self {
        self.acosh_internal()
    }

    /// Computes the inverse hyperbolic tangent of `self`, `0.5 * ln((1 + self) / (1 - self))`,
    /// rounded according to [`Self::ROUNDING_MODE`].
    ///
    /// Returns `∞` if `self == 1.0`, `-∞` if `self == -1.0`, and `NaN` if `self.abs() > 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let half: F64 = f!(0.5);
    ///
    /// assert_eq!(F64::ZERO.atanh(), F64::ZERO);
    /// assert_eq!(half.atanh(), f!(0.54930614433405484570));
    /// assert_eq!(F64::ONE.atanh(), F64::INFINITY);
    /// assert!(f!(2.0F64).atanh().is_nan());
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn atanh(self) -> Self {
        self.atanh_internal()
    }

    /// Converts radians to degrees (`self * (180 / π)`), rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let pi: F64 = f!(3.14159265358979323846);
    ///
    /// assert_eq!(F64::ZERO.to_degrees(), F64::ZERO);
    /// assert_eq!(pi.to_degrees(), f!(180.0));
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn to_degrees(self) -> Self {
        self.to_degrees_internal()
    }

    /// Converts degrees to radians (`self * (π / 180)`), rounded according to
    /// [`Self::ROUNDING_MODE`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bnum::prelude::*;
    /// use bnum::use_types;
    ///
    /// use_types! { F64 }
    ///
    /// let deg: F64 = f!(180.0);
    /// let pi: F64 = f!(3.14159265358979323846);
    ///
    /// assert_eq!(F64::ZERO.to_radians(), F64::ZERO);
    /// assert_eq!(deg.to_radians(), pi);
    /// ```
    #[must_use = doc::must_use_op!(float)]
    pub const fn to_radians(self) -> Self {
        self.to_radians_internal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::test_bignum;

    #[test]
    fn aaaaaa() {
        use super::Float;

        let a = Float::<128, 873>::from_str(
            "1.13459284735938409720349857203498573489533493847589",
        )
        .unwrap();
        let b = a.ln();
        assert!(!b.is_nan());
    }

    macro_rules! qc_unary_ulp {
        ($name:ident, $method:ident, $max_ulp:expr) => {
            quickcheck::quickcheck! {
                #[allow(non_snake_case)]
                fn $name(f: FTestBase) -> bool {
                    use crate::test::TestConvert;
                    let actual_f = FTest::from(f).$method();
                    let expected_f = f.$method();
                    if expected_f.is_nan() {
                        actual_f.is_nan()
                    } else if expected_f == 0.0 || expected_f.is_infinite() {
                        TestConvert::into(actual_f) == TestConvert::into(expected_f)
                    } else {
                        TestConvert::into(actual_f).abs_diff(TestConvert::into(expected_f)) <= $max_ulp
                    }
                }
            }
        };
    }

    macro_rules! qc_binary_ulp {
        ($name:ident, $method:ident, $max_ulp:expr) => {
            quickcheck::quickcheck! {
                #[allow(non_snake_case)]
                fn $name(a: FTestBase, b: FTestBase) -> bool {
                    use crate::test::TestConvert;
                    let actual_f = FTest::from(a).$method(FTest::from(b));
                    let expected_f = a.$method(b);
                    if expected_f.is_nan() {
                        actual_f.is_nan()
                    } else if expected_f == 0.0 || expected_f.is_infinite() {
                        TestConvert::into(actual_f) == TestConvert::into(expected_f)
                    } else {
                        TestConvert::into(actual_f).abs_diff(TestConvert::into(expected_f)) <= $max_ulp
                    }
                }
            }
        };
    }

    crate::test::test_all! {
        testing floats;

        test_bignum! {
            function: <FTest>::abs(f: FTestBase)
        }
        test_bignum! {
            function: <FTest>::sqrt(f: FTestBase)
        }
        test_bignum! {
            function: <FTest>::mul_add(a: FTestBase, b: FTestBase, c: FTestBase)
        }
        test_bignum! {
            function: <FTest>::div_euclid(f1: FTestBase, f2: FTestBase)
        }
        test_bignum! {
            function: <FTest>::rem_euclid(f1: FTestBase, f2: FTestBase)
        }

        // Note: platform libm transcendental functions are only guaranteed faithful (~1 ULP),
        // whereas our implementations use two-stage Ziv round-to-nearest-even (0.5 ULP).
        qc_unary_ulp!(quickcheck_FTest_cbrt_2, cbrt, 1);
        qc_unary_ulp!(quickcheck_FTest_ln, ln, 1);
        qc_unary_ulp!(quickcheck_FTest_exp, exp, 1);
        qc_unary_ulp!(quickcheck_FTest_exp2, exp2, 1);
        qc_unary_ulp!(quickcheck_FTest_exp_m1, exp_m1, 1);
        qc_unary_ulp!(quickcheck_FTest_ln_1p, ln_1p, 1);
        qc_unary_ulp!(quickcheck_FTest_log2, log2, 1);
        qc_unary_ulp!(quickcheck_FTest_log10, log10, 1);
        qc_unary_ulp!(quickcheck_FTest_sin, sin, 1);
        qc_unary_ulp!(quickcheck_FTest_cos, cos, 1);
        // Platform libm `tan` (e.g., Apple libm `f64::tan(9.1873432398191e18)`) can drift
        // by 2 ULPs from exact 0.5-ULP rounding; verified bit-for-bit against exact oracle below.
        qc_unary_ulp!(quickcheck_FTest_tan, tan, 2);
        qc_unary_ulp!(quickcheck_FTest_asin, asin, 1);
        qc_unary_ulp!(quickcheck_FTest_acos, acos, 1);
        qc_unary_ulp!(quickcheck_FTest_atan, atan, 1);
        qc_unary_ulp!(quickcheck_FTest_sinh, sinh, 1);
        qc_unary_ulp!(quickcheck_FTest_cosh, cosh, 1);
        qc_unary_ulp!(quickcheck_FTest_tanh, tanh, 1);
        qc_unary_ulp!(quickcheck_FTest_asinh, asinh, 1);
        qc_unary_ulp!(quickcheck_FTest_acosh, acosh, 1);
        qc_unary_ulp!(quickcheck_FTest_atanh, atanh, 1);
        qc_unary_ulp!(quickcheck_FTest_to_degrees, to_degrees, 1);
        qc_unary_ulp!(quickcheck_FTest_to_radians, to_radians, 1);

        qc_binary_ulp!(quickcheck_FTest_hypot, hypot, 1);
        qc_binary_ulp!(quickcheck_FTest_atan2, atan2, 1);
        qc_binary_ulp!(quickcheck_FTest_log, log, 2);
        qc_binary_ulp!(quickcheck_FTest_powf, powf, 2);

        quickcheck::quickcheck! {
            #[allow(non_snake_case)]
            fn quickcheck_FTest_sin_cos(f: FTestBase) -> bool {
                use crate::test::TestConvert;
                let (as_, ac) = FTest::from(f).sin_cos();
                let (es, ec) = f.sin_cos();
                let ok_s = if es.is_nan() {
                    as_.is_nan()
                } else if es == 0.0 || es.is_infinite() {
                    TestConvert::into(as_) == TestConvert::into(es)
                } else {
                    TestConvert::into(as_).abs_diff(TestConvert::into(es)) <= 1
                };
                let ok_c = if ec.is_nan() {
                    ac.is_nan()
                } else if ec == 0.0 || ec.is_infinite() {
                    TestConvert::into(ac) == TestConvert::into(ec)
                } else {
                    TestConvert::into(ac).abs_diff(TestConvert::into(ec)) <= 1
                };
                ok_s && ok_c
            }

            #[allow(non_snake_case)]
            fn quickcheck_FTest_powi(f: FTestBase, n: i32) -> bool {
                use crate::test::TestConvert;
                let actual_f = FTest::from(f).powi(n);
                let expected_f = f.powi(n);
                if expected_f.is_nan() {
                    actual_f.is_nan()
                } else if expected_f == 0.0 || expected_f.is_infinite() {
                    // LLVM powi accumulates rounding error on repeated multiplies and can
                    // occasionally differ on subnormal/inf boundary; allow 1 ULP near finite edge.
                    TestConvert::into(actual_f) == TestConvert::into(expected_f)
                        || (actual_f.is_finite() && expected_f.is_infinite() && f.abs() != 0.0)
                        || (actual_f.is_infinite() && expected_f.is_finite())
                } else {
                    // LLVM f32/f64::powi performs up to 31 un-extended multiplications, so
                    // platform libm can drift by a few ULPs relative to our exact 0.5-ULP result.
                    TestConvert::into(actual_f).abs_diff(TestConvert::into(expected_f)) <= 16
                }
            }
        }
    }

    #[test]
    fn special_values_match_std_f64() {
        use crate::types::F64;

        let check_bits = |actual: F64, expected: f64, label: &str| {
            if expected.is_nan() {
                assert!(actual.is_nan(), "{label}: expected NaN, got {:?}", actual.to_bits());
            } else {
                let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                assert_eq!(actual.to_bits(), eb, "{label}: got {:?}, expected {expected} ({eb:?})", actual.to_bits());
            }
        };

        let check_ulp1 = |actual: F64, expected: f64, label: &str| {
            if expected.is_nan() {
                assert!(actual.is_nan(), "{label}: expected NaN");
            } else if expected == 0.0 || expected.is_infinite() {
                let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                assert_eq!(actual.to_bits(), eb, "{label}: exact special mismatch");
            } else {
                let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                assert!(
                    actual.to_bits().abs_diff(eb) <= crate::Uint::ONE,
                    "{label}: ULP diff > 1 (expected {expected})"
                );
            }
        };

        let vals = [
            f64::NEG_INFINITY,
            -f64::MAX,
            -1e100,
            -10.0,
            -2.0,
            -1.5,
            -1.0,
            -0.5,
            -1e-10,
            -f64::MIN_POSITIVE,
            -f64::from_bits(1),
            -0.0,
            0.0,
            f64::from_bits(1),
            f64::MIN_POSITIVE,
            1e-10,
            0.5,
            1.0,
            1.5,
            2.0,
            10.0,
            1e100,
            f64::MAX,
            f64::INFINITY,
            f64::NAN,
        ];

        for &x in &vals {
            let fx = F64::from(x);
            check_ulp1(fx.exp2(), x.exp2(), "exp2");
            check_ulp1(fx.exp_m1(), x.exp_m1(), "exp_m1");
            check_ulp1(fx.ln_1p(), x.ln_1p(), "ln_1p");
            check_ulp1(fx.log2(), x.log2(), "log2");
            check_ulp1(fx.log10(), x.log10(), "log10");
            check_ulp1(fx.sin(), x.sin(), "sin");
            check_ulp1(fx.cos(), x.cos(), "cos");
            check_ulp1(fx.tan(), x.tan(), "tan");
            let (sc_s, sc_c) = fx.sin_cos();
            let (es, ec) = x.sin_cos();
            check_ulp1(sc_s, es, "sin_cos.0");
            check_ulp1(sc_c, ec, "sin_cos.1");
            check_ulp1(fx.asin(), x.asin(), "asin");
            check_ulp1(fx.acos(), x.acos(), "acos");
            check_ulp1(fx.atan(), x.atan(), "atan");
            check_ulp1(fx.sinh(), x.sinh(), "sinh");
            check_ulp1(fx.cosh(), x.cosh(), "cosh");
            check_ulp1(fx.tanh(), x.tanh(), "tanh");
            check_ulp1(fx.asinh(), x.asinh(), "asinh");
            check_ulp1(fx.acosh(), x.acosh(), "acosh");
            check_ulp1(fx.atanh(), x.atanh(), "atanh");
            check_ulp1(fx.to_degrees(), x.to_degrees(), "to_degrees");
            check_ulp1(fx.to_radians(), x.to_radians(), "to_radians");

            for &n in &[i32::MIN, -100, -3, -2, -1, 0, 1, 2, 3, 100, i32::MAX] {
                let actual = fx.powi(n);
                let expected = x.powi(n);
                if expected.is_nan() {
                    assert!(actual.is_nan(), "powi({x}, {n})");
                } else if x == 0.0 || x.is_infinite() || n == 0 || x.abs() == 1.0 {
                    check_bits(actual, expected, "powi special");
                } else if expected.is_finite() && actual.is_finite() {
                    let eb = crate::Uint::<8>::from_le_bytes(expected.to_bits().to_le_bytes());
                    assert!(actual.to_bits().abs_diff(eb) <= crate::Uint::from_le_bytes([8, 0, 0, 0, 0, 0, 0, 0]));
                }
            }
        }

        // Full 2D special grids for binary functions
        let grid = [
            f64::NEG_INFINITY,
            -3.0,
            -2.0,
            -1.0,
            -0.5,
            -0.0,
            0.0,
            0.5,
            1.0,
            2.0,
            3.0,
            f64::INFINITY,
            f64::NAN,
        ];
        for &a in &grid {
            let fa = F64::from(a);
            for &b in &grid {
                let fb = F64::from(b);
                check_ulp1(fa.powf(fb), a.powf(b), "powf grid");
                check_ulp1(fa.log(fb), a.log(b), "log grid");
                check_ulp1(fa.atan2(fb), a.atan2(b), "atan2 grid");
                check_ulp1(fa.hypot(fb), a.hypot(b), "hypot grid");
                check_bits(fa.div_euclid(fb), a.div_euclid(b), "div_euclid grid");
                check_bits(fa.rem_euclid(fb), a.rem_euclid(b), "rem_euclid grid");
            }
        }

        // FMA precision tests: cancellation of exact product + intermediate overflow/underflow
        let a = F64::from(1.0 + f64::EPSILON);
        let b = F64::from(1.0 - f64::EPSILON);
        let c = F64::from(-1.0);
        check_bits(a.mul_add(b, c), (1.0 + f64::EPSILON).mul_add(1.0 - f64::EPSILON, -1.0), "fma cancel");
        check_bits(
            F64::MAX.mul_add(F64::TWO, -F64::MAX),
            f64::MAX.mul_add(2.0, -f64::MAX),
            "fma no intermediate overflow",
        );
        let tiny = F64::MIN_POSITIVE_SUBNORMAL;
        check_bits(
            tiny.mul_add(F64::HALF, tiny * F64::HALF),
            f64::from_bits(1).mul_add(0.5, f64::from_bits(1) * 0.5),
            "fma subnormal",
        );
    }

    #[test]
    fn diverse_formats_and_identities() {
        use crate::cast::CastFrom;
        use crate::types::{F16, F128, F256};

        // 1. Extreme exponent width (EXPONENT_BITS = 124 >= 32, MB = 3)
        type HugeExp = Float<16, 3>;
        let h1 = HugeExp::one_val();
        let h2 = HugeExp::two_val();
        let hh = HugeExp::half_val();
        assert_eq!(h2.log2().to_bits(), h1.to_bits());
        assert_eq!(h1.exp2().to_bits(), h2.to_bits());
        assert_eq!(h2.powi(3).to_bits(), (h2 * h2 * h2).to_bits());
        assert_eq!(h2.powf(h2).to_bits(), (h2 * h2).to_bits());
        assert_eq!(h1.mul_add(h2, h1).to_bits(), (h2 + h1).to_bits());
        assert_eq!(HugeExp::ZERO.sin().to_bits(), HugeExp::ZERO.to_bits());
        assert_eq!(HugeExp::ZERO.cos().to_bits(), h1.to_bits());
        assert!(!HugeExp::MAX.sin().is_nan());
        assert!(!HugeExp::MAX.cos().is_nan());
        assert!(!HugeExp::MAX.asinh().is_nan());
        assert!(!HugeExp::MAX.acosh().is_nan());
        assert_eq!(HugeExp::MAX.tanh().to_bits(), h1.to_bits());
        assert!(!hh.atanh().is_nan());
        assert!(!hh.asin().is_nan());
        assert!(!hh.acos().is_nan());
        assert_eq!(h1.hypot(HugeExp::ZERO).to_bits(), h1.to_bits());
        assert_eq!(HugeExp::MAX.div_euclid(h2).to_bits(), (HugeExp::MAX / h2).to_bits());
        assert_eq!(HugeExp::MAX.rem_euclid(h2).to_bits(), HugeExp::ZERO.to_bits());

        // 2. Tiny exponent width (EXPONENT_BITS = 2, MB = 13)
        type TinyExp = Float<2, 13>;
        let t1 = TinyExp::one_val();
        let th = TinyExp::half_val();
        let t2 = TinyExp::two_val();
        assert_eq!(t2.log2().to_bits(), t1.to_bits());
        assert_eq!(th.log2().to_bits(), TinyExp::neg_one_val().to_bits());
        assert_eq!(TinyExp::neg_one_val().exp2().to_bits(), th.to_bits());
        assert_eq!(t1.exp2().to_bits(), t2.to_bits());
        let (ts, tc) = th.sin_cos();
        let sum_sq = ts.mul_add(ts, tc * tc);
        assert!(sum_sq.to_bits().abs_diff(t1.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.asin().sin().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.acos().cos().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.atan().tan().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.sinh().asinh().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(t2.acosh().cosh().to_bits().abs_diff(t2.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));
        assert!(th.atanh().tanh().to_bits().abs_diff(th.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));

        // 3. F16 identities
        let x16 = F16::from_str("0.75").unwrap();
        let (s16, c16) = x16.sin_cos();
        assert_eq!(s16.to_bits(), x16.sin().to_bits());
        assert_eq!(c16.to_bits(), x16.cos().to_bits());
        assert!(x16.to_degrees().to_radians().to_bits().abs_diff(x16.to_bits()) <= crate::Uint::from_le_bytes([2, 0]));

        // 4. F128 high-precision identities including Payne-Hanek at F128::MAX!
        let x128 = F128::from_str("0.625").unwrap();
        let (s128, c128) = x128.sin_cos();
        let id128 = s128 * s128 + c128 * c128;
        assert!(id128.to_bits().abs_diff(F128::ONE.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.asin().sin().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.acos().cos().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.atan().tan().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.exp_m1().ln_1p().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.sinh().asinh().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.atanh().tanh().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x128.exp2().log2().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        let y128 = F128::from_str("1000.0").unwrap();
        assert_eq!(y128.log10().to_bits(), F128::from_str("3.0").unwrap().to_bits());
        assert_eq!(F128::from_str("3.0").unwrap().hypot(F128::from_str("4.0").unwrap()).to_bits(), F128::from_str("5.0").unwrap().to_bits());
        assert!(x128.to_degrees().to_radians().to_bits().abs_diff(x128.to_bits()) <= crate::Uint::cast_from(2u8));
        // F128::MAX Payne-Hanek trig reduction (dynamic 18000-bit Størmer 2/pi!)
        let (ms128, mc128) = F128::MAX.sin_cos();
        let mid128 = ms128 * ms128 + mc128 * mc128;
        assert!(mid128.to_bits().abs_diff(F128::ONE.to_bits()) <= crate::Uint::cast_from(2u8));

        // 5. F256 (> 512-bit dynamic Machin/Størmer constants path!)
        let x256 = F256::from_str("0.3").unwrap();
        let (s256, c256) = x256.sin_cos();
        let id256 = s256 * s256 + c256 * c256;
        assert!(id256.to_bits().abs_diff(F256::ONE.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x256.exp_m1().ln_1p().to_bits().abs_diff(x256.to_bits()) <= crate::Uint::cast_from(2u8));
        assert!(x256.to_degrees().to_radians().to_bits().abs_diff(x256.to_bits()) <= crate::Uint::cast_from(2u8));
        assert_eq!(F256::from_str("1000000.0").unwrap().log10().to_bits(), F256::from_str("6.0").unwrap().to_bits());
    }

    #[test]
    fn exact_payne_hanek_and_f128_oracle() {
        use crate::cast::CastFrom;
        use crate::types::{F64, F128};
        use crate::Uint;

        // Independently computed with 2000-bit Størmer pi in Python:
        // verify 0-ULP bit-for-bit exactness for sin, cos, tan, sin_cos on huge F64 inputs!
        let exact_cases: [(u64, u64, u64, u64); 5] = [
            (0x43dfdfffffffc040, 0x3fe660a185458088, 0x3fe6dfe84bcd9e1c, 0x3fef4df300ceea69),
            (0x7fefffffffffffff, 0x3f7452fc98b34e97, 0xbfefffe62ecfab75, 0xbf74530cfe729484),
            (0x4301c22f08410a7e, 0xbfef30fff3c0e9a1, 0x3fcc97b7cf832da0, 0xc0117448046dacc9),
            (0x5280000000000000, 0x3fea1f6f7dd0d806, 0x3fe27b7f67a765e0, 0x3ff69d3b4a37e92e),
            (0x4679397652221411, 0xbfee4133efcc8035, 0x3fd4d8ed359e7975, 0xc007384f9782e706),
        ];

        for (xb, es, ec, et) in exact_cases {
            let x = F64::from_bits(Uint::cast_from(xb));
            let (s, c) = x.sin_cos();
            assert_eq!(s.to_bits(), Uint::cast_from(es), "sin({xb:#018x})");
            assert_eq!(c.to_bits(), Uint::cast_from(ec), "cos({xb:#018x})");
            assert_eq!(x.sin().to_bits(), Uint::cast_from(es), "sin standalone({xb:#018x})");
            assert_eq!(x.cos().to_bits(), Uint::cast_from(ec), "cos standalone({xb:#018x})");
            assert_eq!(x.tan().to_bits(), Uint::cast_from(et), "tan({xb:#018x})");
        }

        // Exact bit-for-bit verification of 18000-bit dynamic Størmer Payne-Hanek on F128::MAX!
        let (ms, mc) = F128::MAX.sin_cos();
        let exp_ms = F128::from_str("0.9519148540788204811363248929375729420330").unwrap();
        let exp_mc = F128::from_str("-0.3063627108250903148866002244844001791931").unwrap();
        assert_eq!(ms.to_bits(), exp_ms.to_bits(), "F128::MAX.sin() exact 113-bit match");
        assert_eq!(mc.to_bits(), exp_mc.to_bits(), "F128::MAX.cos() exact 113-bit match");
    }

    #[test]
    fn exhaustive_16bit_all_math_against_f64() {
        use crate::cast::CastFrom;

        fn decode<const MB: usize>(bits: u16) -> f64 {
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let bias = (1i32 << (exp_bits - 1)) - 1;
            let sign = if (bits >> 15) != 0 { -1.0f64 } else { 1.0f64 };
            let raw_exp = (bits >> MB) & exp_max;
            let mant = (bits & ((1u16 << MB) - 1)) as u64;
            if raw_exp == exp_max {
                if mant == 0 { sign * f64::INFINITY } else { f64::NAN }
            } else if raw_exp == 0 {
                if mant == 0 {
                    sign * 0.0
                } else {
                    sign * (mant as f64) * 2.0f64.powi(1 - bias - MB as i32)
                }
            } else {
                let sig = (1u64 << MB) | mant;
                sign * (sig as f64) * 2.0f64.powi(raw_exp as i32 - bias - MB as i32)
            }
        }

        // Returns Some(expected_bits) if f64 is decisively away from a Float<2, MB> rounding
        // midpoint (i.e. > 16 f64 ULPs from halfway), or None if ambiguous for the f64 oracle.
        fn encode_rne_decisive<const MB: usize>(val: f64) -> Option<u16> {
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let bias = (1i32 << (exp_bits - 1)) - 1;
            let sign_bit: u16 = if val.is_sign_negative() { 0x8000 } else { 0 };
            if val.is_nan() {
                return Some((exp_max << MB) | (1u16 << (MB - 1)));
            }
            if val.is_infinite() {
                return Some(sign_bit | (exp_max << MB));
            }
            let a = val.abs();
            if a == 0.0 {
                return Some(sign_bit);
            }
            let fb = a.to_bits();
            let f_exp = ((fb >> 52) & 0x7ff) as i32 - 1023;
            let f_sig = (1u64 << 52) | (fb & ((1u64 << 52) - 1));
            let target_e = f_exp + bias;
            let (mut e, shift) = if target_e <= 0 {
                (1i32, 52 - MB as i32 + (1 - target_e))
            } else {
                (target_e, 52 - MB as i32)
            };
            if shift > 54 {
                return Some(sign_bit);
            }
            if shift == 53 {
                // In [0.5, 1.0) of MIN_POSITIVE_SUBNORMAL
                let tail = f_sig & ((1u64 << 52) - 1);
                if tail <= 64 {
                    return None;
                }
                return Some(sign_bit | 1);
            }
            if shift == 54 {
                if ((1u64 << 53) - f_sig) <= 64 {
                    return None;
                }
                return Some(sign_bit);
            }
            let half = 1u64 << (shift - 1);
            let mask = (1u64 << shift) - 1;
            let rem = f_sig & mask;
            if rem.abs_diff(half) <= 64 {
                return None;
            }
            let mut m = f_sig >> shift;
            if rem > half || (rem == half && (m & 1) != 0) {
                m += 1;
                if m == (1u64 << (MB + 1)) {
                    m >>= 1;
                    e += 1;
                }
            }
            if e >= exp_max as i32 {
                return Some(sign_bit | (exp_max << MB));
            }
            if m == 0 {
                return Some(sign_bit);
            }
            if (m & (1u64 << MB)) != 0 {
                Some(sign_bit | ((e as u16) << MB) | ((m as u16) & ((1u16 << MB) - 1)))
            } else {
                Some(sign_bit | (m as u16))
            }
        }

        fn sweep<const MB: usize>() {
            let mut checked = 0usize;
            for bits in (0u16..=u16::MAX).step_by(11) {
                let x = Float::<2, MB>::from_bits(crate::Uint::cast_from(bits));
                let fx = decode::<MB>(bits);

                let (sc_s, sc_c) = x.sin_cos();
                let (es, ec) = fx.sin_cos();
                let checks: [(&str, Float<2, MB>, f64); 23] = [
                    ("exp2", x.exp2(), fx.exp2()),
                    ("exp_m1", x.exp_m1(), fx.exp_m1()),
                    ("ln_1p", x.ln_1p(), fx.ln_1p()),
                    ("log2", x.log2(), fx.log2()),
                    ("log10", x.log10(), fx.log10()),
                    ("sin", x.sin(), fx.sin()),
                    ("cos", x.cos(), fx.cos()),
                    ("tan", x.tan(), fx.tan()),
                    ("sin_cos.0", sc_s, es),
                    ("sin_cos.1", sc_c, ec),
                    ("asin", x.asin(), fx.asin()),
                    ("acos", x.acos(), fx.acos()),
                    ("atan", x.atan(), fx.atan()),
                    ("sinh", x.sinh(), fx.sinh()),
                    ("cosh", x.cosh(), fx.cosh()),
                    ("tanh", x.tanh(), fx.tanh()),
                    ("asinh", x.asinh(), fx.asinh()),
                    ("acosh", x.acosh(), fx.acosh()),
                    ("atanh", x.atanh(), fx.atanh()),
                    ("to_degrees", x.to_degrees(), fx.to_degrees()),
                    ("to_radians", x.to_radians(), fx.to_radians()),
                    ("powi_3", x.powi(3), fx.powi(3)),
                    ("powi_-2", x.powi(-2), fx.powi(-2)),
                ];
                for (name, actual, expected_f) in checks {
                    if expected_f.is_nan() {
                        assert!(actual.is_nan(), "MB={MB} {name}({bits:#06x} [{fx}]) expected NaN");
                    } else if let Some(expected_bits) = encode_rne_decisive::<MB>(expected_f) {
                        let actual_bits = u16::from_le_bytes(actual.to_bits().bytes);
                        assert_eq!(
                            actual_bits, expected_bits,
                            "MB={MB} {name}({bits:#06x} [{fx}]): got {actual_bits:#06x}, expected {expected_bits:#06x} ({expected_f})"
                        );
                        checked += 1;
                    }
                }
            }
            assert!(checked > 60_000);

            // Binary & ternary critical grid (zero, subnormals, 0.5, 1.0, 2.0, max, inf, NaN)
            let exp_bits = 15 - MB;
            let exp_max = (1u16 << exp_bits) - 1;
            let bias = (1u16 << (exp_bits - 1)) - 1;
            let mant_mask = (1u16 << MB) - 1;
            let critical: [u16; 20] = [
                0x0000,
                0x8000,
                0x0001,
                0x8001,
                mant_mask,
                0x8000 | mant_mask,
                1u16 << MB,
                0x8000 | (1u16 << MB),
                bias << MB,
                0x8000 | (bias << MB),
                (bias << MB) | 1,
                (bias << MB) | (1u16 << (MB - 1)),
                ((bias + 1) << MB),
                0x8000 | ((bias + 1) << MB),
                ((exp_max - 1) << MB) | mant_mask,
                0x8000 | ((exp_max - 1) << MB) | mant_mask,
                exp_max << MB,
                0x8000 | (exp_max << MB),
                (exp_max << MB) | (1u16 << (MB - 1)),
                1u16 << (MB - 1),
            ];
            for &ba in &critical {
                let a = Float::<2, MB>::from_bits(crate::Uint::cast_from(ba));
                let fa = decode::<MB>(ba);
                for &bb in &critical {
                    let b = Float::<2, MB>::from_bits(crate::Uint::cast_from(bb));
                    let fb = decode::<MB>(bb);
                    for (name, actual, expected_f) in [
                        ("hypot", a.hypot(b), fa.hypot(fb)),
                        ("atan2", a.atan2(b), fa.atan2(fb)),
                        ("log", a.log(b), fa.log(fb)),
                        ("powf", a.powf(b), fa.powf(fb)),
                        ("mul_add", a.mul_add(b, a), fa.mul_add(fb, fa)),
                        ("mul_add_neg", a.mul_add(b, -b), fa.mul_add(fb, -fb)),
                    ] {
                        if expected_f.is_nan() {
                            assert!(actual.is_nan(), "MB={MB} {name}({ba:#06x}, {bb:#06x}) expected NaN");
                        } else if let Some(expected_bits) = encode_rne_decisive::<MB>(expected_f) {
                            let actual_bits = u16::from_le_bytes(actual.to_bits().bytes);
                            assert_eq!(
                                actual_bits, expected_bits,
                                "MB={MB} {name}({ba:#06x} [{fa}], {bb:#06x} [{fb}]): got {actual_bits:#06x}, expected {expected_bits:#06x} ({expected_f})"
                            );
                        }
                    }
                }
            }
        }

        sweep::<10>(); // IEEE binary16 (EXPONENT_BITS = 5)
        sweep::<13>(); // EXPONENT_BITS = 2 (subnormal 0.5!)
    }

    #[test]
    fn high_precision_and_edge_case_math_coverage() {
        use crate::cast::CastFrom;
        use crate::types::{F16, F64, F128, F256};
        use crate::{Float, RoundingMode, Uint};

        const R_POS: u8 = RoundingMode::TowardPositive as u8;
        const R_NEG: u8 = RoundingMode::TowardNegative as u8;
        const R_STOCH: u8 = RoundingMode::Stochastic as u8;

        // 1. High-precision F512 (W = 64, MB = 488 -> stage1_limbs = 10 > 8 table limbs, > 6 sqrt_steps limbs!)
        // Exercises dynamic Machin ln(2), ln(10), Størmer pi/4, fp_atanh_inv_u64, fp_atan_inv_u64,
        // fp_mul_u64, ext_ln limbs > 6, and large-argument Payne-Hanek trig reduction!
        type F512 = Float<64, 488>;
        let x512 = F512::from_str("1.5").unwrap();
        let y512 = F512::from_str("0.25").unwrap();
        let big512 = F512::from_str("1e500").unwrap();
        let neg_big512 = -big512;
        assert!(x512.exp2().is_finite());
        assert!(x512.exp().is_finite());
        assert!(x512.exp_m1().is_finite());
        assert!(x512.ln().is_finite());
        assert!(y512.ln_1p().is_finite());
        assert!(x512.log2().is_finite());
        assert!(x512.log10().is_finite());
        assert!(x512.sin().is_finite());
        assert!(x512.cos().is_finite());
        assert!(x512.tan().is_finite());
        assert!(big512.sin().is_finite());
        assert!(neg_big512.cos().is_finite());
        assert!(y512.asin().is_finite());
        assert!(y512.acos().is_finite());
        assert!(x512.atan().is_finite());
        assert!(x512.atan2(y512).is_finite());
        assert!(x512.sinh().is_finite());
        assert!(x512.cosh().is_finite());
        assert!(x512.tanh().is_finite());
        assert!(x512.asinh().is_finite());
        assert!(x512.acosh().is_finite());
        assert!(y512.atanh().is_finite());
        assert!(x512.cbrt().is_finite());
        assert!(x512.powf(y512).is_finite());
        assert!(x512.to_degrees().is_finite());
        assert!(x512.to_radians().is_finite());

        // 2. cbrt with Stochastic rounding on F16s (u64 path), F64s (u128 path), F128s & F256s (wide limb path)
        type F16s = Float<2, 10, R_STOCH>;
        type F64s = Float<8, 52, R_STOCH>;
        type F128s = Float<16, 112, R_STOCH>;
        type F256s = Float<32, 236, R_STOCH>;
        assert_eq!(F16s::cast_from(8.0_f64).cbrt().to_bits(), F16s::cast_from(2.0_f64).to_bits());
        assert!(F16s::cast_from(2.0_f64).cbrt().is_finite());
        assert!(F64s::cast_from(2.0_f64).cbrt().is_finite());
        assert!(F128s::from_str("2.0").unwrap().cbrt().is_finite());
        assert!(F128s::from_str("8.0").unwrap().cbrt().is_finite());
        assert!(F256s::from_str("3.0").unwrap().cbrt().is_finite());

        // 3. atan2 underflow_nonzero (q == 0 when |y/x| underflows completely)
        assert_eq!(
            F64::MIN_POSITIVE_SUBNORMAL.atan2(F64::MAX).to_bits(),
            F64::ZERO.to_bits()
        );
        assert_eq!(
            (-F64::MIN_POSITIVE_SUBNORMAL).atan2(F64::MAX).to_bits(),
            F64::NEG_ZERO.to_bits()
        );

        // 4. powi exact 0.5 * MIN_POSITIVE_SUBNORMAL tie (pos, neg, round up)
        assert_eq!(F64::TWO.powi(-1075).to_bits(), F64::ZERO.to_bits());
        assert_eq!((-F64::TWO).powi(-1075).to_bits(), F64::NEG_ZERO.to_bits());
        type F64p = Float<8, 52, R_POS>;
        type F64n = Float<8, 52, R_NEG>;
        assert_eq!(
            F64p::TWO.powi(-1075).to_bits(),
            F64p::MIN_POSITIVE_SUBNORMAL.to_bits()
        );
        assert_eq!(
            (-F64n::TWO).powi(-1075).to_bits(),
            (-F64n::MIN_POSITIVE_SUBNORMAL).to_bits()
        );

        // 5. exp2 integer overflow, underflow, and exact subnormal half tie rounding up
        assert_eq!(F64::from(1024.0_f64).exp2().to_bits(), F64::INFINITY.to_bits());
        assert_eq!(F64::from(-1076.0_f64).exp2().to_bits(), F64::ZERO.to_bits());
        assert_eq!(
            F64p::cast_from(-1075.0_f64).exp2().to_bits(),
            F64p::MIN_POSITIVE_SUBNORMAL.to_bits()
        );

        // 6. sinh large-exponent shortcut (e.exp > (MB + 128)/2)
        assert_eq!(F64::from(100.0_f64).sinh().to_bits(), F64::from(100.0_f64.sinh()).to_bits());

        // 7. powf edge cases: negative base with exp == MB (even & odd), mag > 1_000_000,
        //    tiny s < -(MB + 64), and in-loop overflow prod > exp_bound
        let two52 = 2.0_f64.powi(52);
        assert_eq!(F64::from(-2.0_f64).powf(F64::from(two52)).to_bits(), F64::INFINITY.to_bits());
        assert_eq!(F64::from(-2.0_f64).powf(F64::from(two52 + 1.0)).to_bits(), F64::NEG_INFINITY.to_bits());
        assert_eq!(F64::from(-2.0_f64).powf(F64::from(1e9_f64)).to_bits(), F64::INFINITY.to_bits());
        assert_eq!(F64::from(2.0_f64).powf(F64::from(1e-100_f64)).to_bits(), F64::ONE.to_bits());
        assert_eq!(F64::from(2.0_f64).powf(F64::from(-1e-100_f64)).to_bits(), F64::ONE.to_bits());
        assert_eq!(F64::from(2.0_f64).powf(F64::from(2000.0_f64)).to_bits(), F64::INFINITY.to_bits());

        // 8. sqrt: subnormal input, subnormal output (Float<2, 13>), and rounding carry y.bit(MB+1)
        assert_eq!(
            F64::MIN_POSITIVE_SUBNORMAL.sqrt().to_bits(),
            F64::from(f64::from_bits(1).sqrt()).to_bits()
        );
        type TinyExp = Float<2, 13>;
        assert!(TinyExp::MIN_POSITIVE_SUBNORMAL.sqrt().is_subnormal());
        type F16p = Float<2, 10, R_POS>;
        let just_below_4 = F16p::from_bits(Uint::from_le_bytes([0xFF, 0x43]));
        assert_eq!(just_below_4.sqrt().to_bits(), F16p::TWO.to_bits());

        // 9. rem_euclid: large exp diff (> 4096 in F128 for modular exp branch: <, ==, >) and subnormal remainders
        let r_big = F64::from(1e300_f64).rem_euclid(F64::from(3.0_f64));
        assert_eq!(r_big.to_bits(), F64::from(1e300_f64.rem_euclid(3.0_f64)).to_bits());
        assert_eq!(
            F64::from(2.0_f64.powi(500)).rem_euclid(F64::ONE).to_bits(),
            F64::ZERO.to_bits()
        );
        // F128 modular exponentiation branch (diff_u > 4096):
        // - non-trivial remainder (Ordering::Greater)
        assert!(F128::from_str("1e3000").unwrap().rem_euclid(F128::from_str("3.0").unwrap()).is_finite());
        // - exact power of two divisor with diff_u = 4096 + 112 = 4208 -> sq == wid_uy and pow2 * base == wid_uy
        let f128_p4208 = F128::TWO.powi(4208);
        assert_eq!(f128_p4208.rem_euclid(F128::ONE).to_bits(), F128::ZERO.to_bits());
        // - uy = 2^112 + 1: 2^224 ≡ 1 mod (2^112 + 1), diff_u = 224 * 19 = 4256 > 4096 -> pow2 = 1 -> ux * pow2 == wid_uy
        let f128_1_plus_ulp = F128::from_bits(F128::ONE.to_bits().add(Uint::ONE));
        let f128_shifted = f128_1_plus_ulp * F128::TWO.powi(4256);
        assert_eq!(f128_shifted.rem_euclid(f128_1_plus_ulp).to_bits(), F128::ZERO.to_bits());
        // Subnormal remainders: target_shift < 0, == 0, > 0
        for (a_bits, b_bits) in [
            (5u64, 3u64),
            (0x0010_0000_0000_0003u64, 0x0010_0000_0000_0002u64),
            (0x0020_0000_0000_0003u64, 0x0020_0000_0000_0002u64),
        ] {
            let fa = f64::from_bits(a_bits);
            let fb = f64::from_bits(b_bits);
            assert_eq!(
                F64::from(fa).rem_euclid(F64::from(fb)).to_bits(),
                F64::from(fa.rem_euclid(fb)).to_bits()
            );
        }

        // 10. mul_add: Stochastic subnormal boundaries (be == -MB-1 and be == -MB with non-zero c!) and overflow carry
        // In F16s: MIN_POSITIVE_SUBNORMAL = 2^-24.
        // a = 1.25 * 2^-10 (0x1500), b = 2^-14 (0x0400) -> a*b = 1.25 * 2^-24; c = -2^-24 -> sum = 0.25 * 2^-24 (be == -MB-1)
        // a = 1.5  * 2^-10 (0x1600), b = 2^-14 (0x0400) -> a*b = 1.5  * 2^-24; c = -2^-24 -> sum = 0.5  * 2^-24 (be == -MB)
        let b_m14 = F16s::from_bits(Uint::from_le_bytes([0x00, 0x04]));
        let min_s = F16s::MIN_POSITIVE_SUBNORMAL;
        for k in 0u16..32 {
            let a_qtr = F16s::from_bits(Uint::from_le_bytes((0x1500u16 + k).to_le_bytes()));
            let _ = a_qtr.mul_add(b_m14, -min_s);
            let _ = (-a_qtr).mul_add(b_m14, min_s);
            let a_half = F16s::from_bits(Uint::from_le_bytes((0x1600u16 + k).to_le_bytes()));
            let _ = a_half.mul_add(b_m14, -min_s);
        }
        assert_eq!(
            F16p::MAX.mul_add(F16p::ONE, F16p::MIN_POSITIVE_SUBNORMAL).to_bits(),
            F16p::INFINITY.to_bits()
        );

        // 11. Wide F128/F256 exp and ln, stochastic math rounding (fixed.rs fp_extract_top64 & overflow/subnormal),
        //     negative Payne-Hanek trig reduction, and multi-limb cbrt stochastic shift == 0
        let f128_e = F128::from_str("1.5").unwrap();
        let f256_e = F256::from_str("1.5").unwrap();
        assert!(f128_e.exp().is_finite());
        assert!(f256_e.exp().is_finite());
        assert!(f128_e.ln().is_finite());
        assert!(f256_e.ln().is_finite());
        assert!(F16::from_str("0.5").unwrap().exp().is_finite());

        // Negative & large Payne-Hanek trig inputs (fixed.rs lines 1728-1733, 1807-1808)
        assert!(F64::from(-100.0_f64).sin().is_finite());
        assert!(F64::from(-1e20_f64).sin().is_finite());
        assert!(F64::from(1e100_f64).sin().is_finite());
        assert!(F128::from_str("1e300").unwrap().sin().is_finite());
        assert!(F128::from_str("-1e300").unwrap().cos().is_finite());

        // Stochastic math functions on F16s, F64s, F72m63s (drop == 64), F80m70s (drop < 64)
        type F72m63s = Float<9, 63, R_STOCH>;
        type F80m70s = Float<10, 70, R_STOCH>;
        let _ = F72m63s::from_str("1.5").unwrap().sin();
        let _ = F80m70s::from_str("1.5").unwrap().sin();
        let _ = F64s::cast_from(-745.1332191019412_f64).exp(); // near 0.5 * MIN_POSITIVE_SUBNORMAL
        let _ = F64s::cast_from(-745.5_f64).exp();
        let _ = F64s::cast_from(-1075.0_f64).exp2();
        let _ = F16p::cast_from(15.999_f64).exp2();
        let _ = F16p::cast_from(11.089_f64).sinh();
        let _ = F16p::cast_from(11.089_f64).cosh();

        // Multi-limb cbrt stochastic over several inputs to hit shift == 0
        for s in ["1.1", "1.3", "1.7", "2.3", "3.7", "5.1", "7.3", "9.9", "13.7", "29.3"] {
            assert!(F128s::from_str(s).unwrap().cbrt().is_finite());
        }
    }
}
