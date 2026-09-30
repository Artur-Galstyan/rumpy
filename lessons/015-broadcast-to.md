# 015 — broadcast_to

## One input, larger shape

Edit `exercises/02_shape/015_broadcast_to.rs` to copy one array into a compatible shape.

```rust
pub fn broadcast_to(input: &Array, shape: &[usize]) -> Array
```

NumPy API: <https://numpy.org/doc/stable/reference/generated/numpy.broadcast_to.html>

## Supported contract

- Inputs are valid owned, contiguous, row-major `f64` arrays. Align input axes with the target axes from the right. An input axis matches when its length equals the target length or equals one. Missing leading axes act as length one. Reject an input rank greater than the target rank, even if leading input axes equal one.
- Check all axes before an empty-target shortcut. An incompatible shape panics with exactly `broadcast_to requires compatible shapes`. An input axis of length one can expand to length zero. An input axis of length zero cannot expand to a positive length.
- Return a new owned row-major Array with the target shape. Each output coordinate reads the input coordinate at the same aligned axis, except that an input axis of length one always reads coordinate zero. Preserve input shape and all input bits. Copy floating-point bits rather than calculate new values.
- Shape `[]` has one scalar value. A target with any zero-length axis has no values, even if another axis is `usize::MAX`. Check compatibility before this empty shortcut and avoid multiplying target dimensions for empty arrays. A nonempty target whose element count exceeds `usize` panics with `broadcast_to target size exceeds usize`. Allocation failures for otherwise representable huge arrays are outside the supported scope.

This operation differs from NumPy's read-only view: the Rust result owns independent storage. The exercise omits strides, zero-copy views, dtype conversion, `subok`, and broadcasting between two inputs. Use `Array`, slices, owned vectors, and standard Rust methods. No earlier arithmetic function is required.

## Examples

```text
input shape [3]: [1, 2, 3] -> target [2, 3]: [1, 2, 3, 1, 2, 3]
input shape [2, 1]: [1, 2] -> target [2, 3]: [1, 1, 1, 2, 2, 2]
input shape []: [7] -> target [0, 3]: []
input shape [2] -> target [3]: incompatible
input shape [1, 3] -> target [3]: incompatible rank
```

## Rust tools

`shape()` reads axes, `as_slice()` reads row-major values, and `Array::from_vec` checks an owned output. An input axis of length one reuses its first coordinate. `checked_mul` detects a nonempty target-size overflow. The unfinished TODO must fail.

```sh
cargo test --bin 015_broadcast_to
rustlings hint 015_broadcast_to
rustlings run 015_broadcast_to
```

The separate comparison answer is `solutions/02_shape/015_broadcast_to.rs`.

## Your library checklist

1. Choose the module layout after your tests pass.
2. Move your function into `src/` and expose its public API.
3. Connect the preserved tests to that API without a test change.
4. Refactor shared behavior if useful, but avoid dependency cycles.
5. Rerun the checks, then commit and push your changes.

`subtract` passes its local tests and has a public copy. Its old runner still tests its local function. Connect those preserved tests to `rumpy::subtract` when you graduate it. Add the historical `// TODO (historical, completed):` marker to that runner. The author does not change your code or prior runner.
