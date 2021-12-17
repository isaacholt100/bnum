use crate::vector::Vector;
use core::ops::{Add, Sub, Mul, MulAssign, AddAssign, Neg, Index, IndexMut, Deref, DerefMut};
use num_traits::{Zero, One, Inv};
use core::mem::MaybeUninit;
use core::iter::{IntoIterator, Sum};

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const M: usize, const N: usize> {
    array: Vector<Vector<T, M>, N>,
}

impl<T, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
    type Output = Vector<T, M>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        &self.array[i][j]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Vector<T, M> {
        &mut self.array[index]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        &mut self.array[i][j]
    }
}

impl<T, const M: usize, const N: usize> Deref for Matrix<T, M, N> {
    type Target = [Vector<T, M>];

    fn deref(&self) -> &Self::Target {
       &self.array
    }
}

impl<T, const M: usize, const N: usize> DerefMut for Matrix<T, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
       &mut self.array
    }
}

impl<T: Neg, const M: usize, const N: usize> Neg for Matrix<T, M, N> {
    type Output = Matrix<<T as Neg>::Output, M, N>;

    fn neg(self) -> Self::Output {
        (-self.array).into()
    }
}

impl<T, const M: usize, const N: usize> IntoIterator for Matrix<T, M, N> {
    type Item = Vector<T, M>;
    type IntoIter = core::array::IntoIter<Self::Item, N>;

    #[inline] 
    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> where T: Zero + One + PartialEq {
    pub fn is_rref(&self) -> bool {
        let mut current_leading_one_index = 0;
        let mut leading_ones = [None; M];
        let mut non_zero_rows = [false; M];
        for (i, col) in self.iter().enumerate() {
            let mut leading_one = false;
            let mut zeros = true;
            for (j, item) in col.iter().enumerate() {
                if item.is_one() && !leading_one {
                    if !zeros {
                        return false;
                    }
                    if (i + 1) <= current_leading_one_index {
                        return false;
                    }
                    non_zero_rows[j] = true;
                    leading_one = true;
                    leading_ones[j] = Some(i);
                    current_leading_one_index = i; 
                } else if !item.is_zero() {
                    if leading_one {
                        return false;
                    }
                    if leading_ones[j].is_none() {
                        return false;
                    }
                    non_zero_rows[j] = true;
                    zeros = false;
                }
            }
        }
        let mut changed = false;
        for r in IntoIterator::into_iter(non_zero_rows) {
            if r == changed {
                if changed {
                    return false;
                }
                changed = true;
            }
        }
        true
    }
    fn swap_rows(&mut self, r: usize, s: usize) {
        for col in self.iter_mut() {
            col.swap(r, s);
        }
    }
    fn mul_row(&mut self, r: usize, l: T) where T: MulAssign + Clone {
        for col in self.iter_mut() {
            col[r] *= l.clone();
        }
    }
    fn mul_add_row(&mut self, r: usize, s: usize, l: T) where T: Mul + AddAssign + Clone {
        for col in self.iter_mut() {
            unsafe {
                let r_ref = &mut *(col.get_unchecked_mut(r) as *mut T);
                let s_ref = &mut *(col.get_unchecked_mut(s) as *mut T);
                *s_ref += r_ref.clone() * l.clone();
            }
        }
    }
    fn to_rref_row_k(m: &mut core::cell::RefCell<Self>, k: usize) where T: Inv<Output = T> + MulAssign + Clone + Neg<Output = T> + AddAssign + Mul {
        for col in m.borrow_mut().iter_mut() {
            if !col.is_zero() {
                let mut first_non_zero = None;
                for (j, item) in col.iter_mut().skip(k).enumerate() {
                    if !item.is_zero() {
                        if first_non_zero.is_none() {
                            first_non_zero = Some(j);
                            m.borrow_mut().swap_rows(k, j + k);
                            m.borrow_mut().mul_row(k, item.clone().inv());
                        } else {
                            m.borrow_mut().mul_add_row(k, j + k, -item.clone());
                        }
                    }
                }
                for (j, item) in col.iter_mut().skip(k).take(first_non_zero.unwrap()).enumerate() {
                    m.borrow_mut().mul_add_row(k, j + k, -item.clone());
                }
                break;
            }
        }
        for r in 0..k {
            //m.borrow_mut().mul_add()
        }
    }
    pub fn to_rref(self) -> Self where T: Inv<Output = T> + MulAssign + Clone + Neg<Output = T> + AddAssign + Mul {
        let m = core::cell::RefCell::new(self);
        for col in m.borrow_mut().iter_mut() {
            if !col.is_zero() {
                let mut first_non_zero = None;
                for (j, item) in col.iter_mut().enumerate() {
                    if !item.is_zero() {
                        if first_non_zero.is_none() {
                            first_non_zero = Some(j);
                            m.borrow_mut().swap_rows(1, j);
                            m.borrow_mut().mul_row(1, item.clone().inv());
                        } else {
                            m.borrow_mut().mul_add_row(1, j, -item.clone());
                        }
                    }
                }
                for (j, item) in col.iter_mut().take(first_non_zero.unwrap()).enumerate() {
                    m.borrow_mut().mul_add_row(1, j, -item.clone());
                }
                break;
            }
        }
        m.into_inner()
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn identity() -> Self where T: Zero + One {
        assert_eq!(M, N, "identity matrix must have same number of rows as columns");
        let mut arr = MaybeUninit::uninit_array();
        arr.iter_mut().enumerate().for_each(|(i, item)| {
            *item = MaybeUninit::new(Vector::unit(i));
        });
        let arr = unsafe { MaybeUninit::array_assume_init(arr) };
        arr.into()
    }

    pub fn row(&self, i: usize) -> Vector<T, N> where T: Clone {
        let mut arr = MaybeUninit::uninit_array();
        arr
            .iter_mut()
            .zip(self.array.iter())
            .for_each(|(item, col)| {
                *item = MaybeUninit::new(col[i].clone());
            });
        let arr = unsafe { MaybeUninit::array_assume_init(arr) };
        arr.into()
    }

    pub fn col(&self, i: usize) -> Vector<T, M> where T: Clone {
        self[i].clone()
    }

    pub fn tranpose(self) -> Matrix<T, N, M> {
        let mut arr = MaybeUninit::uninit_array::<M>();
        for item in arr.iter_mut() {
            *item = MaybeUninit::new(MaybeUninit::uninit_array());
        }
        let mut arr = unsafe { MaybeUninit::array_assume_init(arr) };
        self
            .into_iter()
            .enumerate()
            .for_each(|(i, col)| {
                col
                    .into_iter()
                    .enumerate()
                    .for_each(|(j, x)| {
                        arr[j][i] = MaybeUninit::new(x);
                    });
            });
        let mut out = MaybeUninit::uninit_array();
        for (item, a) in out.iter_mut().zip(arr) {
            *item = unsafe {
                MaybeUninit::new(Vector::from(MaybeUninit::array_assume_init(a)))
            };
        }
        unsafe { MaybeUninit::array_assume_init(out).into() }
    }
}

impl<T, const M: usize, const N: usize> From<Vector<Vector<T, M>, N>> for Matrix<T, M, N> {
    fn from(v: Vector<Vector<T, M>, N>) -> Self {
        Self {
            array: v,
        }
    }
}

impl<T, const M: usize, const N: usize> From<[Vector<T, M>; N]> for Matrix<T, M, N> {
    fn from(arr: [Vector<T, M>; N]) -> Self {
        Vector::from(arr).into()
    }
}

impl<T: Zero, const M: usize, const N: usize> Zero for Matrix<T, M, N> {
    fn zero() -> Self {
        Vector::zero().into()
    }
    fn is_zero(&self) -> bool {
        self
            .array
            .iter()
            .all(|i| i.is_zero())
    }
}

impl<T, U, const M: usize, const N: usize> Add<Matrix<U, M, N>> for Matrix<T, M, N> where T: Add<U> {
    type Output = Matrix<<T as Add<U>>::Output, M, N>;

    fn add(self, rhs: Matrix<U, M, N>) -> Self::Output {
        (self.array + rhs.array).into()
    }
}

impl<T, U, const M: usize, const N: usize> Sub<Matrix<U, M, N>> for Matrix<T, M, N> where T: Sub<U> {
    type Output = Matrix<<T as Sub<U>>::Output, M, N>;

    fn sub(self, rhs: Matrix<U, M, N>) -> Self::Output {
        (self.array - rhs.array).into()
    }
}

/*impl<T, U, const M: usize, const N: usize> Mul<U> for Matrix<T, M, N> where T: Mul<U>, U: Clone {
    type Output = Matrix<<T as Mul<U>>::Output, M, N>;

    fn mul(self, rhs: U) -> Self::Output {
        (self.array * rhs).into()
    }
}*/

impl<T, U, const M: usize, const N: usize, const P: usize> Mul<Matrix<U, N, P>> for Matrix<T, M, N> where T: Mul<U> + Clone, <T as Mul<U>>::Output: Add, U: Clone, <<T as Mul<U>>::Output as Add>::Output: Sum<<T as Mul<U>>::Output>, <<T as Mul<U>>::Output as Add>::Output: core::fmt::Debug {
    type Output = Matrix<<<T as Mul<U>>::Output as Add>::Output, M, P>;

    fn mul(self, rhs: Matrix<U, N, P>) -> Self::Output {
        let mut arr = MaybeUninit::uninit_array::<P>();
        for item in arr.iter_mut() {
            *item = MaybeUninit::new(MaybeUninit::uninit_array());
        }
        let mut arr = unsafe { MaybeUninit::array_assume_init(arr) };
        arr
            .iter_mut()
            .enumerate()
            .for_each(|(i, col)| {
                col
                    .iter_mut()
                    .enumerate()
                    .for_each(|(j, item)| {
                        *item = MaybeUninit::new(self.row(j).dot(rhs.col(i)));
                    })
            });

        let mut out = MaybeUninit::uninit_array();
        for (item, a) in out.iter_mut().zip(arr) {
            *item = unsafe {
                MaybeUninit::new(Vector::from(MaybeUninit::array_assume_init(a)))
            };
        }
        unsafe { MaybeUninit::array_assume_init(out).into() }
    }
}