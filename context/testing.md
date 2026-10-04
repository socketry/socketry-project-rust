---
type: skill
description: Add or update tests in Socketry Rust projects, require 100% line coverage, and decide when downstream compatibility tests are useful. Use when asked to add tests or when behavior changes need regression coverage.
---

# Testing Socketry Rust Projects

Use this skill whenever a change adds or changes behavior that should be
verified by tests.

## Expectations

- Add tests that exercise the changed behavior and relevant regression cases.
- Require 100% line coverage for supported target and feature configurations.
  Use the coverage report to find and cover every executable source line.
- Treat uncovered code as an opportunity to review semantics. Check that its
  behavior is correct, and fix defects rather than writing tests that codify
  accidental behavior.
- Run coverage on each supported target or configuration that compiles distinct
  platform-specific code.
- Consider external compatibility tests when a change could affect downstream
  crates that use this project's public API. Select the relevant consumers;
  downstream testing is not required for every change.

Follow the repository's test layout guidance when choosing between unit and
integration tests.

## Difficult-to-trigger I/O failures

Use real temporary directories for normal filesystem behavior. When an
important error path depends on an operating-system failure that is difficult
to trigger reliably, a private `#[cfg(test)]` shim can make that failure
deterministic:

- Keep production builds delegated directly to the standard library.
- In tests, wrap only the needed operation and delegate to the real filesystem
  except for a specifically injected failure.
- Scope injected failures to the operation and path, and isolate them from
  parallel tests.
- Assert observable behavior, such as the returned error and whether changes
  were rolled back or cleaned up.

Keep the shim small and local to the code under test. Do not build a broad fake
filesystem or add production abstractions solely to satisfy coverage. If many
call sites need fault injection, reconsider the design and introduce a shared
abstraction only when it also makes the production code clearer.

## Use the shared testing tasks

Use the shared Bake test tasks for local verification and CI. Consult the
installed `bake-test-rust` context at
`.agents/context/bake-test-rust/testing.md` for canonical workflow files, task
setup and invocation, coverage options, and downstream test configuration.
