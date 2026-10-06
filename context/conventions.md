# Socketry Rust Conventions

Use these conventions for Rust repositories in the Socketry organization. A project's local `.agents/` files and package-specific documentation can add requirements for that repository.

## Repository and package boundaries

- Use a separate repository when packages need independent versions, release notes, or tags.
- Keep packages in one workspace when they are intentionally released together: use one shared version, one root `releases.md`, and one `vVERSION` tag.
- Name crates for their public purpose. Use the `socketry-` prefix where needed to identify Socketry packages in the flat crates.io namespace; keep Rust module paths semantic and concise.
- Use Cargo's standard `src/`, `tests/`, and `examples/` directories. Use integration tests for public behavior across crate boundaries.

## Crate and module paths

Treat the Cargo crate name as the root namespace. Do not repeat it with a same-named top-level module. Keep modules for meaningful domain concepts, and re-export the main public API from the crate root when deeper modules only organize its implementation. For example, callers can import the agent-context API from the root:

```rust
use bake_agent_context::{ContextIndex, Installer, install_skills, list_skills};
```

Keep public paths independent of redundant namespace layers such as `bake_agent_context::context`. Public modules remain useful when they name a real part of the API: a crate named `protocol_http` can expose `protocol_http::headers::AcceptHeader`, where `headers` identifies the domain concept within the crate.

## Source naming

- Use `snake_case` for modules, source files, functions, methods, and variables. Use `UpperCamelCase` for structs, enums, traits, and type parameters.
- Treat acronyms and initialisms as single words in `UpperCamelCase`, following the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case): write `HtmlRenderer`, `HttpClient`, `UrlParser`, and `FileIo`. Use lowercase acronyms in `snake_case`, including filenames such as `html_renderer.rs`, `http_client.rs`, and `file_io.rs`. This keeps Socketry's Rust APIs consistent with the Rust ecosystem; names required by external APIs retain their original spelling.
- Make public type and trait names fully descriptive; include the kind of thing being named instead of relying on the module path to supply it. For example, use `io_stream::BufferedStream` rather than `io_stream::Buffered`.
- Prefer clear, consistent names. Avoid abbreviations unless they are an established domain initialism or required by an external API.
- Give each primary public struct, enum, or trait its own source file, named after the item using lowercase `snake_case`. Keep small, closely related helper types alongside it when that makes the code easier to understand.

## Consistency across packages

Before introducing a new semantic, layout, or naming pattern, check how related Socketry packages handle the same concern. Reuse an established pattern when it fits the package's purpose. When a package establishes or changes a pattern, document the rationale and intended usage in that package's context so later work has a reference. Preserve differences that reflect real domain needs; consistency should make related packages easier to understand, not flatten their APIs into one shape.

## Source and documentation

- Keep authored repository-root Markdown files to lowercase `readme.md`, `license.md`, and `releases.md`.
- Start `license.md` with `# MIT License`.
- Keep `readme.md` human-focused: explain the project, its motivation when useful, how to use it, recent releases, and how to contribute. Follow the `Readme Structure` guide supplied by `bake-readme`.
- Put reusable, package-specific agent guidance in `context/`; keep repository-only instructions and project-owned skills in `.agents/`. Treat the root `agents.md` as repository-owned guidance.
- Avoid duplicating Rust-wide guidance from `bake-agent-context`; add project context for the architecture and decisions that are specific to the crate.
- Follow the Agent Context section in `readme.md` to install and discover shared context and skills. The `bake-agent-context` guide explains the installer's behavior.

## Development and releases

- Keep development tasks in a private `bake/` package. Depend on `socketry-project` there so task tooling does not become a runtime dependency of the published library.
- Use the `cargo:after_version_bump` hook registered by `socketry-project` to update `license.md`, `releases.md`, and the generated sections of `readme.md`, then normalize those files and all Markdown under the public `context/` directory.
- Follow the [Pull Requests guidance](pull-requests.md), including independent adversarial review and local iteration until all non-trivial issues are resolved.
- Follow the `socketry-project-releasing` skill to prepare and publish a release. It links to Bake Cargo task documentation for workflow setup, trusted publishing, reviewers, and tag creation.
