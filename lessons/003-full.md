# 003 — `full`

Adapt your `ones` code into `full(shape: &[usize], fill_value: f64) -> Array`.
Edit `exercises/01_creation/003_full.rs`. Keep the tests intact.

## Purpose

`ones` fixes every stored value at `1.0`. `full` lets the caller choose that value.
This operation creates constant arrays for initial states, test data, and later numerical algorithms.

The exercise starts from a copy of your `ones` code, not a blank function.
The new parameter replaces the fixed value. A temporary `todo!` replaces the final return.
Your verified `rumpy::ones` stays in `src/creation/ones.rs` with its original signature.
`full` is a separate API, not a replacement for `ones`.

## Signature and examples

```rust
pub fn full(shape: &[usize], fill_value: f64) -> Array
```

```text
full(&[2, 3], -2.5)

logical rows: [-2.5, -2.5, -2.5]
              [-2.5, -2.5, -2.5]
shape:        [2, 3]
flat data:    [-2.5, -2.5, -2.5, -2.5, -2.5, -2.5]

full(&[], 7.0)      -> shape [],     data [7.0]
full(&[0], 7.0)     -> shape [0],    data []
full(&[2, 0], 7.0)  -> shape [2, 0], data []
full(&[1, 2], -0.0) -> shape [1, 2], data [-0.0, -0.0]
```

These examples specify values directly. No arithmetic on the fill value is necessary.

## Contract

- Return an owned array with the exact input shape and axis order.
- Copy the scalar `fill_value` into every stored position. Preserve its `f64` bits.
- Keep every axis, including size-one and zero-length axes. Support any number of axes.
- Treat `[]` as one scalar. Treat any zero-length axis as an array with no stored values.
- Give each call independent data and shape storage. Do not borrow the caller's shape.

Axis lengths must be small. Every intermediate shape product must fit `usize`, and the data must fit memory.
Oversized shapes and allocation failures are outside this exercise.
The operation uses owned, contiguous, row-major `f64` storage.

## Special values

Negative values, fractions, both zero signs, infinities, and NaNs are valid fill values.
Copy the input value rather than derive it through arithmetic.
The bit-preservation rule also covers NaN payloads and subnormal values.
Rust's `to_bits()` lets the tests compare stored bits without numeric equality.
A NaN does not equal itself, so ordinary equality cannot check this case.

This exercise supports only a scalar fill value, unlike NumPy's array-like fill values and broadcasting.
It fixes `dtype` to `float64` and `order` to `C`.
It omits the `dtype`, `order`, `device`, and `like` parameters.
Bit preservation is an explicit Rust contract, not a promise about every NumPy conversion or platform.

## Available helpers

```rust
rumpy::zeros(shape: &[usize]) -> Array
rumpy::ones(shape: &[usize]) -> Array
array.as_mut_slice() -> &mut [f64]
array.shape() -> &[usize]
array.as_slice() -> &[f64]
```

Reuse `rumpy::zeros` for shape and storage. Use Rust loops or slice methods to change the stored values.
Do not edit the library functions for this task. Do not use NumPy or another numerical crate in the Rust function.
NumPy supplies independent oracle data, not your function body.

## Checks and comparison

```sh
rustlings hint 003_full
rustlings run 003_full
# Headless checks:
cargo test --bin 003_full
cargo test --lib --test scaffold --bin 001_zeros --bin 002_ones
```

The copied code remains incomplete. The stub deliberately fails at `todo!`.
After your attempt, compare with `solutions/01_creation/003_full.rs`.
The reference contains the same contract tests and stays separate from your library.

Rustlings watches the current exercise only. After a dependency change, press `c` for a full check.
A fresh Rustlings state can start at `zeros`. Complete its check, then press `n` to continue through the completed exercises.

Commit and push your attempt to `main`. A new learner commit triggers one next exercise, even if this task remains incomplete.
After verification, your `full` function can graduate to `src/creation/full.rs` and `rumpy::full`.

## API source

https://numpy.org/doc/stable/reference/generated/numpy.full.html

`validation/full-numpy.json` records actual NumPy results as binary64 bit patterns.
`scripts/generate_full_oracle.py` reproduces these fixtures with NumPy.
