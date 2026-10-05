# `socketry-project`

Shared conventions and development tasks for Rust projects in the Socketry organization.

## Motivation

Rust projects should share a small set of clear conventions and a reliable release process without repeating task wiring in every repository.

## Usage

Install the launcher and create the private task package:

```sh
cargo install socketry-cargo-bake --locked
cargo bake --regenerate
```

Add this dependency under the existing `[dependencies]` table in `bake/Cargo.toml`:

```toml
socketry-project = ">=0.3.6"
```

Then run `cargo bake --regenerate` again to link the dependency's tasks. The command keeps generated links separate from task source, so no manual import in `main.rs` is needed. Run `cargo bake --list` to see the available tasks. The open-ended minimum requirement keeps this development dependency eligible for newer releases. `Cargo.lock` records the selected version for reproducible builds; update it deliberately when adopting a newer release.

This adds the standard Cargo, release, license, Readme, Markdown, agent-context, and testing tasks, including `test` and `test:external`. It also registers a version-bump hook that updates the project's standard files.

See the [repository setup skill](context/setup.md) for the minimal Cargo configuration and the [conventions](context/conventions.md) for repository layout, documentation, and code conventions. The [Rust Repository Layout](context/layout.md) explains source organization. The testing skill sets expectations and points to `bake-test-rust` for task and workflow details.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), then run `cargo bake cargo:release` and open a pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it, then creates or updates the matching GitHub Release from `releases.md`. See the shared [Releasing skill](context/releasing.md) for the standard release process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.3.6

- Normalize standard project Markdown files after version bumps.
- Fix the version bump hook's invocation of the Markdown normalizer.
- Include `bake-markdown` 0.3.0 in the shared task set so version bumps use hyphen markers for unordered lists.

### v0.3.5

- Require `bake-test-rust` 0.3.0 or newer for LLVM region coverage and update the shared testing guidance to match.

### v0.3.4

- Require Bake 0.18.0 and Bake Cargo 0.4.0 for shared task registration.
- Document a test-only shim pattern for deterministic coverage of difficult I/O failures.

<!-- bake-readme:releases:end -->

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/socketry/socketry-project-rust).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Read `.agents/context/index.md` to find relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. See the [Agent Context guide] for guidance on organizing package context and repository-only instructions.

[Agent Context guide]: https://github.com/socketry/bake-agent-context-rust/blob/main/context/agent-context.md
