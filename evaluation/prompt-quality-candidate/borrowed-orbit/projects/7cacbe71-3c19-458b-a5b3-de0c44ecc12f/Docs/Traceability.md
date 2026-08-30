# Traceability

This file is the canonical register for consequential decisions and research evidence. Exact edge tables follow the registers; each row represents one meaningful relationship. Comma-separated IDs in a decision record enumerate provenance, but do not replace those edge rows.

## Decision register

### D-001 — Browser and one-screen product format

- Status: accepted.
- Provenance type: user authority.
- Exact source IDs: SC-001, SC-015, CON-003.
- Relevant evidence IDs: none; no external evidence is needed for this user-owned format.
- Rationale: the prototype is explicitly a browser game and active play is explicitly limited to one screen.
- Governed requirements: R-001, R-016.

### D-002 — One-action attach/release control semantics

- Status: accepted.
- Provenance type: user authority.
- Exact source IDs: SC-001, SC-003, SC-004.
- Relevant evidence IDs: none; accessibility variants are a separate D-009 subject.
- Rationale: the brief directly assigns nearest eligible attachment to hold and momentum-preserving detach to release.
- Governed requirements: R-002, R-004, R-005.

### D-003 — Core run scoring and pressure loop

- Status: accepted at feature level; numerical tuning open.
- Provenance type: user authority, with non-authoritative tuning assumption A-003.
- Exact source IDs: SC-002, SC-005, SC-006, SC-007, SC-008.
- Relevant evidence IDs: none.
- Rationale: the brief explicitly names moving gravity objects, fragments, hazard skims, the multiplier, inward drift, and the collapsing star. It does not choose values.
- Governed requirements: R-003, R-006, R-007, R-008, R-009.

### D-004 — Run cap and restart loop

- Status: accepted for duration/restart intent; restart latency proposed.
- Provenance type: user authority plus non-authoritative operational inference A-001.
- Exact source IDs: SC-009, SC-010, CON-001.
- Relevant evidence IDs: none.
- Rationale: the three-minute bound is explicit. “Instantly” needs a test threshold, so A-001 remains replaceable.
- Governed requirements: R-010, R-011.

### D-005 — Instruction-free audiovisual gameplay communication

- Status: accepted at intent level.
- Provenance type: user authority.
- Exact source IDs: SC-011.
- Relevant evidence IDs: none.
- Rationale: the brief requires understanding through animation and sound instead of text.
- Governed requirements: R-012.

### D-006 — Sparse geometric, solo-producible art direction

- Status: accepted at constraint level; palette and recipes proposed.
- Provenance type: user authority.
- Exact source IDs: SC-012, SC-013, CON-002.
- Relevant evidence IDs: none.
- Rationale: the brief explicitly bounds both style and production model.
- Governed requirements: R-013, R-014.

### D-007 — Movement-proof evaluation protocol

- Status: pending stakeholder decision.
- Provenance type: unresolved stakeholder choice constrained by research.
- Exact source IDs: SC-014, UNK-006, ANS-006.
- Relevant evidence IDs: EVD-014, EVD-015, EVD-016.
- Rationale: evidence informs useful evaluation fields but supplies no project-specific participants, tasks, thresholds, or confidence rule.
- Governed requirements: R-015.

### D-008 — Candidate physics model

- Status: proposed; not accepted.
- Provenance type: design inference/recommendation constrained by research.
- Exact source IDs: SC-016, UNK-001, ANS-001.
- Relevant evidence IDs: EVD-001, EVD-002, EVD-003.
- Rationale: the imported answer proposes a 2D, single-active-anchor inverse-square point-mass model for evaluation. The evidence supports abstractions, not game feel or adoption; OQ-001 and OQ-011 remain open.
- Governed requirements: R-005, R-017.

### D-009 — Input-accessibility scope

- Status: pending stakeholder decisions.
- Provenance type: unresolved stakeholder choice constrained by research.
- Exact source IDs: SC-017, UNK-002, ANS-002.
- Relevant evidence IDs: EVD-004, EVD-005, EVD-006.
- Rationale: the imported answer rejected a bundled baseline. Modalities, remapping, Hold/Toggle, and non-gameplay controls require separate decisions.
- Governed requirements: R-018.

### D-010 — Low-end profile and frame-rate acceptance

- Status: pending stakeholder decisions.
- Provenance type: unresolved stakeholder choice constrained by research.
- Exact source IDs: SC-018, UNK-003, UNK-007, ANS-003, ANS-007.
- Relevant evidence IDs: EVD-007, EVD-008, EVD-009, EVD-010, EVD-017, EVD-018, EVD-019.
- Rationale: browser timing/device evidence informs methodology, but cannot choose supported hardware or a numeric performance target.
- Governed requirements: R-019, R-020.

### D-011 — Player-facing run randomness

- Status: pending stakeholder decision.
- Provenance type: unresolved stakeholder choice constrained by research.
- Exact source IDs: SC-019, UNK-004, ANS-004.
- Relevant evidence IDs: EVD-011, EVD-012.
- Rationale: evidence identifies a variation/controllability tradeoff, not the appropriate amount or scope for this game.
- Governed requirements: R-021.

### D-012 — Target audience

- Status: pending stakeholder decision or project-specific research validation.
- Provenance type: unresolved stakeholder choice constrained by research.
- Exact source IDs: UNK-005, ANS-005.
- Relevant evidence IDs: EVD-013.
- Rationale: the product traits do not establish an audience; evidence supports research practice rather than selecting users.
- Governed requirements: none directly; D-012 is a prerequisite for D-007 and informs D-009, D-010, and D-011.

### D-013 — Modular local browser architecture

- Status: proposed; not accepted.
- Provenance type: design inference.
- Exact source IDs: SC-001, SC-013, SC-015.
- Relevant evidence IDs: none.
- Rationale: separable input, simulation, scoring, rendering/audio, configuration, and local fixtures may support solo iteration and exact tests without creating remote-service scope. This is A-006, not a brief selection.
- Governed requirements: supports verification across the complete canonical requirement register but does not change any requirement's authority.

### D-014 — Visual-primary cues with redundant sound

- Status: proposed; not accepted.
- Provenance type: accessibility/design inference constrained by research.
- Exact source IDs: SC-011, ANS-002.
- Relevant evidence IDs: EVD-004, EVD-005, EVD-006.
- Rationale: A-002 avoids a sound-only mechanical dependency while preserving the requested use of animation and sound. Stakeholders must approve or replace it through OQ-002, OQ-003, and OQ-004.
- Governed requirements: R-012.

## Canonical evidence register

All evidence records below were imported from the supplied project snapshot with retrieval timestamps of 2026-08-30. Their URLs were not independently opened during this package-generation session. “Reliability” reproduces the imported assessment; it is not a fresh source audit.

### EVD-001

- Canonical claim: Gravity toward a central body follows an inverse-square relationship with distance.
- Direct source: [NASA Science — Chapter 3: Gravity & Mechanics](https://science.nasa.gov/learn/basics-of-space-flight/chapter3-3/)
- Imported reliability/note: high; supports the force family, not game-specific suitability or tuning.
- Informs: D-008.

### EVD-002

- Canonical claim: A patched-conic approximation treats a multi-body trajectory as two-body segments dominated by one body while neglecting other-body perturbations.
- Direct source: [NASA Technical Reports Server — The Patched-conic Approximation (PDF)](https://ntrs.nasa.gov/api/citations/20070014643/downloads/20070014643.pdf)
- Imported reliability/note: high; player-selected anchors are a design adaptation.
- Informs: D-008.

### EVD-003

- Canonical claim: A two-body model can propagate a negligible-mass object’s position and velocity around one central mass using that body’s gravitational parameter.
- Direct source: [NASA/JPL NAIF — prop2b_c](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/prop2b_c.html)
- Imported reliability/note: high; supports an abstraction, not adoption.
- Informs: D-008.

### EVD-004

- Canonical claim: WCAG 2.2 Success Criterion 2.1.1 requires web-content functionality to be operable through a keyboard interface without individual-keystroke timing requirements, subject to its path-dependent exception.
- Direct source: [W3C — WCAG 2.2, Keyboard](https://www.w3.org/TR/WCAG22/#keyboard)
- Imported reliability/note: high; does not select bindings, remapping, Toggle mode, or complete supported-device scope.
- Informs: D-009, D-014.

### EVD-005

- Canonical claim: W3C input-modality guidance discusses supporting pointer and keyboard mechanisms concurrently.
- Direct source: [W3C WAI — Understanding Guideline 2.5: Input Modalities](https://www.w3.org/WAI/WCAG22/Understanding/input-modalities.html)
- Imported reliability/note: high; informative guidance, not project authorization.
- Informs: D-009, D-014.

### EVD-006

- Canonical claim: Microsoft game-accessibility guidance identifies sustained input as a potential barrier and recommends considering remapping, toggles/auto-holds, and alternatives to prolonged, repeated, or simultaneous inputs.
- Direct source: [Microsoft Game Dev — Xbox Accessibility Guideline 107: Input](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/107)
- Imported reliability/note: high; recommendations, not settled Borrowed Orbit requirements.
- Informs: D-009, D-014.

### EVD-007

- Canonical claim: `requestAnimationFrame` callbacks generally follow display refresh rate; 60 Hz is common but not universal.
- Direct source: [MDN — Window: requestAnimationFrame()](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)
- Imported reliability/note: high; context only, not a project target.
- Informs: D-010.

### EVD-008

- Canonical claim: A 60 Hz display has about 16.66 ms per frame, while application work must leave time for browser rendering overhead.
- Direct source: [web.dev — Rendering performance](https://web.dev/articles/rendering-performance)
- Imported reliability/note: high; illustrative frame budget, not an authorized requirement.
- Informs: D-010.

### EVD-009

- Canonical claim: Chrome DevTools provides calibrated throttling presets intended to approximate typical low- and mid-tier mobile CPU performance.
- Direct source: [Chrome for Developers — Throttling](https://developer.chrome.com/docs/devtools/settings/throttling/)
- Imported reliability/note: high; possible method after a profile is defined.
- Informs: D-010.

### EVD-010

- Canonical claim: Chrome DevTools CPU throttling is relative to the development computer and cannot fully simulate mobile CPUs because architectures differ.
- Direct source: [Chrome for Developers — Performance features reference](https://developer.chrome.com/docs/devtools/performance/reference)
- Imported reliability/note: high; supports an emulation limitation.
- Informs: D-010.

### EVD-011

- Canonical claim: Procedural-content-generation research characterizes stochasticity as providing variation at a cost to controllability.
- Direct source: [Artificial Intelligence and Games — Chapter 4 (PDF)](https://www.gameaibook.org/book.pdf)
- Imported reliability/note: high; does not prescribe an amount for this project.
- Informs: D-011.

### EVD-012

- Canonical claim: Research on randomness and skill across games states that an appropriate randomness level depends on design goals and target audience.
- Direct source: [Seeding for Success: Skill and Stochasticity in Tabletop Games (PDF)](https://tabletopgames.ai/assets/pdf/Goodman2025SeedingForSuccess.pdf)
- Imported reliability/note: medium; tabletop findings do not establish a numeric browser-game value.
- Informs: D-011.

### EVD-013

- Canonical claim: User research should identify likely users and goals using evidence, while unsupported user-need suggestions should remain assumptions requiring validation.
- Direct source: [GOV.UK Service Manual — Learning about users and their needs](https://www.gov.uk/service-manual/user-research/start-by-learning-user-needs)
- Imported reliability/note: high; supports research, not an audience selection.
- Informs: D-012.

### EVD-014

- Canonical claim: ISO terminology frames evaluation against specified criteria and user-based evaluation around representative users performing tasks to measure qualities such as effectiveness and efficiency.
- Direct source: [ISO Online Browsing Platform — ISO 9241-220 terminology](https://www.iso.org/obp/ui?_escaped_fragment_=iso:std:iso:9241:-220:ed-1:v1:en)
- Imported reliability/note: high; supplies no Borrowed Orbit thresholds.
- Informs: D-007.

### EVD-015

- Canonical claim: GOV.UK usability-benchmarking guidance recommends stable tasks with clear correct answers and measures including task success, completion time, abandonment, confidence, and average completion rates.
- Direct source: [GOV.UK Service Manual — Usability benchmarking](https://www.gov.uk/service-manual/measuring-success/usability-benchmarking-a-website-or-whole-service)
- Imported reliability/note: high; supports candidate measures, not a pass threshold.
- Informs: D-007.

### EVD-016

- Canonical claim: NASA human-integration guidance describes practice-related changes in response time, error, and workload/resource demand, often with larger improvements early in training.
- Direct source: [NASA Human Integration Design Handbook, Revision 1 (PDF)](https://www.nasa.gov/wp-content/uploads/2023/03/human-integration-design-handbook-revision-1.pdf)
- Imported reliability/note: high; does not establish required improvement or trial count.
- Informs: D-007.

### EVD-017

- Canonical claim: W3C device-testing guidance says to determine the target device range and document minimum expected test requirements, including relevant device limitations.
- Direct source: [W3C — Guidelines for writing device independent tests](https://www.w3.org/TR/di-testing/)
- Imported reliability/note: medium; supports profile documentation, not profile selection.
- Informs: D-010.

### EVD-018

- Canonical claim: Chrome Device Mode is an approximation rather than execution on mobile hardware; its low-end preset applies slowdown relative to the development computer.
- Direct source: [Chrome for Developers — Simulate mobile devices with Device Mode](https://developer.chrome.com/docs/devtools/device-mode)
- Imported reliability/note: high; not a universal physical-device definition.
- Informs: D-010.

### EVD-019

- Canonical claim: Chrome states CPU throttling does not simulate all mobile-hardware characteristics and recommends testing on real mobile devices.
- Direct source: [Chrome for Developers — Real-world performance debugging](https://developer.chrome.com/blog/devtools-grounded-real-world)
- Imported reliability/note: high; supports real-device validation, not a device-floor choice.
- Informs: D-010.

## Exact source-clause → requirement edges

| Source | Requirement | Relationship and relevance |
|---|---|---|
| SC-001 | R-001 | Directly specifies browser-game delivery. |
| SC-001 | R-002 | Directly specifies minimalist one-button play. |
| SC-002 | R-003 | Directly specifies satellite survival through moving-object gravity. |
| SC-003 | R-004 | Directly specifies hold-to-nearest eligible attachment. |
| SC-004 | R-005 | Directly specifies momentum-preserving release/new arc. |
| SC-005 | R-006 | Directly specifies data-fragment collection goal. |
| SC-006 | R-007 | Directly specifies hazard-skimming bonus. |
| SC-007 | R-008 | Directly specifies orbit-multiplier continuity. |
| SC-008 | R-009 | Directly specifies object drift toward collapsing star. |
| SC-009 | R-010 | Directly specifies less-than-three-minute runs. |
| SC-010 | R-011 | Directly specifies instant restart intent. |
| SC-011 | R-012 | Directly specifies animation/sound rather than text for understanding. |
| SC-012 | R-013 | Directly specifies sparse geometric space art. |
| SC-013 | R-014 | Directly specifies solo-developer producibility. |
| SC-014 | R-015 | Directly specifies prototype movement-proof outcome. |
| SC-015 | R-016 | Directly specifies one-screen play. |
| SC-016 | R-017 | Directly preserves physics as unchosen pending approval. |
| SC-017 | R-018 | Directly preserves accessibility features as unchosen. |
| SC-018 | R-019 | Low-end phrase requires a reference-profile gate. |
| SC-018 | R-020 | Directly preserves target frame rate as unchosen. |
| SC-019 | R-021 | Directly preserves run randomness as unchosen. |

## Exact constraint → requirement edges

| Constraint | Requirement | Relationship and relevance |
|---|---|---|
| CON-001 | R-010 | Binding run-duration constraint. |
| CON-002 | R-014 | Binding production-scope constraint. |
| CON-003 | R-016 | Binding playfield-layout constraint. |

## Exact requirement → decision edges

| Requirement | Decision | Relationship and relevance |
|---|---|---|
| R-001 | D-001 | Product format governs browser delivery. |
| R-002 | D-002 | Control semantics govern the one-action command set. |
| R-003 | D-003 | Core loop governs gravity survival feature. |
| R-004 | D-002 | Control semantics govern hold/nearest selection. |
| R-005 | D-002 | Control semantics govern release. |
| R-005 | D-008 | Candidate force-model boundary affects continuity verification. |
| R-006 | D-003 | Core loop governs fragments. |
| R-007 | D-003 | Core loop governs hazard skims. |
| R-008 | D-003 | Core loop governs multiplier. |
| R-009 | D-003 | Core loop governs inward pressure. |
| R-010 | D-004 | Session decision governs run cap. |
| R-011 | D-004 | Session decision governs restart. |
| R-012 | D-005 | User-authorized communication intent governs non-textual cues. |
| R-012 | D-014 | Proposed redundancy policy affects muted operation. |
| R-013 | D-006 | Art-direction decision governs sparse geometry. |
| R-014 | D-006 | Production-direction decision governs solo scope. |
| R-015 | D-007 | Pending proof protocol governs success claim. |
| R-016 | D-001 | Product format governs one-screen play. |
| R-017 | D-008 | Proposed physics decision is the subject of the gate. |
| R-018 | D-009 | Pending accessibility decisions are the subject of the gate. |
| R-019 | D-010 | Pending device profile is the subject of the gate. |
| R-020 | D-010 | Pending frame target/method is the subject of the gate. |
| R-021 | D-011 | Pending randomness policy is the subject of the gate. |

## Exact evidence → decision edges

| Evidence | Decision | Relationship and relevance |
|---|---|---|
| EVD-001 | D-008 | Constrains the inverse-square force-family proposal. |
| EVD-002 | D-008 | Constrains the single-dominant-attractor abstraction. |
| EVD-003 | D-008 | Constrains the point-mass/two-body abstraction. |
| EVD-004 | D-009 | Constrains keyboard-operability consideration. |
| EVD-004 | D-014 | Constrains visual-primary interaction accessibility. |
| EVD-005 | D-009 | Constrains multi-modality consideration. |
| EVD-005 | D-014 | Constrains modality-independent cue thinking. |
| EVD-006 | D-009 | Constrains sustained-input/remapping/Toggle consideration. |
| EVD-006 | D-014 | Constrains the proposed avoidance of a single sustained-input dependency. |
| EVD-007 | D-010 | Provides display-refresh timing context without setting target. |
| EVD-008 | D-010 | Provides illustrative frame-budget context without setting target. |
| EVD-009 | D-010 | Identifies a possible diagnostic approximation. |
| EVD-010 | D-010 | Constrains claims made from CPU throttling. |
| EVD-011 | D-011 | Constrains randomness as variation/controllability tradeoff. |
| EVD-012 | D-011 | Constrains randomness choice by goals/audience dependence. |
| EVD-013 | D-012 | Constrains audience ideas to hypotheses pending research. |
| EVD-014 | D-007 | Constrains evaluation to specified criteria and representative users. |
| EVD-015 | D-007 | Constrains benchmark-task and measure design. |
| EVD-016 | D-007 | Constrains practice-related skill measurement. |
| EVD-017 | D-010 | Constrains performance testing to a documented target profile. |
| EVD-018 | D-010 | Constrains Device Mode claims as approximations. |
| EVD-019 | D-010 | Constrains emulation-only acceptance and supports physical-device validation. |

## Exact assumption → governed item edges

| Assumption | Governed item | Relationship and relevance |
|---|---|---|
| A-001 | R-011 | Supplies provisional measurable meaning for “instantly.” |
| A-002 | R-012 | Supplies provisional redundant-cue interpretation. |
| A-003 | R-007 | Preserves skim-zone/value tuning as configuration. |
| A-003 | R-008 | Preserves multiplier tuning as configuration. |
| A-004 | R-005 | Requires a documented numerical continuity tolerance. |
| A-005 | R-004 | Supplies deterministic equal-distance tie behavior. |
| A-006 | D-013 | Supplies the non-authoritative modular architecture inference. |

## Exact requirement → verification edges

| Requirement | Test | Relationship and relevance |
|---|---|---|
| R-001 | T-001 | Verifies clean-profile browser launch. |
| R-002 | T-002 | Verifies the gameplay command set and control inventory. |
| R-003 | T-003 | Verifies moving-anchor acceleration and survival. |
| R-004 | T-004 | Verifies nearest eligible selection and tie handling. |
| R-005 | T-005 | Verifies release continuity. |
| R-006 | T-006 | Verifies single collection. |
| R-007 | T-007 | Verifies skim/collision/outside boundaries. |
| R-008 | T-008 | Verifies multiplier transitions and scoring effect. |
| R-009 | T-009 | Verifies inward drift and collapse progression. |
| R-010 | T-010 | Verifies strict run cap over named seeds. |
| R-011 | T-011 | Verifies provisional restart latency and reset. |
| R-012 | T-012 | Verifies cue inventory and muted operability. |
| R-013 | T-013 | Verifies art inventory and representative capture. |
| R-014 | T-014 | Verifies solo production inventory and accepted estimate. |
| R-015 | T-015 | Runs the approved movement-proof protocol or reports not evaluated. |
| R-016 | T-016 | Verifies a complete run remains in one playfield. |
| R-017 | T-017 | Verifies physics candidate labeling and authority boundary. |
| R-018 | T-018 | Verifies separate accessibility decisions/unresolved states. |
| R-019 | T-019 | Verifies reference-profile completeness. |
| R-020 | T-020 | Verifies target/method completeness and performance result. |
| R-021 | T-021 | Verifies randomness-policy completeness and seed behavior if required. |
