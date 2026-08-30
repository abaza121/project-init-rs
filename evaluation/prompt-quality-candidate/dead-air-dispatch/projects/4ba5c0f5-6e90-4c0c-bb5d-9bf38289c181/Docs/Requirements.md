# Dead Air Dispatch — Canonical Requirements Register

Status: initiation baseline candidate  
Canonicality: this file is the only canonical requirement register. Other files link here rather than restating requirements as authority.  
Authority rule: quoted source clauses are authoritative; normalized requirements preserve intent but do not add stakeholder authority.

## Verbatim source register

The following material brief clauses are retained exactly as supplied.

### SRC-001 — Project name

> Dead Air Dispatch

### SRC-002 — Product premise

> I want to make a strange 2D score-attack game for PC in which the player operates an emergency radio during a supernatural storm.

### SRC-003 — Tuning, revelation, and threat response

> The arena is invisible until the player turns a tuning dial with the mouse wheel; different frequencies reveal different hazards, stranded callers, and safe routes, but tuning also changes which entities can hear and approach the station.

### SRC-004 — Repeating loop

> The repeating loop is to scan, identify a caller's frequency, hold the signal long enough to guide them home, then retune before hostile interference reaches the antenna.

### SRC-005 — Run duration and score

> A run should last about five minutes and score the player for rescues, signal accuracy, and risky time spent near dangerous bands.

### SRC-006 — First-playable production scope

> The first playable can use abstract waveforms, silhouettes, static, and layered sound in a single room.

### SRC-007 — Central risk and accessibility unknown

> The central risk is making hidden information feel intriguing rather than unfair, and I have not decided how to support deaf or hard-of-hearing players.

## Requirement records

Each record has one objective pass/fail acceptance criterion and one exact verification reference. “Blocked” means the requirement is valid but cannot be accepted until the named stakeholder choice is authorized.

### DAD-REQ-001 — Product identity

- Statement: The product shall be identified as **Dead Air Dispatch**.
- Type: identity
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-001`
- Decision or constraint: `D-001`
- Acceptance criterion `AC-001` (pass/fail): Pass if every title-bearing package document uses “Dead Air Dispatch” and no alternate product name is presented as canonical; otherwise fail.
- Verification: `VER-DOC-001`.

### DAD-REQ-002 — 2D score-attack form

- Statement: The game shall provide a strange 2D score-attack play experience.
- Type: product
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-002`
- Decision or constraint: `D-002`; `C-001`
- Acceptance criterion `AC-002` (pass/fail): Pass if a playable run renders gameplay in a 2D presentation and produces a run score affected by the score inputs in `DAD-REQ-012`; otherwise fail.
- Verification: `VER-PLAY-001`.

### DAD-REQ-003 — PC target

- Statement: The game shall run on PC.
- Type: platform
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-002`
- Decision or constraint: `C-002`
- Acceptance criterion `AC-003` (pass/fail): Pass if the candidate build launches to its playable state and completes one run on the stakeholder-approved PC operating-system and hardware baseline; fail if it does not. The baseline is unresolved in `OQ-009`, so final acceptance is blocked until that baseline is approved.
- Verification: `VER-BUILD-001`.

### DAD-REQ-004 — Operator fantasy

- Statement: During a supernatural storm, the player shall operate an emergency radio as the primary interaction premise.
- Type: experience
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-002`
- Decision or constraint: `D-002`; `C-001`
- Acceptance criterion `AC-004` (pass/fail): Pass if a run exposes radio-tuning interaction and communicates the supernatural-storm context in the playable presentation; otherwise fail.
- Verification: `VER-PLAY-002`.

### DAD-REQ-005 — Hidden arena and mouse-wheel tuning

- Statement: The arena shall remain unrevealed until the player changes the tuning dial with the mouse wheel.
- Type: functional
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-003`
- Decision or constraint: `D-002`; `C-003`
- Acceptance criterion `AC-005` (pass/fail): Pass if, from a fresh run, the arena is not revealed before any tuning input and at least one mouse-wheel input changes the dial and causes a frequency-dependent reveal; otherwise fail.
- Verification: `VER-PLAY-003`.

### DAD-REQ-006 — Frequency-dependent information

- Statement: Different frequencies shall reveal different hazards, stranded callers, and safe routes.
- Type: functional
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-003`
- Decision or constraint: `D-002`; `C-003`
- Acceptance criterion `AC-006` (pass/fail): Pass if a controlled test visits at least two configured frequencies and the observed reveal set differs, with the complete test fixture containing at least one hazard, one stranded caller, and one safe route; otherwise fail.
- Verification: `VER-SIM-001`.

### DAD-REQ-007 — Frequency-dependent entity response

- Statement: Tuning shall change which entities can hear and approach the station.
- Type: functional
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-003`
- Decision or constraint: `D-002`; `C-003`
- Acceptance criterion `AC-007` (pass/fail): Pass if the same deterministic entity fixture is simulated on two configured frequencies and at least one entity’s can-hear state and approach behavior differ according to configuration; otherwise fail.
- Verification: `VER-SIM-002`.

### DAD-REQ-008 — Scan phase

- Statement: Each repeating rescue loop shall permit the player to scan frequencies.
- Type: functional
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-004`
- Decision or constraint: `D-002`; `C-004`
- Acceptance criterion `AC-008` (pass/fail): Pass if a run can enter a scan state and the player can traverse multiple frequencies before locking a caller; otherwise fail.
- Verification: `VER-LOOP-001`.

### DAD-REQ-009 — Caller-frequency identification

- Statement: The rescue loop shall require identification of a stranded caller’s frequency.
- Type: functional
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-004`
- Decision or constraint: `D-002`; `C-004`
- Acceptance criterion `AC-009` (pass/fail): Pass if the deterministic loop fixture contains a caller with a configured target frequency and rescue progress cannot begin while tuned outside its acquisition condition; otherwise fail.
- Verification: `VER-LOOP-002`.

### DAD-REQ-010 — Signal hold and guidance

- Statement: The rescue loop shall require the player to hold the caller’s signal long enough to guide the caller home.
- Type: functional
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-004`
- Decision or constraint: `D-002`; `C-004`
- Acceptance criterion `AC-010` (pass/fail): Pass if continuous valid signal hold accumulates configured guidance progress to a rescued state, while an interrupted or invalid hold does not complete that rescue; otherwise fail.
- Verification: `VER-LOOP-003`.

### DAD-REQ-011 — Retune under hostile pressure

- Statement: After a guidance attempt, the loop shall permit the player to retune before hostile interference reaches the antenna.
- Type: functional
- Priority: high
- Status: active
- Provenance: stakeholder brief
- Source: `SRC-004`
- Decision or constraint: `D-002`; `C-004`
- Acceptance criterion `AC-011` (pass/fail): Pass if a hostile-interference fixture advances toward an antenna outcome while its hearing condition is active and a timely retune can change that condition before contact; otherwise fail.
- Verification: `VER-LOOP-004`.

### DAD-REQ-012 — Three score inputs

- Statement: The run score shall include separately inspectable contributions from rescues, signal accuracy, and risky time spent near dangerous bands.
- Type: functional
- Priority: high
- Status: active; total-balance acceptance blocked
- Provenance: stakeholder brief plus unresolved authority boundary
- Source: `SRC-005`; `ANS-003`
- Decision or constraint: `D-009`; `C-005`
- Acceptance criterion `AC-012` (pass/fail): Pass the structural portion if a deterministic scoring fixture can vary each of the three inputs independently and each produces a separately reported score contribution. Final pass additionally requires the stakeholder-approved weights from `OQ-003`; without them, final acceptance is blocked rather than inferred.
- Verification: `VER-SCORE-001`.

### DAD-REQ-013 — Approximately five-minute run

- Statement: A run shall target approximately five minutes.
- Type: performance/experience
- Priority: high
- Status: active; tolerance acceptance blocked
- Provenance: stakeholder brief plus unresolved authority boundary
- Source: `SRC-005`; `ANS-005`
- Decision or constraint: `D-011`; `D-013`; `C-006`
- Acceptance criterion `AC-013` (pass/fail): Pass the nominal-target portion if the run-duration target is configured as 300 seconds. Final pass requires measured runs to fall within stakeholder-approved minimum and maximum bounds from `OQ-005`; until those bounds are approved, final acceptance is blocked.
- Verification: `VER-TIME-001`.

### DAD-REQ-014 — First-playable presentation permission

- Statement: The first playable may use abstract waveforms, silhouettes, static, layered sound, and a single room; none of those permitted treatments is converted here into a mandatory final-art commitment.
- Type: scope permission
- Priority: medium
- Status: authorized option
- Provenance: stakeholder brief
- Source: `SRC-006`
- Decision or constraint: `D-003`; `C-007`
- Acceptance criterion `AC-014` (pass/fail): Pass if the first-playable scope review treats each listed treatment as permitted and does not reject the build solely because it uses any listed treatment; fail if the permission is contradicted or represented as a mandatory final-product style.
- Verification: `VER-SCOPE-001`.

### DAD-REQ-015 — Hidden-information acceptance definition

- Statement: Before hidden-information fairness is declared accepted, stakeholders shall approve an intended player population, evaluation measures, and pass/fail threshold for “intriguing rather than unfair.”
- Type: validation governance
- Priority: high
- Status: blocked by `OQ-002` and `OQ-004`
- Provenance: stakeholder risk plus research-supported authority boundary
- Source: `SRC-007`; `ANS-002`
- Evidence: `EVD-004`; `EVD-005`; `EVD-006`
- Decision or constraint: `D-008`; `C-008`
- Acceptance criterion `AC-015` (pass/fail): Pass only if an approved decision record names the player population, measures, numeric or categorical pass/fail threshold, and approver; otherwise fail. No universal threshold is inferred.
- Verification: `VER-GOV-001`.

### DAD-REQ-016 — Deaf and hard-of-hearing support decision

- Statement: The project shall not claim a settled deaf or hard-of-hearing support scope until stakeholders approve the selected features and acceptance checks.
- Type: accessibility governance
- Priority: high
- Status: blocked by `OQ-001`
- Provenance: stakeholder unknown plus research-supported options
- Source: `SRC-007`; `ANS-001`
- Evidence: `EVD-001`; `EVD-002`; `EVD-003`
- Decision or constraint: `D-007`; `D-015`; `C-009`
- Acceptance criterion `AC-016` (pass/fail): Pass if all project records label support features as open or recommended until an approved `OQ-001` decision names adopted features and exact verification checks; fail if any unapproved feature is presented as committed scope.
- Verification: `VER-GOV-002`.

### DAD-REQ-017 — Intended audience decision

- Statement: The project shall not present an age, skill, demographic, or accessibility population as the intended audience until stakeholders define it.
- Type: positioning governance
- Priority: medium
- Status: blocked by `OQ-002`
- Provenance: research-supported authority boundary
- Source: `ANS-004`
- Evidence: `EVD-008`; `EVD-009`
- Decision or constraint: `D-010`; `C-010`
- Acceptance criterion `AC-017` (pass/fail): Pass if project records either identify an approved audience decision or explicitly label the audience as open; fail if they infer audience from platform, genre, mechanics, or an age rating.
- Verification: `VER-GOV-003`.

### DAD-REQ-018 — Exclusion-list governance

- Statement: The project shall not assert stakeholder-authorized exclusions until interested parties agree them.
- Type: scope governance
- Priority: medium
- Status: blocked by `OQ-006`
- Provenance: research-supported authority boundary
- Source: `ANS-006`
- Evidence: `EVD-011`; `EVD-012`
- Decision or constraint: `D-012`; `C-011`
- Acceptance criterion `AC-018` (pass/fail): Pass if exclusions are either explicitly marked “none authorized” or recorded with approver and approval date; fail if inferred exclusions are presented as authorized.
- Verification: `VER-GOV-004`.

## Verification catalog

| Verification ID | Exact procedure | Produced record |
|---|---|---|
| `VER-DOC-001` | Search all required Markdown files for title headings and inspect every canonical-name statement; compare character-for-character with `SRC-001`. | Documentation audit output |
| `VER-PLAY-001` | Complete one instrumented run; record render mode and the three score-component outputs. | Playtest capture plus score log |
| `VER-BUILD-001` | On the approved PC baseline, launch a clean candidate build, begin a run, and reach its terminal state without a platform-blocking defect. | Build smoke-test report |
| `VER-PLAY-002` | Observe one run and check that radio tuning is the primary interaction and the storm is communicated in presentation. | Playtest checklist |
| `VER-PLAY-003` | Start a fresh run without wheel input, capture reveal state, send one wheel event, then capture dial value and reveal state. | Input/reveal event log |
| `VER-SIM-001` | Run the fixed reveal fixture at frequencies F-A and F-B; compare entity IDs and confirm fixture coverage for hazard, caller, and route types. | Deterministic simulation log |
| `VER-SIM-002` | Run the fixed hearing fixture twice with identical seed and different frequencies; compare can-hear and approach-state transitions. | Deterministic simulation log |
| `VER-LOOP-001` | From run start, enter scan and visit at least two frequencies before caller lock. | State-transition log |
| `VER-LOOP-002` | Attempt guidance below, at, and above the fixture caller’s acquisition band; inspect progress. | Caller-acquisition test log |
| `VER-LOOP-003` | Hold a valid signal through its configured guidance duration, then repeat with an interruption; compare terminal caller states. | Guidance test log |
| `VER-LOOP-004` | Activate hostile approach, run once without retuning and once with a retune before contact, then compare antenna-contact outcomes. | Threat-response test log |
| `VER-SCORE-001` | Execute four deterministic fixtures: zero inputs, rescue-only, accuracy-only, and risk-time-only; inspect named contribution fields and apply approved weights when available. | Scoring unit-test report |
| `VER-TIME-001` | Inspect nominal duration configuration for 300 seconds; after bounds are approved, time the approved run sample and compare each result with both bounds. | Timing report |
| `VER-SCOPE-001` | Review first-playable acceptance notes for treatment of every item in `SRC-006` as permission rather than mandatory final scope. | Scope review checklist |
| `VER-GOV-001` | Inspect the approved hidden-information decision for population, measures, threshold, approver, and date. | Decision review |
| `VER-GOV-002` | Search project records for captions, visual cues, volume controls, mono, and muted-audio claims; verify each is open/recommended or explicitly approved with tests. | Accessibility authority audit |
| `VER-GOV-003` | Search audience statements and compare them with the approved audience decision, if any. | Audience authority audit |
| `VER-GOV-004` | Inspect every exclusion statement for “none authorized” or explicit approval metadata. | Scope authority audit |

## Imported-register reconciliation

The snapshot’s imported `REQ-001` through `REQ-006` are not used as canonical IDs because their wording incorrectly frames unresolved answers as “clarified project choices.” Their valid authority-boundary content is retained as follows:

| Imported ID | Imported source | Canonical requirement | Reconciliation |
|---|---|---|---|
| `REQ-001` | `ANS-001` | `DAD-REQ-016` | Retains that deaf and hard-of-hearing scope is undecided; does not convert recommendations into scope. |
| `REQ-002` | `ANS-002` | `DAD-REQ-015` | Retains that no universal fairness threshold exists and approval is pending. |
| `REQ-003` | `ANS-003` | `DAD-REQ-012` | Retains all three score inputs and leaves relative weights open. |
| `REQ-004` | `ANS-004` | `DAD-REQ-017` | Retains that intended audience is not authorized. |
| `REQ-005` | `ANS-005` | `DAD-REQ-013` | Retains the five-minute target and leaves tolerance open. |
| `REQ-006` | `ANS-006` | `DAD-REQ-018` | Retains that no exclusions are authorized. |

