use crate::Array;

/// Add corresponding f64 values into a new Array with the same shape.
/// Unequal shapes panic with "add requires identical shapes"; no broadcasting.
pub fn add(a: &Array, b: &Array) -> Array {
    if a.shape() != b.shape() {
        panic!("add requires identical shapes")
    }

    let data: Vec<f64> = a
        .as_slice()
        .iter()
        .zip(b.as_slice())
        .map(|x| x.0 + x.1)
        .collect();

    Array::from_vec(data, a.shape().to_vec()).expect("invalid args")
}
