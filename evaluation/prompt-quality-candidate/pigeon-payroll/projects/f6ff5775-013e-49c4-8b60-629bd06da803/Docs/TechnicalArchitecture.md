# Technical architecture

Status: **logical prototype architecture, technology-neutral under A-002**. It does not select an engine, language, platform, backend, SDK, store, or production topology.

## Architecture goals

- Make every gameplay requirement observable and testable.
- Isolate unresolved audience, failure, tuning, and behaviour choices behind configuration or strategy boundaries.
- Support one portrait cafe playfield with three table anchors.
- Keep development-only deterministic verification possible under A-004.

## Logical components

| Component | Responsibility | Primary trace |
|---|---|---|
| App shell | Mobile lifecycle, portrait lock, scene entry, pause/resume. | REQ-002; REQ-016 |
| Input interpreter | Convert one active pointer swipe into origin, direction, length, and release event; reject multi-pointer dependency in the primary loop. | REQ-004 |
| Breadcrumb system | Spawn/land breadcrumb targets and emit one authoritative breadcrumb event. | REQ-005 |
| Attraction system | Apply configurable attraction pressure once per breadcrumb event. | REQ-006 |
| Flock coordinator | Route eligible pigeons toward targets while preserving per-pigeon behaviour state. Any coordination beyond mandatory behaviour remains proposed. | REQ-005; REQ-022; OQ-003 |
| Pigeon state machine | Host only approved states and transitions; mandatory capabilities are breadcrumb attraction and impatient food stealing/combo participation. | REQ-012; REQ-019; REQ-022 |
| Cafe scene model | Own one playfield, exactly three table anchors, customers, pastries, and occupancy changes. | REQ-010; REQ-011; REQ-017; REQ-018 |
| Shift director | Track normalized shift progress, enforce a 60-to-90-second configured duration, and schedule escalating events. | REQ-008; REQ-009; A-005 |
| Combo system | Qualify food steals and count successive qualifying events inside a configurable window. | REQ-012; OQ-006 |
| Score ledger | Record gross score, breadcrumb cleanup entries, other authorized adjustments, and final arithmetic. | REQ-007; OQ-002 |
| Failure strategy | Provide an unselected interface for run termination or continuing wage reduction; no default is product authority. | REQ-021; OQ-002 |
| Presentation adapters | Render readable state, comic animation, HUD, and event-specific audio without owning game rules. | REQ-013; REQ-014; REQ-015 |
| Verification harness | Supply deterministic seeds, event capture, configuration snapshots, and requirement-oriented assertions in development builds. | A-004; VER-003; VER-004; VER-005 |

## Authoritative event contracts

| Event | Required fields | Consumers | Invariant |
|---|---|---|---|
| `SwipeReleased` | pointer count, start, end, duration | Input interpreter | Primary action uses one pointer. |
| `BreadcrumbPlaced` | breadcrumb ID, position, sequence number | Attraction system; score ledger; flock coordinator | Exactly one attraction update and one cleanup entry per event. |
| `CustomerMoved` | customer ID, source table ID, target table ID, time | Cafe model; presentation; telemetry | Source and target are distinct valid anchors among exactly three. |
| `PastryAppeared` | pastry ID, table ID, time | Pigeon state machine; presentation | Pastry changes from absent to stealable. |
| `FoodStolen` | pigeon ID, food ID, time, impatience state | Combo system; score ledger; presentation | The stealing pigeon was in an approved eligible state. |
| `ShiftEnded` | elapsed gameplay time, gross score, cleanup bill, final score, end reason | Score ledger; results overlay; telemetry | Final score equals gross score plus authorized adjustments minus cleanup bill. Failure end reason remains unresolved. |

## State and configuration boundaries

- `ShiftConfig`: duration, event budgets, schedule seeds. Duration must remain within AC-008.
- `TuningConfig`: attraction magnitude, impatience threshold, combo window, scoring values. Values remain open under OQ-006.
- `BehaviourRegistry`: explicit state and transition entries. New entries require an approved OQ-003 answer.
- `FailurePolicy`: intentionally unset until OQ-002 closes. A development test may inject a labelled variant, but no variant may ship or be documented as selected.
- `AudienceProfile`: intentionally unset until OQ-001 closes. It must not silently enable data, ad, account, or store features.
- `EvaluationPlan`: intentionally unset until OQ-004 closes.

## Runtime sequence

1. Load a validated prototype configuration and the single cafe playfield.
2. Instantiate exactly three table anchors and the approved character/behaviour registry.
3. Start the shift clock and event director.
4. Interpret each single-pointer swipe, place one breadcrumb, update attraction pressure, and append one cleanup entry.
5. Process customer moves, pastry appearances, pigeon states, steals, and combos as the shift progresses.
6. Apply the authorized shift-end condition. This step is blocked for a product build until OQ-002 closes; test variants must be explicitly labelled.
7. Calculate and display the score ledger, including cleanup subtraction.

## Data, privacy, and remote systems

Under A-001, the prototype architecture includes no accounts, remote persistence, ads, analytics, purchases, or user-generated content. This is a scope-containment assumption, not a production privacy decision. OQ-001 and OQ-008 must close before any such integration is designed.

## Quality gates

- Configuration validation fails closed when audience, failure, behaviour, or evaluation data is falsely marked approved.
- Automated gameplay checks implement VER-003, VER-004, and VER-005 where feasible.
- Scene inspection implements VER-007.
- Perceptual results remain **not evaluated** until VER-006 has an approved protocol through OQ-004.
- Production technology and performance budgets remain blocked by OQ-005.

