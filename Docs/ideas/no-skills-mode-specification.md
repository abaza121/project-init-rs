# Specification: Skill-Free Codex Invocations by Default

## Objective

Make skill suppression the default for the Rust `project-init` CLI so every Codex subprocess receives no model-visible local, repository, bundled, or installed-plugin skills unless the user opts in with `--skills`. This isolates pipeline behavior from reusable skill instructions without changing `AGENTS.md`, available tools, sandboxing, approval policy, or offline execution.

## Acceptance criteria

1. Skill suppression is enabled when no skill-related option is present.
2. `--skills` is a global opt-in accepted before or after a subcommand, while the former `--no-skills` spelling remains accepted for compatibility.
3. The resolved policy reaches initial analysis, non-interactive generation, resumed workflow generation or repair, and workbench-triggered Codex execution.
4. When suppression is enabled, the adapter discovers every visible `SKILL.md` under repository/user `.agents/skills`, Codex user/system skills, installed plugin caches, and the Unix admin skill root when present.
5. The adapter passes one deterministic `skills.config` command-line override that marks every discovered skill disabled.
6. Discovery follows symlinked directories without looping, deduplicates overlapping roots, escapes arbitrary platform paths as valid TOML strings, and fails before the paid Codex run if a present skill root cannot be read.
7. An empty inventory fails closed because a new Codex installation may not have materialized its bundled skills yet.
8. With `--skills`, existing skill-enabled Codex argument lists and behavior remain unchanged.
9. `--offline` remains deterministic and does not require Codex or skill discovery regardless of skill policy.
10. The historical `baseline/` runner remains unchanged.

## Architecture and data flow

The command-line `Cli` resolves a global skill policy and passes it through additive client and TUI runtime builder methods. The public configuration constructors themselves use the safe skill-free default. This is similar to a C# options object with a conservative default plus an explicit builder override. The Rust adapter borrows the actual subprocess working-directory path, finds skill files, and converts their paths into a TOML array passed with Codex `-c`; no persistent Codex configuration is edited.

Skill roots are derived from the effective `CODEX_HOME`, the user profile, working-directory ancestors, and the platform admin location. A small standard-library directory walker owns a visited-directory set and a sorted skill-file set, keeping output deterministic and preventing symlink cycles. Recoverable filesystem failures use `Result<T, AgentError>` rather than exceptions or partial configuration.

## Explicit behavior

- Skill suppression is the default and remains invocation-scoped.
- `--skills` restores configured skills only for the current invocation.
- A skill-free invocation either disables every discovered skill or returns an error before the main Codex subprocess starts.
- Existing configured tools and project instructions remain available.

## Policy decisions

- Use exact `SKILL.md` paths in `skills.config`. The current Codex skills guide and Codex CLI 0.151.0 accept this form; the current config-reference wording that describes the containing folder did not match observed CLI behavior.
- Scan installed plugin caches because plugin-provided skills can otherwise remain model-visible even when user and repository skill roots are empty.
- Do not mutate `config.toml`, remove skill directories, or create an isolated `CODEX_HOME`; authentication and unrelated user settings must remain intact.

## Invariants

- The skill-enabled argument sequence is unchanged when suppression is explicitly disabled.
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

- Always: preserve existing sandbox and approval arguments, fail closed on incomplete discovery while suppression is active, and test default compatibility.
- Ask first: changing baseline behavior, persistent Codex configuration, or the meaning of `AGENTS.md`.
- Never: delete or rewrite skills, copy authentication into another home, bypass sandboxing, or invoke a paid model in tests.

## Ordered implementation plan

1. Add failing adapter and runtime-constructor tests for the skill-free default.
2. Add failing CLI tests for the default, global `--skills` placement, and compatibility spelling.
3. Set adapter and workbench constructors to the skill-free default and implement CLI opt-in policy resolution.
4. Propagate the resolved policy through analysis, workflow, and TUI configuration.
5. Update user documentation and changelog.
6. Run targeted, full, and lint verification; review the complete diff and improve only justified findings.

## Open questions

No unresolved question blocks implementation. If a future Codex release adds a documented global skills switch, the adapter can replace enumeration without changing the public default or `project-init --skills` contract.
