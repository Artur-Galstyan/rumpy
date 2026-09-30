// Read lessons/015-broadcast-to.md. Align input axes from the right.
use rumpy::Array;

/// Copy an owned array into a new array with a compatible target shape.
/// Incompatible shapes panic with "broadcast_to requires compatible shapes".
pub fn broadcast_to(input: &Array, shape: &[usize]) -> Array {
    if input.shape().len() > shape.len() {
        panic!("broadcast_to requires compatible shapes")
    }
    let mut input_shape = input.shape().to_vec();
    let to_pad = shape.len() - input.shape().len();
    (0..to_pad).for_each(|i| input_shape.insert(i, 1));

    assert!(
        input_shape.len() == shape.len(),
        "input_shape.len() after padding != target shape.len()"
    );

    let output_shape: Vec<usize> = input_shape
        .iter()
        .zip(shape)
        .map(|shapes| {
            let (i, t) = shapes;
            if i != t && *i != 1 {
                panic!("broadcast_to requires compatible shapes")
            }
            *t
        })
        .collect();
    let mut total: usize = 1;
    for &dim in shape {
        total = total
            .checked_mul(dim)
            .expect("broadcast_to target size exceeds usize");
    }
    let mut data: Vec<f64> = Vec::new();
    for i in 0..total {
        let mut temp = i;
        let mut coords = vec![0; shape.len()];

        for axis in (0..shape.len()).rev() {
            coords[axis] = temp % shape[axis];
            temp /= shape[axis];
        }

        let mut input_coords = vec![0; shape.len()];
        for axis in 0..input_shape.len() {
            if input_shape[axis] != 1 {
                input_coords[axis] = coords[axis];
            }
        }

        let mut in_idx = 0;
        for axis in 0..input_shape.len() {
            in_idx = in_idx * input_shape[axis] + input_coords[axis];
        }
        data.push(input.as_slice()[in_idx]);
    }

    Array::from_vec(data, output_shape.to_vec()).expect("invalid args")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::broadcast_to;
    use rumpy::Array;

    fn array(values: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(values.to_vec(), shape.to_vec()).unwrap()
    }

    fn check(input: &Array, shape: &[usize], expected: &[f64]) {
        let input_shape = input.shape().to_vec();
        let input_bits: Vec<u64> = input.as_slice().iter().map(|x| x.to_bits()).collect();
        let mut output = broadcast_to(input, shape);
        assert_eq!(output.shape(), shape);
        assert_eq!(output.size(), expected.len());
        assert_eq!(
            output
                .as_slice()
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
            expected.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(input.shape(), input_shape);
        assert_eq!(
            input
                .as_slice()
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
            input_bits
        );
        if !expected.is_empty() {
            output.as_mut_slice()[0] = 42.0;
            assert_eq!(
                input
                    .as_slice()
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                input_bits
            );
        }
    }

    #[test]
    fn scalar_expands_and_remains_scalar() {
        let input = array(&[2.5], &[]);
        check(&input, &[], &[2.5]);
        check(&input, &[2, 3], &[2.5; 6]);
    }

    #[test]
    fn trailing_axes_align_and_singletons_repeat() {
        check(
            &array(&[1.0, 2.0, 3.0], &[3]),
            &[2, 3],
            &[1.0, 2.0, 3.0, 1.0, 2.0, 3.0],
        );
        check(
            &array(&[1.0, 2.0], &[2, 1]),
            &[2, 3],
            &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0],
        );
        check(
            &array(&[1.0, 2.0], &[1, 2, 1]),
            &[3, 2, 4],
            &[
                1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0,
                1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0,
            ],
        );
    }

    #[test]
    fn exact_shape_copies_bits() {
        let input = array(
            &[f64::from_bits(0x7ff0_0000_0000_0001), -0.0, f64::INFINITY],
            &[1, 3],
        );
        check(&input, &[1, 3], input.as_slice());
        let output = broadcast_to(&input, &[2, 3]);
        for row in output.as_slice().chunks_exact(3) {
            assert_eq!(
                row.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                input
                    .as_slice()
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn empty_targets_preserve_shape_without_axis_products() {
        check(&array(&[7.0], &[1]), &[0, usize::MAX, 2], &[]);
        check(&array(&[], &[0, 1]), &[0, usize::MAX], &[]);
        check(&array(&[], &[0, usize::MAX, 2]), &[0, usize::MAX, 2], &[]);
        check(&array(&[], &[usize::MAX, 0]), &[usize::MAX, 0], &[]);
        check(&array(&[1.0], &[]), &[2, 0, 3], &[]);
    }

    #[test]
    fn nonempty_target_count_overflow_is_rejected() {
        let input = array(&[1.0], &[]);
        let panic =
            std::panic::catch_unwind(|| broadcast_to(&input, &[usize::MAX, 2])).unwrap_err();
        let message = panic
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied());
        assert_eq!(message, Some("broadcast_to target size exceeds usize"));
    }

    #[test]
    fn mismatched_shapes_panic_even_for_empty_targets() {
        for (input, shape) in [
            (array(&[1.0, 2.0], &[2]), vec![3]),
            (array(&[1.0, 2.0], &[1, 2]), vec![2]),
            (array(&[], &[0, 2]), vec![0, 3]),
            (array(&[1.0, 2.0], &[2]), vec![0, 3]),
            (array(&[], &[0, 1]), vec![1, 1]),
            (array(&[], &[0, 2]), vec![0, usize::MAX, 2]),
        ] {
            let panic = std::panic::catch_unwind(|| broadcast_to(&input, &shape)).unwrap_err();
            let message = panic
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic.downcast_ref::<&str>().copied());
            assert_eq!(message, Some("broadcast_to requires compatible shapes"));
        }
    }
}
