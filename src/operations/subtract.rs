use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::Array;

/// Subtract corresponding f64 values into a new owned Array.
/// panics if the shapes are not equal.
pub fn subtract(a: &Array, b: &Array) -> Array {
    if a.shape() != b.shape() {
        panic!("subtract requires identical shapes")
    }
    if a.size() > 1_000_000 {
        let data = a
            .as_slice()
            .par_iter()
            .zip(b.as_slice())
            .map(|(a, b)| a - b)
            .collect::<Vec<f64>>();
        Array::from_vec(data, a.shape().to_vec()).expect("invalid args")
    } else {
        let data = a
            .as_slice()
            .iter()
            .zip(b.as_slice())
            .map(|(a, b)| a - b)
            .collect::<Vec<f64>>();
        Array::from_vec(data, a.shape().to_vec()).expect("invalid args")
    }
}
