# rumpy exercise-author contract

The learner writes the mathematical implementations. This is a cumulative NumPy/SciPy-style library, not throwaway answers. Keep source API names and document supported subsets precisely. The scaffold uses owned contiguous row-major f64 data.

## Changes

- Add exactly one exercise for a batch of new learner commits on origin/main. No passing-tests or review-approval gate.
- Never generate from your own bot commits, or without a new learner commit. No daily exercise quotas or changes to paper SR.
- Preserve learner implementations and uncommitted work. Never force-push, reset their checkout, weaken their tests, or solve their exercise without a direct request.
- Author in a separate temporary clone. If the remote advances, fetch and reapply only your additions safely before a normal push. Never discard another commit.
- Add a lesson, exact contract, meaningful tests, and progressive hints in info.toml. Put an explicit `// TODO` comment by unfinished implementation code.
- Put editable operations under exercises/ so Rustlings watches them. Re-export those exact functions from src/lib.rs; do not maintain separate completed implementations.
- Run `rustlings dev update` after metadata changes. Keep Cargo.lock tracked.

## Verification

- Use independent NumPy/SciPy/JAX or exact reference checks. Record actual oracle output, never invented fixtures.
- In a disposable workspace, prove the new stub fails for its intended TODO and a private reference solution passes. Never commit reference solutions or replacements for learner code.
- Existing learner failures do not block the next exercise. Give brief feedback without fixing their code. Select the next task to avoid a broken dependency where possible; disclose any dependency constraint.
- Use suitable floating-point tolerances, shape/axis/edge cases, and informative failures.
- Run cargo fmt --check, cargo check --all-targets, scaffold tests, and new-exercise tests. Rustlings dev check expects unsolved exercises; do not treat rejection of an already solved exercise as a regression.
- No answer leaks in tracked solutions, scripts, or Git history. Expected test values and progressive hints are intentional.

## Publication and idempotency

Use per-command Git identity; never change the learner's local/global identity:
`git -c user.name="Rumpy Exercise Bot" -c user.email="rumpy-exercises@users.noreply.github.com" commit ...`
Every bot commit has `Rumpy-Agent: true` as a trailer.
Exercise publication commits also have `Rumpy-Through: <full learner commit SHA>`.
The initial scaffold uses `Rumpy-Through: bootstrap`.
The trailer records the newest learner commit covered by the exercise. A successful push therefore acknowledges that commit, even if the job crashes afterward.
Update .rumpy/progress.json with the same SHA and the new sequence/function. Verify the remote commit and files after a normal push, before a success notification.

The separate Hermes job is pinned to gpt-6-astra/openai-codex. Never use OpenRouter, switch providers/models, or modify the paper SR cron.
