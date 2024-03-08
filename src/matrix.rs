#![allow(dead_code)]

#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    arr: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn new(arr: [[T; N]; M]) -> Self {
        Self { arr }
    }
}

#[derive(Debug)]
pub struct MatrixVec<T> {
    vec: Vec<Vec<T>>,
}

impl<T> MatrixVec<T> {
    pub const fn new(vec: Vec<Vec<T>>) -> Self {
        Self { vec }
    }
}

macro_rules! matrix_mult_int_impl {
    ($($t:ty)+) => ($(
        impl<const M: usize, const N: usize> Matrix<$t, M, N> {
            pub const fn mult<const O: usize, const P: usize>(
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
        }
    )+)
}

macro_rules! matrix_mult_float_impl {
    ($($t:ty)+) => ($(
        impl<const M: usize, const N: usize> Matrix<$t, M, N> {
            pub fn mult<const O: usize, const P: usize>(
                &self,
                other: &Matrix<$t, O, P>,
            ) -> Matrix<$t, M, P> {
                if O != N { panic!() }
                let mut arr: [[$t; P]; M] = [[0.0; P]; M];
                let mut i = 0;
                while i < M {
                    let mut j = 0;
                    while j < P {
                        let mut k = 0;
                        let mut sum = 0.0;
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
        }
    )+)
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for MatrixVec<T> {
    fn from(value: [[T; N]; M]) -> Self {
        let vec = value.into_iter().map(|i| i.into_iter().collect()).collect();
        Self { vec }
    }
}

macro_rules! matrix_vec_mult_int_impl {
    ($($t:ty)+) => ($(
        impl MatrixVec<$t> {
            pub fn mult(&self, other: &MatrixVec<$t>) -> MatrixVec<$t> {
                let m = self.vec.len();
                let n = self.vec[0].len();
                let o = other.vec.len();
                let p = other.vec[0].len();
                if o != n {
                    panic!()
                }
                let mut vec: Vec<Vec<$t>> = vec![vec![0; p]; m];
                for i in 0..m {
                    for j in 0..p {
                        let mut sum = 0;
                        for k in 0..n {
                            sum += self.vec[i][k] * other.vec[k][j];
                        }
                        vec[i][j] = sum;
                    }
                }

                MatrixVec { vec }
            }
        }
    )+)
}

macro_rules! matrix_vec_mult_float_impl {
    ($($t:ty)+) => ($(
        impl MatrixVec<$t> {
            pub fn mult(&self, other: &MatrixVec<$t>) -> MatrixVec<$t> {
                let m = self.vec.len();
                let n = self.vec[0].len();
                let o = other.vec.len();
                let p = other.vec[0].len();
                if o != n {
                    panic!()
                }
                let mut vec: Vec<Vec<$t>> = vec![vec![0.0; p]; m];
                for i in 0..m {
                    for j in 0..p {
                        let mut sum = 0.0;
                        for k in 0..n {
                            sum += self.vec[i][k] * other.vec[k][j];
                        }
                        vec[i][j] = sum;
                    }
                }

                MatrixVec { vec }
            }
        }
    )+)
}

matrix_mult_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
matrix_mult_float_impl! { f32 f64 }

matrix_vec_mult_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
matrix_vec_mult_float_impl! { f32 f64 }
