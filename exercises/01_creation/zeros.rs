// Exercise 001: numpy.zeros, in Rust.
// Read lessons/001-zeros.md before editing this file.
// Implement ONLY zeros. Keep the tests intact. Ask Rustlings for a hint with `h`.
// Your function is also exported from the library as rumpy::zeros.

use rumpy::Array;

/// Return an owned f64 array filled with positive zero, with exactly this shape.
/// Scope: small shapes whose element count fits usize and whose data fits memory.
/// No dtype/order/like/device parameters yet. See the lesson for scalar/empty cases.
pub fn zeros(shape: &[usize]) -> Array {
    // TODO: Compute the element count, create the zero-filled data, and preserve the shape.
    let _ = shape;
    todo!("Implement zeros: preserve the shape and create the right number of zeros")
}

#[allow(dead_code)]
fn main() {}

#[cfg(test)]
mod tests {
    use super::zeros;

    #[test]
    fn vector_has_requested_length() {
        let a = zeros(&[4]);
        assert_eq!(a.shape(), &[4]);
        assert_eq!(a.as_slice(), &[0.0; 4]);
    }
    #[test]
    fn rectangular_matrix_preserves_axis_order() {
        let a = zeros(&[2, 3]);
        assert_eq!(a.shape(), &[2, 3], "shape means rows, then columns");
        assert_eq!(a.as_slice(), &[0.0; 6], "allocate one value per coordinate");
    }
    #[test]
    fn supports_more_than_two_axes() {
        let a = zeros(&[2, 3, 4]);
        assert_eq!(a.shape(), &[2, 3, 4]);
        assert_eq!(a.ndim(), 3);
        assert_eq!(a.size(), 24);
        assert!(
            a.as_slice()
                .iter()
                .all(|x| x.to_bits() == 0.0_f64.to_bits())
        );
    }
    #[test]
    fn zero_length_axis_keeps_shape_but_has_no_values() {
        for shape in [vec![0], vec![0, 3], vec![2, 0, 4]] {
            let a = zeros(&shape);
            assert_eq!(a.shape(), shape);
            assert!(a.as_slice().is_empty());
        }
    }
    #[test]
    fn empty_shape_is_a_scalar_not_an_empty_vector() {
        let a = zeros(&[]);
        assert_eq!(a.ndim(), 0);
        assert_eq!(a.shape(), &[] as &[usize]);
        assert_eq!(a.as_slice(), &[0.0], "a scalar still holds one value");
    }
    #[test]
    fn size_one_axes_are_not_squeezed() {
        let a = zeros(&[1, 3, 1]);
        assert_eq!(a.shape(), &[1, 3, 1]);
        assert_eq!(a.size(), 3);
    }
    #[test]
    fn arrays_own_their_data_and_shape() {
        let mut shape = vec![2];
        let mut a = zeros(&shape);
        let b = zeros(&shape);
        shape[0] = 9;
        a.as_mut_slice()[0] = 7.0;
        assert_eq!(a.shape(), &[2]);
        assert_eq!(b.as_slice(), &[0.0, 0.0]);
    }
}
