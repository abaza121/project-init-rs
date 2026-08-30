# SWOT analysis

This is an initiation analysis, not a market verdict. Strengths and weaknesses derive mainly from the brief; opportunities and threats are hypotheses to validate. Evidence references point to the canonical records in [Traceability.md](Traceability.md).

## Strengths

- A distinctive control thesis is stated precisely: nearest-anchor hold and momentum-preserving release (SC-003, SC-004; R-004, R-005).
- The run loop already combines collection, risk/reward skimming, score continuity, and environmental pressure (R-006, R-007, R-008, R-009, R-010).
- One screen, one gameplay action, and sparse geometry create a plausible solo-prototype boundary (R-002, R-013, R-014, R-016).
- Instant restart and sub-three-minute runs support rapid repetition if the inferred restart threshold is accepted (R-010, R-011, A-001).

## Weaknesses

- The central success terms “skillful” and “predictable” have no approved measures, participants, or thresholds (R-015, OQ-008, OQ-009, OQ-010).
- Physics and feel constants remain unsettled; the D-008 model is only a candidate (OQ-001, OQ-011).
- Accessibility scope is split across unresolved stakeholder choices (OQ-002, OQ-003, OQ-004).
- Performance cannot be accepted until both a reference profile and numeric target exist (OQ-005, OQ-006).
- Core scoring details are named but untuned (A-003), so an implemented loop can still fail to create understandable incentives.

## Opportunities — hypotheses, not findings

- A deterministic fixture/replay layer could make cause-and-effect debugging and movement studies more credible (D-013), while the eventual run-randomness policy remains open (OQ-007).
- Redundant visual and audio cues may broaden playable contexts and strengthen causal feedback (A-002; evidence context EVD-004, EVD-005, EVD-006), subject to stakeholder accessibility choices.
- Configuration-driven tuning may let a solo developer compare movement variants without restructuring the build (A-003, A-006).
- The movement system may support depth from timing alone, but that remains the specific claim R-015 must test.

## Threats — risks, not predictions

- Anchor switching may be mathematically consistent yet not feel predictable on screen; internal consistency does not pass R-015.
- Excess randomness could obscure learning, while too little could reduce replay variation; no approved balance exists (D-011; EVD-011, EVD-012).
- Browser/display timing and hardware variation could produce inconsistent feel; generic emulation cannot by itself establish low-end support (D-010; EVD-007, EVD-008, EVD-009, EVD-010, EVD-017, EVD-018, EVD-019).
- A sustained hold interaction may exclude some players if the project does not make separate modality and activation-mode choices (D-009; EVD-004, EVD-005, EVD-006).
- “Instant” restart and solo scope may be overclaimed without a reference profile and a reviewed backlog (R-011, R-014).

## Initiation response

Run the smallest configurable movement spike first, instrument it against T-003, T-004, and T-005, and do not lock content volume around it. In parallel, seek stakeholder closure of every item in OpenQuestions.md. Treat any tuning and architecture in this package as proposed until its corresponding decision is accepted.
