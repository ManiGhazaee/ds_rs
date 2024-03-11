#![cfg(test)]

use ds_rs::matrix::{Matrix, MatrixVec};

#[test]
fn test_matrix_mult() {
    let m1 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected = Matrix::new([[30, 36, 42], [66, 81, 96], [102, 126, 150]]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);
}

#[test]
fn test_matrix_vec_mult() {
    let m1 = MatrixVec::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = MatrixVec::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected = MatrixVec::from([[30, 36, 42], [66, 81, 96], [102, 126, 150]]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);
}

#[test]
fn test_matrix_edge_cases() {
    let m1 = Matrix::<i32, 3, 0>::new([[], [], []]);
    let m2 = Matrix::<i32, 0, 3>::new([]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected = Matrix::<i32, 3, 3>::new([[0, 0, 0], [0, 0, 0], [0, 0, 0]]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);

    let m1 = Matrix::<i32, 0, 0>::new([]);
    let m2 = Matrix::<i32, 0, 0>::new([]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected = Matrix::<i32, 0, 0>::new([]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);
}

#[test]
fn test_matrix_vec_edge_cases() {
    let m1 = MatrixVec::<i32>::from([[], [], []] as [[i32; 0]; 3]);
    let m2 = MatrixVec::<i32>::from([] as [[i32; 3]; 0]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected = MatrixVec::<i32>::from([[0, 0, 0], [0, 0, 0], [0, 0, 0]]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);

    let m1 = MatrixVec::<i32>::from([] as [[i32; 0]; 0]);
    let m2 = MatrixVec::<i32>::from([] as [[i32; 0]; 0]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected = MatrixVec::<i32>::from([] as [[i32; 0]; 0]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);
}