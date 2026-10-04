// Read lessons/016-add-broadcast.md. Extend your equal-shape addition to compatible shapes.
use rumpy::{Array, add, broadcast_to};

/// Add two owned row-major f64 arrays with right-aligned broadcast shapes.
/// This separate extension does not change the existing rumpy::add API.
pub fn add_broadcast(a: &Array, b: &Array) -> Array {
    if a.shape() == b.shape() {
        return add(a, b);
    }
    
    let mut a_shape = a.shape().to_vec();
    let mut b_shape = b.shape().to_vec();

    if a.shape().len() > b.shape().len() {
        while b_shape.len() < a_shape.len() {
            b_shape.insert(0, 1);
        }
    } else {
        while a_shape.len() < b_shape.len() {
            a_shape.insert(0, 1);
        }
    } 

    let target_shape: Vec<usize> = a_shape.iter().zip(b_shape).map(|p| {
        let (dim_a, dim_b) = p;
        
        if *dim_a != dim_b && (*dim_a != 1 && dim_b != 1) {
            panic!("add_broadcast requires compatible shapes")
        }

        if *dim_a == dim_b {
            *dim_a
        } else {
            if *dim_a == 1 {
                dim_b
            } else {
                *dim_a
            }
        }
    }).collect();

    let target_shape = target_shape.as_slice();
    if target_shape.contains(&0) {
        return Array::from_vec(Vec::new(), target_shape.to_vec())
            .expect("empty target shape must have zero elements");
    }

    let broadcasted_a = broadcast_to(a, target_shape);
    let broadcasted_b = broadcast_to(b, target_shape);

    add(&broadcasted_a, &broadcasted_b)
    
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
