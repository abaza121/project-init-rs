# Research 02 — Experience and Accessibility

Research date: 2026-08-30  
Scope: primary Microsoft guidance used as nonbinding design and test constraints. These recommendations do not provide stakeholder authority or prove compliance.

## Canonical evidence

### EVD-EXP-001 — Television text guidance is context-sensitive

- Claim type: sourced recommendation
- Claim: Microsoft advises considering expected couch-to-television distance and gives a 26-pixel minimum default text body height for console experiences at 1080p.
- Primary source: [Xbox Accessibility Guideline 101: Text display](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/101)
- Reliability: high; official platform accessibility guidance.
- Checked: available 2026-08-30.
- Informs: REQ-016, DEC-035, ASM-001, and OQ-010D. The numeric guidance informs the provisional test but does not approve the project’s display, distance, cohort, or threshold.

### EVD-EXP-002 — In-game remapping is recommended

- Claim type: sourced recommendation
- Claim: Microsoft recommends in-game remapping for gameplay controls and recommends that displayed mappings reflect changes.
- Primary source: [Xbox Accessibility Guideline 107: Input](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/107)
- Reliability: high; official platform accessibility guidance.
- Checked: available 2026-08-30.
- Informs: DEC-029 and OQ-001. It supports accessibility value but does not place remapping in prototype scope.

### EVD-EXP-003 — Experienced difficulty is player-and-barrier dependent

- Claim type: sourced guidance
- Claim: Microsoft treats experienced difficulty as dependent on the relationship between player abilities and gameplay barriers, and recommends offering adjustable difficulty mechanisms.
- Primary source: [Xbox Accessibility Guideline 108: Game difficulty options](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/108)
- Reliability: high; official platform accessibility guidance.
- Checked: available 2026-08-30.
- Informs: DEC-031, DEC-039, OQ-003, and OQ-010A. It provides no project-specific scaling model or mastery threshold.

### EVD-EXP-004 — Critical elements should remain distinguishable

- Claim type: sourced recommendation
- Claim: Microsoft recommends that characters and other key gameplay elements remain visually distinguishable from their backgrounds.
- Primary source: [Xbox Accessibility Guideline 102: Contrast](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/102)
- Reliability: high; official platform accessibility guidance.
- Checked: available 2026-08-30.
- Informs: REQ-014, DEC-037, DEC-025, and DEC-026 as a presentation constraint, not an identity selection.

### EVD-EXP-005 — Critical cues should use additional sensory channels

- Claim type: sourced recommendation
- Claim: Microsoft recommends expressing important gameplay information through additional sensory methods so players need not rely on sight or hearing alone.
- Primary source: [Xbox Accessibility Guideline 103: Additional channels for visual and audio cues](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/103)
- Reliability: high; official platform accessibility guidance.
- Checked: available 2026-08-30.
- Informs: REQ-016, DEC-025, and DEC-026 as a cue-design constraint, not a creative content choice.

### EVD-EXP-006 — Xbox accessibility guidance is advisory

- Claim type: sourced fact about authority
- Claim: Microsoft characterises the Xbox Accessibility Guidelines as best practices and not as validation of legal or compliance requirements.
- Primary source: [Xbox Accessibility Guidelines](https://learn.microsoft.com/en-us/xbox/accessibility/guidelines)
- Reliability: high; official publisher of the guidance.
- Checked: available 2026-08-30.
- Informs: DEC-023 by preserving the authority boundary for every EVD-EXP recommendation.

## Evidence-based conclusion

The experience evidence constrains presentation and suggests accessibility options. It does not settle scope, content identities, target cohort, difficulty design, or project-specific pass thresholds. Those choices remain in OpenQuestions.md.

## Unavailable research

- No paired-player sessions were observed.
- No players with disabilities participated in the supplied work.
- No comedy, coordination, mastery, readability, or restart data exists.
- No final display, room, controller models, keyboard layout, or target-hardware performance is available.
