# Research 02 — Experience, Fairness, and Accessibility

## Research questions

1. What external guidance is relevant to deaf and hard-of-hearing access for an audio-layered game?
2. Can external guidance provide a universal pass/fail threshold for hidden-information fairness?

The answers inform `D-007`, `D-008`, `D-015`, `DAD-REQ-015`, and `DAD-REQ-016`. They do not authorize product scope.

## Canonical evidence records

### EVD-001 — Multiple sensory channels

- Claim: Microsoft recommends expressing important visual and audio gameplay cues through multiple sensory methods so success does not rely exclusively on sight or hearing.
- Source: [Xbox Accessibility Guideline 103: Additional channels for visual and audio cues](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/103)
- Reliability: high; primary platform-holder guidance.
- Verified: 2026-08-30.
- Authority boundary: supports considering equivalent visual channels for gameplay-relevant audio; does not commit the feature.
- Trace: `DAD-REQ-016`; `D-005`; `D-007`; `D-015`; `OQ-001`.

### EVD-002 — Subtitles and captions

- Claim: Microsoft recommends text equivalents for spoken content and important non-speech sounds, speaker identification, spatial indication where relevant, and communication in addition to color.
- Source: [Xbox Accessibility Guideline 104: Subtitles and captions](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/104)
- Reliability: high; primary platform-holder guidance.
- Verified: 2026-08-30.
- Authority boundary: supports a caption proposal; does not commit captions or their milestone.
- Trace: `DAD-REQ-016`; `D-005`; `D-007`; `D-015`; `OQ-001`.

### EVD-003 — Audio controls and mono

- Claim: Microsoft recommends independently adjustable audio categories and an option to convert stereo output to mono.
- Source: [Xbox Accessibility Guideline 105: Audio accessibility](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/105)
- Reliability: high; primary platform-holder guidance.
- Verified: 2026-08-30.
- Authority boundary: supports considering controls and mono; does not commit them.
- Trace: `DAD-REQ-016`; `D-007`; `D-015`; `OQ-001`.

### EVD-004 — Quality values depend on context

- Claim: ISO/IEC 25022:2016 defines quality-in-use measures but does not assign universal value ranges or compliance grades because acceptable values depend on a product’s context of use and users’ needs.
- Source: [ISO/IEC 25022:2016 — Measurement of quality in use](https://www.iso.org/standard/35746.html)
- Reliability: high; primary standards body.
- Verified: official abstract/status page accessed 2026-08-30. It also reports the standard as published and under revision.
- Authority boundary: supports project-specific values; does not supply a fairness cutoff or duration bounds.
- Trace: `DAD-REQ-015`; `DAD-REQ-013`; `D-008`; `D-011`; `D-013`; `OQ-004`; `OQ-005`.

### EVD-005 — Usability framework does not prescribe a method

- Claim: ISO 9241-11:2018 provides a usability framework and describes usability as an outcome of use, but does not prescribe specific design-development or evaluation processes or methods.
- Source: [ISO 9241-11:2018 — Usability: Definitions and concepts](https://www.iso.org/standard/63500.html)
- Reliability: high; primary standards body.
- Verified: 2026-08-30.
- Authority boundary: supports explicit contextual evaluation; does not provide a test method or threshold.
- Trace: `DAD-REQ-015`; `D-008`; `OQ-004`.

### EVD-006 — Difficulty is player-relative

- Claim: Microsoft describes game difficulty as subjective and arising from the relationship between player abilities and game barriers, and advises consulting relevant user-research and disability-community expertise for a specific game.
- Source: [Xbox Accessibility Guideline 108: Game difficulty options](https://learn.microsoft.com/en-us/xbox/accessibility/xbox-accessibility-guidelines/108)
- Reliability: high; primary platform-holder guidance.
- Verified: 2026-08-30.
- Authority boundary: supports game- and audience-specific validation; does not authorize a Dead Air Dispatch threshold or difficulty scope.
- Trace: `DAD-REQ-015`; `D-008`; `OQ-004`.

## Synthesis

- Evidence: important audio can be represented through additional channels; captions and audio controls are established candidate techniques.
- Evidence: usability and difficulty evaluation depend on people and context, and the cited standards do not provide a universal cutoff.
- Inference/recommendation: take the options in `D-015` to an accessibility scope review and design the signal-state interface so visual equivalents can be added without rewriting simulation rules.
- Not authorized: “playable with audio muted,” captions, pre-game settings, static-resistant caption styling, mono, volume categories, an audience, a sample size, a rating scale, or a passing percentage.

## Retired duplicate input record

The supplied `EVD-010` repeats the same ISO/IEC 25022 claim as `EVD-004`. To maintain one canonical evidence record per claim, `EVD-010` is retired as a duplicate alias and all traces use `EVD-004`.

