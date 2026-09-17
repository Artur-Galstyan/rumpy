# 006 — reshape

Write `reshape` in `exercises/02_shape/006_reshape.rs`.

Your public `eye` passes its original tests. This lesson starts the shape operations.
A reshape gives existing values a new shape. It does not transpose, sort, or change the values.
This operation prepares arrays for later reductions and matrix operations.

## Public contract

```rust
pub fn reshape(a: &Array, shape: &[usize]) -> Result<Array, ShapeError>
```

The function borrows an existing `Array` and an explicit target shape.
On success, it returns an owned, contiguous, row-major `f64` array.
The result contains the same flat values in the same order, with the requested shape.
The result owns independent data and shape, even when the requested shape matches the input.
The input remains unchanged after success or failure.

Original API: [numpy.reshape](https://numpy.org/doc/stable/reference/generated/numpy.reshape.html).
The matching NumPy call is `np.reshape(a, shape, order="C", copy=True)`.
NumPy can also return a view. This exercise always copies because `Array` owns its storage.
That choice costs a data copy but avoids shared-storage lifetime rules at this stage.

## Supported shapes

- Every target dimension is an explicit `usize`. The target can have a different rank from the input.
- The target element count must equal `a.size()`. Keep every axis, including axes of length one.
- An empty target shape `[]` denotes one scalar. It requires exactly one value.
- Any zero-length target axis denotes an empty array. Preserve all target axes.
- A zero-length axis takes precedence over shape-product overflow, as in `Array::from_vec`.

There is no axis parameter. The whole array receives a new shape.
Inputs contain owned contiguous `f64` storage. Views, strides, other dtypes, and nested input lists are outside this lesson.
Negative dimensions, inferred `-1`, Fortran order, in-place changes, and NumPy's optional copy policies are also outside this lesson.
Memory-allocation failure is outside the contract. The tests use small data buffers.

## Errors

The existing scaffold defines the error contract. Do not add a new error type.

- If a nonempty shape product overflows `usize`, return `Err(ShapeError::SizeOverflow)`.
- If the counts differ, return `Err(ShapeError::LengthMismatch { expected, actual })`.
- `expected` is the target element count. `actual` is the input data length.
- If the shape contains zero, the target count is zero, even if another partial product could overflow.
- A shape error must not panic or change the input.

For example, six input values cannot fit shape `[2, 2]`.
That call returns `LengthMismatch { expected: 4, actual: 6 }`.
An empty input cannot become a scalar. Shape `[]` requires one value, not zero.

## Shape examples

The examples show flat data and its row layout. All outputs keep the original flat sequence.

```text
Input shape: [6]
Input data:  [0, 1, 2, 3, 4, 5]

Target shape: [2, 3]
Output rows:
[0, 1, 2]
[3, 4, 5]

Target shape: [3, 2]
Output rows:
[0, 1]
[2, 3]
[4, 5]

Flat output in both cases: [0, 1, 2, 3, 4, 5]
```

The second output is not the transpose of the first output.
A transpose changes which value occupies each flat position. Reshape does not.

```text
Input shape: []       Input data: [-7.5]
Target shape: [1, 1]  Output data: [-7.5]

Input shape: [2, 0, 3]  Input data: []
Target shape: [0, 7]    Output data: []
```

Every floating-point bit remains unchanged. This includes negative zero, infinities, and NaN payloads.
Copy values rather than apply arithmetic to them.

## Allowed helpers

1. Inspect `Array::as_slice` and `Array::from_vec` in `src/array.rs`.
2. Inspect `ShapeError` in that file.
3. Use standard slice and `Vec` methods for owned copies.
4. Reuse the scaffold's validation rather than duplicate it.
5. Keep the errors in the returned `Result`.

The code can be short. The exercise concerns ownership, shape semantics, and error reuse, not a new indexing algorithm.
Do not change `src/array.rs`, dependencies, or earlier runners to solve it.

## Checks

```sh
rustlings hint 006_reshape
cargo test --bin 006_reshape
rustlings run 006_reshape
```

The new stub compiles. Its tests fail at the intended `todo!` until you complete it.
The tests cover ordinary shapes, scalar and empty shapes, exact errors, value bits, and independent storage.
A separate reference appears at `solutions/02_shape/006_reshape.rs` with identical contract tests.

Rustlings watches the current exercise only. After a dependency change, press `c` for a full check.
Alternatively, use `rustlings --manual-run` and press `r`.

## Library checklist

You own the library changes after the exercise passes. The author does not make these changes for you.

1. Choose a module layout for shape operations.
2. Move your function into `src/` without importing the reference.
3. Expose the unnumbered `reshape` API from the library.
4. Connect the preserved exercise tests to that API, with a historical `// TODO` marker.
5. Refactor shared behaviour without dependency cycles, then run the checks below.

`Array::from_vec` is the low-level constructor. If `reshape` calls it, do not make that constructor call `reshape` in return.
The current creation helpers can stay as they are. No caller migration is required for this new operation.

```sh
cargo test --lib --test scaffold
cargo test --bin 001_zeros --bin 002_ones --bin 003_full --bin 004_arange --bin 005_eye --bin 006_reshape
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

## Earlier runner markers

The public `full`, `arange`, and `eye` regression tests pass.
Their runners still lack the literal historical `// TODO` marker required by Rustlings.
Add the matching comment yourself without a test change.

```rust
// In exercises/01_creation/003_full.rs:
// TODO (historical, completed): implementation moved to rumpy::full.

// In exercises/01_creation/004_arange.rs:
// TODO (historical, completed): implementation moved to rumpy::arange.

// In exercises/01_creation/005_eye.rs:
// TODO (historical, completed): implementation moved to rumpy::eye.
```

This metadata issue does not block the next exercise or your Cargo tests.
