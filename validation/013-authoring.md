# 013 author verification

## Learner batch

The handled learner commits are `ee8655ca8d29f61845ed1ea47c38962b5bb48aef` and `19526d16186bb607ee369e9659961f8d7d2c4551`. The learner moved `add` into `src/operations/add.rs`, exported `rumpy::add`, and connected all 12 preserved assertions to that public API. The original runner lacks a historical `// TODO` marker. The author did not edit its source. The earlier `011_argmax` runner still tests a local function and remains solved but not graduated.

The learner added Rayon and an ordered parallel path above 1,000,000 values. A separate Rust check passed 1,000,001 values, checked every output bit in order, and checked that both inputs kept their bits. The prior 141 saved NumPy add cases passed. No numerical defect was found in the documented subset.

## New task

`013_multiply` uses identical-shape, owned, row-major `f64` arrays. It makes independent output storage, rejects all unequal shapes before empty handling, and does not broadcast. The exercise and reference share the same 11 tests. The stub compiles and fails at the intended `todo!("multiply")`. The reference passes all 11 tests in debug and release. The signed-zero, infinity, NaN, subnormal, scalar, huge-empty, and shape-mismatch cases remain explicit.

NumPy 2.4.3 made 141 saved cases with 909 output values. The author checker tests every case against the reference in debug and release. It also tests all earlier saved cases against their library or active public APIs. The full count is 5,156 cases, plus four shape rejections and ten empty-reduction cases. NaN outputs use class checks. Other outputs use exact binary64 bits. `validation/generate_multiply_numpy.py` regenerates the fixture.

## Checks

- `cargo fmt --check`, `cargo check --all-targets`, and `cargo clippy --all-targets -- -D warnings` passed.
- Library, scaffold, all graduated runners, solved-active argmax, and all matching references passed. The new stub failed at its intended TODO.
- `python3 scripts/check_authoring.py` passed, apart from the known Rustlings historical-marker baseline. `python /Users/arturgalstyan/.hermes/rumpy/author_check.py --repo "$PWD"` also passed its Git-trigger cases.
- The full `rustlings dev check --require-solutions` stops at the existing missing marker in `003_full`. A disposable copy excludes only the ten historical-marker runners from that metadata check, and the author checker still runs every one of their unchanged tests and references. The isolated Rustlings metadata check passed.
- `python3 validation/pty_013.py` passed. Rustlings showed the `012_add` reference path and selected `013_multiply` next.
- An independent reviewer found a `012_add` test-module whitespace mismatch against its reference. A whitespace-only edit to that reference restored byte-exact assertion parity. The reviewer also identified the missing report link, which this document supplies. No learner code, callers, exports, existing runners, or `Cargo.lock` changed in this publication.

The public `add` function is graduated. The new `multiply` task is unfinished by design. The ten historical-marker edits remain with the learner.
