# 014 — subtract

## One difference per pair

Subtract the right value from the left value at each stored position. Edit `exercises/04_elementwise/014_subtract.rs`.

```rust
pub fn subtract(a: &Array, b: &Array) -> Array
```

NumPy API: <https://numpy.org/doc/stable/reference/generated/numpy.subtract.html>

## Supported contract

- Inputs are valid owned, contiguous, row-major `f64` arrays. Require identical full shapes, including rank and singleton axes. Equal element counts alone do not suffice.
- If shapes differ, panic with exactly `subtract requires identical shapes`. Check before an empty-array shortcut. Do not broadcast.
- At each stored position, use one ordinary `f64` subtraction: `left - right`. Operand order matters. Do not replace subtraction with addition of a negated right value: signed-zero behavior can differ.
- Return a new Array with the same shape and independent storage. Keep both inputs' shapes and every input bit unchanged. The same Array may supply both inputs.
- Shape `[]` holds one scalar difference. Any zero-length axis produces an empty result with the original shape. For equal empty shapes such as `[0, usize::MAX, 2]`, do not multiply dimensions or visit axes to create values.

This lesson omits broadcasting, integer data, dtype conversion, `out`, `where`, views, and operator overloads. It uses only `Array`, slices, and standard Rust iterators. You do not need `add` or `multiply`.

## Examples

```text
shape [3]: [5.0, -3.0, 0.25] minus [2.0, 4.0, -0.5]
result:  [3.0, -7.0, 0.75] with shape [3]
shape []: [2.5] minus [-4.0] gives [6.5] with shape [].
shape [2, 0, 3]: two empty arrays give an empty array with shape [2, 0, 3].
shape [] minus shape [1] must panic, even though NumPy broadcasts it.
```

## Floating-point rules

Use `f64` subtraction at each position without an initial value. The tests compare non-NaN outputs by their bits and NaN outputs by class. Output NaN payloads and signs are not specified. Input bits must remain unchanged, including signaling-NaN bits. Check signed zero, infinity, overflow, subnormal values, and rounding. For example, negative zero minus positive zero gives negative zero, while negative zero minus negative zero gives positive zero.

## Rust tools

`shape()` gives the full shape. `as_slice()` gives contiguous stored values. `zip` stops at the shorter slice, so check the shapes before you use it. `Array::from_vec` checks the result length against its owned shape. Do not copy a reference function into `src/`.

```sh
cargo test --bin 014_subtract
rustlings hint 014_subtract
rustlings run 014_subtract
```

The unfinished TODO must fail. The separate answer is `solutions/04_elementwise/014_subtract.rs`.

## Your library checklist

1. Choose a module layout after your tests pass.
2. Move your function into `src/` and expose its public API.
3. Connect the preserved tests to that API without a test change.
4. Refactor shared behavior if useful, but avoid dependency cycles.
5. Rerun the checks, then commit and push your changes.

Your `multiply` function and its public copy pass their initial tests, but its runner still tests the local function. Connect the preserved runner tests to `rumpy::multiply` when you graduate it. Add a historical `// TODO (historical, completed):` marker to that runner so Rustlings author checks can accept it. The author does not change your source or prior runner.
