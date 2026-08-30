# Working Bounds

## Assumptions
ASM-001: ASSUMPTION: “Single-screen” means one fixed, non-scrolling playfield during a run; this is a non-authoritative interpretation pending implementation review.
ASM-002: ASSUMPTION: “About two minutes” is operationalized as a 120-second default battery timer with a one-second test tolerance; this is a non-authoritative tuning interpretation derived from REQ-008.
ASM-003: ASSUMPTION: “A few seconds” is operationalized as no more than five seconds before essential controls and goals are visible; this is a non-authoritative usability threshold derived from REQ-017.
ASM-004: ASSUMPTION: Godot 4 with GDScript and the Compatibility renderer is a reversible, non-authoritative prototype technology choice inferred from the 2D, desktop, and browser needs.
ASM-005: ASSUMPTION: One fixed physics step and one authoritative trail geometry model will provide sufficiently deterministic collision and scoring behavior for prototype verification; this is a non-authoritative technical inference.
ASM-006: ASSUMPTION: The initiation prototype will use no online service or persistent progression data because longer-term progression is unresolved; this is a non-authoritative scope inference and not a decision against later persistence.

## Constraints
CON-001: The prototype should use keyboard controls. [exact supplied constraint]
CON-002: simple neon shapes. [exact supplied constraint]
CON-003: strong audio feedback. [exact supplied constraint]
CON-004: rather than detailed art. [exact supplied constraint]
CON-005: without needing a tutorial longer than a few seconds. [exact supplied constraint]

## Interpretation Notes

Assumptions authorize planning only. They do not replace stakeholder answers, and each may be revised without changing the preserved wording in [Requirements.md](Requirements.md).
