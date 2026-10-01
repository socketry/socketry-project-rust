# Rust Testing

Use Cargo's built-in test command for Rust tests. Keep the standard workflow in
`.github/workflows/test.yml`, separate from release publication.

For the shared Bake interface, add `bake-test-rust` to the private `bake/`
package and run `cargo bake --regenerate` to link its task registration:

```toml
[dependencies]
bake-test-rust = "0.1"
```

Regeneration updates the generated dependency links; no manual source import is
needed.

## Run tests locally

At a workspace root, run:

```sh
cargo test --workspace --locked
```

For a single package that is not a workspace, run `cargo test --locked`. Cargo
runs the selected packages' unit, integration, and documentation tests. Follow
the [Rust repository layout guide](layout.md#test-layout) for organizing test
files. Add `--all-targets` when examples and benchmarks should also be built as
test targets. Cargo remains the underlying test runner. A project can expose a
consistent `bake test` command through `bake-test-rust`; it delegates to the
standard Cargo command and runs the optional `test:before` preparation hook.

Run formatting and lint checks separately:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

## Coverage

Use the `test:coverage` task from `bake-test-rust` when a project needs a
coverage gate. It is available after the lockfile resolves a version that
includes the task; update `Cargo.lock` before using it with `--locked`. Install
`cargo-llvm-cov` once on local machines:

```sh
rustup component add llvm-tools-preview --toolchain stable
cargo +stable install cargo-llvm-cov --locked
rustup run stable cargo bake test:coverage
```

Run the task with the same toolchain that has `llvm-tools-preview`. If another
Rust installation such as Homebrew's `cargo` comes first on `PATH`, plain
`cargo bake` may use that compiler and fail to find the Rustup component.

The task runs documentation tests and requires 100% line coverage for the
selected feature configuration. It prints uncovered lines directly in the
terminal. Select all features with `--all-features true`, or repeat
`--features name` for one supported feature combination. Projects with
architecture-specific code should run the gate on each supported architecture;
coverage applies to the code compiled for that target. The task covers the
whole workspace by default; use `--package name` to gate one package when the
workspace contains supporting tools or applications with separate coverage
needs.

In GitHub Actions, use `actions-rust-lang/setup-rust-toolchain@v2` to set up
Rust and its components. For a coverage workflow, the setup looks like this:

```yaml
- uses: actions-rust-lang/setup-rust-toolchain@v2
  with:
    components: clippy, llvm-tools-preview, rustfmt
- name: Install coverage tool
  run: cargo install cargo-llvm-cov --locked
```

Then run `cargo bake --locked test:coverage`. The coverage task runs the tests
itself; do not run `cargo bake --locked test` as a duplicate step in the same
job. Keep coverage enforcement and target selection in Bake so workflows call
one stable task while its implementation can evolve.

## GitHub Actions

Every Rust repository should have `.github/workflows/test.yml` with the workflow
name `Test`. Run it for pushes, pull requests, and manual dispatch. When the
project includes a `bake-test-rust` version with `test:coverage`, install
Rust with `actions-rust-lang/setup-rust-toolchain@v2` and run the coverage task
as described above, so CI applies the same gate and preparation hook as local
development. Otherwise, use `cargo bake --locked test` when `bake-test-rust` is
available, or `cargo test --workspace --locked`; a single-package repository
may use `cargo test --locked`.

Add operating-system, target, or feature matrix entries when the code needs
coverage beyond the default configuration. Test supported feature combinations
explicitly; do not assume that every feature can be enabled together. Keep the
coverage gate visible in `test.yml` even when `publish.yml` repeats tests as a
release safety check.

## External compatibility tests

Cargo tests the selected package and its workspace members; it does not provide
a built-in command for testing downstream repositories. The shared
`bake-test-rust` task library provides both `bake test` and
`bake test:external`, keeping the ordinary and downstream test commands
together.

List selected downstream repositories in Cargo metadata, rather than keeping a
second list in `config/external.yaml`:

```toml
[[workspace.metadata.bake.test.external]]
repository = "https://github.com/socketry/downstream-project"
branch = "main"
name = "downstream-project"
```

For a single-package repository, use
`[[package.metadata.bake.test.external]]`. The `name` field is optional; it
chooses the checkout directory under `external/` and lets repositories with the
same name use distinct directories. Omit the metadata entirely when the list is
empty. Only create `.github/workflows/external.yml` when at least one
downstream repository is listed; remove it when the list becomes empty.

`bake test:external` clones each listed repository into
`external/<repository-name>/` and keeps that checkout for reuse. It adds
`[patch.crates-io]` entries to the cloned repository's `Cargo.toml`, pointing to
the local workspace packages. Patching is idempotent and preserves unrelated
manifest content. The clone is a local test workspace, so the patch remains
available when you enter the downstream checkout and run
`cargo test --workspace` manually to diagnose a failure. The task does not
reset, fetch, or reclone an existing checkout, preserving local investigation
work. Add `/external/` to the repository's `.gitignore`.

Run downstream tests without `--locked`, since Cargo may need to update the
cloned repository's lockfile to resolve the local path patches.

The external runner is provided by `bake-test-rust`. It verifies that Cargo
selected each local patch and stops when a dependency version requirement still
resolves to the registry package. The local crate version must satisfy the
downstream dependency's version requirement; update that requirement in the
checkout when testing a breaking version. The optional `test:before` task runs
once before the local or external Cargo tests; use it for shared asset or
fixture preparation.

When external repositories are listed, `.github/workflows/external.yml`
should run `cargo bake --locked test:external`. Leave that workflow out when
the list is empty.
