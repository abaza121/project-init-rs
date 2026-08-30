# Assumptions Register

Canonicality: this is the only canonical register of inferred choices. Assumptions are non-authoritative and must not be restated elsewhere as facts.

## ASM-001 — “First playable” as the current planning horizon

- Assumption: The initiation package treats the first playable as the immediate planning horizon, not as the final product definition.
- Impact: medium
- Confidence: medium
- Status: active, unapproved
- Basis: `SRC-006` names a first playable but does not define milestones.
- Consequence if wrong: architecture and validation sequencing may need revision.
- Validation/owner: stakeholder confirms milestone intent; see `OQ-011`.
- Used by: [TechnicalArchitecture.md](TechnicalArchitecture.md), [Research-05-Delivery.md](Research-05-Delivery.md).

## ASM-002 — Single-room wording describes presentation scope

- Assumption: “in a single room” is treated as a permitted first-playable presentation boundary, not proof that the final game has one scene, one level, or one software module.
- Impact: high
- Confidence: high
- Status: active, unapproved interpretive safeguard
- Basis: the permissive wording in `SRC-006` and absence of authorized exclusions in `ANS-006`.
- Consequence if wrong: content and scene architecture may be over- or under-partitioned.
- Validation/owner: stakeholder confirms first-playable spatial scope; see `OQ-011`.
- Used by: `D-003`, `D-014`, `DAD-REQ-014`.

## ASM-003 — Engine-agnostic architecture is useful at initiation

- Assumption: A behavioral component model can be planned before an engine, language, renderer, or audio middleware is selected.
- Impact: high
- Confidence: medium
- Status: active, unapproved, reversible
- Basis: the brief specifies behavior but no technical stack; `D-006` records the provisional choice.
- Consequence if wrong: component interfaces may need to be recast around engine-native patterns.
- Validation/owner: technical owner answers `OQ-007` after team, budget, licensing, and platform needs are known.
- Used by: [TechnicalArchitecture.md](TechnicalArchitecture.md).

## ASM-004 — Deterministic fixtures are feasible

- Assumption: Core signal, caller, threat, timing, and scoring rules can be driven by fixed seeds or fixed fixtures for verification.
- Impact: high
- Confidence: medium
- Status: active, unapproved design assumption
- Basis: exact acceptance tests for `DAD-REQ-006` through `DAD-REQ-013` require controlled comparison; no brief clause forbids deterministic simulation.
- Consequence if wrong: objective verification becomes more expensive and may require recorded/replayed input streams.
- Validation/owner: prototype spike demonstrates repeatable state logs before system implementation.
- Used by: `D-014` and the verification catalog in [Requirements.md](Requirements.md).

## Explicit non-assumptions

No assumption is made about engine, programming language, PC operating systems, minimum hardware, storefront, business model, intended audience, age rating, multiplayer, persistence, score weights, run-duration bounds, accessibility feature adoption, fairness threshold, team size, budget, schedule, or final exclusions. These remain in [OpenQuestions.md](OpenQuestions.md).

