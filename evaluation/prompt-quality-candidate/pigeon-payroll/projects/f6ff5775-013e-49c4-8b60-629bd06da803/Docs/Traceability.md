# Traceability and decisions

This file is the canonical consequential-decision register and exact trace-edge ledger. Each edge occupies its own row and includes a semantic explanation; proximity, wording similarity, ID grouping, or an ID range does not imply a relationship.

## Constraint register

| Constraint | Status | Provenance type | Exact source wording | Source ID | Rationale |
|---|---|---|---|---|---|
| CON-001 | active | stakeholder brief | `Each shift lasts roughly 60 to 90 seconds` | BCL-004 | Sets the prototype session-duration boundary. |
| CON-002 | active | stakeholder brief | `playful rather than stressful` | BCL-005 | Sets the desired emotional tone. |
| CON-003 | active | stakeholder brief | `portrait-mode play` | BCL-005 | Sets playable orientation. |
| CON-004 | active | stakeholder brief | `A first prototype only needs one cafe screen` | BCL-006 | Bounds playable scene scope. |
| CON-005 | active | stakeholder brief | `three customer tables` | BCL-006 | Sets an exact table count. |
| CON-006 | active but quantitatively ambiguous | stakeholder brief | `a small set of pigeon behaviours` | BCL-006 | Bounds behaviour scope qualitatively; OQ-003 must supply the exact completion. |

## Consequential decision register

| Decision | One subject | Status | Provenance type | Exact source IDs | Relevant evidence IDs | Rationale |
|---|---|---|---|---|---|---|
| DEC-001 | Product name | accepted | stakeholder brief | BCL-001 | none | Preserve the supplied identity. |
| DEC-002 | Product format | accepted | stakeholder brief | BCL-002 | none | The prototype is a comedic mobile arcade game. |
| DEC-003 | Player role and objective | accepted | stakeholder brief | BCL-002 | none | The cafe worker protects outdoor customers from pigeons. |
| DEC-004 | Primary control | accepted | stakeholder brief | BCL-003 | none | The primary loop uses one-finger swipes. |
| DEC-005 | Breadcrumb herding action | accepted | stakeholder brief | BCL-003 | none | A swipe throws breadcrumbs that can herd the flock. |
| DEC-006 | Breadcrumb attraction consequence | accepted | stakeholder brief | BCL-003 | none | Each breadcrumb must also increase pigeon attraction pressure. |
| DEC-007 | Cleanup scoring consequence | accepted | stakeholder brief | BCL-003 | none | Breadcrumb use adds a bill deducted from final score. |
| DEC-008 | Shift duration | accepted | stakeholder brief | BCL-004 | none | A completed shift must stay within the stated duration boundary. |
| DEC-009 | Chaos escalation | partially accepted | stakeholder brief plus design assumption | BCL-004; A-005 | none | Escalation is authoritative; the concurrent-threat verification definition is non-authoritative pending OQ-006. |
| DEC-010 | Customer table movement | accepted | stakeholder brief | BCL-004 | none | Customers moving tables is a named source of escalation. |
| DEC-011 | Pastry appearance | accepted | stakeholder brief | BCL-004 | none | Pastries appearing is a named shift event. |
| DEC-012 | Impatient food-steal combos | accepted with tuning open | stakeholder brief | BCL-004 | none | The behaviour is required; thresholds and combo window remain OQ-006. |
| DEC-013 | Experience tone | accepted as intent, not evaluated | stakeholder brief | BCL-005 | none | The tone target is authoritative; passing it requires OQ-004 criteria. |
| DEC-014 | Character visual direction | proposed within accepted readability requirement | stakeholder brief plus design assumption | BCL-005; A-006 | none | Bold readability is authoritative; flat shapes, palette, and styling remain proposed. |
| DEC-015 | Slapstick audio | accepted | stakeholder brief | BCL-005 | none | Slapstick sound effects are required presentation feedback. |
| DEC-016 | Play orientation | accepted | stakeholder brief | BCL-005 | none | Portrait mode is explicit. |
| DEC-017 | Cafe gameplay-screen count | accepted | stakeholder brief | BCL-006 | none | The first prototype contains one playable cafe screen. |
| DEC-018 | Customer-table count | accepted | stakeholder brief | BCL-006 | none | The screen contains exactly three table anchors. |
| DEC-019 | Exact pigeon-behaviour set | partially accepted and otherwise deferred | stakeholder brief plus imported failed clarification | BCL-003; BCL-004; BCL-006; ANS-003 | EVD-008; EVD-009; EVD-010 | Attraction and impatient steal/combo behaviour are authoritative; external evidence cannot authorize the remaining set. |
| DEC-020 | Intended audience | deferred to stakeholder | stakeholder brief plus imported failed clarification | BCL-007; ANS-001 | EVD-001; EVD-002; EVD-003; EVD-004 | Research establishes consequences, not product-positioning authority. |
| DEC-021 | Failure rule | deferred to stakeholder | stakeholder brief plus imported failed clarification | BCL-007; ANS-002 | EVD-005; EVD-006; EVD-007 | Evidence does not directly compare or authorize the two candidates. |
| DEC-022 | Prototype success criteria | deferred to stakeholder | imported failed clarification | ANS-004 | EVD-011; EVD-012; EVD-013 | General evaluation guidance supplies no project-specific thresholds. |
| DEC-023 | Connected services in the first prototype | assumed out of scope | non-authoritative planning assumption | A-001 | EVD-002 | Keeping services absent limits unauthorized data scope; OQ-008 can replace the assumption. |
| DEC-024 | Technology stack | deferred | non-authoritative planning assumption | A-002 | none | Platform, engine, device, team, budget, and schedule constraints are unavailable under OQ-005. |
| DEC-025 | Scene dimensionality | proposed | non-authoritative design assumption | A-003 | none | A 2D playfield is a scope-oriented inference, not brief authority. |
| DEC-026 | Development verification instrumentation | proposed | non-authoritative engineering assumption | A-004 | none | Deterministic seeds and event capture support exact verification without defining product behaviour. |

## Exact source-to-requirement edges

| From | To | Relationship | Semantic relevance |
|---|---|---|---|
| BCL-001 | REQ-001 | generates | The exact source text supplies the product title. |
| BCL-002 | REQ-002 | generates | The clause explicitly supplies comedy, mobile, and arcade format. |
| BCL-002 | REQ-003 | generates | The clause explicitly supplies cafe-worker role, pigeons, and protected customers. |
| BCL-003 | REQ-004 | generates | The clause explicitly supplies one-finger swipes. |
| BCL-003 | REQ-005 | generates | The clause explicitly connects swipes, breadcrumb throwing, and herding. |
| BCL-003 | REQ-006 | generates | The clause explicitly states every breadcrumb attracts more pigeons. |
| BCL-003 | REQ-007 | generates | The clause explicitly supplies cleanup billing and final-score deduction. |
| BCL-004 | REQ-008 | generates | The clause supplies the 60-to-90-second shift duration. |
| BCL-004 | REQ-009 | generates | The clause explicitly requires increasing chaos. |
| BCL-004 | REQ-010 | generates | Customer table movement is a named chaos event. |
| BCL-004 | REQ-011 | generates | Pastry appearance is a named chaos event. |
| BCL-004 | REQ-012 | generates | Impatient birds stealing food to form combos is explicit. |
| BCL-005 | REQ-013 | generates | The clause supplies playful rather than stressful tone. |
| BCL-005 | REQ-014 | generates | The clause requires bold readable characters. |
| BCL-005 | REQ-015 | generates | The clause requires slapstick sound effects. |
| BCL-005 | REQ-016 | generates | The clause requires portrait-mode play. |
| BCL-006 | REQ-017 | generates | The clause limits the first prototype to one cafe screen. |
| BCL-006 | REQ-018 | generates | The clause supplies three customer tables. |
| BCL-006 | REQ-019 | generates | The clause qualitatively limits pigeon-behaviour scope. |
| BCL-007 | REQ-020 | generates governance requirement | The clause explicitly states audience uncertainty. |
| ANS-001 | REQ-020 | preserves authority boundary | The imported answer says research did not resolve audience selection. |
| BCL-007 | REQ-021 | generates governance requirement | The clause explicitly states failure-rule uncertainty. |
| ANS-002 | REQ-021 | preserves authority boundary | The imported answer says research did not resolve the failure rule. |
| ANS-003 | REQ-022 | generates governance requirement | The imported answer identifies the exact behaviour set and transitions as stakeholder-owned. |
| ANS-004 | REQ-023 | generates governance requirement | The imported answer identifies success criteria and thresholds as stakeholder-owned. |

## Exact requirement-to-decision-or-constraint edges

| From | To | Relationship | Semantic relevance |
|---|---|---|---|
| REQ-001 | DEC-001 | governed by | The decision accepts the exact product name. |
| REQ-002 | DEC-002 | governed by | The decision accepts the required product format. |
| REQ-003 | DEC-003 | governed by | The decision accepts the role and protection objective. |
| REQ-004 | DEC-004 | governed by | The decision accepts one-finger swipe control. |
| REQ-005 | DEC-005 | governed by | The decision accepts breadcrumb throwing as the herding action. |
| REQ-006 | DEC-006 | governed by | The decision accepts attraction as a breadcrumb consequence. |
| REQ-007 | DEC-007 | governed by | The decision accepts cleanup subtraction from final score. |
| REQ-008 | CON-001 | constrained by | The constraint gives the exact shift-duration boundary. |
| REQ-009 | DEC-009 | governed by | The decision distinguishes accepted escalation intent from the proposed metric. |
| REQ-010 | DEC-010 | governed by | The decision accepts customer table movement. |
| REQ-011 | DEC-011 | governed by | The decision accepts pastry appearance. |
| REQ-012 | DEC-012 | governed by | The decision accepts impatient steal combos while leaving values open. |
| REQ-013 | CON-002 | constrained by | The constraint gives the authoritative tone direction. |
| REQ-014 | DEC-014 | governed by | The decision separates required readability from proposed styling. |
| REQ-015 | DEC-015 | governed by | The decision accepts slapstick audio. |
| REQ-016 | CON-003 | constrained by | The constraint fixes portrait play. |
| REQ-017 | CON-004 | constrained by | The constraint fixes one cafe screen. |
| REQ-018 | CON-005 | constrained by | The constraint fixes three customer tables. |
| REQ-019 | CON-006 | constrained by | The constraint limits behaviours but leaves exact count open. |
| REQ-020 | DEC-020 | governed by | The decision explicitly defers audience selection. |
| REQ-021 | DEC-021 | governed by | The decision explicitly defers the failure rule. |
| REQ-022 | DEC-019 | governed by | The decision separates mandatory behaviour from unapproved additions. |
| REQ-023 | DEC-022 | governed by | The decision explicitly defers success criteria. |

## Exact evidence-to-decision edges

| From | To | Relationship | Semantic relevance |
|---|---|---|---|
| EVD-001 | DEC-020 | constrains | Directly bears on the platform-declaration consequence of OQ-001 without selecting a candidate. |
| EVD-002 | DEC-020 | constrains | Directly bears on the data-practice consequence of OQ-001 without selecting a candidate. |
| EVD-003 | DEC-020 | constrains | Directly bears on the regulatory-classification consequence of OQ-001 without supplying stakeholder intent. |
| EVD-004 | DEC-020 | constrains | Directly bears on the Apple category consequence of OQ-001 without selecting a candidate. |
| EVD-005 | DEC-021 | informs without authorizing | Directly informs the experience dimension of OQ-002 but does not compare its two candidates. |
| EVD-006 | DEC-021 | informs without authorizing | Directly informs the challenge dimension of OQ-002 but prescribes no candidate. |
| EVD-007 | DEC-021 | informs without authorizing | Directly informs the feedback dimension of OQ-002 with a context-transfer limitation. |
| EVD-008 | DEC-019 | inspires without authorizing | Directly informs biological plausibility for OQ-003, not game suitability or approval. |
| EVD-009 | DEC-019 | inspires without authorizing | Directly informs coordination plausibility for OQ-003, not game suitability or approval. |
| EVD-010 | DEC-019 | inspires without authorizing | Directly informs social-foraging plausibility for OQ-003, not game suitability or approval. |
| EVD-011 | DEC-022 | constrains | Directly informs the usability-framework dimension of OQ-004 without supplying criteria. |
| EVD-012 | DEC-022 | informs without authorizing | Directly informs possible measurement categories for OQ-004 without supplying thresholds. |
| EVD-013 | DEC-022 | constrains | Directly informs stakeholder ownership and verifiability for OQ-004 without supplying targets. |

## Exact assumption-to-decision edges

| From | To | Relationship | Semantic relevance |
|---|---|---|---|
| A-001 | DEC-023 | provisional basis | The local-only assumption keeps unrequested services out of prototype scope. |
| A-002 | DEC-024 | provisional basis | Missing delivery constraints require stack deferral. |
| A-003 | DEC-025 | provisional basis | The 2D proposal supplies a disposable scene model. |
| A-004 | DEC-026 | provisional basis | Repeatability needs development-only instrumentation. |
| A-005 | DEC-009 | provisional metric | The assumption operationalizes escalation for verification without claiming authority. |
| A-006 | DEC-014 | provisional style | The assumption proposes a visual treatment beneath required readability. |

## Research coverage rule

Canonical claims exist only in `Research-01-Audience.md`, `Research-02-Experience.md`, and `Research-05-Delivery.md`. Each retained evidence ID has one explicit evidence-to-decision edge above. `Research-03-Market.md` and `Research-04-Technology.md` explicitly record unavailable research and retain no claims.
