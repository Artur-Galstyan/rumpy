use crate::Array;

pub fn amax(a: &Array) -> f64 {
    if a.size() == 0 {
        panic!("amax requires at least one element");
    }

    let mut max = f64::NEG_INFINITY;
    for &v in a.as_slice() {
        if v.is_nan() {
            return f64::NAN;
        }
        if v > max {
            max = v;
        }
    }
    max
}
