#![allow(dead_code)]

use std::ops::{AddAssign, Mul};

////////////////////////////////////////////////////////////////////////////////
/// Matrix
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    arr: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Default + Copy,
{
    pub const fn new(arr: [[T; N]; M]) -> Self {
        Self { arr }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.arr.get(row)?.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.arr.get_mut(row)?.get_mut(col)
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.arr.get_unchecked(row).get_unchecked(col)
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.arr.get_unchecked_mut(row).get_unchecked_mut(col)
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut new_arr = [[T::default(); M]; N];
        for i in 0..M {
            for j in 0..N {
                new_arr[j][i] = self.arr[i][j];
            }
        }
        Matrix { arr: new_arr }
    }
}

impl<T, const M: usize, const N: usize> From<Vec<Vec<T>>> for Matrix<T, M, N>
where
    T: Default + Copy,
{
    fn from(value: Vec<Vec<T>>) -> Self {
        let arr = [[Default::default(); N]; M];
        Matrix {
            arr: fill(value, arr),
        }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Default + Copy + Mul + AddAssign<<T as Mul>::Output>,
{
    pub fn mult_slow<const P: usize>(&self, other: &Matrix<T, N, P>) -> Matrix<T, M, P> {
        let mut arr: [[T; P]; M] = [[T::default(); P]; M];
        for i in 0..M {
            for j in 0..P {
                let mut sum = T::default();
                for k in 0..N {
                    sum += self.arr[i][k] * other.arr[k][j];
                }
                arr[i][j] = sum;
            }
        }

        Matrix { arr }
    }

    pub fn mult_transpose<const P: usize>(&self, other: &Matrix<T, N, P>) -> Matrix<T, M, P> {
        let mut arr: [[T; P]; M] = [[T::default(); P]; M];
        let transposed = other.transpose();
        for i in 0..M {
            for j in 0..P {
                let mut sum = T::default();
                for k in 0..N {
                    sum += self.arr[i][k] * transposed.arr[j][k];
                }
                arr[i][j] = sum;
            }
        }

        Matrix { arr }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// MatrixVec
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct MatrixVec<T> {
    vec: Vec<Vec<T>>,
    col_len: usize,
    row_len: usize,
}

impl<T> MatrixVec<T>
where
    T: Default + Copy,
{
    pub fn new(vec: Vec<Vec<T>>) -> Self {
        Self {
            col_len: vec.len(),
            row_len: vec[0].len(),
            vec,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.vec.get(row)?.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.vec.get_mut(row)?.get_mut(col)
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.vec.get_unchecked(row).get_unchecked(col)
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.vec.get_unchecked_mut(row).get_unchecked_mut(col)
    }

    pub fn transpose(&self) -> MatrixVec<T> {
        let mut new_vec = vec![vec![T::default(); self.col_len]; self.row_len];
        for i in 0..self.col_len {
            for j in 0..self.row_len {
                unsafe {
                    *new_vec.get_unchecked_mut(j).get_unchecked_mut(i) =
                        *self.vec.get_unchecked(i).get_unchecked(j);
                }
            }
        }
        MatrixVec {
            vec: new_vec,
            col_len: self.row_len,
            row_len: self.col_len,
        }
    }
}

impl<T> MatrixVec<T>
where
    T: Default + Copy + Mul + AddAssign<<T as Mul>::Output>,
{
    pub fn mult_slow(&self, other: &MatrixVec<T>) -> MatrixVec<T> {
        let mut vec: Vec<Vec<T>> = vec![vec![T::default(); other.row_len]; self.col_len];
        for i in 0..self.col_len {
            for j in 0..other.row_len {
                let mut sum = T::default();
                for k in 0..self.row_len {
                    unsafe {
                        sum += *self.vec.get_unchecked(i).get_unchecked(k)
                            * *other.vec.get_unchecked(k).get_unchecked(j);
                    }
                }
                unsafe {
                    *vec.get_unchecked_mut(i).get_unchecked_mut(j) = sum;
                }
            }
        }

        MatrixVec {
            vec,
            col_len: self.col_len,
            row_len: other.row_len,
        }
    }

    pub fn mult_transpose(&self, other: &MatrixVec<T>) -> MatrixVec<T> {
        let mut vec: Vec<Vec<T>> = vec![vec![T::default(); other.row_len]; self.col_len];
        let transposed = other.transpose();
        for i in 0..self.col_len {
            for j in 0..other.row_len {
                let mut sum = T::default();
                for k in 0..self.row_len {
                    unsafe {
                        sum += *self.vec.get_unchecked(i).get_unchecked(k)
                            * *transposed.vec.get_unchecked(j).get_unchecked(k);
                    }
                }
                unsafe {
                    *vec.get_unchecked_mut(i).get_unchecked_mut(j) = sum;
                }
            }
        }

        MatrixVec {
            vec,
            col_len: self.col_len,
            row_len: other.row_len,
        }
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for MatrixVec<T> {
    fn from(value: [[T; N]; M]) -> Self {
        let vec: Vec<Vec<T>> = value.into_iter().map(|i| i.into_iter().collect()).collect();
        Self {
            col_len: vec.len(),
            row_len: vec[0].len(),
            vec,
        }
    }
}

fn fill<T: Copy, const M: usize, const N: usize>(
    vec: Vec<Vec<T>>,
    mut arr: [[T; N]; M],
) -> [[T; N]; M] {
    for (i, v) in vec.into_iter().enumerate() {
        for (j, f) in v.into_iter().enumerate() {
            unsafe {
                *arr.get_unchecked_mut(i).get_unchecked_mut(j) = f;
            }
        }
    }
    arr
}
