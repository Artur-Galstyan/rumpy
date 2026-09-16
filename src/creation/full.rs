use crate::Array;

/// Return an owned array with the exact shape and copies of fill_value.
/// Scope: small valid shapes, scalar f64 fill, contiguous row-major storage.
pub fn full(shape: &[usize], fill_value: f64) -> Array {
    let _ = shape;
    let mut a = crate::zeros(shape);
    a.as_mut_slice().fill(fill_value);
    a
}
