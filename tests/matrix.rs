#![cfg(test)]

use pretty_assertions::assert_eq;

#[test]
fn test_matrix_mult() {
    let m1 = ds_rs::matrix::array::Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = ds_rs::matrix::array::Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected =
        ds_rs::matrix::array::Matrix::new([[30, 36, 42], [66, 81, 96], [102, 126, 150]]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);
}

#[test]
fn test_matrix_vec_mult() {
    let m1 = ds_rs::matrix::vec::Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = ds_rs::matrix::vec::Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    let m3 = m1.mult_slow(&m2);
    let m4 = m1.mult_transpose(&m2);
    let m5 = m1.mult_par_transpose(&m2);
    let mult_expected =
        ds_rs::matrix::vec::Matrix::from([[30, 36, 42], [66, 81, 96], [102, 126, 150]]);

    assert_eq!(m3, mult_expected);
    assert_eq!(m4, mult_expected);
    assert_eq!(m5, mult_expected);
}

#[test]
fn test_matrix_add() {
    let m1 = ds_rs::matrix::array::Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let m2 = ds_rs::matrix::array::Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    let m3 = m1.add(&m2);
    let m4 = m1.add_par(&m2);
    let add_expected = ds_rs::matrix::array::Matrix::new([[2, 4, 6], [8, 10, 12], [14, 16, 18]]);

    assert_eq!(m3, add_expected);
    assert_eq!(m4, add_expected);
}
