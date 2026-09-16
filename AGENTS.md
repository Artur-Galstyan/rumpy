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

## Graduation and extensions

- Active tasks live under exercises/ so Rustlings watches edits. Do not import active tasks
  into src/lib.rs. Only verified, graduated learner implementations belong under src/.
- Inspect all pending attempts on each triggered check, not only the latest task. Graduate
  only code that meets its documented contract and passes original tests plus independent
  oracle/edge checks. Passing tests alone do not excuse removed assertions or a known bug.
- Move the learner's actual implementation into a topic module, for example src/creation/zeros.rs.
  Keep the function body unchanged apart from necessary import/path changes and formatting.
  Export it through src/lib.rs. Record its source learner commit in .rumpy/progress.json.
- Replace the old exercise implementation with a public-API import or thin adapter. Retain
  its tests as regression checks. Set skip_check_unsolved = true in info.toml for graduated
  runners. Keep a labelled historical // TODO: Rustlings 6.5.0 requires that marker even here.
- Failed/incomplete attempts stay in exercises/ without fixes. They still trigger the next
  exercise. Avoid unavailable/broken library dependencies or clearly report a blocker.
- For a later extension, leave the old src implementation usable. Copy that implementation
  into a new exercise with an explicit parameter/behaviour task. Do not start over or move
  the library code back. Temporary duplication is deliberate until the extension graduates.
- Before replacement, verify the extension and old behaviours. Explain signature changes,
  update callers/regression adapters, and retain assertions. Preserve prior solutions for
  their original contracts where possible, using documented adapters for changed dependencies.
- Update the active/graduated maps, source commit provenance, current task, and next candidate.
  Never mark an unfinished function as graduated or publish a reference as learner code.

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
