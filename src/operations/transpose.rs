use crate::Array;

pub fn transpose(a: &Array) -> Array {
    if a.shape().len() != 2 {
        panic!("transpose requires a two-axis array")
    } else if a.size() == 0 || a.shape().iter().product::<usize>() == 0 {
        let rows = a.shape()[0];
        let cols = a.shape()[1];
        return Array::from_vec(a.as_slice().to_vec(), vec![cols, rows]).expect("invalid args");
    }
    let mut data: Vec<f64> = Vec::with_capacity(a.size());
    let rows = a.shape()[0];
    let cols = a.shape()[1];
    for col in 0..cols {
        for row in 0..rows {
            let index = row * cols + col;
            data.push(a.as_slice()[index]);
        }
    }

    Array::from_vec(data.to_vec(), vec![cols, rows]).expect("invalid args")
}
