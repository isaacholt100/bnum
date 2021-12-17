use core::ops::{Add, Sub, Mul, Neg, Index, IndexMut, Deref, DerefMut};
use core::mem::MaybeUninit;
use core::iter::{IntoIterator, Sum};
use num_traits::{Zero, One};

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

#[derive(Debug)]
pub struct Vector<T, const N: usize> {
    array: [T; N],
}

impl<T: Zero, const N: usize> Zero for Vector<T, N> {
    fn zero() -> Self {
        let mut array = MaybeUninit::uninit_array();
        for item in array.iter_mut() {
            *item = MaybeUninit::new(T::zero());
        }
        unsafe { MaybeUninit::array_assume_init(array).into() }
    }
    fn is_zero(&self) -> bool {
        self
            .array
            .iter()
            .all(|i| i.is_zero())
    }
}

impl<T: Clone, const N: usize> Clone for Vector<T, N> {
    fn clone(&self) -> Self {
        self.array.clone().into()
    }
}

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
       &self.array
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
       &mut self.array
    }
}

impl<T: Copy, const N: usize> Copy for Vector<T, N> {
    
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    #[inline]
    fn from(array: [T; N]) -> Self {
        Self {
            array,
        }
    }
}

impl<T, const N: usize> From<Vector<T, N>> for [T; N] {
    #[inline]
    fn from(vector: Vector<T, N>) -> Self {
        vector.array
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = core::array::IntoIter<Self::Item, N>;

    #[inline] 
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.array)
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.array[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.array[index]
    }
}

impl<T, U, const N: usize> Add<Vector<U, N>> for Vector<T, N> where T: Add<U> {
    type Output = Vector<<T as Add<U>>::Output, N>;

    fn add(self, rhs: Vector<U, N>) -> Self::Output {
        let mut array = MaybeUninit::uninit_array();
        self
            .into_iter()
            .zip(rhs.into_iter())
            .zip(array.iter_mut())
            .for_each(|((a, b), u)| {
                *u = MaybeUninit::new(a + b);
            });
        unsafe { MaybeUninit::array_assume_init(array).into() }
    }
}

impl<T, U, const N: usize> Sub<Vector<U, N>> for Vector<T, N> where T: Sub<U> {
    type Output = Vector<<T as Sub<U>>::Output, N>;

    fn sub(self, rhs: Vector<U, N>) -> Self::Output {
        let mut array = MaybeUninit::uninit_array();
        self
            .into_iter()
            .zip(rhs.into_iter())
            .zip(array.iter_mut())
            .for_each(|((a, b), u)| {
                *u = MaybeUninit::new(a - b);
            });
        unsafe { MaybeUninit::array_assume_init(array).into() }
    }
}

impl<T, U, const N: usize> Mul<U> for Vector<T, N> where T: Mul<U>, U: Clone {
    type Output = Vector<<T as Mul<U>>::Output, N>;

    fn mul(self, rhs: U) -> Self::Output {
        let mut array = MaybeUninit::uninit_array();
        self
            .into_iter()
            .zip(array.iter_mut())
            .for_each(|(a, u)| {
                *u = MaybeUninit::new(a * rhs.clone());
            });
        unsafe { MaybeUninit::array_assume_init(array).into() }
    }
}

impl<T: Neg, const N: usize> Neg for Vector<T, N> {
    type Output = Vector<<T as Neg>::Output, N>;

    fn neg(self) -> Self::Output {
        let mut array = MaybeUninit::uninit_array();
        self
            .into_iter()
            .zip(array.iter_mut())
            .for_each(|(a, u)| {
                *u = MaybeUninit::new(-a);
            });
        unsafe { MaybeUninit::array_assume_init(array).into() }
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn dot<U>(self, rhs: Vector<U, N>) -> <<T as Mul<U>>::Output as Add>::Output where T: Mul<U>, <T as Mul<U>>::Output: Add, <<T as Mul<U>>::Output as Add>::Output: Sum<<T as Mul<U>>::Output> {
        self
            .into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a * b)
            .sum()
    }
    
    pub fn sum_squares(self) -> <<T as Mul>::Output as Add>::Output where T: Mul + Clone, <T as Mul>::Output: Add, <<T as Mul>::Output as Add>::Output: Sum<<T as Mul>::Output> {
        self.clone().dot(self)
    }

    pub fn length(self) -> <<<T as Mul>::Output as Add>::Output as Sqrt>::Output where T: Mul + Clone, <T as Mul>::Output: Add, <<T as Mul>::Output as Add>::Output: Sqrt, <<T as Mul>::Output as Add>::Output: Sum<<T as Mul>::Output> {
        self.sum_squares().sqrt()
    }

    pub fn unit(n: usize) -> Self where T: Zero + One {
        let mut out = Self::zero();
        out[n] = T::one();
        out
    }
}

impl<T> Vector<T, 3> {
    pub fn cross<U>(self, rhs: Vector<U, 3>) -> Vector<<<T as Mul<U>>::Output as Sub>::Output, 3> where T: Mul<U> + Clone, <T as Mul<U>>::Output: Sub, U: Clone {
        let [a1, a2, a3] = self.array;
        let [b1, b2, b3] = rhs.array;

        let x = a2.clone() * b3.clone() - a3.clone() * b2.clone();
        let y = a3 * b1.clone() - a1.clone() * b3;
        let z = a1 * b2 - a2 * b1;

        [x, y, z].into()
    }
}