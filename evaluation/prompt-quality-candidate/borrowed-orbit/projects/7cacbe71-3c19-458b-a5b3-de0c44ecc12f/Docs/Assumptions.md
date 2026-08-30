# Assumptions

This is the canonical register for inferred choices. Assumptions are non-authoritative and may be used only as provisional test scaffolding. A stakeholder decision supersedes an assumption.

| ID | Assumption / provisional inference | Impact | Basis | Used by | Validation or retirement condition |
|---|---|---|---|---|---|
| A-001 | **Non-authoritative:** operationalize “restart instantly” as activation-to-first-interactive-tick `≤250 ms` on every one of 30 trials, with no reload or network dependency. | High | “Instantly” in SC-010 needs a measurable threshold; no source selects `250 ms`. | R-011, T-011, D-004 | Stakeholder accepts/replaces the latency and trial count after OQ-005 closes. |
| A-002 | **Non-authoritative design inference:** animation should carry all mechanically necessary information and sound should be redundant, so muted play remains possible. | High | SC-011 requests animation and sound rather than text; the brief does not authorize sound-only dependencies. | R-012, D-005, VisualIdentity.md | Accessibility decisions in OQ-002, OQ-003, and OQ-004 approve or replace the cue policy; user testing validates comprehension. |
| A-003 | **Non-authoritative tuning scaffold:** skim radii, multiplier qualification, decay/reset, and score values will be configuration rather than hard-coded constants. No numeric value is approved. | High | SC-006 and SC-007 specify systems but not thresholds or tuning. | R-007, R-008, D-003, TechnicalArchitecture.md | Stakeholders approve tuning after instrumented playtests. |
| A-004 | **Non-authoritative engineering scaffold:** release-continuity comparisons will use a documented tolerance appropriate to the selected numeric representation; the value is unset. | Medium | Exact floating-point equality is not a responsible cross-platform criterion; no tolerance is authorized. | R-005, T-005, D-008 | Engineering selects and documents a tolerance after the numeric model/timestep is approved. |
| A-005 | **Non-authoritative deterministic rule:** equal-distance anchor ties use ascending stable object ID. | Medium | SC-003 defines nearest selection but not exact ties; a stable rule makes tests reproducible. | R-004, T-004, D-002 | Stakeholder/engineering approves a different tie policy or accepts this rule. |
| A-006 | **Non-authoritative architecture inference:** keep simulation, rendering, audio, input, and telemetry boundaries separable in a small browser codebase. | High | Supports testing and solo iteration; the brief does not select an architecture. | D-013, TechnicalArchitecture.md | Technical spike confirms the separation is proportionate, or an architecture decision replaces it. |

## Usage rules

- Do not cite an assumption as user, stakeholder, or research authority.
- Repeat the assumption ID wherever the inference materially affects a requirement, decision, design, or test.
- If an assumption becomes approved, update the relevant decision status and retain provenance; do not silently rewrite history.
- If an assumption is rejected, mark it retired and update every listed use.
