// Exercise 009: mean of every element of an owned contiguous f64 array.
// Read lessons/009-mean.md. Keep the tests intact.
use rumpy::Array;

/// Add flat values left-to-right from positive zero, then divide by count as f64.
/// Return NaN for empty arrays. Accept every rank and leave input unchanged.
/// Compare NaN results by class only. Do not depend on the unfinished sum exercise.
pub fn mean(a: &Array) -> f64 {
    // TODO (historical, completed): Reduce all stored values, then divide.
    // Empty input has no mean. No shape product or axis traversal is needed.
    if a.size() == 0 {
        return f64::NAN;
    }
    // Keep this local until the learner exposes a compatible sum API.
    let mut total = 0.0_f64;
    for &value in a.as_slice() {
        total += value;
    }
    // Divide only after the complete ordered sum, not per element.
    total / a.size() as f64
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::mean;
    use rumpy::Array;

    fn array(values: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(values.to_vec(), shape.to_vec()).unwrap()
    }

    fn assert_mean(values: &[f64], shape: &[usize], expected: f64) {
        let a = array(values, shape);
        let before: Vec<u64> = a.as_slice().iter().map(|v| v.to_bits()).collect();
        let strides_before = a.strides().to_vec();
        let actual = mean(&a);
        if expected.is_nan() {
            assert!(actual.is_nan());
        } else {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        assert_eq!(a.shape(), shape);
        assert_eq!(a.strides(), strides_before);
        assert_eq!(
            a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            before
        );
    }

    #[test]
    fn vectors_include_every_value() {
        assert_mean(&[1.0, 2.0, 3.0, 4.0], &[4], 2.5);
        assert_mean(&[-4.0, -2.0, 0.0], &[3], -2.0);
        assert_mean(&[0.5, -0.25, 0.125], &[3], 0.125);
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
            assert_mean(&[1.0, -2.0, 3.0, 4.0, -5.0, 6.0], &shape, 7.0 / 6.0);
        }
    }

    #[test]
    fn scalar_still_uses_the_initial_addition() {
        assert_mean(&[3.5], &[], 3.5);
        assert_mean(&[-0.0], &[], 0.0);
        assert_mean(&[-0.0], &[1, 1, 1], 0.0);
    }

    #[test]
    fn empty_arrays_return_nan() {
        for shape in [
            vec![0],
            vec![0, 0],
            vec![0, 7],
            vec![9, 0],
            vec![2, 0, 3, 1],
        ] {
            assert_mean(&[], &shape, f64::NAN);
        }
    }

    #[test]
    fn huge_empty_dimensions_need_no_axis_traversal() {
        for shape in [vec![usize::MAX, 0], vec![0, usize::MAX]] {
            assert_mean(&[], &shape, f64::NAN);
        }
    }

    #[test]
    fn positive_zero_identity_and_cancellation() {
        assert_mean(&[-0.0, -0.0], &[2], 0.0);
        assert_mean(&[0.0, -0.0, 0.0], &[3], 0.0);
        assert_mean(&[1.0, -1.0], &[2], 0.0);
    }

    #[test]
    fn flat_left_to_right_order_is_part_of_the_contract() {
        // A pairwise or compensated reduction need not give this result.
        assert_mean(&[1e16, 1.0, -1e16, 1.0], &[4], 0.25);
        assert_mean(&[1e16, 1.0, -1e16, 1.0], &[2, 2], 0.25);
        assert_mean(&[1e16, -1e16, 1.0, 1.0], &[4], 0.5);
    }

    #[test]
    fn subnormal_values_are_not_discarded() {
        assert_mean(
            &[f64::from_bits(1), f64::from_bits(1)],
            &[2],
            f64::from_bits(1),
        );
        assert_mean(
            &[f64::from_bits(0x8000_0000_0000_0001)],
            &[],
            f64::from_bits(0x8000_0000_0000_0001),
        );
    }

    #[test]
    fn infinities_follow_ieee_arithmetic() {
        assert_mean(&[2.0, f64::INFINITY, -3.0], &[3], f64::INFINITY);
        assert_mean(&[f64::NEG_INFINITY, 2.0], &[2], f64::NEG_INFINITY);
        assert_mean(&[f64::INFINITY, f64::NEG_INFINITY], &[2], f64::NAN);
        assert_mean(&[f64::NEG_INFINITY, f64::INFINITY], &[2], f64::NAN);
    }

    #[test]
    fn finite_overflow_is_not_an_error() {
        assert_mean(&[f64::MAX, f64::MAX, -f64::MAX], &[3], f64::INFINITY);
        assert_mean(&[-f64::MAX, -f64::MAX], &[2], f64::NEG_INFINITY);
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
            assert_mean(&[nan], &[], f64::NAN);
            assert_mean(&[-0.0, 2.0, nan, f64::INFINITY], &[2, 2], f64::NAN);
            assert_mean(&[nan, 2.0], &[2], f64::NAN);
        }
    }
    #[test]
    fn divide_only_after_the_complete_sum() {
        // Per-element division would lose both smallest subnormal inputs.
        assert_mean(
            &[f64::from_bits(1), f64::from_bits(1)],
            &[2],
            f64::from_bits(1),
        );
        // An online average avoids overflow, but does not meet this contract.
        assert_mean(&[f64::MAX, f64::MAX], &[2], f64::INFINITY);
    }

    #[test]
    fn division_can_underflow_to_signed_zero() {
        assert_mean(&[f64::from_bits(1), 0.0], &[2], 0.0);
        assert_mean(&[-f64::from_bits(1), 0.0], &[2], -0.0);
    }
}
