use crate::Array;

/// Reduce all ranks to a flat row-major usize index without changing the input.
/// Return the first NaN index, or the first maximum index when no NaN exists.
/// Both zero signs tie. Empty input panics with "argmax requires at least one element".
pub fn argmax(a: &Array) -> usize {
    if a.size() == 0 {
        panic!("argmax requires at least one element")
    }

    let mut res = (0, f64::NEG_INFINITY);

    for (i, v) in a.as_slice().iter().enumerate() {
        if v.is_nan() {
            return i;
        }
        if *v > res.1 {
            res.0 = i;
            res.1 = *v;
        }
    }
    res.0
}
