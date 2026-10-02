# Testing

Every Rust project should include tests for its public behavior and maintain
100% line coverage for compiled workspace targets. Organize unit, integration,
and documentation tests according to the [Rust repository layout guide](layout.md#test-layout).

Use the shared `bake-test-rust` tasks from the private `bake/` package. The
`test:coverage` task is the canonical CI test entry point: it runs the tests and
enforces the line coverage requirement. Add `--all-targets true` when examples
and benchmarks should also be included.

External compatibility tests are optional. List selected downstream projects
in Cargo metadata and add an external test workflow only when the list is
nonempty.

Consult the installed
`.agents/context/bake-test-rust/testing.md` for task configuration, local and
CI commands, coverage setup, `test:before`, and external test details. That
guide owns the canonical workflow and the mechanics for using the tasks.
