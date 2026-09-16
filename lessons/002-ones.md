# 002 — `ones`: reuse an operation without changing the shape

**Status:** completed and already in the library. The original exercise instructions
follow. For future tasks, you own graduation and refactoring as described in the README.

**Original task:** implement `ones(shape: &[usize]) -> Array` in
`exercises/01_creation/002_ones.rs`. Reuse your completed `rumpy::zeros` operation.

Your `zeros` implementation now lives in `src/creation/zeros.rs`. Do not edit it
for this exercise. The old exercise runs its original tests against the library.

## The idea

An operation can reuse another operation's shape and allocation behaviour.
For `ones`, the shape stays the same. Every stored value becomes `1.0`.

```text
shape [2, 3]

zeros                       ones
[0, 0, 0]                   [1, 1, 1]
[0, 0, 0]                   [1, 1, 1]

flat data after: [1, 1, 1, 1, 1, 1]
shape after:     [2, 3]
```

The allocation already exists. Change its values rather than construct another
shape algorithm. This is a small step toward larger operations built from your
own library functions. The tests check behaviour; they do not prescribe a loop
or a particular slice method.

## Contract

- Return an owned `Array` with the exact supplied shape. Do not sort or remove axes.
- Every stored value is exactly `1.0` (`f64`). Support any number of axes.
- A zero-length axis means no values. Keep the zero-length axis in the shape.
- An empty shape `[]` denotes a scalar with one value, `1.0`. It differs from `[0]`.
- The result owns its shape and data. Separate calls must not share mutable storage.

The input scope matches exercise 001: small axis lengths whose products fit
`usize`, with data that fits memory. Oversized shapes and allocation failures
are outside this exercise. The operation fixes `dtype` to `f64` and storage to
contiguous row-major order. It omits NumPy's `dtype`, `order`, `device`, and `like`
parameters. Exact equality is appropriate here because `1.0` is represented exactly.

## Available helpers

```rust
rumpy::zeros(shape: &[usize]) -> Array
array.as_mut_slice() -> &mut [f64]
array.shape() -> &[usize]
array.as_slice() -> &[f64]
```

Use Rust loops or slice methods. Do not call NumPy or a numerical crate from your
implementation. NumPy is an independent test oracle, not the implementation.

## Checks and comparison

```sh
rustlings hint 002_ones
rustlings run 002_ones
# Headless alternative:
cargo test --bin 002_ones
# Check the completed dependency:
cargo test --bin 001_zeros
```

The tests cover vectors, rectangular matrices, higher-rank arrays, zero-length
axes, scalars, size-one axes, and ownership. The stub intentionally fails them.

After your attempt, compare with `solutions/01_creation/002_ones.rs`. That file does
not replace your exercise or library code. Commit and push your attempt to `main`.
A new pushed commit triggers the next exercise, even if this one is unfinished.
A passing implementation can graduate to `src/creation/ones.rs` and `rumpy::ones`.
Until then, `ones` exists only in the exercise, not in the public library.

## API source

https://numpy.org/doc/stable/reference/generated/numpy.ones.html

Actual oracle cases are recorded in `validation/ones-numpy.json`.
