# rumpy

Build a small NumPy/SciPy-style numerical library in Rust, through custom Rustlings exercises.
Start at the array foundations, preserve familiar API names, and reuse your own implementations as the library grows.

## Start

Rust and Cargo are required. The repository pins Rust 1.97.1; rustup installs it if needed.
Install the exercise runner if it is absent:

```sh
cargo install rustlings --version 6.5.0 --locked
```

```sh
git clone git@github.com:Artur-Galstyan/rumpy.git
cd rumpy
rustlings
```

Do **not** run `rustlings init` here: that creates the official Rust-language course, not this custom course.
Read [the first lesson](lessons/001-zeros.md), then edit `exercises/01_creation/zeros.rs`.
Press **h** in Rustlings for hints. Save the file to rerun its tests.

Commands from a terminal:

```sh
rustlings hint zeros
rustlings run zeros
cargo test --bin zeros
```

Rustlings needs a real terminal, including for its success screen. For headless checks,
use `cargo test --bin zeros`. Rustlings watches the current file under `exercises/`.
If you edit an earlier dependency or scaffold code, press `c` for a full check, or restart
with `rustlings --manual-run` and use `r`.

The first exercise is intentionally unfinished. Its tests should fail until you implement `zeros`.
The tests are part of the task, not code to remove or weaken.

## One implementation, two uses

`src/array.rs` supplies the small owned `f64` array container and shape checks.
Your operation lives under `exercises/`, where Rustlings watches edits.
`src/lib.rs` re-exports that exact implementation as `rumpy::zeros`.
Later exercises call this library; nothing needs to be copied or promoted manually.
Exercise test modules can also run through `cargo test --lib`, so a whole-suite run includes unfinished exercises and may be red by design.

We start with owned contiguous row-major storage and `f64`. Generic dtypes, views,
parallel kernels, GPUs, and complete NumPy compatibility are explicitly out of scope for now.
Every lesson documents its supported subset rather than silently changing the source API.

## Pull, solve, push

```sh
git pull --ff-only
# Edit the exercise and use Rustlings.
git add exercises/01_creation/zeros.rs
git commit -m "Implement zeros"
git push origin main
```

A separate Hermes job checks `main` daily at 07:30 Europe/Berlin. **Any new learner
commit triggers one next exercise per check**, even if the attempt is unfinished or tests fail.
No review-approval gate. Several new commits count as one batch. Bot commits never trigger
more exercises. Unpushed edits and commits on other branches are not visible to the job.

The job preserves your code, adds the next exercise with tests and hints, and sends a Discord notification.
Keep local work committed before you pull. If Git reports a conflict, stop and inspect it; never discard your work to get an update.
This is separate from the paper spaced-repetition system.

## Curriculum and checks

- [Roadmap](ROADMAP.md)
- [Exercise-author rules](AGENTS.md)
- `.rumpy/progress.json`: current position and the last handled learner commit.
- `cargo test --test scaffold`: checks the supplied container, independent of the unfinished exercise.
- `cargo check --all-targets` and `cargo fmt --check`: scaffold/build checks.
- `rustlings dev check`: validates a freshly authored unsolved exercise set. It is an authoring check, **not** your completion command; solved exercises intentionally cause its unsolved check to fail.

The author tests solutions in a disposable private workspace. Reference implementations
are not included in your exercise files or committed to Git history. NumPy oracle checks
for the first exercise are recorded in `validation/zeros-numpy.json`.
