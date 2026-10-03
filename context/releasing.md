---
type: skill
description: Prepare and follow through a reviewed release for Socketry Rust projects. Use when asked to bump a version, publish a crate, or check release readiness.
---

# Releasing

Use this skill to prepare releases for Socketry Rust projects. Read the root
`agents.md`, `readme.md`, and installed agent context for any project-specific
release steps.

## Prepare a release

Keep one Cargo workspace when its publishable packages share a version, release
notes, and tag. Use separate repositories for packages that need independent
release timing. See [Conventions](conventions.md#repository-and-package-boundaries).

Use the tasks linked by the project's private `bake/` package to bump the
workspace version:

```sh
cargo bake cargo:version:patch
cargo bake cargo:version:minor
cargo bake cargo:version:major
cargo bake cargo:version:bump --version X.Y.Z
```

Choose one version task. The `socketry-project` hook updates `license.md`,
`releases.md`, and generated sections of `readme.md`. Review all generated
changes and ensure the release notes describe the actual changes.

Run the project's required tests and coverage checks using the
`socketry-project-testing` skill. Commit the version and release files, then run
`cargo bake cargo:release` from the clean worktree. This validates and packages
the release candidate; it does not publish or tag it. Open a reviewed pull
request after the task succeeds.

## Publish through GitHub Actions

The standard `publish.yml` workflow publishes a release after its pull request
merges to the configured branch and the `crates-io` environment approves the
deployment. It publishes remaining workspace packages through GitHub OIDC,
creates the version tag after successful uploads, then creates or updates the
matching GitHub Release from `releases.md`.

After merging, check the workflow result, published package versions, version
tag, and GitHub Release. If a publish workflow is still waiting for environment
approval, obtain that approval through the configured reviewer process.

## Set up publishing

For a new repository, use `cargo bake cargo:setup:workflow` to generate the
workflow. Put required environment reviewers in `Cargo.toml`, preview settings
with `cargo bake cargo:setup:github:plan`, and apply them with
`cargo bake cargo:setup:github:apply` after reviewing the plan.

For a crate's first upload, `cargo bake cargo:bootstrap PACKAGE` publishes it
with an owner token and registers its trusted publisher. Keep the token out of
the repository. After the initial upload, use GitHub OIDC for routine releases;
enable trusted-publishing-only mode after the workflow has published
successfully.

For exact arguments and behavior of these tasks, consult the
[Bake Cargo Readme](https://github.com/socketry/bake-cargo-rust/blob/main/readme.md)
and each task's `--help` output.
