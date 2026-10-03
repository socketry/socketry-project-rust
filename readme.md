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
socketry-project = "0.3"
```

Then run `cargo bake --regenerate` again to link the dependency's tasks. The
command keeps generated links separate from task source, so no manual import in
`main.rs` is needed. Run `cargo bake --list` to see the available tasks.

This adds the standard Cargo, release, license, Readme, agent-context, and
testing tasks, including `test` and `test:external`. It also registers a
version-bump hook that updates the project's standard files.

See the [repository setup skill](context/setup.md) for the minimal Cargo
configuration and the [conventions](context/conventions.md) for repository
layout, documentation, and code conventions. The
[Rust Repository Layout](context/layout.md) explains source organization. The
testing skill sets expectations and points to `bake-test-rust` for task and
workflow details.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`,
or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a
pull request. After review and merge, GitHub Actions publishes the release
when the configured `crates-io` environment approves it, then creates or updates
the matching GitHub Release from `releases.md`. See the shared
[Releasing skill](context/releasing.md) for the standard release process.

## Releases

<!-- bake-readme:releases:start -->
See [releases.md](releases.md) for the full release history.

### v0.3.3

- Point the project update skill at the shared Releasing skill and Bake Cargo
  task documentation.

### v0.3.2

- Add a shared Releasing skill and remove links to duplicated Cargo publishing
  guidance.
- Update the setup instructions to use `socketry-project` 0.3.

### v0.3.1

- Add a testing skill that sets the 100% line-coverage expectation and directs
  agents to review uncovered code for semantic defects and use `bake-test-rust`
  for canonical workflows and task details.
- Document how to maintain consistency across Socketry packages and record new
  semantic, layout, and naming patterns in the package that establishes them.
<!-- bake-readme:releases:end -->

## See Also

- [socketry-project](https://github.com/socketry/socketry-project-rust) — Shared project conventions and development tasks for Socketry Rust crates <!-- bake-readme:package -->

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/socketry-project-rust).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills.
Read `.agents/context/index.md` to find relevant guides, follow `agents.md` if
present, and apply skills under `.agents/skills/`. See the [Agent Context guide]
for guidance on organizing package context and repository-only instructions.

[Agent Context guide]: https://github.com/socketry/bake-agent-context-rust/blob/main/context/agent-context.md
