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

**Current task:** read [004 — arange](lessons/004-arange.md) and edit
`exercises/01_creation/004_arange.rs`. Build a one-dimensional range with integer
endpoints and a positive step. The output uses owned `f64` storage.
Your `zeros`, `ones`, and `full` now live in `src/creation/`.
Your `creation.rs` layout and `ones` → `full` → `zeros` dependency direction remain intact.
If Rustlings starts at an earlier completed exercise, check it and press `n`.
The filename prefix is the global exercise order. Matching solutions use the same prefix.
Public function names stay unnumbered.
Press `h` for hints. Save the current exercise to rerun its tests.

The `003_full` tests pass through your public API. Its runner lacks Rustlings' required
historical marker. Add this comment to `exercises/01_creation/003_full.rs` without changing its tests:

```rust
// TODO (historical, completed): implementation moved to rumpy::full.
```

```sh
rustlings hint 004_arange
rustlings run 004_arange
# Headless checks:
cargo test --bin 004_arange
cargo test --bin 001_zeros --bin 002_ones --bin 003_full
```

Rustlings success output needs a real terminal. Use Cargo for headless checks.
The watcher tracks the current exercise, not `src/` or other exercises.
After dependency changes, press `c` for a full check, or use `--manual-run` with `r`.

## Exercise to library

```text
exercises/01_creation/004_arange.rs active task: edit this
solutions/01_creation/004_arange.rs reference: compare after your attempt
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
git commit -m "Implement arange"
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
- `cargo test --lib --test scaffold --bin 001_zeros --bin 002_ones --bin 003_full` checks the completed library and scaffold.
- `cargo check --all-targets`, `cargo fmt --check`, and Clippy check build quality.
- `rustlings dev check --require-solutions` validates unfinished tasks and all references.

Graduated runners use `skip_check_unsolved = true` in `info.toml`. Rustlings 6.5.0 still
requires a literal `// TODO` comment in each runner, so completed runners label it historical.
Do not remove their regression tests. The author checks are not a learner completion gate.

`python3 scripts/check_authoring.py` verifies the current publication in disposable copies
(Python 3.11+, Rustlings, and Cargo required). It checks the intentionally failing new stub,
the passing reference solutions, and recorded NumPy fixtures. The script does not alter your
exercise files. After you solve the current task, its expected-red check will fail by design.

The current full-repository Rustlings author check fails because `003_full` lacks its historical marker.
The checker reports that baseline error. A second disposable project excludes that runner and its reference from the Rustlings check.
It still runs that runner's full tests, its reference tests, and the public-API oracle checks separately.
It does not add a marker to your source. The normal Rustlings check resumes after you add the comment.
