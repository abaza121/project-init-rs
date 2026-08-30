# Assumptions Register

This is the canonical register for inferred choices. Every assumption is non-authoritative until a stakeholder approves it. An assumption may support a provisional test but may not close an open question or silently become product scope.

| ID | Impact | Status | Non-authoritative working assumption | Used by | Falsification or approval path |
|---|---|---|---|---|---|
| ASM-001 | High | Proposed | Television evaluation uses a 55-inch 1920×1080 display viewed from 3 metres. | REQ-016; DEC-035 | Stakeholder approves a reference display and distance, or OQ-010D records replacements. |
| ASM-002 | High | Proposed | “Quick” restart is provisionally tested as controllable gameplay within 5.0 seconds after restart input. | REQ-012; DEC-034 | Measure on target hardware after OQ-005; stakeholder approves or replaces threshold through OQ-010E. |
| ASM-003 | Medium | Proposed | Exaggeration is provisionally observable through at least one mech-height of throw travel and one-quarter mech-height of damage recoil, with no more than 1.0 second loss of control. | REQ-013; DEC-036 | Playtest capture demonstrates inadequacy, or stakeholder approves/replaces values. |
| ASM-004 | High | Proposed | A five-person evaluator set and four-of-five recognition gate are sufficient for a prototype silhouette check. Evaluator age cannot be targeted until OQ-004 closes. | REQ-014; DEC-037 | Audience decision and evaluation plan replace cohort and gate. |
| ASM-005 | High | Proposed | A five-pair playtest with a four-of-five-pairs amusement gate is sufficient only as a prototype comedy signal. | REQ-015; REQ-023; DEC-038 | OQ-010B approves comedy definition, cohort, observation method, and threshold. |
| ASM-006 | High | Proposed | A five-person evaluator set and four-of-five correct-report gate are sufficient for a prototype television-readability check. | REQ-016; DEC-035 | OQ-010D approves readability context and threshold. |
| ASM-007 | High | Proposed | A within-pair coordinated-versus-deliberately-uncoordinated comparison can indicate whether coordination affects rescue outcomes. It does not define desired effect size. | REQ-021; DEC-040 | OQ-010C approves outcome metric, control protocol, cohort, and effect threshold. |
| ASM-008 | High | Proposed | First-to-fifth-attempt improvement for four of five pairs is a prototype mastery signal, not proof of long-term mastery. | REQ-022; DEC-039 | OQ-010A approves mastery definition, cohort, attempt count, and threshold. |
| ASM-009 | High | Proposed | Until engine and platform are chosen, the architecture remains engine-agnostic, uses abstract ports, and specifies no vendor SDK. | TechnicalArchitecture.md; DEC-041 | OQ-005 and OQ-006 close with compatible selections. |
| ASM-010 | Medium | Proposed | Unselected civilian and hazard identities are represented by neutral test-fixture archetypes, not creative commitments. | REQ-019; REQ-020; DEC-025; DEC-026 | OQ-007 and OQ-008 close. |
| ASM-011 | Medium | Proposed | The one authored city-block unit may be streamed within a larger logical zone, but neither equality nor inequality between those boundaries is assumed. | REQ-007; REQ-018; DEC-024 | OQ-009 closes. |
| ASM-012 | High | Proposed | Online services, monetisation, progression, extra content, and release packaging are not planned in the initiation slice, but are not declared exclusions. | DEC-033; Research-05-Delivery.md | Stakeholder answers OQ-011A, OQ-011B, OQ-011C, and OQ-011D. |

## Usage rule

When an assumption appears outside this file, it must retain the label **non-authoritative** and cite its exact `ASM-*` ID. Acceptance of a prototype under an assumed threshold is provisional; it cannot be represented as stakeholder acceptance of the underlying qualitative brief term.
