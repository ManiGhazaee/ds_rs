use std::ops::{AddAssign, Mul};
use rayon::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    vec: Vec<T>,
    col_len: usize,
    row_len: usize,
}

impl<T> Matrix<T>
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

    pub fn transpose(&self) -> Matrix<T> {
        let mut new_matrix = Matrix {
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

impl<T> Matrix<T>
where
    T: Default + Copy + Mul + AddAssign<<T as Mul>::Output>,
{
    pub fn mult_slow(&self, other: &Matrix<T>) -> Matrix<T> {
        let mut matrix = Matrix {
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

    pub fn mult_transpose(&self, other: &Matrix<T>) -> Matrix<T> {
        let mut matrix = Matrix {
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

impl<T> Matrix<T>
where
    T: Default + Copy + Mul + AddAssign<<T as Mul>::Output> + Sync + Send,
{
    pub fn transpose_par(&self) -> Matrix<T> {
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

        Matrix {
            vec: new_vec,
            row_len: self.col_len,
            col_len: self.row_len,
        }
    }

    pub fn mult_par_transpose(&self, other: &Matrix<T>) -> Matrix<T> {
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

        Matrix {
            vec,
            col_len: self.col_len,
            row_len: other.row_len,
        }
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T> {
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