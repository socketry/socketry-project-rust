# `socketry-project`

Shared conventions and development tasks for Rust projects in the Socketry
organization.

## Motivation

Rust projects should share a small set of clear conventions and a reliable
release process without repeating task wiring in every repository.

## Usage

Install the launcher and create the private task package:

```sh
cargo install socketry-cargo-bake --locked
cargo bake --regenerate
```

Add this dependency under the existing `[dependencies]` table in
`bake/Cargo.toml`:

```toml
socketry-project = "0.2"
```

Then run `cargo bake --regenerate` again to link the dependency's tasks. The
command keeps generated links separate from task source, so no manual import in
`main.rs` is needed. Run `cargo bake --list` to see the available tasks.

This adds the standard Cargo, release, license, Readme, agent-context, and
testing tasks, including `test` and `test:external`. It also registers a
version-bump hook that updates the project's standard files.

See the [setup guide](context/setup.md) for the minimal Cargo configuration
and the [conventions](context/conventions.md) for repository layout,
documentation, and code conventions. The
[Rust Repository Layout](context/layout.md) and
[GitHub Repository Setup](context/github-repository.md) guides provide more
detail. The [Rust Testing](context/testing.md) guide describes the standard
test workflow and optional downstream compatibility workflow.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`,
or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a
pull request. After review and merge, GitHub Actions publishes the release
when the configured `crates-io` environment approves it, then creates or updates
the matching GitHub Release from `releases.md`. See the
[Cargo publishing guide](https://github.com/socketry/bake-cargo-rust/blob/main/context/publishing.md).

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

### v0.2.3

- Rename the project guides to shorter context filenames and titles.
- Clarify source, test, naming, and coverage conventions.

### v0.2.2

- Use Bake Cargo's GitHub Release automation in the standard publishing workflow.

### v0.2.1

- Document setup and task linking with `cargo bake --regenerate`.
<!-- bake-readme:releases:end -->

## See Also

- [socketry-project](https://github.com/socketry/socketry-project-rust) — Shared project conventions and development tasks for Socketry Rust crates <!-- bake-readme:package -->

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/socketry-project-rust).

### Agent Context

Before contributing, read `agents.md` and the relevant context files it links. If `agents.md` is missing or out of date, run `cargo bake agent:context:install` to install context from dependencies and update the index. See [Agent Context](context/agent-context.md) for guidance on writing and organizing context for a project.
