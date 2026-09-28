use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::Array;

pub fn multiply(a: &Array, b: &Array) -> Array {
    if a.shape() != b.shape() {
        panic!("multiply requires identical shapes")
    }
    if a.size() > 1_000_000 {
        let data: Vec<f64> = a
            .as_slice()
            .par_iter()
            .zip(b.as_slice())
            .map(|x| x.0 * x.1)
            .collect();
        Array::from_vec(data, a.shape().to_vec()).expect("invalid args")
    } else {
        let data: Vec<f64> = a
            .as_slice()
            .iter()
            .zip(b.as_slice())
            .map(|x| x.0 * x.1)
            .collect();
        Array::from_vec(data, a.shape().to_vec()).expect("invalid args")
    }
}
