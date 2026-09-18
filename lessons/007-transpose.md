# 007 — transpose

Write `transpose` in `exercises/02_shape/007_transpose.rs`.

Reshape keeps the flat value order. Transpose exchanges rows and columns, so its flat value order usually changes.

## Public contract

```rust
pub fn transpose(a: &Array) -> Array
```

The input must have exactly two axes. Shape `[rows, columns]` becomes `[columns, rows]`.
The value at input row `r`, column `c` becomes the value at output row `c`, column `r`.
The result owns contiguous, row-major `f64` data and its own shape.
The input stays unchanged. Each result owns independent storage, even for square or single-element arrays.

Preserve every value bit, including signed zeros, infinities, subnormals, and quiet or signaling NaN payloads.
No arithmetic on the stored values is necessary. Numeric equality alone cannot check this contract.

For any rank other than two, panic with the message `transpose requires a two-axis array`.
Check the rank even when the input contains no elements.

Original API: [numpy.transpose](https://numpy.org/doc/stable/reference/generated/numpy.transpose.html).
The matching NumPy expression for this subset is `np.transpose(a).copy(order="C")`.
NumPy usually returns a view. This exercise pays for a copy to keep ownership simple and avoid shared-storage rules.

## Supported subset

- Use the existing owned `Array` with two explicit `usize` dimensions and `f64` values.
- Support rectangles, squares, and singleton axes. Do not remove axes of length one.
- Preserve empty dimensions: `[0, 7]` becomes `[7, 0]`, and `[0, 0]` stays `[0, 0]`.
- Empty shapes `[usize::MAX, 0]` and `[0, usize::MAX]` must finish promptly without huge allocations or axis-length loops.
- Nonempty inputs use manageable allocations. Allocation failure is outside this contract. No extra dimension-limit assertion is required.

A scalar has rank zero, not shape `[1, 1]`. A vector has rank one, even when its length is one.
Both ranks panic here, although NumPy accepts them.
Higher ranks, custom axis permutations, negative axis indices, views, arbitrary strides, other dtypes, and in-place operations are outside this lesson.
There is no `axes`, `out`, or copy-policy parameter in this Rust API.
NumPy cannot represent the full `usize` shape range, so the largest empty dimensions have separate Rust tests.

## Visual example

```text
Input shape: [2, 3]       Output shape: [3, 2]
Input rows:              Output rows:
[11, 12, 13]             [11, 21]
[21, 22, 23]             [12, 22]
                         [13, 23]

Input flat data:         [11, 12, 13, 21, 22, 23]
Transpose flat data:     [11, 21, 12, 22, 13, 23]
Reshape to [3, 2]:       [11, 12, 13, 21, 22, 23]
```

The output's first row contains the input's first column.
Read the output rows in order to check the flat result.
Transpose that output again to recover the original shape and every original bit.

## Available helpers

1. Inspect `Array::ndim`, `shape`, `size`, and `as_slice` in `src/array.rs`.
2. Use `Array::from_vec` to validate a new owned result.
3. Use standard `Vec` methods for the output buffer.
4. Check empty inputs separately from nonempty traversal.
5. Preserve the tests and leave earlier code and dependencies unchanged.

A shape swap alone is not sufficient for a general rectangle or nonsymmetric square.
The lesson does not require a new error type or a change to `Array`.

## Checks

```sh
rustlings hint 007_transpose
cargo test --bin 007_transpose
rustlings run 007_transpose
```

The unfinished stub compiles. Its tests fail at `todo!` until you complete the function.
The tests cover both rectangle orientations, squares, singleton axes, empty axes, bit preservation, independent storage, double transpose, and wrong ranks.
A comparison answer exists at `solutions/02_shape/007_transpose.rs`, with the same contract tests.

The fixture stores unsigned binary64 integers, not JSON floating-point values.
The generator uses real NumPy and direct integer views to preserve NaN payloads.

```sh
uv run --with numpy==2.4.3 scripts/generate_transpose_oracle.py --check
```

Rustlings watches the current exercise only. After a dependency change, press `c` for a full check.
Alternatively, use `rustlings --manual-run` and press `r`.

## Library checklist

You own these changes after the exercise passes. The author does not move your code or change your callers.

1. Choose the module layout for shape operations.
2. Move your function into `src/` and expose the unnumbered `transpose` API.
3. Connect the preserved exercise tests to that API. Keep the literal `// TODO (historical, completed):` marker.
4. Refactor shared behavior only where it helps. Keep dependencies one-way and avoid cycles.
5. Run the regression tests and checks below.

`Array::from_vec` can remain the low-level constructor. Do not make it call `transpose` if `transpose` calls it.
Reshape and transpose have different data-order contracts. Preserve both test sets if you share any helper.
No caller migration is required just to add this new operation.

```sh
cargo test --lib --test scaffold
cargo test --bin 001_zeros --bin 002_ones --bin 003_full --bin 004_arange --bin 005_eye --bin 006_reshape --bin 007_transpose
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
```
