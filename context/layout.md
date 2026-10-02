# Rust Repository Layout

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
the [Socketry Rust naming conventions](conventions.md#source-naming)
for source files and Rust items.

## Source modules and files

Organize code by subsystem, using directories to make the module tree visible.
Use the modern file-plus-directory layout: a module with child modules has a
same-named source file and directory. For example, `src/parser.rs` defines the
`parser` module and declares children whose files live under `src/parser/`:

```text
src/
├── lib.rs
├── parser.rs
└── parser/
    ├── block_parser.rs
    └── inline_parser.rs
```

Declare each source module with `mod` in its parent. Keep module names and
directory names aligned; avoid `mod.rs` for new modules. For each primary
public struct, enum, or trait, use a source file named after the item in
`snake_case`—for example, define `HTMLRenderer` in `html_renderer.rs`. Small,
closely related helper types can share that file.

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

Keep file names lowercase. Preserve a repository-owned `agents.md` when one is
present; project context installation leaves it alone. Cargo package metadata
belongs in `Cargo.toml`. Include documentation and agent context in the
published crate when downstream users benefit from it; use the Cargo `include`
field only when you can keep the complete package file list accurate.

## Agent guidance

Put reusable, package-specific guidance in the tracked `context/` directory.
Put repository-only agent instructions and project-owned skills in `.agents/`.
Run `cargo bake agent:context:install` to create `.agents/context/index.md`
with links to ordinary context from dependencies. Agents can use this index to
discover project guidance and inspect applicable skills without the installer
changing a repository-owned `agents.md`. Bake Agent Context records
`.agents/context/`, its skills registry, and dependency-installed skill
directories in the local Git exclude file at `.git/info/exclude`; these
generated files do not need entries in the project's `.gitignore`.
Project-owned skills remain trackable. See the [Agent Context guide](https://github.com/socketry/bake-agent-context-rust/blob/main/context/agent-context.md)
for the distinction and installation workflow.

## Test layout

Mirror the source organization in tests. Keep unit tests within the module they
exercise, using an inline `#[cfg(test)] mod tests` for small suites. When a
suite grows, move it into that module's directory; for example,
`src/parser/inline_parser.rs` can declare tests from
`src/parser/inline_parser/tests.rs`. This keeps tests able to access private
implementation details without exposing them as public API.

Put integration tests under `tests/` and use them to check the public crate
interface. Mirror the local source module path under `tests/`, starting below
`src/`. Integration tests use the crate name in Rust imports; test directory
names mirror local modules and omit the crate identifier. For example, tests
for `src/parser/inline_parser.rs` belong in a parser test suite. Group
multi-file suites by subsystem, with `main.rs` as the test target root and
sibling files as test modules:

```text
tests/
└── parser/
    ├── main.rs
    ├── block_parser.rs
    └── inline_parser.rs
```

Declare the child modules from `main.rs`; Cargo discovers the target root and
the declared modules organize its tests. See the [Cargo integration test
layout](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests).
Integration tests can use the public crate API. Follow the [Rust Testing](testing.md)
guide for local commands, the `test.yml` workflow, and optional downstream
compatibility tests.

Put runnable examples under `examples/`. Store project tool configuration in a
clearly named configuration file or the relevant Cargo metadata; avoid adding
a configuration directory without a concrete tool that uses it.

## Workspace checks

Run checks from the workspace root so all members are covered:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The generated Cargo publishing workflow runs these checks. See the
[Cargo Publishing guide](https://github.com/socketry/bake-cargo-rust/blob/main/context/publishing.md)
for release workflow and registry details.
