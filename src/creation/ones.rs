use crate::{Array, full};

pub fn ones(shape: &[usize]) -> Array {
    full(shape, 1.0)
}
