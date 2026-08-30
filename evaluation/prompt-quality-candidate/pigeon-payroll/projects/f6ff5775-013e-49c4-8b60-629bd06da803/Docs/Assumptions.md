# Assumptions

This is the canonical register for inferred choices. Every item is **non-authoritative**. An assumption may support planning or a disposable prototype, but it cannot override the brief or close an open question.

| Assumption | Impact | Non-authoritative working statement | Basis | Affected records | Validation or retirement trigger |
|---|---|---|---|---|---|
| A-001 | high | **ASSUMPTION:** The prototype is single-player and locally playable without accounts, networking, ads, analytics, or purchases. | The brief describes one player and no connected systems; silence is not approval. | DEC-023; TechnicalArchitecture.md | Stakeholder confirms service, monetization, telemetry, and privacy scope; otherwise keep integrations out of the prototype. |
| A-002 | high | **ASSUMPTION:** Technology and engine remain unselected; the architecture is logical and engine-agnostic. | No platform, engine, language, device floor, budget, or team capability was supplied. | DEC-024; OQ-005; Research-04-Technology.md | Stakeholder supplies target platforms and delivery constraints, followed by a recorded technology decision. |
| A-003 | medium | **ASSUMPTION:** The one cafe is a single 2D playfield with overlays, rather than a 3D navigable environment. | Portrait, one-screen arcade scope suggests this implementation, but the brief does not state dimensionality. | DEC-025; TechnicalArchitecture.md; VisualIdentity.md | Approve an art/interaction prototype or replace the assumption. |
| A-004 | medium | **ASSUMPTION:** Deterministic seeds and event telemetry may be included in development builds solely to verify gameplay requirements. | Objective verification needs repeatable observations; this is an engineering inference. | VER-004; VER-005; TechnicalArchitecture.md | Technical review accepts the harness, or specifies an equivalent exact verification method. |
| A-005 | high | **ASSUMPTION:** For prototype verification only, “becomes more chaotic” means a higher concurrent-threat budget in the final third than the first third, plus exposure to at least two named event types. | BCL-004 requires escalation but gives no metric. | REQ-009; AC-009; OQ-006 | Stakeholder approves or replaces the escalation measure and values. |
| A-006 | medium | **ASSUMPTION:** Visual guidance uses flat, high-contrast, exaggerated shapes and avoids detailed realism. | This is a design inference from BCL-005, not explicit art-direction authority. | DEC-014; VisualIdentity.md; BrandPrompt.md | Stakeholder approves a moodboard or revised identity direction. |
| A-007 | medium | **ASSUMPTION:** Mission and vision statements in this package are internal proposed summaries, not approved public copy. | The brief supplies intent but no corporate mission or vision. | MissionVision.md | Stakeholder approves or replaces each statement. |

## Guardrail

No assumption in this register selects the audience, failure rule, complete behaviour set, success criteria, technology stack, or commercial model.
