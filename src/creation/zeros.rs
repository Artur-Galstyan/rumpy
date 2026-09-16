use crate::Array;

/// Return an owned f64 array of positive zeros with the requested shape.
///
/// Supports small shapes whose intermediate products fit usize and data fits memory.
/// An empty shape denotes one scalar; a zero-length axis denotes no values.
/// Graduated from the learner's first exercise without changing its implementation.
pub fn zeros(shape: &[usize]) -> Array {
    let count: usize = shape.iter().product();
    let data = vec![0.0; count];
    Array::from_vec(data, shape.to_vec()).expect("Invalid shape")
}
