# Bottom-up roadmap

The order follows dependencies, not an alphabetical API list. Only the current exercise
is published as a task; these are directions, not a daily workload or a fixed deadline.

1. Array creation: `zeros`, `ones`, `full`, `arange`, `eye`, and array construction.
2. Shape and elements: indexing, `reshape`, `transpose`, elementwise functions, broadcasting.
3. Reductions and algebra: `sum`, `mean`, extrema, `dot`, `matmul`, norms, linear systems.
4. SciPy-style functions: selected `signal`, `special`, `stats`, and `optimize` operations.
5. ML/RL: numerical losses, gradients, models, discounted returns, and Bellman updates.

JAX-specific operations retain a separate namespace and contract when introduced.
`conv_general_dilated` is a later milestone, reached through smaller convolution exercises.

New attempts guide examples and difficulty, but **a passing test result is not a gate**
for the next exercise. If an earlier operation is broken, give brief feedback without
rewriting it. A next task can exercise the same concept through another function or avoid
that dependency. Do not secretly replace learner code with a working reference.

Current step: `015_broadcast_to`, copy one array through right-aligned singleton-axis expansion.
Next candidate: extend elementwise `add` to compatible broadcast shapes in a new exercise.
`014_subtract` passes locally and has a public copy, but its runner still tests the local function.
`013_multiply` and `011_argmax` also await their public-API regression connections.
`012_add` is learner-graduated through its public API and original regression tests.
