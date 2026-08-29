# Project Init: Complete Project Loop

## Problem Statement

How might we let a person or automation provide an incomplete project brief, answer only the questions that matter, and receive a complete, validated, traceable documentation package through one resumable command?

## Recommended Direction

Build a deterministic, resumable orchestrator around three components:

1. A step runner derives the next legal action from authoritative SQLite state.
2. A typed artifact dependency graph controls document generation, ordering, and invalidation.
3. A run-scoped approval policy determines when execution pauses for human authority.

The workflow engine remains deterministic. Codex may use every tool made available by its configured CLI sandbox to propose findings, evidence, decisions, repairs, or document content, but validated workflow services decide whether those proposals enter authoritative state. Tool use and outcomes remain auditable.

The workflow is:

```text
Brief
  -> Analysis
  -> Questions and answers
  -> Research
  -> Proposed decisions and approvals
  -> Document generation
  -> Validation and bounded repair
  -> Complete package
```

A failed or interrupted run can resume without repeating completed work or creating duplicate records.

## Product Contract

The human entry points support both explicit project creation and direct execution:

```text
project-init new --brief project-prompt.txt --name "Project Name" --run
project-init run --brief project-prompt.txt --name "Project Name"
project-init run <project-id>
```

Automation receives deterministic primitives:

```text
project-init step <project-id> --json
project-init status <project-id> --json
project-init approve <decision-id>
project-init reject <decision-id> --reason "..."
```

Every pause reports a stable reason such as `question_required`, `approval_required`, `research_unavailable`, or `validation_failed`.

## Approval Policies

The caller selects the policy when a run starts. The effective policy is recorded with that run so resumption remains deterministic.

- **Strict:** Pause for every proposed decision.
- **Consequential:** Pause for high-impact, weakly supported, contradictory, irreversible, naming, and brand decisions.
- **Autonomous:** Continue where safe while preserving assumptions and warnings. It never silently accepts unsupported consequential claims.

Changing policy during a resumed workflow is an explicit new run decision rather than an invisible mutation of the earlier run.

## Artifact Graph

The MVP uses a fixed, code-defined graph:

```text
Brief + Answers + Evidence + Decisions
              |
              v
         Requirements
          |-- SWOT -- Mission/Vision -- Visual Identity -- Brand Prompt
          |-- Technical Architecture
          `-- Project-specific Research
                        |
                        v
            Traceability + Session Log
                        |
                        v
                Validation Report
```

Required output:

- `README.md`
- `Requirements.md`
- `SWOT.md`
- `MissionVision.md`
- `VisualIdentity.md`
- `BrandPrompt.md`
- `TechnicalArchitecture.md`
- at least five `Research-NN-<topic>.md` files
- a timestamped session log
- `Assumptions.md`
- `OpenQuestions.md`
- `Traceability.md`
- `ValidationReport.md`

When an authoritative input changes, only dependent artifacts become stale. Regeneration updates their stored hashes and metadata.

## Manual Override Contract

Generated files are never silently overwritten after manual edits. When file content differs from its stored hash, the workflow:

1. Preserves the edited file.
2. Registers it as a manual artifact revision.
3. Records its previous generated revision and new hash.
4. Validates required structure, citations, and stable IDs.
5. Marks downstream artifacts stale.
6. Regenerates dependants without replacing the override.

The user may explicitly remove an override to return an artifact to generated control.

## Validation and Repair Contract

Every validation code declares:

- whether deterministic repair exists;
- whether agent-assisted repair is permitted;
- which inputs the repair may change;
- its maximum attempt count;
- which human action resolves it when automation cannot establish truth safely;
- how the repaired state is revalidated.

The workflow attempts deterministic repair first, then bounded Codex-assisted repair, then pauses for human authority. Every failure has either an automated resolution strategy or an explicit escalation path. Automation never suppresses, downgrades, or merely accepts a blocking failure to manufacture completion.

## Definition of Complete

A project may enter `Complete` only when:

- no consequential question remains open;
- every accepted decision has provenance;
- research-derived claims cite stored evidence;
- every required artifact exists and is current;
- stored hashes match generated files or an explicit validated override;
- the latest validation run passed;
- no policy-required approval remains unresolved.

Having files on disk is not sufficient.

## Key Assumptions to Validate

- [ ] **Workflow steps can be retried safely.** Interrupt a run at every step boundary, resume it, and verify that no records or display IDs are duplicated.
- [ ] **Dependencies provide reliable invalidation.** Change representative answers and decisions, then verify that exactly the affected artifacts become stale.
- [ ] **Three approval policies cover real usage.** Exercise the same fixture under all policies and compare pauses, accepted decisions, and warnings.
- [ ] **Automation does not require terminal behavior.** Complete a fixture using only `step --json`, approval commands, and machine-readable status.
- [ ] **Structured agent boundaries are sufficient.** Reject malformed, unsupported, or contradictory Codex proposals without partially mutating state.
- [ ] **Manual edits survive regeneration.** Change a generated artifact, register the override, resume the workflow, and prove that the edited file remains intact while dependants refresh.
- [ ] **The full package remains internally consistent.** Run the fishing fixture end to end and verify cross-document terminology, stable IDs, citations, hashes, and validation results.

## MVP Scope

- One new-project run and existing-project resume path.
- Deterministic next-step planning.
- Sequential execution with one authoritative mutation per step.
- Run-scoped approval policy and explicit decision approval records.
- Workflow operations for evidence, decisions, validation runs, document metadata, and manual overrides.
- Fixed artifact registry with dependency-based staleness.
- Full baseline document package.
- Structured validation with bounded deterministic and Codex-assisted repair.
- Interactive question and approval handling.
- Non-interactive JSON status and step execution.
- Offline behavior that pauses honestly when external evidence is required.

## Not Doing

- **General-purpose workflow graphs** -- the fixed package must prove the model first.
- **User-defined document types** -- this requires a stable artifact contract that does not yet exist.
- **Parallel execution** -- sequential steps are easier to retry, audit, and validate.
- **Collaborative locking** -- multi-user invocation is supported, but concurrent editing is a separate problem.
- **Semantic/vector retrieval** -- relational state is sufficient for the first complete loop.
- **Unbounded autonomous repair** -- this risks hiding failures and consuming unpredictable resources.
- **Silent consequential decisions** -- this conflicts with the product's trust model.
- **Rich document editing inside the TUI** -- generated files remain the editing surface.
- **Automatic publication or deployment** -- completion produces a local validated package only.

## Remaining Implementation Decisions

- Define the smallest persisted workflow-run and approval schema that guarantees deterministic resumption.
- Define which document edits qualify as structurally valid overrides.
- Map each validation code to deterministic repair, Codex-assisted repair, or explicit human escalation.
- Define a bounded Codex capability policy that exposes all configured tools without bypassing the caller's sandbox or approval controls.
