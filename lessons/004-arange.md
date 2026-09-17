# 004 — arange

Build a sequence with `arange`, not a repeated value. This operation supplies coordinates and sample indices for later array tasks.

## Supported API

```rust
pub fn arange(start: i32, stop: i32, step: i32) -> Array
```

The original API is [numpy.arange](https://numpy.org/doc/stable/reference/generated/numpy.arange.html).
This exercise supports explicit integer inputs and a positive step. The output uses owned contiguous row-major `f64` storage.

1. Include `start` when `start < stop`.
2. Advance by `step` between values.
3. Exclude `stop` and all greater values.
4. Return shape `[number_of_values]`, including `[0]` for an empty interval.
5. Reject `step <= 0` with a panic that contains `step must be positive`, even when `start >= stop`.

An empty interval is not a scalar. Negative endpoints are valid. A positive step with `start >= stop` produces an empty vector.
Each call owns separate storage. Zero in the output is positive zero.

Inputs span the full `i32` range. The caller limits the output to at most 100,000 elements.
Larger allocations are outside this lesson's contract. You do not need a separate size-limit check.

This subset omits fractional inputs, negative steps, optional arguments, dtype selection, device selection, and array-like dispatch.
NumPy supports negative steps. This exercise deliberately rejects them to keep the task small.
Integer inputs avoid NumPy's fractional-step precision issues. All emitted `i32` values fit exactly in `f64`.

## Examples

These are exact expected outputs, not a floating-point approximation.

| Call | Shape | Data |
| --- | --- | --- |
| `arange(0, 5, 1)` | `[5]` | `[0.0, 1.0, 2.0, 3.0, 4.0]` |
| `arange(2, 8, 2)` | `[3]` | `[2.0, 4.0, 6.0]` |
| `arange(2, 9, 3)` | `[3]` | `[2.0, 5.0, 8.0]` |
| `arange(-4, 3, 2)` | `[4]` | `[-4.0, -2.0, 0.0, 2.0]` |
| `arange(7, -3, 2)` | `[0]` | `[]` |

The tests check these examples against exact values. Saved fixtures also compare the output with NumPy 2.4.3.

## Allowed helpers

Use `Array::from_vec`, `Vec`, standard loops, iterators, and numeric conversions.
No new dependency is necessary. This task does not require `zeros`, `ones`, or `full`.

Keep integer overflow in view. A valid final value can need an increment beyond `i32::MAX` before the loop stops.
A span can also exceed `i32::MAX`, even when the result contains only a few elements.
Use wider integer arithmetic or another safe loop design. Do not rely on release-mode integer wraparound.

`Array::from_vec` checks the data length against the shape. Use a useful `expect` message if your construction makes them agree.

## Work now

1. Edit `exercises/01_creation/004_arange.rs`.
2. Run `cargo test --bin 004_arange`.
3. Read `rustlings hint 004_arange` if you need a hint.
4. Compare your result with `solutions/01_creation/004_arange.rs` after your attempt.

The new stub intentionally fails. Keep all contract tests intact.
Rustlings watches only the current exercise. After a library change, press `c`, or use `--manual-run` and `r`.

## Library checklist

After the exercise passes, you own the library changes.

1. Choose a module layout for `arange`.
2. Move your implementation into `src/` and expose the public API.
3. Connect the preserved exercise tests to your public API.
4. Refactor shared behaviour only where it helps.
5. Rerun the scaffold, earlier runners, and new runner.

Keep a completed marker in the old runner:

```rust
// TODO (historical, completed): implementation moved to the public library API.
```

Your current dependency direction is `ones` → `full` → `zeros`.
If you later make `zeros` call `full`, remove the reverse dependency first. A cycle causes unbounded recursion.
The author does not change your modules or callers.

## Review of full

Your `full` now lives in `src/creation/full.rs` and the old tests call `rumpy::full`.
Your `ones` now calls `full`. Your `creation.rs` module layout remains unchanged.
The public-API tests pass, including signed zero, infinities, and NaN payloads.

The `003_full` runner lacks Rustlings' required `// TODO` marker.
Add the historical marker above to `exercises/01_creation/003_full.rs`. Keep its tests unchanged.
This is a Rustlings author-check issue, not a numerical failure. The author left your runner untouched.
