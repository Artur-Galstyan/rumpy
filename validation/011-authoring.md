# 011 publication review

The publication adds `011_argmax` after learner commit `9fd675d5647b181fb0595fb62f66092c1aeed8f8`.

## Learner review

The learner added `src/operations/amax.rs`, exposed `rumpy::amax`, and connected the original runner to that API.
All 10 original contract tests pass. The assertions remain byte-identical after the recorded public-import normalization.
The public API passes 125 saved NumPy cases and 5 empty cases.
An independent disposable review also passed 10 additional edge cases in debug and optimized builds.
The fixture reproduced with NumPy 2.4.3.

The negative-infinity initial value satisfies the documented `f64` value-return contract.
Strict comparisons retain the first equal maximum, including both zero signs.
Explicit NaN checks prevent an earlier infinity from hiding a later NaN.
No numerical defect was found. The progress record now records learner-owned graduation.

Before author edits, `cargo fmt --check`, `cargo check --all-targets`, Clippy with denied warnings, and all-target tests passed.
The author changed no learner source, exports, callers, earlier runners, scaffold tests, or `Cargo.lock`.

## New contract

`argmax(&Array) -> usize` returns a flat row-major index at every rank.
Any NaN selects the first NaN index. Without NaNs, equal maxima retain the first index.
Both zero signs tie. Empty input panics with exactly `argmax requires at least one element`.
The lesson states the supported subset and the learner-owned library checklist.

The stub compiles and fails at its intended `todo!`.
The separate reference passes all 13 identical contract tests in debug and optimized builds.
The tests include signaling NaNs, unchanged input bits, and huge empty shapes under a timeout.
No reference entered `src/` or replaced a learner attempt.

Real NumPy 2.4.3 produced 183 nonempty index cases and 5 empty rejections.
The strict JSON fixture contains raw binary64 input bits and integer output indices.
Unsigned integer views preserve signaling NaN inputs.
The generator reproduces the fixture and verifies the Rust reference in a disposable directory:

```sh
uv run --with numpy==2.4.3 scripts/generate_argmax_oracle.py --check --verify-rust
```

## Publication checks

`rustlings dev update` added only the two new top-level Cargo bin entries.
The repository checker and the external author-check wrapper passed:

```sh
python3 scripts/check_authoring.py
python3 ~/.hermes/rumpy/author_check.py --repo <isolated-author-clone>
```

The checks cover formatting, all-target Cargo checks, Clippy with denied warnings, scaffold tests, all graduated runners, every reference, and the intended stub failure.
The checker explicitly compares `argmax` integer results instead of treating them as `f64` or `Array` outputs.
It passed 4,874 saved nonempty/general NumPy cases, 4 shape rejections, and 10 empty reduction rejections.
The wrapper also passed Git-trigger, batch, bot-marker, acknowledgement, and spoofing tests.

## Known baseline

The full-repository Rustlings author check still rejects missing historical TODO markers.
The affected runners are `003_full`, `004_arange`, `005_eye`, `006_reshape`, `007_transpose`, `008_sum`, `009_mean`, and `010_amax`.
README contains the exact comments for the learner to add. The author left these runners untouched.

A second disposable metadata check excludes those runners and their references from Rustlings only.
Their preserved assertions, reference tests, and public-API oracle checks still run separately.
That isolated `rustlings dev check --require-solutions` passes.
This result does not make the full-repository Rustlings baseline green.

## Independent review

A separate reviewer approved the contracts, code, fixtures, metadata, and protected-source boundaries.
The reviewer repeated the publication checker and fixture reproduction in disposable copies.
Negative controls rejected last-tie selection, ignored NaNs, and incorrect indices.

A disposable PTY smoke test ran `rustlings run 010_amax` successfully.
It displayed `solutions/03_reductions/010_amax.rs` and the next exercise `exercises/03_reductions/011_argmax.rs`.
The reviewer found a missing link target for this report. This file resolves that documentation issue.

The new unfinished exercise is intentionally red. Completed learner tests and reference tests pass.
