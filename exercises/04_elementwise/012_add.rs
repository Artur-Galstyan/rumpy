// Read lessons/012-add.md. This exercise supports equal shapes only.
use rumpy::Array;

/// Add corresponding f64 values into a new Array with the same shape.
/// Unequal shapes panic with "add requires identical shapes"; no broadcasting.
pub fn add(a: &Array, b: &Array) -> Array {
    // TODO: Check the shape contract and add corresponding stored values.
    let _ = (a, b);
    todo!("add")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::add;
    use rumpy::Array;

    fn array(data: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(data.to_vec(), shape.to_vec()).unwrap()
    }

    fn bits(a: &Array) -> Vec<u64> {
        a.as_slice().iter().map(|x| x.to_bits()).collect()
    }

    fn check(left: &[f64], right: &[f64], shape: &[usize], expected: &[f64]) {
        let a = array(left, shape);
        let b = array(right, shape);
        let before_a = bits(&a);
        let before_b = bits(&b);
        let result = add(&a, &b);
        assert_eq!(result.shape(), shape);
        assert_eq!(result.size(), expected.len());
        for (actual, expected) in result.as_slice().iter().zip(expected) {
            if expected.is_nan() {
                assert!(actual.is_nan());
            } else {
                assert_eq!(actual.to_bits(), expected.to_bits());
            }
        }
        assert_eq!(a.shape(), shape);
        assert_eq!(b.shape(), shape);
        assert_eq!(bits(&a), before_a);
        assert_eq!(bits(&b), before_b);
    }

    #[test]
    fn vectors_add_each_corresponding_pair() {
        check(
            &[1.0, -2.0, 3.5, 0.25],
            &[4.0, 8.0, -1.5, -0.5],
            &[4],
            &[5.0, 6.0, 2.0, -0.25],
        );
    }

    #[test]
    fn matrix_preserves_row_major_order() {
        check(
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            &[60.0, 50.0, 40.0, 30.0, 20.0, 10.0],
            &[2, 3],
            &[61.0, 52.0, 43.0, 34.0, 25.0, 16.0],
        );
    }

    #[test]
    fn scalar_shape_contains_one_sum() {
        check(&[2.5], &[-4.0], &[], &[-1.5]);
    }

    #[test]
    fn higher_rank_keeps_singleton_axes() {
        check(
            &[1.0, 2.0, 3.0, 4.0],
            &[8.0, 7.0, 6.0, 5.0],
            &[1, 2, 1, 2],
            &[9.0; 4],
        );
    }

    #[test]
    fn equal_empty_shapes_keep_all_axes() {
        for shape in [vec![0], vec![2, 0, 3], vec![0, 0]] {
            check(&[], &[], &shape, &[]);
        }
    }

    #[test]
    fn huge_empty_axes_need_no_axis_traversal_or_products() {
        for shape in [
            vec![usize::MAX, 0],
            vec![0, usize::MAX, 2],
            vec![2, usize::MAX, 0],
        ] {
            check(&[], &[], &shape, &[]);
        }
    }

    #[test]
    fn signed_zeros_and_exact_cancellation() {
        check(
            &[0.0, 0.0, -0.0, -0.0, 2.0],
            &[0.0, -0.0, 0.0, -0.0, -2.0],
            &[5],
            &[0.0, 0.0, 0.0, -0.0, 0.0],
        );
    }

    #[test]
    fn rounding_subnormals_and_overflow_follow_f64_addition() {
        let tiny = f64::from_bits(1);
        check(
            &[9007199254740992.0, tiny, -tiny, f64::MAX, -f64::MAX],
            &[1.0, tiny, -tiny, f64::MAX, -f64::MAX],
            &[5],
            &[
                9007199254740992.0,
                f64::from_bits(2),
                -f64::from_bits(2),
                f64::INFINITY,
                f64::NEG_INFINITY,
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
                1.0,
                signaling,
            ],
            &[3.0, -3.0, f64::NEG_INFINITY, 2.0, nan, 0.0],
            &[6],
            &[
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NAN,
                f64::NAN,
                f64::NAN,
                f64::NAN,
            ],
        );
    }

    #[test]
    fn inputs_and_outputs_own_independent_storage() {
        let mut a = array(&[1.0, 2.0], &[2]);
        let b = array(&[3.0, 4.0], &[2]);
        let mut first = add(&a, &b);
        let second = add(&a, &b);
        first.as_mut_slice()[0] = 99.0;
        a.as_mut_slice()[1] = -99.0;
        assert_eq!(first.as_slice(), &[99.0, 6.0]);
        assert_eq!(second.as_slice(), &[4.0, 6.0]);
        assert_eq!(b.as_slice(), &[3.0, 4.0]);
        assert_eq!(a.as_slice(), &[1.0, -99.0]);
    }

    #[test]
    fn same_array_can_supply_both_inputs() {
        let a = array(&[1.0, -2.0], &[2]);
        let result = add(&a, &a);
        assert_eq!(result.as_slice(), &[2.0, -4.0]);
        assert_eq!(a.as_slice(), &[1.0, -2.0]);
    }

    #[test]
    fn unequal_shapes_panic_exactly_before_any_empty_shortcut() {
        let pairs = [
            (vec![2, 3], vec![3, 2]),
            (vec![2], vec![1]),
            (vec![], vec![1]),
            (vec![2, 1], vec![2, 3]),
            (vec![0, 2], vec![0, 3]),
            (vec![0], vec![1]),
            (vec![0, usize::MAX], vec![usize::MAX, 0]),
        ];
        for (left_shape, right_shape) in pairs {
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
                let panic = std::panic::catch_unwind(|| add(a, b)).unwrap_err();
                let message = panic
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic.downcast_ref::<&str>().copied());
                assert_eq!(message, Some("add requires identical shapes"));
                assert_eq!(a.as_slice(), vec![1.0; a.size()]);
                assert_eq!(b.as_slice(), vec![1.0; b.size()]);
            }
        }
    }
}
