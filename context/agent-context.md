# Agent Context

Distribute useful package guidance with the package, and keep repository-only
instructions with the repository. This follows the Socketry Ruby project model
of maintaining context at its source and installing it for local agent use.

## Package context

Write reusable guidance in the tracked `context/` directory. Keep it specific
to the package: explain its architecture, important invariants, supported use,
and project decisions that are not obvious from the API. General Rust guidance
belongs in `bake-agent-context`; project-specific guidance belongs in
`socketry-project` or in the package that owns the behavior.

Use descriptive lowercase Markdown names and a clear title. Keep one guide per
topic, use ordinary Markdown without a separate index file, and link related
guides to each other. Include `context/**` in the Cargo package when the
guidance should be available to downstream projects.

Agent context supports development and maintenance. Keep the user-facing
overview and first examples in `readme.md`, and document the API with Rust doc
comments. Avoid turning the Readme into a repository handbook.

## Repository-only guidance

Put instructions and skills that only apply to this checkout in `.agents/`.
Do not publish credentials, local paths, or private process details as package
context. Keep generated dependency context under `.agents/context/` and ignore
that directory in Git; refresh it from its dependency rather than editing the
generated copy.

Bake Agent Context also maintains an `agents.md` index that points agents to
installed context. Let its task maintain that index instead of duplicating the
context contents in a manually maintained root guide.

## Install and update

Add `bake-agent-context` to the private `bake/` package directly or through
`socketry-project`. Then install guidance from dependencies:

```sh
cargo bake agent:context:install
```

To install only one package's context, pass its package name:

```sh
cargo bake agent:context:install --package socketry-project
```

Run the task again after updating a context-providing dependency. Edit the
tracked source files under that package's `context/` directory, not the
generated local copies.
