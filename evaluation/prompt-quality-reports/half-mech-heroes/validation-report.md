# Validation Report

## Overall Score

DRPFS: 60.0 / 100

## Scorecard

| Dimension | Score |
|---|---:|
| Brief Fidelity | 20.0/20 |
| Assumption Discipline | 12.0/15 |
| Cross-Document Consistency | 15.0/15 |
| Evidence Quality | 0.0/15 |
| Requirements → Decision Traceability | 0.0/15 |
| Actionability | 8.0/15 |
| Artifact Completeness | 5.0/5 |
| TOTAL | 60.0/100 |

## Deterministic Metrics

- Required Artifact Completion: Unavailable
- Acceptance Criteria Coverage: 0.0%
- Requirement Traceability Coverage: 48.0%
- Unsupported Decision Rate: 16.3%
- High-Impact Assumption Labeling Rate: 66.7%
- Evidence Linkage Rate: 93.2%
- User Answer Adoption Rate: Unavailable
- Duplicate Question Rate: Unavailable
- Answer Reuse Rate: Unavailable
- Retrieval Utilization Rate: Unavailable
- Repeated Research Rate: Unavailable
- Stale Retrieval Rate: Unavailable
- Broken Internal Links: 0
- Unresolved High-Severity Findings: Unavailable
- Cross-Project Leakage: Unavailable
- Context Supplied: Unavailable
- Retrieval Calls: Unavailable
- Context Compression Ratio: Unavailable
- Execution Time (seconds): Unavailable
- Human Interaction Time (seconds): Unavailable
- User Questions: Unavailable
- Model Calls: Unavailable
- Token Usage: Unavailable
- Cost: Unavailable
- Conflicting Project Names: None discovered

## Critical Findings

### [HIGH] A consequential inferred choice is presented as a settled decision without labeled provenance.

- Criterion: Assumption Discipline.A
- Artifact: SWOT.md
- Related ID: DEC-041
- Failure mode: AssumptionPromotion
- Evidence: Abstract input ports in DEC-041 allow early mechanic tests while platform and engine decisions remain open.
- Recommended correction: Label the choice as an assumption or link it to user input, a constraint, or evidence.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: EvidenceMisuse
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: EvidenceMisuse
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: EvidenceMisuse
- Evidence: | EVD-DEL-001 | DEC-038 | Supports explicit use context without supplying a comedy threshold. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: EvidenceMisuse
- Evidence: | EVD-DEL-001 | DEC-039 | Supports context-specific evaluation without supplying a mastery threshold. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-EXP-003
- Failure mode: EvidenceMisuse
- Evidence: | EVD-EXP-003 | DEC-039 | Supports cohort-sensitive interpretation of skill barriers. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-EXP-004
- Failure mode: EvidenceMisuse
- Evidence: | EVD-EXP-004 | DEC-037 | Supports distinguishability as part of silhouette evaluation. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-TEC-001
- Failure mode: EvidenceMisuse
- Evidence: | EVD-TEC-001 | DEC-028 | Shows one candidate technology supports relevant input. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-TEC-002
- Failure mode: EvidenceMisuse
- Evidence: | EVD-TEC-002 | DEC-028 | Shows another candidate technology supports relevant input. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Requirements.md
- Related ID: DEC-035
- Failure mode: ResearchAfterDecision
- Evidence: Consequential decision: DEC-035. Research constraint: EVD-EXP-001.
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Failure mode: ResearchAfterDecision
- Evidence: This file is the canonical consequential-decision register and the explicit semantic trace graph. Each edge names one source and one target; comma-separated ID lists are not used as substitutes for trace edges. Evidence constrains a decisio
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Failure mode: ResearchAfterDecision
- Evidence: | Decision ID | One subject | Status | Provenance type | Exact source IDs | Relevant evidence IDs | Rationale |
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Related ID: DEC-035
- Failure mode: ResearchAfterDecision
- Evidence: | DEC-035 | Television-readability test gate | Provisional | Research-constrained inference | SRC-015; ASM-001; ASM-006 | EVD-EXP-001; EVD-DEL-001 | A reproducible context is needed; display, distance, cohort, and pass rate remain unapprove
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Failure mode: ResearchAfterDecision
- Evidence: | Evidence | Decision | Relationship and semantic relevance |
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: TraceabilityTheater
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: TraceabilityTheater
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: TraceabilityTheater
- Evidence: | EVD-DEL-001 | DEC-038 | Supports explicit use context without supplying a comedy threshold. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: TraceabilityTheater
- Evidence: | EVD-DEL-001 | DEC-039 | Supports context-specific evaluation without supplying a mastery threshold. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-EXP-003
- Failure mode: TraceabilityTheater
- Evidence: | EVD-EXP-003 | DEC-039 | Supports cohort-sensitive interpretation of skill barriers. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-EXP-004
- Failure mode: TraceabilityTheater
- Evidence: | EVD-EXP-004 | DEC-037 | Supports distinguishability as part of silhouette evaluation. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-TEC-001
- Failure mode: TraceabilityTheater
- Evidence: | EVD-TEC-001 | DEC-028 | Shows one candidate technology supports relevant input. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-TEC-002
- Failure mode: TraceabilityTheater
- Evidence: | EVD-TEC-002 | DEC-028 | Shows another candidate technology supports relevant input. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: OpenQuestions.md
- Related ID: OQ-005
- Failure mode: TraceabilityTheater
- Evidence: | OQ-005 | Open | Stakeholder | What is the primary target platform? | Determines input APIs, performance budgets, packaging, certification, and hardware tests. | ANS-002 cannot decide. Platform prerequisites are evidence, not authority; se
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: OpenQuestions.md
- Related ID: OQ-005
- Failure mode: TraceabilityTheater
- Evidence: | OQ-005 | Open | Stakeholder | What is the primary target platform? | Determines input APIs, performance budgets, packaging, certification, and hardware tests. | ANS-002 cannot decide. Platform prerequisites are evidence, not authority; se
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: TechnicalArchitecture.md
- Related ID: OQ-006
- Failure mode: TraceabilityTheater
- Evidence: A stakeholder decision on OQ-006 should evaluate at least: compatibility with the selected platform from OQ-005, two-controller and shared-keyboard input, deterministic physics/test seams, television output, team capability, licensing, cons
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] Not all identified requirements have explicit acceptance criteria.

- Criterion: Actionability.A
- Artifact: generated package
- Failure mode: UnderSpecification
- Evidence: Acceptance criteria coverage: 0.0%.
- Recommended correction: Add testable acceptance criteria for every implementable important requirement.


## Detailed Findings

### [HIGH] A consequential inferred choice is presented as a settled decision without labeled provenance.

- Criterion: Assumption Discipline.A
- Artifact: SWOT.md
- Related ID: DEC-041
- Failure mode: AssumptionPromotion
- Evidence: Abstract input ports in DEC-041 allow early mechanic tests while platform and engine decisions remain open.
- Recommended correction: Label the choice as an assumption or link it to user input, a constraint, or evidence.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: EvidenceMisuse
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: EvidenceMisuse
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: EvidenceMisuse
- Evidence: | EVD-DEL-001 | DEC-038 | Supports explicit use context without supplying a comedy threshold. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: EvidenceMisuse
- Evidence: | EVD-DEL-001 | DEC-039 | Supports context-specific evaluation without supplying a mastery threshold. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-EXP-003
- Failure mode: EvidenceMisuse
- Evidence: | EVD-EXP-003 | DEC-039 | Supports cohort-sensitive interpretation of skill barriers. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-EXP-004
- Failure mode: EvidenceMisuse
- Evidence: | EVD-EXP-004 | DEC-037 | Supports distinguishability as part of silhouette evaluation. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-TEC-001
- Failure mode: EvidenceMisuse
- Evidence: | EVD-TEC-001 | DEC-028 | Shows one candidate technology supports relevant input. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] Cited evidence has no meaningful subject overlap with the decision it claims to support.

- Criterion: Evidence Quality.B
- Artifact: Traceability.md
- Related ID: EVD-TEC-002
- Failure mode: EvidenceMisuse
- Evidence: | EVD-TEC-002 | DEC-028 | Shows another candidate technology supports relevant input. |
- Recommended correction: Replace the evidence relationship with a source that actually supports the decision, or remove the claim.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Requirements.md
- Related ID: DEC-035
- Failure mode: ResearchAfterDecision
- Evidence: Consequential decision: DEC-035. Research constraint: EVD-EXP-001.
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Failure mode: ResearchAfterDecision
- Evidence: This file is the canonical consequential-decision register and the explicit semantic trace graph. Each edge names one source and one target; comma-separated ID lists are not used as substitutes for trace edges. Evidence constrains a decisio
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Failure mode: ResearchAfterDecision
- Evidence: | Decision ID | One subject | Status | Provenance type | Exact source IDs | Relevant evidence IDs | Rationale |
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Related ID: DEC-035
- Failure mode: ResearchAfterDecision
- Evidence: | DEC-035 | Television-readability test gate | Provisional | Research-constrained inference | SRC-015; ASM-001; ASM-006 | EVD-EXP-001; EVD-DEL-001 | A reproducible context is needed; display, distance, cohort, and pass rate remain unapprove
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [HIGH] A research-dependent decision has no explicit evidence relationship.

- Criterion: Evidence Quality.C
- Artifact: Traceability.md
- Failure mode: ResearchAfterDecision
- Evidence: | Evidence | Decision | Relationship and semantic relevance |
- Recommended correction: Link the decision to identifiable evidence that actually informed it.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: MissionVision.md
- Related ID: SRC-002
- Failure mode: DecorativeResearch
- Evidence: This mission is a normalized summary of SRC-002, SRC-003, SRC-006, SRC-008, SRC-011, and SRC-020. The exact stakeholder wording remains canonical in [Requirements.md](Requirements.md).
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: README.md
- Related ID: EVD-
- Failure mode: DecorativeResearch
- Evidence: Each retained research claim is recorded once in the relevant file below. Other documents cite only its `EVD-*` ID.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: README.md
- Failure mode: DecorativeResearch
- Evidence: [Research-01-Audience.md](Research-01-Audience.md)
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: README.md
- Failure mode: DecorativeResearch
- Evidence: [Research-02-Experience.md](Research-02-Experience.md)
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: README.md
- Failure mode: DecorativeResearch
- Evidence: [Research-03-Market.md](Research-03-Market.md)
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: README.md
- Failure mode: DecorativeResearch
- Evidence: [Research-04-Technology.md](Research-04-Technology.md)
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: README.md
- Failure mode: DecorativeResearch
- Evidence: [Research-05-Delivery.md](Research-05-Delivery.md)
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: README.md
- Related ID: SRC-TO-REQ
- Failure mode: DecorativeResearch
- Evidence: Accepted stakeholder scope is represented by SRC-to-REQ traces and accepted `DEC-*` records.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Failure mode: DecorativeResearch
- Evidence: Each quote below is exact source wording. Each row maps to at least one generated requirement; the detailed semantic edge appears separately in [Traceability.md](Traceability.md).
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Failure mode: DecorativeResearch
- Evidence: | Source ID | Exact source wording | Generated requirement |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Failure mode: DecorativeResearch
- Evidence: |---|---|---|
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-001
- Failure mode: DecorativeResearch
- Evidence: | SRC-001 | “Half-Mech Heroes” | REQ-001 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-002
- Failure mode: DecorativeResearch
- Evidence: | SRC-002 | “I want a local two-player couch arcade game” | REQ-002 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-003
- Failure mode: DecorativeResearch
- Evidence: | SRC-003 | “where both players operate the same malfunctioning rescue mech.” | REQ-003 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-004
- Failure mode: DecorativeResearch
- Evidence: | SRC-004 | “One player controls movement and jumping with the left side of a gamepad” | REQ-004 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-005
- Failure mode: DecorativeResearch
- Evidence: | SRC-005 | “while the other aims a magnetic arm and shield with the right side” | REQ-005 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-006
- Failure mode: DecorativeResearch
- Evidence: | SRC-006 | “the roles automatically swap whenever the mech takes damage.” | REQ-006 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-007
- Failure mode: DecorativeResearch
- Evidence: | SRC-007 | “Together they cross a single scrolling disaster zone” | REQ-007 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-008
- Failure mode: DecorativeResearch
- Evidence: | SRC-008 | “catch falling civilians” | REQ-008 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-009
- Failure mode: DecorativeResearch
- Evidence: | SRC-009 | “throw wreckage out of the way” | REQ-009 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-010
- Failure mode: DecorativeResearch
- Evidence: | SRC-010 | “build a teamwork multiplier before time runs out.” | REQ-010 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-011
- Failure mode: DecorativeResearch
- Evidence: | SRC-011 | “Sessions should consist of short three-minute rescue attempts” | REQ-011 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-012
- Failure mode: DecorativeResearch
- Evidence: | SRC-012 | “with quick restarts” | REQ-012 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-013
- Failure mode: DecorativeResearch
- Evidence: | SRC-013 | “exaggerated physics” | REQ-013 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-014
- Failure mode: DecorativeResearch
- Evidence: | SRC-014 | “chunky silhouettes” | REQ-014 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-015
- Failure mode: DecorativeResearch
- Evidence: | SRC-015 | “funny mechanical failures that remain readable on a television.” | REQ-015 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-015
- Failure mode: DecorativeResearch
- Evidence: | SRC-015 | “funny mechanical failures that remain readable on a television.” | REQ-016 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-016
- Failure mode: DecorativeResearch
- Evidence: | SRC-016 | “The first prototype should support two controllers or one shared keyboard” | REQ-017 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-017
- Failure mode: DecorativeResearch
- Evidence: | SRC-017 | “include one city block” | REQ-018 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-018
- Failure mode: DecorativeResearch
- Evidence: | SRC-018 | “one civilian type” | REQ-019 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-019
- Failure mode: DecorativeResearch
- Evidence: | SRC-019 | “three hazards.” | REQ-020 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: | SRC-020 | “I want coordination to create both mastery and comedy” | REQ-021 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: | SRC-020 | “I want coordination to create both mastery and comedy” | REQ-022 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: | SRC-020 | “I want coordination to create both mastery and comedy” | REQ-023 |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-001
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-001
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-002
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-002
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-003
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-003
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-004
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-004
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-005
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-005
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-006
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-006
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-007
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-007
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-008
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-008
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-009
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-009
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-010
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-010
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-011
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-011
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-012
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-012
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-013
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-013
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-014
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-014
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-015
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-015
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-015
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-015
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-016
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-016
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-017
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-017
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-018
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-018
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-019
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-019
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-020
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-020
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Requirements.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: Authority and source: stakeholder brief, SRC-020
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-01-Audience.md
- Failure mode: DecorativeResearch
- Evidence: Research date: 2026-08-30
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-01-Audience.md
- Failure mode: DecorativeResearch
- Evidence: Scope: primary-source checks that constrain audience discussion. No audience interviews, demographic data, or stakeholder audience decision were available.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-01-Audience.md
- Failure mode: DecorativeResearch
- Evidence: No stakeholder audience choice.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-01-Audience.md
- Failure mode: DecorativeResearch
- Evidence: No interviews or observational research with children, families, adult couch-co-op players, disabled players, or mixed-skill pairs.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-01-Audience.md
- Failure mode: DecorativeResearch
- Evidence: No jurisdiction or ratings-submission plan.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-01-Audience.md
- Failure mode: DecorativeResearch
- Evidence: No approved language, content-sensitivity, motor-skill, reading-level, or cognitive-load criteria.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-01-Audience.md
- Failure mode: DecorativeResearch
- Evidence: Any audience persona created before those inputs would be invented and is therefore omitted.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-02-Experience.md
- Failure mode: DecorativeResearch
- Evidence: Research date: 2026-08-30
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-02-Experience.md
- Failure mode: DecorativeResearch
- Evidence: Scope: primary Microsoft guidance used as nonbinding design and test constraints. These recommendations do not provide stakeholder authority or prove compliance.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-02-Experience.md
- Failure mode: DecorativeResearch
- Evidence: The experience evidence constrains presentation and suggests accessibility options. It does not settle scope, content identities, target cohort, difficulty design, or project-specific pass thresholds. Those choices remain in OpenQuestions.m
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-02-Experience.md
- Failure mode: DecorativeResearch
- Evidence: No paired-player sessions were observed.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-02-Experience.md
- Failure mode: DecorativeResearch
- Evidence: No players with disabilities participated in the supplied work.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-02-Experience.md
- Failure mode: DecorativeResearch
- Evidence: No comedy, coordination, mastery, readability, or restart data exists.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-02-Experience.md
- Failure mode: DecorativeResearch
- Evidence: No final display, room, controller models, keyboard layout, or target-hardware performance is available.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Research date: 2026-08-30
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Scope: official platform documentation only. No sales estimates, market sizing, competitor ranking, price research, store review mining, or commercial forecast was performed.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Claim type: sourced fact
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Claim: Nintendo describes developer registration and a subsequent request for Nintendo Switch information access.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Primary source: [Nintendo Developer Portal registration](https://developer.nintendo.com/register)
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Reliability: high; official platform documentation.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Checked: available 2026-08-30.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Official platform records establish differing capability labels and onboarding constraints. They do not identify a primary platform, target market, commercial opportunity, price, audience, or distribution plan for this project.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Comparable-game list and verified feature comparison.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Current unit sales, revenue, wishlists, concurrent players, or conversion data.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Regional platform share for the intended audience.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Pricing, launch-window, publisher, funding, or marketing constraints.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Stakeholder commercial goals.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-03-Market.md
- Failure mode: DecorativeResearch
- Evidence: Market positioning is therefore unresolved and no competitor claim is asserted.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: Research date: 2026-08-30
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: Scope: official engine capability documentation used to show that the input brief does not uniquely determine a technology.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: At least two candidate engines document relevant mapped keyboard/gamepad input, so the control brief does not uniquely select a stack. Console support models may materially affect feasibility. The architecture must remain engine-agnostic un
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: Team size, skills, existing licenses, budget, source-control/build infrastructure, and schedule.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: Physics determinism spike or controller coexistence spike in any engine.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: Target-hardware profiling.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: License/legal review.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: Console NDA material or approved SDK access.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-04-Technology.md
- Failure mode: DecorativeResearch
- Evidence: No engine ranking or recommendation is supportable from the available evidence.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: Research date: 2026-08-30
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: Scope: primary institutional sources that constrain evaluation and prototype-scope reasoning.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: Evaluation must be bound to approved users, goals, contexts, methods, and thresholds. Prototype discipline should focus on risky assumptions, but neither source supplies game-specific acceptance gates or stakeholder authority over exclusion
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: Budget, team, milestone date, release objective, delivery methodology, and stakeholder review cadence.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: Reference devices and performance budgets.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: Definition of done beyond the brief and this initiation package.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: User-research recruitment, consent, safeguarding, compensation, and data-handling plan.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Research-05-Delivery.md
- Failure mode: DecorativeResearch
- Evidence: Production, certification, localisation, ratings, support, and operations plans.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: SWOT.md
- Related ID: EVD-EXP-001
- Failure mode: DecorativeResearch
- Evidence: Readability constraints informed by EVD-EXP-001, EVD-EXP-004, and EVD-EXP-005 can improve couch play beyond minimum legibility.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-001
- Failure mode: DecorativeResearch
- Evidence: | SRC-001 | REQ-001 | Declares the exact project name. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-002
- Failure mode: DecorativeResearch
- Evidence: | SRC-002 | REQ-002 | Declares local two-player couch arcade play. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-003
- Failure mode: DecorativeResearch
- Evidence: | SRC-003 | REQ-003 | Declares one shared malfunctioning mech. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-004
- Failure mode: DecorativeResearch
- Evidence: | SRC-004 | REQ-004 | Assigns movement and jump to the left-side role. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-005
- Failure mode: DecorativeResearch
- Evidence: | SRC-005 | REQ-005 | Assigns arm and shield aiming to the right-side role. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-006
- Failure mode: DecorativeResearch
- Evidence: | SRC-006 | REQ-006 | Defines damage as automatic role-swap trigger. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-007
- Failure mode: DecorativeResearch
- Evidence: | SRC-007 | REQ-007 | Defines the single scrolling zone. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-008
- Failure mode: DecorativeResearch
- Evidence: | SRC-008 | REQ-008 | Defines falling-civilian catching. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-009
- Failure mode: DecorativeResearch
- Evidence: | SRC-009 | REQ-009 | Defines wreckage throwing. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-010
- Failure mode: DecorativeResearch
- Evidence: | SRC-010 | REQ-010 | Defines teamwork-multiplier growth before timeout. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-011
- Failure mode: DecorativeResearch
- Evidence: | SRC-011 | REQ-011 | Defines three-minute attempts. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-012
- Failure mode: DecorativeResearch
- Evidence: | SRC-012 | REQ-012 | Defines quick-restart intent. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-013
- Failure mode: DecorativeResearch
- Evidence: | SRC-013 | REQ-013 | Defines exaggerated-physics intent. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-014
- Failure mode: DecorativeResearch
- Evidence: | SRC-014 | REQ-014 | Defines chunky-silhouette intent. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-015
- Failure mode: DecorativeResearch
- Evidence: | SRC-015 | REQ-015 | Defines funny mechanical failures. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-015
- Failure mode: DecorativeResearch
- Evidence: | SRC-015 | REQ-016 | Defines the television-readability condition. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-016
- Failure mode: DecorativeResearch
- Evidence: | SRC-016 | REQ-017 | Defines both mandatory input configurations. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-017
- Failure mode: DecorativeResearch
- Evidence: | SRC-017 | REQ-018 | Sets city-block content count. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-018
- Failure mode: DecorativeResearch
- Evidence: | SRC-018 | REQ-019 | Sets civilian-type content count. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-019
- Failure mode: DecorativeResearch
- Evidence: | SRC-019 | REQ-020 | Sets hazard-type content count. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: | SRC-020 | REQ-021 | Declares coordination as an outcome. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: | SRC-020 | REQ-022 | Declares mastery as an outcome. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: SRC-020
- Failure mode: DecorativeResearch
- Evidence: | SRC-020 | REQ-023 | Declares comedy as an outcome. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-MKT-001
- Failure mode: DecorativeResearch
- Evidence: | EVD-MKT-001 | DEC-002 | Corroborates local multiplayer as a distinct platform capability. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-EXP-001
- Failure mode: DecorativeResearch
- Evidence: | EVD-EXP-001 | DEC-015 | Constrains television-readable text design. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-EXP-005
- Failure mode: DecorativeResearch
- Evidence: | EVD-EXP-005 | DEC-015 | Constrains critical state to redundant cue channels. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-DEL-002
- Failure mode: DecorativeResearch
- Evidence: | EVD-DEL-002 | DEC-024 | Shows geographic “block” terminology cannot supply a fixed extent. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-EXP-005
- Failure mode: DecorativeResearch
- Evidence: | EVD-EXP-005 | DEC-025 | Constrains eventual civilian cues without choosing identity. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-EXP-005
- Failure mode: DecorativeResearch
- Evidence: | EVD-EXP-005 | DEC-026 | Constrains eventual hazard cues without choosing identities. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-EXP-002
- Failure mode: DecorativeResearch
- Evidence: | EVD-EXP-002 | DEC-029 | Supplies nonbinding remapping benefit and breadth guidance. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-MKT-001
- Failure mode: DecorativeResearch
- Evidence: | EVD-MKT-001 | DEC-030 | Establishes local capability without implying solo capability. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-MKT-004
- Failure mode: DecorativeResearch
- Evidence: | EVD-MKT-004 | DEC-030 | Establishes single-player as a distinct capability label. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: Traceability.md
- Related ID: EVD-EXP-001
- Failure mode: DecorativeResearch
- Evidence: | EVD-EXP-001 | DEC-035 | Supplies a provisional 1080p text floor and context reminder. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: ValidationReport.md
- Related ID: SRC-
- Failure mode: DecorativeResearch
- Evidence: | VAL-003 | Parsed the material source ledger and source-to-requirement trace table. | Pass | 21 unique `SRC-*` clauses found; every clause has at least one generated-requirement mapping and at least one explicit semantic edge. |
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: log-2026-08-30-02-17-08.md
- Related ID: SRC-
- Failure mode: DecorativeResearch
- Evidence: Preserved the full brief verbatim and split it into exact material `SRC-*` clauses in Requirements.md.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: log-2026-08-30-02-17-08.md
- Failure mode: DecorativeResearch
- Evidence: Accessed only public, primary-source web pages; no remote record was created or changed.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: log-2026-08-30-02-17-08.md
- Related ID: EVD-
- Failure mode: DecorativeResearch
- Evidence: Checked the retained sources on 2026-08-30 and consolidated the useful material into the canonical `EVD-*` records in the five research files.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: log-2026-08-30-02-17-08.md
- Failure mode: DecorativeResearch
- Evidence: Removed duplicate claims from the supplied evidence set and omitted claims that did not constrain a requirement or consequential decision.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [MEDIUM] An evidence claim is not connected to any consequential decision.

- Criterion: Evidence Quality.A
- Artifact: log-2026-08-30-02-17-08.md
- Failure mode: DecorativeResearch
- Evidence: Recorded unavailable audience, market, technical, delivery, and playtest research instead of inventing results.
- Recommended correction: Connect useful evidence to a decision or remove decorative research.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: TraceabilityTheater
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: DEC-032
- Failure mode: TraceabilityTheater
- Evidence: | DEC-032 | Exact target age | Deferred | Stakeholder-declared open choice | SRC-021; ANS-008 | EVD-AUD-001; EVD-AUD-002 | Ratings evidence cannot select intended audience capability. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: TraceabilityTheater
- Evidence: | EVD-DEL-001 | DEC-038 | Supports explicit use context without supplying a comedy threshold. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-DEL-001
- Failure mode: TraceabilityTheater
- Evidence: | EVD-DEL-001 | DEC-039 | Supports context-specific evaluation without supplying a mastery threshold. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-EXP-003
- Failure mode: TraceabilityTheater
- Evidence: | EVD-EXP-003 | DEC-039 | Supports cohort-sensitive interpretation of skill barriers. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-EXP-004
- Failure mode: TraceabilityTheater
- Evidence: | EVD-EXP-004 | DEC-037 | Supports distinguishability as part of silhouette evaluation. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-TEC-001
- Failure mode: TraceabilityTheater
- Evidence: | EVD-TEC-001 | DEC-028 | Shows one candidate technology supports relevant input. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: EVD-TEC-002
- Failure mode: TraceabilityTheater
- Evidence: | EVD-TEC-002 | DEC-028 | Shows another candidate technology supports relevant input. |
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: OpenQuestions.md
- Related ID: OQ-005
- Failure mode: TraceabilityTheater
- Evidence: | OQ-005 | Open | Stakeholder | What is the primary target platform? | Determines input APIs, performance budgets, packaging, certification, and hardware tests. | ANS-002 cannot decide. Platform prerequisites are evidence, not authority; se
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: OpenQuestions.md
- Related ID: OQ-005
- Failure mode: TraceabilityTheater
- Evidence: | OQ-005 | Open | Stakeholder | What is the primary target platform? | Determines input APIs, performance budgets, packaging, certification, and hardware tests. | ANS-002 cannot decide. Platform prerequisites are evidence, not authority; se
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: TechnicalArchitecture.md
- Related ID: OQ-006
- Failure mode: TraceabilityTheater
- Evidence: A stakeholder decision on OQ-006 should evaluate at least: compatibility with the selected platform from OQ-005, two-controller and shared-keyboard input, deterministic physics/test seams, television output, team capability, licensing, cons
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [HIGH] An explicit trace links statements with no meaningful subject overlap.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-001
- Failure mode: TraceabilityTheater
- Evidence: The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were
- Recommended correction: Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: MissionVision.md
- Related ID: REQ-013
- Evidence: | Readable chaos | REQ-013; REQ-014; REQ-016 | Physics and failures can be exaggerated, but state and hazards must remain interpretable. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: MissionVision.md
- Related ID: REQ-012
- Evidence: | Immediate replay | REQ-012 | Restart stays in the attempt flow and resets state cleanly. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Requirements.md
- Related ID: REQ-
- Evidence: Rule: this file is the only canonical register of requirements. `REQ-*` records are generated project requirements; `SRC-*` records preserve exact stakeholder wording.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Research-02-Experience.md
- Related ID: REQ-016
- Evidence: Informs: REQ-016, DEC-035, ASM-001, and OQ-010D. The numeric guidance informs the provisional test but does not approve the project’s display, distance, cohort, or threshold.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Research-02-Experience.md
- Related ID: REQ-014
- Evidence: Informs: REQ-014, DEC-037, DEC-025, and DEC-026 as a presentation constraint, not an identity selection.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Research-02-Experience.md
- Related ID: REQ-016
- Evidence: Informs: REQ-016, DEC-025, and DEC-026 as a cue-design constraint, not a creative content choice.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: SWOT.md
- Related ID: REQ-018
- Evidence: Prototype content counts are bounded: REQ-018, REQ-019, and REQ-020.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: SWOT.md
- Related ID: REQ-022
- Evidence: Mastery and comedy are both explicit intended outcomes: REQ-022 and REQ-023.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: SWOT.md
- Related ID: REQ-015
- Evidence: Damage-driven swaps may turn ordinary mistakes into replayable social stories if REQ-015 and REQ-023 test positively.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: TechnicalArchitecture.md
- Related ID: REQ-013
- Evidence: | REQ-013 | Physics parameters are data-driven and observable in fixtures. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: TechnicalArchitecture.md
- Related ID: REQ-014
- Evidence: | REQ-014; REQ-015; REQ-016 | Presentation state exposes role, hazard, civilian, timer, multiplier, and failure cues through redundant channels. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: TechnicalArchitecture.md
- Related ID: REQ-018
- Evidence: | REQ-018; REQ-019; REQ-020 | Content manifests enforce one block, one civilian archetype, and three hazard archetypes. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-012
- Evidence: | REQ-012 | DEC-011 | Restart-intent decision establishes repeat-attempt behaviour. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-012
- Evidence: | REQ-012 | DEC-034 | Provisional timing decision supplies the temporary measurable threshold. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-013
- Evidence: | REQ-013 | DEC-012 | Physics-style decision establishes the qualitative behaviour. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-013
- Evidence: | REQ-013 | DEC-036 | Provisional physics decision supplies temporary observable bounds. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-014
- Evidence: | REQ-014 | DEC-013 | Silhouette-style decision establishes visual intent. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-014
- Evidence: | REQ-014 | DEC-037 | Provisional recognition decision supplies the temporary evaluation gate. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-015
- Evidence: | REQ-015 | DEC-014 | Failure-tone decision establishes humorous malfunction intent. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-015
- Evidence: | REQ-015 | DEC-038 | Provisional comedy decision supplies the temporary observation gate. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-016
- Evidence: | REQ-016 | DEC-015 | Television-presentation decision establishes readability intent. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-016
- Evidence: | REQ-016 | DEC-035 | Provisional readability decision supplies the temporary context and threshold. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-018
- Evidence: | REQ-018 | DEC-017 | City-block count decision governs manifest count. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-018
- Evidence: | REQ-018 | DEC-024 | Deferred boundary prevents conflating authored block and logical zone. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-019
- Evidence: | REQ-019 | DEC-018 | Civilian-count decision governs archetype count. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-019
- Evidence: | REQ-019 | DEC-025 | Deferred identity prevents invented civilian art or behaviour. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-020
- Evidence: | REQ-020 | DEC-019 | Hazard-count decision governs archetype count. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-020
- Evidence: | REQ-020 | DEC-026 | Deferred identities prevent invented hazard selections. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-021
- Evidence: | REQ-021 | DEC-020 | Coordination-outcome decision establishes desired effect. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-021
- Evidence: | REQ-021 | DEC-040 | Provisional coordination decision supplies the temporary comparison gate. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-022
- Evidence: | REQ-022 | DEC-021 | Mastery-outcome decision establishes desired learning effect. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-022
- Evidence: | REQ-022 | DEC-039 | Provisional mastery decision supplies the temporary progression gate. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-023
- Evidence: | REQ-023 | DEC-022 | Comedy-outcome decision establishes desired amusement effect. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-023
- Evidence: | REQ-023 | DEC-038 | Provisional comedy decision supplies the temporary observation gate. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: Traceability.md
- Related ID: REQ-024
- Evidence: | REQ-024 | DEC-023 | Authority decision prevents research from closing stakeholder-open choices. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: ValidationReport.md
- Related ID: REQ-
- Evidence: | VAL-002 | Parsed `Requirements.md` for generated requirement sections and required fields. | Pass | 24 `REQ-*` sections found; each contains an explicitly labelled objective pass/fail acceptance criterion, exact verification reference, an
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: ValidationReport.md
- Related ID: REQ-
- Evidence: | VAL-004 | Compared all `REQ-*`, `SRC-*`, `ASM-*`, `OQ-*`, `EVD-*`, and `DEC-*` references with their canonical definitions. | Pass | No undefined IDs and no single-occurrence canonical IDs remain. |
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: VisualIdentity.md
- Related ID: REQ-014
- Evidence: Use a broad, asymmetric rescue-mech silhouette assembled from large geometric masses: cab-like torso, piston legs, one oversized magnetic arm, and one clearly separate shield plane. Civilians should read as a single compact silhouette class
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [MEDIUM] An identified important requirement has no explicit decision trace.

- Criterion: Requirements → Decision Traceability
- Artifact: VisualIdentity.md
- Related ID: REQ-015
- Evidence: Keep all humorous failures playable; a failure may disrupt control allocation or motion but must not obscure critical state or prevent attempt completion under REQ-015.
- Recommended correction: Link the requirement to at least one relevant decision or documented constraint.

### [HIGH] Not all identified requirements have explicit acceptance criteria.

- Criterion: Actionability.A
- Artifact: generated package
- Failure mode: UnderSpecification
- Evidence: Acceptance criteria coverage: 0.0%.
- Recommended correction: Add testable acceptance criteria for every implementable important requirement.


## Unsupported Decisions

- The exact `TST-REQ-*` references are defined with each requirement in [Requirements.md](Requirements.md). Automated fixtures cover state, input isolation, manifests, timer, catches, throws, score, and reset. Human protocols cover silhouettes, television readability, mastery, coordination, and comedy; all current thresholds are explicitly provisional assumptions.
- | Evidence | Decision | Relationship and semantic relevance |
- |---|---|---|
- | Decision | Open question | Relationship and semantic relevance |
- |---|---|---|
- This file is the canonical consequential-decision register and the explicit semantic trace graph. Each edge names one source and one target; comma-separated ID lists are not used as substitutes for trace edges. Evidence constrains a decision but never supplies stakeholder authority.
- | Decision ID | One subject | Status | Provenance type | Exact source IDs | Relevant evidence IDs | Rationale |
- |---|---|---|---|---|---|---|
- Consequential decision: DEC-001.
- Consequential decision: DEC-002.
- Consequential decision: DEC-003.
- Consequential decision: DEC-004.
- Consequential decision: DEC-005.
- Consequential decision: DEC-006. Boundary constraint: DEC-024 and OQ-009.
- Consequential decision: DEC-007.
- Consequential decision: DEC-008.
- Consequential decision: DEC-009.
- Consequential decision: DEC-010.
- Consequential decision: DEC-016.
- Consequential decision: DEC-017. Boundary constraint: DEC-024 and OQ-009.
- Consequential decision: DEC-018. Identity constraint: DEC-025 and OQ-008.
- Consequential decision: DEC-019. Identity constraint: DEC-026 and OQ-007.
- Consequential decision: DEC-034.
- Informs: DEC-035, DEC-038, DEC-039, and DEC-040 by supporting context-specific evaluation while withholding project-specific thresholds.
- Consequential decision: DEC-036.
- Abstract input ports in DEC-041 allow early mechanic tests while platform and engine decisions remain open.

## Unresolved Assumptions and Questions

- A stakeholder decision on OQ-006 should evaluate at least: compatibility with the selected platform from OQ-005, two-controller and shared-keyboard input, deterministic physics/test seams, television output, team capability, licensing, console-support model, build automation, and profiling. EVD-TEC-001, EVD-TEC-002, and EVD-TEC-003 show that capability evidence constrains but does not decide this selection.
- Added the missing use-site citation for ASM-010.
- Checked: available 2026-08-30.
- Claim type: sourced recommendation
- Claim: The GOV.UK Service Manual advises using alpha to prototype and test risky assumptions and to build enough to support a decision about moving forward.
- Core evaluation terms lack stakeholder-approved measures: OQ-010A, OQ-010B, OQ-010C, OQ-010D, and OQ-010E.
- Evidence supports only an authority boundary: age-classification information cannot choose the exact intended audience or validate gameplay difficulty for Half-Mech Heroes. Target age remains stakeholder-owned in OQ-004.
- Hazard and civilian identities are undecided, blocking final art and behaviour specifications: OQ-007 and OQ-008.
- Hazards remain abstract because OQ-007 is open.
- Imported Q-010 was split into OQ-010A, OQ-010B, OQ-010C, OQ-010D, and OQ-010E.
- Imported Q-011 was split into OQ-011A, OQ-011B, OQ-011C, and OQ-011D.
- Keep platform adapters and engine APIs outside the gameplay domain until OQ-005 and OQ-006 close.
- No stakeholder approved the proposed `ASM-*` values or answered the open questions during this session.
- No target age is implied as an approved choice while OQ-004 remains open.
- Objective pass/fail acceptance criterion: **Provisional pass** if a restart input produces controllable gameplay in no more than 5.0 seconds on the reference test device, resets timer, score, multiplier, roles, entities, and deterministic seed, and does not reload the application; otherwise **fail**. The 5.0-second gate is non-authoritative ASM-002 and cannot close OQ-010E.
- Objective pass/fail acceptance criterion: **Provisional pass** if at least four of five evaluators correctly identify every entity class from randomized, label-free, single-colour 1080p silhouette captures at the assumed couch setup; otherwise **fail**. The setup and cohort are non-authoritative ASM-004.
- Objective pass/fail acceptance criterion: **Provisional pass** if at least four of five test pairs improve either rescued-civilian count or peak teamwork multiplier from their first valid attempt to their fifth valid attempt without a difficulty reduction; otherwise **fail**. Cohort and threshold are non-authoritative ASM-008.
- Objective pass/fail acceptance criterion: **Provisional pass** if at least four of five test pairs independently record at least one laughter or amusement event during a valid attempt and can identify the triggering gameplay event immediately afterward; otherwise **fail**. Cohort and coding rule are non-authoritative ASM-005.
- Objective pass/fail acceptance criterion: **Provisional pass** if the scripted damage fixture exposes at least three mechanically distinct failure presentations, none blocks completion, and at least four of five paired-playtest participants mark at least one observed failure as funny on the defined binary prompt; otherwise **fail**. Counts and cohort are non-authoritative ASM-005.
- Objective pass/fail acceptance criterion: **Provisional pass** if the standard throw fixture propels wreckage at least one mech-height horizontally before first ground contact and the standard damage fixture produces a visible mech recoil of at least one-quarter mech-height without losing player control for more than 1.0 second; otherwise **fail**. These bounds are non-authoritative ASM-003.
- Objective pass/fail acceptance criterion: **Provisional pass** if, across the fixed paired-playtest script, the median rescued-civilian count in coordinated trials is greater than the median in deliberately uncoordinated control trials; otherwise **fail**. Protocol and sample size are non-authoritative ASM-007.
- Objective pass/fail acceptance criterion: **Provisional pass** if, at 1920×1080 on the assumed 55-inch display at 3 metres, all critical text has at least 26-pixel body height and at least four of five evaluators correctly report timer, multiplier, current role, civilian state, and active hazard cue during the test script; otherwise **fail**. Display, distance, and cohort are non-authoritative ASM-001 and ASM-006.
- Platform and technology are undecided, blocking concrete budgets and tooling: OQ-005 and OQ-006.
- Primary source: [How the alpha phase works — GOV.UK Service Manual](https://www.gov.uk/service-manual/agile-delivery/how-the-alpha-phase-works)
- Reads immutable frame state and maps critical state to shape, placement, text/icon, motion, audio, and optional haptic channels. Presentation cannot author damage, rescue, or score events. Television test settings remain provisional under ASM-001 and ASM-006.
- Reliability: high for general government service-delivery practice; indirect for game development.
- Status: **provisional, engine-agnostic architecture** under ASM-009 and DEC-041. It describes boundaries and test seams, not a selected implementation stack. OQ-005 and OQ-006 must close before vendor APIs, build targets, performance budgets, or release packaging can be authoritative.
- Status: accepted intent; provisional evaluation pending OQ-010B
- Status: accepted intent; provisional measurement pending OQ-010A
- Status: accepted intent; provisional measurement pending OQ-010B
- Status: accepted intent; provisional measurement pending OQ-010C
- Status: accepted intent; provisional test context pending OQ-010D
- Status: accepted intent; provisional threshold pending OQ-010E
- The city-block/disaster-zone boundary is unresolved: OQ-009.
- The civilian remains a neutral placeholder because OQ-008 is open.
- The mandatory prototype is local. The architecture specifies no accounts, chat, remote telemetry, purchases, cloud saves, or deployment. This is a working omission under ASM-012, not an approved exclusion. If any remote capability enters scope, threat modelling, consent, data retention, and network-failure requirements must be added before implementation.
- The zone streamer accepts one `city_block` manifest reference. It does not assume whether that unit equals the whole logical zone under ASM-011. Under non-authoritative ASM-010, factories load one neutral civilian fixture and three neutral hazard fixtures until OQ-007 and OQ-008 close; fixture names carry no creative identity.
- This is the canonical register for inferred choices. Every assumption is non-authoritative until a stakeholder approves it. An assumption may support a provisional test but may not close an open question or silently become product scope.
- This is the canonical register for unresolved choices. Snapshot questions marked “answered” remain open here when their imported answer explicitly says the evidence cannot decide or begins with `FAIL`.
- Translate controller and keyboard events into semantic actions such as `move`, `jump`, `arm_aim`, `magnet`, and `shield_aim`. Controller brand, key layout, dead zones, and remapping UI are not selected. Remapping remains behind OQ-001.
- Use a heavy condensed display face for the title and a highly legible sans serif for all operational UI. The implementation font is not selected. At the provisional 1080p television context, critical text uses at least 26-pixel body height under ASM-001 and EVD-EXP-001; this is a non-authoritative test baseline, not a final brand size system.
- When a stakeholder answers an open question, update the matching `OQ-*`, add or revise one single-subject `DEC-*` record with provenance and rationale, update affected requirements and assumptions, add explicit trace edges, and rerun the checks described in ValidationReport.md. Research may constrain that decision but may not be cited as stakeholder authority.
- When an assumption appears outside this file, it must retain the label **non-authoritative** and cite its exact `ASM-*` ID. Acceptance of a prototype under an assumed threshold is provisional; it cannot be represented as stakeholder acceptance of the underlying qualitative brief term.
- | ASM-001 | High | Proposed | Television evaluation uses a 55-inch 1920×1080 display viewed from 3 metres. | REQ-016; DEC-035 | Stakeholder approves a reference display and distance, or OQ-010D records replacements. |
- | ASM-002 | High | Proposed | “Quick” restart is provisionally tested as controllable gameplay within 5.0 seconds after restart input. | REQ-012; DEC-034 | Measure on target hardware after OQ-005; stakeholder approves or replaces threshold through OQ-010E. |
- | ASM-003 | Medium | Proposed | Exaggeration is provisionally observable through at least one mech-height of throw travel and one-quarter mech-height of damage recoil, with no more than 1.0 second loss of control. | REQ-013; DEC-036 | Playtest capture demonstrates inadequacy, or stakeholder approves/replaces values. |
- | ASM-004 | High | Proposed | A five-person evaluator set and four-of-five recognition gate are sufficient for a prototype silhouette check. Evaluator age cannot be targeted until OQ-004 closes. | REQ-014; DEC-037 | Audience decision and evaluation plan replace cohort and gate. |
- | ASM-005 | High | Proposed | A five-pair playtest with a four-of-five-pairs amusement gate is sufficient only as a prototype comedy signal. | REQ-015; REQ-023; DEC-038 | OQ-010B approves comedy definition, cohort, observation method, and threshold. |
- | ASM-006 | High | Proposed | A five-person evaluator set and four-of-five correct-report gate are sufficient for a prototype television-readability check. | REQ-016; DEC-035 | OQ-010D approves readability context and threshold. |
- | ASM-007 | High | Proposed | A within-pair coordinated-versus-deliberately-uncoordinated comparison can indicate whether coordination affects rescue outcomes. It does not define desired effect size. | REQ-021; DEC-040 | OQ-010C approves outcome metric, control protocol, cohort, and effect threshold. |
- | ASM-008 | High | Proposed | First-to-fifth-attempt improvement for four of five pairs is a prototype mastery signal, not proof of long-term mastery. | REQ-022; DEC-039 | OQ-010A approves mastery definition, cohort, attempt count, and threshold. |
- | ASM-009 | High | Proposed | Until engine and platform are chosen, the architecture remains engine-agnostic, uses abstract ports, and specifies no vendor SDK. | TechnicalArchitecture.md; DEC-041 | OQ-005 and OQ-006 close with compatible selections. |
- | ASM-010 | Medium | Proposed | Unselected civilian and hazard identities are represented by neutral test-fixture archetypes, not creative commitments. | REQ-019; REQ-020; DEC-025; DEC-026 | OQ-007 and OQ-008 close. |
- | ASM-011 | Medium | Proposed | The one authored city-block unit may be streamed within a larger logical zone, but neither equality nor inequality between those boundaries is assumed. | REQ-007; REQ-018; DEC-024 | OQ-009 closes. |
- | ASM-012 | High | Proposed | Online services, monetisation, progression, extra content, and release packaging are not planned in the initiation slice, but are not declared exclusions. | DEC-033; Research-05-Delivery.md | Stakeholder answers OQ-011A, OQ-011B, OQ-011C, and OQ-011D. |
- | ID | Impact | Status | Non-authoritative working assumption | Used by | Falsification or approval path |
- | ID | Status | Owner | Decision needed | Why it matters | Unavailable authority / next action |
- | OQ-001 | Open | Stakeholder | Should in-game input remapping be included in the first prototype, and if so for which inputs? | Affects accessibility, UI, persistence, tests, and schedule. | ANS-006 supplies guidance but no authority. Decide scope explicitly; see EVD-EXP-002. |
- | OQ-002 | Open | Stakeholder | Should solo play be included in addition to mandatory local two-player play? | Changes control substitution, AI, onboarding, and scope. | ANS-001 cannot decide. No current project authority is available. |
- | OQ-003 | Open | Stakeholder | Should difficulty scaling be included, and what experience should it preserve? | Changes tuning, UI, content parameters, and evaluation. | ANS-007 supplies guidance but no authority; see EVD-EXP-003. |
- | OQ-004 | Open | Stakeholder | What exact target age or age range is intended? | Affects evaluation cohort, language, complexity, content, and ratings planning. | ANS-008 cannot decide; classification is not audience selection. See EVD-AUD-001 and EVD-AUD-002. |
- | OQ-005 | Open | Stakeholder | What is the primary target platform? | Determines input APIs, performance budgets, packaging, certification, and hardware tests. | ANS-002 cannot decide. Platform prerequisites are evidence, not authority; see EVD-MKT-002 and EVD-MKT-003. |
- | OQ-006 | Open | Stakeholder | Which engine, language, and framework should implement the prototype? | Determines runtime architecture, tooling, team needs, and platform feasibility. | ANS-003 cannot decide. Resolve after or together with OQ-005; see EVD-TEC-001, EVD-TEC-002, and EVD-TEC-003. |
- | OQ-007 | Open | Stakeholder / creative lead | Which three hazard identities should fill the required hazard slots? | Required for art, behaviour, telegraphing, and tuning. | ANS-009 constrains communication only. No identity authority is available. |
- | OQ-008 | Open | Stakeholder / creative lead | Which single civilian type should fill the required civilian slot? | Required for art, tone, animation, and rescue presentation. | ANS-010 cannot decide and incorrectly suggested the rescue mechanic was absent; SRC-008 already requires catching. |
- | OQ-009 | Open | Stakeholder / level-design lead | Does the one city block constitute the entire single scrolling disaster zone? | Changes level length, content boundaries, camera design, and completion definition. | ANS-004 cannot decide; see EVD-DEL-002 only for why “block” lacks a standard extent. |
- | OQ-010A | Open | Stakeholder / research lead | What observable definition, cohort, method, and pass threshold establish mastery? | Required to replace ASM-008 and approve REQ-022. | Imported Q-010 bundled this with unrelated outcomes; no threshold authority is available. |
- | OQ-010B | Open | Stakeholder / research lead | What observable definition, cohort, method, and pass threshold establish comedy? | Required to replace ASM-005 and approve REQ-015 and REQ-023. | Imported Q-010 bundled this with unrelated outcomes; no threshold authority is available. |
- | OQ-010C | Open | Stakeholder / research lead | What observable definition, cohort, method, and pass threshold establish coordination? | Required to replace ASM-007 and approve REQ-021. | Imported Q-010 bundled this with unrelated outcomes; no threshold authority is available. |
- | OQ-010D | Open | Stakeholder / UX lead | What display, viewing distance, cohort, tasks, and pass threshold establish television readability? | Required to replace ASM-001 and ASM-006 and approve REQ-016. | EVD-EXP-001 informs a test but cannot approve project context or threshold. |
- | OQ-010E | Open | Stakeholder / delivery lead | What device and elapsed-time threshold establish a quick restart? | Required to replace ASM-002 and approve the qualitative part of REQ-012. | No project-specific threshold authority is available. |
- | OQ-011A | Open | Stakeholder | Which gameplay features, if any, are explicitly excluded from the first prototype? | Prevents accidental scope expansion while preserving authority. | ANS-011 correctly identifies no current exclusion; decide individually. |
- | OQ-011B | Open | Stakeholder | Which platforms or distribution channels, if any, are explicitly excluded? | Constrains technical evaluation and packaging. | No current exclusion authority is available. |
- | OQ-011C | Open | Stakeholder | Which quality attributes, if any, may be deferred from the first prototype? | Affects definition of done and technical debt. | No current exclusion authority is available. |
- | OQ-011D | Open | Stakeholder | Which deliverables beyond a playable prototype, if any, are explicitly excluded? | Clarifies whether tooling, analytics, documentation, or release artifacts are expected. | No current exclusion authority is available. |
- |---|---|---|---|---|---|

## Strengths

- All extracted explicit brief requirements are represented without detected contradiction.
- No broken submitted internal artifact links were found.

## Recommended Improvements

1. Label the choice as an assumption or link it to user input, a constraint, or evidence.
2. Replace the evidence relationship with a source that actually supports the decision, or remove the claim.
3. Link the decision to identifiable evidence that actually informed it.
4. Remove the unrelated link or replace it with a relationship whose rationale is substantively meaningful.
5. Add testable acceptance criteria for every implementable important requirement.
6. Connect useful evidence to a decision or remove decorative research.
7. Link the requirement to at least one relevant decision or documented constraint.

## External Verification Scope

External source correctness was not independently verified; citation identity and submitted decision linkage were evaluated locally.
