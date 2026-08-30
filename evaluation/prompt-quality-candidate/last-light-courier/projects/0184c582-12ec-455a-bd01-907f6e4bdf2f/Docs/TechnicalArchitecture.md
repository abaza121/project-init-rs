# Technical Architecture

## Status

**Provisional technical design.** DEC-006 and DEC-007 are reversible inferences. The architecture implements the authorized loop while isolating open tuning and delivery choices. It does not settle any OQ record in OpenQuestions.md.

## Architecture goals

- Satisfy every product requirement in Requirements.md with one deterministic, testable gameplay model.
- Serve a browser-first single-screen 2D build while keeping native packaging deferred.
- Render simple geometric neon forms without making visual effects part of collision truth.
- Produce strong event-driven audio after an allowed user interaction.
- Keep tuning values named, centralized, and labeled by authority state.
- Avoid servers and accounts under ASS-008 unless OQ-003 changes scope.

## Provisional stack

| Layer | Choice | Status | Rationale and trace |
|---|---|---|---|
| Language | TypeScript with strict checking | Provisional | Supports explicit domain types and testable state boundaries; DEC-006. |
| Game framework | Phaser 4.1.x, exact patch pinned at implementation start | Provisional | EVD-014 and EVD-016; supports browser-first 2D and primitive rendering. |
| Build | Vite, exact major and patch pinned with lockfile | Provisional | EVD-015; static build and explicit browser target configuration. |
| Tests | Unit and property tests for pure domain logic; browser automation for integration | Provisional | Required by DEC-007 and the T- verification register. Tool selection occurs at implementation start. |
| Audio | Phaser audio abstraction or thin Web Audio adapter | Provisional | EVD-017; start or resume only from explicit user input. |
| Storage | None by default | Working inference | ASS-008; revisit only after OQ-003. |
| Backend | None | Working inference | No authorized network feature. |
| Desktop wrapper | None in baseline | Deferred | DEC-013 and OQ-008. |

No dependency is approved merely because it is listed. Implementation start requires version, license, vulnerability, bundle, and compatibility review.

## System boundaries

The runtime is divided into six modules with one-way dependencies.

1. **Domain model:** immutable or controlled state for run phase, ship, trail segments, parcel, towers, battery, multiplier, score, and tuning configuration.
2. **Simulation:** fixed-step movement, trail emission, spatial queries, collision eligibility, collection, delivery, close-pass detection, scoring, battery, and end transitions.
3. **Input adapter:** maps approved keyboard events to abstract actions. It owns focus and visibility transitions but not game rules.
4. **Presentation adapter:** maps domain state and events to Phaser geometry, text, particles, and HUD. Glow is cosmetic; collision geometry comes from simulation.
5. **Audio adapter:** maps domain events to cues and mixer channels. It owns user-gesture activation and mute behavior after OQ-011.
6. **Shell:** boot, title, mandatory onboarding, pause, settings if authorized, run, result, and restart.

Dependency direction: shell and adapters depend on the simulation interface; simulation depends only on domain types and deterministic math. Domain code does not import Phaser, browser globals, audio, storage, or build tools.

## Runtime states

| State | Entry | Allowed actions | Exit |
|---|---|---|---|
| Boot | Bundle loaded | Initialize config and assets | Ready or explicit load error |
| Ready | Assets available | Keyboard start; activate audio | Onboarding or Running |
| Onboarding | First run if required | Dismiss or demonstrate only approved mandatory instruction | Running within OQ-007 limit |
| Running | Run initialized | Movement; pause if authorized | Paused, EndedCollision, or EndedBattery |
| Paused | Authorized pause or visibility policy | Resume or abandon | Running or Results |
| EndedCollision | Authorized collision outcome | No score-changing input | Results |
| EndedBattery | Battery reaches zero | No score-changing input | Results |
| Results | End snapshot | Keyboard restart | Running |

Focus-loss behavior, pause authority, collision outcome, and onboarding timing remain open. The shell must not silently select them; prototype experiments use labeled configuration.

## Core data contracts

- **RunConfig:** logical bounds, fixed-step duration, nominal battery duration, movement parameters, trail thickness, collision exemption rule, close-pass band and windows, score values, multiplier limits, and feature flags. Every field carries a provenance comment: source, approved decision, or assumption.
- **RunState:** phase, elapsed active time, ship transform, ordered trail, parcel state, tower states, power state, score, multiplier state, and deterministic random seed.
- **InputFrame:** abstract directional or steering actions plus start, pause, resume, and restart.
- **DomainEvent:** TrailExtended, CityPowered, ParcelCollected, ParcelDelivered, ClosePass, MultiplierChanged, BatteryLow, Collision, BatteryExpired, RunEnded.
- **FrameView:** read-only geometry and HUD values for presentation.

## Simulation and timing

Use a fixed simulation step with an accumulator driven by a monotonic clock. Battery decreases from simulation-active elapsed time, not render-frame count. On hidden-tab or focus changes, the shell follows an explicit policy selected through OQ-009; EVD-018 makes this a required design choice.

Trail segments are ordered centerline primitives with width. Broad-phase spatial hashing or a uniform grid limits proximity checks; exact point-to-segment or swept-body tests determine collision and close passes. A segment’s cosmetic halo never affects collision. Recent-segment eligibility is controlled by ASS-003 and ASS-007 until OQ-010.

The random seed, configuration, and abstract input frames should be serializable for reproducible failing tests. This replay fixture is diagnostic only and is not a progression or player-replay feature.

## Gameplay event flow

1. Input adapter produces an InputFrame.
2. Simulation advances ship position.
3. Simulation emits contiguous trail and updates powered-city state.
4. Spatial checks evaluate bounds, eligible old-trail collision, parcel overlap, matching tower overlap, and close-pass band.
5. Score and multiplier consume domain events.
6. Battery advances and may emit BatteryLow or BatteryExpired.
7. A terminal event freezes score-changing simulation and snapshots results.
8. Presentation and audio independently consume the same events.

Ordering must be specified in tests for simultaneous collision, delivery, and expiry. OQ-010 must authorize the intended precedence before final acceptance.

## Rendering

Use one fixed logical arena and scale it to the available desktop viewport under ASS-009. Render primitives in layers: blackout background, unpowered city, powered city, towers and parcel, trail halo, trail collision core, ship, effects, HUD, shell overlay.

Use automatic WebGL or Canvas selection during exploration. Any WebGL-only glow must have a Canvas fallback that preserves hierarchy and collision-edge clarity. REQ-014 is about simple neon forms, not a particular shader.

## Input and audio

Keyboard mappings live in configuration and are shown using the actual approved binding labels. Use browser key handling that does not make display labels depend incorrectly on physical-layout codes; final binding and remapping scope remain OQ-009 and OQ-002.

Audio activation occurs on the first explicit start interaction. Every critical DomainEvent in REQ-015 has a stable cue ID, with cue assets or synthesis behind the adapter. If activation fails, present a visible state and retry on the next interaction. Mixer categories remain OQ-011.

## Testing architecture

| Test layer | Scope | Requirement references |
|---|---|---|
| Pure unit | State transitions, timer, score, matching, multiplier | T-TIME-001; T-PARCEL-001; T-DELIVERY-001; T-MULT-001; T-END-001 |
| Geometry property | Trail contiguity, collision eligibility, maze persistence, swept intersections | T-TRAIL-001; T-COLL-001; T-MAZE-001 |
| Scenario replay | Paired routes and simultaneous events | T-DESIGN-001; T-END-001 |
| Adapter integration | Keyboard actions, audio event map, visual state map | T-INPUT-001; T-AUDIO-001; T-REQ-004 |
| Browser end-to-end | Build boot, movement response, complete run, focus transition, restart | T-PLAT-001; T-MOVE-001; V-INPUT-001 |
| Human review | Visual hierarchy, effects, cue recognition, timing, onboarding | V-IDENT-001; V-FX-001; V-AUDIO-001; V-TIME-001; V-ONBOARD-001 |
| Governance static audit | Deferrals, asset manifest, source and ID integrity | T-GOV-001; T-GOV-002; T-GOV-003; T-GOV-004; T-GOV-005; T-GOV-006; T-ASSET-001 |

Exact assertions are canonical in [Requirements.md](Requirements.md). Planned tests are not reported as executed.

## Delivery topology

The baseline artifact is a versioned static web bundle served over HTTPS or local HTTP. Deployment host is unselected and no deployment is authorized. If OQ-008 requires a native executable, create a separate decision comparing wrapper choices against operating-system, signing, update, size, security, and store requirements; do not infer a wrapper from EVD-014.

## Risks and mitigations

| Risk | Current mitigation | Gate |
|---|---|---|
| Tuning assumptions harden into requirements | Central RunConfig with provenance and tests parameterized by config | OQ-001; OQ-006; OQ-009; OQ-010; OQ-013; OQ-014 |
| Visual effects alter collision readability | Separate logical core from cosmetic halo; fallback visual review | V-IDENT-001; EVD-016 |
| Audio is blocked | Activate on user gesture and show status | EVD-017; T-PLAT-001 |
| Hidden tabs distort timer | Explicit visibility policy and monotonic simulation time | EVD-018; OQ-009 |
| Accessibility recommendations become accidental scope | Feature flags and decision links; no compliance claim | OQ-002; DEC-009 |
| Native packaging expands scope | Keep DEC-013 deferred | OQ-008 |
| Dependency change or license risk | Pin versions and review at implementation start | Delivery gate |

## Implementation sequence

1. Close critical questions or authorize named, reversible assumptions.
2. Pin stack versions and record licenses and browser targets.
3. Build pure RunState, RunConfig, DomainEvent, fixed-step clock, and replay fixtures.
4. Add movement and trail geometry, then collision and maze tests.
5. Add parcel, tower, score, multiplier, and battery scenarios.
6. Add Phaser presentation and input adapter.
7. Add audio adapter after explicit interaction.
8. Add shell, onboarding, results, and focus policy.
9. Run all applicable T- and V- references; record results without unsupported success claims.

## Out of scope until authorized

Native wrapper, mobile or gamepad controls, networking, accounts, leaderboards, telemetry, cloud save, progression, procedural content, multiple difficulty modes, achievements, detailed narrative, detailed art, monetization, publishing, and deployment.
