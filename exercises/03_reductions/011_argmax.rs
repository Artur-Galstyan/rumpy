// Exercise 011: flat index of the first maximum.
// Read lessons/011-argmax.md. Keep the tests intact.
use rumpy::Array;

/// Reduce all ranks to a flat row-major usize index without changing the input.
/// Return the first NaN index, or the first maximum index when no NaN exists.
/// Both zero signs tie. Empty input panics with "argmax requires at least one element".
pub fn argmax(a: &Array) -> usize {
    if a.size() == 0 {
        panic!("argmax requires at least one element")
    }

    let mut res = (0, f64::NEG_INFINITY);

    for (i, v) in a.as_slice().iter().enumerate() {
        if v.is_nan() {
            return i;
        }
        if *v > res.1 {
            res.0 = i;
            res.1 = *v;
        }
    }
    res.0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::argmax;
    use rumpy::Array;

    fn assert_argmax(values: &[f64], shape: &[usize], expected: usize) {
        let a = Array::from_vec(values.to_vec(), shape.to_vec()).unwrap();
        let before: Vec<_> = a.as_slice().iter().map(|v| v.to_bits()).collect();
        assert_eq!(argmax(&a), expected);
        assert_eq!(a.shape(), shape);
        assert_eq!(
            a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            before
        );
    }

    fn assert_empty(shape: &[usize]) {
        let a = Array::from_vec(vec![], shape.to_vec()).unwrap();
        let panic = std::panic::catch_unwind(|| argmax(&a)).expect_err("empty input must panic");
        let message = panic
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied());
        assert_eq!(message, Some("argmax requires at least one element"));
        assert_eq!(a.shape(), shape);
        assert!(a.as_slice().is_empty());
    }

    #[test]
    fn scalars_and_singletons_return_zero() {
        for value in [
            3.5,
            -7.0,
            0.0,
            -0.0,
            f64::MAX,
            -f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
        ] {
            assert_argmax(&[value], &[], 0);
            assert_argmax(&[value], &[1, 1, 1], 0);
        }
    }

    #[test]
    fn negative_values_have_no_zero_identity() {
        assert_argmax(&[-9.0, -2.0, -5.0], &[3], 1);
        assert_argmax(&[-f64::MAX, -f64::MAX], &[2], 0);
    }

    #[test]
    fn maximum_can_occur_at_every_position() {
        for position in 0..7 {
            let mut values = [-3.0; 7];
            values[position] = 12.5;
            assert_argmax(&values, &[7], position);
        }
    }

    #[test]
    fn all_ranks_return_flat_row_major_indices() {
        for shape in [
            vec![6],
            vec![2, 3],
            vec![3, 2],
            vec![1, 2, 1, 3],
            vec![1, 1, 6, 1, 1],
        ] {
            assert_argmax(&[-1.0, 2.0, 3.0, 4.0, 9.0, 6.0], &shape, 4);
        }
    }

    #[test]
    fn repeated_maxima_keep_the_first_index() {
        assert_argmax(&[-1.0, 9.0, 9.0, 2.0, 9.0, 0.0], &[2, 3], 1);
        assert_argmax(&[4.0, 4.0, 4.0, 4.0], &[4], 0);
        assert_argmax(&[-8.0, -3.0, -3.0], &[3], 1);
    }

    #[test]
    fn infinities_follow_numeric_order_and_first_ties() {
        assert_argmax(&[f64::NEG_INFINITY, f64::NEG_INFINITY], &[2], 0);
        assert_argmax(&[f64::INFINITY, f64::NEG_INFINITY], &[2], 0);
        assert_argmax(&[f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY], &[3], 1);
        assert_argmax(&[f64::NEG_INFINITY, -7.0], &[2], 1);
        assert_argmax(&[f64::MAX, f64::MAX], &[2], 0);
    }

    #[test]
    fn both_zero_signs_tie() {
        for values in [
            vec![-0.0, 0.0],
            vec![0.0, -0.0],
            vec![-0.0, -0.0],
            vec![0.0, 0.0],
        ] {
            assert_argmax(&values, &[2], 0);
        }
        assert_argmax(&[-4.0, -0.0, 0.0], &[3], 1);
        assert_argmax(&[-4.0, 0.0, -0.0], &[3], 1);
        assert_argmax(&[-0.0, 0.0, 2.0], &[3], 2);
    }

    #[test]
    fn subnormals_follow_numeric_order() {
        let tiny = f64::from_bits(1);
        let next = f64::from_bits(2);
        assert_argmax(&[tiny, next, 0.0], &[3], 1);
        assert_argmax(&[-next, -tiny], &[2], 1);
        assert_argmax(&[-tiny, -0.0], &[2], 1);
        assert_argmax(&[next, tiny, next], &[3], 0);
    }

    #[test]
    fn nan_at_first_middle_or_last_wins_over_infinity() {
        for position in 0..3 {
            let mut values = [f64::INFINITY, -0.0, f64::NEG_INFINITY];
            values[position] = f64::NAN;
            assert_argmax(&values, &[3], position);
        }
    }

    #[test]
    fn quiet_and_signaling_nan_bits_stay_unchanged() {
        for bits in [
            0x7ff8_0000_0000_0042,
            0xfff8_0000_0000_1234,
            0x7ff0_0000_0000_0001,
            0xfff0_0000_0000_0042,
        ] {
            let nan = f64::from_bits(bits);
            assert_argmax(&[nan], &[], 0);
            for position in 0..3 {
                let mut values = [f64::INFINITY, 7.0, f64::NEG_INFINITY];
                values[position] = nan;
                assert_argmax(&values, &[3], position);
            }
        }
    }

    #[test]
    fn multiple_nans_keep_the_first_nan_index() {
        let quiet = f64::from_bits(0xfff8_0000_0000_1234);
        let signaling = f64::from_bits(0x7ff0_0000_0000_0001);
        for (first, second) in [(quiet, signaling), (signaling, quiet)] {
            assert_argmax(&[f64::INFINITY, first, -2.0, second], &[2, 2], 1);
            assert_argmax(&[first, second], &[2], 0);
        }
    }

    #[test]
    fn empty_shapes_panic_with_the_exact_message() {
        for shape in [
            vec![0],
            vec![0, 0],
            vec![0, 7],
            vec![9, 0],
            vec![2, 0, 3, 1],
        ] {
            assert_empty(&shape);
        }
    }

    #[test]
    fn huge_empty_shapes_panic_without_axis_traversal() {
        for shape in [
            vec![usize::MAX, 0],
            vec![0, usize::MAX],
            vec![usize::MAX, usize::MAX, 0],
            vec![0, usize::MAX, usize::MAX],
            vec![usize::MAX, 0, usize::MAX],
        ] {
            assert_empty(&shape);
        }
    }
}
