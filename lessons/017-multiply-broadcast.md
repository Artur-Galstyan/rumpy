# 017 — multiply_broadcast

## Two input shapes

Edit `exercises/04_elementwise/017_multiply_broadcast.rs` to multiply two arrays under NumPy's trailing-axis broadcast rule.

```rust
pub fn multiply_broadcast(a: &Array, b: &Array) -> Array
```

NumPy API: <https://numpy.org/doc/stable/reference/generated/numpy.multiply.html>. Broadcast rules: <https://numpy.org/doc/stable/user/basics.broadcasting.html>.

## Supported contract

- Both inputs are owned, contiguous, row-major `f64` arrays. Compare axes from the right. Missing leading axes have length one. On each axis, equal lengths match, or a length-one axis expands to the other length. Length one expands to zero; length zero does not expand to a positive length greater than one.
- Reject any incompatible axis before an empty-output shortcut. Panic with exactly `multiply_broadcast requires compatible shapes`. Keep the larger rank and every axis in the output shape. A scalar has shape `[]` and one value.
- The output is independent owned storage. Each row-major output value is the Rust `f64` product of the two selected inputs. Preserve both input shapes and their value bits. Compare output NaNs by class, and all other output values by bits.
- If any output axis is zero, return an empty result without products of other axes, even for `usize::MAX`. A nonempty output whose element count exceeds `usize` must panic with `multiply_broadcast output size exceeds usize`. Allocation failure for otherwise representable huge arrays is outside scope.
- This separate extension does not change `rumpy::multiply` or its old equal-shape tests. You may use `rumpy::multiply` for equal shapes and `rumpy::broadcast_to` where its current behavior is suitable, but handle huge empty outputs without calling an unsafe shape product. Do not edit the old runners automatically.

```text
[3] * [2, 1] -> [2, 3]
[] * [3] -> [3]
[0, 1] * [1] -> [0, 1]
[2] * [3] -> incompatible
```

Views, arbitrary strides, dtypes other than `f64`, `out`, `where`, and other NumPy parameters are not supported. Use `Array`, slices, vectors, and standard Rust methods.

```sh
cargo test --bin 017_multiply_broadcast
rustlings hint 017_multiply_broadcast
rustlings run 017_multiply_broadcast
```

The comparison answer is `solutions/04_elementwise/017_multiply_broadcast.rs`.

## Your library checklist

1. Choose a module layout for the new behavior.
2. Move your code into `src/` and expose the public API you choose.
3. Connect the preserved regression tests to your public API.
4. Refactor shared behavior if useful, without a dependency cycle between `multiply` and `broadcast_to`.
5. Rerun the checks, then commit and push.

The `015_broadcast_to` local tests pass, and a public copy exists. Its old runner still tests the local function. Connect the preserved tests to `rumpy::broadcast_to` for graduation. A huge empty target such as `[usize::MAX, 2, 0]` currently panics in the public copy because its product overflows before it reaches zero. Fix that in your own code before reuse. Add a historical `// TODO (historical, completed):` marker to the old runner when you connect it.

The previous `016_add_broadcast` passes its six local tests; connect its preserved tests to a public API before calling it graduated. Its marker remains learner-owned.
