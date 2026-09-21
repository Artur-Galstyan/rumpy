// Exercise 008: sum every element of an owned contiguous f64 array.
// Read lessons/008-sum.md. Keep the tests intact.

fn main() {}

#[cfg(test)]
mod tests {

    use rumpy::{Array, operations::sum};

    fn array(values: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(values.to_vec(), shape.to_vec()).unwrap()
    }

    fn assert_sum(values: &[f64], shape: &[usize], expected: f64) {
        let a = array(values, shape);
        let before: Vec<u64> = a.as_slice().iter().map(|v| v.to_bits()).collect();
        let actual = sum(&a);
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

    #[test]
    fn vectors_include_every_value() {
        assert_sum(&[1.0, 2.0, 3.0, 4.0], &[4], 10.0);
        assert_sum(&[-4.0, -2.0, 1.0], &[3], -5.0);
        assert_sum(&[0.5, -0.25, 0.125], &[3], 0.375);
    }

    #[test]
    fn all_axes_reduce_to_one_rust_scalar() {
        for shape in [
            vec![6],
            vec![2, 3],
            vec![3, 2],
            vec![1, 2, 1, 3],
            vec![1, 1, 6, 1, 1],
        ] {
            assert_sum(&[1.0, -2.0, 3.0, 4.0, -5.0, 6.0], &shape, 7.0);
        }
    }

    #[test]
    fn scalar_still_uses_the_initial_addition() {
        assert_sum(&[3.5], &[], 3.5);
        assert_sum(&[-0.0], &[], 0.0);
        assert_sum(&[-0.0], &[1, 1, 1], 0.0);
    }

    #[test]
    fn empty_arrays_return_positive_zero() {
        for shape in [
            vec![0],
            vec![0, 0],
            vec![0, 7],
            vec![9, 0],
            vec![2, 0, 3, 1],
        ] {
            assert_sum(&[], &shape, 0.0);
        }
    }

    #[test]
    fn huge_empty_dimensions_need_no_axis_traversal() {
        for shape in [
            vec![usize::MAX, 0],
            vec![0, usize::MAX],
            vec![usize::MAX, usize::MAX, 0],
        ] {
            assert_sum(&[], &shape, 0.0);
        }
    }

    #[test]
    fn positive_zero_identity_and_cancellation() {
        assert_sum(&[-0.0, -0.0], &[2], 0.0);
        assert_sum(&[0.0, -0.0, 0.0], &[3], 0.0);
        assert_sum(&[1.0, -1.0], &[2], 0.0);
    }

    #[test]
    fn flat_left_to_right_order_is_part_of_the_contract() {
        // A pairwise or compensated reduction need not give this result.
        assert_sum(&[1e16, 1.0, -1e16, 1.0], &[4], 1.0);
        assert_sum(&[1e16, 1.0, -1e16, 1.0], &[2, 2], 1.0);
        assert_sum(&[1e16, -1e16, 1.0, 1.0], &[4], 2.0);
    }

    #[test]
    fn subnormal_values_are_not_discarded() {
        assert_sum(
            &[f64::from_bits(1), f64::from_bits(1)],
            &[2],
            f64::from_bits(2),
        );
        assert_sum(
            &[f64::from_bits(0x8000_0000_0000_0001)],
            &[],
            f64::from_bits(0x8000_0000_0000_0001),
        );
    }

    #[test]
    fn infinities_follow_ieee_arithmetic() {
        assert_sum(&[2.0, f64::INFINITY, -3.0], &[3], f64::INFINITY);
        assert_sum(&[f64::NEG_INFINITY, 2.0], &[2], f64::NEG_INFINITY);
        assert_sum(&[f64::INFINITY, f64::NEG_INFINITY], &[2], f64::NAN);
        assert_sum(&[f64::NEG_INFINITY, f64::INFINITY], &[2], f64::NAN);
    }

    #[test]
    fn finite_overflow_is_not_an_error() {
        assert_sum(&[f64::MAX, f64::MAX, -f64::MAX], &[3], f64::INFINITY);
        assert_sum(&[-f64::MAX, -f64::MAX], &[2], f64::NEG_INFINITY);
    }

    #[test]
    fn nan_class_and_input_payloads() {
        for bits in [
            0x7ff8_0000_0000_0042,
            0xfff8_0000_0000_1234,
            0x7ff0_0000_0000_0001,
            0xfff0_0000_0000_0042,
        ] {
            let nan = f64::from_bits(bits);
            assert_sum(&[nan], &[], f64::NAN);
            assert_sum(&[-0.0, 2.0, nan, f64::INFINITY], &[2, 2], f64::NAN);
            assert_sum(&[nan, 2.0], &[2], f64::NAN);
        }
    }
}
