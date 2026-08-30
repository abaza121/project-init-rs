# Technical architecture

Status: **proposed, non-authoritative architecture** under D-013 and A-006. It is intended to make the required tests feasible for a solo-developed browser prototype. It does not settle OQ-001, OQ-002, OQ-003, OQ-004, OQ-005, OQ-006, OQ-007, or OQ-011.

## Context and boundaries

The prototype is one browser page with one active playfield (R-001, R-016). A small local build may use standard browser Canvas 2D and Web Audio APIs, but the exact language, framework, renderer, build tool, persistence, and hosting are not selected by the brief. No remote service is required by any requirement.

## Proposed component model

| Component | Responsibility | Requirement/test boundary |
|---|---|---|
| Input adapter | Normalize approved devices to `ACTION_DOWN` and `ACTION_UP`; expose gameplay vs non-gameplay controls separately | R-002, R-004, R-005, R-018; T-002, T-004, T-005, T-018 |
| Run state machine | `READY → RUNNING → ENDED → READY`; enforce run clock and clean restart | R-010, R-011; T-010, T-011 |
| World/simulation | Satellite state, moving anchors, eligibility, selected anchor, collision/skim zones, inward drift, star state | R-003, R-004, R-005, R-007, R-009, R-017 |
| Candidate force model | Configurable implementation of D-008 if approved for the spike; no hard-coded authority | R-017; T-003, T-004, T-005, T-017 |
| Scoring | Fragment collection, skim awards, multiplier state machine | R-006, R-007, R-008; T-006, T-007, T-008 |
| Renderer | Single playfield; geometric assets and audiovisual state cues | R-012, R-013, R-016; T-012, T-013, T-016 |
| Audio | Redundant event cues under A-002; mute-capable | R-012; T-012 |
| Configuration | Versioned values for all unsettled physics, zones, scoring, timing, and seed policy | A-003, A-004, OQ-007, OQ-011 |
| Local instrumentation | Simulation/input/event logs, seed/setup manifest, frame samples, restart timestamps; no remote export | Every applicable named verification in Requirements.md |
| Fixture/replay harness | Named deterministic scenarios and test assertions separate from gameplay randomness policy | T-003, T-004, T-005, T-006, T-007, T-008, T-009, T-010, T-015, T-021 |

## Proposed state and update sequence

For each simulation step:

1. Read normalized action transitions.
2. Update moving objects and collapse state from configuration.
3. On `ACTION_DOWN`, compute eligible distances and select the nearest, applying A-005 only for exact ties.
4. While held, apply only the configured selected-anchor force if the D-008 candidate is in use.
5. On `ACTION_UP`, remove anchor acceleration without a position/velocity impulse.
6. Integrate satellite motion using the approved timestep/model when available.
7. Resolve collision, skim, fragment, multiplier, death, and run-clock state.
8. Emit semantic events for renderer, audio, scoring, and local instrumentation.
9. Render from current state with interpolation only if the selected implementation requires it.

This sequence is a design proposal. OQ-001 and OQ-011 must settle model and numerical details. A fixed-step simulation is a tuning/engineering candidate, not a selected requirement.

## Data contracts

### Versioned configuration manifest

Proposed fields include `configVersion`, model name/status, timestep, gravitational parameters, object radii, eligibility rules, spawn/setup manifest, drift parameters, star-collapse curve, fragment value, skim radii/value, multiplier transitions, run cap, seed, and visual/audio timing tokens. Unset values remain explicit `TBD`; they must not receive invented defaults in approval documents.

### Local event record

Each record should contain simulation timestamp, event type, stable object IDs, relevant pre/post state, configuration version, and fixture/seed identifier. Player research records should use study-specific pseudonymous IDs and only fields approved by the research protocol. No analytics collection or remote transmission is authorized.

## Physics candidate boundary

D-008 proposes a 2D single-active-anchor inverse-square point-mass evaluation model based on ANS-001, with acceleration outside collision radius expressed there as `a = μ_A r/|r|³`. EVD-001, EVD-002, and EVD-003 support the physical abstractions but do not establish game suitability. The build must identify the model as `candidate`, expose feel parameters, and preserve release continuity under T-005. Alternative models remain possible until D-008 is accepted.

## Determinism and randomness

Named verification fixtures need repeatable initial conditions so failures can be reproduced. That does not decide the player-facing randomness amount. OQ-007 separately controls which live-run elements vary, their distribution/bounds, and seed/replay policy (D-011).

## Performance measurement

Use animation-frame timestamps for presentation sampling and simulation timestamps for game rules, but do not claim a frame-rate target. OQ-005 must first define the supported reference profile; OQ-006 must then define statistic, threshold, and method. Browser throttling may assist diagnosis but cannot silently substitute for the approved profile (EVD-007, EVD-008, EVD-009, EVD-010, EVD-017, EVD-018, EVD-019).

## Failure containment

- Pausing or losing focus should not consume active-run simulation time unless stakeholders explicitly decide otherwise; behavior is unresolved and should be included with non-gameplay control decisions in OQ-004.
- Restart should replace run-local state and retain only explicitly approved settings; persistence is otherwise not assumed.
- Invalid or missing required configuration should fail closed into a diagnostic state, not invent tuning values.
- Sound initialization failures should preserve visual mechanical operability under A-002.

## Delivery slices

1. **Movement fixture:** input normalization, moving anchors, candidate force configuration, attach/release logs, T-003, T-004, and T-005.
2. **Run loop:** fragments, skims, multiplier, inward drift, end/restart, T-006, T-007, T-008, T-009, T-010, and T-011.
3. **Communication layer:** geometric renderer, cue matrix, audio/mute, T-012, T-013, T-014, and T-016.
4. **Proof build:** approved input/performance/randomness choices, fixture harness, and stakeholder-approved T-015 protocol.

Each slice is a planning recommendation. It should be re-estimated by the solo developer; no schedule commitment is inferred.
