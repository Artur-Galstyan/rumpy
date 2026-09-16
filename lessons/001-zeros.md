# 001 — `zeros`: shape is not storage

**Status:** completed. Your implementation moved to `src/creation/zeros.rs`.
The original exercise now tests `rumpy::zeros`. Its tests remain unchanged.
Compare with `solutions/01_creation/001_zeros.rs` if useful. The original lesson follows.

**Original task:** implement `zeros(shape: &[usize]) -> Array` in `exercises/01_creation/001_zeros.rs`.
This is the first building block of your numerical library.

## The idea

An array has values and a shape. Rumpy stores the values in one flat buffer.
The shape tells future operations how to interpret that buffer.

For a shape of `[2, 3]`, the logical array is:

```text
             column
             0  1  2
row 0        0  0  0
row 1        0  0  0

flat data:  [0, 0, 0, 0, 0, 0]
shape:      [2, 3]
```

Element-count example, with each step explicit:

1. `count = rows * columns` — multiplication principle: each row contains every column position.
2. `count = 2 * 3` — substitute the given axis lengths.
3. `count = 6` — evaluate the multiplication.

Verification:

1. `count = 3 + 3` — count the entries in each displayed row.
2. `count = 6` — evaluate the addition.
3. Both counts equal six — the shape and buffer agree.

`[3, 2]` has the same element count but a different shape. Do not sort or remove axes.
The last axis changes fastest in row-major storage. Today's zeros hide that ordering;
later indexing exercises will make it visible.

## Contract

- Return an owned `Array` with the exact supplied shape.
- Every stored value is positive `0.0` (`f64`).
- Support any number of axes, not just vectors or matrices.
- A zero-length axis means no stored elements. Preserve the shape, including its zero.
- An empty shape `[]` means a scalar: one stored value and no axes. This is different from `[0]`.
- Do not borrow the caller's shape or share mutable storage between results.
- Inputs are deliberately small: their element count fits `usize`, and their data fits memory.
  Allocation failure and oversized shapes are not part of this exercise.

The empty-shape convention matches `numpy.zeros(())`. The zero-length-vector convention matches
`numpy.zeros((0,))`. These are not interchangeable.

## Available scaffold

```rust
Array::from_vec(data: Vec<f64>, shape: Vec<usize>) -> Result<Array, ShapeError>
```

This constructor checks that the data length matches the shape. You may use `.expect(...)`
with a useful message after you construct consistent data and shape. You do not need to
write error types, memory management, or the array container today.

Useful accessors for tests and later exercises:

```rust
array.shape()
array.ndim()
array.size()
array.as_slice()
array.as_mut_slice()
```

Use standard Rust iterators or loops and `Vec`. Do not call a numerical crate, Python,
or an external process from your implementation. No special solution style is required.

## Hints and verification

```sh
rustlings hint 001_zeros
rustlings run 001_zeros
```

Hints progress from the shape concept to useful Rust tools. Stop reading when you have enough.
The seven tests cover vectors, rectangular matrices, higher-rank arrays, empty axes,
scalars, size-one axes, and independent ownership. Test names explain their purpose.

After you pass, briefly consider: why must `zeros(&[])` differ from `zeros(&[0])`?
This is optional reflection, not an additional submission requirement.

## API connection

Reference: https://numpy.org/doc/stable/reference/generated/numpy.zeros.html

NumPy supports more parameters. This first Rust version fixes `dtype` to `f64` and `order`
to row-major storage. It has no `like` or `device` parameter. Later functions will reuse
this container and your implementation through the `rumpy` library.
