use crate::{Array, sum};

pub fn mean(a: &Array) -> f64 {
    if a.size() == 0 {
        return f64::NAN;
    }
    sum(a) / a.size() as f64
}
