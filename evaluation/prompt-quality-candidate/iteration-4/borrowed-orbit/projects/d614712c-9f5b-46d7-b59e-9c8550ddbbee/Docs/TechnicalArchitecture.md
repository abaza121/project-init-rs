# Technical Design

## Decisions

DEC-001: Status PROPOSED; provenance type supplied recommendation from ANS-001 and non-authoritative inference from ASM-005; use a gated single-attractor experiment whose anchor-directed acceleration is constrained by evidence EVD-005, without treating its tuning as approved, supports REQ-002 because the gated single-attractor gravity experiment tests how the satellite borrows gravity from moving objects.
DEC-002: Status ACCEPTED; provenance type direct brief requirement from REQ-004; preserve satellite world-space velocity at release, with momentum behavior constrained by evidence EVD-006, supports REQ-004 because unchanged release velocity implements preserved momentum into the subsequent arc.
DEC-003: Status PROPOSED; provenance type supplied recommendation from ANS-001 and non-authoritative inference from ASM-005; run the physics experiment at a fixed 1/60-second step with semi-implicit Euler, using evidence EVD-007 and EVD-008 only as starting-method evidence, supports REQ-013 because a fixed-step integrator makes gravity-anchor movement repeatable enough to test predictability.
DEC-004: Status ACCEPTED; provenance type direct brief requirement from REQ-013 and constraint CON-004; keep the full active playfield and camera on one screen supports REQ-013 because one-screen gravity-anchor switching is the stated prototype proof boundary.
DEC-005: Status PROPOSED; provenance type direct brief requirements from REQ-001 and REQ-010 plus non-authoritative threshold from ASM-001; reuse the one primary action to restart after a terminal state supports REQ-010 because the same one-button action provides an immediate restart path.
DEC-006: Status PROPOSED; provenance type supplied boundary from ANS-002 and non-authoritative inference from ASM-007; implement a keyboard-operable primary-action experiment, constrained by accessibility evidence EVD-002 but not presented as the approved accessibility scope, supports REQ-001 because a keyboard path preserves the one-button browser interaction.
DEC-007: Status PROPOSED; provenance type supplied boundary from ANS-002 and non-authoritative inference from ASM-007; implement concurrent pointer and touch primary-action experiments, constrained by input-modality evidence EVD-003 but not presented as the approved accessibility scope, supports REQ-001 because concurrent pointer and touch paths preserve the one-button browser interaction.
DEC-008: Status PROPOSED; provenance type supplied boundary from ANS-003 and non-authoritative inference from ASM-008; log render frame times and defer any low-end pass threshold until a device cohort is approved, using representative-device evidence EVD-009, supports REQ-014 because the frame-rate target remains unapproved while measurement capability is prepared.
DEC-009: Status PROPOSED; provenance type supplied boundary from ANS-004 and non-authoritative inference from ASM-006; use an authored deterministic default scenario and isolate optional seeded variation until variation goals exist, using goal-specific generator evidence EVD-004, supports REQ-014 because the randomness amount remains unapproved while predictable movement can still be tested.
DEC-010: Status PROPOSED; provenance type supplied boundary from ANS-005 and non-authoritative inference from ASM-009; defer claims about representative players until the intended audience is approved, using target-user evidence EVD-001, supports REQ-013 because the skillful movement trial remains explicitly provisional rather than an audience-wide claim.
DEC-011: Status ACCEPTED; provenance type direct brief requirement from REQ-012 and constraint CON-003; construct gameplay art from sparse reusable geometric primitives supports REQ-012 because geometric primitives bound visual production for a solo developer.
DEC-012: Status PROPOSED; provenance type design inference from ASM-010 and direct prototype requirement from REQ-013; separate input, simulation, rules, presentation, and telemetry modules behind one ordered state-update boundary supports REQ-013 because separable module responsibility and data flow make predictable gravity-anchor behavior inspectable.
DEC-013: Status PROPOSED; provenance type visual inference from ASM-011 and direct brief requirements from REQ-011 and REQ-012; explore the indigo, ivory, cyan, violet, amber, and coral geometric palette with critical states paired across shape, motion, and sound supports REQ-012 because the proposed palette and paired cues provide a producible sparse geometric space-art direction.

## Implementation structure

The input module has responsibility for converting keyboard, pointer, and touch events into a single normalized action state: pressed, held, or released. It does not choose accessibility policy; its bindings and modes remain configurable pending approval.

The scenario module has responsibility for one-screen initial state, authored anchor drift paths, the collapsing-star sink, fragments, hazards, and optional logged seeds. Its smallest useful boundary is a serializable scenario definition so the same setup can be replayed.

The anchor-selection module has responsibility for eligibility, distance comparison at press time, and the locked active-anchor reference. Its data flow is normalized action plus current positions in, active-anchor state out.

The simulation module has responsibility for satellite position and velocity, provisional anchor acceleration, fixed-step integration, and kinematic object drift. Its data flow is prior simulation state plus active anchor and elapsed fixed steps in, next transforms and velocities out. Exact force parameters, collision radii, and the final model remain open.

The collision module has responsibility for contact, hazard-skim band entry, fragment collection, and star consumption. It consumes transforms after simulation and emits typed events; the still-open skim distances and terminal collision rules stay in configuration rather than hidden constants.

The rules module has responsibility for score, data-fragment count, orbit multiplier, run timer, terminal state, and restart reset. Its data flow is typed collision and timing events in, authoritative run state out. Multiplier timing and early run-ending events remain open implementation choices.

The presentation module has responsibility for sparse geometry, animation cues, sound cues, and display of the current simulation state. It reads state without changing gameplay outcomes, allowing animation and sound comprehension to be tested separately from motion rules.

The telemetry module has responsibility for timestamped normalized inputs, fixed-step state samples, frame times, seeds, outcomes, and acceptance-test exports. It is observational and must not alter the simulation data flow.

One ordered update transition connects the modules: collect normalized input, update anchor state, advance kinematic objects and satellite simulation, resolve collision events, update rules, then publish immutable state to presentation and telemetry. Browser framework, rendering API, audio library, build tool, hosting, and analytics technology are intentionally not prescribed because no supported choice was supplied.

## Initial slices

First, build a replayable one-screen movement sandbox with input normalization, nearest-anchor locking, provisional acceleration, release velocity continuity, and telemetry. Second, add drifting objects and the collapsing-star sink. Third, add fragments, hazard skims, multiplier rules, run cap, terminal state, and restart. Fourth, add geometric presentation, animation cues, sound cues, and the provisional evaluation harness. Each slice should preserve deterministic replay before the next is added.
