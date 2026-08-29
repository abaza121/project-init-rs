# Specification: Skill-Free Codex Invocations

## Objective

Add an opt-in `--no-skills` mode to the Rust `project-init` CLI so every Codex subprocess started by that invocation receives no model-visible local, repository, bundled, or installed-plugin skills. This isolates pipeline behavior from reusable skill instructions without changing `AGENTS.md`, available tools, sandboxing, approval policy, or offline execution.

## Acceptance criteria

1. `--no-skills` is a global option accepted before or after a subcommand.
2. The option reaches initial analysis, non-interactive generation, resumed workflow generation or repair, and workbench-triggered Codex execution.
3. When enabled, the adapter discovers every visible `SKILL.md` under repository/user `.agents/skills`, Codex user/system skills, installed plugin caches, and the Unix admin skill root when present.
4. The adapter passes one deterministic `skills.config` command-line override that marks every discovered skill disabled.
5. Discovery follows symlinked directories without looping, deduplicates overlapping roots, escapes arbitrary platform paths as valid TOML strings, and fails before the paid Codex run if a present skill root cannot be read.
6. An empty inventory fails closed because a new Codex installation may not have materialized its bundled skills yet.
7. When the option is omitted, existing Codex argument lists and behavior remain unchanged.
8. `--offline` remains deterministic and does not require Codex or skill discovery even when `--no-skills` is also present.
9. The historical `baseline/` runner remains unchanged.

## Architecture and data flow

The command-line `Cli` owns a global boolean and passes it through additive client and TUI runtime builder methods, preserving the existing public configuration constructors. This is similar to applying an optional setting to a C# builder without adding a required property to an existing options record. The Rust adapter borrows the actual subprocess working-directory path, finds skill files, and converts their paths into a TOML array passed with Codex `-c`; no persistent Codex configuration is edited.

Skill roots are derived from the effective `CODEX_HOME`, the user profile, working-directory ancestors, and the platform admin location. A small standard-library directory walker owns a visited-directory set and a sorted skill-file set, keeping output deterministic and preventing symlink cycles. Recoverable filesystem failures use `Result<T, AgentError>` rather than exceptions or partial configuration.

## Explicit behavior

- Skill suppression is opt-in and invocation-scoped.
- A no-skills request either disables every discovered skill or returns an error before the main Codex subprocess starts.
- Existing configured tools and project instructions remain available.

## Policy decisions

- Use exact `SKILL.md` paths in `skills.config`. The current Codex skills guide and Codex CLI 0.151.0 accept this form; the current config-reference wording that describes the containing folder did not match observed CLI behavior.
- Scan installed plugin caches because plugin-provided skills can otherwise remain model-visible even when user and repository skill roots are empty.
- Do not mutate `config.toml`, remove skill directories, or create an isolated `CODEX_HOME`; authentication and unrelated user settings must remain intact.

## Invariants

- The default argument sequence is unchanged when skill suppression is disabled.
- Paths are sorted and deduplicated before serialization.
- A rejected or unreadable discovery operation launches no paid Codex task.
- User-owned skill files and configuration are never modified.

## Evaluator risks

- Paths containing spaces, quotes, backslashes, or non-ASCII characters.
- Overlapping roots and symlink cycles.
- Missing optional roots versus unreadable existing roots.
- System skills materialized by a newly installed Codex version only at startup.
- Global flag placement before and after subcommands.
- Workbench resume losing the original invocation option.

## Commands

```text
Targeted: cargo test --all-features agents::codex::tests
CLI:      cargo test --all-features --bin project-init
Format:   cargo fmt --check
Check:    cargo check --all-features
Test:     cargo test --all-features
Lint:     cargo clippy --all-targets --all-features -- -D warnings
```

## Dependencies and boundaries

No new dependency is required. The standard library provides directory traversal, path handling, and deterministic sets; existing `serde_json` safely produces TOML-compatible basic-string escaping for path text.

- Always: preserve existing sandbox and approval arguments, fail closed on incomplete enabled discovery, and test default compatibility.
- Ask first: changing baseline behavior, persistent Codex configuration, or the meaning of `AGENTS.md`.
- Never: delete or rewrite skills, copy authentication into another home, bypass sandboxing, or invoke a paid model in tests.

## Ordered implementation plan

1. Add failing adapter tests for discovery and command serialization.
2. Add failing CLI tests for global flag placement and default behavior.
3. Implement adapter discovery and opt-in argument injection.
4. Propagate the option through CLI, workflow, and TUI configuration.
5. Update user documentation and changelog.
6. Run targeted, full, and lint verification; review the complete diff and improve only justified findings.

## Open questions

No unresolved question blocks implementation. If a future Codex release adds a documented global skills switch, the adapter can replace enumeration without changing the public `project-init --no-skills` contract.
