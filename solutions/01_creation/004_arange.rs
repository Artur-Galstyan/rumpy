// Comparison reference for exercise 004. This file is not library code.
use rumpy::Array;

/// Return the supported positive-step integer range as owned f64 data.
pub fn arange(start: i32, stop: i32, step: i32) -> Array {
    // Validate before the empty-interval case, so invalid steps always fail.
    assert!(step > 0, "step must be positive");
    // Widen before arithmetic. The final increment can pass i32::MAX.
    let mut current = i64::from(start);
    let stop = i64::from(stop);
    let step = i64::from(step);
    let mut data = Vec::new();
    while current < stop {
        // Every emitted value stays within i32 bounds and is exact in binary64.
        data.push(current as f64);
        current += step;
    }
    // An empty range has shape [0], never the scalar shape [].
    let length = data.len();
    Array::from_vec(data, vec![length]).expect("range length matches its one-axis shape")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::arange;

    fn assert_range(start: i32, stop: i32, step: i32, expected: &[f64]) {
        let a = arange(start, stop, step);
        assert_eq!(a.shape(), &[expected.len()]);
        assert_eq!(a.ndim(), 1);
        assert_eq!(a.size(), expected.len());
        assert_eq!(a.as_slice(), expected);
        for (actual, expected) in a.as_slice().iter().zip(expected) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn unit_step_starts_at_the_given_value() {
        assert_range(0, 5, 1, &[0.0, 1.0, 2.0, 3.0, 4.0]);
        assert_range(3, 6, 1, &[3.0, 4.0, 5.0]);
    }

    #[test]
    fn exact_stop_is_excluded() {
        assert_range(2, 8, 2, &[2.0, 4.0, 6.0]);
    }

    #[test]
    fn step_can_pass_the_stop() {
        assert_range(2, 9, 3, &[2.0, 5.0, 8.0]);
        assert_range(4, 5, 20, &[4.0]);
    }

    #[test]
    fn negative_values_and_zero_are_supported() {
        assert_range(-5, 3, 2, &[-5.0, -3.0, -1.0, 1.0]);
        assert_range(-4, 3, 2, &[-4.0, -2.0, 0.0, 2.0]);
        assert_range(-6, -1, 2, &[-6.0, -4.0, -2.0]);
    }

    #[test]
    fn empty_intervals_are_vectors_not_scalars() {
        assert_range(3, 3, 1, &[]);
        assert_range(7, -3, 2, &[]);
        assert_range(i32::MAX, i32::MIN, 1, &[]);
    }

    #[test]
    fn final_increment_must_not_overflow_i32() {
        assert_range(2_147_483_646, i32::MAX, 2, &[2_147_483_646.0]);
        assert_range(
            i32::MIN,
            -2_147_483_643,
            2,
            &[-2_147_483_648.0, -2_147_483_646.0, -2_147_483_644.0],
        );
    }

    #[test]
    fn span_can_exceed_i32_max() {
        assert_range(
            i32::MIN,
            i32::MAX,
            i32::MAX,
            &[-2_147_483_648.0, -1.0, 2_147_483_646.0],
        );
    }

    #[test]
    fn each_call_owns_its_storage() {
        let mut a = arange(1, 4, 1);
        let b = arange(1, 4, 1);
        a.as_mut_slice()[0] = 99.0;
        assert_eq!(a.as_slice(), &[99.0, 2.0, 3.0]);
        assert_eq!(b.as_slice(), &[1.0, 2.0, 3.0]);
        assert_eq!(a.shape(), &[3]);
        assert_eq!(b.shape(), &[3]);
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn zero_step_is_rejected() {
        arange(0, 3, 0);
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn negative_step_is_rejected() {
        arange(3, 0, -1);
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn invalid_step_is_rejected_even_for_an_empty_interval() {
        arange(2, 2, 0);
    }
}
