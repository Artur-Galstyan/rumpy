use crate::Array;

pub fn broadcast_to(input: &Array, shape: &[usize]) -> Array {
    if input.shape().len() > shape.len() {
        panic!("broadcast_to requires compatible shapes")
    }
    let mut input_shape = input.shape().to_vec();
    let to_pad = shape.len() - input.shape().len();
    (0..to_pad).for_each(|i| input_shape.insert(i, 1));

    assert!(
        input_shape.len() == shape.len(),
        "input_shape.len() after padding != target shape.len()"
    );

    let output_shape: Vec<usize> = input_shape
        .iter()
        .zip(shape)
        .map(|shapes| {
            let (i, t) = shapes;
            if i != t && *i != 1 {
                panic!("broadcast_to requires compatible shapes")
            }
            *t
        })
        .collect();
    let mut total: usize = 1;
    for &dim in shape {
        total = total
            .checked_mul(dim)
            .expect("broadcast_to target size exceeds usize");
    }
    let mut data: Vec<f64> = Vec::new();
    for i in 0..total {
        let mut temp = i;
        let mut coords = vec![0; shape.len()];

        for axis in (0..shape.len()).rev() {
            coords[axis] = temp % shape[axis];
            temp /= shape[axis];
        }

        let mut input_coords = vec![0; shape.len()];
        for axis in 0..input_shape.len() {
            if input_shape[axis] != 1 {
                input_coords[axis] = coords[axis];
            }
        }

        let mut in_idx = 0;
        for axis in 0..input_shape.len() {
            in_idx = in_idx * input_shape[axis] + input_coords[axis];
        }
        data.push(input.as_slice()[in_idx]);
    }

    Array::from_vec(data, output_shape.to_vec()).expect("invalid args")
}
