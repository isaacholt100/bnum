/// Used for generating random big integers in a given range.
///
/// Implements the [`UniformSampler`](rand::distr::uniform::UniformSampler) trait from the [`rand`] crate. This struct should not be used directly; instead use the [`Uniform`](rand::distr::Uniform) struct from the [`rand`] crate.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniformInt<X> {
    pub(crate) low: X,
    pub(crate) range: X,
    pub(crate) thresh: X,
}