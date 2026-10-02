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

## Use the shared testing tasks

Use the shared Bake test tasks for local verification and CI. Consult the
installed `bake-test-rust` context at
`.agents/context/bake-test-rust/testing.md` for canonical workflow files, task
setup and invocation, coverage options, and downstream test configuration.
