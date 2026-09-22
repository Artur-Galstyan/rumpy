# 010 publication review

## Learner baseline

Reviewed learner commit `f5ce2d5ce1684c635a6d30138073b1cc9b6e1ec6` in an isolated clone.
The author did not use or edit the learner checkout.

`mean` now lives in `src/operations/mean.rs` and is public as `rumpy::mean`.
Its runner calls that public API. Only the documented grouped import differs from the original test module.
All 13 preserved contract tests pass. The public function also matches all 100 saved NumPy mean cases.
The root exports for `sum` and `transpose` retain their original implementations.
Mean graduation records the learner's changes, not an author move.

Before author edits, these checks passed against the pushed learner version:

- `cargo fmt --check`
- `cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`, including all earlier exercise and reference tests and the scaffold

The full Rustlings author check reports a missing historical `// TODO` comment.
The runners `003_full` through `009_mean` lack the required marker.
The README gives the learner the exact comments and paths. No runner was changed to hide this error.
There are no current numerical baseline failures.

## New maximum contract

`010_amax` reduces all stored values to one `f64` maximum.
Any NaN produces NaN by class. Empty input panics with exactly `amax requires at least one element`.
Equal maxima retain the first stored value, including signed-zero ties.
Non-NaN results preserve the selected bits. The input remains unchanged.

NumPy does not specify mixed-zero tie bits. This Rust tie rule is a deliberate, explicit extension.
The oracle excludes mixed-zero signs. Dedicated Rust assertions cover both mixed-zero orders.
NumPy raises `ValueError` for empty input, while the Rust contract specifies a panic.
No arithmetic, result allocation, shape product, or axis-length traversal is necessary.

The lesson states the supported subset and the omitted NumPy parameters.
The reference remains separate from the unfinished exercise and the public library.

## Publication checks

`rustlings dev update` added the exercise and solution bins to Cargo's literal top-level list.
Cargo.lock remains unchanged.

`python3 scripts/check_authoring.py`, through the external `author_check.py --repo` wrapper, passed:

- Formatting, all-target compilation, Clippy, scaffold tests, and every graduated public-API runner.
- The expected `010_amax` TODO failure and all reference tests with identical normalized contract assertions.
- An isolated Rustlings author check after exclusion of the unchanged missing-marker runners and their references from that check only.
- All 4691 saved NumPy value cases, four shape rejections, and five empty-maximum rejections.
- Git-trigger tests for batches, bot suppression, acknowledgment, separated markers, spoofing, and stale cursors.

The checker still runs the excluded runners, their references, and their public-API oracle cases separately.
The full-repository `rustlings dev check --require-solutions` remains blocked by the historical markers.
The new unfinished stub intentionally fails. This publication does not claim a green whole-suite learner run.

`uv run --with numpy==2.4.3 scripts/generate_amax_oracle.py --check --verify-rust` passed.
The real NumPy 2.4.3 fixture contains 125 scalar cases and five empty cases, with seed 10010.
The generator proved that the stub compiles and fails at its intended TODO.
The reference passed all 10 contract tests in debug and optimized builds.
Each run checks huge empty shapes under a timeout, exact panic text, NaN classes, and input bits.
The generator confirmed exact test-module equality and reproduced every saved maximum fixture.
`generate_mean_oracle.py --check` also reproduced the 100 saved mean cases.

A disposable PTY run of `rustlings run 009_mean` succeeded and displayed:

```text
Solution for comparison: solutions/03_reductions/009_mean.rs
Next exercise: exercises/03_reductions/010_amax.rs
```

The `rustlings hint 010_amax` command displayed the new progressive hints.
No learner state file changed during the PTY check.

## Independent review

A separate read-only reviewer found no blocking issues in the learner mean or the new publication.
The reviewer confirmed the preserved mean assertions, public API connection, maximum semantics, fixtures, and author boundaries.
Its additional disposable public-mean harness passed 1231 numerical cases and three huge-empty shapes.
The review included exact-rational dyadic cases, order-sensitive permutations, and IEEE edge cases.
The parent separately reran the saved-fixture generator and full publication checker.

## Learner ownership

No author change touches `src/`, earlier exercise runners, learner imports or exports, or caller code.
No reference was installed in the public library. Earlier solutions and lessons remain in place.
Only the learner chooses later module layout, API exports, caller changes, and shared-helper refactors.
