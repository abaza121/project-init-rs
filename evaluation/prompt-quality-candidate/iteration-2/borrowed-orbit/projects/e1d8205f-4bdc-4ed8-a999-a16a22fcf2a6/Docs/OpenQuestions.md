# Pending Matters

## User Answers
ANS-001: The exact physics model remains unresolved and requires stakeholder approval. The cited sources support inverse-square gravitational acceleration, momentum continuing when gravitational acceleration is removed, and fixed-timestep simulation as technically grounded options. They do not establish that Borrowed Orbit should lock one anchor for the entire hold, use only one gravitational source, make anchors kinematic, stop approach at collision radii, or adopt those choices as the project’s settled model.
ANS-002: FAIL — The input-accessibility feature set remains unresolved. The supplied evidence supports considering keyboard operability and configurable alternatives to prolonged holds, but it does not determine Borrowed Orbit’s conformance target, bindings, remapping behavior, or whether a toggle or auto-hold mode may change the core hold/release interaction. Those choices require stakeholder approval.
ANS-003: FAIL — The target frame rate on low-end devices remains unresolved. The evidence identifies 60 FPS as general smooth-animation guidance, but it does not establish that target for Borrowed Orbit or define the project’s “low-end devices.” A responsible acceptance target requires stakeholder selection of the supported device/browser benchmark and acceptable sustained frame rate.
ANS-004: FAIL — The amount of randomness per run remains unresolved. The supplied evidence supports evaluating procedural generation against designer goals and player experience, but it cannot determine Borrowed Orbit’s intended balance between run-to-run variety and its authoritative requirement for skillful, predictable movement. That stakeholder-owned choice must be clarified rather than inferred from research.

## Constraints
CON-001: Runs should take less than three minutes.
CON-002: Core play is understandable from animation and sound “instead of text”.
CON-003: The sparse geometric space art is something “a solo developer could produce”.
CON-004: The prototype demonstrates its movement “on one screen”.

## Open Questions
OQ-001: Which exact physics model does the stakeholder approve, including anchor selection timing, force-source count, moving-body behavior, collision-radius behavior, numerical integrator, and simulation timestep?
OQ-002: Which input-accessibility target does the stakeholder approve, including keyboard bindings, remapping, prolonged-hold alternatives, and whether toggle or auto-hold may alter hold/release timing?
OQ-003: Which benchmark devices and browsers define “low-end devices,” and what sustained frame-rate and frame-time thresholds must they meet?
OQ-004: Which subsystems may vary between runs, what seed behavior is required, and what bounds preserve skillful, predictable movement?
OQ-005: What stakeholder-approved participant count, practice allowance, task, and pass threshold will prove skillful, predictable gravity-anchor switching?
OQ-006: What stakeholder-approved maximum restart latency qualifies as “instantly,” and on which benchmark environment?
OQ-007: Which desktop and mobile browsers, viewport sizes, orientations, pointer types, and audio policies are in the supported prototype matrix?

## Neutral priority

`OQ-001` through `OQ-004` block final implementation baselines. `OQ-005` and `OQ-006` block final acceptance thresholds. `OQ-007` blocks a complete compatibility and performance matrix.
