# Requirements

This is the canonical requirement register. Source clauses reproduce the material brief wording exactly. A requirement is not complete merely because prose resembles its source: it must pass its named acceptance criterion using the named verification.

## Source-clause inventory

| Source ID | Exact source wording | Generated requirement |
|---|---|---|
| BCL-001 | `Pigeon Payroll` | REQ-001 |
| BCL-002 | `I want to create a comedic mobile arcade game about a cafe worker who must keep a flock of pigeons away from outdoor customers.` | REQ-002; REQ-003 |
| BCL-003 | `The player uses one-finger swipes to throw breadcrumbs and herd the flock, but every breadcrumb also attracts more pigeons and adds to a cleanup bill deducted from the final score.` | REQ-004; REQ-005; REQ-006; REQ-007 |
| BCL-004 | `Each shift lasts roughly 60 to 90 seconds and becomes more chaotic as customers move tables, pastries appear, and impatient birds form combos by stealing food.` | REQ-008; REQ-009; REQ-010; REQ-011; REQ-012 |
| BCL-005 | `The tone should be playful rather than stressful, with bold readable characters, slapstick sound effects, and portrait-mode play.` | REQ-013; REQ-014; REQ-015; REQ-016 |
| BCL-006 | `A first prototype only needs one cafe screen, three customer tables, and a small set of pigeon behaviours.` | REQ-017; REQ-018; REQ-019 |
| BCL-007 | `I am unsure whether the best audience is children, casual adults, or both, and whether failure should end a run or simply reduce the player's wages.` | REQ-020; REQ-021 |

## Imported governance-source mappings

The exact imported non-decision texts are canonical in [OpenQuestions.md](OpenQuestions.md).

| Source ID | Generated requirement |
|---|---|
| ANS-001 | REQ-020 |
| ANS-002 | REQ-021 |
| ANS-003 | REQ-022 |
| ANS-004 | REQ-023 |

## Verification catalogue

| Verification ID | Exact test or inspection |
|---|---|
| VER-001 | Inspect the installed product title and the prototype title screen; both must equal `Pigeon Payroll` character-for-character. |
| VER-002 | On a supported mobile build, start a shift and inspect genre presentation, player role, customer targets, and comic feedback against AC-002 and AC-003. |
| VER-003 | Run the input instrumentation test: complete the primary throw/herd loop with one active pointer and record gesture-to-action events. |
| VER-004 | In a deterministic seeded shift, throw one breadcrumb and inspect telemetry for breadcrumb placement, flock steering response, an increase in attraction pressure, cleanup-bill increment, and final-score subtraction. |
| VER-005 | Run three deterministic full shifts and compare elapsed time, phase counters, table-move events, pastry events, and steal/combo events against AC-008 through AC-012. |
| VER-006 | Conduct the stakeholder-approved tone review and readability inspection at the supported portrait reference viewport; record each checklist result and sound-event trigger. This verification cannot be executed until OQ-004 supplies success thresholds where perception is involved. |
| VER-007 | Inspect the prototype scene manifest and runtime object count for cafe screens and customer tables; inspect the behaviour registry against the stakeholder-approved list. |
| VER-008 | Inspect this package and product configuration for an explicit unresolved marker and absence of an unauthorized audience value. |
| VER-009 | Inspect this package and prototype rule configuration for an explicit unresolved marker and absence of an unauthorized run-ending or wage-only failure choice. |
| VER-010 | Inspect the behaviour registry: breadcrumb attraction and impatient food-stealing combo behaviour must be present; no additional behaviour may be represented as stakeholder-approved without an approved OQ-003 answer. |
| VER-011 | Inspect the evaluation plan and release gate: no prototype-success claim may be made until an approved OQ-004 answer defines measures and pass/fail thresholds and recorded results meet them. |

## Canonical register

| Requirement | Type; priority | Normative statement | Source | Objective pass/fail acceptance criterion | Verification | Relevant decision or constraint |
|---|---|---|---|---|---|---|
| REQ-001 | identity; medium | The product title shall be `Pigeon Payroll`. | BCL-001 | **AC-001 — Title identity.** Pass only if both inspected title locations equal `Pigeon Payroll`; otherwise fail. | VER-001 | DEC-001 |
| REQ-002 | product; high | The prototype shall be a comedic mobile arcade game. | BCL-002 | **AC-002 — Format and genre.** Pass only if a build launches on a supported mobile target, presents a real-time score-driven arcade shift, and the evaluation protocol approved through OQ-004 classifies its presentation as comedic at the approved threshold; otherwise fail or remain not evaluated. | VER-002 | DEC-002 |
| REQ-003 | gameplay; high | The player shall act as a cafe worker keeping pigeons away from outdoor customers. | BCL-002 | **AC-003 — Role and objective.** Pass only if a shift identifies the player role as cafe worker, includes outdoor customers, and pigeon proximity or stealing creates an observable protect-customer objective; otherwise fail. | VER-002 | DEC-003 |
| REQ-004 | input; high | The primary gameplay shall use one-finger swipes. | BCL-003 | **AC-004 — One-pointer control.** Pass only if the primary loop is completable with one active pointer and every sampled primary action is initiated by a swipe; otherwise fail. | VER-003 | DEC-004 |
| REQ-005 | gameplay; high | A swipe shall throw breadcrumbs and permit the player to herd the flock. | BCL-003 | **AC-005 — Throw and herd response.** Pass only if a recorded swipe creates a breadcrumb target, at least one eligible pigeon changes target to that breadcrumb, and that pigeon's distance from a designated customer is greater at the stable post-response sample than at the pre-swipe sample; otherwise fail. | VER-004 | DEC-005 |
| REQ-006 | gameplay; high | Every breadcrumb shall also attract more pigeons. | BCL-003 | **AC-006 — Attraction trade-off.** Pass only if each breadcrumb event increases the defined attraction pressure relative to the immediately preceding state; otherwise fail. The magnitude remains tuning work. | VER-004 | DEC-006 |
| REQ-007 | scoring; high | Every breadcrumb shall add to a cleanup bill deducted from the final score. | BCL-003 | **AC-007 — Cleanup deduction.** Pass only if each breadcrumb increments the cleanup bill and the results calculation subtracts exactly the displayed bill from the pre-deduction score; otherwise fail. | VER-004 | DEC-007 |
| REQ-008 | pacing; high | Each completed shift shall last from 60 through 90 seconds. | BCL-004 | **AC-008 — Shift duration.** Pass only if each deterministic full shift ends with elapsed gameplay time greater than or equal to 60.0 seconds and less than or equal to 90.0 seconds; otherwise fail. | VER-005 | CON-001 |
| REQ-009 | pacing; high | The shift shall become more chaotic over time. | BCL-004 | **AC-009 — Escalation.** Pass only if the final third of each seeded shift has a higher configured concurrent-threat budget than its first third and exposes at least two event types among table movement, pastry appearance, and food-steal pressure; otherwise fail. This operational definition is proposed under A-005. | VER-005 | DEC-009 |
| REQ-010 | gameplay; medium | Customers shall move tables during a shift. | BCL-004 | **AC-010 — Table movement.** Pass only if at least one customer changes from one table anchor to another during every full seeded shift and both anchors remain within the one-screen cafe; otherwise fail. | VER-005 | DEC-010 |
| REQ-011 | gameplay; medium | Pastries shall appear during a shift. | BCL-004 | **AC-011 — Pastry event.** Pass only if at least one pastry transitions from absent to visible and stealable during every full seeded shift; otherwise fail. | VER-005 | DEC-011 |
| REQ-012 | gameplay; medium | Impatient pigeons shall steal food and form combos. | BCL-004 | **AC-012 — Steal combo.** Pass only if an impatient state can lead to a food-steal event and two qualifying steals inside the configured combo window produce a combo count greater than one; otherwise fail. Thresholds and window values remain unresolved tuning. | VER-005 | DEC-012 |
| REQ-013 | experience; high | The tone shall be playful rather than stressful. | BCL-005 | **AC-013 — Tone.** Pass only if the stakeholder-approved evaluation defined by OQ-004 classifies the prototype as playful and does not classify it as stressful at the approved thresholds; otherwise fail or remain not evaluated. | VER-006 | CON-002 |
| REQ-014 | visual; medium | Characters shall be bold and readable. | BCL-005 | **AC-014 — Character readability.** Pass only if each active character silhouette is distinguishable from the background and each gameplay role is correctly identified at the supported portrait reference viewport under the stakeholder-approved inspection protocol; otherwise fail or remain not evaluated. | VER-006 | DEC-014 |
| REQ-015 | audio; medium | Gameplay shall use slapstick sound effects. | BCL-005 | **AC-015 — Slapstick audio.** Pass only if breadcrumb impact, pigeon steal, and combo events each trigger a distinct comic sound cue with audio enabled; otherwise fail. | VER-006 | DEC-015 |
| REQ-016 | platform; high | Play shall be presented in portrait mode. | BCL-005 | **AC-016 — Portrait orientation.** Pass only if the supported build's playable viewport has height greater than width and gameplay does not rotate into landscape; otherwise fail. | VER-006 | CON-003 |
| REQ-017 | scope; high | The first prototype shall contain one cafe gameplay screen. | BCL-006 | **AC-017 — One-screen scope.** Pass only if the runtime scene manifest contains exactly one playable cafe gameplay screen; overlays do not count as gameplay screens; otherwise fail. | VER-007 | CON-004 |
| REQ-018 | scope; high | The first prototype shall contain exactly three customer tables. | BCL-006 | **AC-018 — Table count.** Pass only if exactly three customer-table anchors are active in the gameplay screen; otherwise fail. | VER-007 | CON-005 |
| REQ-019 | scope; high | The first prototype shall use a small set of pigeon behaviours. | BCL-006 | **AC-019 — Behaviour scope gate.** Pass only if the registry contains breadcrumb attraction and impatient food-stealing/combo behaviour plus only those additional behaviours later approved through OQ-003; otherwise fail. Because “small” has no approved numeric limit, quantitative scope remains blocked. | VER-007 | CON-006 |
| REQ-020 | governance; high | The intended audience shall remain unresolved among children, casual adults, or both until the stakeholder selects one. | BCL-007; ANS-001 | **AC-020 — Audience authority.** Pass only if project records and product configuration mark audience as unresolved and do not claim any candidate is selected without a stakeholder answer to OQ-001; otherwise fail. | VER-008 | DEC-020 |
| REQ-021 | governance; high | The failure rule shall remain unresolved between ending a run and reducing wages while play continues until the stakeholder selects one. | BCL-007; ANS-002 | **AC-021 — Failure authority.** Pass only if project records and rule configuration mark the choice unresolved and do not claim either candidate is selected without a stakeholder answer to OQ-002; otherwise fail. | VER-009 | DEC-021 |
| REQ-022 | governance; high | The exact pigeon-behaviour set and its transitions shall remain unresolved beyond brief-mandated behaviour until the stakeholder approves them. | ANS-003 | **AC-022 — Behaviour authority.** Pass only if mandatory behaviours are present and every additional behaviour is either absent or linked to an approved OQ-003 answer; otherwise fail. | VER-010 | DEC-019 |
| REQ-023 | governance; high | Prototype success criteria shall remain unset until the stakeholder approves learning objectives, measures, and pass/fail thresholds. | ANS-004 | **AC-023 — Success authority.** Pass only if no success declaration exists before approved criteria and conforming recorded results exist; otherwise fail. | VER-011 | DEC-022 |

## Interpretation rules

- “Shall” is normative. Proposed operational definitions do not convert tuning values or product choices into stakeholder decisions.
- A blocked verification yields **not evaluated**, not pass.
- OQ references resolve in [OpenQuestions.md](OpenQuestions.md); assumption references resolve in [Assumptions.md](Assumptions.md); decisions and constraints resolve in [Traceability.md](Traceability.md).
