# Set Up a Socketry Rust Project

Use this guide to start a Rust repository with the shared Socketry conventions,
agent context, and release tasks.

## Create the repository

Use a separate repository for a crate that needs its own version, release notes,
or release tag. Use a Cargo workspace when its packages are meant to ship
together at one version and share one `releases.md` and one `vVERSION` tag.

Choose the crate name for its public purpose. Crates.io has a flat package
namespace, so use a `socketry-` prefix when needed to identify a Socketry crate.
The repository can use the `-rust` suffix to distinguish it from a related
project in another language.

Start with the standard Cargo layout and the root files described in
[Rust Project Layout](project-layout.md). Keep the library's dependencies in
the root package and development automation in a private `bake/` workspace
member.

## Add shared project tasks

In the root `Cargo.toml`, add the private Bake member and release reviewers:

```toml
[workspace]
members = ["bake"]
resolver = "3"

[workspace.metadata.bake]
manifest = "bake/Cargo.toml"

[workspace.metadata.bake.release]
reviewers = ["socketry/managers"]
```

Create `bake/Cargo.toml` with `publish = false`, then add the shared task crates
as dependencies there, rather than to the library package:

```toml
[package]
name = "project-bake"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
bake = { package = "socketry-bake", version = "0.2" }
socketry-project = "0.1"
```

Create `bake/src/main.rs`:

```rust,ignore
use bake::{Registry, Result};
use socketry_project as _;

fn main() -> Result<()> {
    Registry::discover()?.run()
}
```

The `socketry-project` dependency links the shared tasks into the private Bake
binary. It also registers `cargo:after_version_bump`, which updates `license.md`,
`releases.md`, and generated sections in `readme.md` after a version change.
Keep task tooling out of the published library's dependency list.

Install the Bake command and the project's agent context:

```sh
cargo install socketry-cargo-bake --locked
cargo bake agent:context:install
```

This installs all context guides provided by resolved dependencies under
`.agents/context/` and updates `agents.md` with links. Read that index, then
open the guides relevant to the work. Add `--package socketry-project` to
install only this crate's guides. See [Agent Context](agent-context.md) for how
to organize and update shared and project-only guidance.

## Set up GitHub

Configure repository metadata, collaboration features, pull request defaults,
and branch protection using [GitHub Repository Setup](github-repository.md).
Generate the Cargo workflow with `cargo:setup:workflow`; follow the
[Cargo Publishing guide](https://github.com/socketry/bake-cargo-rust/blob/main/context/publishing.md)
before applying rulesets, environment reviewers, or crates.io trusted
publishing.

## Work on the project

Keep the root `readme.md` concise and human-focused. Use the project context and
Rust API documentation for detailed implementation guidance. Run the project's
checks before opening a pull request, and update `releases.md` for user-visible
changes.
