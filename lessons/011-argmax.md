# 011 — argmax

Write `argmax` in `exercises/03_reductions/011_argmax.rs`.

`amax` returns a value. `argmax` returns the position of a selected value.
A flat index counts stored elements from zero in row-major order.

## Public contract

```rust
pub fn argmax(a: &Array) -> usize
```

Accept the existing owned, contiguous, row-major `f64` array at every rank.
Reduce all stored elements, not one axis. Return a Rust `usize`, not an `Array` or a coordinate tuple.

If any value is NaN, return the **first NaN index** in flat order.
Otherwise, return the **first maximal value index** in flat order.
Positive and negative zero compare equal. Their signs do not change the index rule.
Infinities follow numeric order. Subnormal values also follow numeric order.
Quiet and signaling NaNs both count as NaN, regardless of sign or payload.

Shape `[]` contains one scalar. Its result is index `0`, even for NaN.
An empty array must panic with exactly:

```text
argmax requires at least one element
```

Use the stored slice or element count to detect empty input, not a shape product.
Empty shapes `[usize::MAX, 0]` and `[0, usize::MAX]` must panic promptly.
Do not traverse axis lengths. Nonzero dimensions can overflow a product even when another dimension is zero.
Preserve the input shape and every input bit, including NaN payloads and zero signs.
The same preservation rule applies after an empty-input panic.
No result buffer is necessary.

## Supported API

Original API: [numpy.argmax](https://numpy.org/doc/stable/reference/generated/numpy.argmax.html).
The comparable NumPy call is `np.argmax(a)`, with the default `axis=None`.
NumPy documents the first occurrence rule for repeated maxima.
This exercise also states an explicit first-NaN rule. The saved NumPy cases check that behavior.

This exercise has no `axis`, `out`, `keepdims`, or `dtype` parameter.
Other element types, views, and arbitrary strides are outside this lesson.
NumPy raises `ValueError` for empty input. This Rust API uses the exact panic above.
Do not confuse `argmax` with `nanargmax`, which ignores NaNs.

## Examples

| Input shape | Flat values | Index |
| --- | --- | --- |
| `[2, 3]` | `[-1, 2, 3, 4, 9, 6]` | `4` |
| `[3]` | `[-9, -2, -5]` | `1` |
| `[2, 3]` | `[-1, 9, 9, 2, 9, 0]` | `1` |
| `[3]` | `[-4, -0.0, +0.0]` | `1` |
| `[]` | `[-0.0]` | `0` |
| `[3]` | `[+infinity, NaN, -infinity]` | `1` |
| `[2, 0, 3, 1]` | `[]` | Exact empty-input panic |

The first row assigns indices `[0, 1, 2, 3, 4, 5]` to the flat values.
The value `9` has index `4`. The result is not its row or column coordinate.
The fixture generator checks these examples with real NumPy calls.

## Available helpers

1. Read the public `Array::as_slice`, `size`, and `shape` methods in `src/array.rs`.
2. Review slice methods such as `first`, `is_empty`, and `iter`.
3. Review `Iterator::enumerate` for zero-based positions.
4. Use ordinary comparisons and `f64::is_nan` for the selection rules.
5. Use `to_bits()` in tests to check that input values stay unchanged.

`Array::from_vec` constructs test inputs. The reduction needs no constructor or external crate.
Do not import functions from earlier exercise files.
The public `amax` API exists, but its scalar result does not retain an index.
NaN does not compare equal to itself, so an equality search for a NaN maximum is not sufficient.

## Progressive hints

First, distinguish an empty slice from a scalar shape.
Next, identify the information that a value-only maximum loses.
Then, consider how equal values affect the first-index rule.
Finally, consider NaNs before, between, and after numeric maxima.
A comparison alone does not establish the NaN rule.

The full comparison answer belongs only in `solutions/03_reductions/011_argmax.rs`.
Keep the tests intact. Leave earlier exercises and library code unchanged during this exercise.

## Checks

```sh
rustlings hint 011_argmax
cargo test --bin 011_argmax
rustlings run 011_argmax
```

The stub compiles and fails at `todo!` until you complete it.
The reference contains the identical 13 contract tests.
Each successful call checks the input shape and every data bit.
Empty tests check the exact panic text and the unchanged input.
The author generator runs the reference tests with a timeout in debug and optimized builds.

The strict JSON fixture records `input_shape`, `input_bits`, and `expected_index` for each nonempty case.
Its `empty_cases` list records observed NumPy error types and messages.
The generator records the actual NumPy version and a deterministic seed.
Unsigned integer views preserve signaling NaN input bits.
Both zero signs can occur together because the result is an index, not a selected zero bit pattern.
Huge empty shapes have separate Rust tests because NumPy has different shape limits.

```sh
uv run --with numpy==2.4.3 scripts/generate_argmax_oracle.py --check --verify-rust
```

Rustlings watches only the current exercise. After a dependency change, press `c` for a full check.
Alternatively, use `rustlings --manual-run` and press `r`.

## Library checklist

You own these changes after the exercise passes. The author does not move your code or change your callers.

1. Choose a module layout for reductions.
2. Move your implementation into `src/` and expose the unnumbered `argmax` API.
3. Connect the preserved tests to that API. Keep the literal `// TODO (historical, completed):` marker.
4. Refactor shared behavior only when the APIs exist and their contracts agree.
5. Run the preserved regression tests, formatting checks, Cargo checks, and Clippy.

CAUTION: Keep helper dependencies one-way. A cycle can cause unbounded recursion.
For example, do not make `amax` call `argmax` while `argmax` still calls `amax`.
Do not force a shared empty-input or NaN rule onto reductions with different contracts.

```sh
cargo test --lib --test scaffold
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
```

Run the earlier exercise regression tests separately from the new `argmax` tests.
