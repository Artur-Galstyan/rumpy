# Exercise 004 author checks

## Learner review

The reviewed learner commit is `059a926cc70f04c2d6797ddb53012df9fe67c5f9`.
The parent publication recorded `ca1d4b1f29d700a77f34c8bad8f5363bdddd3675`.

The learner moved `full` into `src/creation/full.rs`, exposed `rumpy::full`, and connected the preserved tests to that API.
The learner changed the module entry to `src/creation.rs` and made `ones` call `full`.
The dependency direction is `ones` → `full` → `zeros`, without a cycle.
The author left all learner implementations and earlier runners unchanged.

The pushed baseline passed formatting, build checks, Clippy, all Cargo tests, and all existing references.
The `full` tests include negative zero, quiet and signaling NaN payloads, infinities, finite extremes, and separate storage.
The regenerated NumPy 2.4.3 `full` fixture was byte-identical to the saved fixture.
The final oracle check tests graduated operations through their public library APIs.

## Known baseline error

The raw command `rustlings dev check --require-solutions` fails with:

```text
Error: Didn't find any `// TODO` comment in the file `exercises/01_creation/003_full.rs`.
You need to have at least one such comment to guide the user.
```

The learner must add this historical comment to that runner:

```rust
// TODO (historical, completed): implementation moved to rumpy::full.
```

The author did not add the comment or change the runner.
`skip_check_unsolved = true` correctly marks the completed task, but does not remove Rustlings' marker requirement.

The checker reports this exact baseline error before a separate Rustlings check in a disposable copy.
That copy omits only the affected entry from `info.toml` and moves its unchanged runner and reference outside Rustlings' directories.
It then runs `rustlings dev update` and `rustlings dev check --require-solutions`.
The full regression runner, matching reference, and public-API oracle still pass separate checks.
No assertion is removed or weakened. This is not a claim that the full-repository Rustlings author check passes.

## New publication

The single new task is `004_arange`. It accepts explicit `i32` endpoints and a positive `i32` step.
It returns owned one-dimensional `f64` data. The lesson states the supported subset and omitted NumPy features.
The caller limits output to 100,000 elements. The tests focus on interval, overflow, shape, ownership, and invalid-step semantics.
The reference widens integer arithmetic before an increment. The exercise and reference contain identical contract tests.

The reproducible oracle generator produced 1,450 actual NumPy 2.4.3 range cases.
The fixtures cover a small endpoint grid plus wide-span, boundary, and empty intervals.
The combined check passed 1,583 saved NumPy cases with shape, size, and exact value-bit comparisons.
Each generated Rust case uses a separate lexical scope to avoid deep debug-info recursion from repeated shadow declarations.

## Execution results

The following checks passed:

- `cargo fmt --check`, `cargo check --all-targets`, and `cargo clippy --all-targets -- -D warnings`.
- The library, scaffold, and `001_zeros`, `002_ones`, and `003_full` public-API runners.
- Explicit reference formatting and every `*_sol` Cargo test binary.
- The expected TODO failure for `004_arange`, then its reference with identical tests in a disposable copy.
- `python3 scripts/check_authoring.py` and the external `author_check.py --repo ...` wrapper, with the baseline error reported separately.

The external wrapper also passed Git-trigger tests for batches, bot suppression, acknowledgment, spoofed markers, and stale cursors.
The independent reviewer found no security or logic errors and independently confirmed the arange fixtures against NumPy 2.4.3.
The reviewer suggested an optional test at the 100,000-element scope limit. The current fixtures reach 25 elements.

A PTY smoke test with `script -q /dev/null rustlings run 003_full` returned:

```text
Successfully ran exercises/01_creation/003_full.rs
Solution for comparison: solutions/01_creation/003_full.rs
Next exercise: exercises/01_creation/004_arange.rs
```

The new stub intentionally fails. This publication does not claim that all exercise tests pass.
