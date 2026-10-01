# `socketry-project`

Shared conventions and development tasks for Rust projects in the Socketry
organization.

## Motivation

Rust projects should share a small set of clear conventions and a reliable
release process without repeating task wiring in every repository.

## Usage

Add `socketry-project` to the dependencies of your private `bake/` package:

```toml
# bake/Cargo.toml
[dependencies]
bake = "0.17"
socketry-project = "0.2"
```

Link the tasks from `bake/src/main.rs`:

```rust,ignore
use bake::{Registry, Result};
use socketry_project as _;

fn main() -> Result<()> {
    Registry::discover()?.run()
}
```

This adds the standard Cargo, release, license, Readme, agent-context, and
testing tasks, including `test` and `test:external`. It also registers a
version-bump hook that updates the project's standard files.

See the [project setup guide](context/project-setup.md) for the minimal Cargo
configuration and the [project conventions](context/project-conventions.md) for
repository layout, documentation, and code conventions. The
[Rust Project Layout](context/project-layout.md) and
[GitHub Repository Setup](context/github-repository.md) guides provide more
detail. The [Rust Testing](context/testing.md) guide describes the standard
test workflow and optional downstream compatibility workflow.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`,
or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a
pull request. After review and merge, GitHub Actions publishes the release
when the configured `crates-io` environment approves it. See the
[Cargo publishing guide](https://github.com/socketry/bake-cargo-rust/blob/main/context/publishing.md).

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

### v0.2.0

- Move the shared project tasks to `bake` 0.17 and include standard Rust test tasks.
- Document standard local, downstream, and publishing workflows.

### v0.1.1

- Add a reusable Pull Requests guide to the packaged agent context.

### v0.1.0

- Establish shared conventions and standard Bake tasks for Socketry Rust projects.
- Document project layout, GitHub setup, and agent context installation.
<!-- bake-readme:releases:end -->

## See Also

- [socketry-project](https://github.com/socketry/socketry-project-rust) — Shared project conventions and development tasks for Socketry Rust crates <!-- bake-readme:package -->

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/socketry-project-rust).

### Agent Context

Before contributing, read `agents.md` and the relevant context files it links. If `agents.md` is missing or out of date, run `cargo bake agent:context:install` to install context from dependencies and update the index. See [Agent Context](context/agent-context.md) for guidance on writing and organizing context for a project.
