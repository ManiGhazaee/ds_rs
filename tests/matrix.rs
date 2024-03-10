#![cfg(test)]

use ds_rs::matrix::{Matrix, MatrixVec};

#[test]
fn test_matrix_mult() {
    let m1 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);

    assert_eq!(
        m3,
        Matrix::new([[30, 36, 42], [66, 81, 96], [102, 126, 150]])
    );
    assert_eq!(
        m4,
        Matrix::new([[30, 36, 42], [66, 81, 96], [102, 126, 150]])
    );
}

#[test]
fn test_matrix_vec_mult() {
    let m1 = MatrixVec::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = MatrixVec::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);

    assert_eq!(
        m3,
        MatrixVec::from([[30, 36, 42], [66, 81, 96], [102, 126, 150]])
    );
    assert_eq!(
        m4,
        MatrixVec::from([[30, 36, 42], [66, 81, 96], [102, 126, 150]])
    );
}
