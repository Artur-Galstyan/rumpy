use crate::Array;

/// Return an owned f64 array filled with 1.0, preserving the requested shape.
/// Scope: the same small, valid shapes supported by crate::zeros.
/// Graduated from learner commit ca1d4b1f29d700a77f34c8bad8f5363bdddd3675.
pub fn ones(shape: &[usize]) -> Array {
    // TODO: Reuse rumpy::zeros, change each stored value to one, and return the array.
    let _ = shape;
    let mut a = crate::zeros(shape);
    a.as_mut_slice().fill(1.0);
    a
}
