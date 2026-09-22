// Exercise 010: maximum of every stored value.
// Read lessons/010-amax.md. Keep the tests intact.

fn main() {}

#[cfg(test)]
mod tests {
    use rumpy::{Array, amax};

    fn assert_amax(values: &[f64], shape: &[usize], expected: f64) {
        let a = Array::from_vec(values.to_vec(), shape.to_vec()).unwrap();
        let before: Vec<_> = a.as_slice().iter().map(|v| v.to_bits()).collect();
        let actual = amax(&a);
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        assert_eq!(a.shape(), shape);
        assert_eq!(
            a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            before
        );
    }

    fn assert_empty(shape: &[usize]) {
        let a = Array::from_vec(vec![], shape.to_vec()).unwrap();
        let panic = std::panic::catch_unwind(|| amax(&a)).expect_err("empty input must panic");
        let message = panic
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied());
        assert_eq!(message, Some("amax requires at least one element"));
        assert_eq!(a.shape(), shape);
        assert!(a.as_slice().is_empty());
    }

    #[test]
    fn scalars_copy_the_stored_value() {
        for value in [3.5, -7.0, 0.0, -0.0, f64::MAX, -f64::MAX] {
            assert_amax(&[value], &[], value);
            assert_amax(&[value], &[1, 1, 1], value);
        }
    }

    #[test]
    fn negative_values_have_no_zero_identity() {
        assert_amax(&[-9.0, -2.0, -5.0], &[3], -2.0);
        assert_amax(&[-f64::MAX, -f64::MAX], &[2], -f64::MAX);
    }

    #[test]
    fn maximum_can_occur_at_every_position() {
        for position in 0..7 {
            let mut values = [-3.0; 7];
            values[position] = 12.5;
            assert_amax(&values, &[7], 12.5);
        }
        assert_amax(&[4.0, 4.0, -2.0, 4.0], &[4], 4.0);
    }

    #[test]
    fn all_ranks_reduce_all_elements() {
        for shape in [
            vec![6],
            vec![2, 3],
            vec![3, 2],
            vec![1, 2, 1, 3],
            vec![1, 1, 6, 1, 1],
        ] {
            assert_amax(&[-1.0, 2.0, 9.0, 4.0, -5.0, 6.0], &shape, 9.0);
        }
    }

    #[test]
    fn infinities_are_selected_without_arithmetic() {
        assert_amax(
            &[f64::NEG_INFINITY, f64::NEG_INFINITY],
            &[2],
            f64::NEG_INFINITY,
        );
        assert_amax(&[f64::INFINITY, f64::NEG_INFINITY], &[2], f64::INFINITY);
        assert_amax(&[f64::NEG_INFINITY, f64::INFINITY], &[2], f64::INFINITY);
        assert_amax(&[f64::NEG_INFINITY, -7.0], &[2], -7.0);
        assert_amax(&[f64::MAX, f64::MAX], &[2], f64::MAX);
    }

    #[test]
    fn signed_zero_ties_keep_the_first_stored_equal_value() {
        for (values, expected) in [
            (vec![-0.0, 0.0], -0.0),
            (vec![0.0, -0.0], 0.0),
            (vec![-0.0, -0.0], -0.0),
            (vec![0.0, 0.0], 0.0),
            (vec![-4.0, -0.0, 0.0], -0.0),
            (vec![-4.0, 0.0, -0.0], 0.0),
            (vec![-0.0, 0.0, 2.0], 2.0),
        ] {
            assert_amax(&values, &[values.len()], expected);
        }
    }

    #[test]
    fn subnormal_bits_are_preserved() {
        let tiny = f64::from_bits(1);
        let next = f64::from_bits(2);
        assert_amax(&[tiny], &[], tiny);
        assert_amax(&[tiny, next, 0.0], &[3], next);
        assert_amax(&[-next, -tiny], &[2], -tiny);
        assert_amax(&[-tiny, -0.0], &[2], -0.0);
    }

    #[test]
    fn nan_at_first_middle_or_last_propagates() {
        for bits in [
            0x7ff8_0000_0000_0042,
            0xfff8_0000_0000_1234,
            0x7ff0_0000_0000_0001,
            0xfff0_0000_0000_0042,
        ] {
            let nan = f64::from_bits(bits);
            assert_amax(&[nan], &[], f64::NAN);
            for position in 0..3 {
                let mut values = [f64::INFINITY, -0.0, f64::NEG_INFINITY];
                values[position] = nan;
                assert_amax(&values, &[3], f64::NAN);
            }
            assert_amax(&[nan, nan], &[1, 2], f64::NAN);
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
        ] {
            assert_empty(&shape);
        }
    }
}
