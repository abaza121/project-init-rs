# Technical Design

## Decisions

DEC-001: STATUS: active boundary; PROVENANCE TYPE: supplied answer plus externally verified constraint; EXACT AUTHORITY: ANS-001; RELEVANT EVD: EVD-001 on Google child-inclusive audience policy; RELEVANT EVD: EVD-002 on Apple’s Kids category; RELEVANT EVD: EVD-003 on FTC child-directed factors; the intended audience remains unselected supports REQ-001 because platform and regulatory audience constraints do not supply stakeholder product-positioning authority.
DEC-002: STATUS: active boundary with proposed experiment; PROVENANCE TYPE: supplied answer plus non-authoritative inference; EXACT AUTHORITY: ANS-002; EXACT INFERENCE: ASM-001; RELEVANT EVD: EVD-004 on recoverable game failure; RELEVANT EVD: EVD-005 on forgiving interaction recovery; the failure rule remains unselected supports REQ-002 because recoverable-failure guidance can motivate testing wage reduction but cannot choose whether failure ends a run.
DEC-003: STATUS: active boundary; PROVENANCE TYPE: supplied answer plus method guidance; EXACT AUTHORITY: ANS-003; RELEVANT EVD: EVD-006 on goal-to-metric mapping; RELEVANT EVD: EVD-007 on defining learning objectives; RELEVANT EVD: EVD-008 on intended-outcome metrics; measurable prototype pass/fail criteria remain unset supports REQ-003 because a stakeholder learning goal and acceptable threshold must precede a prototype success test.
DEC-004: STATUS: active boundary; PROVENANCE TYPE: supplied answer plus method guidance; EXACT AUTHORITY: ANS-004; RELEVANT EVD: EVD-009 on minimum complexity for risky assumptions; RELEVANT EVD: EVD-010 on question-led prototyping; exact pigeon behaviour membership remains unselected supports REQ-004 because the gameplay question or riskiest assumption must determine which pigeon behaviours are necessary.
DEC-005: STATUS: active boundary; PROVENANCE TYPE: supplied answer plus empirical-tuning guidance; EXACT AUTHORITY: ANS-005; EXACT INFERENCE: ASM-003; RELEVANT EVD: EVD-011 on design-goal-led parameter tuning; RELEVANT EVD: EVD-012 on score distributions and player skill; exact scoring and cleanup-bill values remain unset supports REQ-005 because intended player behaviour and target score outcomes must precede numeric tuning.
DEC-006: STATUS: active boundary; PROVENANCE TYPE: supplied answer plus current platform constraints; EXACT AUTHORITY: ANS-006; RELEVANT EVD: EVD-013 on Xcode macOS requirements; RELEVANT EVD: EVD-014 on Android Studio host support; RELEVANT EVD: EVD-015 on Apple distribution membership cost; RELEVANT EVD: EVD-016 on Android device variability; the primary mobile platform remains unselected supports REQ-006 because tooling, cost, and compatibility facts cannot choose the stakeholder’s market, budget, hardware, or testers.
DEC-007: STATUS: accepted; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-024; the Pigeon Payroll product name supports REQ-024 because the supplied project identity is exactly Pigeon Payroll.
DEC-008: STATUS: accepted; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-007; the comedic mobile arcade genre supports REQ-007 because comedy, mobile use, and arcade pacing are explicit product attributes.
DEC-009: STATUS: accepted; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-008; the cafe-worker protection objective supports REQ-008 because the cafe worker must keep pigeons away from outdoor customers.
DEC-010: STATUS: accepted; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-009; a swipe-to-breadcrumb command supports REQ-009 because one-finger swipes must both throw breadcrumbs and direct flock herding.
DEC-011: STATUS: accepted; PROVENANCE TYPE: user brief constraint; EXACT AUTHORITY: REQ-010; EXACT CONSTRAINT: CON-001; the breadcrumb attraction consequence supports REQ-010 because every thrown breadcrumb must attract more pigeons.
DEC-012: STATUS: accepted; PROVENANCE TYPE: user brief constraint; EXACT AUTHORITY: REQ-011; EXACT CONSTRAINT: CON-002; the cleanup-bill score deduction supports REQ-011 because each breadcrumb cost must be subtracted from gross score at shift completion.
DEC-013: STATUS: accepted; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-012; the configurable 60-to-90-second shift timer supports REQ-012 because every completed shift must end within that inclusive duration window.
DEC-014: STATUS: accepted; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-013; the elapsed-time chaos schedule supports REQ-013 because table moves, pastry appearances, and impatient-bird food-steal combos must increase visible chaos later in the shift.
DEC-015: STATUS: accepted with provisional styling; PROVENANCE TYPE: user brief plus non-authoritative creative inference; EXACT AUTHORITY: REQ-018; EXACT INFERENCE: ASM-005; bold silhouette-first character rendering supports REQ-018 because cafe workers, customers, and pigeons must remain readable at portrait gameplay scale.
DEC-016: STATUS: accepted with provisional sound selection; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-019; event-bound slapstick audio supports REQ-019 because breadcrumb throws, food thefts, and combos require distinct comic sound effects.
DEC-017: STATUS: accepted; PROVENANCE TYPE: user brief; EXACT AUTHORITY: REQ-020; a portrait-locked gameplay viewport supports REQ-020 because the play field and touch path must operate without landscape rotation.
DEC-018: STATUS: accepted; PROVENANCE TYPE: user brief constraint; EXACT AUTHORITY: REQ-021; EXACT CONSTRAINT: CON-004; a single cafe gameplay scene supports REQ-021 because the first prototype is limited to exactly one cafe screen.
DEC-019: STATUS: accepted; PROVENANCE TYPE: user brief constraint; EXACT AUTHORITY: REQ-022; EXACT CONSTRAINT: CON-005; the three-customer-table entity choice supports REQ-022 because the single cafe screen must contain exactly three customer tables.
DEC-020: STATUS: accepted scope with unresolved membership; PROVENANCE TYPE: user brief constraint plus supplied answer; EXACT AUTHORITY: REQ-023; EXACT CONSTRAINT: CON-006; EXACT BOUNDARY: ANS-004; a finite small pigeon behaviour collection supports REQ-023 because prototype scope requires a small set while its exact behaviour membership remains stakeholder-owned.
DEC-021: STATUS: proposed and replaceable; PROVENANCE TYPE: non-authoritative implementation inference; EXACT INFERENCE: ASM-002; a platform-neutral gameplay core supports REQ-006 because replaceable input, audio, storage, and lifecycle adapters preserve the unselected primary mobile platform boundary.

## System Shape

The prototype is a single local gameplay scene divided into six replaceable areas:

1. `InputAdapter` converts one active pointer’s swipe path into a normalized throw vector.
2. `ShiftController` owns the ready, active, resolving, and result states plus a configurable duration constrained to 60–90 seconds.
3. `CafeWorld` owns the worker, three tables, customers, pastries, breadcrumbs, and pigeon entities.
4. `FlockDirector` receives breadcrumb attraction events, updates pigeon state, and publishes food-theft and combo events; its exact behaviour catalogue is intentionally unset.
5. `ScoreLedger` records gross earnings, breadcrumb cleanup charges, theft effects, and combos as separate entries, then exposes `finalScore = grossScore - cleanupBill`; all numeric values are unset parameters.
6. `PresentationLayer` renders portrait-safe visuals, event-bound slapstick audio, timer, combo state, gross score, cleanup bill, and result breakdown.

## State and Event Flow

`Ready → Active → Resolving → Result` is the only committed shift lifecycle. OQ-002 prevents adding a terminal failure transition. During `Active`, a swipe emits `BreadcrumbThrown`; that event creates a breadcrumb, increases the cleanup ledger, and supplies an attraction target to the flock. Timed escalation emits table-move and pastry-appearance opportunities. Pigeon food theft emits combo and gross-score effects. `Resolving` freezes new input, completes the ledger calculation, and displays the result.

## Replaceable Seams

- Platform adapters: touch, orientation, audio focus, app lifecycle, local storage, and build packaging.
- Tuning data: shift duration, attraction response, event timings, reward values, cleanup cost, and combo multipliers.
- Behaviour catalogue: registered pigeon states selected only after OQ-004 closes.
- Failure policy: a strategy slot with no selected implementation until OQ-002 closes.
- Telemetry: an interface with a no-op local implementation under ASM-004; analytics collection is out of scope unless separately authorized.

## Prototype Test Surfaces

Deterministic seeds and an event log should expose entity counts, timestamps, state transitions, attraction changes, ledger entries, and audio triggers required by the gameplay acceptance checks. This observability supports feature conformance only; it does not create the stakeholder-owned success threshold in OQ-003.
