# Requirements

This is the canonical requirement register. Source clauses preserve the exact brief wording; requirements below are generated, testable interpretations. When an acceptance threshold is inferred, the criterion cites the canonical assumption and remains provisional.

## Verbatim source-clause register

| Source ID | Exact source wording | Generated requirement(s) |
|---|---|---|
| SC-001 | “I want a minimalist one-button browser game” | R-001, R-002 |
| SC-002 | “where a small satellite survives by borrowing gravity from objects moving across the screen.” | R-003 |
| SC-003 | “Holding the button attaches the satellite to the nearest planet or piece of debris;” | R-004 |
| SC-004 | “releasing it preserves momentum and sends the player into a new arc.” | R-005 |
| SC-005 | “The goal is to collect data fragments,” | R-006 |
| SC-006 | “skim hazards for bonus points,” | R-007 |
| SC-007 | “and keep an orbit multiplier alive” | R-008 |
| SC-008 | “while the available objects drift toward a collapsing star.” | R-009 |
| SC-009 | “Runs should take less than three minutes,” | R-010 |
| SC-010 | “restart instantly,” | R-011 |
| SC-011 | “and be understandable from animation and sound instead of text.” | R-012 |
| SC-012 | “The visual style should be sparse geometric space art” | R-013 |
| SC-013 | “that a solo developer could produce.” | R-014 |
| SC-014 | “The prototype should prove that switching gravity anchors creates skillful, predictable movement” | R-015 |
| SC-015 | “on one screen.” | R-016 |
| SC-016 | “I have not chosen the exact physics model,” | R-017 |
| SC-017 | “input accessibility features,” | R-018 |
| SC-018 | “target frame rate on low-end devices,” | R-019, R-020 |
| SC-019 | “or how much randomness each run should contain.” | R-021 |

## Requirement register

### R-001 — Browser delivery

- Priority/status: high / active.
- Requirement: The prototype shall run as an interactive browser game without requiring a native application install.
- Authority: SC-001; user brief.
- Relevant decision/constraint: D-001.
- Objective acceptance criterion (`AC-R-001`): **Pass** when the production build opens from a clean browser profile through its documented local or hosted URL and reaches interactive play without installing a native executable, extension, or plugin; otherwise **fail**.
- Exact verification: `T-001` in the verification catalogue.

### R-002 — Minimal one-button gameplay

- Priority/status: high / active.
- Requirement: During a run, all intentional satellite-control state changes shall use one gameplay action with pressed/held and released states; presentation shall remain minimalist.
- Authority: SC-001; user brief.
- Relevant decision/constraint: D-002.
- Objective acceptance criterion (`AC-R-002`): **Pass** when an input-event trace for a complete scripted run contains only `ACTION_DOWN` and `ACTION_UP` as gameplay control commands and the visual inventory contains no nonessential gameplay control; otherwise **fail**.
- Exact verification: `T-002`.

### R-003 — Moving-object gravity survival

- Priority/status: high / active.
- Requirement: The controllable satellite shall survive through gravity interactions with eligible planets or debris that move across the playfield.
- Authority: SC-002; user brief.
- Relevant decision/constraint: D-003.
- Objective acceptance criterion (`AC-R-003`): **Pass** when the scripted fixture logs at least one eligible anchor changing screen position, the satellite receiving anchor-derived acceleration while attached, and survival state continuing after that interaction; otherwise **fail**.
- Exact verification: `T-003`.

### R-004 — Hold selects nearest eligible anchor

- Priority/status: high / active.
- Requirement: While the gameplay action is held, the satellite shall attach to the nearest eligible planet or debris object.
- Authority: SC-003; user brief.
- Relevant decision/constraint: D-002.
- Objective acceptance criterion (`AC-R-004`): **Pass** for every fixture when `ACTION_DOWN` selects the eligible object with minimum measured centre-to-centre distance at that simulation tick, with ties resolved by the documented stable object ID rule in A-005; otherwise **fail**.
- Exact verification: `T-004`.

### R-005 — Release preserves instantaneous state

- Priority/status: high / active.
- Requirement: Releasing the gameplay action shall detach the current anchor without an impulse, preserving satellite position and velocity at the release instant so momentum produces the subsequent arc.
- Authority: SC-004; user brief.
- Relevant decision/constraint: D-002, D-008.
- Objective acceptance criterion (`AC-R-005`): **Pass** when the release fixture shows no discontinuity beyond floating-point tolerance A-004 in position or velocity between the pre-release and post-release boundary samples and subsequent unanchored motion follows the preserved velocity; otherwise **fail**.
- Exact verification: `T-005`.

### R-006 — Data-fragment collection

- Priority/status: high / active.
- Requirement: A run shall contain collectible data fragments that increase the run score or collection count once per fragment.
- Authority: SC-005; user brief.
- Relevant decision/constraint: D-003.
- Objective acceptance criterion (`AC-R-006`): **Pass** when collision with each of three fixture fragments increments the documented fragment counter exactly once and removes or marks that fragment collected; otherwise **fail**.
- Exact verification: `T-006`.

### R-007 — Hazard-skimming bonus

- Priority/status: medium / active.
- Requirement: Passing through a hazard’s skim zone without entering its collision zone shall award bonus points.
- Authority: SC-006; user brief.
- Relevant decision/constraint: D-003, A-003.
- Objective acceptance criterion (`AC-R-007`): **Pass** when the near-pass fixture awards the configured positive skim bonus exactly once and the collision and outside-zone fixtures award none; otherwise **fail**.
- Exact verification: `T-007`.

### R-008 — Orbit multiplier continuity

- Priority/status: high / active.
- Requirement: The game shall maintain an orbit multiplier whose continuity depends on successful orbital play and whose current state affects scoring.
- Authority: SC-007; user brief.
- Relevant decision/constraint: D-003, A-003.
- Objective acceptance criterion (`AC-R-008`): **Pass** when the multiplier fixture demonstrates a documented qualifying event increasing or sustaining the multiplier, a documented break condition resetting or reducing it, and identical base-score events producing scores in the configured multiplier ratio; otherwise **fail**.
- Exact verification: `T-008`.

### R-009 — Inward object drift and collapsing star

- Priority/status: high / active.
- Requirement: Available gravity objects shall drift toward a visibly collapsing star during the run.
- Authority: SC-008; user brief.
- Relevant decision/constraint: D-003.
- Objective acceptance criterion (`AC-R-009`): **Pass** when every eligible object in the deterministic 30-second fixture has a smaller star-centre distance at the final sample than at the initial sample and the star’s collapse-state animation advances; otherwise **fail**.
- Exact verification: `T-009`.

### R-010 — Run duration

- Priority/status: high / active.
- Requirement: Every run shall end before three minutes of active-run simulation time.
- Authority: SC-009 and CON-001; user brief.
- Relevant decision/constraint: D-004, CON-001.
- Objective acceptance criterion (`AC-R-010`): **Pass** when 100 deterministic seeded runs all transition out of `RUNNING` at an active simulation timestamp strictly less than `180000 ms`; otherwise **fail**.
- Exact verification: `T-010`.

### R-011 — Immediate restart

- Priority/status: high / active; provisional operational threshold.
- Requirement: After a run ends, one restart activation shall produce a fresh, interactive run without a page reload.
- Authority: SC-010; user brief. The latency threshold is non-authoritative A-001.
- Relevant decision/constraint: D-004, A-001.
- Objective acceptance criterion (`AC-R-011`): **Pass provisionally** when 30 restart trials on the approved reference profile reach the first interactive simulation tick within `250 ms` of restart activation, with state reset and no network dependency; **fail** otherwise. The criterion cannot be finally accepted until OQ-005 and OQ-006 close.
- Exact verification: `T-011`.

### R-012 — Non-textual gameplay comprehension

- Priority/status: high / active; validation population unresolved.
- Requirement: Required gameplay state and feedback shall be communicated through animation with sound as a redundant cue, not through instructional gameplay text.
- Authority: SC-011; user brief. Redundant rather than sound-only communication is non-authoritative A-002.
- Relevant decision/constraint: D-005, A-002.
- Objective acceptance criterion (`AC-R-012`): **Pass** when the cue inventory maps attach, detach, fragment collection, hazard skim, multiplier change, danger, death, and restart-ready states to distinct animations; maps each to an audio cue or explicitly justified silent treatment; and a muted-audio run remains mechanically operable without instructional text. Otherwise **fail**.
- Exact verification: `T-012`.

### R-013 — Sparse geometric space art

- Priority/status: medium / active.
- Requirement: Gameplay visuals shall use a sparse geometric space-art language.
- Authority: SC-012; user brief.
- Relevant decision/constraint: D-006.
- Objective acceptance criterion (`AC-R-013`): **Pass** when every gameplay asset in the art inventory is composed from documented geometric primitives or procedural effects, uses the approved limited palette, and the representative one-screen capture contains no unapproved illustrative or photoreal asset; otherwise **fail**.
- Exact verification: `T-013`.

### R-014 — Solo-developer producibility

- Priority/status: high / active.
- Requirement: Prototype scope and asset production shall be achievable by one developer without a required specialist content pipeline.
- Authority: SC-013 and CON-002; user brief.
- Relevant decision/constraint: D-006, CON-002.
- Objective acceptance criterion (`AC-R-014`): **Pass** when every required prototype asset and system has one documented in-project production method, no backlog item requires a second concurrent contributor or paid bespoke asset, and the stakeholder accepts the single-developer estimate; otherwise **fail**.
- Exact verification: `T-014`.

### R-015 — Demonstrate skillful, predictable anchor switching

- Priority/status: high / active but proof gate blocked.
- Requirement: The prototype shall provide evidence that switching gravity anchors produces skillful, predictable movement.
- Authority: SC-014; user brief.
- Relevant decision/constraint: D-007, OQ-009, OQ-010.
- Objective acceptance criterion (`AC-R-015`): **Pass only** when a stakeholder-approved protocol defines representative participants, fixed scenarios, trial count, measurable prediction outcome, measurable skill-improvement outcome, thresholds, exclusion rules, and confidence treatment, and the recorded results meet every approved threshold. Until those fields are set and the study is run, status is **not evaluated**, not passed.
- Exact verification: `T-015`.

### R-016 — One-screen play

- Priority/status: high / active.
- Requirement: All active-run play shall occur in one playfield view without scene changes or scrolling to another gameplay screen.
- Authority: SC-015 and CON-003; user brief.
- Relevant decision/constraint: D-001, CON-003.
- Objective acceptance criterion (`AC-R-016`): **Pass** when a complete scripted run uses one playfield camera/viewport and all player, anchors, hazards, collectibles, star, and required state cues are presented within it without navigation to another gameplay scene; otherwise **fail**.
- Exact verification: `T-016`.

### R-017 — Physics model approval gate

- Priority/status: high / active governance constraint.
- Requirement: The exact physics model shall remain explicitly provisional until stakeholder approval; prototype documentation may implement the D-008 candidate only as a configurable evaluation model.
- Authority: SC-016, UNK-001, ANS-001.
- Relevant decision/constraint: D-008.
- Objective acceptance criterion (`AC-R-017`): **Pass** when the build identifies its physics model as `candidate`, exposes its feel parameters in configuration, and no record labels it approved unless D-008 status is changed by a stakeholder record; otherwise **fail**.
- Exact verification: `T-017`.

### R-018 — Input-accessibility decision gate

- Priority/status: high / active governance constraint.
- Requirement: Supported input modalities, remapping, Hold/Toggle behavior, and non-gameplay control operation shall remain separate stakeholder decisions and shall not be represented as a settled bundled baseline.
- Authority: SC-017, UNK-002, ANS-002.
- Relevant decision/constraint: D-009, OQ-002, OQ-003, OQ-004.
- Objective acceptance criterion (`AC-R-018`): **Pass** when each of the four subjects has an individual approved decision or is visibly marked unresolved at the release gate, and no document claims ANS-002 approved a bundle; otherwise **fail**.
- Exact verification: `T-018`.

### R-019 — Reference performance profile gate

- Priority/status: high / active governance constraint.
- Requirement: A reproducible low-end reference profile shall be stakeholder-approved before low-end performance is claimed.
- Authority: SC-018, UNK-007, ANS-007.
- Relevant decision/constraint: D-010, OQ-005.
- Objective acceptance criterion (`AC-R-019`): **Pass** when one approved record states device/hardware, operating system, browser/version, display mode, and whether the run uses physical hardware or a named approximation validated against it; otherwise **fail**.
- Exact verification: `T-019`.

### R-020 — Frame-rate target gate

- Priority/status: high / active governance constraint.
- Requirement: A numeric frame-rate target and measurement method shall be approved for the R-019 reference profile before performance acceptance is claimed.
- Authority: SC-018, UNK-003, ANS-003.
- Relevant decision/constraint: D-010, OQ-006.
- Objective acceptance criterion (`AC-R-020`): **Pass** when an approved decision states target statistic, numeric threshold, sampling interval, warm-up, permitted dropped-frame treatment, and R-019 profile, and a recorded test meets it; otherwise **fail**.
- Exact verification: `T-020`.

### R-021 — Gameplay randomness decision gate

- Priority/status: high / active governance constraint.
- Requirement: Gameplay-affecting randomness scope, quantity, seeding, and replay policy shall remain unset until stakeholders choose the desired variation/controllability balance.
- Authority: SC-019, UNK-004, ANS-004.
- Relevant decision/constraint: D-011, OQ-007.
- Objective acceptance criterion (`AC-R-021`): **Pass** when a stakeholder-approved record defines which run elements vary, their bounds/distribution, seed handling, and whether identical seeds reproduce identical setups; otherwise **fail**.
- Exact verification: `T-021`.

## Verification catalogue

Each test name is exact and may be implemented as automation or a recorded manual protocol with the same inputs and assertions.

| Test ID | Exact verification procedure |
|---|---|
| T-001 | `browser-clean-profile-launch`: open the documented build URL in a clean supported browser profile; record install prompts and time to interactive play; assert no native executable, extension, or plugin is installed. |
| T-002 | `single-action-command-audit`: replay the canonical complete-run input fixture; export gameplay command log and UI-control inventory; assert commands are only `ACTION_DOWN`/`ACTION_UP` and inventory contains no additional gameplay control. |
| T-003 | `moving-anchor-survival-fixture`: run fixture `moving-anchor-01`; assert anchor screen coordinates change, nonzero anchor-derived acceleration is logged while attached, and satellite remains alive after detach. |
| T-004 | `nearest-anchor-selection-table`: for fixtures with two anchors, ineligible nearest object, and an equal-distance tie, calculate distances at `ACTION_DOWN`; assert selected ID equals minimum eligible distance, using A-005 only for equality. |
| T-005 | `release-continuity-fixture`: sample position and velocity immediately before and after detach in fixture `release-01`; assert component deltas are within A-004 and no detach impulse event exists; advance unanchored simulation and compare with preserved-velocity expectation. |
| T-006 | `fragment-single-collection`: collide with fixture fragments F1–F3 twice each; assert each counter/score increment occurs once and collected state persists. |
| T-007 | `hazard-zone-boundaries`: execute paths inside collision radius, between collision and skim radii, and outside skim radius; assert bonus only for the middle path and only once. |
| T-008 | `multiplier-state-machine`: execute configured qualify, sustain, score, and break events; assert documented state transitions and score ratios. |
| T-009 | `inward-drift-30s`: run deterministic fixture for 30 simulation seconds; compare every eligible object’s initial/final star distance and capture star collapse-state progression. |
| T-010 | `run-cap-100-seeds`: simulate approved seeds 1–100 without pause; assert every `RUNNING` exit timestamp is `<180000 ms`. |
| T-011 | `restart-latency-30-trials`: on the OQ-005 profile, measure activation-to-first-interactive-tick for 30 ended runs; assert each is `≤250 ms`, new state is clean, and no page reload/network request is required. |
| T-012 | `audiovisual-cue-audit`: inspect the eight-state cue matrix, then complete scripted play muted and with instructional text disabled; assert every required animation exists, audio mapping/silent rationale exists, and mechanics remain operable. |
| T-013 | `art-inventory-conformance`: inspect each asset recipe, palette token, and a representative play capture against VisualIdentity; fail on unapproved non-geometric/photoreal assets or palette values. |
| T-014 | `solo-scope-audit`: inspect backlog and asset/system inventory for owner count, production method, external specialist dependency, and accepted estimate; assert the R-014 criterion. |
| T-015 | `anchor-switch-proof-study`: execute only the stakeholder-approved OQ-009 and OQ-010 protocol; retain anonymized task outcomes and analysis; compare every predeclared metric to its threshold. No protocol means `not evaluated`. |
| T-016 | `single-playfield-run-capture`: record one complete scripted run and scene/camera transition log; assert one gameplay viewport and zero navigation to another gameplay scene. |
| T-017 | `physics-authority-audit`: inspect build metadata, physics configuration, D-008 status, and all approval wording; assert candidate labeling and parameter exposure. |
| T-018 | `accessibility-decision-audit`: inspect D-009, OQ-002, OQ-003, and OQ-004 closure records; assert four separate decision subjects and no bundled approval claim. |
| T-019 | `reference-profile-schema-check`: validate the approved OQ-005 record contains every R-019 field and any approximation-to-hardware validation reference. |
| T-020 | `frame-target-schema-and-run`: validate the approved OQ-006 record contains every R-020 field; execute its named measurement method and compare result with threshold. |
| T-021 | `randomness-policy-schema-check`: validate the approved OQ-007 record contains every R-021 field; if reproducibility is required, run identical seed twice and compare setup manifests. |

## Constraint register

| Constraint ID | Exact source wording | Effect |
|---|---|---|
| CON-001 | “Runs should take less than three minutes” | Binding upper bound implemented by R-010. |
| CON-002 | “a solo developer could produce” | Scope/pipeline bound implemented by R-014. |
| CON-003 | “on one screen” | Presentation bound implemented by R-016. |
