# 012 author verification

## Scope and learner review

Base: `5dd5bcb26f9c39ea61b4623e94ceb9cc9914c208`.
Exactly one new exercise: `012_add`, same-shape elementwise addition.
The author prepared this exercise without a commit or push. Independent parent review then approved the contract, reference, tests, fixture generator, metadata, and learner-code boundaries. The parent reran the complete publication checker, reproduced the NumPy fixture with an identical SHA-256, and reran the optimized reference tests. All checks passed apart from the documented historical-marker baseline.

The argmax attempt and public copy use a strict comparison, retain the first equal maximum,
and return the first NaN index. Negative infinity is a valid initial value because index zero
remains correct for an all-negative-infinity input. No numerical defect appeared within the contract.
All 13 preserved tests pass locally and against the public API. A disposable test module changes
only `use super::argmax;` to `use rumpy::argmax;`. The real runner remains untouched and still tests
its local function. Progress therefore records `solved_pending_regression_connection`, not graduation.
The public API passes all 183 saved NumPy index cases and 5 empty cases. The fixture regenerated
with actual NumPy 2.4.3 and matched the saved data. Debug and release checks pass.

No learner implementation, export, caller, prior runner, prior reference, or Cargo.lock changed.
No cron setting, profile, or global configuration changed. The stale schedule prose in existing
files was not treated as permission to alter the live job.

## Baseline before edits

- `cargo fmt --check`: passed.
- `cargo check --all-targets`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo test --all-targets`: all learner runners, references, library and scaffold passed.
- `rustlings dev check --require-solutions`: failed on the existing missing TODO in `003_full`.
- `python3 scripts/check_authoring.py`: failed its old expected-red assertion because
  `011_argmax` now passes all 13 tests. This is obsolete publication metadata, not a learner defect.

The missing markers are `003_full`, `004_arange`, `005_eye`, `006_reshape`, `007_transpose`,
`008_sum`, `009_mean`, `010_amax`, and `011_argmax`. The author did not add them to learner runners.
The checker tests solved-active runners and their public copies before permitting that narrow baseline.
It excludes the unchanged baseline runners and matching references only from a second disposable
Rustlings metadata check. Their full tests and reference tests run separately.

## New lesson contract

`pub fn add(a: &Array, b: &Array) -> Array` requires identical shapes and returns independent storage.
Unequal shapes panic with exactly `add requires identical shapes`, before any empty shortcut.
A panic keeps this lesson independent of shared scaffold edits or a new shared error type.
Broadcasting is deliberately deferred. Non-NaN result bits must match ordinary pairwise f64 addition.
NaN result payloads are unspecified. Input NaN bits remain unchanged.

The stub and reference have byte-identical test modules. The 12 tests cover pair correspondence,
row-major order, scalars, singleton axes, higher rank, equal empty arrays, huge empty dimensions,
signed zeros, subnormals, rounding, overflow, infinities, NaNs, input immutability, output independence,
aliasing of the two borrowed inputs, and unequal shapes including broadcastable pairs.

The stub compiles and fails all 12 tests at the intended `todo!("add")`.
The reference passes all 12 tests in debug and release.
Five negative controls fail assertions rather than compilation: count-only validation, a zero seed,
subtraction, flattened output shape, and an empty shortcut before validation.
A sixth control proves the checker rejects a missing add fixture.

## Actual oracle evidence

`target/author-venv/bin/python validation/generate_add_numpy.py` used NumPy 2.4.3.
The generator produced 141 cases and 909 output values. A second run produced identical bytes.
SHA-256: `886427ba1d9299d59394e7d87565260c737db84ebf704b63e1e8c5be64e831fb`.
The system Python initially lacked NumPy. A venv under the clone's ignored `target/` resolved this.
No package was installed globally.

`target/author-venv/bin/python scripts/generate_argmax_oracle.py --check` passed:
`Verified 183 scalar and 5 empty NumPy 2.4.3 cases; seed 10011`.
The prior generator's `--verify-rust` option assumes an unfinished argmax and was not used.
The current checker and `validation/review_012.py` test the solved public copy instead.

The final author wrapper passed all 5015 saved NumPy cases in both debug and release,
plus 4 shape rejections and 10 empty-reduction cases. The add oracle checks both input shapes
and all original bits after the call. No NumPy rejection is claimed for broadcastable mismatches.
Rust-only tests cover the deliberately narrower shape contract and huge empty dimensions.

## Reproduction

```sh
python3 scripts/check_authoring.py
python3 validation/review_012.py
python3 validation/pty_012.py
python3 /Users/arturgalstyan/.hermes/rumpy/author_check.py --repo "$PWD"
```

The PTY smoke test runs the solved `011_argmax` in a disposable project. Rustlings reports
`solutions/03_reductions/011_argmax.rs` and then `exercises/04_elementwise/012_add.rs`.
The author wrapper also passes Git-trigger cases for batches, bot suppression, acknowledgement,
separated markers, spoofing, and stale cursors.
`git diff --check` passes. Source and old-runner preservation checks report no diff.

The final remote probe remains pending at the exact base SHA. No other author acknowledged the batch.
The parent must recheck the probe, independently review, then commit and publish if acceptable.

## Captured author output

```text
PASS cargo fmt --check
PASS cargo check --all-targets
PASS cargo clippy --all-targets -- -D warnings
PASS cargo test --lib --test scaffold
PASS cargo test --bin 001_zeros
PASS cargo test --bin 002_ones
PASS cargo test --bin 003_full
PASS cargo test --bin 004_arange
PASS cargo test --bin 005_eye
PASS cargo test --bin 006_reshape
PASS cargo test --bin 007_transpose
PASS cargo test --bin 008_sum
PASS cargo test --bin 009_mean
PASS cargo test --bin 010_amax
PASS cargo test --bin 011_argmax
PASS cargo test --test author_011_argmax_public
PASS expected TODO failure: cargo test --bin 012_add
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/01_creation/001_zeros.rs
PASS cargo test --bin 001_zeros_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/01_creation/002_ones.rs
PASS cargo test --bin 002_ones_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/01_creation/003_full.rs
PASS cargo test --bin 003_full_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/01_creation/004_arange.rs
PASS cargo test --bin 004_arange_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/01_creation/005_eye.rs
PASS cargo test --bin 005_eye_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/02_shape/006_reshape.rs
PASS cargo test --bin 006_reshape_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/02_shape/007_transpose.rs
PASS cargo test --bin 007_transpose_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/03_reductions/008_sum.rs
PASS cargo test --bin 008_sum_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/03_reductions/009_mean.rs
PASS cargo test --bin 009_mean_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/03_reductions/010_amax.rs
PASS cargo test --bin 010_amax_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/03_reductions/011_argmax.rs
PASS cargo test --bin 011_argmax_sol
PASS rustfmt --edition 2024 --check /var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-author-check-k6zcjg5g/repo/solutions/04_elementwise/012_add.rs
PASS cargo test --bin 012_add_sol
BASELINE ERROR: missing historical // TODO in ['003_full', '004_arange', '005_eye', '006_reshape', '007_transpose', '008_sum', '009_mean', '010_amax', '011_argmax']
PASS rustlings dev update
PASS rustlings dev check --require-solutions
PASS isolated Rustlings metadata check; full-repository baseline error remains
PASS cargo test --lib --test scaffold
PASS cargo test --bin 001_zeros
PASS cargo test --bin 001_zeros_sol
PASS cargo test --bin 002_ones
PASS cargo test --bin 002_ones_sol
PASS cargo test --bin 003_full
PASS cargo test --bin 003_full_sol
PASS cargo test --bin 004_arange
PASS cargo test --bin 004_arange_sol
PASS cargo test --bin 005_eye
PASS cargo test --bin 005_eye_sol
PASS cargo test --bin 006_reshape
PASS cargo test --bin 006_reshape_sol
PASS cargo test --bin 007_transpose
PASS cargo test --bin 007_transpose_sol
PASS cargo test --bin 008_sum
PASS cargo test --bin 008_sum_sol
PASS cargo test --bin 009_mean
PASS cargo test --bin 009_mean_sol
PASS cargo test --bin 010_amax
PASS cargo test --bin 010_amax_sol
PASS cargo test --bin 011_argmax
PASS cargo test --bin 011_argmax_sol
PASS cargo test --bin 012_add
PASS cargo test --bin 012_add_sol
PASS cargo test --bin rumpy
PASS cargo test --test author_numpy_oracle
PASS cargo test --release --test author_numpy_oracle
PASS 5015 saved NumPy cases in debug and release, including array shapes, exact non-NaN bits, reduction NaN classes, and argmax indices
PASS 4 saved NumPy shape rejections with exact scaffold errors
PASS 10 saved NumPy empty reductions with exact Rust panic text
PASS publication checks with any BASELINE ERROR above reported separately. Learner files remain untouched.
PASS Git-trigger scenarios: batches, bot suppression, acknowledgment, separated markers, spoofing, stale cursors
```

## Captured extra checks

```text
PASS  cargo test --test review_public_argmax --bin 012_add_sol
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS  cargo test --release --test review_public_argmax --bin 012_add_sol
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS  cargo test --test review_argmax_oracle
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS  cargo test --release --test review_argmax_oracle
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS public argmax: 183 saved NumPy index cases in both modes
PASS negative control cargo test --bin 012_add
test result: FAILED. 0 passed; 12 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS negative control cargo test --bin 012_add
test result: FAILED. 11 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS rejected count-only-validation: ['tests::unequal_shapes_panic_exactly_before_any_empty_shortcut']
PASS negative control cargo test --bin 012_add
test result: FAILED. 11 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS rejected zero-seed: ['tests::signed_zeros_and_exact_cancellation']
PASS negative control cargo test --bin 012_add
test result: FAILED. 3 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS rejected subtraction: ['tests::scalar_shape_contains_one_sum', 'tests::rounding_subnormals_and_overflow_follow_f64_addition', 'tests::inputs_and_outputs_own_independent_storage', 'tests::higher_rank_keeps_singleton_axes', 'tests::infinities_and_nans_use_classes_not_nan_payloads', 'tests::signed_zeros_and_exact_cancellation', 'tests::matrix_preserves_row_major_order', 'tests::same_array_can_supply_both_inputs', 'tests::vectors_add_each_corresponding_pair']
PASS negative control cargo test --bin 012_add
test result: FAILED. 7 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS rejected flattened-output: ['tests::matrix_preserves_row_major_order', 'tests::scalar_shape_contains_one_sum', 'tests::higher_rank_keeps_singleton_axes', 'tests::huge_empty_axes_need_no_axis_traversal_or_products', 'tests::equal_empty_shapes_keep_all_axes']
PASS negative control cargo test --bin 012_add
test result: FAILED. 11 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
PASS rejected empty-before-validation: ['tests::unequal_shapes_panic_exactly_before_any_empty_shortcut']
PASS missing-fixture negative control
```

## Captured PTY output

```text
Welcome to rumpy: build a NumPy/SciPy-style library in Rust.
Your argmax passes locally and through its public API; its old runner still tests the local copy.
Read lessons/012-add.md, then edit exercises/04_elementwise/012_add.rs. Press h for hints.
Add matching pairs into a new Array. Require identical shapes; no broadcasting yet.

Press ENTER to continue 


Output

✓ Successfully ran exercises/03_reductions/011_argmax.rs

Solution for comparison: ]8;;file:///private/var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-012-pty-preec0s7/repo/solutions/03_reductions/011_argmax.rs\solutions/03_reductions/011_argmax.rs]8;;\

Next exercise: ]8;;file:///private/var/folders/9d/2g2x7lk54pzgqfdl3h3474b00000gn/T/rumpy-012-pty-preec0s7/repo/exercises/04_elementwise/012_add.rs\exercises/04_elementwise/012_add.rs]8;;\

PASS Rustlings solution path and next exercise 012_add in disposable PTY
```
