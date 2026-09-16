// Reference solution for exercise 002. Compare after your own attempt.
use rumpy::Array;

pub fn ones(shape: &[usize]) -> Array {
    // Reuse the learner's shape/allocation behaviour, including scalar and empty cases.
    let mut array = rumpy::zeros(shape);
    // Change values in place. This preserves the shape and avoids another allocation.
    array.as_mut_slice().fill(1.0);
    array
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::ones;

    #[test]
    fn vector_has_requested_length() {
        let a = ones(&[4]);
        assert_eq!(a.shape(), &[4]);
        assert_eq!(a.as_slice(), &[1.0; 4]);
    }
    #[test]
    fn rectangular_matrix_preserves_axis_order() {
        let a = ones(&[2, 3]);
        assert_eq!(a.shape(), &[2, 3], "shape means rows, then columns");
        assert_eq!(a.as_slice(), &[1.0; 6], "allocate one value per coordinate");
    }
    #[test]
    fn supports_more_than_two_axes() {
        let a = ones(&[2, 3, 4]);
        assert_eq!(a.shape(), &[2, 3, 4]);
        assert_eq!(a.ndim(), 3);
        assert_eq!(a.size(), 24);
        assert!(
            a.as_slice()
                .iter()
                .all(|x| x.to_bits() == 1.0_f64.to_bits())
        );
    }
    #[test]
    fn zero_length_axis_keeps_shape_but_has_no_values() {
        for shape in [vec![0], vec![0, 3], vec![2, 0, 4]] {
            let a = ones(&shape);
            assert_eq!(a.shape(), shape);
            assert!(a.as_slice().is_empty());
        }
    }
    #[test]
    fn empty_shape_is_a_scalar_not_an_empty_vector() {
        let a = ones(&[]);
        assert_eq!(a.ndim(), 0);
        assert_eq!(a.shape(), &[] as &[usize]);
        assert_eq!(a.as_slice(), &[1.0], "a scalar still holds one value");
    }
    #[test]
    fn size_one_axes_are_not_squeezed() {
        let a = ones(&[1, 3, 1]);
        assert_eq!(a.shape(), &[1, 3, 1]);
        assert_eq!(a.size(), 3);
    }
    #[test]
    fn arrays_own_their_data_and_shape() {
        let mut shape = vec![2];
        let mut a = ones(&shape);
        let b = ones(&shape);
        shape[0] = 9;
        a.as_mut_slice()[0] = 7.0;
        assert_eq!(a.shape(), &[2]);
        assert_eq!(b.as_slice(), &[1.0, 1.0]);
    }
}
