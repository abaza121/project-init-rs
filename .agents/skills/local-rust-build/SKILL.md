---
name: local-rust-build
description: Update this repository's changelog, then build and package its Rust CLI locally into the project-root builds folder. Use for local builds and local release artifacts; do not use for CI, crates.io, GitHub Releases, or deployment.
---

# Local Rust Build

Update the unreleased changelog and build a verified local release artifact without publishing or changing the project version.

## Update the changelog

Before building:

1. Read `Docs/CHANGELOG_GUIDELINES.md` completely.
2. Inspect `git status`, unstaged and staged diffs, and relevant recent history to understand the actual changes being built.
3. Update `CHANGELOG.md` under `## [Unreleased]` with any notable changes that are not already represented.
4. Update an existing entry when it describes the same logical change; do not add duplicates or copy commit messages mechanically.

Do not create a released version heading, assign a version, or add an entry solely because a local build was run. If the changelog already represents every notable change, leave it unchanged.

## Run the build

From the repository root, run:

```powershell
& .\.agents\skills\local-rust-build\scripts\build.ps1
```

The script must complete these checks before copying a build artifact:

1. `cargo fmt --check`
2. Clippy for all targets and features with warnings denied
3. Tests for all targets and features
4. A release build using the committed `Cargo.lock`

Stop immediately if any command fails. Do not publish, create tags, update versions, or install global tools.

## Output

Write the executable and its SHA-256 checksum to:

```text
builds/<package>-v<version>-<target>/
```

Keep `builds/` untracked. After success, report the absolute executable path and checksum to the user.
