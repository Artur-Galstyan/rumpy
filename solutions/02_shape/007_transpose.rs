// Exercise 007: numpy.transpose, two axes and an owned row-major copy.
// Read lessons/007-transpose.md. Keep the tests intact.
use rumpy::Array;

/// Swap the two axes and return independently owned, row-major data.
/// Preserve every f64 bit and leave the input unchanged.
/// Panic with "transpose requires a two-axis array" for any other rank.
/// Empty axes, including usize::MAX paired with zero, must finish promptly.
pub fn transpose(a: &Array) -> Array {
    // TODO (historical, completed): Transpose without arithmetic on values.
    assert_eq!(a.ndim(), 2, "transpose requires a two-axis array");
    let rows = a.shape()[0];
    let columns = a.shape()[1];
    let shape = vec![columns, rows];
    // Avoid even an outer-axis loop when an enormous axis is paired with zero.
    if a.size() == 0 {
        return Array::from_vec(Vec::new(), shape).unwrap();
    }
    let mut data = Vec::with_capacity(a.size());
    // Each input column becomes a contiguous output row. Copies preserve bits.
    for column in 0..columns {
        for row in 0..rows {
            data.push(a.as_slice()[row * columns + column]);
        }
    }
    // Swapping axes preserves the validated element count.
    Array::from_vec(data, shape).unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::transpose;
    use rumpy::Array;

    fn array(data: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(data.to_vec(), shape.to_vec()).unwrap()
    }

    fn assert_bits(a: &Array, shape: &[usize], values: &[f64]) {
        assert_eq!(a.shape(), shape);
        assert_eq!(a.ndim(), 2);
        assert_eq!(a.size(), values.len());
        assert_eq!(a.as_slice().len(), values.len());
        for (actual, expected) in a.as_slice().iter().zip(values) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn wide_rectangle_reorders_flat_values() {
        let values = [11.0, 12.0, 13.0, 21.0, 22.0, 23.0];
        let a = array(&values, &[2, 3]);
        assert_bits(
            &transpose(&a),
            &[3, 2],
            &[11.0, 21.0, 12.0, 22.0, 13.0, 23.0],
        );
        assert_bits(&a, &[2, 3], &values);
    }

    #[test]
    fn tall_rectangle_reorders_flat_values() {
        let a = array(&[11.0, 12.0, 21.0, 22.0, 31.0, 32.0], &[3, 2]);
        assert_bits(
            &transpose(&a),
            &[2, 3],
            &[11.0, 21.0, 31.0, 12.0, 22.0, 32.0],
        );
    }

    #[test]
    fn nonsymmetric_square_changes_values_not_shape() {
        let a = array(&[1.0, 2.0, 3.0, 4.0], &[2, 2]);
        assert_bits(&transpose(&a), &[2, 2], &[1.0, 3.0, 2.0, 4.0]);
    }

    #[test]
    fn singleton_axes_still_swap() {
        let values = [-7.0, 8.0, 9.0];
        for shape in [[1, 3], [3, 1]] {
            assert_bits(
                &transpose(&array(&values, &shape)),
                &[shape[1], shape[0]],
                &values,
            );
        }
        assert_bits(&transpose(&array(&[-0.0], &[1, 1])), &[1, 1], &[-0.0]);
    }

    #[test]
    fn empty_axes_keep_their_lengths() {
        for shape in [[0, 0], [0, 1], [1, 0], [0, 7], [9, 0]] {
            let a = array(&[], &shape);
            assert_bits(&transpose(&a), &[shape[1], shape[0]], &[]);
            assert_bits(&a, &shape, &[]);
        }
    }

    #[test]
    fn huge_empty_axes_do_not_require_axis_length_iterations() {
        // A zero-sized array must not loop usize::MAX times over its other axis.
        for shape in [[usize::MAX, 0], [0, usize::MAX]] {
            let a = array(&[], &shape);
            assert_bits(&transpose(&a), &[shape[1], shape[0]], &[]);
        }
    }

    #[test]
    fn special_float_bits_move_without_arithmetic() {
        let values = [
            0.0,
            -0.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(0x7ff8_0000_0000_0042),
            f64::from_bits(0xfff8_0000_0000_1234),
            f64::from_bits(0x7ff0_0000_0000_0001),
            f64::from_bits(0xfff0_0000_0000_0042),
            f64::from_bits(1),
            f64::from_bits(0x8000_0000_0000_0001),
        ];
        let a = array(&values, &[2, 5]);
        let expected = [
            values[0], values[5], values[1], values[6], values[2], values[7], values[3], values[8],
            values[4], values[9],
        ];
        assert_bits(&transpose(&a), &[5, 2], &expected);
        assert_bits(&a, &[2, 5], &values);
    }

    #[test]
    fn input_and_each_output_have_independent_storage() {
        let mut a = array(&[1.0, 2.0, 3.0, 4.0], &[2, 2]);
        let mut b = transpose(&a);
        let c = transpose(&a);
        assert_ne!(a.as_slice().as_ptr(), b.as_slice().as_ptr());
        assert_ne!(a.shape().as_ptr(), b.shape().as_ptr());
        b.as_mut_slice()[1] = -100.0;
        a.as_mut_slice()[1] = -200.0;
        assert_bits(&a, &[2, 2], &[1.0, -200.0, 3.0, 4.0]);
        assert_bits(&b, &[2, 2], &[1.0, -100.0, 2.0, 4.0]);
        assert_bits(&c, &[2, 2], &[1.0, 3.0, 2.0, 4.0]);
    }

    #[test]
    fn double_transpose_restores_shape_and_all_bits() {
        for shape in [[2, 3], [3, 2], [1, 6], [6, 1], [0, 4], [usize::MAX, 0]] {
            let values = if shape.contains(&0) {
                vec![]
            } else {
                vec![
                    -0.0,
                    f64::from_bits(0x7ff0_0000_0000_0001),
                    2.0,
                    -3.0,
                    f64::INFINITY,
                    f64::from_bits(1),
                ]
            };
            let a = array(&values, &shape);
            assert_bits(&transpose(&transpose(&a)), &shape, &values);
        }
    }

    #[test]
    #[should_panic(expected = "transpose requires a two-axis array")]
    fn scalar_rank_panics() {
        transpose(&array(&[1.0], &[]));
    }

    #[test]
    #[should_panic(expected = "transpose requires a two-axis array")]
    fn vector_rank_panics() {
        transpose(&array(&[1.0, 2.0], &[2]));
    }

    #[test]
    #[should_panic(expected = "transpose requires a two-axis array")]
    fn empty_vector_rank_panics_before_empty_shortcut() {
        transpose(&array(&[], &[0]));
    }

    #[test]
    #[should_panic(expected = "transpose requires a two-axis array")]
    fn three_axis_rank_panics() {
        transpose(&array(&[1.0, 2.0], &[1, 2, 1]));
    }

    #[test]
    #[should_panic(expected = "transpose requires a two-axis array")]
    fn empty_higher_rank_panics_before_empty_shortcut() {
        transpose(&array(&[], &[0, 2, 1, 1]));
    }
}
