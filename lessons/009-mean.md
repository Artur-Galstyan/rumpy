# 009 — mean

Write `mean` in `exercises/03_reductions/009_mean.rs`.

A mean reduces every stored value to one Rust `f64`. This exercise adds one division to an explicitly ordered reduction.
Your public `rumpy::operations::sum` satisfies the required addition order. You may reuse it or write a local sequential reduction.

## Public contract

```rust
pub fn mean(a: &Array) -> f64
```

Start with positive zero (`0.0_f64`). Add every value from `a.as_slice()` in flat, left-to-right order.
After the complete sum, divide by the element count converted with `as f64`.
Use ordinary `f64` arithmetic for each addition and the final division.
Do not reorder, group, compensate, or parallelize the additions. Do not divide each element first or use an online average.

An empty array returns NaN. No warning or error return is required.
All ranks are valid. Shape `[]` contains one scalar, not zero elements.
A scalar still participates in the initial addition. A scalar `-0.0` therefore produces `+0.0`.
The denominator counts elements, not axes or the first axis length.

Read the stored element count, not a new shape product. Empty shapes `[usize::MAX, 0]` and `[0, usize::MAX]` must finish promptly.
Do not traverse axis lengths. No result buffer or new allocation is necessary.
The input shape and every data bit must stay unchanged.

Infinities follow IEEE arithmetic. Opposite infinities produce NaN.
Finite addition can overflow before division, even when the mathematical mean is finite.
Division can underflow to a subnormal value or signed zero. Do not replace those results with a tolerance or a clamp.
A NaN input produces NaN. Only the result's NaN class matters, not its sign, payload, or signaling state.
Input NaN bits must remain unchanged.

## Supported API

Original API: [numpy.mean](https://numpy.org/doc/stable/reference/generated/numpy.mean.html).
The comparable NumPy call is `np.mean(a, dtype=np.float64)`, with its default `axis=None`.

This Rust API accepts the existing owned, contiguous, row-major `f64` array.
There is no `axis`, `dtype`, `out`, `keepdims`, or `where` parameter.
Views, arbitrary strides, integer arrays, and other element types are outside this lesson.
The result is a Rust scalar, not an `Array` or a `Result`.

NumPy can use pairwise summation. It does not promise this exercise's left-to-right addition order.
This simple order makes the contract explicit, but can lose accuracy or overflow compared with other mean algorithms.
Do not expect bit-identical NumPy results for arbitrary inputs.

## Examples

| Input shape | Flat values | Result |
| --- | --- | --- |
| `[2, 2]` | `[1.0, 2.0, 3.0, 4.0]` | `2.5` |
| `[3]` | `[0.5, -0.25, 0.125]` | `0.125` |
| `[]` | `[-0.0]` | `+0.0` |
| `[2, 0, 3]` | `[]` | NaN |
| `[4]` | `[1e16, 1.0, -1e16, 1.0]` | `0.25` |

The last example checks operation order, not exact real-number arithmetic.

## Available helpers

1. Read `Array::as_slice`, `size`, and `shape` in `src/array.rs`.
2. Use a `for` loop or a sequential `fold` with an explicit initial value.
3. Use ordinary addition, division, `usize` to `f64` conversion, and `f64::NAN`.
4. Use `to_bits()` for exact non-NaN results and `is_nan()` for NaN results.
5. Preserve the tests and leave earlier exercises and library code unchanged.

No new constructor, shape helper, error type, or external crate is necessary.
Do not import `sum` from an exercise file or copy a reference into the library.

## Progressive hints

First, distinguish a scalar shape from an empty data slice.
Next, identify the existing method that reports the stored element count.
Then, separate the ordered reduction from the final division.
Finally, check negative zero, an empty array, and a sum that overflows before division.
The full comparison answer belongs only in `solutions/03_reductions/009_mean.rs`.

## Checks

```sh
rustlings hint 009_mean
cargo test --bin 009_mean
rustlings run 009_mean
```

The stub compiles and fails at `todo!` until you complete it.
The reference has identical tests for ranks, empty dimensions, order, overflow, subnormals, signed zeros, infinities, and NaNs.
Each helper call checks the unchanged input shape and data bits.

The saved NumPy fixture uses small exact dyadic inputs, empty arrays, signed zeros, and nonfinite values.
Finite sums are exact in either order. The final division can round, but both paths divide the same sum by the same count.
The generator checks agreement between NumPy and a sequential `float64` mean for every saved case.
It saves the actual NumPy version and uses integer views to preserve signaling NaN input bits.
Strict JSON uses `input_shape`, `input_bits`, `expected_nan`, and `expected_bits`.
Expected NaN bits are `null`. Other results use unsigned binary64 bits.
Order-sensitive results, finite overflow, subnormal division, and enormous empty dimensions have separate Rust tests.

```sh
uv run --with numpy==2.4.3 scripts/generate_mean_oracle.py --check --verify-rust
```

Rustlings watches only the current exercise. After a dependency change, press `c` for a full check.
Alternatively, use `rustlings --manual-run` and press `r`.

## Library checklist

You own these changes after the exercise passes. The author does not move your code or change your callers.

1. Choose a module layout for reductions.
2. Move your implementation into `src/` and expose the unnumbered `mean` API.
3. Connect the preserved tests to that API. Keep the literal `// TODO (historical, completed):` marker.
4. Refactor shared behavior only after the relevant APIs exist and their contracts agree.
5. Run the preserved regression tests and the checks below.

Your `mean` can use the public `sum` because it preserves this exact order and positive-zero start.
Keep dependencies one-way. Do not make `sum` call `mean` while `mean` calls `sum`.
Do not make `Array::as_slice` or `size` call a reduction that depends on those methods.
Do not fix earlier attempts or change array constructors merely to add this operation.

```sh
cargo test --lib --test scaffold
cargo test --bin 001_zeros --bin 002_ones --bin 003_full --bin 004_arange --bin 005_eye --bin 006_reshape --bin 007_transpose --bin 008_sum --bin 009_mean
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
```

Earlier unfinished exercises can still fail. Check those failures separately from the new mean tests.
