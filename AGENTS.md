# rumpy exercise-author contract

The learner writes the mathematical implementations. Build a cumulative NumPy/SciPy-style
library with familiar API names and precise supported subsets. Use owned contiguous
row-major f64 arrays until a later lesson explicitly extends that scope.

## Trigger and safety

- A new learner commit batch on origin/main triggers exactly one next exercise. Passing
  tests, approval, and a separate completion message are not gates. Bot commits do not count.
- No new learner commit means no new exercise or routine notification. Paper SR stays separate.
- Author in an isolated clone. Never reset the learner checkout, force-push, weaken tests,
  replace an attempt with a reference solution, or silently fix their implementation.
- Inspect baseline commits and test results before edits. If origin advances, preserve all
  commits, reapply only safe author changes, retest, and use a normal push.

## Learner-owned graduation and refactoring

- The learner owns all moves into src/, module layout, exports, function refactors,
  dependency changes, caller migrations, and conversion of exercise files into regression
  runners. Do NOT perform these edits automatically, even when an exercise passes.
  Only a separate explicit request to edit that code permits an exception.
- Review pushed attempts and library changes against the original contract, assertions,
  oracle data, and edge cases. Give feedback rather than silently fixing code or layout.
- A passing exercise that still lives only under exercises/ is solved but not graduated.
  Leave its source untouched. Record graduation only after the learner actually adds the
  public library implementation and connects the old exercise tests to that public API,
  and those checks pass. Inspect actual paths; suggested library_target paths are advisory.
- The learner may consolidate functions into shared modules. Do not enforce one file per
  function or restore the previous layout. Keep public API names unnumbered.
- The author may maintain info.toml, generated Cargo bins, progress records, documentation,
  and reference solutions. These bookkeeping changes must reflect the learner's actual
  state and must not edit their implementations, imports/exports, callers, or old test runners.
- Mark passing older exercises skip_check_unsolved = true when needed for author checks,
  including solved-but-not-yet-integrated attempts. This is not proof of graduation.
  Preserve all regression tests. If a runner lacks Rustlings' required historical // TODO
  marker, explain the needed learner edit rather than silently changing its source.
- Each new lesson includes a short post-exercise checklist: choose module layout, move
  the implementation, expose its API, connect preserved tests, refactor dependencies,
  and run regression tests. The learner does this work, not the author.
- For extensions, the author may start a NEW exercise from a copy of the current learner
  implementation while leaving src/ untouched. The learner decides how and when to merge
  the extension, migrate callers, and refactor earlier operations. Explain compatibility
  requirements and dependency cycles without doing the refactor for them.
- Solving, graduation, and next-exercise publication are separate. An incomplete or
  unintegrated attempt still triggers one next exercise for a new learner commit batch.
  Avoid unavailable/broken dependencies where possible. Report problems without an approval gate.

## Exercise and solution files

- Prefix every exercise ID and filename with its global three-digit sequence:
  001_zeros, 002_ones, 003_full. Continue across topic folders, not from one per folder.
  Use the same basename in solutions/. Keep info.toml names, Cargo bins, documented
  commands, and progress paths aligned. Run rustlings dev update after a rename.
  Public function names and src module filenames stay unnumbered. The active/graduated
  maps stay keyed by function, with an explicit exercise field for the numbered ID.
  current_exercise stores the numbered ID. A later extension gets a fresh sequence ID.
- Add one small operation, a lesson, exact contract, meaningful tests, and progressive hints
  in info.toml. Include // TODO and fn main(). Leave new learner code unfinished and compilable.
- Publish a tested, commented solution at solutions/<same-topic>/<same-name>.rs. Include the
  exercise's full contract tests. Keep solutions separate from src and from learner attempts.
  Full answers belong only there, never in hints or the unfinished exercise.
- Link the original NumPy/SciPy/JAX API. State unsupported parameters/types explicitly.
  Show every algebra step and rule if a lesson includes a mathematical derivation.
- Run rustlings dev update after metadata changes. Preserve its top-level Cargo bin list,
  Cargo.lock, prior lessons, and prior exercises. Update README/welcome text to the current task.

## Verification

- Compare values with real NumPy/SciPy/JAX or exact references and save honest oracle fixtures.
- Prove the new stub fails for its intended TODO, not a setup error. Test its reference against
  the same tests in a disposable copy. Test every graduated function through the public API.
- Run formatting, cargo check --all-targets, Clippy, scaffold and graduated-runner tests,
  then rustlings dev check --require-solutions. Run python3 scripts/check_authoring.py and
  extend that checker when new signatures/operations need different oracle checks.
- Record baseline failures separately. Earlier unfinished but passing tasks may need author
  metadata handling, never test weakening. Do not mistake an expected new-stub failure for green.
- Rustlings watches only the current exercise. Use c for dependency checks, or manual-run/r.
  Use Cargo headlessly and a PTY for successful Rustlings run output.

## Publication

Use per-command Git identity, never global configuration:
`git -c user.name="Rumpy Exercise Bot" -c user.email="rumpy-exercises@users.noreply.github.com" commit ...`
Every bot commit has `Rumpy-Agent: true`. An exercise publication also has
`Rumpy-Through: <full newest handled learner SHA>`. Bootstrap used `bootstrap`.
Record that same SHA in .rumpy/progress.json. A successful push acknowledges the batch.
Recheck the probe before publication to avoid a duplicate. After a normal push, read back
remote SHA/files and probe state before notification. A newer unhandled commit remains pending.

Keep the separate Hermes job on gpt-6-astra/openai-codex, daily 07:30 local time.
Never change providers/models, other profiles, or the paper SR job.
