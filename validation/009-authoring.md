# 009 publication review

The publication adds `009_mean` for learner commit `ff2cf661835fb53384ab2e820045cd116c6cadef`.
The author used an isolated clone. No learner implementation, earlier runner, export, caller, or dependency changed.

## Learner baseline

The commit connects all original reshape assertions to `rumpy::reshape`.
It also adds an element-stride vector to `Array` and a small example in `src/main.rs`.

The reshape runner has 12 passing tests and one failing test:
`tests::zero_axis_takes_precedence_over_overflow`.
The reverse stride product at `src/array.rs:53` overflows for `[0, usize::MAX, 2]`.
The original zero-axis rule still requires an empty array for this shape.
The matching reshape reference depends on the same constructor and fails the same test.
Reshape therefore remains active with `connected_regression_failure`, not graduated.

An independent review compared 364 positive shapes per profile with NumPy 2.4.3.
Debug and optimized builds both matched `numpy.ndarray.strides / itemsize` for ranks zero through five.
The tests include scalar and singleton shapes, storage counts, stride lengths, and constructor error cases.

Empty-array strides need an explicit safe convention. The current reverse products give `[3, 1]` for `[0, 3]`.
A fresh NumPy C-order allocation gives `[0, 0]` in element units. Empty arrays have no addressable elements.
This convention difference alone is not an indexing defect. Unchecked overflow is a defect.
For `[0, usize::MAX, 2]`, the debug build panics and the optimized build wraps the first stride.
Do not treat release-mode wrapping as a fix. The learner owns this constructor change.

`008_sum` remains at its TODO. All 11 tests fail there as expected for the unfinished task.
Its reference passes all 11 tests.
The scaffold, creation runners, and transpose runner pass their preserved assertions.
The author did not change any of these files.

## New task

`009_mean` accepts every existing array rank and returns one `f64`.
It adds flat values left-to-right from positive zero, then divides by the element count converted to `f64`.
An empty array returns NaN. The function does not depend on an unfinished public `sum` API.
This simple order can lose accuracy or overflow compared with other mean algorithms.
The lesson states this trade-off and NumPy's possible pairwise addition order.

The stub compiles and fails at `todo!("mean")`.
The reference passes all 13 contract tests. The test modules are identical.
The tests cover shape independence, input preservation, signed zeros, subnormals, infinities, NaNs, overflow, and operation order.
Both `[usize::MAX, 0]` and `[0, usize::MAX]` finish without axis-length traversal.
No temporary library substitute was necessary.

The fixture contains 100 real NumPy 2.4.3 cases.
The generator compares NumPy with an explicit sequential calculation before it saves a case.
Finite fixtures use small dyadic values where the addition orders agree.
Strict JSON stores unsigned binary64 input and output bits, with a separate NaN flag.
Integer views preserve signaling NaN input bits. Order-sensitive Rust tests remain separate.

## Author checks

The following checks passed:

- `cargo fmt --check`, `cargo check --all-targets`, and strict Clippy.
- Scaffold tests, graduated public-API tests, and all references except the exact known reshape dependency failure.
- The mean generator's fixture regeneration, reference tests, and direct NumPy comparison.
- `python3 scripts/check_authoring.py`: 4,566 saved NumPy cases and four shape rejections.
- The isolated author wrapper, including Git-trigger and bot-marker scenarios.

The full `rustlings dev check --require-solutions` still fails on missing historical TODO markers.
The affected runners are `003_full`, `004_arange`, `005_eye`, `006_reshape`, and `007_transpose`.
The author did not add comments to learner files.
A disposable metadata check excludes these files and their references only from Rustlings' marker check.
Their full contract tests and reference tests still run separately.

The checker accepts only the exact recorded reshape failure with 12 passing tests and one failure.
It requires the overflow message and the constructor source location.
Independent negative controls reject extra failures, wrong tests, wrong causes, compile errors, and wrong pass counts.
The checker normalizes only the documented public-API imports for test equality.
It routes reshape oracle cases to the public API and does not replace that runner with a reference.
The small saved reshape fixtures pass, but do not invalidate the huge-empty-shape failure.

A disposable PTY check used the sum reference, not the learner attempt.
`rustlings run 008_sum` exited successfully and displayed both:

```text
Solution for comparison: solutions/03_reductions/008_sum.rs
Next exercise: exercises/03_reductions/009_mean.rs
```

This UI check does not count as learner completion of sum.
An independent reviewer found no new-task correctness blocker.

## Reproduce

```sh
uv run --with numpy==2.4.3 scripts/generate_mean_oracle.py --check --verify-rust
PYTHONDONTWRITEBYTECODE=1 uv run --with numpy==2.4.3 validation/review_009.py
python3 scripts/check_authoring.py
cargo test --bin 006_reshape
cargo test --bin 009_mean
```

The last two commands intentionally show different states: the existing constructor regression and the new unfinished mean.
