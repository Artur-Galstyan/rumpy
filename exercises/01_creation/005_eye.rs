// Exercise 005: numpy.eye(rows, cols), main diagonal only.
// Read lessons/005-eye.md. Keep the tests intact.
use rumpy::Array;

/// Return an owned row-major f64 matrix with shape [rows, cols].
/// Put 1.0 where row == column and positive 0.0 elsewhere.
/// Scope: rows and cols are each at most 1,000. Zero-length axes are valid.
pub fn eye(rows: usize, cols: usize) -> Array {
    // TODO: Build a rectangular matrix with ones on its main diagonal.
    let _ = (rows, cols);
    todo!("Build eye with an explicit two-axis shape")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::eye;

    fn assert_matrix(rows: usize, cols: usize, expected: &[f64]) {
        let a = eye(rows, cols);
        assert_eq!(a.shape(), &[rows, cols]);
        assert_eq!(a.ndim(), 2);
        assert_eq!(a.size(), rows * cols);
        assert_eq!(a.as_slice().len(), expected.len());
        for (actual, expected) in a.as_slice().iter().zip(expected) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn square_matrix_has_only_diagonal_ones() {
        assert_matrix(3, 3, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn wide_matrix_uses_column_count_as_row_width() {
        assert_matrix(2, 4, &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn tall_matrix_stops_at_last_column() {
        assert_matrix(4, 2, &[1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn one_by_one_is_a_matrix_not_a_scalar() {
        assert_matrix(1, 1, &[1.0]);
    }

    #[test]
    fn single_row_and_column_preserve_both_axes() {
        assert_matrix(1, 4, &[1.0, 0.0, 0.0, 0.0]);
        assert_matrix(4, 1, &[1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn empty_axes_preserve_the_exact_shape() {
        assert_matrix(0, 0, &[]);
        assert_matrix(0, 5, &[]);
        assert_matrix(5, 0, &[]);
        assert_matrix(0, 1000, &[]);
        assert_matrix(1000, 0, &[]);
    }

    #[test]
    fn larger_rectangle_has_positive_zero_off_the_diagonal() {
        let a = eye(7, 11);
        assert_eq!(a.shape(), &[7, 11]);
        assert_eq!(a.size(), 77);
        for row in 0..7 {
            for col in 0..11 {
                let expected: f64 = if row == col { 1.0 } else { 0.0 };
                assert_eq!(a.as_slice()[row * 11 + col].to_bits(), expected.to_bits());
            }
        }
    }

    #[test]
    fn supported_dimension_boundary() {
        let a = eye(1000, 1000);
        assert_eq!(a.shape(), &[1000, 1000]);
        assert_eq!(a.size(), 1_000_000);
        for (index, value) in a.as_slice().iter().enumerate() {
            let expected: f64 = if index / 1000 == index % 1000 {
                1.0
            } else {
                0.0
            };
            assert_eq!(value.to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn calls_own_independent_storage() {
        let mut a = eye(2, 3);
        let b = eye(2, 3);
        a.as_mut_slice()[0] = -4.0;
        a.as_mut_slice()[1] = 9.0;
        assert_eq!(b.as_slice(), &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert_eq!(a.shape(), &[2, 3]);
        assert_eq!(b.shape(), &[2, 3]);
    }
}
