use crate::Array;

/// Return an owned row-major f64 matrix with shape [rows, cols].
/// Put 1.0 where row == column and positive 0.0 elsewhere.
/// Scope: rows and cols are each at most 1,000. Zero-length axes are valid.
pub fn eye(rows: usize, cols: usize) -> Array {
    if rows == 0 || cols == 0 {
        return Array::from_vec(vec![], vec![rows, cols]).expect("valid empty array");
    }

    let total = rows * cols;
    let mut data = vec![0.0; total];

    (0..rows.min(cols)).for_each(|i| {
        data[i * cols + i] = 1.0;
    });

    Array::from_vec(data, vec![rows, cols]).expect("invalid arguments")
}
