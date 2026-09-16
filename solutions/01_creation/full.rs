// Comparison reference for exercise 003. This file is not library code.
use rumpy::Array;

/// Copy a scalar f64 into every position of an owned array.
pub fn full(shape: &[usize], fill_value: f64) -> Array {
    // Reuse the learner's shape and allocation logic, including scalars and empty axes.
    let mut array = rumpy::zeros(shape);
    // Direct copies preserve negative zero and NaN payload bits without arithmetic.
    array.as_mut_slice().fill(fill_value);
    array
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::full;

    fn assert_array(shape: &[usize], fill_value: f64, expected_size: usize) {
        let a = full(shape, fill_value);
        assert_eq!(a.shape(), shape);
        assert_eq!(a.ndim(), shape.len());
        assert_eq!(a.size(), expected_size);
        for actual in a.as_slice() {
            assert_eq!(actual.to_bits(), fill_value.to_bits());
        }
    }

    #[test]
    fn vector_uses_the_parameter() {
        assert_array(&[4], 3.5, 4);
        assert_array(&[4], -2.25, 4);
    }

    #[test]
    fn rectangular_matrix_keeps_axis_order() {
        assert_array(&[2, 3], 7.0, 6);
        assert_array(&[3, 2], 7.0, 6);
    }

    #[test]
    fn higher_rank_and_size_one_axes() {
        assert_array(&[2, 3, 4], -0.5, 24);
        assert_array(&[1, 3, 1], 2.0, 3);
    }

    #[test]
    fn empty_shape_is_one_scalar() {
        assert_array(&[], 8.5, 1);
        assert_array(&[], -0.0, 1);
    }

    #[test]
    fn zero_length_axes_keep_shape() {
        for shape in [vec![0], vec![0, 3], vec![2, 0, 4], vec![2, 3, 0]] {
            assert_array(&shape, 9.0, 0);
            assert_array(&shape, f64::NAN, 0);
        }
    }

    #[test]
    fn both_zero_signs_are_preserved() {
        assert_array(&[3], 0.0, 3);
        assert_array(&[3], -0.0, 3);
    }

    #[test]
    fn infinities_are_copied() {
        assert_array(&[2], f64::INFINITY, 2);
        assert_array(&[2], f64::NEG_INFINITY, 2);
    }

    #[test]
    fn nan_payloads_are_copied() {
        for bits in [
            0x7ff8_0000_0000_0042,
            0xfff8_0000_0000_0011,
            0x7ff0_0000_0000_0001,
        ] {
            assert_array(&[3], f64::from_bits(bits), 3);
        }
    }

    #[test]
    fn finite_extremes_are_copied() {
        assert_array(&[2], f64::MAX, 2);
        assert_array(&[2], f64::MIN, 2);
        assert_array(&[2], f64::MIN_POSITIVE, 2);
        assert_array(&[2], f64::from_bits(1), 2);
    }

    #[test]
    fn calls_own_their_data_and_shape() {
        let mut shape = vec![2];
        let mut a = full(&shape, -3.0);
        let b = full(&shape, 4.0);
        shape[0] = 9;
        a.as_mut_slice()[0] = 99.0;
        assert_eq!(a.shape(), &[2]);
        assert_eq!(a.as_slice(), &[99.0, -3.0]);
        assert_eq!(b.shape(), &[2]);
        assert_eq!(b.as_slice(), &[4.0, 4.0]);
    }

    #[test]
    fn zero_and_one_match_existing_apis() {
        for shape in [vec![], vec![0, 2], vec![2, 3]] {
            assert_eq!(full(&shape, 0.0), rumpy::zeros(&shape));
            assert_eq!(full(&shape, 1.0), rumpy::ones(&shape));
        }
    }
}
