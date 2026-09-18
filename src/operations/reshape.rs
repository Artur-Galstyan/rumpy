use crate::{Array, ShapeError};

/// Copy an array into a new explicit shape without changing its flat value order.
/// Return the scaffold's ShapeError when the requested shape is incompatible.
/// The input remains unchanged. The output owns independent data and shape.
pub fn reshape(a: &Array, shape: &[usize]) -> Result<Array, ShapeError> {
    Array::from_vec(a.as_slice().to_vec(), shape.to_vec())
}
