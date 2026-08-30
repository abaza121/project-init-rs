# Implementation Plan: Evaluator-Guided Prompt Optimization

## Status

Proposed on 2026-08-30 from the docs-only evaluation of pipeline `v0.3.0` project `1e651ba8-fe0d-479f-8f71-e1dc49741145`. The evaluator rubric and reports are evidence for this plan, not instructions executed by the pipeline. Implementation must begin only after human review of this plan.

## Overview

Improve the prompts used by the existing Codex analyzer, research workers, documentation generator, and repair pass so generated packages preserve brief language, distinguish authority from inference, provide one-to-one acceptance criteria, and contain semantically valid requirement/decision/evidence traces. Keep the current analyzer architecture, structured-output schemas, staging safety, and required artifact set. Add one small domain-neutral clarification guardrail because the current downstream formatter rewrites every platform unknown as a VR question and can corrupt otherwise-correct analysis output.

## Evaluator Analysis

The evaluator assigns 100 points across seven qualitative dimensions. Deterministic metrics explain deductions but do not independently determine the total score, and finding counts are not additive. This is why 77 evidence findings cost only 2.5 Evidence Quality points while 13 invalid trace relationships reduced Traceability to zero.

| Dimension | Current | Maximum | Main signal |
|---|---:|---:|---|
| Brief Fidelity | 14.3 | 20 | One explicit brief requirement was only partially preserved in an identifiable generated requirement. |
| Assumption Discipline | 4.7 | 15 | Three consequential inferred choices were stated as settled; assumption labeling was 0.0%. |
| Cross-Document Consistency | 15.0 | 15 | No material contradictions were found. |
| Evidence Quality | 12.5 | 15 | Evidence was generally credible, but 74 claims were decorative and three decisions lacked explicit evidence relationships. |
| Requirements to Decision Traceability | 0.0 | 15 | Coverage was 57.1%, but 13 links were judged semantically unrelated, triggering traceability theater. |
| Actionability | 8.0 | 15 | Architecture and blockers were useful, but acceptance-criteria coverage was measured at 0.0%. |
| Artifact Completeness and Navigation | 5.0 | 5 | All submitted links worked and navigation was sound. |
| **DRPFS** | **59.5** | **100** | The largest recoverable losses are traceability, authority labeling, and explicit acceptance criteria. |

Current deterministic signals:

| Metric | Current | Direction |
|---|---:|---|
| Acceptance Criteria Coverage | 0.0% | Raise |
| Requirement Traceability Coverage | 57.1% | Raise |
| Unsupported Decision Rate | 81.4% | Lower |
| High-Impact Assumption Labeling Rate | 0.0% | Raise |
| Evidence Linkage Rate | 72.7% | Raise |
| Broken Internal Links | 0 | Preserve |

The docs-only run correctly reports several metrics as unavailable, including required-artifact completion and user-answer adoption. Prompt work must not manufacture those values or claim improvement in unavailable metrics.

## Root-Cause Map

| Observed failure | Current cause | Optimization response |
|---|---|---|
| Partial brief fidelity | `analysis_prompt` asks for analysis but gives no atomic coverage or literal-source rule. Application provenance recognizes a user-brief finding only when the entire generated statement occurs verbatim in the brief. The evaluator also relies on deterministic token overlap. | Require a complete inventory of material brief clauses, retain exact contiguous wording for user-sourced findings, and forbid paraphrase from replacing the only source-preserving record. |
| Malformed VR clarification | `clarification_copy` converts every statement containing `platform` into “Which VR platform...”, even for a conventional browser game. | Make platform clarification domain-neutral and test a non-VR browser fixture. |
| Assumption promotion and high UDR | Research prompts ask for recommendations, while generated decisions combine externally supported facts, product choices, and tuning hypotheses into one adopted ADR. | Require explicit authority labels and atomize evidence-backed constraints, recommendations, assumptions, and user-owned decisions. |
| Traceability theater | `documentation_prompt` requests consistency but no link semantics. The generated package uses broad ADRs, grouped evidence ranges, composite sources such as `REQ-001/006`, and synthetic links without demonstrated subject overlap. | Require one subject per decision, direct IDs rather than ranges, a stated rationale for every edge, and removal of a link when relevance cannot be shown. |
| Acceptance coverage 0% | Requirements contain an “Acceptance summary” column and a separate test catalog, but the evaluator cannot establish a complete one-to-one criterion for every applicable requirement. | Require an explicitly named, objective pass/fail acceptance criterion on every implementable requirement and an exact requirement-to-test mapping. |
| Decorative research | Evidence is repeated in research files, `README.md`, `Traceability.md`, the session log, and `ValidationReport.md`; many repetitions do not point to a consequential decision. | Keep one canonical evidence record, link only decision-relevant evidence, and make other artifacts reference the canonical ID instead of restating the claim. |
| Weak repair leverage | `repair_prompt` preserves paths and overrides but does not restate the semantic quality contract or require a coverage self-audit. | Share the same quality contract between generation and repair and require semantic revalidation before completion. |

## Architecture Decisions

- Preserve `ANALYSIS_SCHEMA`, research schemas, the current client traits, SQLite model, required artifact paths, and safety delimiters. This is prompt optimization around the current analyzer, not an analyzer replacement.
- Keep source authority application-controlled. The optimized analysis prompt must produce literal brief-backed statements compatible with the existing `assigned_source` check rather than trusting model-declared provenance.
- Add a shared prompt-quality contract inside `src/agents/codex.rs` so generation and repair cannot drift. Every new or changed Rust function and test receives a behavior-focused `///` comment.
- Keep decisions atomic. A decision may link to several relevant sources, but a source range or composite ID must not stand in for individually meaningful relationships.
- Treat research as evidence, not authority over preferences. External facts may constrain a choice; tuning values, product scope, and stakeholder preferences remain labeled assumptions or open decisions unless the snapshot records proper authority.
- Optimize for evaluator-legible semantics without copying evaluator output into generated packages or teaching the generator to game a specific brief.
- Use the evaluator as an external acceptance gate. Preserve its independence and continue evaluating generated artifacts read-only.

## Dependency Graph

```text
Evaluator-backed quality gate
  -> faithful analysis prompt + neutral clarification
      -> authority-preserving research prompts
          -> semantic documentation prompt
              -> shared repair/self-audit contract
                  -> paired and multi-brief evaluation
                      -> changelog and final quality record
```

## Task 1: Establish the prompt-quality baseline and acceptance gate

**Description:** Record the attached docs-only result as the baseline, map each scored failure to an owned prompt or deterministic guardrail, and define the controlled evaluator command and quality thresholds before changing prompts. This prevents cherry-picking individual findings or optimizing only for Borrowed Orbit.

**Acceptance criteria:**

- [ ] The baseline record contains the seven dimension scores, six available deterministic signals, evaluated package identity, brief, pipeline revision, and evaluator mode.
- [ ] Unavailable metrics remain explicitly unavailable, and the quality gate distinguishes qualitative DRPFS dimensions from deterministic diagnostic metrics.
- [ ] A reproducible docs-only `evaluate` or paired `compare` command is documented for Borrowed Orbit and for the existing five-case suite.

**Verification:**

- [ ] Manual check: every available field from `validation-report.json` appears once in the baseline matrix.
- [ ] Run from the evaluator repository: `cargo run -- evaluate --brief <brief> --generated <Docs> --output <report-directory>`.
- [ ] Confirm the baseline rerun reports DRPFS 59.5 and zero broken internal links before using it for comparison.

**Dependencies:** None.

**Files likely touched:**

- `evaluation/README.md`
- `evaluation/prompt-quality-gate.md` (new)

**Estimated scope:** Small, 2 files.

## Task 2: Make initial analysis complete, literal, and domain-neutral

**Description:** Write adversarial tests first, then strengthen `analysis_prompt` so it extracts every material explicit requirement as an atomic finding using exact contiguous source wording, keeps unresolved choices domain-neutral, and introduces no concept absent from the brief. Replace the VR-specific platform clarification template with wording derived from the actual unknown.

**Acceptance criteria:**

- [ ] A browser-game fixture produces no VR, headset, or WebXR clarification unless those concepts occur in the brief.
- [ ] Every user-backed finding used for provenance is an exact non-empty substring of the supplied brief; inferred findings are labeled as inference and require confirmation when consequential.
- [ ] The prompt explicitly performs a final coverage check for goals, constraints, exclusions, success criteria, named users/platforms, and declared unknowns without merging unrelated clauses.

**Verification:**

- [ ] Tests pass: `cargo test analysis_prompt`.
- [ ] Tests pass: `cargo test clarification_copy`.
- [ ] Existing injection-delimiting and structured-analysis tests remain green.

**Dependencies:** Task 1.

**Files likely touched:**

- `src/agents/codex.rs`
- `src/workflow/mod.rs`

**Estimated scope:** Medium, 2 files.

## Checkpoint: Source fidelity

- [ ] Baseline and targets are reviewable before behavioral changes continue.
- [ ] The Borrowed Orbit brief cannot produce the malformed VR question.
- [ ] Prompt-injection delimiters, no-tool analysis behavior, structured-output bounds, and existing analyzer schemas remain unchanged.

## Task 3: Preserve authority and evidence boundaries in research prompts

**Description:** Tighten the single-question research, batch-planning, and judgment prompts so answers distinguish supported facts from project inference, recommendations, tuning hypotheses, and stakeholder-owned choices. Keep each worker focused on one question and stop evidence from being used as authority for unrelated product decisions.

**Acceptance criteria:**

- [ ] Research notes label each non-sourced recommendation or design inference and never present it as a user requirement or settled project fact.
- [ ] Every evidence claim has a direct HTTPS source that supports that exact claim, while preference questions that cannot be resolved by research remain open or return a narrowly bounded recommendation.
- [ ] The judge rejects bundled, stale, internally inconsistent, unsupported, or authority-crossing answers instead of polishing them into adopted certainty.

**Verification:**

- [ ] Tests pass: `cargo test research_prompt`.
- [ ] Tests pass: `cargo test research_plan_prompt`.
- [ ] Tests pass: `cargo test research_judgment_prompt`.

**Dependencies:** Task 2.

**Files likely touched:**

- `src/agents/codex.rs`

**Estimated scope:** Small, 1 file.

## Task 4: Generate an evaluator-legible canonical package

**Description:** Expand `documentation_prompt` from a general consistency request into an artifact-specific semantic contract. Require canonical registers for brief clauses, requirements, assumptions, open questions, decisions, evidence, acceptance criteria, and trace links while keeping the existing 17 required paths and the project's useful document depth.

**Acceptance criteria:**

- [ ] Every material brief clause maps to at least one identified requirement, and every implementable requirement contains an explicitly named objective pass/fail acceptance criterion plus an exact test or verification reference.
- [ ] Every consequential decision is atomic and declares status, provenance type, exact source IDs, and any relevant evidence; every important requirement links to at least one semantically overlapping decision or constraint without ID ranges or unrelated filler links.
- [ ] Every research claim has one canonical evidence record and either an explicit decision relationship or no place in the package; assumptions and unresolved questions remain visibly non-authoritative across all artifacts.

**Verification:**

- [ ] Tests pass: `cargo test documentation_prompt`.
- [ ] A generated fixture contains no grouped trace IDs such as `REQ-001/006` or evidence ranges used as a substitute for exact edges.
- [ ] Manual package check: README, log, and validation report link to canonical records instead of duplicating evidence claims.

**Dependencies:** Tasks 2 and 3.

**Files likely touched:**

- `src/agents/codex.rs`

**Estimated scope:** Medium, 1 file.

## Task 5: Share the semantic contract with repair and self-audit

**Description:** Reuse the documentation quality contract in `repair_prompt` and require both generation and repair to calculate an internal coverage checklist before returning. Repairs must change or remove semantically invalid relationships rather than add more IDs, prose, or decorative citations.

**Acceptance criteria:**

- [ ] Generation and repair enforce the same definitions of explicit requirement coverage, acceptance criteria, valid provenance, meaningful traceability, and decision-linked evidence.
- [ ] The self-audit detects uncovered important requirements, missing acceptance criteria, unlabeled consequential assumptions, unsupported decisions, unlinked research, and invalid trace subjects before the staged package is accepted.
- [ ] Repair preserves manual overrides and unrelated content while removing invalid links or downgrading unsupported decisions to assumptions/open questions.

**Verification:**

- [ ] Tests pass: `cargo test repair_prompt`.
- [ ] Tests prove generation and repair include the same quality-contract marker exactly once.
- [ ] Existing manual-override, path-presence, untrusted-context, and repair-bound tests remain green.

**Dependencies:** Task 4.

**Files likely touched:**

- `src/agents/codex.rs`

**Estimated scope:** Small, 1 file.

## Checkpoint: Prompt system complete

- [ ] Analysis, research, generation, and repair prompts use compatible authority vocabulary.
- [ ] No structured schema, database migration, public API, required artifact path, or external-write permission changed.
- [ ] All new and changed Rust functions, including tests, satisfy the repository documentation-comment policy.

## Task 6: Run paired evaluation and close the regression loop

**Description:** Generate a fresh Borrowed Orbit candidate under the same model, settings, approval policy, and resource boundary as the baseline; compare it read-only with the evaluator; then run the existing five project briefs to guard against overfitting. Record measured results and add one concise unreleased changelog entry following `Docs/CHANGELOG_GUIDELINES.md`.

**Acceptance criteria:**

- [ ] Borrowed Orbit reaches DRPFS at least 80, Brief Fidelity at least 18, Assumption Discipline at least 12, Traceability at least 12, and Actionability at least 13 while preserving Consistency 15, Artifact Quality 5, and zero broken links.
- [ ] Borrowed Orbit reaches at least 90% acceptance coverage, 90% requirement traceability, 90% high-impact assumption labeling, and 90% evidence linkage, with unsupported decisions at or below 25%.
- [ ] Across the five-case suite, at least four cases improve, no case regresses by more than 3 DRPFS points, and median DRPFS improves by at least 10 points under recorded equivalent resources.

**Verification:**

- [ ] Evaluator comparison succeeds: `cargo run -- compare --brief <brief> --baseline <baseline-Docs> --candidate <candidate-Docs> --output <comparison-directory>`.
- [ ] Project checks pass: `cargo fmt --check`, `cargo check`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Fresh diff review confirms prompt safety, Rust comments, changelog placement under `Unreleased`, and no overwrite of pre-existing worktree changes.

**Dependencies:** Tasks 1-5.

**Files likely touched:**

- `evaluation/README.md`
- `evaluation/prompt-quality-gate.md`
- `CHANGELOG.md`

**Estimated scope:** Medium, 3 files plus generated evaluation outputs kept outside authoritative source artifacts.

## Checkpoint: Ready for review

- [ ] All targeted and full tests pass.
- [ ] The evaluator reports and package paths are preserved for independent inspection.
- [ ] Score gains come from semantic coverage and provenance, not missing artifacts, suppressed unknowns, duplicated citations, or resource differences.
- [ ] Human review approves the measured result before release or further prompt expansion.

## Risks and Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Prompt grows too long and reduces compliance | High | Use concise shared rules, artifact-specific responsibilities, and tests for required clauses; remove repeated wording. |
| Literal brief preservation produces awkward requirements | Medium | Keep an exact source clause as provenance and a separate normalized requirement; never replace the source clause with the paraphrase. |
| Model adds IDs that look valid but remain semantically unrelated | High | Require edge rationale and exact subject overlap; self-audit must remove unsupported edges rather than maximizing link count. |
| Research prompt still settles stakeholder preferences | High | Make authority classification mandatory and fail closed when evidence can only inform, not decide, the question. |
| Optimizing for Borrowed Orbit harms other project types | High | Gate on the existing five-case suite and preserve a maximum allowed per-case regression. |
| Evaluator extraction counts Markdown scaffolding as content | Medium | Use canonical typed sections and explicit status/provenance fields; do not optimize by hiding legitimate unknowns or flattening navigation. |
| Live generation variance obscures prompt effect | Medium | Record model/settings/resources, use paired runs, retain raw reports, and repeat any threshold-edge case before acceptance. |
| Existing dirty edits in `src/agents/codex.rs` or `CHANGELOG.md` are overwritten | High | Inspect staged and unstaged diffs before each patch, merge narrowly, and never replace whole files. |

## Open Questions

- Should DRPFS 80 be the release gate, or should the first implementation cycle require the stricter dimension and deterministic thresholds even if overall DRPFS already exceeds 80?
- Should controlled live evaluation run once per brief or three times per brief to estimate generation variance? Three runs provide better confidence but materially increase time and model usage.

