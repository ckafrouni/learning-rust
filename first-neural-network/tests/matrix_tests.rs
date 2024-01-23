use first_neural_network::linalg::*;
use first_neural_network::matrix;

#[test]
fn add() -> Result<(), MatrixError> {
    let mut a = matrix![
        1.0, 2.0, 3.0;
        4.0, 5.0, 6.0;
    ];

    let b = matrix![
        5.0, 4.0, 3.0;
        2.0, 1.0, 0.0
    ];

    a.add(&b)?;

    assert_eq!(vec![6.0; 6], a.data);
    Ok(())
}

#[test]
fn sub() {
    let mut a = matrix![
        1.0, 2.0, 3.0;
        4.0, 5.0, 6.0
    ];

    let b = matrix![
        4.0, 5.0, 6.0;
        7.0, 8.0, 9.0
    ];

    a.sub(&b).unwrap();

    assert_eq!(vec![-3.0; 6], a.data)
}

#[test]
fn scalar_prod() {
    let mut ma = matrix![
        1.0, 2.0;
        3.0, 4.0
    ];

    let scalar = 4.0;
    let res = vec![4.0, 8.0, 12.0, 16.0];
    ma.scalar_prod(scalar);

    assert_eq!(res, ma.data);
}

#[test]
fn dot_prod() {
    let mut a = matrix![
        1.0, 2.0, 3.0;
        4.0, 5.0, 6.0
    ];

    let b = matrix![
        4.0, 5.0;
        6.0, 7.0;
        8.0, 9.0
    ];

    a.dot_prod(&b).unwrap();

    assert_eq!((2, 2), a.dimensions());
    assert_eq!(
        vec![
            (4.0 + 12.0 + 24.0),
            (5.0 + 14.0 + 27.0),
            (16.0 + 30.0 + 48.0),
            (20.0 + 35.0 + 54.0),
        ],
        a.data
    );
}

#[test]
fn cross_prod() {}

#[test]
fn transpose() {
    let mut a = matrix![
        1.0, 2.0, 3.0;
        4.0, 5.0, 6.0
    ];

    let ta = matrix![
        1.0, 4.0;
        2.0, 5.0;
        3.0, 6.0
    ];

    a.transpose();

    assert_eq!(ta, a);
}

#[test]
fn det() {
    let m1 = Matrix::new(2, 2, vec![4.0, 7.0, 2.0, 6.0]).unwrap();
    assert_eq!(m1.det().unwrap(), 10.0); // Det = 4*6 - 7*2 = 24 - 14 = 10

    let m2 = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]).unwrap();
    assert_eq!(m2.det().unwrap(), 1.0); // For simplicity, chosen a matrix with a known determinant
}

#[test]
fn inverse() {
    let m = Matrix::new(2, 2, vec![4.0, 7.0, 2.0, 6.0]).unwrap();
    let minv = m.inverse().unwrap();

    // The expected inverse matrix calculated by hand or using a mathematical tool
    let expected_inv = Matrix::new(2, 2, vec![0.6, -0.7, -0.2, 0.4]).unwrap();

    // Since floating point calculations can have slight inaccuracies, consider using an epsilon for comparison
    let epsilon = 1e-9; // Adjust based on the precision you expect
    assert!(minv
        .data
        .iter()
        .zip(expected_inv.data.iter())
        .all(|(&a, &b)| (a - b).abs() < epsilon));
}
