// Read lessons/017-multiply-broadcast.md. Extend equal-shape multiplication to compatible shapes.
use rumpy::{Array, multiply};
/// Multiply two owned contiguous row-major f64 arrays under right-aligned broadcasting.
pub fn multiply_broadcast(a: &Array, b: &Array) -> Array {
    if a.shape() == b.shape() {
        return multiply(a, b);
    }
    // TODO: Validate aligned axes, handle empty outputs, and select values in row-major order.
    let _ = (a, b);
    todo!("multiply_broadcast")
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::multiply_broadcast;
    use rumpy::Array;
    fn array(data: &[f64], shape: &[usize]) -> Array {
        Array::from_vec(data.to_vec(), shape.to_vec()).unwrap()
    }
    fn check(a: &Array, b: &Array, shape: &[usize], values: &[f64]) {
        let before_a: Vec<u64> = a.as_slice().iter().map(|v| v.to_bits()).collect();
        let before_b: Vec<u64> = b.as_slice().iter().map(|v| v.to_bits()).collect();
        let mut result = multiply_broadcast(a, b);
        assert_eq!(result.shape(), shape);
        assert_eq!(result.size(), values.len());
        for (actual, expected) in result.as_slice().iter().zip(values) {
            if expected.is_nan() {
                assert!(actual.is_nan());
            } else {
                assert_eq!(actual.to_bits(), expected.to_bits());
            }
        }
        assert_eq!(a.shape(), a.shape());
        assert_eq!(
            a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            before_a
        );
        assert_eq!(
            b.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            before_b
        );
        if !values.is_empty() {
            result.as_mut_slice()[0] = 42.0;
            assert_eq!(
                a.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                before_a
            );
            assert_eq!(
                b.as_slice().iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                before_b
            );
        }
    }
    #[test]
    fn equal_shapes_and_scalar() {
        check(
            &array(&[2.0, 3.0], &[2]),
            &array(&[4.0, -5.0], &[2]),
            &[2],
            &[8.0, -15.0],
        );
        check(&array(&[2.0], &[]), &array(&[3.0], &[]), &[], &[6.0]);
    }
    #[test]
    fn trailing_axes_and_singletons() {
        let a = array(&[1.0, 2.0, 3.0], &[3]);
        let b = array(&[10.0, 20.0], &[2, 1]);
        check(&a, &b, &[2, 3], &[10.0, 20.0, 30.0, 20.0, 40.0, 60.0]);
        check(&b, &a, &[2, 3], &[10.0, 20.0, 30.0, 20.0, 40.0, 60.0]);
        check(&array(&[2.0], &[]), &a, &[3], &[2.0, 4.0, 6.0]);
        check(
            &array(&[1.0, 2.0], &[1, 2, 1]),
            &array(&[10.0, 20.0, 30.0], &[3]),
            &[1, 2, 3],
            &[10.0, 20.0, 30.0, 20.0, 40.0, 60.0],
        );
    }
    #[test]
    fn huge_empty_axes_skip_products() {
        check(
            &array(&[], &[usize::MAX, 2, 0]),
            &array(&[2.0], &[]),
            &[usize::MAX, 2, 0],
            &[],
        );
        check(
            &array(&[2.0], &[]),
            &array(&[], &[0, usize::MAX]),
            &[0, usize::MAX],
            &[],
        );
        check(&array(&[], &[0, 1]), &array(&[2.0], &[1]), &[0, 1], &[]);
    }
    #[test]
    fn special_float_products() {
        check(
            &array(&[-0.0, f64::INFINITY, f64::NAN], &[3]),
            &array(&[2.0], &[1]),
            &[3],
            &[-0.0, f64::INFINITY, f64::NAN],
        );
        check(
            &array(&[f64::INFINITY], &[]),
            &array(&[0.0], &[1]),
            &[1],
            &[f64::NAN],
        );
        check(&array(&[-0.0], &[]), &array(&[-2.0], &[1]), &[1], &[0.0]);
    }
    #[test]
    fn incompatible_axes_rejected_before_empty_shortcut() {
        for (a, b) in [
            (array(&[1.0, 2.0], &[2]), array(&[1.0, 2.0, 3.0], &[3])),
            (array(&[], &[0, 2]), array(&[], &[0, 3])),
            (array(&[], &[0, 2]), array(&[], &[0, usize::MAX, 3])),
        ] {
            let failure = std::panic::catch_unwind(|| multiply_broadcast(&a, &b)).unwrap_err();
            let text = failure
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| failure.downcast_ref::<&str>().copied());
            assert_eq!(text, Some("multiply_broadcast requires compatible shapes"));
        }
    }
}
