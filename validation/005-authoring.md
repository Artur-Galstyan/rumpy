# Exercise 005 author checks

## Learner review

The reviewed learner commit is `3c799f23a6addad69d4dbb62362513dcfcd41a48`.
The previous handled commit is `059a926cc70f04c2d6797ddb53012df9fe67c5f9`.

The learner added `src/creation/arange.rs`, exposed `rumpy::arange`, and connected the original regression assertions to that API.
The author recorded graduation only after tests through this public API passed.
The author left `src/`, all earlier exercise files, scaffold tests, and `Cargo.lock` unchanged.

The positive-step check precedes the empty-interval return. The arithmetic widens both endpoints before subtraction and iteration.
The computed length matches the half-open positive-step sequence within the documented scope.
No correctness issue was found within that scope.

The pushed baseline passed formatting, all-target compilation, Clippy, scaffold tests, and all four public-API exercise runners.
The arange runner passed its 11 original tests.
An independent disposable Cargo test checked the public API against 1,652 actual NumPy 2.4.3 cases.
Those cases included the saved grid, 200 deterministic random endpoint cases, and two 100,000-element boundary cases.
The random seed was 417. Endpoints used the full i32 range, and positive random steps ranged from 100,000 to i32::MAX.
The two large cases were `(0, 100000, 1)` and `(-100000, 0, 1)`.
All 1,450 saved arange cases also matched fresh NumPy output.

## Known baseline errors

The raw `rustlings dev check --require-solutions` command fails at the missing historical marker in `003_full`.
Inspection also confirms the missing marker in `004_arange`.
Both runners still pass their unchanged public-API tests.
The learner owns these comment edits, so the author did not add them.

```rust
// In exercises/01_creation/003_full.rs:
// TODO (historical, completed): implementation moved to rumpy::full.

// In exercises/01_creation/004_arange.rs:
// TODO (historical, completed): implementation moved to rumpy::arange.
```

Metadata skips only the solved-state author check for completed exercises, not their tests.
The repository checker reports the baseline errors and excludes those two metadata entries in a second disposable Rustlings project.
It moves the unchanged runner and reference files outside that copy's Rustlings directories.
It still checks every reference, every original regression runner, and every public-API oracle separately.
No assertion is removed or weakened. The full-repository Rustlings author check is not green.

## New publication

Exactly one new exercise is present: `005_eye`.
The supported API is `eye(rows: usize, cols: usize) -> Array` for dimensions in `0..=1000`.
It returns an owned row-major f64 matrix with a main diagonal of ones and positive zeros elsewhere.
The contract covers rectangular matrices, empty axes, unit axes, independent storage, and the supported dimension boundary.
The lesson states omitted NumPy options and includes the learner-owned library checklist.
The separate commented reference contains identical contract tests.

The reproducible generator wrote 175 actual NumPy 2.4.3 eye cases.
The combined checker passed 1,758 saved NumPy cases with shape, size, and exact-bit comparisons.
Completed operations were checked through their public library APIs, not substitute references.

## Execution results

The following checks passed:

- `cargo fmt --check`, `cargo check --all-targets`, and `cargo clippy --all-targets -- -D warnings`.
- Library and scaffold checks, all four public-API runners, and all five reference binaries.
- Explicit reference formatting and identical contract-test checks.
- The expected TODO failure for `005_eye`, followed by its successful reference tests in a disposable copy.
- `python3 scripts/check_authoring.py` and the external `author_check.py --repo ...` wrapper, with baseline errors reported separately.

The external wrapper also passed its Git-trigger cases for batches, bot suppression, acknowledgment, spoofed markers, and stale cursors.
The independent reviewer approved the publication with no blockers and independently reran the repository checker.
That reviewer also reproduced the eye fixture byte-for-byte with NumPy 2.4.3.

A PTY smoke test with `script -q /dev/null rustlings run 004_arange` showed:

```text
Successfully ran exercises/01_creation/004_arange.rs
Solution for comparison: solutions/01_creation/004_arange.rs
Next exercise: exercises/01_creation/005_eye.rs
```

`rustlings hint 005_eye` displayed the new progressive hints.
The new stub intentionally fails. This publication does not claim that all exercise tests pass.
