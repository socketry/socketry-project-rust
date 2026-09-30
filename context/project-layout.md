# Rust Project Layout

Use Cargo's standard structure so contributors can find package code, tests,
examples, and project documentation quickly. Add directories as the project
needs them; a small crate does not need every directory shown here.

```text
project/
├── Cargo.toml
├── src/
│   └── lib.rs
├── tests/
├── examples/
├── context/
├── bake/
├── .agents/
├── .github/
├── readme.md
├── license.md
└── releases.md
```

## Cargo package and workspace

The root `Cargo.toml` defines the public package and, when needed, the Cargo
workspace. Put library implementation under `src/`; use `tests/` for tests of
the public crate interface and `examples/` for runnable usage examples. Follow
Rust naming conventions for source files and avoid abbreviations unless an
established Rust API requires them.

Use separate repositories for packages with independent versions or release
histories. Packages in one workspace should share a version and a release
boundary. A private `bake/` workspace member can carry project automation
without making those tools dependencies of the published library:

```toml
[workspace]
members = ["bake"]
resolver = "3"

[workspace.metadata.bake]
manifest = "bake/Cargo.toml"
```

Set `publish = false` in `bake/Cargo.toml`. Keep project tools such as
`socketry-project` and `bake-cargo` in that package's dependencies.

## Project files

Maintain only these authored Markdown files at the repository root:

- `readme.md` introduces the project, its motivation, basic usage, recent
  releases, and contribution path. Follow the `Readme Structure` guide supplied
  by `bake-readme`.
- `license.md` begins with `# MIT License` and records the license and copyright.
- `releases.md` records user-visible changes and is the source for release
  descriptions.

Keep file names lowercase. Cargo package metadata belongs in `Cargo.toml`.
Include documentation and agent context in the published crate when downstream
users benefit from it; use the Cargo `include` field only when you can keep the
complete package file list accurate.

## Agent guidance

Put reusable, package-specific guidance in the tracked `context/` directory.
Put repository-only agent instructions and skills in `.agents/`. Bake Agent
Context installs generated dependency context under `.agents/context/`; ignore
that generated directory and refresh it from its source package when needed.
See [Agent Context](agent-context.md) for the distinction and installation
workflow.

## Tests and configuration

Keep unit tests close to private implementation details and use integration
tests under `tests/` to check the public interface. Put runnable examples under
`examples/`. Follow the [Rust Testing](testing.md) guide for local commands,
the `test.yml` workflow, and optional downstream compatibility tests. Store
project tool configuration in a clearly named configuration file or the
relevant Cargo metadata; avoid adding a configuration directory without a
concrete tool that uses it.

Run checks from the workspace root so all members are covered:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The generated Cargo publishing workflow runs these checks. See the
[Cargo Publishing guide](https://github.com/socketry/bake-cargo-rust/blob/main/context/publishing.md)
for release workflow and registry details.
