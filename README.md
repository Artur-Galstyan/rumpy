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

**Current task:** read [003 — full](lessons/003-full.md) and edit
`exercises/01_creation/full.rs`. Your `zeros` and `ones` now live in `src/creation/`.
The new task copies your `ones` code and adds a scalar fill parameter.
The public `ones` signature stays unchanged. `full` is a separate operation.
If a fresh Rustlings state starts at `zeros`, check it and press `n`, then repeat for `ones`.
Press `h` for hints. Save the current exercise to rerun its tests.

```sh
rustlings hint full
rustlings run full
# Headless checks:
cargo test --bin full
cargo test --bin zeros --bin ones
```

Rustlings success output needs a real terminal. Use Cargo for headless checks.
The watcher tracks the current exercise, not `src/` or other exercises.
After dependency changes, press `c` for a full check, or use `--manual-run` with `r`.

## Exercise to library

```text
exercises/01_creation/full.rs   active task: edit this
solutions/01_creation/full.rs   reference: compare after your attempt
src/creation/ones.rs            your completed implementation
exercises/01_creation/ones.rs   original tests against rumpy::ones
src/creation/zeros.rs           your completed implementation
exercises/01_creation/zeros.rs  original tests against rumpy::zeros
src/lib.rs                     public library exports
```

The library contains only verified learner implementations, never reference solutions.
`src/array.rs` supplies the owned contiguous row-major `f64` array container.
Active exercise implementations are not library modules. An unfinished exercise can
fail without breaking the completed library. `cargo test` still includes unfinished
exercise binaries and can therefore fail intentionally.

After a pushed implementation meets its contract, the author moves **your code**
into a topic module under `src/` and exposes it through `lib.rs`. The old exercise
becomes a test runner against that public API. Its tests stay as regression checks.
A passing test result permits graduation, not automatic replacement with an answer.

If a later exercise extends an operation, the existing library implementation stays
in place. The new exercise starts from a copy of your implementation and adds the
new task. After verification, your extended version replaces the library version.
Temporary duplication keeps the library usable while you work. A signature change
must include an explicit migration and updated callers without weakening their tests.

## Reference solutions

Every exercise has a matching file under `solutions/`, including explanatory comments
and the same contract tests. Rustlings can report its path after a successful run.
You can also open it directly. These files are spoilers for comparison, not library code.

## Pull, solve, push

```sh
git pull --ff-only
# Edit exercises/01_creation/full.rs and use Rustlings.
git add exercises/01_creation/full.rs
git commit -m "Implement full"
git push origin main
```

A separate Hermes job checks `main` daily at **07:30 Europe/Berlin**.
Any new learner commit triggers one next exercise per check, even if tests fail.
Several commits form one batch. Bot commits never trigger another exercise.
No new learner commit means no new exercise or routine notification.

The job also checks pending implementations for graduation. Failed attempts remain
untouched. The next exercise avoids broken dependencies where possible.
Unpushed work and other branches are not visible to the job.
Commit local work before a pull. If Git reports a conflict, inspect it rather than
discard your work. This job remains separate from paper spaced repetition.

## Author checks

- [Roadmap](ROADMAP.md) and [author contract](AGENTS.md).
- `.rumpy/progress.json` records active/graduated functions and the handled learner SHA.
- `cargo test --lib --test scaffold --bin zeros --bin ones` checks the completed library and scaffold.
- `cargo check --all-targets`, `cargo fmt --check`, and Clippy check build quality.
- `rustlings dev check --require-solutions` validates unfinished tasks and all references.

Graduated runners use `skip_check_unsolved = true` in `info.toml`. Rustlings 6.5.0 still
requires a literal `// TODO` comment in each runner, so completed runners label it historical.
Do not remove their regression tests. The author checks are not a learner completion gate.

`python3 scripts/check_authoring.py` verifies the current publication in disposable copies
(Python 3.11+, Rustlings, and Cargo required). It checks the intentionally failing new stub,
the passing reference solutions, and recorded NumPy fixtures. The script does not alter your
exercise files. After you solve the current task, its expected-red check will fail by design.
