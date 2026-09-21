# rumpy

Build a small NumPy/SciPy-style numerical library in Rust through custom Rustlings exercises.
Your verified implementations become the library used by later exercises.

## Start

Rust and Cargo are required. `rust-toolchain.toml` pins Rust 1.97.1.

```sh
cargo install rustlings --version 6.5.0 --locked
git clone git@github.com:Artur-Galstyan/rumpy.git
cd rumpy
rustlings
```

Do **not** run `rustlings init` here. That creates the official Rust course.

**Current task:** read [009 — mean](lessons/009-mean.md) and edit
`exercises/03_reductions/009_mean.rs`. Reduce every stored value to one `f64` mean.
Add left-to-right from positive zero, then divide by the stored count as `f64`.
An empty array returns NaN. NumPy can use a different addition order.
Your public `rumpy::operations::sum` passes all 11 preserved contract tests.
You may reuse it for mean. The restored mean exercise uses the current Array API without strides.
Your stride rollback resolves the reshape regression. All 13 public-API reshape tests pass.
The author changed no learner source, exports, callers, or earlier runners.
If Rustlings starts at an earlier completed exercise, check it and press `n`.
The filename prefix is the global exercise order. Matching solutions use the same prefix.
Public function names stay unnumbered.
Press `h` for hints. Save the current exercise to rerun its tests.

The runners `003_full`, `004_arange`, `005_eye`, `006_reshape`, `007_transpose`, and `008_sum` lack Rustlings' required historical markers.
Add the corresponding comment to each file without a test change:

```rust
// In exercises/01_creation/003_full.rs:
// TODO (historical, completed): implementation moved to rumpy::full.

// In exercises/01_creation/004_arange.rs:
// TODO (historical, completed): implementation moved to rumpy::arange.

// In exercises/01_creation/005_eye.rs:
// TODO (historical, completed): implementation moved to rumpy::eye.

// In exercises/02_shape/006_reshape.rs:
// TODO (historical, completed): implementation moved to rumpy::reshape.

// In exercises/02_shape/007_transpose.rs:
// TODO (historical, completed): implementation moved to rumpy::operations::transpose.

// In exercises/03_reductions/008_sum.rs:
// TODO (historical, completed): implementation moved to rumpy::operations::sum.
```

```sh
rustlings hint 009_mean
rustlings run 009_mean
# Headless checks:
cargo test --bin 009_mean
cargo test --bin 001_zeros --bin 002_ones --bin 003_full --bin 004_arange --bin 005_eye --bin 006_reshape --bin 007_transpose
```

Rustlings success output needs a real terminal. Use Cargo for headless checks.
The watcher tracks the current exercise, not `src/` or other exercises.
After dependency changes, press `c` for a full check, or use `--manual-run` with `r`.

## Exercise to library

```text
exercises/03_reductions/009_mean.rs active task: edit this
solutions/03_reductions/009_mean.rs reference: compare after your attempt
exercises/03_reductions/008_sum.rs  original tests against rumpy::operations::sum
src/operations/transpose.rs        your completed implementation
exercises/02_shape/007_transpose.rs original tests against rumpy::operations::transpose
src/operations/reshape.rs          your public implementation
exercises/02_shape/006_reshape.rs   public-API runner: all preserved tests pass
src/creation/eye.rs                your completed implementation
exercises/01_creation/005_eye.rs    original tests against rumpy::eye
src/creation/arange.rs             your completed implementation
exercises/01_creation/004_arange.rs original tests against rumpy::arange
src/creation/full.rs            your completed implementation
exercises/01_creation/003_full.rs   original tests against rumpy::full
src/creation/ones.rs            your completed implementation
exercises/01_creation/002_ones.rs   original tests against rumpy::ones
src/creation/zeros.rs           your completed implementation
exercises/01_creation/001_zeros.rs  original tests against rumpy::zeros
src/lib.rs                     public library exports
```

The library contains only verified learner implementations, never reference solutions.
`src/array.rs` supplies the owned contiguous row-major `f64` array container.
Active exercise implementations are not library modules. An unfinished exercise can
fail without breaking the completed library. `cargo test` still includes unfinished
exercise binaries and can therefore fail intentionally.

**You own graduation and refactoring.** After an exercise passes, you choose the
module layout, move your implementation into `src/`, and expose it through `lib.rs`.
You also convert the old exercise into a public-API test runner without removing
its assertions. The author does not move your functions or rewrite your library.

Refactor earlier functions when a more general operation makes shared behaviour
clear. For example, `ones` can call `full(shape, 1.0)`. If `zeros` also calls `full`,
`full` must no longer call `zeros`, or the functions recurse indefinitely.
You can group related functions in one module rather than keep one file per function.

For an extension, the author can provide a new exercise based on your current code.
Your library remains in place while you work. You decide how to merge the extension,
update callers, and preserve earlier behaviour. The lesson includes a short checklist.

The author reviews your pushed work and reports problems without fixing your code.
The author can maintain Rustlings metadata and progress records. A solved exercise
is not recorded as graduated until you move it and connect its public-API tests.
The existing `zeros` and `ones` remain where they are; this policy does not undo them.

## Reference solutions

Every exercise has a matching file under `solutions/`, including explanatory comments
and the same contract tests. Rustlings can report its path after a successful run.
You can also open it directly. These files are spoilers for comparison, not library code.

## Pull, solve, push

```sh
git pull --ff-only
# Solve the exercise, then move/refactor your code when ready.
git add exercises/ src/
git commit -m "Implement mean"
git push origin main
```

A separate Hermes job checks `main` daily at **07:30 Europe/Berlin**.
Any new learner commit triggers one next exercise per check, even if tests fail.
Several commits form one batch. Bot commits never trigger another exercise.
No new learner commit means no new exercise or routine notification.

The job reviews your attempts and your own library changes, then updates bookkeeping.
It never moves or refactors your implementations automatically. Passing tests and
completed graduation are not gates for the next exercise. The next task avoids
broken or unavailable dependencies where possible.
Unpushed work and other branches are not visible to the job.
Commit local work before a pull. If Git reports a conflict, inspect it rather than
discard your work. This job remains separate from paper spaced repetition.

## Author checks

- [Roadmap](ROADMAP.md) and [author contract](AGENTS.md).
- `.rumpy/progress.json` records active/graduated functions and the handled learner SHA.
- `cargo test --lib --test scaffold --bin 001_zeros --bin 002_ones --bin 003_full --bin 004_arange --bin 005_eye` checks the completed library and scaffold.
- `cargo test --bin 006_reshape` checks the public reshape. Its empty-shape stride regression currently fails.
- `cargo check --all-targets`, `cargo fmt --check`, and Clippy check build quality.
- `rustlings dev check --require-solutions` validates unfinished tasks and all references.

Graduated runners use `skip_check_unsolved = true` in `info.toml`. Rustlings 6.5.0 still
requires a literal `// TODO` comment in each runner, so completed runners label it historical.
Do not remove their regression tests. The author checks are not a learner completion gate.

`python3 scripts/check_authoring.py` verifies the current publication in disposable copies
(Python 3.11+, Rustlings, and Cargo required). It checks the intentionally failing new stub,
the passing reference solutions, and recorded NumPy fixtures. The script does not alter your
exercise files. After you solve the current task, its expected-red check will fail by design.

The full-repository Rustlings author check fails because `003_full`, `004_arange`, `005_eye`, `006_reshape`, `007_transpose`, and `008_sum` lack historical markers.
The checker reports this baseline error. A second disposable project excludes those runners and their references from the Rustlings check.
It still runs their full tests, their reference tests, and the public-API oracle checks separately.
It does not add markers to your source. The normal Rustlings check resumes after you add the comments.

The checker runs all original reshape assertions against the public API and the reference.
It reports the exact known stride overflow separately and rejects any additional failure.
No temporary library substitute or weaker assertion is necessary for the new task.
The unchanged public reshape still passes the saved NumPy cases. Those small cases do not disprove the huge-empty-shape regression.
The transpose fixture contains 174 real NumPy 2.4.3 cases. Its 14 contract tests cover unsupported ranks and huge empty axes.
The mean stub deliberately fails at `todo!`. Its matching reference passes all 13 identical contract tests.
The mean fixture contains 100 real NumPy 2.4.3 cases where NumPy agrees with the explicit sequential order.
The earlier sum reference passes its 11 tests and 100 saved NumPy cases. Its learner stub remains unfinished.
Order-sensitive inputs have separate Rust tests. The checker normalizes only documented public-API imports.
All original assertions remain unchanged. See [the publication review](validation/009-authoring.md) for the checks and baseline failures.
