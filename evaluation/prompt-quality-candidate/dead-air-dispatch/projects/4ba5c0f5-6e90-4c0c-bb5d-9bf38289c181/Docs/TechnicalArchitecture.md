# Technical Architecture

Status: proposed, engine-agnostic, reversible, and non-authoritative.  
Canonical decisions: `D-006` and `D-014` in [Traceability.md](Traceability.md).  
Canonical requirements: [Requirements.md](Requirements.md).

## Architecture objective

Provide testable seams for the brief-authorized tuning, revelation, caller guidance, hostile approach, timing, and scoring behavior without committing to an engine or unapproved services. The architecture is a planning model, not a technology selection.

## System boundary

In scope for the behavioral model:

- PC input event for mouse-wheel tuning.
- Run state, nominal timer, frequency/band state, reveal queries, caller guidance, hostile hearing/approach, and score-component accounting.
- Presentation adapters for 2D visuals and layered audio.
- Configuration and deterministic test fixtures.

Unresolved—not excluded:

- Operating-system/hardware targets, engine, language, middleware, alternate input, save data, leaderboard, network, analytics, content pipeline, and final accessibility features. See `OQ-007`, `OQ-009`, `OQ-013`, and `OQ-014`.

## Logical component model

| Component | Responsibility | Requirement inputs | Proposed interfaces |
|---|---|---|---|
| `InputAdapter` | Translate PC mouse-wheel events into signed tuning intents; leave alternate-device adapters possible. | `DAD-REQ-005` | `onTune(delta)`, event timestamps |
| `Tuner` | Hold current frequency and expose band membership; no continuous/stepped model selected. | `DAD-REQ-005`, `DAD-REQ-006`, `DAD-REQ-007` | `set/step`, `currentFrequency`, `bandsAt` |
| `RevealResolver` | Return hazards, callers, and routes revealed for current frequency and state. | `DAD-REQ-006` | `queryReveal(frequency, worldState)` |
| `CallerSystem` | Model caller target frequency, acquisition, hold progress, interruption, and rescued state. | `DAD-REQ-008`, `DAD-REQ-009`, `DAD-REQ-010` | `scan`, `acquire`, `advanceGuidance`, `callerState` |
| `ThreatSystem` | Model per-entity hearing conditions, approach, and antenna contact. | `DAD-REQ-007`, `DAD-REQ-011` | `canHear`, `advance`, `distance/state` |
| `RunDirector` | Coordinate loop states and terminal states; terminal rules await `OQ-008`. | `DAD-REQ-008`, `DAD-REQ-011`, `DAD-REQ-013` | `start`, `tick`, `transition`, `end` |
| `ScoreLedger` | Record rescue, accuracy, and risky-time contributions independently; weights remain unset. | `DAD-REQ-012` | `record*`, `componentTotals`, `weightedTotal` after approval |
| `PresentationModel` | Convert simulation state into semantic cue tokens independent of renderer/audio implementation. | `DAD-REQ-004`, `DAD-REQ-005`, `DAD-REQ-006`, `DAD-REQ-007` | `cueFrame`, semantic state IDs |
| `VisualAdapter` | Render the selected 2D/first-playable visual treatment. | `DAD-REQ-002`, `DAD-REQ-014` | engine-specific, undecided |
| `AudioAdapter` | Render layered sound and any adopted audio controls. | `DAD-REQ-014`, future `OQ-001` decision | engine-specific, undecided |
| `AccessibilityAdapter` | Optional extension point for adopted captions or equivalent cues; contains no committed feature today. | `DAD-REQ-016` | defined only after `OQ-001` |
| `ConfigRepository` | Hold tuning bands, content fixtures, nominal duration, and eventually approved score weights/bounds. | `DAD-REQ-006`, `DAD-REQ-007`, `DAD-REQ-012`, `DAD-REQ-013` | validated immutable run config |
| `Telemetry/TestPort` | Emit deterministic state transitions and score components for exact verification; analytics service is not implied. | verification catalog | event stream/file in development builds |

## Core state model

Proposed state sequence derived from `D-002`:

`RunStart → Scan → CallerCandidate → SignalHold → RescueResolved → RetuneWindow → Scan`

Concurrent hostile progression may occur while frequency-specific hearing is active. `AntennaContact` and other terminal outcomes are placeholders blocked by `OQ-008`; they are not invented requirements.

State invariants proposed for verification:

- Reveal results are a function of current frequency plus explicit world state.
- Entity hearing/approach decisions are inspectable separately from their rendering.
- Guidance progress begins only under the caller-acquisition condition.
- Score components remain individually inspectable before any weighting.
- Nominal duration is stored as 300 seconds under `D-013`; acceptance bounds remain absent until `OQ-005`.

## Data concepts

| Concept | Minimum fields for planning | Open detail |
|---|---|---|
| Frequency band | ID, bounds/membership rule, semantic type | continuous vs stepped, widths: `OQ-010` |
| Revealable | ID, type, reveal conditions | counts/archetypes: `OQ-012` |
| Caller | ID, target/acquisition condition, guidance duration, state | caller count and tuning values |
| Threat | ID, hearing condition, approach rule, antenna distance/state | contact outcome: `OQ-008` |
| Run config | nominal duration, content fixture, tuning config | bounds and PC performance baseline |
| Score ledger | rescue contribution, accuracy contribution, risk-time contribution | weights/caps/formula: `OQ-003` |
| Cue token | semantic event/state, intensity, direction if relevant | adopted visual/audio/accessibility mappings |

## Dependency direction

`InputAdapter → Tuner → Simulation systems → PresentationModel → Visual/Audio/Accessibility adapters`

`RunDirector` coordinates simulation systems; `ConfigRepository` supplies immutable run configuration; `Telemetry/TestPort` observes without controlling outcomes. Engine-facing adapters depend on the behavioral interfaces, not the reverse. This is the proposed separation in `D-014`.

## Verification architecture

- Fixed fixtures or seeds implement `ASM-004` for `VER-SIM-001`, `VER-SIM-002`, `VER-LOOP-002`, `VER-LOOP-003`, `VER-LOOP-004`, and `VER-SCORE-001`.
- Event records include timestamp/tick, frequency, loop state, revealed IDs, caller progress/state, threat hearing/approach state, and three score contributions.
- Presentation tests consume semantic cue tokens so identity experiments do not alter simulation assertions.
- Build smoke testing remains blocked on the exact baseline in `OQ-009`.

## Error and configuration handling

- Reject invalid band definitions and missing referenced content IDs before a run begins.
- Reject unapproved or incomplete score-weight configuration as “not baselined”; do not silently default it.
- Label absent duration bounds as unresolved; do not substitute arbitrary bounds.
- Version run configuration in test output so balance changes are traceable.
- Do not log personal data; player analytics is not authorized by the brief.

## Security and privacy posture

No remote service, account, leaderboard, telemetry collection, or personal-data flow is authorized. The current model is local and behavioral only, but this is not an approved project exclusion. If `OQ-014` adds services, perform a separate data-flow, threat, privacy, and retention review.

## Architecture acceptance gates

1. Stakeholder/product answers `OQ-011` for first-playable content.
2. Technical owner answers `OQ-007` and `OQ-009` before engine/build commitment.
3. Design owner answers `OQ-008`, `OQ-010`, and `OQ-012` before complete simulation implementation.
4. Stakeholders answer `OQ-001`, `OQ-003`, `OQ-004`, and `OQ-005` before accessibility, scoring, fairness, and duration acceptance claims.

