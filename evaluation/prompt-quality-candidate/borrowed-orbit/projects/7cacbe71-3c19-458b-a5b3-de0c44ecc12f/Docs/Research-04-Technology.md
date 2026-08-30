# Research 04 — Technology

## Research questions

Which physics abstraction is a responsible prototype candidate, and how should browser performance claims be bounded?

## Evidence retained

Physics: [EVD-001](Traceability.md#evd-001), [EVD-002](Traceability.md#evd-002), [EVD-003](Traceability.md#evd-003).  
Browser timing/device evaluation: [EVD-007](Traceability.md#evd-007), [EVD-008](Traceability.md#evd-008), [EVD-009](Traceability.md#evd-009), [EVD-010](Traceability.md#evd-010), [EVD-017](Traceability.md#evd-017), [EVD-018](Traceability.md#evd-018), [EVD-019](Traceability.md#evd-019).

## Evidence-grounded interpretation

- EVD-001, EVD-002, and EVD-003 provide credible foundations for an inverse-square, one-dominant-body point-mass abstraction. Applying it to a player-selected game anchor is an inference, so D-008 remains proposed and R-017 preserves the approval boundary.
- EVD-007, EVD-008, EVD-009, and EVD-010 provide browser frame-timing and diagnostic-throttling context. EVD-017, EVD-018, and EVD-019 reinforce the need to define a target device range and distinguish approximations from real hardware. Together they inform D-010, R-019, and R-020; none selects Borrowed Orbit’s device floor or FPS target.

## Proposed technical spike — design inference

Implement the D-008 model behind configuration and record input, selection, acceleration, release, position, and velocity for T-003, T-004, and T-005. Compare at least internally consistent fixture outcomes before investing in scoring content. This can test implementation behavior; it cannot by itself prove player skill or predictability.

For performance, first close OQ-005, then OQ-006. An emulated throttle may be retained as a diagnostic companion only if the approved method states its relationship to physical hardware.

## Unavailable research

No prototype benchmark, numerical-stability experiment, integrator comparison, cross-browser result, physical-device measurement, memory profile, GPU trace, thermal result, asset-size budget, or browser support matrix was supplied or performed. The exact framework, language, renderer, audio approach, build system, and host remain unselected.

## Project consequence

D-008 is proposed, D-010 is pending, and D-013 is proposed. No document may claim that inverse-square physics produces the desired feel or that a generic 60 Hz example is the project target.
