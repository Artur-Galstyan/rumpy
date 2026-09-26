// Reference for lessons/013-multiply.md. Compare after your attempt.
use rumpy::Array;

/// Multiply corresponding f64 values into a new Array with the same shape.
/// Unequal shapes panic with "multiply requires identical shapes"; no broadcasting.
pub fn multiply(a: &Array, b: &Array) -> Array {
    // Validate entire shapes before even an empty-slice shortcut.
    if a.shape() != b.shape() {
        panic!("multiply requires identical shapes");
    }
    // Valid equal shapes imply equal stored lengths. Each result uses exactly one product.
    let data = a
        .as_slice()
        .iter()
        .zip(b.as_slice())
        .map(|(&x, &y)| x * y)
        .collect();
    Array::from_vec(data, a.shape().to_vec()).expect("equal shapes preserve element count")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::multiply;
    use rumpy::Array;

    fn array(data: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(data.to_vec(), shape.to_vec()).unwrap()
    }

    fn bits(a: &Array) -> Vec<u64> {
        a.as_slice().iter().map(|value| value.to_bits()).collect()
    }

    fn check(left: &[f64], right: &[f64], shape: &[usize], expected: &[f64]) {
        let a = array(left, shape);
        let b = array(right, shape);
        let before_a = bits(&a);
        let before_b = bits(&b);
        let result = multiply(&a, &b);
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
        assert_eq!(bits(&a), before_a);
        assert_eq!(bits(&b), before_b);
    }

    #[test]
    fn vectors_multiply_each_corresponding_pair() {
        check(
            &[1.0, -2.0, 3.5, 0.25],
            &[4.0, 8.0, -2.0, -0.5],
            &[4],
            &[4.0, -16.0, -7.0, -0.125],
        );
    }

    #[test]
    fn matrix_preserves_row_major_order() {
        check(
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            &[6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
            &[2, 3],
            &[6.0, 10.0, 12.0, 12.0, 10.0, 6.0],
        );
    }

    #[test]
    fn scalar_shape_contains_one_product() {
        check(&[2.5], &[-4.0], &[], &[-10.0]);
    }

    #[test]
    fn higher_rank_keeps_singleton_axes() {
        check(
            &[1.0, 2.0, 3.0, 4.0],
            &[8.0, 7.0, 6.0, 5.0],
            &[1, 2, 1, 2],
            &[8.0, 14.0, 18.0, 20.0],
        );
    }

    #[test]
    fn equal_empty_shapes_keep_all_axes() {
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
    fn signed_zeros_and_exact_multiplication() {
        check(
            &[0.0, 0.0, -0.0, -0.0, -2.0, -2.0],
            &[3.0, -3.0, 3.0, -3.0, 0.0, -0.0],
            &[6],
            &[0.0, -0.0, -0.0, 0.0, -0.0, 0.0],
        );
    }

    #[test]
    fn rounding_subnormals_and_overflow_follow_f64_multiplication() {
        let tiny = f64::from_bits(1);
        check(
            &[tiny, tiny, f64::MAX, f64::MIN_POSITIVE, 9007199254740992.0],
            &[2.0, 0.5, 2.0, 0.5, 1.0 + f64::EPSILON],
            &[5],
            &[
                f64::from_bits(2),
                0.0,
                f64::INFINITY,
                f64::MIN_POSITIVE / 2.0,
                9007199254740994.0,
            ],
        );
    }

    #[test]
    fn infinities_and_nans_use_classes_not_nan_payloads() {
        let nan = f64::from_bits(0x7ff8_0000_0000_1234);
        let signaling = f64::from_bits(0x7ff0_0000_0000_0001);
        check(
            &[
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                nan,
                signaling,
                1.0,
            ],
            &[3.0, -3.0, 0.0, 2.0, 0.0, nan],
            &[6],
            &[
                f64::INFINITY,
                f64::INFINITY,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
            ],
        );
    }

    #[test]
    fn inputs_and_outputs_own_independent_storage() {
        let mut a = array(&[2.0, 3.0], &[2]);
        let b = array(&[4.0, 5.0], &[2]);
        let mut first = multiply(&a, &b);
        let second = multiply(&a, &b);
        first.as_mut_slice()[0] = 99.0;
        a.as_mut_slice()[1] = -99.0;
        assert_eq!(first.as_slice(), &[99.0, 15.0]);
        assert_eq!(second.as_slice(), &[8.0, 15.0]);
        assert_eq!(b.as_slice(), &[4.0, 5.0]);
        assert_eq!(a.as_slice(), &[2.0, -99.0]);
    }

    #[test]
    fn same_array_can_supply_both_inputs() {
        let a = array(&[1.5, -2.0], &[2]);
        let result = multiply(&a, &a);
        assert_eq!(result.as_slice(), &[2.25, 4.0]);
        assert_eq!(a.as_slice(), &[1.5, -2.0]);
    }

    #[test]
    fn unequal_shapes_panic_before_empty_shortcut() {
        for (left_shape, right_shape) in [
            (vec![2, 3], vec![3, 2]),
            (vec![2], vec![1]),
            (vec![], vec![1]),
            (vec![0, 2], vec![0, 3]),
            (vec![0, usize::MAX], vec![usize::MAX, 0]),
        ] {
            let make = |shape: &[usize]| {
                let size = if shape.contains(&0) {
                    0
                } else {
                    shape.iter().product()
                };
                array(&vec![1.0; size], shape)
            };
            let a = make(&left_shape);
            let b = make(&right_shape);
            for (a, b) in [(&a, &b), (&b, &a)] {
                let panic = std::panic::catch_unwind(|| multiply(a, b)).unwrap_err();
                let message = panic
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic.downcast_ref::<&str>().copied());
                assert_eq!(message, Some("multiply requires identical shapes"));
            }
        }
    }
}
