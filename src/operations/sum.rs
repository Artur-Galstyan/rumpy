use crate::Array;

pub fn sum(a: &Array) -> f64 {
    if a.shape().contains(&0) {
        return 0.0;
    }
    let mut s = 0.0_f64;
    for v in a.as_slice() {
        s += *v;
    }

    s
}
