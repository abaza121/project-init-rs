# Hackathon Baseline

## Status

The pre-existing [project-initiation-pipeline](https://github.com/abaza121/project-initiation-pipeline) is the hackathon baseline. Its two upstream commits are retained in this repository's Git ancestry, and its files are preserved without weakening or rewriting them under [`baseline/`](../baseline/).

The baseline remains independently runnable on Windows:

```bat
baseline\run-codex-docs-pipeline.bat -f baseline\project-prompt.txt
```

It requires Windows PowerShell plus a `codex` executable on `PATH`, or a `CODEX_BIN` environment variable pointing to that executable.

## Input

The batch entry point accepts either:

- an inline natural-language project brief; or
- `-f <path>` referencing a prompt file.

If neither is supplied, it prompts on standard input. The bundled demonstration is a short calm fishing VR game brief.

## Pipeline stages

1. The batch wrapper validates the prompt and locates PowerShell and Codex.
2. The PowerShell runner resolves the project root, prompt, writable Codex home, and optional latest root-level `log-*.md` template.
3. It expands the brief into a fixed 14-entry modeled prompt sequence.
4. It combines that sequence, required artifact list, run metadata, and optional log template into one large Codex instruction.
5. It launches one ephemeral `codex exec` process with workspace-write access.
6. Codex creates the documentation package and a last-message summary.

## Outputs

The fixed artifact contract is:

- `Docs/README.md`
- `Docs/requirements.md`
- `Docs/SWOT.md`
- `Docs/MissionVision.md`
- `Docs/VisualIdentity.md`
- `Docs/BrandPrompt.md`
- `Docs/TechnicalArchitecture.md`
- at least five `Docs/Research-NN-<topic>.md` files
- one timestamped `Docs/log-*.md`
- `Docs/_pipeline-last-message.txt`

The root also acts as a Codex skill. Native skill mode instructs Codex to create the same staged package directly; runner mode reproduces the PowerShell automation.

## Useful baseline characteristics

- It accepts a very small brief and produces a broad, ordered project-document set.
- Later document prompts explicitly build on earlier artifacts.
- The command is reproducible, supports inline or file input, and returns the nested Codex exit code.
- It records a timestamped session log and can use an earlier log as a structural template.
- The skill distinguishes native execution from runner reproduction.

## Assumptions

- A single generated instruction can coordinate the entire project-initiation run.
- Markdown files are sufficient both as intermediate state and final state.
- A fixed artifact sequence is appropriate for every project.
- The agent can resolve missing requirements, naming, research topics, and architecture without an interactive clarification gate.
- Coherence can be checked by instructions at generation time instead of by persistent typed relationships and deterministic validation.
- The environment is Windows when using the reproducible runner.

## Current limitations

- There is no typed, persistent project knowledge model or resumable lifecycle.
- Facts, requirements, assumptions, evidence, and decisions are not stored as distinct records.
- Provenance is narrative rather than queryable; inferred statements can be difficult to distinguish from user requirements.
- The user cannot inspect, accept, reject, edit, or defer consequential assumptions during a run.
- Research evidence is not persisted separately from conclusions.
- Traceability between requirements, answers, evidence, decisions, and document sections is not represented as data.
- Verification is prompt-directed and has no deterministic contradiction, traceability, or required-artifact result model.
- Repair attempts are not explicitly bounded.
- The fixed sequence may spend effort on low-impact documents before high-impact uncertainties are resolved.
- The runner depends on a live Codex invocation, so it cannot be tested fully offline with deterministic fixtures.
- It has no relational-versus-semantic ablation mode or retrieval observability.

## Comparison boundary

The Rust implementation must not claim superiority merely because it uses a different language. Evaluation must compare the same brief and record observable measures such as execution time, user questions, agent calls, repair passes, trace coverage, unresolved high-impact uncertainty, and artifact validity. Results belong under `evaluation/`; they must never be fabricated.
