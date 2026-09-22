# 010 — amax

Write `amax` in `exercises/03_reductions/010_amax.rs`.

A maximum reduces every stored value to one Rust `f64`. Unlike `sum` and `mean`, this reduction selects an input value without arithmetic.

## Public contract

```rust
pub fn amax(a: &Array) -> f64
```

Accept the existing owned, contiguous, row-major `f64` array at every rank.
Reduce every value in `a.as_slice()`, not one axis. Return a Rust scalar, not an `Array` or a `Result`.
Shape `[]` contains one scalar. It is not an empty array.

For nonempty input without NaNs, select the numerically greatest stored value.
Copy its exact bits. Do not add, subtract, multiply, divide, clamp, or round the values.
If equal maxima occur, choose the **FIRST stored equal value** in flat order.
Positive and negative zero compare equal, so the first maximal zero determines the result's sign.

Any NaN input produces NaN, even when another value is positive infinity.
Only the result's NaN class matters. Its sign, payload, and signaling state are unspecified.
Quiet and signaling NaNs must not change any input bits.
Infinities follow numerical ordering. Opposite infinities produce positive infinity, not NaN.
Subnormal values retain the selected bits.

An empty array must panic with exactly:

```text
amax requires at least one element
```

Use the stored slice or element count to detect empty input, not a shape product.
Empty shapes `[usize::MAX, 0]` and `[0, usize::MAX]` must panic promptly.
Do not traverse axis lengths. An empty shape can contain dimensions whose nonzero product overflows.
The input shape and every data bit must stay unchanged, including after an empty-input panic.
No result buffer or new allocation is necessary.

## Supported API

Original API: [numpy.amax](https://numpy.org/doc/stable/reference/generated/numpy.amax.html).
The comparable NumPy call is `np.amax(a)`, with its default `axis=None`.

This exercise has no `axis`, `out`, `initial`, `where`, `keepdims`, or `dtype` parameter.
Views, arbitrary strides, integer arrays, and other element types are outside this lesson.
NumPy raises `ValueError` for an empty maximum without `initial`. This Rust API uses the exact panic above.

NumPy does not specify signed-zero tie bits. This Rust contract deliberately specifies the first stored equal value.
Do not use NumPy's observed mixed-zero result as a bit-level rule.
Rust's `f64::max` also does not express this full contract: it ignores one NaN rather than propagating every NaN.

## Examples

| Input shape | Flat values | Result |
| --- | --- | --- |
| `[2, 2]` | `[-4.0, -2.0, -7.0, -3.0]` | `-2.0` |
| `[]` | `[-0.0]` | `-0.0` |
| `[3]` | `[-5.0, -0.0, +0.0]` | `-0.0` |
| `[2]` | `[+infinity, NaN]` | NaN |
| `[2, 0, 3]` | `[]` | Exact empty-input panic |

## Available helpers

1. Read `Array::as_slice`, `size`, and `shape` in `src/array.rs`.
2. Review slice methods such as `first`, `split_first`, and `is_empty`.
3. Use ordinary comparisons and `f64::is_nan` to distinguish selection from NaN policy.
4. Use `to_bits()` for exact non-NaN checks and `is_nan()` for NaN checks.
5. Preserve the tests and leave earlier exercises and library code unchanged.

No constructor, error type, or external crate is necessary.
Do not import a reduction from an exercise file or copy a reference into the library.

## Progressive hints

First, distinguish a scalar from an empty slice.
Next, consider why zero cannot serve as the initial maximum for all-negative inputs.
Then, compare the effect of strict and non-strict comparisons on equal values.
Finally, consider where a NaN can occur and why a comparison alone cannot detect it.
The full comparison answer belongs only in `solutions/03_reductions/010_amax.rs`.

## Checks

```sh
rustlings hint 010_amax
cargo test --bin 010_amax
rustlings run 010_amax
```

The stub compiles and fails at `todo!` until you complete it.
The reference uses identical contract tests. Each successful reduction checks the input shape and every input bit.
The empty tests check the exact panic text and the unchanged input.
The huge empty tests run under a timeout in the author generator, in debug and optimized builds.

The strict JSON fixture uses the same scalar fields as the mean fixture:
`input_shape`, `input_bits`, `expected_nan`, and `expected_bits`.
Expected NaN bits are `null`. Other results use unsigned binary64 bits.
A separate `empty_cases` list contains `input_shape` for each NumPy-verified empty rejection.
The generator records the actual NumPy version and the reproducible seed.
Integer views preserve signaling NaN input bits.

Fixtures exclude inputs with both zero signs because NumPy does not specify their tie bits.
Dedicated Rust tests verify the first mixed-zero tie in both orders.
Huge empty dimensions also have separate Rust tests because NumPy's shape limits differ.

```sh
uv run --with numpy==2.4.3 scripts/generate_amax_oracle.py --check --verify-rust
```

Rustlings watches only the current exercise. After a dependency change, press `c` for a full check.
Alternatively, use `rustlings --manual-run` and press `r`.

## Library checklist

You own these changes after the exercise passes. The author does not move your code or change your callers.

1. Choose a module layout for reductions.
2. Move your implementation into `src/` and expose the unnumbered `amax` API.
3. Connect the preserved regression tests to that API. Keep the literal `// TODO (historical, completed):` marker.
4. Refactor shared behavior only when the APIs exist and their contracts agree.
5. Run the preserved regression tests and the checks below.

Keep dependencies one-way. A helper must not call a reduction that already depends on that helper.
For example, do not make `Array::as_slice` call `amax` while `amax` calls `as_slice`.
Do not force a shared identity or NaN policy onto `sum`, `mean`, and `amax`. Their contracts differ.

```sh
cargo test --lib --test scaffold
cargo test --bin 001_zeros --bin 002_ones --bin 003_full --bin 004_arange --bin 005_eye --bin 006_reshape --bin 007_transpose --bin 008_sum --bin 009_mean --bin 010_amax
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
```

Check earlier failures separately from the new maximum tests.
