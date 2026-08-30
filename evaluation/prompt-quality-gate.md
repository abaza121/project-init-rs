# Prompt Quality Gate

## Objective

Measure whether changes to the existing analyzer, research, documentation, and repair prompts produce more decision-ready project foundations without changing public schemas, required artifacts, or evaluator independence.

The evaluator rubric and reports are read-only evidence. They are not runtime instructions for the pipeline.

## Baseline

| Field | Value |
|---|---|
| Pipeline version | `0.3.0` |
| Evaluated project | `Borrowed Orbit` |
| Project ID | `1e651ba8-fe0d-479f-8f71-e1dc49741145` |
| Evaluator mode | Docs only |
| Evaluator revision | `aea6530` |
| Evaluator report | `project-initiation-evaluator/reports/pipeline-2-v0.3.0-1e651ba8-docs-only/validation-report.json` |
| Generated package | `builds/project-init-v0.3.0-x86_64-pc-windows-msvc/.project-init/projects/1e651ba8-fe0d-479f-8f71-e1dc49741145/Docs` |

### DRPFS dimensions

| Dimension | Baseline | Maximum |
|---|---:|---:|
| Brief Fidelity | 14.3 | 20 |
| Assumption Discipline | 4.7 | 15 |
| Cross-Document Consistency | 15.0 | 15 |
| Evidence Quality | 12.5 | 15 |
| Requirements to Decision Traceability | 0.0 | 15 |
| Actionability | 8.0 | 15 |
| Artifact Completeness and Navigation | 5.0 | 5 |
| **DRPFS** | **59.5** | **100** |

### Available deterministic signals

| Metric | Baseline | Desired direction |
|---|---:|---|
| Acceptance Criteria Coverage | 0.0% | Raise |
| Requirement Traceability Coverage | 57.1% | Raise |
| Unsupported Decision Rate | 81.4% | Lower |
| High-Impact Assumption Labeling Rate | 0.0% | Raise |
| Evidence Linkage Rate | 72.7% | Raise |
| Broken Internal Links | 0 | Preserve |

Required Artifact Completion, Unresolved High-Severity Findings, User Answer Adoption Rate, duplicate/reuse/retrieval metrics, execution time, interaction time, question/model-call counts, token usage, and cost were unavailable in the docs-only report. Do not infer or manufacture values for unavailable metrics.

## Failure Ownership

| Evaluator signal | Owning behavior |
|---|---|
| Partial explicit-requirement preservation | Initial analysis and documentation prompts preserve atomic literal brief clauses. |
| Assumption promotion and unsupported decisions | Research and documentation prompts distinguish evidence, inference, recommendations, and authority. |
| Traceability theater | Documentation and repair prompts require atomic decisions and semantically relevant exact-ID edges. |
| Missing acceptance criteria | Documentation and repair prompts require one explicit objective criterion per applicable requirement. |
| Decorative research | Documentation and repair prompts retain canonical decision-linked evidence without claim duplication. |
| Malformed VR question for a browser project | Domain-neutral deterministic clarification copy. |

## Borrowed Orbit Acceptance Thresholds

All thresholds apply to the same candidate report; a high aggregate score does not waive a failed deterministic threshold.

| Measure | Required candidate result |
|---|---:|
| DRPFS | At least 80 |
| Brief Fidelity | At least 18/20 |
| Assumption Discipline | At least 12/15 |
| Cross-Document Consistency | 15/15 |
| Evidence Quality | At least 13/15 |
| Requirements to Decision Traceability | At least 12/15 |
| Actionability | At least 13/15 |
| Artifact Completeness and Navigation | 5/5 |
| Acceptance Criteria Coverage | At least 90% |
| Requirement Traceability Coverage | At least 90% |
| Unsupported Decision Rate | At most 25% |
| High-Impact Assumption Labeling Rate | At least 90% |
| Evidence Linkage Rate | At least 90% |
| Broken Internal Links | 0 |

## Five-Brief Regression Gate

Run Last Light Courier, Pigeon Payroll, Borrowed Orbit, Dead Air Dispatch, and Half-Mech Heroes once with equivalent model, settings, approval policy, and resource boundaries. Repeat only a threshold-edge result before accepting or rejecting it.

- At least four of five cases improve.
- No case loses more than 3 DRPFS points.
- Median DRPFS improves by at least 10 points.
- Record resource differences; retrieval diagnostics do not substitute for project-quality scores.

## Measured Results

All live runs used the same local debug build workflow, Codex-backed auto-answer mode, autonomous approval policy, five source briefs, and independent evaluator revision. Raw candidates, logs, and reports are preserved under `evaluation/prompt-quality-candidate`, `evaluation/prompt-quality-logs`, and `evaluation/prompt-quality-reports` respectively.

### Controlled five-case prompt comparison

Iteration 1 exposed that the evaluator could not recognize legacy generated identifiers or table-based relationships. Iteration 2 applied the evaluator-legible record grammar while keeping the run policy and briefs equivalent.

| Case | Iteration 1 | Iteration 2 | Change |
|---|---:|---:|---:|
| Last Light Courier | 53.2 | 79.6 | +26.4 |
| Pigeon Payroll | 58.9 | 74.5 | +15.6 |
| Borrowed Orbit | 65.0 | 77.6 | +12.6 |
| Dead Air Dispatch | 59.3 | 83.0 | +23.7 |
| Half-Mech Heroes | 60.0 | 76.9 | +16.9 |
| **Median** | **59.3** | **77.6** | **+18.3** |

Result: all five cases improved, no case regressed, and the median improved by 18.3 points. This satisfies the five-case regression thresholds. Following the threshold-edge repeat policy, the later global ID-uniqueness, evidence-decision, and architecture-concreteness refinements were rerun only on Borrowed Orbit rather than repeating four non-edge cases.

### Borrowed Orbit progression

| Package | DRPFS | Purpose |
|---|---:|---|
| Immutable v0.3.0 baseline | 59.5 | Original docs-only baseline |
| Iteration 1 | 65.0 | Initial semantic prompt contract |
| Iteration 2 | 77.6 | Evaluator-legible canonical record grammar |
| Iteration 3 | 91.5 | Unique definition-leading IDs and no wildcard IDs |
| Iteration 4 | 98.0 | Evidence-dependent decision and concrete implementation structure |

The formal evaluator comparison at `evaluation/prompt-quality-comparison/borrowed-orbit-iteration-4` reports baseline 59.5 and candidate 98.0, an improvement of 38.5 points.

### Final Borrowed Orbit threshold matrix

| Measure | Required | Iteration 4 | Result |
|---|---:|---:|---|
| DRPFS | At least 80 | 98.0 | Pass |
| Brief Fidelity | At least 18/20 | 20.0 | Pass |
| Assumption Discipline | At least 12/15 | 15.0 | Pass |
| Cross-Document Consistency | 15/15 | 15.0 | Pass |
| Evidence Quality | At least 13/15 | 15.0 | Pass |
| Requirements to Decision Traceability | At least 12/15 | 13.0 | Pass |
| Actionability | At least 13/15 | 15.0 | Pass |
| Artifact Completeness and Navigation | 5/5 | 5.0 | Pass |
| Acceptance Criteria Coverage | At least 90% | 100.0% | Pass |
| Requirement Traceability Coverage | At least 90% | 100.0% | Pass |
| Unsupported Decision Rate | At most 25% | 0.0% | Pass |
| High-Impact Assumption Labeling Rate | At least 90% when available | N/A | No high-impact inferred-choice denominator |
| Evidence Linkage Rate | At least 90% | 100.0% | Pass |
| Broken Internal Links | 0 | 0 | Pass |

The final report contains no critical failures. The evaluator records one remaining traceability-theater finding, while the resulting traceability dimension remains above the acceptance threshold. Required Artifact Completion and operational efficiency/resource metrics remain unavailable and are not inferred.

## Commands

Run the pipeline suite from the pipeline repository:

```powershell
.\evaluation\run-five-codex.ps1 -DataRoot .\evaluation\prompt-quality-candidate -LogRoot .\evaluation\prompt-quality-logs
```

Re-evaluate the immutable Borrowed Orbit baseline from the evaluator repository:

```powershell
cargo run -- evaluate `
  --brief "C:\Users\Micro1\Documents\Rust Projects\project-initiation-pipeline-2\evaluation\cases\borrowed-orbit.txt" `
  --generated "C:\Users\Micro1\Documents\Rust Projects\project-initiation-pipeline-2\builds\project-init-v0.3.0-x86_64-pc-windows-msvc\.project-init\projects\1e651ba8-fe0d-479f-8f71-e1dc49741145\Docs" `
  --output ".\reports\pipeline-2-v0.3.0-1e651ba8-docs-only-rerun"
```

Compare the baseline and a candidate from the evaluator repository after resolving the candidate `Docs` directory:

```powershell
cargo run -- compare `
  --brief "C:\Users\Micro1\Documents\Rust Projects\project-initiation-pipeline-2\evaluation\cases\borrowed-orbit.txt" `
  --baseline "C:\Users\Micro1\Documents\Rust Projects\project-initiation-pipeline-2\builds\project-init-v0.3.0-x86_64-pc-windows-msvc\.project-init\projects\1e651ba8-fe0d-479f-8f71-e1dc49741145\Docs" `
  --candidate "C:\Users\Micro1\Documents\Rust Projects\project-initiation-pipeline-2\evaluation\prompt-quality-candidate\borrowed-orbit\projects\PROJECT_ID\Docs" `
  --output ".\reports\pipeline-2-prompt-quality-comparison"
```

Replace `PROJECT_ID` only with the project directory created by the controlled candidate run. Preserve the raw evaluator reports for inspection and record measured results in this file before completion.
