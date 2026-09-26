# 013 — multiply

## One product per pair

Multiply corresponding values in two arrays. Keep the shape and make a new output array. This task follows `add`, but it does not call `add`. Write this function in `exercises/04_elementwise/013_multiply.rs`:

```rust
pub fn multiply(a: &Array, b: &Array) -> Array
```

NumPy API: <https://numpy.org/doc/stable/reference/generated/numpy.multiply.html>

## Supported contract

- Both inputs are valid owned, contiguous, row-major `f64` arrays. Require identical full shapes, including rank, axis order, and singleton axes. The same element count is not sufficient.
- If shapes differ, panic with exactly `multiply requires identical shapes`. Check this before a shortcut for empty inputs. NumPy can broadcast some such inputs, but this task must reject them.
- At each flat position `i`, use the ordinary Rust `f64` product `a[i] * b[i]`. Do not multiply in an initial accumulator.
- Return a new array with the same shape and independent storage. Keep both input shapes and all input bits unchanged. The same array may supply both arguments.
- Shape `[]` contains one scalar product. A zero-length axis contains no products. Preserve matching empty shapes such as `[usize::MAX, 0]` without a product of axis lengths or an axis traversal.

This lesson omits broadcasting, axis parameters, `out`, `where`, integer data, dtype conversion, views, and operator overloads. It needs no new library code or dependency to solve the exercise.

## Examples

```text
shape [3]:
[ 2.0, -3.0, 0.5 ]
[ 4.0,  5.0, 8.0 ]
[ 8.0, -15.0, 4.0 ]  shape [3]

shape []: [2.5] times [-4.0] gives [-10.0] with shape [].
shape [2, 0, 3]: two empty inputs give shape [2, 0, 3].
shape [2, 3] times shape [3, 2] must panic.
shape [] times shape [1] must panic, even if NumPy broadcasts it.
```

## Floating-point rules

One `f64` multiplication controls each result. The tests compare non-NaN results by their bits and NaN results by `is_nan()`. NaN payloads and signs in the output are not specified. Preserve all input bits, including signaling-NaN bits.

A negative value times positive zero gives negative zero. Two negative values give a positive product. Infinity times zero gives NaN. A finite product can overflow to infinity or underflow to zero. The test checks subnormal products and normal rounding.

Do not use a zero seed for each product. A zero seed can change the sign of a zero result or turn an infinity product into NaN.

## Rust tools

Use `shape()` for full-shape checks and `as_slice()` for stored values. An iterator `zip` pairs values, but it stops at the shorter slice and does not check shapes. `Array::from_vec` checks that the new stored length matches the copied shape. You may use an ordinary loop or standard iterators.

```sh
cargo test --bin 013_multiply
rustlings hint 013_multiply
rustlings run 013_multiply
```

The initial TODO fails by design. Compare your answer with `solutions/04_elementwise/013_multiply.rs` after your attempt.

## Your library checklist

1. Choose a module for `multiply` after its tests pass.
2. Move your function into `src/` and expose its public API.
3. Connect the preserved tests to that public API without a test change.
4. Refactor shared behavior if useful. Avoid a dependency cycle between operations.
5. Rerun the checks, then commit and push your changes.

Your public `add` function passes its preserved tests. Its runner lacks a historical `// TODO` marker required by the Rustlings author check. Add `// TODO (historical, completed): implementation moved to rumpy::add.` to `012_add.rs` when you edit your runner. The author leaves that file unchanged.
