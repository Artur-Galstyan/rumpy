# Exercise 003 verification

> Historical report: this verification predates the filename migration.
> The exercise IDs are now `001_zeros`, `002_ones`, and `003_full`.

## Learner baseline

Source learner commit: `ca1d4b1f29d700a77f34c8bad8f5363bdddd3675`.
All checks ran in an isolated author clone, never in the learner checkout.

- `cargo test --all-targets` passed before author edits.
- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` passed before author edits.
- The learner changed only the `ones` body. The original contract assertions stayed unchanged.
- Independent checks passed for all 341 shapes of ranks zero through four with axis lengths zero through three.
- The learner implementation also matched the eight saved NumPy `ones` cases.

`ones` graduated to `src/creation/ones.rs`. A direct text comparison confirmed that its body changed only at the necessary `rumpy::zeros` to `crate::zeros` call path. The original regression tests remain byte-identical after the test-module marker.

## Publication checks

`python3 /Users/arturgalstyan/.hermes/rumpy/author_check.py --repo <isolated-clone>` passed.
The wrapper also checked Git-trigger batches, bot suppression, acknowledgments, spoofed markers, and stale cursors.

The repository checker ran these checks in disposable copies:

```text
PASS cargo fmt --check
PASS cargo check --all-targets
PASS cargo clippy --all-targets -- -D warnings
PASS cargo test --lib --test scaffold
PASS cargo test --bin zeros
PASS cargo test --bin ones
PASS expected TODO failure: cargo test --bin full
PASS rustlings dev check --require-solutions
PASS cargo test --all-targets (active references installed only in the disposable copy)
PASS cargo test --test author_numpy_oracle
PASS 133 saved NumPy cases, including shape and exact value bits
```

`uv run --with numpy==2.4.3 python scripts/generate_full_oracle.py` produced 117 actual `full` cases. The checker also retained the earlier `zeros` and `ones` fixtures.
The new contract includes scalar and empty shapes, axis order, ownership, negative zero, infinities, NaN payloads, and finite extremes.
Exercise and reference test modules match exactly. No reference function entered `src/`.

## Rustlings and review

`script -q /dev/null rustlings run ones` passed and displayed:

```text
Solution for comparison: solutions/01_creation/ones.rs
Next exercise: exercises/01_creation/full.rs
```

`rustlings hint full` displayed the new progressive hints.
An independent reviewer approved the staged code and independently reproduced all 117 new fixtures with NumPy 2.4.3.
The static security scan found no issues. `git diff --check` passed.

The published `full` stub deliberately fails. These results do not claim that an unfinished learner suite passes.
