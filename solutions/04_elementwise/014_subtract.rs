// Reference for lessons/014-subtract.md. Compare after your attempt.
use rumpy::Array;

/// Subtract corresponding f64 values into a new owned Array.
/// Unequal shapes panic with "subtract requires identical shapes".
pub fn subtract(a: &Array, b: &Array) -> Array {
    // A zip alone cannot detect differing shapes, even if lengths match.
    if a.shape() != b.shape() {
        panic!("subtract requires identical shapes");
    }
    // The input Array values already follow contiguous row-major order.
    let values = a
        .as_slice()
        .iter()
        .zip(b.as_slice())
        .map(|(&left, &right)| left - right)
        .collect();
    Array::from_vec(values, a.shape().to_vec()).expect("matching shapes preserve element count")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::subtract;
    use rumpy::Array;

    fn array(values: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(values.to_vec(), shape.to_vec()).unwrap()
    }

    fn bits(array: &Array) -> Vec<u64> {
        array
            .as_slice()
            .iter()
            .map(|value| value.to_bits())
            .collect()
    }

    fn check(left: &[f64], right: &[f64], shape: &[usize], expected: &[f64]) {
        let a = array(left, shape);
        let b = array(right, shape);
        let left_before = bits(&a);
        let right_before = bits(&b);
        let mut result = subtract(&a, &b);
        assert_eq!(result.shape(), shape);
        assert_eq!(result.size(), expected.len());
        for (actual, wanted) in result.as_slice().iter().zip(expected) {
            if wanted.is_nan() {
                assert!(actual.is_nan());
            } else {
                assert_eq!(actual.to_bits(), wanted.to_bits());
            }
        }
        assert_eq!(a.shape(), shape);
        assert_eq!(b.shape(), shape);
        assert_eq!(bits(&a), left_before);
        assert_eq!(bits(&b), right_before);
        if !result.as_slice().is_empty() {
            result.as_mut_slice()[0] = 42.0;
            assert_eq!(bits(&a), left_before);
            assert_eq!(bits(&b), right_before);
        }
    }

    #[test]
    fn order_matters_for_vectors() {
        check(
            &[5.0, -3.0, 0.25],
            &[2.0, 4.0, -0.5],
            &[3],
            &[3.0, -7.0, 0.75],
        );
        check(
            &[2.0, 4.0, -0.5],
            &[5.0, -3.0, 0.25],
            &[3],
            &[-3.0, 7.0, -0.75],
        );
    }

    #[test]
    fn matrix_and_higher_rank_keep_row_major_shape() {
        check(
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            &[6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
            &[2, 3],
            &[-5.0, -3.0, -1.0, 1.0, 3.0, 5.0],
        );
        check(
            &[1.0, 2.0, 3.0, 4.0],
            &[4.0, 3.0, 2.0, 1.0],
            &[1, 2, 1, 2],
            &[-3.0, -1.0, 1.0, 3.0],
        );
    }

    #[test]
    fn scalar_and_equal_empty_shapes() {
        check(&[2.5], &[-4.0], &[], &[6.5]);
        for shape in [
            vec![0],
            vec![2, 0, 3],
            vec![0, 0],
            vec![usize::MAX, 0],
            vec![0, usize::MAX, 2],
        ] {
            check(&[], &[], &shape, &[]);
        }
    }

    #[test]
    fn signed_zeros_use_direct_subtraction() {
        check(
            &[0.0, -0.0, 0.0, -0.0],
            &[0.0, 0.0, -0.0, -0.0],
            &[4],
            &[0.0, -0.0, 0.0, 0.0],
        );
    }

    #[test]
    fn infinities_nan_and_overflow() {
        let signaling = f64::from_bits(0x7ff0_0000_0000_0001);
        check(
            &[
                f64::INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::MAX,
                signaling,
            ],
            &[
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                -f64::MAX,
                1.0,
            ],
            &[5],
            &[
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NAN,
            ],
        );
    }

    #[test]
    fn rounding_and_subnormals() {
        let tiny = f64::from_bits(1);
        check(
            &[tiny, 1.0, 9007199254740992.0],
            &[tiny, tiny, -1.0],
            &[3],
            &[0.0, 1.0, 9007199254740992.0],
        );
    }

    #[test]
    fn same_input_can_supply_both_arguments() {
        let a = array(&[1.0, -0.0, f64::INFINITY], &[3]);
        let result = subtract(&a, &a);
        assert_eq!(result.as_slice()[0].to_bits(), 0.0f64.to_bits());
        assert_eq!(result.as_slice()[1].to_bits(), 0.0f64.to_bits());
        assert!(result.as_slice()[2].is_nan());
        assert_eq!(a.as_slice()[1].to_bits(), (-0.0f64).to_bits());
    }

    #[test]
    fn mismatched_shapes_panic_before_empty_handling() {
        for (left_shape, right_shape) in [
            (vec![2, 3], vec![3, 2]),
            (vec![2], vec![1]),
            (vec![], vec![1]),
            (vec![0, 2], vec![0, 3]),
            (vec![0, usize::MAX], vec![usize::MAX, 0]),
        ] {
            let make = |shape: &[usize]| {
                let count = if shape.contains(&0) {
                    0
                } else {
                    shape.iter().product()
                };
                array(&vec![1.0; count], shape)
            };
            let a = make(&left_shape);
            let b = make(&right_shape);
            for (a, b) in [(&a, &b), (&b, &a)] {
                let panic = std::panic::catch_unwind(|| subtract(a, b)).unwrap_err();
                let message = panic
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic.downcast_ref::<&str>().copied());
                assert_eq!(message, Some("subtract requires identical shapes"));
            }
        }
    }
}
