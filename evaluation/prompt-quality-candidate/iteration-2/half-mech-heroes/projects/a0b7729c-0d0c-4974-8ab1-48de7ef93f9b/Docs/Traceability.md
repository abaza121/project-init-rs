# Trace Map

## Product coverage

- AC-001 supports REQ-001 because exact title inspection verifies the Half-Mech Heroes project title; DEC-001 supports REQ-001 because the same Half-Mech Heroes title is canonical.
- AC-002 supports REQ-002 because a two-human local run verifies the local two-player couch arcade game; DEC-002 supports REQ-002 because its two local player slots implement that local session.
- AC-003 supports REQ-003 because one mech-entity inspection verifies both players operate the same malfunctioning rescue mech; DEC-003 supports REQ-003 because the shared MechAggregate is that same mech.
- AC-004 supports REQ-004 because left-side-only input inspection verifies movement and jumping; DEC-004 supports REQ-004 because MovementDomain owns those left-side actions.
- AC-005 supports REQ-005 because right-side-only input inspection verifies magnetic-arm and shield aim; DEC-005 supports REQ-005 because ToolDomain owns those right-side actions.
- AC-006 supports REQ-006 because one swap per confirmed damage event verifies automatic role swapping; DEC-006 supports REQ-006 because RoleRouter atomically swaps roles on confirmed damage.
- AC-007 supports REQ-007 because one scrolling level-instance count verifies the single scrolling disaster zone; DEC-007 supports REQ-007 because BlockDirector owns that one scrolling instance.
- AC-008 supports REQ-008 because the falling-to-caught state transition verifies catching falling civilians; DEC-008 supports REQ-008 because CivilianSystem owns that catch transition.
- AC-009 supports REQ-009 because a thrown-clear boundary verifies wreckage is thrown out of the way; DEC-009 supports REQ-009 because magnetic wreckage owns the thrown-clear state.
- AC-010 supports REQ-010 because a visible increase before timeout verifies the teamwork multiplier once OQ-008 is answered; DEC-010 supports REQ-010 because MultiplierPolicy preserves the teamwork-multiplier boundary.
- AC-011 supports REQ-011 because a 180-second timer measurement verifies the three-minute rescue attempt; CON-001 supports REQ-011 because it fixes the same three-minute duration.
- AC-012 supports REQ-012 because a 5.0-second measurement tests quick restarts; DEC-012 supports REQ-012 because fresh-session reconstruction implements restart under ASM-002.
- AC-013 supports REQ-013 because exact throw-distance, recoil-angle, and recovery-time observations test exaggerated physics; DEC-013 supports REQ-013 because tunable force and impulse parameters implement exaggerated physics under ASM-008.
- AC-014 supports REQ-014 because solid-mask identification tests chunky silhouettes; DEC-014 supports REQ-014 because distinct gameplay-class masks implement chunky silhouettes.
- AC-015 supports REQ-015 because inspection of every tagged failure tests funny mechanical failures; DEC-015 supports REQ-015 because tagged authored reactions implement the failures.
- AC-016 supports REQ-016 because timed state identification tests television readability; DEC-016 supports REQ-016 because redundant presentation cues are tested in the television setup under ASM-003.
- AC-017 supports REQ-017 because full action runs verify two controllers and one shared keyboard; DEC-017 supports REQ-017 because DeviceAdapter normalises both input configurations.
- AC-018 supports REQ-018 because manifest and traversal counts verify one city block; CON-003 supports REQ-018 because it fixes the city-block count.
- AC-019 supports REQ-019 because registry and spawn counts verify one civilian type; DEC-030 supports REQ-019 because CivilianSystem enforces the one civilian-type entry.
- AC-020 supports REQ-020 because registry and spawn counts verify three hazards after OQ-006 is answered; DEC-031 supports REQ-020 because HazardSystem enforces three catalogue entries.
- AC-021 supports REQ-021 because dyad improvement and independent humour reports test coordination mastery and comedy; DEC-019 supports REQ-021 because its coordinated-action and failure instrumentation serves that dyad test.
- AC-022 supports REQ-022 because scope inspection keeps input remapping unresolved; DEC-023 supports REQ-022 because physical-input separation does not select remapping scope.
- AC-023 supports REQ-023 because scope inspection keeps solo play unresolved; DEC-027 supports REQ-023 because no solo mode is included or excluded.
- AC-024 supports REQ-024 because scope inspection keeps difficulty scaling unresolved; DEC-022 supports REQ-024 because DifficultyPolicy remains unconfigured.
- AC-025 supports REQ-025 because audience-record inspection keeps the exact target age unresolved; DEC-024 supports REQ-025 because no exact target age or rating proxy is assigned.

## Clarification edges

- ANS-006 answers OQ-001 with a non-resolution because the supplied input-remapping text explicitly requests stakeholder clarification.
- ANS-001 answers OQ-002 with a non-resolution because the supplied solo-play text explicitly requests stakeholder clarification.
- ANS-007 answers OQ-003 with a non-resolution because the supplied difficulty-scaling text explicitly requests stakeholder clarification.
- ANS-002 answers OQ-004 with a non-resolution because the supplied target-age text explicitly requests stakeholder clarification.
- ANS-003 answers OQ-005 with a non-resolution because the supplied platform text explicitly requests stakeholder clarification.
- ANS-004 answers OQ-006 with a non-resolution because the supplied hazard text labels the proposed identities provisional.
- ANS-005 answers OQ-007 with a non-resolution because the supplied success-condition text explicitly requests stakeholder clarification.
- ANS-008 answers OQ-008 with a non-resolution because the supplied multiplier text explicitly requests stakeholder clarification.

## External-material edges

- EVD-001 supports DEC-024 because both records concern the boundary between rating categories and the exact target age.
- EVD-002 supports DEC-023 because both records concern input remapping and the action-to-input boundary.
- EVD-003 supports DEC-022 because both records concern difficulty options and the unconfigured DifficultyPolicy.
- EVD-004 supports DEC-026 because both records concern objective clarity and the visible OutcomePolicy boundary.
- EVD-005 supports DEC-025 because both records concern disaster-hazard categories and the configurable hazard catalogue.
- EVD-006 supports DEC-017 because both records concern standard gamepad controls and the controller DeviceAdapter.
- EVD-007 supports DEC-028 because both records concern the Steam delivery fee and the blocked platform-delivery plan.
- EVD-008 supports DEC-029 because both records concern accessibility guidance as best-practice input rather than project authority.

## Inference edges

- ASM-001 supports DEC-021 because the platform-neutral core is the provisional response to the unresolved primary platform.
- ASM-002 supports AC-012 because the 5.0-second restart threshold operationalises quick restarts without claiming stakeholder authority.
- ASM-003 supports AC-014 because the silhouette test uses the provisional television setup.
- ASM-003 supports AC-016 because the readability test uses the provisional television setup.
- ASM-004 supports DEC-013 because exaggerated-physics tuning remains provisional.
- ASM-004 supports DEC-015 because mechanical-failure timing and intensity remain provisional.
- ASM-005 supports AC-021 because the five-dyad threshold is a prototype discovery assumption.
- ASM-006 supports AC-018 because one continuous level instance is the provisional interpretation of one city block.
- ASM-007 supports DEC-017 because two non-overlapping keyboard groups feed the shared semantic-action schema.
- ASM-008 supports AC-013 because the exact exaggerated-physics distance, angle, and recovery thresholds are provisional tuning markers.
