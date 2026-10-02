# Releases

## v0.3.0

- Add an update skill for auditing and modernizing existing Socketry Rust
  projects.

## v0.2.7

- Clarify how context installation uses `.agents/context/index.md` and local
  Git exclusions without changing a repository-owned `agents.md`.
- Express shared task dependencies as minimum versions so compatible updates
  can be selected by the host project's lockfile.
- Require 100% line coverage for workspace targets in GitHub Actions.
- Use the shared `bake-test-rust` coverage task in the standard workflow.
- Cover Bake task execution and the version-bump hook's task ordering.

## v0.2.6

- Support `bake-agent-context` 0.2 while retaining compatibility with 0.1.
- Keep the generated context index under `.agents/context/` without modifying
  the repository owner's `agents.md`.

## v0.2.5

- Distribute repository setup, GitHub setup, and pull request guidance as
  installable skills.
- Provide shared Agent Context guidance through `bake-agent-context`.

## v0.2.4

- Clarify crate namespaces, descriptive type names, module layout, and test paths.

## v0.2.3

- Rename the project guides to shorter context filenames and titles.
- Clarify source, test, naming, and coverage conventions.

## v0.2.2

- Use Bake Cargo's GitHub Release automation in the standard publishing workflow.

## v0.2.1

- Document setup and task linking with `cargo bake --regenerate`.

## v0.2.0

- Move the shared project tasks to `bake` 0.17 and include standard Rust test tasks.
- Document standard local, downstream, and publishing workflows.

## v0.1.1

- Add a reusable Pull Requests guide to the packaged agent context.

## v0.1.0

- Establish shared conventions and standard Bake tasks for Socketry Rust projects.
- Document project layout, GitHub setup, and agent context installation.
