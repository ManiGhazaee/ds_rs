#![allow(dead_code)]

////////////////////////////////////////////////////////////////////////////////
/// Matrix
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    arr: [[T; N]; M],
}

impl<T: Default + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn new(arr: [[T; N]; M]) -> Self {
        Self { arr }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.arr.get(row)?.get(col)
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

impl<T: Default + Copy, const M: usize, const N: usize> From<Vec<Vec<T>>> for Matrix<T, M, N> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let arr = [[Default::default(); N]; M];
        Matrix {
            arr: fill(value, arr),
        }
    }
}
macro_rules! matrix_mult_int_impl {
    ($($t:ty)+) => ($(
        impl<const M: usize, const N: usize> Matrix<$t, M, N> {
            pub const fn mult_slow<const O: usize, const P: usize>(
                &self,
                other: &Matrix<$t, O, P>,
            ) -> Matrix<$t, M, P> {
                if O != N { panic!() }
                let mut arr: [[$t; P]; M] = [[0; P]; M];
                let mut i = 0;
                while i < M {
                    let mut j = 0;
                    while j < P {
                        let mut k = 0;
                        let mut sum = 0;
                        while k < N {
                            sum += self.arr[i][k] * other.arr[k][j];
                            k += 1;
                        }
                        arr[i][j] = sum;
                        j += 1;
                    }
                    i += 1;
                }

                Matrix { arr }
            }

            pub fn mult_transpose<const P: usize>(&self, other: &Matrix<$t, N, P>) -> Matrix<$t, M, P> {
                let mut arr: [[$t; P]; M] = [[0; P]; M];
                let transposed = other.transpose();
                for i in 0..M {
                    for j in 0..P {
                        let mut sum = 0;
                        for k in 0..N {
                            sum += self.arr[i][k] * transposed.arr[j][k];
                        }
                        arr[i][j] = sum;
                    }
                }

                Matrix { arr }
            }
        }
    )+)
}

macro_rules! matrix_mult_float_impl {
    ($($t:ty)+) => ($(
        impl<const M: usize, const N: usize> Matrix<$t, M, N> {
            pub fn mult_slow<const P: usize>(
                &self,
                other: &Matrix<$t, N, P>,
            ) -> Matrix<$t, M, P> {
                let mut arr: [[$t; P]; M] = [[0.0; P]; M];
                for i in 0..M {
                    for j in 0..P {
                        let mut sum = 0.0;
                        for k in 0..N {
                            sum += self.arr[i][k] * other.arr[k][j];
                        }
                        arr[i][j] = sum;
                    }
                }

                Matrix { arr }
            }

            pub fn mult_transpose<const P: usize>(&self, other: &Matrix<$t, N, P>) -> Matrix<$t, M, P> {
                let mut arr: [[$t; P]; M] = [[0.0; P]; M];
                let transposed = other.transpose();
                for i in 0..M {
                    for j in 0..P {
                        let mut sum = 0.0;
                        for k in 0..N {
                            sum += self.arr[i][k] * transposed.arr[j][k];
                        }
                        arr[i][j] = sum;
                    }
                }

                Matrix { arr }
            }
        }
    )+)
}

matrix_mult_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
matrix_mult_float_impl! { f32 f64 }

////////////////////////////////////////////////////////////////////////////////
/// MatrixVec
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct MatrixVec<T> {
    vec: Vec<Vec<T>>,
    col_len: usize,
    row_len: usize,
}

impl<T: Default + Copy> MatrixVec<T> {
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
macro_rules! matrix_vec_mult_int_impl {
    ($($t:ty)+) => ($(
        impl MatrixVec<$t> {
            pub fn mult_slow(&self, other: &MatrixVec<$t>) -> MatrixVec<$t> {
                let mut vec: Vec<Vec<$t>> = vec![vec![0; other.row_len]; self.col_len];
                for i in 0..self.col_len {
                    for j in 0..other.row_len {
                        let mut sum = 0;
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

            pub fn mult_transpose(&self, other: &MatrixVec<$t>) -> MatrixVec<$t> {
                let mut vec: Vec<Vec<$t>> = vec![vec![0; other.row_len]; self.col_len];
                let transposed = other.transpose();
                for i in 0..self.col_len {
                    for j in 0..other.row_len {
                        let mut sum = 0;
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
    )+)
}

macro_rules! matrix_vec_mult_float_impl {
    ($($t:ty)+) => ($(
        impl MatrixVec<$t> {
            pub fn mult_slow(&self, other: &MatrixVec<$t>) -> MatrixVec<$t> {
                let mut vec: Vec<Vec<$t>> = vec![vec![0.0; other.row_len]; self.col_len];
                for i in 0..self.col_len {
                    for j in 0..other.row_len {
                        let mut sum = 0.0;
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

            pub fn mult_transpose(&self, other: &MatrixVec<$t>) -> MatrixVec<$t> {
                let mut vec: Vec<Vec<$t>> = vec![vec![0.0; other.row_len]; self.col_len];
                let transposed = other.transpose();
                for i in 0..self.col_len {
                    for j in 0..other.row_len {
                        let mut sum = 0.0;
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
    )+)
}

matrix_vec_mult_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
matrix_vec_mult_float_impl! { f32 f64 }

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
