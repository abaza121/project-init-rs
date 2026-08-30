# Technical Design

## Operating boundaries

The planned deliverable is a client-side browser prototype with one fixed playfield. No account, network service, leaderboard, purchase, analytics pipeline, or remote persistence is authorized. Client-only delivery is proposed under `ASM-010`, not confirmed by the brief.

## Runtime flow

1. The input adapter emits one action’s press, held, and release transitions.
2. The selector evaluates eligible planets and debris and exposes the nearest candidate at attachment time without yet assuming whether an anchor locks for the full hold.
3. The replaceable simulation policy advances bodies and the satellite; exact force, integration, collision-radius, and timestep rules wait on `OQ-001`.
4. The rule system evaluates fragments, skims, collision, multiplier continuity, star drift, run timeout, terminal state, and restart.
5. The renderer interpolates presentational state in the fixed viewport; the audio layer maps state events to cues.
6. The test harness records initial state, configuration, input transitions, state transitions, and sampled trajectories for repeatable acceptance checks.

## Module map

| Module | Responsibility | Canonical boundary |
|---|---|---|
| `InputAdapter` | Normalize pointer, touch, and eventual keyboard/accessibility modes into one action state | Feature set waits on `OQ-002` |
| `AnchorSelector` | Find eligible nearest body and expose selection feedback | `REQ-004` |
| `SimulationPolicy` | Apply the approved motion and collision model behind a replaceable interface | `OQ-001`, `ASM-006` |
| `RunRules` | Score collection and skims, update multiplier, drift objects, cap run duration, and terminate | `REQ-006` through `REQ-010` |
| `Presentation` | Draw primitive geometry, motion cues, score, multiplier, and state feedback | `REQ-001`, `REQ-012`, `REQ-013`, `REQ-015` |
| `AudioCues` | Emit short event-coded cues without carrying essential information alone until accessibility is settled | `REQ-012`, `OQ-002` |
| `RunLifecycle` | Initialize, terminate, and reset complete run state | `REQ-011`, `ASM-002` |
| `ReplayHarness` | Capture deterministic fixtures and compare input-trace outcomes | `REQ-014`, `ASM-004` |

## State model

`RunState` contains phase, elapsed time, score, multiplier value and expiry, satellite transform and velocity, eligible bodies, fragments, hazards, star state, active or candidate anchor identifiers, and a run configuration identifier. Random seeds and distributions are intentionally absent from the settled model until `OQ-004` is answered.

## Decisions
DEC-001: status=CONFIRMED; provenance=user brief; source=REQ-002; the one-button interaction uses one primary gameplay action with hold and release phases supports REQ-002 because the one-button subject is explicitly supplied.
DEC-002: status=CONFIRMED; provenance=user brief; source=REQ-004; the hold attachment targets the nearest eligible planet or debris supports REQ-004 because nearest planet-or-debris attachment is explicitly supplied without settling anchor-lock timing.
DEC-003: status=CONFIRMED; provenance=user brief; source=REQ-005; release preserves satellite momentum into a new arc supports REQ-005 because preserved momentum and new arc are explicitly supplied.
DEC-004: status=CONFIRMED; provenance=user brief; source=REQ-006; data-fragment collection is a play goal supports REQ-006 because data fragments are explicitly supplied.
DEC-005: status=CONFIRMED; provenance=user brief; source=REQ-007; hazard skims award bonus points supports REQ-007 because hazard skim bonus points are explicitly supplied.
DEC-006: status=CONFIRMED; provenance=user brief; source=REQ-008; orbit-multiplier continuity is a play goal supports REQ-008 because keeping an orbit multiplier alive is explicitly supplied.
DEC-007: status=CONFIRMED; provenance=user brief; source=REQ-009; available objects drift toward the collapsing star supports REQ-009 because object drift and collapsing star pressure are explicitly supplied.
DEC-008: status=CONFIRMED; provenance=user brief; source=REQ-010; the run-duration cap is strictly below three minutes supports REQ-010 because the less-than-three-minute run is explicitly supplied.
DEC-009: status=CONFIRMED; provenance=user brief; source=REQ-011; terminal play offers an immediate restart path supports REQ-011 because instant restart is explicitly supplied while the latency threshold remains open.
DEC-010: status=CONFIRMED; provenance=user brief; source=REQ-012; animation and sound teach core play instead of instructional text supports REQ-012 because non-textual animation-and-sound understanding is explicitly supplied.
DEC-011: status=CONFIRMED; provenance=user brief; source=REQ-013; sparse geometric space art defines the production style supports REQ-013 because solo-producible sparse geometric art is explicitly supplied.
DEC-012: status=CONFIRMED; provenance=user brief; source=REQ-014; gravity-anchor switching is the prototype’s skillful-and-predictable movement proof supports REQ-014 because the switching gravity anchors proof is explicitly supplied.
DEC-013: status=CONFIRMED; provenance=user brief; source=REQ-015; gameplay remains in one fixed screen supports REQ-015 because the one-screen subject is explicitly supplied.
DEC-014: status=DEFERRED; provenance=supplied answer; source=ANS-001; relevant=EVD-001; the exact physics model remains stakeholder-owned supports REQ-016 because the exact physics model is explicitly unchosen and cannot be confirmed without new authority.
DEC-015: status=DEFERRED; provenance=supplied answer; source=ANS-002; relevant=EVD-005; the input-accessibility feature set remains stakeholder-owned supports REQ-017 because input accessibility features are explicitly unchosen and cannot be confirmed without new authority.
DEC-016: status=DEFERRED; provenance=supplied answer; source=ANS-003; relevant=EVD-010; the low-end-device frame-rate target remains stakeholder-owned supports REQ-018 because the target frame rate on low-end devices is explicitly unchosen and lacks an approved benchmark.
DEC-017: status=DEFERRED; provenance=supplied answer; source=ANS-004; relevant=EVD-014; the amount of run randomness remains stakeholder-owned supports REQ-019 because how much randomness each run contains is explicitly unchosen and lacks approved bounds.
DEC-018: status=PROPOSED; provenance=agent design inference; source=ASM-005; the Canvas 2D geometric renderer remains replaceable supports REQ-013 because Canvas 2D can express the proposed sparse geometric primitives without becoming stakeholder authority.
DEC-019: status=PROPOSED; provenance=agent design inference; source=ASM-006; relevant=EVD-004; the replaceable simulation-policy boundary isolates timestep and force choices supports REQ-014 because predictable movement tests need swappable simulation choices until approval.
DEC-020: status=PROPOSED; provenance=agent design inference; source=ASM-007; the synthesized audio-cue layer remains replaceable supports REQ-012 because sound feedback can be prototyped with solo-producible procedural cues.
DEC-021: status=PROPOSED; provenance=agent design inference; source=ASM-008; the deep-navy cyan amber and red-orange palette remains provisional supports REQ-013 because the palette applies a sparse geometric space-art system without claiming stakeholder approval.
DEC-022: status=PROPOSED; provenance=agent design inference; source=ASM-009; the system sans-serif typography remains provisional supports REQ-013 because the typography avoids a custom asset while preserving sparse production scope.
DEC-023: status=PROPOSED; provenance=agent design inference; source=ASM-010; the client-only browser prototype remains free of required server state supports REQ-003 because the browser game can run in one client while unrequested services stay out of scope.
DEC-024: status=PROPOSED; provenance=agent design inference; source=ASM-011; relevant=EVD-011; requestAnimationFrame schedules visual presentation but does not settle simulation timestep supports REQ-003 because browser presentation cadence and gravity simulation policy remain separated.
DEC-025: status=CONFIRMED; provenance=user brief; source=REQ-020; the project name is Borrowed Orbit supports REQ-020 because the Borrowed Orbit identity is exactly supplied.

## Delivery slices

1. Instrumented movement sandbox: input, nearest selection, replaceable physics, trace drawing, and replay capture.
2. Proof loop: fragments, hazard skim, multiplier, star drift, terminal state, and restart.
3. Communication layer: geometric state animation, synthesized cue prototypes, and text-free comprehension fixture.
4. Evaluation build: scripted trajectories, duration and reset tests, seeded fixtures if approved, performance harness, and participant test protocol.

No slice may silently resolve `OQ-001` through `OQ-007`; experiments must label candidate configurations and record them separately from stakeholder-approved baselines.
