# 006 publication checks

## Learner baseline

The reviewed learner commit is `dc4a6584102d3a8b4468badf6e9affde8bf74ef1`.
The author used an isolated clone. The learner checkout remained untouched.

The learner added `src/creation/eye.rs`, exported `rumpy::eye`, and connected the old runner to that API.
All 9 original eye tests remain intact after normalization of the public-API import.
The earlier runners and their assertions remain unchanged.

The baseline passed `cargo test --all-targets`, `cargo fmt --check`, `cargo check --all-targets`, and strict Clippy.
An independent review compared the public eye API with actual NumPy 2.4.3.
It passed 1,209 shapes and 24,811,424 bit-exact values, including the 1,000-by-1,000 boundary.
No eye correctness issue appeared within its documented scope.
The progress record now records the learner's graduation. The author moved no code.

## New exercise

`006_reshape` adds the owned-copy subset of `numpy.reshape(a, shape, order='C', copy=True)`.
It uses the existing `Array` and `ShapeError` types, with no dependency or source-library change.
Its 13 tests cover shape changes, ownership, float bits, exact errors, and scaffold overflow rules.
The reference and exercise contain identical contract tests.
The new stub compiles and fails at its intended `todo!`. The reference passes all 13 tests.

`scripts/generate_reshape_oracle.py` produced 2,434 successful cases and 4 shape rejections with actual NumPy 2.4.3.
The saved fixture uses unsigned binary64 bits. Its generator uses direct NumPy bit views, not JSON NaNs.
Scaffold-specific `usize` overflow and signaling-NaN copy semantics have separate Rust tests.
An independent reviewer regenerated the fixture and confirmed an exact match.

## Checks

The following checks passed in the author clone or disposable verification copies:

- `rustlings dev update`, with the existing top-level Cargo bin list and lockfile preserved.
- `cargo fmt --check`, `cargo check --all-targets`, and `cargo clippy --all-targets -- -D warnings`.
- The library, scaffold, all five public-API runners, and all six references.
- `python3 scripts/check_authoring.py`, including the expected-red stub check and unchanged contract assertions.
- `python3 ~/.hermes/rumpy/author_check.py --repo <isolated-clone>`, including Git-trigger tests.

The oracle checker passed 4,192 saved successful NumPy cases and 4 saved shape rejections.
The all-target test suite passed only in a disposable copy with the active reference in place of the stub.
No reference entered `src/` or replaced the published learner exercise.
The published stub remains intentionally unfinished, so ordinary all-target tests are not all green.

The first oracle compile exceeded the existing 180-second timeout with one large test function.
The checker now divides all cases into 64-case test functions. Each case retains its own lexical scope.
The repeated check passed without a timeout increase, compiler-setting change, or dropped case.

## Rustlings baseline error

The complete `rustlings dev check --require-solutions` still fails on missing historical `// TODO` markers.
The affected runners are `003_full`, `004_arange`, and `005_eye`.
Their public-API tests pass. Their metadata skips only the unsolved author check.
The author did not insert comments into learner files.

The checker reports that baseline error explicitly.
A second disposable project excludes only those runners and their references from the Rustlings metadata check.
That isolated Rustlings check passes. The full tests, reference tests, and public-API oracles still run separately.
The README and lesson explain the required learner-owned comment edits.

A PTY smoke test with `script -q /dev/null rustlings run 005_eye` succeeded.
It displayed `solutions/01_creation/005_eye.rs` and the next exercise `exercises/02_shape/006_reshape.rs`.
`rustlings hint 006_reshape` displayed the new progressive hints.

## Independent review

A separate reviewer approved the publication with the documented historical-marker exception.
The review found no new blockers, complete answers in hints, or learner-source changes.
`git diff --check` passed. The new exercise keeps the next global sequence number across topic folders.
