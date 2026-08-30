# Dead Air Dispatch Working Bounds

## Assumptions
ASM-001: ASSUMPTION: A framework-neutral component design can preserve the gameplay intent until the engine and programming language are selected; this is non-authoritative and may change after OQ-008 is answered.
ASM-002: ASSUMPTION: A single logical room can contain the station, callers, routes, hazards, and approaching entities needed to exercise the full loop; this is non-authoritative and must be tested against REQ-005.
ASM-003: ASSUMPTION: The nominal centre of “about five minutes” is 300 seconds; this is non-authoritative and does not define the unresolved duration tolerance in OQ-005.
ASM-004: ASSUMPTION: Deterministic simulation fixtures and event logs are feasible for tuning, entities, rescue state, and scoring regardless of the eventual engine; this is non-authoritative and may change after OQ-008 is answered.
ASM-005: ASSUMPTION: Playtest instrumentation may collect run duration, rescues, signal error, dangerous-band exposure, deaths or failures, and post-run responses without collecting direct personal identifiers; this is non-authoritative pending a telemetry and consent decision.

## Constraints
CON-001: The first playable can use abstract waveforms, silhouettes, static, and layered sound in a single room.
CON-002: The first playable is for PC and its mandatory tuning input is the mouse wheel.
CON-003: Scoring weights, the hidden-information acceptance cutoff, the run-duration tolerance, first-playable success criteria, audience, exclusions, and accessibility scope cannot be settled without stakeholder authority recorded in their owning OQ records.
CON-004: External guidance may inform candidate designs and measurement methods but cannot supply stakeholder authority for product scope, priorities, or acceptance thresholds.

## Working Notes

Assumptions are deliberately separable from requirements. A future answer may replace an ASM record with an ANS record and a consequential DEC record; until then, downstream documents must retain the `ASSUMPTION:` label.
