# Mean restoration after sum

Reviewed learner commits 829210ce6f001b3726ee053fa18c26b0be39d34a and f7f655c6f2bb5e640098304c1efd83abbcd9c91d.

The learner requested restoration of 009_mean instead of another operation.
Restore its unfinished exercise and lesson from 2970c0f, retaining the sequence number.
The learner removed Array strides. Remove only the obsolete stride snapshot and assertion from the restored mean test helper, matching solution, and mean oracle harness. All numerical, shape, and data-bit assertions remain intact.
Allow reuse of the learner's public rumpy::operations::sum, while the independent reference remains self-contained.
No src file, earlier exercise runner, learner implementation, or cron configuration changed.

## Baseline

- cargo check --all-targets and cargo fmt --check failed because the 009_mean source was deleted while Cargo still declared it.
- 008_sum: all 11 original public-API tests passed.
- 006_reshape: all 13 original public-API tests passed. The stride rollback resolved the prior constructor overflow.
- Both operations now qualify as graduated. The author did not move either implementation or connect either runner.

## Validation

- rustlings dev update completed without a Cargo change.
- cargo fmt --check, cargo check --all-targets, and cargo clippy --all-targets -- -D warnings passed.
- python3 scripts/check_authoring.py passed all scaffold, graduated runner, reference, and numerical checks. It verified 4566 saved NumPy cases and 4 shape rejections.
- The restored mean stub compiles and fails at its intended todo!("mean"). It is deliberately unfinished, not a passing learner implementation.
- The mean reference passed all 13 identical contract tests.
- uv run --with numpy==2.4.3 scripts/generate_mean_oracle.py --check --verify-rust regenerated and verified all 100 saved mean cases against actual NumPy 2.4.3 and Rust.
- Rustlings full-repository author validation still reports missing historical // TODO markers in 003_full, 004_arange, 005_eye, 006_reshape, 007_transpose, and 008_sum. These are learner-owned edits. The checker preserves their full tests and separately passes an isolated metadata check that excludes only those known marker failures.

Independent read-only review approved the restoration with no blockers. It separately verified the unchanged learner files, identical adapted mean test modules, sum and reshape graduation records, passing focused tests, and the intended mean TODO failures.

The tracked rust_out binary came from the learner and remains untouched.
validation/review_009.py is the historical stride review for the prior commit, not a current validation command after the learner removed strides.
