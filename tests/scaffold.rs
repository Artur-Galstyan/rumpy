use rumpy::{Array, ShapeError};

#[test]
fn valid_container_preserves_shape_and_data() {
    let a = Array::from_vec(vec![1.0, 2.0], vec![1, 2]).unwrap();
    assert_eq!(a.shape(), &[1, 2]);
    assert_eq!(a.as_slice(), &[1.0, 2.0]);
}
#[test]
fn invalid_length_is_rejected() {
    assert_eq!(
        Array::from_vec(vec![0.0], vec![2]),
        Err(ShapeError::LengthMismatch {
            expected: 2,
            actual: 1
        })
    );
}
#[test]
fn scalar_requires_one_value() {
    assert_eq!(Array::from_vec(vec![5.0], vec![]).unwrap().size(), 1);
    assert!(Array::from_vec(vec![], vec![]).is_err());
}
#[test]
fn empty_axis_takes_precedence_over_overflow() {
    let a = Array::from_vec(vec![], vec![usize::MAX, 2, 0]).unwrap();
    assert_eq!(a.size(), 0);
}
#[test]
fn overflowing_nonempty_shape_is_rejected() {
    assert_eq!(
        Array::from_vec(vec![], vec![usize::MAX, 2]),
        Err(ShapeError::SizeOverflow)
    );
}
