# Requirements

**Canonical status:** This is the authoritative requirement register for the initiation package. Source wording is preserved in the Source Clause Register. Product requirements translate those clauses into testable obligations without replacing them. Inferences remain non-authoritative and link to [Assumptions.md](Assumptions.md); unresolved choices link to [OpenQuestions.md](OpenQuestions.md).

## Source Clause Register

| Source ID | Exact source wording | Generated requirement |
|---|---|---|
| SC-001 | “Last Light Courier” | REQ-001 |
| SC-002 | “fast single-screen 2D arcade game” | REQ-002 |
| SC-003 | “for desktop and browser” | REQ-003 |
| SC-004 | “the player pilots a tiny courier ship through a blackout” | REQ-004 |
| SC-005 | “Every movement leaves behind a glowing trail that powers the city” | REQ-005 |
| SC-006 | “also becomes a solid obstacle” | REQ-006 |
| SC-007 | “the player gradually builds the maze they must survive” | REQ-007 |
| SC-008 | “A run should last about two minutes” | REQ-008 |
| SC-009 | “collect a parcel” | REQ-009 |
| SC-010 | “route it to the matching tower” | REQ-010 |
| SC-011 | “preserve a score multiplier by making close passes” | REQ-011 |
| SC-012 | “avoid colliding with old trails before the battery expires” | REQ-012 |
| SC-013 | “The prototype should use keyboard controls” | REQ-013 |
| SC-014 | “simple neon shapes” | REQ-014 |
| SC-015 | “strong audio feedback rather than detailed art” | REQ-015 |
| SC-016 | “rather than detailed art” | REQ-016 |
| SC-017 | “reward route planning and precision” | REQ-017 |
| SC-018 | “without needing a tutorial longer than a few seconds” | REQ-018 |
| SC-019 | “I have not decided how difficulty, accessibility options, or longer-term progression should work.” | REQ-019 |

Every SC row above is an exact excerpt from the supplied user brief. The full brief remains in the supplied project snapshot; these excerpts collectively retain every material clause used by this package.

## Product Requirement Register

| ID | Requirement | Priority | Source | Objective pass/fail acceptance criterion | Exact verification | Decision or constraint |
|---|---|---:|---|---|---|---|
| REQ-001 | The product and visible prototype title shall be Last Light Courier. | Must | SC-001 | **Pass** when the title screen and build metadata both equal “Last Light Courier”; otherwise **fail**. | T-REQ-001: assert title DOM/canvas text and package metadata string. | DEC-001 |
| REQ-002 | The prototype shall be a fast, single-screen, 2D arcade game with no scrolling playfield during a run. | Must | SC-002 | **Pass** when one complete run occurs inside one fixed playfield viewport with 2D rendering and no camera translation, and movement response and speed equal values authorized under OQ-009; otherwise **fail**. Until those values are authorized, the “fast” aspect is **blocked**. | T-REQ-002: automated scene-bounds assertion plus recorded full-run review; T-MOVE-001: input-to-motion latency and speed assertion against approved configuration. | DEC-002 |
| REQ-003 | The prototype shall be playable in a supported desktop browser and shall have a defined desktop distribution path. | Must | SC-003 | **Pass** when the browser build passes T-PLAT-001 and a stakeholder-approved desktop interpretation in OQ-008 passes its recorded verification; **fail** if either approved check fails; until OQ-008 is answered, overall acceptance is **blocked**, not assumed. | T-PLAT-001: production-build smoke test in the approved browser matrix; T-PLAT-002: approved desktop-package or desktop-browser verification. | DEC-006; DEC-013 |
| REQ-004 | The player shall control a visibly tiny courier ship in a blackout-city setting. | Must | SC-004 | **Pass** when a run exposes exactly one player-controlled courier ship, visually smaller than each delivery tower, against a predominantly dark playfield; otherwise **fail**. | T-REQ-004: scene-object assertion and visual review checklist V-IDENT-001. | DEC-002; CON-001 |
| REQ-005 | While the ship moves, it shall leave a persistent glowing trail and the city-power presentation shall visibly increase in response. | Must | SC-005 | **Pass** when a scripted movement adds contiguous trail geometry and increases at least one visible city-power state; stationary input adds no trail length; otherwise **fail**. | T-TRAIL-001: movement/stationary simulation assertions; V-FX-001: captured before/after visual comparison. | DEC-002; DEC-005 |
| REQ-006 | Previously laid trail geometry shall act as a solid collision obstacle. | Must | SC-006 | **Pass** when crossing eligible old-trail geometry triggers the configured collision outcome and cannot be traversed normally; otherwise **fail**. Eligibility timing remains ASS-003 and ASS-007 pending OQ-010. | T-COLL-001: deterministic trail-crossing scenario. | DEC-002; DEC-007; CON-004 |
| REQ-007 | Accumulated trails shall persist during the run so the navigable space progressively becomes a player-created maze. | Must | SC-007 | **Pass** when trail count and blocked-space coverage never decrease during normal active play and a scripted route changes later path availability; otherwise **fail**. | T-MAZE-001: state-history monotonicity assertion and route-availability fixture. | DEC-002; DEC-007 |
| REQ-008 | A normal run shall target about two minutes and end no later than battery expiry. | Must | SC-008 | **Pass** when battery expiry ends an uninterrupted default-config run at the stakeholder-approved target and tolerance; **fail** when an observed run falls outside that authorized rule; until OQ-006 is answered, the provisional 120,000 ms nominal value in ASS-001 may be tested but final acceptance is **blocked**. | T-TIME-001: fake-clock battery-expiry test; V-TIME-001: three real-clock observations. | DEC-003; CON-002 |
| REQ-009 | The ship shall be able to collect an available parcel. | Must | SC-009 | **Pass** when ship/parcel overlap changes the parcel from available to carried exactly once and updates visible state; otherwise **fail**. | T-PARCEL-001: overlap/state-transition test. | DEC-002; DEC-007 |
| REQ-010 | A carried parcel shall be deliverable only to its matching tower. | Must | SC-010 | **Pass** when delivery to the matching tower scores and clears the carried parcel, while contact with each non-matching tower does neither; otherwise **fail**. | T-DELIVERY-001: matching and non-matching tower parameterized test. | DEC-002; DEC-007; CON-005 |
| REQ-011 | Close passes shall preserve a score multiplier. | Must | SC-011 | **Pass** when an eligible close-pass event refreshes or preserves the multiplier window without collision and an otherwise identical route without the event allows the multiplier to decay; otherwise **fail**. Numeric distance and window remain blocked by OQ-014. | T-MULT-001: paired deterministic close-pass and no-close-pass simulations. | DEC-004; CON-003 |
| REQ-012 | The player shall avoid old-trail collisions while battery time remains, and battery expiry shall terminate the run. | Must | SC-012 | **Pass** when old-trail collision and battery expiry each produce the authorized run outcome, prevent further score-changing movement, and show an end-state cue; **fail** if either path violates the authorized outcome; exact collision outcome is blocked by OQ-010. | T-END-001: collision and expiry end-state scenarios. | DEC-003; DEC-004; CON-004 |
| REQ-013 | All prototype gameplay shall be operable with keyboard input. | Must | SC-013 | **Pass** when every action required to start, play, pause/resume, and restart a run is completable using only documented keyboard commands in the approved browser matrix; otherwise **fail**. Specific bindings and remapping remain OQ-009 and OQ-002. | T-INPUT-001: keyboard-only end-to-end script; V-INPUT-001: manual focus-loss and restart check. | DEC-004; CON-006 |
| REQ-014 | Gameplay entities and effects shall use simple neon geometric forms. | Should | SC-014 | **Pass** when the ship, parcels, towers, trails, and critical HUD states are legible using primitives, lines, text, and generated effects without required detailed illustration; otherwise **fail**. | V-IDENT-001: asset and scene visual audit. | DEC-005; CON-001 |
| REQ-015 | Collection, correct delivery, close pass, multiplier change, collision, low battery, and run end shall each have strong, distinguishable audio feedback. | Should | SC-015 | **Pass** when each named event emits an assigned cue and blinded A/B inspection distinguishes each cue pair used for different critical meanings; otherwise **fail**. Accessibility scope and audio controls remain OQ-002 and OQ-011. | T-AUDIO-001: event-to-cue mapping test; V-AUDIO-001: cue differentiation checklist. | DEC-014; CON-007 |
| REQ-016 | Detailed art production shall not be required for the prototype. | Must | SC-016 | **Pass** when the production asset manifest contains no required hand-painted, high-detail, or frame-by-frame illustrated gameplay asset; otherwise **fail**. | T-ASSET-001: asset-manifest review. | DEC-005; CON-001 |
| REQ-017 | The scoring and obstacle systems shall reward both route planning and precise close-pass execution. | Must | SC-017 | **Pass** when (a) a planned valid delivery route scores above a route that fails delivery and (b) a collision-free eligible close-pass route yields a higher retained multiplier than its paired non-close route; otherwise **fail**. No claim about player enjoyment is implied. | T-DESIGN-001: paired route fixtures; T-MULT-001. | DEC-002; DEC-004; CON-003 |
| REQ-018 | Required onboarding shall take no longer than “a few seconds.” | Must | SC-018 | **Pass** when a first-run timed test completes all mandatory instruction before the stakeholder-authorized maximum in OQ-007 and no undisclosed required instruction appears later; **fail** if the authorized maximum is exceeded or later required instruction is discovered; until that threshold is authorized, acceptance is **blocked**. | V-ONBOARD-001: stopwatch protocol and instruction inventory. | DEC-012; CON-008 |
| REQ-019 | Difficulty, accessibility options, and longer-term progression shall remain explicitly unresolved until separately authorized by the stakeholder. | Must | SC-019 | **Pass** when no document or prototype baseline represents any of the three topics as approved and each has its own open question; otherwise **fail**. | T-GOV-001: text and ID audit for OQ-001, OQ-002, OQ-003, DEC-008, DEC-009, and DEC-010. | DEC-008; DEC-009; DEC-010 |

## Imported Governance Requirements

These five records originate in the supplied project snapshot. They are retained as governance constraints, not treated as authority to select product behavior. The accidental doubled terminal punctuation in the imported statements is preserved inside the quotations.

Because the snapshot’s imported requirement display IDs collide with this package’s product IDs, the package uses explicit aliases: CTX-REQ-001 means snapshot display ID REQ-001 with UUID 9141ce3e-c5b1-4dda-9907-7649950c16fb; CTX-REQ-002 means snapshot display ID REQ-002 with UUID 9cb435a4-89ec-4aed-95db-9b149642f118; CTX-REQ-003 means snapshot display ID REQ-003 with UUID b5b78448-7f98-4e71-b374-cc543be912cb; CTX-REQ-004 means snapshot display ID REQ-004 with UUID c450e0ee-66f8-40ef-84b3-bf49056e1a47; and CTX-REQ-005 means snapshot display ID REQ-005 with UUID 89015214-e110-48dd-b714-01a9a8b9ed6f.

| ID | Exact imported statement | Source | Objective pass/fail acceptance criterion | Exact verification |
|---|---|---|---|---|
| GOV-001 | “The clarified project choice must be FAIL — The intended difficulty remains unresolved. The project brief explicitly leaves difficulty undecided and does not specify the target audience or desired challenge experience. External guidance can inform later design and testing, but it cannot authorize this stakeholder-owned product choice..” | CTX-REQ-001 and ANS-001 | **Pass** if difficulty remains deferred to OQ-001 and external evidence is used only to inform; otherwise **fail**. | T-GOV-002: DEC-008 authority/provenance audit. |
| GOV-002 | “The clarified project choice must be FAIL — The prototype’s accessibility options remain unresolved. The cited guidance supports considering remappable controls, keyboard-operable menus, redundant sensory cues, separate audio controls, reduced nonessential motion effects, and removal of hazardous flashing, but it does not authorize adopting that collection as the project’s settled accessibility scope. The stakeholder must select or authorize these independently; challenge-altering assists also remain linked to the unresolved difficulty experience..” | CTX-REQ-002 and ANS-002 | **Pass** if each accessibility topic is recommendation-only pending OQ-002 and OQ-011 and no challenge assist is approved implicitly; otherwise **fail**. | T-GOV-003: DEC-009 and document-language audit. |
| GOV-003 | “The clarified project choice must be FAIL — Longer-term progression remains unresolved. The brief expressly leaves it undecided and provides neither an intended audience nor a progression goal that would justify selecting persistent rewards, unlocks, power growth, or no metaprogression. External research can inform later evaluation, but it cannot authorize this stakeholder-owned product choice..” | CTX-REQ-003 and ANS-003 | **Pass** if progression remains deferred to OQ-003 with no selected model; otherwise **fail**. | T-GOV-004: DEC-010 and feature-scope audit. |
| GOV-004 | “The clarified project choice must be FAIL — The intended audience remains unresolved. The project context supplies no project-specific evidence or stakeholder direction that would responsibly select an age group, skill level, or degree of arcade-game familiarity. External guidance supports researching likely users, but cannot choose the audience on the stakeholder’s behalf..” | CTX-REQ-004 and ANS-004 | **Pass** if audience remains deferred to OQ-004 and personas are not presented as confirmed; otherwise **fail**. | T-GOV-005: DEC-011 and audience-language audit. |
| GOV-005 | “The clarified project choice must be FAIL — Additional measurable success criteria remain unresolved. The project does not specify what the prototype must demonstrate or authorize project-specific acceptance thresholds. External guidance identifies possible measurement methods, but it cannot determine stakeholder-owned outcomes or pass/fail values..” | CTX-REQ-005 and ANS-005 | **Pass** if additional outcome thresholds remain deferred to OQ-005 and research metrics are recommendation-only; otherwise **fail**. | T-GOV-006: DEC-012 and success-claim audit. |

## Verification Reference Definitions

All T- references are exact planned automated or static checks. All V- references are exact planned human verification protocols. “Planned” means not yet performed; [ValidationReport.md](ValidationReport.md) records only package-level checks actually performed in this initiation session.

- **T-PLAT-001:** Build the production web bundle, serve it over HTTP, start a run, move, collect, deliver, collide, expire, and restart in every stakeholder-approved browser/version.
- **T-PLAT-002:** Execute the same smoke path using the stakeholder-approved meaning of desktop distribution.
- **Register-row checks:** The precise assertion for every T- reference is stated in its corresponding register row above; implementation shall give each reference a test file or checklist entry with the identical ID.
- **V-IDENT-001:** At the reference viewport and each approved scaled viewport, inspect entity hierarchy, darkness, neon geometry, text, and absence of detailed required art.
- **V-FX-001:** Capture stationary, moving, and powered-city frames and compare against REQ-005.
- **V-TIME-001:** Observe three uninterrupted default runs with a monotonic real clock and record expiry times.
- **V-INPUT-001:** Complete one entire run keyboard-only, including tab/window focus loss and restart.
- **V-AUDIO-001:** Trigger each critical event in isolation, record its cue, and have a reviewer identify the event without visuals.
- **V-ONBOARD-001:** From first interactive frame, time only mandatory instruction until free play; inventory every later instruction that gates required action.

## Change Control

A requirement changes only when its row changes here. Any change must update its exact source mapping, decision or constraint link, verification reference, [Traceability.md](Traceability.md), and the session log. Stakeholder answers update an open question and decision before they authorize product behavior.
