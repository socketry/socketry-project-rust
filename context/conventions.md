# Socketry Rust Conventions

Use these conventions for Rust repositories in the Socketry organization. A
project's local `.agents/` files and package-specific documentation can add
requirements for that repository.

## Repository and package boundaries

- Use a separate repository when packages need independent versions, release
  notes, or tags.
- Keep packages in one workspace when they are intentionally released together:
  use one shared version, one root `releases.md`, and one `vVERSION` tag.
- Name crates for their public purpose. Use the `socketry-` prefix where needed
  to identify Socketry packages in the flat crates.io namespace; keep Rust
  module paths semantic and concise.
- Use Cargo's standard `src/`, `tests/`, and `examples/` directories. Use
  integration tests for public behavior across crate boundaries.

## Source naming

- Use `snake_case` for modules, source files, functions, methods, and variables.
  Use `UpperCamelCase` for structs, enums, traits, and type parameters.
- Preserve established initialisms in type and trait names: write
  `HTMLRenderer`, `HTTPClient`, and `URLParser`, rather than
  `HtmlRenderer`, `HttpClient`, or `UrlParser`. Keep filenames lowercase
  `snake_case`, such as `html_renderer.rs`, `http_client.rs`, and
  `url_parser.rs`.
- Prefer clear, consistent names. Avoid abbreviations unless they are an
  established domain initialism or required by an external API.
- Give each primary public struct, enum, or trait its own source file, named
  after the item using lowercase `snake_case`. Keep small, closely related
  helper types alongside it when that makes the code easier to understand.

## Source and documentation
- Keep repository-root Markdown files to lowercase `readme.md`, `license.md`,
  and `releases.md`.
- Start `license.md` with `# MIT License`.
- Keep `readme.md` human-focused: explain the project, its motivation when
  useful, how to use it, recent releases, and how to contribute. Follow the
  `Readme Structure` guide supplied by `bake-readme`.
- Put public, package-specific agent guidance in `context/`. Keep repository-only
  instructions directly in `.agents/`. The generated `.agents/context/`
  directory is ignored by Git.
- Avoid duplicating Rust-wide guidance from `bake-agent-context`; add project
  context for the architecture and decisions that are specific to the crate.

## Development and releases

- Keep development tasks in a private `bake/` package. Depend on
  `socketry-project` there so task tooling does not become a runtime dependency
  of the published library.
- Use the `cargo:after_version_bump` hook registered by `socketry-project` to
  update `license.md`, `releases.md`, and the generated sections of `readme.md`.
- Review generated changes before committing a release pull request. The
  `bake-cargo` publishing guide describes workflow setup, trusted publishing,
  reviewer settings, and tag creation.
