# Technical Architecture

Status: **provisional, engine-agnostic architecture** under ASM-009 and DEC-041. It describes boundaries and test seams, not a selected implementation stack. OQ-005 and OQ-006 must close before vendor APIs, build targets, performance budgets, or release packaging can be authoritative.

## Architectural drivers

| Driver | Architectural consequence |
|---|---|
| REQ-002 and REQ-017 | Local input router supports two-controller and shared-keyboard device profiles. |
| REQ-003; REQ-004; REQ-005; REQ-006 | One mech aggregate owns two swappable role assignments. |
| REQ-007; REQ-008; REQ-009; REQ-010 | One attempt orchestrator coordinates scrolling, rescue, wreckage, scoring, and timeout. |
| REQ-011 and REQ-012 | Fixed-clock session state and deterministic reset are first-class services. |
| REQ-013 | Physics parameters are data-driven and observable in fixtures. |
| REQ-014; REQ-015; REQ-016 | Presentation state exposes role, hazard, civilian, timer, multiplier, and failure cues through redundant channels. |
| REQ-018; REQ-019; REQ-020 | Content manifests enforce one block, one civilian archetype, and three hazard archetypes. |

## Context map

```text
Device Adapters -> Input Router -> Role Assignment -> Shared Mech
                                         |              |
                                         v              v
                                  Attempt Orchestrator -> Interaction/Physics
                                         |              |
                                         v              v
                                  Score/Timer State -> Event Stream
                                         |              |
                                         +-------> Presentation
                                         |
                                         +-------> Telemetry/Test Probes

Content Manifest -> Zone Streamer -> Civilian/Wreckage/Hazard Factories
```

## Components

### Device adapters

Translate controller and keyboard events into semantic actions such as `move`, `jump`, `arm_aim`, `magnet`, and `shield_aim`. Controller brand, key layout, dead zones, and remapping UI are not selected. Remapping remains behind OQ-001.

### Input router and role assignment

Bind two player assignments to `MovementRole` and `ToolRole`. The role state machine consumes a validated damage event, swaps assignments exactly once, emits `roles_swapped`, and updates presentation before accepting the next gameplay input. Device identity remains stable while role identity changes.

### Shared mech aggregate

Own one mech identity, movement state, jump state, magnetic-arm state, shield state, damage state, and current failure presentation. It rejects tool actions from the movement role and locomotion actions from the tool role.

### Attempt orchestrator

State machine: `Ready → Active → Results → Resetting → Ready`. Active gameplay owns the 180-second fixed clock. Restart disposes all attempt-scoped entities and restores the initial timer, score, multiplier, roles, seed, and manifest references.

### Interaction and physics

Provide collision, catch, magnet acquisition, throw impulse, shield response, damage validation, and recoverable-failure effects. Gameplay outcomes use domain events rather than presentation callbacks so exaggerated animation cannot duplicate damage or rescue scoring.

### Zone and content

The zone streamer accepts one `city_block` manifest reference. It does not assume whether that unit equals the whole logical zone under ASM-011. Under non-authoritative ASM-010, factories load one neutral civilian fixture and three neutral hazard fixtures until OQ-007 and OQ-008 close; fixture names carry no creative identity.

### Score, timer, and teamwork

The fixed clock is monotonic during `Active` and frozen elsewhere. The score service consumes idempotent action events. Multiplier tuning is data; the required test asserts only that the specified coordinated sequence increases it and timeout freezes it.

### Presentation

Reads immutable frame state and maps critical state to shape, placement, text/icon, motion, audio, and optional haptic channels. Presentation cannot author damage, rescue, or score events. Television test settings remain provisional under ASM-001 and ASM-006.

### Telemetry and test probes

Local structured events support verification: input assignment, role transition, damage source, civilian state, wreckage state, multiplier change, timer transition, restart milestones, and manifest counts. No remote analytics service is selected or authorised.

## Core event contracts

| Event | Required fields | Invariant |
|---|---|---|
| `damage_validated` | attempt ID, mech ID, source fixture ID, tick | At most one role swap per event ID. |
| `roles_swapped` | attempt ID, previous assignments, new assignments, tick | New movement assignment equals previous tool assignment and conversely. |
| `civilian_rescued` | attempt ID, civilian ID, catch actor, tick | Mutually exclusive with impact for the same civilian. |
| `wreckage_cleared` | attempt ID, wreckage ID, route volume ID, tick | Emitted only when fully outside the route volume. |
| `multiplier_changed` | attempt ID, old value, new value, cause event ID | Rejected after timeout. |
| `attempt_timed_out` | attempt ID, tick | Exactly once at 180 seconds of active time. |
| `restart_completed` | old attempt ID, new attempt ID, elapsed time | New state equals manifest-defined initial state. |

## Determinism and reset strategy

- Use a fixed simulation timestep and injectable clock for tests.
- Seed attempt-scoped randomness and record the seed locally.
- Give domain events unique IDs; score and swap consumers are idempotent.
- Compare reset state to a canonical initial-state snapshot, excluding new attempt ID and wall-clock timestamp.
- Keep visual-only randomness outside gameplay collision and scoring.

## Test architecture

The exact `TST-REQ-*` references are defined with each requirement in [Requirements.md](Requirements.md). Automated fixtures cover state, input isolation, manifests, timer, catches, throws, score, and reset. Human protocols cover silhouettes, television readability, mastery, coordination, and comedy; all current thresholds are explicitly provisional assumptions.

## Security, privacy, and network posture

The mandatory prototype is local. The architecture specifies no accounts, chat, remote telemetry, purchases, cloud saves, or deployment. This is a working omission under ASM-012, not an approved exclusion. If any remote capability enters scope, threat modelling, consent, data retention, and network-failure requirements must be added before implementation.

## Technology selection gate

A stakeholder decision on OQ-006 should evaluate at least: compatibility with the selected platform from OQ-005, two-controller and shared-keyboard input, deterministic physics/test seams, television output, team capability, licensing, console-support model, build automation, and profiling. EVD-TEC-001, EVD-TEC-002, and EVD-TEC-003 show that capability evidence constrains but does not decide this selection.
