// Comparison answer for lessons/016-add-broadcast.md; not library code.
use rumpy::Array;

pub fn add_broadcast(a: &Array, b: &Array) -> Array {
    // Align both input shapes from the right. A missing axis behaves like one.
    let rank = a.shape().len().max(b.shape().len());
    let mut output_shape = Vec::with_capacity(rank);
    for axis in 0..rank {
        let left = a
            .shape()
            .get(a.shape().len().wrapping_sub(rank - axis))
            .copied()
            .unwrap_or(1);
        let right = b
            .shape()
            .get(b.shape().len().wrapping_sub(rank - axis))
            .copied()
            .unwrap_or(1);
        if left != right && left != 1 && right != 1 {
            panic!("add_broadcast requires compatible shapes");
        }
        output_shape.push(if left == 1 { right } else { left });
    }
    // An empty result needs no strides or products; compatibility was checked above.
    if output_shape.contains(&0) {
        return Array::from_vec(Vec::new(), output_shape).unwrap();
    }
    let count = output_shape
        .iter()
        .try_fold(1usize, |size, &axis| size.checked_mul(axis))
        .expect("add_broadcast output size exceeds usize");
    let mut values = Vec::with_capacity(count);
    for flat in 0..count {
        let mut remaining = flat;
        let mut left_index = 0;
        let mut right_index = 0;
        for axis in 0..rank {
            // Convert a flat output index into each input's row-major index.
            let trailing = output_shape[axis + 1..].iter().product::<usize>();
            let coordinate = remaining / trailing;
            remaining %= trailing;
            if axis >= rank - a.shape().len() {
                let length = a.shape()[axis - (rank - a.shape().len())];
                left_index = left_index * length + if length == 1 { 0 } else { coordinate };
            }
            if axis >= rank - b.shape().len() {
                let length = b.shape()[axis - (rank - b.shape().len())];
                right_index = right_index * length + if length == 1 { 0 } else { coordinate };
            }
        }
        values.push(a.as_slice()[left_index] + b.as_slice()[right_index]);
    }
    Array::from_vec(values, output_shape).unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::add_broadcast;
    use rumpy::Array;

    fn array(values: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(values.to_vec(), shape.to_vec()).unwrap()
    }

    fn bits(a: &Array) -> Vec<u64> {
        a.as_slice().iter().map(|value| value.to_bits()).collect()
    }

    fn check(a: &Array, b: &Array, shape: &[usize], expected: &[f64]) {
        let before_a = bits(a);
        let before_b = bits(b);
        let mut result = add_broadcast(a, b);
        assert_eq!(result.shape(), shape);
        assert_eq!(result.size(), expected.len());
        for (actual, value) in result.as_slice().iter().zip(expected) {
            if value.is_nan() {
                assert!(actual.is_nan());
            } else {
                assert_eq!(actual.to_bits(), value.to_bits());
            }
        }
        assert_eq!(bits(a), before_a);
        assert_eq!(bits(b), before_b);
        if !expected.is_empty() {
            result.as_mut_slice()[0] = 42.0;
            assert_eq!(bits(a), before_a);
            assert_eq!(bits(b), before_b);
        }
    }

    #[test]
    fn equal_shapes_keep_previous_addition() {
        check(
            &array(&[1.0, 2.0], &[2]),
            &array(&[3.0, 4.0], &[2]),
            &[2],
            &[4.0, 6.0],
        );
        check(&array(&[2.0], &[]), &array(&[3.0], &[]), &[], &[5.0]);
    }

    #[test]
    fn trailing_axes_and_both_singletons_expand() {
        let a = array(&[1.0, 2.0, 3.0], &[3]);
        let b = array(&[10.0, 20.0], &[2, 1]);
        check(&a, &b, &[2, 3], &[11.0, 12.0, 13.0, 21.0, 22.0, 23.0]);
        check(&b, &a, &[2, 3], &[11.0, 12.0, 13.0, 21.0, 22.0, 23.0]);
        check(&array(&[1.0], &[]), &a, &[3], &[2.0, 3.0, 4.0]);
        check(
            &array(&[1.0, 2.0], &[1, 2, 1]),
            &array(&[10.0, 20.0, 30.0], &[3]),
            &[1, 2, 3],
            &[11.0, 21.0, 31.0, 12.0, 22.0, 32.0],
        );
    }

    #[test]
    fn empty_axes_preserve_the_full_shape_without_products() {
        check(&array(&[], &[0, 1]), &array(&[1.0], &[1]), &[0, 1], &[]);
        check(
            &array(&[], &[usize::MAX, 2, 0]),
            &array(&[1.0], &[]),
            &[usize::MAX, 2, 0],
            &[],
        );
        check(&array(&[], &[0]), &array(&[3.0], &[1]), &[0], &[]);
    }

    #[test]
    fn floating_point_results_follow_f64_addition() {
        let nan = f64::from_bits(0x7ff8_0000_0000_1234);
        check(
            &array(&[-0.0, f64::INFINITY, nan], &[3]),
            &array(&[-0.0], &[1]),
            &[3],
            &[-0.0, f64::INFINITY, f64::NAN],
        );
        check(
            &array(&[f64::INFINITY], &[]),
            &array(&[f64::NEG_INFINITY], &[1]),
            &[1],
            &[f64::NAN],
        );
    }

    #[test]
    fn incompatible_axes_fail_before_empty_or_allocation() {
        for (a, b) in [
            (array(&[1.0, 2.0], &[2]), array(&[1.0, 2.0, 3.0], &[3])),
            (array(&[], &[0, 2]), array(&[], &[0, 3])),
            (array(&[], &[0, 2]), array(&[], &[0, usize::MAX, 2])),
        ] {
            let failure = std::panic::catch_unwind(|| add_broadcast(&a, &b)).unwrap_err();
            let text = failure
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| failure.downcast_ref::<&str>().copied());
            assert_eq!(text, Some("add_broadcast requires compatible shapes"));
        }
    }

    #[test]
    fn empty_output_with_huge_axis_does_not_overflow() {
        let a = array(&[1.0], &[]);
        let b = array(&[], &[0, usize::MAX]);
        check(&a, &b, &[0, usize::MAX], &[]);
    }
}
