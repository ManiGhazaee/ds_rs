#![allow(dead_code)]

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

use rayon::prelude::*;

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
        assert!(N > 0);
        assert!(M > 0);
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

    #[inline]
    fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        unsafe { self.arr.get_unchecked_mut(row).get_unchecked_mut(col) }
    }
    #[inline]
    fn at(&self, row: usize, col: usize) -> &T {
        unsafe { self.arr.get_unchecked(row).get_unchecked(col) }
    }
}

impl<T, const M: usize, const N: usize> From<&Vec<Vec<T>>> for Matrix<T, M, N>
where
    T: Default + Copy,
{
    fn from(value: &Vec<Vec<T>>) -> Self {
        assert!(M > 0);
        assert!(N > 0);
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
                    sum += *self.at(i, k) * *transposed.at(j, k);
                }
                unsafe {
                    *arr.get_unchecked_mut(i).get_unchecked_mut(j) = sum;
                }
            }
        }

        Matrix { arr }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Add<Output = T> + Default,
{
    pub fn add(&self, other: &Self) -> Self {
        let mut arr = [[T::default(); N]; M];
        self.arr
            .iter()
            .zip(other.arr.iter())
            .enumerate()
            .for_each(|(i, rows)| {
                rows.0
                    .iter()
                    .zip(rows.1.iter())
                    .enumerate()
                    .for_each(|(j, t)| unsafe {
                        *arr.get_unchecked_mut(i).get_unchecked_mut(j) = *t.0 + *t.1
                    })
            });

        Self { arr }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Add<Output = T> + Default + Send + Sync + Debug,
{
    pub fn add_par(&self, other: &Self) -> Self {
        // let mut arr = [[T::default(); N]; M];

        let arr = self
            .arr
            .par_iter()
            .zip(other.arr.par_iter())
            .map(|rows| {
                rows.0
                    .par_iter()
                    .zip(rows.1.par_iter())
                    .map(|t| *t.0 + *t.1)
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[T; N]>>()
            .try_into()
            .unwrap();

        Self { arr }
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Default + Copy + Mul + AddAssign<<T as Mul>::Output> + Sync + Send,
{
    pub fn transpose_par(&self) -> Matrix<T, N, M> {
        let mut new_arr = [[T::default(); M]; N];

        new_arr.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..M {
                unsafe {
                    *row.get_unchecked_mut(j) = *self.at(j, i);
                }
            }
        });

        Matrix { arr: new_arr }
    }

    pub fn mult_par_transpose<const P: usize>(&self, other: &Matrix<T, N, P>) -> Matrix<T, M, P> {
        let mut arr = [[T::default(); P]; M];
        let transposed = other.transpose_par();

        arr.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..P {
                let mut sum = T::default();
                for k in 0..M {
                    sum += *self.at(i, k) * *transposed.at(j, k);
                }
                unsafe {
                    *row.get_unchecked_mut(j) = sum;
                }
            }
        });

        Matrix { arr }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// MatrixVec
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq)]
pub struct MatrixVec<T> {
    vec: Vec<T>,
    col_len: usize,
    row_len: usize,
}

impl<T> MatrixVec<T>
where
    T: Default + Copy,
{
    pub fn new(vec: Vec<Vec<T>>) -> Self {
        let m = vec.len();
        let n = vec[0].len();
        assert!(m > 0);
        assert!(n > 0);
        Self {
            col_len: m,
            row_len: n,
            vec: vec.into_iter().flatten().collect(),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.vec.get(row * self.col_len + col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.vec.get_mut(row * self.col_len + col)
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.vec.get_unchecked(row * self.col_len + col)
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.vec.get_unchecked_mut(row * self.col_len + col)
    }

    pub fn transpose(&self) -> MatrixVec<T> {
        let mut new_matrix = MatrixVec {
            vec: vec![T::default(); self.col_len * self.row_len],
            col_len: self.row_len,
            row_len: self.col_len,
        };

        for i in 0..self.col_len {
            for j in 0..self.row_len {
                *new_matrix.at_mut(j, i) = *self.at(i, j);
            }
        }

        new_matrix
    }

    pub fn transpose_in_place(&mut self) {
        for i in 0..self.row_len {
            for j in (i + 1)..self.col_len {
                unsafe {
                    let temp = std::ptr::read(&*self.at(i, j));
                    std::ptr::write(&mut *self.at_mut(i, j), *self.at(j, i));
                    std::ptr::write(&mut *self.at_mut(j, i), temp);
                }
            }
        }
    }

    #[inline]
    fn at(&self, i: usize, j: usize) -> &T {
        unsafe {
            return self.vec.get_unchecked(i * self.col_len + j);
        }
    }
    #[inline]
    fn at_mut(&mut self, i: usize, j: usize) -> &mut T {
        unsafe {
            return self.vec.get_unchecked_mut(i * self.col_len + j);
        }
    }
}

impl<T> MatrixVec<T>
where
    T: Default + Copy + Mul + AddAssign<<T as Mul>::Output>,
{
    pub fn mult_slow(&self, other: &MatrixVec<T>) -> MatrixVec<T> {
        let mut matrix = MatrixVec {
            vec: vec![T::default(); other.row_len * self.col_len],
            col_len: self.col_len,
            row_len: other.row_len,
        };

        for i in 0..self.col_len {
            for j in 0..other.row_len {
                let mut sum = T::default();
                for k in 0..self.row_len {
                    sum += *self.at(i, k) * *other.at(k, j);
                }
                *matrix.at_mut(i, j) = sum;
            }
        }

        matrix
    }

    pub fn mult_transpose(&self, other: &MatrixVec<T>) -> MatrixVec<T> {
        let mut matrix = MatrixVec {
            vec: vec![T::default(); other.row_len * self.col_len],
            col_len: self.col_len,
            row_len: other.row_len,
        };
        let transposed = other.transpose();
        for i in 0..self.col_len {
            for j in 0..other.row_len {
                let mut sum = T::default();
                for k in 0..self.row_len {
                    sum += *self.at(i, k) * *transposed.at(j, k);
                }
                *matrix.at_mut(i, j) = sum;
            }
        }

        matrix
    }
}

impl<T> MatrixVec<T>
where
    T: Default + Copy + Mul + AddAssign<<T as Mul>::Output> + Sync + Send,
{
    pub fn transpose_par(&self) -> MatrixVec<T> {
        let mut new_vec = vec![T::default(); self.col_len * self.row_len];

        if self.row_len != 0 {
            new_vec
                .par_chunks_exact_mut(self.row_len)
                .enumerate()
                .for_each(|(i, row)| {
                    for j in 0..self.col_len {
                        unsafe {
                            *row.get_unchecked_mut(j) = *self.at(j, i);
                        }
                    }
                });
        }

        MatrixVec {
            vec: new_vec,
            row_len: self.col_len,
            col_len: self.row_len,
        }
    }

    pub fn mult_par_transpose(&self, other: &MatrixVec<T>) -> MatrixVec<T> {
        let mut vec = vec![T::default(); other.row_len * self.col_len];
        let transposed = other.transpose_par();

        if transposed.row_len != 0 {
            vec.par_chunks_exact_mut(transposed.row_len)
                .enumerate()
                .for_each(|(i, row)| {
                    for j in 0..other.col_len {
                        let mut sum = T::default();
                        for k in 0..self.col_len {
                            sum += *self.at(i, k) * *transposed.at(j, k);
                        }
                        unsafe {
                            *row.get_unchecked_mut(j) = sum;
                        }
                    }
                });
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
        assert!(M > 0);
        assert!(N > 0);
        let vec: Vec<T> = value.into_iter().flat_map(|i| i.into_iter()).collect();
        Self {
            col_len: M,
            row_len: N,
            vec,
        }
    }
}

fn fill<T: Copy, const M: usize, const N: usize>(
    vec: &[Vec<T>],
    mut arr: [[T; N]; M],
) -> [[T; N]; M] {
    assert!(M > 0);
    assert!(N > 0);
    for (i, v) in vec.into_iter().enumerate() {
        for (j, f) in v.into_iter().enumerate() {
            unsafe {
                *arr.get_unchecked_mut(i).get_unchecked_mut(j) = *f;
            }
        }
    }
    arr
}

// unsafe fn set_2darray_with_ptr<T: Copy>(
//     arr_ptr: *mut T,
//     arr_len: usize,
//     i: usize,
//     j: usize,
//     new_val: T,
// ) {
//     let offset = i * arr_len + j;
//     let target_ptr = arr_ptr.offset(offset as isize);
//     *target_ptr = new_val;
// }
