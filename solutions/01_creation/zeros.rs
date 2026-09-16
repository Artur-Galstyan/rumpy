// Reference solution for exercise 001. This is not the library implementation.
use rumpy::Array;

pub fn zeros(shape: &[usize]) -> Array {
    // The empty product is one: [] is a scalar, while a zero axis makes an empty array.
    let count: usize = shape.iter().product();
    // Copy both storage and shape. The result owns them independently of the caller.
    Array::from_vec(vec![0.0; count], shape.to_vec()).expect("consistent shape and data")
}

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
