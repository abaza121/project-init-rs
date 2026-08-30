# Technical Design

## Decisions
DEC-001: STATUS: Accepted; PROVENANCE TYPE: user brief plus non-authoritative interpretation; SOURCES: REQ-001 and ASM-001; CHOICE: use one fixed non-scrolling 2D arcade playfield; DEC-001 supports REQ-001 because the fixed non-scrolling 2D arcade playfield implements the single-screen 2D arcade subject while ASM-001 makes the interpretation visible.
DEC-002: STATUS: Provisional; PROVENANCE TYPE: technical inference from user brief, assumption, and external documentation; SOURCES: REQ-002, ASM-004, EVD-016, and EVD-017; CHOICE: use Godot 4 GDScript with the Compatibility renderer for desktop and browser builds; DEC-002 supports REQ-002 because Godot 4 GDScript and Compatibility can produce both desktop and browser artifacts while avoiding the documented Godot 4 C# web-export limitation.
DEC-003: STATUS: Accepted; PROVENANCE TYPE: user brief; SOURCE: REQ-003; CHOICE: compose the playfield around a player-controlled tiny courier ship, unlit city field, parcels, and towers; DEC-003 supports REQ-003 because the courier ship and unlit city composition directly realizes the blackout courier subject.
DEC-004: STATUS: Provisional; PROVENANCE TYPE: user brief plus technical inference; SOURCES: REQ-004, REQ-006, REQ-007, REQ-012, and ASM-005; CHOICE: store one sampled trail polyline as both glowing render geometry and old-trail collision geometry; DEC-004 supports REQ-004 because sampled glowing trail geometry is added by movement; DEC-004 supports REQ-006 because the same old-trail geometry becomes collidable; DEC-004 supports REQ-007 because retained segments accumulate into a maze; DEC-004 supports REQ-012 because the ship can test collision against old trail geometry before battery expiry.
DEC-005: STATUS: Provisional; PROVENANCE TYPE: design inference from user brief; SOURCE: REQ-005; CHOICE: derive a visible city-power meter and nearby building illumination from accumulated trail length; DEC-005 supports REQ-005 because accumulated glowing trail length visibly drives the city-power subject instead of serving as decoration alone.
DEC-006: STATUS: Provisional; PROVENANCE TYPE: design inference from user brief and external guidance; SOURCES: REQ-009, REQ-010, EVD-005, and ASM-005; CHOICE: model one active parcel state and require exact tower matching by paired color and glyph; DEC-006 supports REQ-009 because the active parcel state records collection; DEC-006 supports REQ-010 because paired parcel and tower color-plus-glyph identifiers make exact matching inspectable.
DEC-007: STATUS: Provisional; PROVENANCE TYPE: design inference from user brief; SOURCE: REQ-011; CHOICE: calculate close passes from minimum ship-to-old-trail distance inside a configurable near-pass band and expose the multiplier on the HUD; DEC-007 supports REQ-011 because measured close-pass distance directly governs preservation of the visible score multiplier.
DEC-008: STATUS: Provisional; PROVENANCE TYPE: user brief plus non-authoritative tuning interpretation; SOURCES: REQ-008, REQ-012, and ASM-002; CHOICE: drive the default run from a visible 120-second battery countdown ending at zero; DEC-008 supports REQ-008 because the 120-second battery countdown operationalizes about two minutes; DEC-008 supports REQ-012 because reaching battery zero provides a separate expiry outcome from old-trail collision.
DEC-009: STATUS: Accepted; PROVENANCE TYPE: user brief constraint plus external guidance; SOURCES: REQ-013, CON-001, and EVD-004; CHOICE: route every start, menu, movement, pause, restart, and exit action through named keyboard input actions; DEC-009 supports REQ-013 because named keyboard actions allow the complete keyboard-control task sequence required by CON-001 and preserve a remapping seam without adopting remapping scope.
DEC-010: STATUS: Accepted; PROVENANCE TYPE: user brief constraints; SOURCES: REQ-014, REQ-015, CON-002, and CON-004; CHOICE: use flat vector geometry, luminous strokes, restrained bloom, and text instead of detailed illustration; DEC-010 supports REQ-014 because geometric luminous primitives implement simple neon shapes from CON-002; DEC-010 supports REQ-015 because omitting detailed illustration enforces the rather-than-detailed-art boundary in CON-004.
DEC-011: STATUS: Provisional; PROVENANCE TYPE: user brief constraint plus external platform evidence; SOURCES: REQ-015, CON-003, EVD-006, and EVD-019; CHOICE: map gameplay events to short pre-rendered samples on separate music, gameplay, and ambience buses and verify browser playback after a key gesture; DEC-011 supports REQ-015 because event samples and separate buses implement strong audio feedback from CON-003 while accounting for browser audio activation and effect limits.
DEC-012: STATUS: Provisional; PROVENANCE TYPE: user brief constraint plus non-authoritative threshold interpretation; SOURCES: REQ-017, CON-005, and ASM-003; CHOICE: show a skippable first-run controls-and-goal overlay immediately and permit movement within five seconds; DEC-012 supports REQ-017 because the five-second skippable overlay objectively bounds the few-seconds tutorial constraint in CON-005.
DEC-013: STATUS: Deferred; PROVENANCE TYPE: supplied stakeholder-status answer constrained by external guidance; SOURCES: REQ-018, ANS-001, EVD-001, EVD-002, and EVD-003; CHOICE: adopt no default difficulty or target challenge until the stakeholder supplies it; DEC-013 supports REQ-018 because player-dependent difficulty guidance cannot replace the stakeholder-owned default challenge identified by ANS-001.
DEC-014: STATUS: Deferred; PROVENANCE TYPE: supplied stakeholder-status answer constrained by external guidance; SOURCES: REQ-019, ANS-002, EVD-004, EVD-005, EVD-006, EVD-007, EVD-008, and EVD-009; CHOICE: adopt no bundled accessibility scope while preserving separable input, redundant-cue, audio, contrast, text, and reduced-effects seams; DEC-014 supports REQ-019 because the breadth of accessibility practices does not authorize a stakeholder priority bundle and ANS-002 explicitly keeps that accessibility scope unresolved.
DEC-015: STATUS: Deferred; PROVENANCE TYPE: supplied stakeholder-status answer plus non-authoritative scope assumption and external frameworks; SOURCES: REQ-020, ANS-003, ASM-006, EVD-010, and EVD-011; CHOICE: implement no persistent progression in the initiation prototype; DEC-015 supports REQ-020 because progression mechanics cannot be selected before the intended long-term player experience in ANS-003, while ASM-006 keeps the no-persistence scope visibly non-authoritative.
DEC-016: STATUS: Deferred; PROVENANCE TYPE: supplied stakeholder-status answer constrained by general guidance; SOURCES: REQ-021, ANS-004, EVD-012, and EVD-013; CHOICE: publish no target-audience claim or demographic tuning until the stakeholder defines or authorizes audience work; DEC-016 supports REQ-021 because general discovery guidance cannot identify the intended audience left unresolved by ANS-004.
DEC-017: STATUS: Deferred; PROVENANCE TYPE: supplied stakeholder-status answer constrained by general guidance; SOURCES: REQ-022, ANS-005, EVD-014, and EVD-015; CHOICE: treat acceptance checks as prototype verification only and adopt no continuation threshold; DEC-017 supports REQ-022 because general success-measure guidance cannot supply the continuation outcomes or thresholds left unresolved by ANS-005.
DEC-018: STATUS: Provisional; PROVENANCE TYPE: delivery inference from user brief and external documentation; SOURCES: REQ-002 and EVD-018; CHOICE: create versioned Godot export presets for one selected native desktop target and HTML5, then launch-test both from the same revision; DEC-018 supports REQ-002 because explicit desktop and HTML5 export artifacts make both requested build targets inspectable.
DEC-019: STATUS: Provisional; PROVENANCE TYPE: design inference from user brief; SOURCE: REQ-016; CHOICE: score deliveries, efficient route length, and measured near-pass precision rather than adding random score bonuses; DEC-019 supports REQ-016 because the scoring inputs are direct observations of route planning and precision.

## Runtime Shape

Use one `Game` scene containing `RunController`, `Playfield`, `Ship`, `TrailSystem`, `ParcelSystem`, `TowerSystem`, `ScoreSystem`, `BatterySystem`, `CityPowerView`, `HUD`, `AudioDirector`, and `PauseOverlay`. The run controller owns the state transition `READY → ACTIVE → ENDED`; it is the only module allowed to begin or end a run.

The simulation uses a fixed physics update. `Ship` consumes named input actions and publishes displacement. `TrailSystem` samples displacement, appends a segment to one authoritative run-local trail model, renders that segment, and adds collision after a configurable grace value. `ScoreSystem` reads deliveries and minimum distance to eligible old trail segments. `BatterySystem` emits remaining time and expiry. Presentation modules subscribe to events and never author gameplay state.

## Data Contracts

| Value | Type | Owner | Verification use |
|---|---|---|---|
| `run_state` | enum | `RunController` | Exactly one legal run phase |
| `battery_seconds` | float | `BatterySystem` | AC-008 and AC-012 timing |
| `trail_segments` | ordered point pairs plus creation tick | `TrailSystem` | AC-004, AC-006, AC-007 |
| `active_parcel_id` | nullable identifier | `ParcelSystem` | AC-009 and AC-010 |
| `tower_match_id` | identifier plus color and glyph | `TowerSystem` | AC-010 |
| `near_pass_distance` | float | `ScoreSystem` | AC-011 and AC-016 |
| `multiplier` | numeric configured value | `ScoreSystem` | AC-011 |
| `city_power` | normalized value derived from trail length | `CityPowerView` | AC-005 |

## State Invariants

- A trail segment is appended only when the ship moves beyond the sample interval.
- Rendered and collidable old-trail geometry derive from the same ordered points.
- Only one parcel can be active or carried at a time in the prototype.
- A tower accepts a delivery only when its match identifier equals the carried parcel identifier.
- Battery time decreases only while the run state is `ACTIVE` and ends the run once.
- Score events are deterministic for the same input trace and configuration.
- No persistent progression data is written while DEC-015 remains deferred.

## Configuration Boundary

Keep timer duration, ship speed, trail sample spacing, collision grace, parcel and tower counts, near-pass band, multiplier behavior, score weights, volumes, and presentation intensity in versioned data. Provisional values must be labeled as such, and values controlled by OQ-001, OQ-002, OQ-007, OQ-008, or OQ-009 must not be described as stakeholder-approved.

## Verification Hooks

Provide a deterministic seed, an input-trace replay mode, a compact event log, a collision-geometry debug view, a mute-safe visual event overlay, and an on-screen build revision. These hooks support objective acceptance inspection without becoming player-facing progression or analytics.

## Failure Handling

Invalid parcel or tower configuration prevents a run from starting and names the mismatched identifier. Unsupported browser audio begins muted until a keyboard gesture activates playback; the HUD remains sufficient to operate the run. Export or launch failure fails AC-002 and does not silently narrow the platform claim.
