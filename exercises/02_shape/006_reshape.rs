// Exercise 006: numpy.reshape, explicit shapes and row-major order.
// Read lessons/006-reshape.md. Keep the tests intact.
use rumpy::{Array, ShapeError};

/// Copy an array into a new explicit shape without changing its flat value order.
/// Return the scaffold's ShapeError when the requested shape is incompatible.
/// The input remains unchanged. The output owns independent data and shape.
pub fn reshape(a: &Array, shape: &[usize]) -> Result<Array, ShapeError> {
    // TODO: Preserve the flat values and validate the requested shape.
    Array::from_vec(a.as_slice().to_vec(), shape.to_vec())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::reshape;
    use rumpy::{Array, ShapeError};

    fn array(data: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(data.to_vec(), shape.to_vec()).unwrap()
    }

    fn assert_bits(a: &Array, shape: &[usize], values: &[f64]) {
        assert_eq!(a.shape(), shape);
        assert_eq!(a.ndim(), shape.len());
        assert_eq!(a.size(), values.len());
        assert_eq!(a.as_slice().len(), values.len());
        for (actual, expected) in a.as_slice().iter().zip(values) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn vector_becomes_a_matrix_in_row_major_order() {
        let values = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let a = array(&values, &[6]);
        assert_bits(&reshape(&a, &[2, 3]).unwrap(), &[2, 3], &values);
        assert_bits(&a, &[6], &values);
    }

    #[test]
    fn matrix_changes_shape_without_transpose() {
        let values = [11.0, 12.0, 13.0, 21.0, 22.0, 23.0];
        let a = array(&values, &[2, 3]);
        assert_bits(&reshape(&a, &[3, 2]).unwrap(), &[3, 2], &values);
    }

    #[test]
    fn higher_rank_can_flatten_or_keep_size_one_axes() {
        let values = [2.0, -4.0, 6.0, -8.0, 10.0, -12.0];
        let a = array(&values, &[1, 2, 1, 3]);
        assert_bits(&reshape(&a, &[6]).unwrap(), &[6], &values);
        assert_bits(&reshape(&a, &[3, 1, 2, 1]).unwrap(), &[3, 1, 2, 1], &values);
    }

    #[test]
    fn scalar_and_single_element_axes_are_interchangeable() {
        let a = array(&[-7.5], &[]);
        let b = reshape(&a, &[1, 1, 1]).unwrap();
        assert_bits(&b, &[1, 1, 1], &[-7.5]);
        assert_bits(&reshape(&b, &[]).unwrap(), &[], &[-7.5]);
    }

    #[test]
    fn empty_arrays_keep_the_requested_axes() {
        let a = array(&[], &[2, 0, 3]);
        for shape in [&[0][..], &[0, 7], &[4, 0, 2], &[0, 0]] {
            assert_bits(&reshape(&a, shape).unwrap(), shape, &[]);
        }
    }

    #[test]
    fn incompatible_size_returns_exact_error() {
        let a = array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
        assert_eq!(
            reshape(&a, &[2, 2]),
            Err(ShapeError::LengthMismatch {
                expected: 4,
                actual: 6
            })
        );
        assert_eq!(
            reshape(&a, &[0]),
            Err(ShapeError::LengthMismatch {
                expected: 0,
                actual: 6
            })
        );
        assert_bits(&a, &[2, 3], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn empty_shape_requires_one_value_not_zero() {
        let a = array(&[], &[0]);
        assert_eq!(
            reshape(&a, &[]),
            Err(ShapeError::LengthMismatch {
                expected: 1,
                actual: 0
            })
        );
    }

    #[test]
    fn scalar_cannot_become_an_empty_array() {
        let a = array(&[3.0], &[]);
        assert_eq!(
            reshape(&a, &[2, 0]),
            Err(ShapeError::LengthMismatch {
                expected: 0,
                actual: 1
            })
        );
    }

    #[test]
    fn overflowing_nonempty_shape_returns_error() {
        let a = array(&[1.0], &[1]);
        assert_eq!(reshape(&a, &[usize::MAX, 2]), Err(ShapeError::SizeOverflow));
    }

    #[test]
    fn zero_axis_takes_precedence_over_overflow() {
        let a = array(&[], &[0]);
        for shape in [
            &[usize::MAX, 2, 0][..],
            &[0, usize::MAX, 2],
            &[usize::MAX, 0, 2],
        ] {
            assert_bits(&reshape(&a, shape).unwrap(), shape, &[]);
        }
    }

    #[test]
    fn every_float_bit_is_copied_without_arithmetic() {
        let values = [
            0.0,
            -0.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(0x7ff8_0000_0000_0042),
            f64::from_bits(0xfff8_0000_0000_1234),
            f64::from_bits(0x7ff0_0000_0000_0001),
            f64::from_bits(1),
        ];
        let a = array(&values, &[8]);
        assert_bits(&reshape(&a, &[2, 4]).unwrap(), &[2, 4], &values);
    }

    #[test]
    fn output_does_not_share_data_or_shape() {
        let values = [1.0, 2.0, 3.0, 4.0];
        let mut a = array(&values, &[4]);
        let mut shape = vec![2, 2];
        let mut b = reshape(&a, &shape).unwrap();
        let c = reshape(&a, &shape).unwrap();
        shape[0] = 99;
        b.as_mut_slice()[0] = -100.0;
        a.as_mut_slice()[1] = -200.0;
        assert_bits(&b, &[2, 2], &[-100.0, 2.0, 3.0, 4.0]);
        assert_bits(&a, &[4], &[1.0, -200.0, 3.0, 4.0]);
        assert_bits(&c, &[2, 2], &values);
    }

    #[test]
    fn same_shape_still_makes_an_independent_copy() {
        let mut a = array(&[9.0, 8.0], &[1, 2]);
        let mut b = reshape(&a, &[1, 2]).unwrap();
        b.as_mut_slice()[0] = 0.0;
        a.as_mut_slice()[1] = 0.0;
        assert_bits(&a, &[1, 2], &[9.0, 0.0]);
        assert_bits(&b, &[1, 2], &[0.0, 8.0]);
    }
}
