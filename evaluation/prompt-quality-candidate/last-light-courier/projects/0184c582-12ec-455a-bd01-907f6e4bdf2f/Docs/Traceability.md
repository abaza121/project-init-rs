# Traceability

This file is the canonical decision, constraint, and trace-edge register. Each row names exact IDs; no ranges or slash-combined surrogate edges are used. Research evidence constrains or informs decisions but never supplies stakeholder authority.

## Decision Register

| ID | One subject | Status | Provenance type | Exact source IDs | Relevant evidence IDs | Rationale |
|---|---|---|---|---|---|---|
| DEC-001 | Product name | Accepted | Stakeholder brief | SC-001 | None | The brief directly names the project. |
| DEC-002 | Core single-screen courier, trail, and delivery loop | Accepted | Stakeholder brief | SC-002; SC-004; SC-005; SC-006; SC-007; SC-009; SC-010; SC-017 | None | These clauses jointly define the authorized core loop; no external claim is needed. |
| DEC-003 | Battery-timed run structure | Accepted with unresolved tuning | Stakeholder brief plus design inference | SC-008; SC-012; ASS-001; OQ-006 | None | Battery expiry and an approximately two-minute run are authoritative; numeric tolerance is not. |
| DEC-004 | Close-pass multiplier risk and reward | Accepted with unresolved tuning | Stakeholder brief plus design inference | SC-011; SC-017; ASS-006; OQ-014 | None | The mechanic is authorized, but its proximity band and timing are tuning hypotheses. |
| DEC-005 | Simple neon visual direction | Accepted direction; identity details proposed | Stakeholder brief plus creative inference | SC-004; SC-014; SC-016; ASS-004; ASS-010; OQ-012 | EVD-004; EVD-006; EVD-007 | The brief authorizes neon shapes and no detailed art. Evidence makes redundant cues and effect safeguards relevant, not mandatory. |
| DEC-006 | TypeScript, Phaser 4.1.x, and Vite web-first architecture | Provisional | Technical design inference | SC-002; SC-003; SC-013; ASS-005; ASS-008; ASS-009; OQ-008 | EVD-014; EVD-015; EVD-016; EVD-017; EVD-018 | The stack directly supports a 2D browser build, primitive rendering, TypeScript, static output, audio policy handling, and visibility events. Pin exact patch versions at implementation start. |
| DEC-007 | Deterministic domain simulation separated from rendering | Provisional | Technical design inference | SC-005; SC-006; SC-007; SC-008; SC-009; SC-010; SC-011; SC-012; ASS-003; ASS-007 | None | Pure, fixed-step state transitions make collision, timing, score, and replayable test fixtures verifiable across renderers. |
| DEC-008 | Difficulty | Deferred | Stakeholder authority boundary | SC-019; ANS-001; GOV-001; OQ-001 | EVD-001; EVD-002 | The brief withholds this choice; evidence supports player-relative evaluation only. |
| DEC-009 | Accessibility scope | Deferred | Stakeholder authority boundary | SC-019; ANS-002; GOV-002; OQ-002 | EVD-003; EVD-004; EVD-005; EVD-006; EVD-007 | Relevant practices are documented, but none is adopted by research alone. |
| DEC-010 | Longer-term progression | Deferred | Stakeholder authority boundary | SC-019; ANS-003; GOV-003; OQ-003 | EVD-008 | Player-motive variation makes purpose and audience relevant; it does not select a progression model. |
| DEC-011 | Intended audience | Deferred | Stakeholder authority boundary | ANS-004; GOV-004; OQ-004 | EVD-009; EVD-010 | Audience requires stakeholder direction or actual discovery with defined recruitment criteria. |
| DEC-012 | Additional success thresholds | Deferred | Stakeholder authority boundary | ANS-005; GOV-005; OQ-005; OQ-007 | EVD-011; EVD-012; EVD-013 | Research supplies candidate measurement methods, not project outcomes or pass values. |
| DEC-013 | Native desktop packaging | Deferred | Scope and design inference boundary | SC-003; ASS-005; OQ-008 | EVD-014; EVD-015 | A web build is feasible, but the phrase “desktop and browser” does not settle packaging. |
| DEC-014 | Strong audio-feedback direction | Accepted direction; control scope proposed | Stakeholder brief plus design inference | SC-015; OQ-011 | EVD-004; EVD-005; EVD-017 | The brief authorizes strong audio feedback; evidence informs cue redundancy, controls, and browser activation but does not authorize mixer categories. |

## Constraint Register

| ID | Constraint | Authority | Applies to |
|---|---|---|---|
| CON-001 | Use simple neon shapes rather than detailed art. | SC-014; SC-016 | REQ-004; REQ-014; REQ-016 |
| CON-002 | Battery expiry bounds the run; approximate duration tolerance is unresolved. | SC-008; SC-012; OQ-006 | REQ-008; REQ-012 |
| CON-003 | Close passes preserve the multiplier but may not bypass collision. | SC-011; SC-012 | REQ-011; REQ-017 |
| CON-004 | Old trails are solid obstacles; exact eligibility and consequence remain unresolved. | SC-006; SC-012; OQ-010 | REQ-006; REQ-012 |
| CON-005 | Delivery depends on parcel and tower matching. | SC-010 | REQ-010 |
| CON-006 | Required prototype interaction is keyboard-operable; accessibility expansion is unresolved. | SC-013; OQ-002 | REQ-013 |
| CON-007 | Critical events require strong audio feedback; control categories are unresolved. | SC-015; OQ-011 | REQ-015 |
| CON-008 | Mandatory onboarding must fit “a few seconds”; numeric interpretation is unresolved. | SC-018; OQ-007 | REQ-018 |

## Requirement Trace Edges

| Requirement | Exact source edge | Decision or constraint edge | Verification edge | Why the edge is meaningful |
|---|---|---|---|---|
| REQ-001 | SC-001 | DEC-001 | T-REQ-001 | Name source controls title verification. |
| REQ-002 | SC-002 | DEC-002 | T-REQ-002; T-MOVE-001 | Game-form clause controls viewport, render review, and approved motion response. |
| REQ-003 | SC-003 | DEC-006; DEC-013 | T-PLAT-001; T-PLAT-002 | Delivery phrase drives web architecture and unresolved packaging. |
| REQ-004 | SC-004 | DEC-002; CON-001 | T-REQ-004; V-IDENT-001 | Role and theme clause controls scene hierarchy and art treatment. |
| REQ-005 | SC-005 | DEC-002; DEC-005 | T-TRAIL-001; V-FX-001 | Trail and power clause controls state and feedback. |
| REQ-006 | SC-006 | DEC-007; CON-004 | T-COLL-001 | Solid-obstacle clause controls collision simulation. |
| REQ-007 | SC-007 | DEC-002; DEC-007 | T-MAZE-001 | Maze clause controls trail persistence and path changes. |
| REQ-008 | SC-008 | DEC-003; CON-002 | T-TIME-001; V-TIME-001 | Duration clause controls battery timing. |
| REQ-009 | SC-009 | DEC-002; DEC-007 | T-PARCEL-001 | Collection clause controls parcel state transition. |
| REQ-010 | SC-010 | DEC-002; CON-005 | T-DELIVERY-001 | Matching clause controls valid delivery. |
| REQ-011 | SC-011 | DEC-004; CON-003 | T-MULT-001 | Close-pass clause controls multiplier preservation. |
| REQ-012 | SC-012 | DEC-003; CON-004 | T-END-001 | Collision and battery clause controls run termination. |
| REQ-013 | SC-013 | DEC-004; CON-006 | T-INPUT-001; V-INPUT-001 | Keyboard clause controls the full interaction path. |
| REQ-014 | SC-014 | DEC-005; CON-001 | V-IDENT-001 | Shape clause controls visual asset style. |
| REQ-015 | SC-015 | DEC-014; CON-007 | T-AUDIO-001; V-AUDIO-001 | Audio clause controls event cue mapping. |
| REQ-016 | SC-016 | DEC-005; CON-001 | T-ASSET-001 | Art constraint controls the asset manifest. |
| REQ-017 | SC-017 | DEC-002; DEC-004 | T-DESIGN-001; T-MULT-001 | Reward clause controls paired scoring fixtures. |
| REQ-018 | SC-018 | DEC-012; CON-008 | V-ONBOARD-001 | Tutorial clause controls onboarding timing. |
| REQ-019 | SC-019 | DEC-008; DEC-009; DEC-010 | T-GOV-001 | Explicit undecided clause controls deferral. |
| GOV-001 | CTX-REQ-001 | DEC-008 | T-GOV-002 | Imported authority boundary controls difficulty language. |
| GOV-002 | CTX-REQ-002 | DEC-009 | T-GOV-003 | Imported authority boundary controls accessibility language. |
| GOV-003 | CTX-REQ-003 | DEC-010 | T-GOV-004 | Imported authority boundary controls progression language. |
| GOV-004 | CTX-REQ-004 | DEC-011 | T-GOV-005 | Imported authority boundary controls audience language. |
| GOV-005 | CTX-REQ-005 | DEC-012 | T-GOV-006 | Imported authority boundary controls success claims. |

## Evidence-to-Outcome Edges

| Evidence | Informs | Exact relevance |
|---|---|---|
| EVD-001 | DEC-008 | Explains why difficulty depends on player abilities. |
| EVD-002 | DEC-008 | Supports representative consultation before selecting options. |
| EVD-003 | DEC-009 | Identifies input-access considerations relevant to keyboard play. |
| EVD-004 | DEC-005 | Supports considering redundant critical cues in neon visuals. |
| EVD-004 | DEC-009 | Identifies a candidate accessibility practice without adopting it. |
| EVD-005 | DEC-009 | Identifies candidate audio controls without adopting categories. |
| EVD-005 | DEC-014 | Constrains the proposed audio-control direction. |
| EVD-006 | DEC-005 | Constrains proposed motion effects. |
| EVD-006 | DEC-009 | Identifies reduced-motion considerations. |
| EVD-007 | DEC-005 | Constrains flashing-effect exploration. |
| EVD-007 | DEC-009 | Identifies photosensitivity testing considerations. |
| EVD-008 | DEC-010 | Supports deferring progression until purpose and audience are known. |
| EVD-009 | DEC-011 | Supports likely-user research when audience is unclear. |
| EVD-010 | DEC-011 | Supports explicit participant criteria. |
| EVD-011 | DEC-012 | Frames usability around specified users, goals, and context. |
| EVD-012 | DEC-012 | Supplies candidate usability data categories. |
| EVD-013 | DEC-012 | Supplies candidate benchmarking measures. |
| EVD-014 | DEC-006 | Supports Phaser as a 2D web framework. |
| EVD-014 | DEC-013 | Shows native packaging depends on third-party tooling. |
| EVD-015 | DEC-006 | Supports a static production web bundle and explicit browser targets. |
| EVD-016 | DEC-006 | Supports Canvas fallback and cautious use of renderer-specific effects. |
| EVD-017 | DEC-006 | Requires audio activation from user interaction. |
| EVD-017 | DEC-014 | Constrains how authorized audio feedback starts in browsers. |
| EVD-018 | DEC-006 | Supports pausing or explicitly handling hidden-tab state. |

## File Navigation Edges

All relative Markdown links were checked by the procedure in [ValidationReport.md](ValidationReport.md). Canonical registers are [Requirements.md](Requirements.md), [Assumptions.md](Assumptions.md), [OpenQuestions.md](OpenQuestions.md), this file, and the evidence records distributed once across the five Research files.
