# 015 author review

- Trigger: learner commit `ca7fb8d4d4ad79bc357030401e81d5398c838331`.
- Learner subtract: 8 unchanged local tests pass. An unchanged copy of the tests passes against `rumpy::subtract` in a disposable integration test. Its Rayon branch passes 1,000,001 ordered differences in local and public copies. The original runner still imports its local function, so subtract is solved but not graduated. Its historical TODO marker remains learner-owned.
- The new broadcast_to stub compiles and fails at `todo!("broadcast_to")`. Its separate answer passes all 6 matching contract tests, including empty huge axes, rank rejection, input-bit preservation, and nonempty target-size overflow.
- Real NumPy 2.5.3 generated 130 broadcast_to cases with 670 bit-preserved output values. The fixture selects unsigned 64-bit patterns directly, including signaling NaNs and signed subnormals. The checker compares 5,438 saved NumPy cases in debug and release, including all earlier operations.
- `cargo fmt --check`, `cargo check --all-targets`, and `cargo clippy --all-targets -- -D warnings` pass. The scaffold, graduated runners, active local runners, active public-copy tests, and reference tests pass. The checker passes its isolated Rustlings metadata and Git-trigger scenarios.
- Full `rustlings dev check --require-solutions` still stops on the older historical TODO baseline. The checker detects all 12 missing markers and tests unchanged runners outside the isolated Rustlings metadata check. It does not edit learner code. The new `015_broadcast_to` file has its required TODO marker.
- The array contract uses owned contiguous row-major `f64`. No learner implementation, old runner, or public API file changed in this publication.
