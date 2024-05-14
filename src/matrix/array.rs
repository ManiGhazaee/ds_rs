use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

use rayon::prelude::*;

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
