# Half-Mech Heroes — Canonical Requirement Register

Status: initiation baseline  
Authority order: stakeholder brief > later stakeholder decisions > explicitly approved assumptions > research guidance > design inference.  
Rule: this file is the only canonical register of requirements. `REQ-*` records are generated project requirements; `SRC-*` records preserve exact stakeholder wording.

## Authoritative brief (verbatim)

> Half-Mech Heroes
>
> I want a local two-player couch arcade game where both players operate the same malfunctioning rescue mech. One player controls movement and jumping with the left side of a gamepad while the other aims a magnetic arm and shield with the right side; the roles automatically swap whenever the mech takes damage. Together they cross a single scrolling disaster zone, catch falling civilians, throw wreckage out of the way, and build a teamwork multiplier before time runs out. Sessions should consist of short three-minute rescue attempts with quick restarts, exaggerated physics, chunky silhouettes, and funny mechanical failures that remain readable on a television. The first prototype should support two controllers or one shared keyboard and include one city block, one civilian type, and three hazards. I want coordination to create both mastery and comedy, but input remapping, solo play, difficulty scaling, and the exact target age are still open questions.

## Material source-clause coverage

Each quote below is exact source wording. Each row maps to at least one generated requirement; the detailed semantic edge appears separately in [Traceability.md](Traceability.md).

| Source ID | Exact source wording | Generated requirement |
|---|---|---|
| SRC-001 | “Half-Mech Heroes” | REQ-001 |
| SRC-002 | “I want a local two-player couch arcade game” | REQ-002 |
| SRC-003 | “where both players operate the same malfunctioning rescue mech.” | REQ-003 |
| SRC-004 | “One player controls movement and jumping with the left side of a gamepad” | REQ-004 |
| SRC-005 | “while the other aims a magnetic arm and shield with the right side” | REQ-005 |
| SRC-006 | “the roles automatically swap whenever the mech takes damage.” | REQ-006 |
| SRC-007 | “Together they cross a single scrolling disaster zone” | REQ-007 |
| SRC-008 | “catch falling civilians” | REQ-008 |
| SRC-009 | “throw wreckage out of the way” | REQ-009 |
| SRC-010 | “build a teamwork multiplier before time runs out.” | REQ-010 |
| SRC-011 | “Sessions should consist of short three-minute rescue attempts” | REQ-011 |
| SRC-012 | “with quick restarts” | REQ-012 |
| SRC-013 | “exaggerated physics” | REQ-013 |
| SRC-014 | “chunky silhouettes” | REQ-014 |
| SRC-015 | “funny mechanical failures that remain readable on a television.” | REQ-015 |
| SRC-015 | “funny mechanical failures that remain readable on a television.” | REQ-016 |
| SRC-016 | “The first prototype should support two controllers or one shared keyboard” | REQ-017 |
| SRC-017 | “include one city block” | REQ-018 |
| SRC-018 | “one civilian type” | REQ-019 |
| SRC-019 | “three hazards.” | REQ-020 |
| SRC-020 | “I want coordination to create both mastery and comedy” | REQ-021 |
| SRC-020 | “I want coordination to create both mastery and comedy” | REQ-022 |
| SRC-020 | “I want coordination to create both mastery and comedy” | REQ-023 |
| SRC-021 | “but input remapping, solo play, difficulty scaling, and the exact target age are still open questions.” | REQ-024 |

## Functional and content requirements

### REQ-001 — Project identity

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-001
- Requirement: The project and prototype title shall be **Half-Mech Heroes**.
- Rationale: preserves the declared identity.
- Objective pass/fail acceptance criterion: **Pass** only if the launch screen and package metadata display the exact case-sensitive string `Half-Mech Heroes`; otherwise **fail**.
- Exact verification: `TST-REQ-001`, static assertion against launch-title and package-title fields.
- Consequential decision: DEC-001.

### REQ-002 — Local two-player couch arcade mode

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-002
- Requirement: The playable prototype shall provide a simultaneous local two-player couch arcade session.
- Rationale: defines the mandatory player mode and session context.
- Objective pass/fail acceptance criterion: **Pass** only if two local input assignments can start and complete one attempt together on one host without a network connection; otherwise **fail**.
- Exact verification: `TST-REQ-002`, offline two-player end-to-end attempt.
- Consequential decision: DEC-002.

### REQ-003 — Shared mech

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-003
- Requirement: Both players shall operate the same malfunctioning rescue-mech entity.
- Rationale: shared embodiment is the core cooperative constraint.
- Objective pass/fail acceptance criterion: **Pass** only if telemetry for a complete attempt contains one player-controlled mech entity and accepted actions from both player assignments target that same entity; otherwise **fail**.
- Exact verification: `TST-REQ-003`, shared-entity integration test.
- Consequential decision: DEC-003.

### REQ-004 — Movement-side role

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-004
- Requirement: The movement role shall control mech movement and jumping through the left-side gamepad control set.
- Rationale: establishes one half of the asymmetric control contract.
- Objective pass/fail acceptance criterion: **Pass** only if the movement-role input fixture moves left, moves right, and jumps via configured left-side gamepad inputs while magnetic-arm aim and shield inputs from that assignment have no gameplay effect; otherwise **fail**.
- Exact verification: `TST-REQ-004`, movement-role input-isolation test.
- Consequential decision: DEC-004.

### REQ-005 — Tool-side role

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-005
- Requirement: The tool role shall aim the magnetic arm and shield through the right-side gamepad control set.
- Rationale: establishes the other half of the asymmetric control contract.
- Objective pass/fail acceptance criterion: **Pass** only if the tool-role fixture independently changes magnetic-arm aim and shield aim through configured right-side gamepad inputs while movement and jump inputs from that assignment have no gameplay effect; otherwise **fail**.
- Exact verification: `TST-REQ-005`, tool-role input-isolation test.
- Consequential decision: DEC-004.

### REQ-006 — Damage-triggered role swap

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-006
- Requirement: Whenever the mech transitions from undamaged to damaged by a valid damage event, the two player-role assignments shall swap automatically once.
- Rationale: creates forced adaptation and the intended malfunction.
- Objective pass/fail acceptance criterion: **Pass** only if each of three isolated valid damage events causes exactly one assignment swap before the next gameplay input is processed, and non-damaging contacts cause none; otherwise **fail**.
- Exact verification: `TST-REQ-006`, damage-event role-state transition test.
- Consequential decision: DEC-005.

### REQ-007 — Single scrolling disaster zone

- Status: accepted; physical boundary unresolved
- Priority: required
- Authority and source: stakeholder brief, SRC-007
- Requirement: Each attempt shall take place in one continuously scrolling disaster zone.
- Rationale: defines the spatial run structure.
- Objective pass/fail acceptance criterion: **Pass** only if an attempt loads exactly one gameplay-zone instance, advances the camera through that instance, and performs no zone-selection or zone-loading transition before the result state; otherwise **fail**.
- Exact verification: `TST-REQ-007`, zone-lifecycle integration test.
- Consequential decision: DEC-006. Boundary constraint: DEC-024 and OQ-009.

### REQ-008 — Catch falling civilians

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-008
- Requirement: Players shall be able to catch a falling civilian before impact and produce a rescued-civilian outcome.
- Rationale: defines the principal rescue action.
- Objective pass/fail acceptance criterion: **Pass** only if the fixed falling-civilian fixture can be intercepted by the mech or magnetic-arm catch volume and then emits one `civilian_rescued` event without emitting `civilian_impact`; otherwise **fail**.
- Exact verification: `TST-REQ-008`, civilian-catch scenario test.
- Consequential decision: DEC-007.

### REQ-009 — Throw wreckage

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-009
- Requirement: Players shall be able to acquire movable wreckage with the magnetic arm and throw it clear of the traversable route.
- Rationale: defines the obstacle-clearing action.
- Objective pass/fail acceptance criterion: **Pass** only if the fixed wreckage fixture can be acquired, released with throw impulse, and moved fully outside the marked route-clearance volume; otherwise **fail**.
- Exact verification: `TST-REQ-009`, wreckage-clearance scenario test.
- Consequential decision: DEC-008.

### REQ-010 — Teamwork multiplier

- Status: accepted; tuning unresolved
- Priority: required
- Authority and source: stakeholder brief, SRC-010
- Requirement: Successful coordinated rescue and clearance actions shall build a visible teamwork multiplier before the attempt timer expires.
- Rationale: rewards cooperation inside the timed loop.
- Objective pass/fail acceptance criterion: **Pass** only if the deterministic sequence `catch civilian → clear wreckage` increases the displayed multiplier above its attempt-start value and the multiplier no longer changes after timeout; otherwise **fail**.
- Exact verification: `TST-REQ-010`, teamwork-multiplier state test.
- Consequential decision: DEC-009.

### REQ-011 — Three-minute attempt

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-011
- Requirement: A rescue attempt shall provide exactly 180 seconds of active gameplay time.
- Rationale: makes sessions short and repeatable.
- Objective pass/fail acceptance criterion: **Pass** only if a fixed-clock test advances from `180.000` seconds remaining to the result transition at `0.000`, with pause time excluded and tolerance no greater than one simulation tick; otherwise **fail**.
- Exact verification: `TST-REQ-011`, fixed-clock attempt-duration test.
- Consequential decision: DEC-010.

### REQ-012 — Quick restart

- Status: accepted intent; provisional threshold pending OQ-010E
- Priority: required
- Authority and source: stakeholder brief, SRC-012
- Requirement: From the result state, both players shall be able to restart the same attempt without returning to a title screen or relaunching the application.
- Rationale: preserves rapid replay. The quantitative meaning of “quick” is not stakeholder-approved.
- Objective pass/fail acceptance criterion: **Provisional pass** if a restart input produces controllable gameplay in no more than 5.0 seconds on the reference test device, resets timer, score, multiplier, roles, entities, and deterministic seed, and does not reload the application; otherwise **fail**. The 5.0-second gate is non-authoritative ASM-002 and cannot close OQ-010E.
- Exact verification: `TST-REQ-012`, result-to-control stopwatch and clean-state test.
- Consequential decision: DEC-034.

### REQ-013 — Exaggerated physics

- Status: accepted intent; provisional observable pending tuning approval
- Priority: required
- Authority and source: stakeholder brief, SRC-013
- Requirement: Core mech, civilian, and wreckage interactions shall visibly use exaggerated physics.
- Rationale: supports arcade feel and comic motion.
- Objective pass/fail acceptance criterion: **Provisional pass** if the standard throw fixture propels wreckage at least one mech-height horizontally before first ground contact and the standard damage fixture produces a visible mech recoil of at least one-quarter mech-height without losing player control for more than 1.0 second; otherwise **fail**. These bounds are non-authoritative ASM-003.
- Exact verification: `TST-REQ-013`, calibrated throw-and-recoil capture.
- Consequential decision: DEC-036.

### REQ-014 — Chunky silhouettes

- Status: accepted intent; provisional evaluation pending audience approval
- Priority: required
- Authority and source: stakeholder brief, SRC-014
- Requirement: The mech, civilian, wreckage, and each hazard shall use visually distinct chunky silhouettes.
- Rationale: enables rapid television-distance recognition.
- Objective pass/fail acceptance criterion: **Provisional pass** if at least four of five evaluators correctly identify every entity class from randomized, label-free, single-colour 1080p silhouette captures at the assumed couch setup; otherwise **fail**. The setup and cohort are non-authoritative ASM-004.
- Exact verification: `TST-REQ-014`, blinded silhouette-recognition protocol.
- Consequential decision: DEC-037.

### REQ-015 — Funny mechanical failures

- Status: accepted intent; provisional evaluation pending OQ-010B
- Priority: required
- Authority and source: stakeholder brief, SRC-015
- Requirement: Gameplay shall produce mechanical-failure events intended to create comedy while preserving the rescue loop.
- Rationale: comedy is a declared outcome rather than decorative flavour.
- Objective pass/fail acceptance criterion: **Provisional pass** if the scripted damage fixture exposes at least three mechanically distinct failure presentations, none blocks completion, and at least four of five paired-playtest participants mark at least one observed failure as funny on the defined binary prompt; otherwise **fail**. Counts and cohort are non-authoritative ASM-005.
- Exact verification: `TST-REQ-015`, failure-catalogue inspection plus paired-playtest prompt.
- Consequential decision: DEC-038.

### REQ-016 — Television readability

- Status: accepted intent; provisional test context pending OQ-010D
- Priority: required
- Authority and source: stakeholder brief, SRC-015
- Requirement: Critical gameplay state, entities, prompts, timer, multiplier, and role assignment shall remain readable on a television.
- Rationale: couch-distance legibility is essential to play.
- Objective pass/fail acceptance criterion: **Provisional pass** if, at 1920×1080 on the assumed 55-inch display at 3 metres, all critical text has at least 26-pixel body height and at least four of five evaluators correctly report timer, multiplier, current role, civilian state, and active hazard cue during the test script; otherwise **fail**. Display, distance, and cohort are non-authoritative ASM-001 and ASM-006.
- Exact verification: `TST-REQ-016`, pixel audit and couch-distance recognition protocol.
- Consequential decision: DEC-035. Research constraint: EVD-EXP-001.

### REQ-017 — Prototype input configurations

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-016
- Requirement: The first prototype shall be fully playable with either two controllers or one shared keyboard.
- Rationale: establishes the two mandatory local hardware configurations.
- Objective pass/fail acceptance criterion: **Pass** only if `TST-REQ-002` completes once with two controllers and once with one keyboard whose disjoint key groups are assigned to the two roles; otherwise **fail**.
- Exact verification: `TST-REQ-017A`, dual-controller full attempt; `TST-REQ-017B`, shared-keyboard full attempt. These are separate test executions, not an ID range.
- Consequential decision: DEC-016.

### REQ-018 — One city block

- Status: accepted; relationship to zone boundary unresolved
- Priority: required
- Authority and source: stakeholder brief, SRC-017
- Requirement: The first prototype shall contain exactly one city-block content unit.
- Rationale: bounds the prototype’s authored environment count.
- Objective pass/fail acceptance criterion: **Pass** only if the content manifest contains exactly one entry of type `city_block` and the attempt references that entry; otherwise **fail**.
- Exact verification: `TST-REQ-018`, content-manifest assertion.
- Consequential decision: DEC-017. Boundary constraint: DEC-024 and OQ-009.

### REQ-019 — One civilian type

- Status: accepted; identity unresolved
- Priority: required
- Authority and source: stakeholder brief, SRC-018
- Requirement: The first prototype shall contain exactly one civilian gameplay type.
- Rationale: bounds rescue-content variety without selecting its creative identity.
- Objective pass/fail acceptance criterion: **Pass** only if the content manifest contains exactly one civilian gameplay archetype and every spawned civilian references it; otherwise **fail**.
- Exact verification: `TST-REQ-019`, civilian-archetype manifest and spawn audit.
- Consequential decision: DEC-018. Identity constraint: DEC-025 and OQ-008.

### REQ-020 — Three hazards

- Status: accepted; identities unresolved
- Priority: required
- Authority and source: stakeholder brief, SRC-019
- Requirement: The first prototype shall contain exactly three mechanically distinct hazard types.
- Rationale: fixes variety count while leaving creative selection open.
- Objective pass/fail acceptance criterion: **Pass** only if the content manifest contains exactly three hazard archetypes and an automated fixture demonstrates a different trigger-and-consequence signature for each; otherwise **fail**.
- Exact verification: `TST-REQ-020`, hazard-manifest count and signature test.
- Consequential decision: DEC-019. Identity constraint: DEC-026 and OQ-007.

## Experience-outcome requirements

### REQ-021 — Coordination

- Status: accepted intent; provisional measurement pending OQ-010C
- Priority: important
- Authority and source: stakeholder brief, SRC-020
- Requirement: Success shall depend on coordination between the movement role and tool role.
- Rationale: cooperation must affect outcomes, not merely coexist.
- Objective pass/fail acceptance criterion: **Provisional pass** if, across the fixed paired-playtest script, the median rescued-civilian count in coordinated trials is greater than the median in deliberately uncoordinated control trials; otherwise **fail**. Protocol and sample size are non-authoritative ASM-007.
- Exact verification: `TST-REQ-021`, paired coordinated-versus-control scenario.
- Consequential decision: DEC-040.

### REQ-022 — Mastery

- Status: accepted intent; provisional measurement pending OQ-010A
- Priority: important
- Authority and source: stakeholder brief, SRC-020
- Requirement: Repeated coordinated play shall permit observable mastery.
- Rationale: establishes learnable skill growth as a product outcome.
- Objective pass/fail acceptance criterion: **Provisional pass** if at least four of five test pairs improve either rescued-civilian count or peak teamwork multiplier from their first valid attempt to their fifth valid attempt without a difficulty reduction; otherwise **fail**. Cohort and threshold are non-authoritative ASM-008.
- Exact verification: `TST-REQ-022`, five-attempt paired-playtest progression protocol.
- Consequential decision: DEC-039.

### REQ-023 — Comedy

- Status: accepted intent; provisional measurement pending OQ-010B
- Priority: important
- Authority and source: stakeholder brief, SRC-020
- Requirement: Coordinated play and malfunctions shall create observable comedy for the players.
- Rationale: comedy is co-equal with mastery in the stated experience goal.
- Objective pass/fail acceptance criterion: **Provisional pass** if at least four of five test pairs independently record at least one laughter or amusement event during a valid attempt and can identify the triggering gameplay event immediately afterward; otherwise **fail**. Cohort and coding rule are non-authoritative ASM-005.
- Exact verification: `TST-REQ-023`, observed-amusement event protocol.
- Consequential decision: DEC-038.

## Governance requirement

### REQ-024 — Protect stakeholder-open choices

- Status: accepted
- Priority: required
- Authority and source: stakeholder brief, SRC-021
- Requirement: Input-remapping scope, solo-play scope, difficulty-scaling scope, and exact target age shall remain explicitly unresolved until a stakeholder supplies a decision; research shall not be treated as authority to settle them.
- Rationale: preserves the brief’s declared uncertainty and authority boundary.
- Objective pass/fail acceptance criterion: **Pass** only if each subject is `Open` in [OpenQuestions.md](OpenQuestions.md), no accepted decision or unconditional implementation requirement selects an answer, and any temporary treatment is labelled non-authoritative; otherwise **fail**.
- Exact verification: `TST-REQ-024`, open-choice register and decision-status audit.
- Consequential decision: DEC-023.

## Imported-record disposition

The snapshot’s imported requirement-shaped records `REQ-001`, `REQ-002`, `REQ-003`, `REQ-004`, `REQ-005`, `REQ-006`, `REQ-007`, `REQ-008`, `REQ-009`, `REQ-010`, and `REQ-011` repeat research answers beginning with “FAIL” as though they were implementation requirements. They are not adopted into this canonical register because they express unresolved authority boundaries, not desired system behaviour. Their useful content is preserved in the matching `OQ-*` records and in DEC-024, DEC-025, DEC-026, DEC-027, DEC-028, DEC-029, DEC-030, DEC-031, DEC-032, and DEC-033. This correction is a documentation interpretation, not a stakeholder decision.
