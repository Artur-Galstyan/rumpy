use crate::Array;

pub fn arange(start: i32, stop: i32, step: i32) -> Array {
    if step <= 0 {
        panic!("step must be positive")
    }
    if start >= stop {
        return Array::from_vec(vec![], vec![0]).expect("valid empty array");
    }
    let diff = (stop as i64) - (start as i64);
    let step = step as u64;
    let capacity = (diff as u64).div_ceil(step) as usize;
    let mut data: Vec<f64> = Vec::with_capacity(capacity);

    for x in (start as i64..stop as i64).step_by(step as usize) {
        data.push(x as f64);
    }

    Array::from_vec(data, vec![capacity]).expect("invalid arguments")
}
